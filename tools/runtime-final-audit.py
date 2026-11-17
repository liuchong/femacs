#!/usr/bin/env python3
"""Converge post-acceptance runtime evidence for one frozen source and product set.

This tool reads existing stage, corpus, measurement, component and optional
local workflow receipts.  It accepts two cohort bindings: the daily
official-scale functional cohort (--functional/--protection, with the
48-candidate matrix recorded as an audit node) and the explicit matrix audit
node (--stable/--classified-stable). It never spawns compilers or editors,
never stats or opens physical product binaries/dumps/resources or source trees,
never installs or deletes files, never mutates inputs, and never promotes
compatibility, performance, or whole-Goal acceptance.
"""
from __future__ import annotations

import argparse
import codecs
import hashlib
import json
import math
import os
from pathlib import Path
import re
import stat
import sys
import time

REPO = Path(__file__).resolve().parent.parent

SCHEMA = 'femacs-runtime-final-audit-v1'
STAGE_SCHEMA = 'femacs-runtime-stage-v1'
EXECUTION_SCHEMA = 'femacs-runtime-execution-v1'
COMPLETION_SCHEMA = 'femacs-runtime-completion-v1'
STABLE_COMPLETION_SCHEMA = 'femacs-stable-completion-v1'
STABLE_RUN_SCHEMA = 'femacs-stable-suite-run-v2'
CLASSIFIED_STAGE_SCHEMA = 'femacs-stable-classified-stage-v1'
CLASSIFIED_LEDGER_SCHEMA = 'femacs-stable-classified-accounting-v1'
CLASSIFICATION_CATALOG_SCHEMA = 'femacs-stable-classification-v1'
REFERENCE_SELECTION_SCHEMA = 'femacs-runtime-reference-selection-v1'
REFERENCE_SELECTION_SCOPE = (
    'explicit-reference-replacement-preserving-60-compiled-candidates'
)

EXPECTED_BUILDS = 20
EXPECTED_PROFILES = 61
EXPECTED_CANDIDATES = 60
EXPECTED_ORDINARY_CANDIDATES = 48
EXPECTED_DIAGNOSTIC_CANDIDATES = 12
EXPECTED_NATIVE_CANDIDATES = 32
EXPECTED_DUMP_CANDIDATES = 48
EXPECTED_NON_HOOK_PROFILES = 49
EXPECTED_STABLE_ENTRYPOINTS = 555
EXPECTED_PERFORMANCE_ROUNDS = 9
EXPECTED_WARMUP_ROUNDS = 2
EXPECTED_MEASURED_ROUNDS = 7

SEMANTICS_ACTIONS = ('ert', 'negative', 'dump', 'relocation')
PERFORMANCE_ACTIONS = ('performance', 'performance-fallback')
COMPONENT_COMMANDS = (
    'build-owner-accounting',
    'stable-accounting',
    'runtime-accounting',
    'actual-source-build',
    'actual-source-contracts',
)
COMPONENT_PRIMARY_SOURCES = (
    'src/runtime_gc_observation.rs',
    'src/runtime_gc_collector.rs',
)
COMPONENT_SUPPORT_SOURCES = (
    'src/alloc.rs',
    'src/include/lisp.rs',
)
PERFORMANCE_LIMITATION = (
    'measurement collection only; busy/inconclusive data, unresolved '
    'compatibility and required layout comparisons prevent promotion'
)
PERFORMANCE_TOP_KEYS = frozenset({
    'schema', 'action', 'processes', 'samples', 'startups', 'summary',
    'comparisons', 'errors', 'excluded', 'fixture_sources',
    'fixtures_unchanged', 'passed', 'performance_accepted', 'limitation',
    'full_product_inputs_unchanged', 'driver_sources_unchanged',
    'goal_complete', 'compatibility_accepted', 'state_retirement_complete',
})
PERFORMANCE_FALLBACK_KEYS = frozenset({
    'permanent_full_fallback', 'startup_measured',
})
REFERENCE_SELECTION_FALSE_FLAGS = (
    'candidate_and_reference_libraries_byte_identical',
    'existing_components_reverted',
    'new_waivers_added',
    'reference_semantics_accepted',
    'compatibility_accepted',
    'performance_accepted',
    'goal_complete',
)
CLASSIFIED_POSTFLIGHT_CHECKS = (
    'all61_product_identities_unchanged',
    'all61_complete_receipts_valid',
    'source_export_unchanged',
    'reference_selection_unchanged',
    'classification_catalog_unchanged',
    'raw_stage_results_unchanged',
)

READ_CHUNK = 65536
MAX_SMALL_JSON_BYTES = 64 * 1024 * 1024
MAX_STREAM_ITEM_BYTES = 64 * 1024 * 1024
MAX_PERFORMANCE_BYTES = 512 * 1024 * 1024
ABSURD_INT_LIMIT = 10**18
STAT_FIELDS = ('st_dev', 'st_ino', 'st_size', 'st_mtime_ns', 'st_mode')
HEX64 = re.compile(r'^[0-9a-f]{64}$')
HEX40 = re.compile(r'^[0-9a-f]{40}$')


class AuditError(ValueError):
    """Structured audit validation or bounded-input failure."""

    def __init__(self, code: str, message: str) -> None:
        super().__init__(f'{code}: {message}')
        self.code = code
        self.detail = message


class OutputDirectoryError(ValueError):
    """Invalid or pre-existing fresh output directory."""


def _is_hex64(value: object) -> bool:
    return isinstance(value, str) and HEX64.fullmatch(value) is not None


def _is_nonempty_str(value: object) -> bool:
    return isinstance(value, str) and bool(value) and '\x00' not in value


def _is_finite_nonneg_num(value: object) -> bool:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        return False
    if isinstance(value, int):
        return 0 <= value <= ABSURD_INT_LIMIT
    return math.isfinite(value) and value >= 0.0


def _reject_nonfinite(_token: str) -> None:
    raise AuditError('non-finite-json-number', 'JSON NaN/Infinity constant is forbidden')


def _parse_finite_float(token: str) -> float:
    try:
        value = float(token)
    except (ValueError, OverflowError):
        raise AuditError('non-finite-json-number', 'JSON float overflow') from None
    if not math.isfinite(value):
        raise AuditError('non-finite-json-number', 'JSON float is not finite')
    return value


def _unique_object_pairs(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise AuditError('duplicate-json-key', f'duplicate JSON object key {key!r}')
        result[key] = value
    return result


def _decode_strict_json(raw: bytes, label: str) -> object:
    try:
        text = raw.decode('utf-8')
    except UnicodeDecodeError as exc:
        raise AuditError('invalid-utf8', f'{label} is not valid UTF-8') from exc
    try:
        return json.loads(
            text,
            object_pairs_hook=_unique_object_pairs,
            parse_float=_parse_finite_float,
            parse_constant=_reject_nonfinite,
        )
    except AuditError:
        raise
    except RecursionError as exc:
        raise AuditError('json-recursion-limit', f'{label} exceeded recursion limit') from exc
    except (json.JSONDecodeError, ValueError, TypeError, OverflowError) as exc:
        raise AuditError('invalid-json', f'{label}: {exc}') from exc


def _stat_signature(info: os.stat_result) -> tuple[int, int, int, int, int]:
    return tuple(getattr(info, field) for field in STAT_FIELDS)  # type: ignore[return-value]


def _normalize_path_text(raw_path: object, label: str) -> Path:
    if not isinstance(raw_path, (str, os.PathLike)):
        raise AuditError('invalid-path', f'{label} path must be a string or Path')
    text = os.fspath(raw_path)
    if not isinstance(text, str) or not text or '\x00' in text:
        raise AuditError('invalid-path', f'{label} path is empty or contains NUL')
    return Path(os.path.abspath(text))


def _assert_physical_directory(raw_path: object, label: str) -> Path:
    absolute = _normalize_path_text(raw_path, label)
    current = Path(absolute.anchor)
    info = None
    for part in absolute.parts[1:]:
        current = current / part
        try:
            info = current.lstat()
        except FileNotFoundError as exc:
            raise AuditError('missing-directory', f'{label} missing: {absolute}') from exc
        except OSError as exc:
            raise AuditError('inaccessible-directory', f'{label}: {exc}') from exc
        if stat.S_ISLNK(info.st_mode):
            raise AuditError('symlink-path', f'{label} traverses symlink: {current}')
        if not stat.S_ISDIR(info.st_mode):
            raise AuditError('not-directory', f'{label} is not a physical directory: {current}')
    if info is None:
        try:
            info = absolute.lstat()
        except OSError as exc:
            raise AuditError('inaccessible-directory', f'{label}: {exc}') from exc
        if not stat.S_ISDIR(info.st_mode) or stat.S_ISLNK(info.st_mode):
            raise AuditError('not-directory', f'{label} is not a physical directory: {absolute}')
    return absolute


def _assert_physical_regular_file(raw_path: object, label: str) -> tuple[Path, os.stat_result]:
    absolute = _normalize_path_text(raw_path, label)
    current = Path(absolute.anchor)
    parts = absolute.parts[1:]
    if not parts:
        raise AuditError('not-regular-file', f'{label} cannot be filesystem root: {absolute}')
    info = None
    for index, part in enumerate(parts):
        current = current / part
        try:
            info = current.lstat()
        except FileNotFoundError as exc:
            if index == len(parts) - 1:
                raise AuditError('missing-file', f'{label} missing: {absolute}') from exc
            raise AuditError('missing-file', f'{label} parent missing: {current}') from exc
        except OSError as exc:
            raise AuditError('inaccessible-file', f'{label}: {exc}') from exc
        if stat.S_ISLNK(info.st_mode):
            raise AuditError('symlink-path', f'{label} traverses symlink: {current}')
        if index < len(parts) - 1 and not stat.S_ISDIR(info.st_mode):
            raise AuditError('not-regular-file', f'{label} parent is not a directory: {current}')
    if info is None or not stat.S_ISREG(info.st_mode) or stat.S_ISLNK(info.st_mode):
        raise AuditError('not-regular-file', f'{label} is not a regular file: {absolute}')
    return absolute, info


def _prepare_fresh_output(out_arg: Path) -> Path:
    try:
        text = os.fspath(out_arg)
    except TypeError as exc:
        raise OutputDirectoryError('output path must be a valid filesystem path') from exc
    if not isinstance(text, str) or not text or '\x00' in text:
        raise OutputDirectoryError('output path must be non-empty without NUL')
    absolute = Path(os.path.abspath(text))
    repo_abs = Path(os.path.abspath(REPO))
    ref_tree = repo_abs / 'emacs'
    if absolute == ref_tree or absolute.is_relative_to(ref_tree):
        raise OutputDirectoryError('output directory must never lie inside emacs/')
    if absolute == repo_abs or (
        absolute.is_relative_to(repo_abs)
        and (not absolute.is_relative_to(repo_abs / 'build') or absolute == repo_abs / 'build')
    ):
        raise OutputDirectoryError('repository-internal output must be a child of build/')
    if os.path.lexists(absolute):
        raise OutputDirectoryError(f'fresh output directory already exists: {absolute}')
    current = Path(absolute.anchor)
    parts = absolute.parts[1:]
    if not parts:
        raise OutputDirectoryError('output path cannot be filesystem root')
    missing_index = len(parts)
    for index, part in enumerate(parts):
        candidate = current / part
        try:
            info = candidate.lstat()
        except FileNotFoundError:
            missing_index = index
            break
        except OSError as exc:
            raise OutputDirectoryError(f'cannot inspect output ancestor {candidate}: {exc}') from exc
        if stat.S_ISLNK(info.st_mode):
            raise OutputDirectoryError(f'output path traverses symlink: {candidate}')
        if not stat.S_ISDIR(info.st_mode):
            raise OutputDirectoryError(f'output ancestor is not a directory: {candidate}')
        current = candidate
    try:
        for index in range(missing_index, len(parts)):
            current = current / parts[index]
            mode = 0o700 if index == len(parts) - 1 else 0o755
            current.mkdir(mode=mode, exist_ok=False)
            info = current.lstat()
            if stat.S_ISLNK(info.st_mode) or not stat.S_ISDIR(info.st_mode):
                raise OutputDirectoryError(f'created output directory replaced unexpectedly: {current}')
    except OSError as exc:
        raise OutputDirectoryError(f'cannot create fresh output directory {absolute}: {exc}') from exc
    return absolute


def _sync_directory(directory: Path) -> None:
    flags = os.O_RDONLY | getattr(os, 'O_CLOEXEC', 0) | getattr(os, 'O_DIRECTORY', 0)
    fd = os.open(directory, flags)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def _write_durable_json(path: Path, payload: dict[str, object]) -> None:
    parent = path.parent
    repo_abs = Path(os.path.abspath(REPO))
    if path == repo_abs / 'emacs' or path.is_relative_to(repo_abs / 'emacs'):
        raise OutputDirectoryError('refusing to write under emacs/')
    parent_info = parent.lstat()
    if stat.S_ISLNK(parent_info.st_mode) or not stat.S_ISDIR(parent_info.st_mode):
        raise OutputDirectoryError(f'output parent is not a physical directory: {parent}')
    serialized = json.dumps(payload, indent=2, sort_keys=True, ensure_ascii=False, allow_nan=False) + '\n'
    encoded = serialized.encode('utf-8')
    temp_path = path.with_name(f'.{path.name}.tmp.{os.getpid()}')
    open_flags = (
        os.O_WRONLY | os.O_CREAT | os.O_EXCL
        | getattr(os, 'O_NOFOLLOW', 0) | getattr(os, 'O_CLOEXEC', 0)
    )
    fd = os.open(temp_path, open_flags, 0o600)
    try:
        view = memoryview(encoded)
        written = 0
        while written < len(encoded):
            count = os.write(fd, view[written:])
            if count <= 0:
                raise OSError('short write while persisting audit result')
            written += count
        os.fsync(fd)
    except BaseException:
        os.close(fd)
        try:
            temp_path.unlink()
        except OSError:
            pass
        raise
    os.close(fd)
    os.replace(temp_path, path)
    _sync_directory(parent)


class _IncrementalJsonTopStream:
    """Single-FD streaming reader for top-level JSON objects with large arrays."""

    def __init__(self, raw_read, max_item_bytes: int = MAX_STREAM_ITEM_BYTES) -> None:
        self._read = raw_read
        self._max_item_bytes = max_item_bytes
        self._utf8 = codecs.getincrementaldecoder('utf-8')('strict')
        self._decoder = json.JSONDecoder(
            object_pairs_hook=_unique_object_pairs,
            parse_float=_parse_finite_float,
            parse_constant=_reject_nonfinite,
        )
        self._buffer = ''
        self._eof = False

    def _fill(self) -> None:
        chunk = self._read(READ_CHUNK)
        if not chunk:
            try:
                tail = self._utf8.decode(b'', final=True)
            except UnicodeDecodeError as exc:
                raise AuditError('invalid-utf8', 'streamed JSON ended mid-character') from exc
            self._buffer += tail
            self._eof = True
            return
        try:
            decoded = self._utf8.decode(chunk, final=False)
        except UnicodeDecodeError as exc:
            raise AuditError('invalid-utf8', 'streamed JSON contains invalid UTF-8') from exc
        self._buffer += decoded
        if len(self._buffer) > self._max_item_bytes or (
            len(self._buffer) * 4 > self._max_item_bytes
            and len(self._buffer.encode('utf-8')) > self._max_item_bytes
        ):
            raise AuditError('oversized-json-item', 'streamed JSON item exceeded memory bound')

    def peek(self) -> str:
        while True:
            self._buffer = self._buffer.lstrip(' \t\r\n')
            if self._buffer or self._eof:
                return self._buffer[:1]
            self._fill()

    def consume_char(self, expected: str) -> None:
        if self.peek() != expected:
            raise AuditError('invalid-json', f'expected delimiter {expected!r}')
        self._buffer = self._buffer[1:]

    def decode_value(self) -> object:
        self.peek()
        while True:
            try:
                value, end = self._decoder.raw_decode(self._buffer)
                if end == len(self._buffer) and not self._eof:
                    self._fill()
                    continue
                if end < len(self._buffer) and self._buffer[end] not in ' \t\r\n,]}:':
                    raise AuditError('invalid-json', 'invalid token boundary in streamed JSON')
                self._buffer = self._buffer[end:]
                return value
            except AuditError:
                raise
            except RecursionError as exc:
                raise AuditError('json-recursion-limit', 'streamed JSON recursion exceeded') from exc
            except json.JSONDecodeError as exc:
                if self._eof:
                    raise AuditError('invalid-json', f'truncated or malformed streamed JSON: {exc}') from exc
                self._fill()


class InputPinRegistry:
    """Track single-FD {path, bytes, sha256} pins and re-verify before exit."""

    def __init__(self) -> None:
        self._pins: dict[str, dict[str, object]] = {}
        self._signatures: dict[str, tuple[int, int, int, int, int]] = {}

    def _check_expected(self, pin: dict[str, object], expected: object, label: str) -> None:
        if expected is None:
            return
        if isinstance(expected, str):
            if pin['sha256'] != expected:
                raise AuditError('digest-mismatch', f'{label} sha256 mismatch for {pin["path"]}')
            return
        if isinstance(expected, dict):
            exp_bytes = expected.get('bytes')
            exp_sha = expected.get('sha256')
            if type(exp_bytes) is not int or exp_bytes < 0 or not _is_hex64(exp_sha):
                raise AuditError('invalid-pin', f'{label} expected pin is malformed')
            if pin['bytes'] != exp_bytes or pin['sha256'] != exp_sha:
                raise AuditError('digest-mismatch', f'{label} bytes/sha256 mismatch for {pin["path"]}')
            return
        raise AuditError('invalid-pin', f'{label} unsupported expected pin type')

    def _record_pin(
        self,
        absolute: Path,
        byte_count: int,
        sha_hex: str,
        signature: tuple[int, int, int, int, int],
        expected: object,
        label: str,
    ) -> dict[str, object]:
        key = str(absolute)
        pin: dict[str, object] = {'path': key, 'bytes': byte_count, 'sha256': sha_hex}
        self._check_expected(pin, expected, label)
        if key in self._pins:
            if self._pins[key] != pin or self._signatures[key] != signature:
                raise AuditError('input-changed', f'{label} changed during audit: {key}')
        else:
            self._pins[key] = pin
            self._signatures[key] = signature
        return dict(pin)

    def _open_single_fd(self, raw_path: object, label: str) -> tuple[Path, int, os.stat_result]:
        absolute, pre_info = _assert_physical_regular_file(raw_path, label)
        flags = (
            os.O_RDONLY
            | getattr(os, 'O_NOFOLLOW', 0)
            | getattr(os, 'O_CLOEXEC', 0)
            | getattr(os, 'O_NONBLOCK', 0)
        )
        try:
            fd = os.open(absolute, flags)
        except OSError as exc:
            raise AuditError('open-failed', f'{label} cannot open {absolute}: {exc}') from exc
        try:
            opened = os.fstat(fd)
        except OSError as exc:
            os.close(fd)
            raise AuditError('stat-failed', f'{label} fstat failed {absolute}: {exc}') from exc
        if not stat.S_ISREG(opened.st_mode) or stat.S_ISLNK(opened.st_mode):
            os.close(fd)
            raise AuditError('not-regular-file', f'{label} descriptor is not a regular file: {absolute}')
        if _stat_signature(opened) != _stat_signature(pre_info):
            os.close(fd)
            raise AuditError('input-changed', f'{label} changed while opening: {absolute}')
        return absolute, fd, opened

    def _close_and_verify_fd(
        self,
        absolute: Path,
        fd: int,
        opened: os.stat_result,
        byte_count: int,
        label: str,
    ) -> tuple[int, int, int, int, int]:
        try:
            after = os.fstat(fd)
        except OSError as exc:
            raise AuditError('stat-failed', f'{label} post-read fstat failed: {exc}') from exc
        finally:
            os.close(fd)
        _, post_lstat = _assert_physical_regular_file(absolute, label)
        sig = _stat_signature(opened)
        if (
            sig != _stat_signature(after)
            or sig != _stat_signature(post_lstat)
            or byte_count != opened.st_size
        ):
            raise AuditError('input-changed', f'{label} mutated while reading: {absolute}')
        return sig

    def stream_pin(
        self,
        raw_path: object,
        *,
        expected: object = None,
        max_bytes: int | None = None,
        count_newlines: bool = False,
        label: str = 'evidence',
    ) -> tuple[Path, dict[str, object], int | None]:
        absolute, fd, opened = self._open_single_fd(raw_path, label)
        if max_bytes is not None and opened.st_size > max_bytes:
            os.close(fd)
            raise AuditError('input-too-large', f'{label} exceeds bound ({opened.st_size} > {max_bytes})')
        digest = hashlib.sha256()
        total = 0
        newlines = 0
        try:
            while True:
                try:
                    block = os.read(fd, READ_CHUNK)
                except OSError as exc:
                    raise AuditError('read-failed', f'{label} read error: {exc}') from exc
                if not block:
                    break
                total += len(block)
                if max_bytes is not None and total > max_bytes:
                    raise AuditError('input-too-large', f'{label} grew past {max_bytes} bytes')
                digest.update(block)
                if count_newlines:
                    newlines += block.count(b'\n')
        except BaseException:
            os.close(fd)
            raise
        sig = self._close_and_verify_fd(absolute, fd, opened, total, label)
        pin = self._record_pin(absolute, total, digest.hexdigest(), sig, expected, label)
        return absolute, pin, (newlines if count_newlines else None)

    def read_json(
        self,
        raw_path: object,
        *,
        max_bytes: int = MAX_SMALL_JSON_BYTES,
        expected: object = None,
        label: str = 'json-input',
    ) -> tuple[Path, dict[str, object], object]:
        absolute, fd, opened = self._open_single_fd(raw_path, label)
        if opened.st_size > max_bytes:
            os.close(fd)
            raise AuditError('input-too-large', f'{label} exceeds {max_bytes} bytes: {absolute}')
        digest = hashlib.sha256()
        chunks: list[bytes] = []
        total = 0
        try:
            while True:
                try:
                    block = os.read(fd, READ_CHUNK)
                except OSError as exc:
                    raise AuditError('read-failed', f'{label} read error: {exc}') from exc
                if not block:
                    break
                total += len(block)
                if total > max_bytes:
                    raise AuditError('input-too-large', f'{label} grew past {max_bytes} bytes: {absolute}')
                digest.update(block)
                chunks.append(block)
        except BaseException:
            os.close(fd)
            raise
        sig = self._close_and_verify_fd(absolute, fd, opened, total, label)
        pin = self._record_pin(absolute, total, digest.hexdigest(), sig, expected, label)
        document = _decode_strict_json(b''.join(chunks), f'{label} ({absolute})')
        return absolute, pin, document

    def stream_json_object(
        self,
        raw_path: object,
        *,
        stream_arrays: frozenset[str] = frozenset(),
        max_bytes: int = MAX_PERFORMANCE_BYTES,
        expected: object = None,
        label: str = 'streamed-json',
    ):
        """Yield ('array-item', key, item), ('array-end', key, count), or ('field', key, val)."""
        absolute, fd, opened = self._open_single_fd(raw_path, label)
        if opened.st_size > max_bytes:
            os.close(fd)
            raise AuditError('input-too-large', f'{label} exceeds {max_bytes} bytes: {absolute}')
        digest = hashlib.sha256()
        total = 0

        def read_chunk(size: int) -> bytes:
            nonlocal total
            try:
                block = os.read(fd, size)
            except OSError as exc:
                raise AuditError('read-failed', f'{label} read error: {exc}') from exc
            if block:
                total += len(block)
                if total > max_bytes:
                    raise AuditError('input-too-large', f'{label} grew past {max_bytes} bytes')
                digest.update(block)
            return block

        closed = False
        try:
            reader = _IncrementalJsonTopStream(read_chunk)
            seen_keys: set[str] = set()
            reader.consume_char('{')
            if reader.peek() != '}':
                while True:
                    key = reader.decode_value()
                    if not isinstance(key, str):
                        raise AuditError('invalid-json', f'{label} object key must be string')
                    if key in seen_keys:
                        raise AuditError('duplicate-json-key', f'{label} duplicate top-level key {key!r}')
                    seen_keys.add(key)
                    reader.consume_char(':')
                    if key in stream_arrays:
                        reader.consume_char('[')
                        count = 0
                        if reader.peek() != ']':
                            while True:
                                item = reader.decode_value()
                                count += 1
                                yield ('array-item', key, item)
                                if reader.peek() == ']':
                                    break
                                reader.consume_char(',')
                        reader.consume_char(']')
                        yield ('array-end', key, count)
                    else:
                        value = reader.decode_value()
                        yield ('field', key, value)
                    if reader.peek() == '}':
                        break
                    reader.consume_char(',')
            reader.consume_char('}')
            if reader.peek():
                raise AuditError('invalid-json', f'{label} trailing content after JSON object')
        except BaseException:
            closed = True
            os.close(fd)
            raise
        if not closed:
            sig = self._close_and_verify_fd(absolute, fd, opened, total, label)
            pin = self._record_pin(absolute, total, digest.hexdigest(), sig, expected, label)
            yield ('pin', '', pin)

    def snapshot_pins(self) -> list[dict[str, object]]:
        return [dict(self._pins[key]) for key in sorted(self._pins)]

    def verify_all_unchanged(self) -> tuple[bool, list[str]]:
        failures: list[str] = []
        for path_str in sorted(self._pins):
            expected_pin = self._pins[path_str]
            expected_sig = self._signatures[path_str]
            try:
                absolute, fd, opened = self._open_single_fd(path_str, 'exit-reverify')
                digest = hashlib.sha256()
                total = 0
                try:
                    while True:
                        block = os.read(fd, READ_CHUNK)
                        if not block:
                            break
                        total += len(block)
                        digest.update(block)
                except BaseException:
                    os.close(fd)
                    raise
                sig = self._close_and_verify_fd(absolute, fd, opened, total, 'exit-reverify')
                actual_pin = {'path': str(absolute), 'bytes': total, 'sha256': digest.hexdigest()}
                if actual_pin != expected_pin or sig != expected_sig:
                    failures.append(f'exit-reverify: input mutated during audit: {path_str}')
            except AuditError as exc:
                failures.append(f'exit-reverify ({path_str}): {exc}')
            except OSError as exc:
                failures.append(f'exit-reverify ({path_str}): OSError: {exc}')
        return (not failures), failures


def _require(condition: bool, code: str, message: str) -> None:
    if not condition:
        raise AuditError(code, message)


def _validate_process_record(proc: object, label: str, *, expected_exit: int = 0) -> None:
    _require(isinstance(proc, dict), 'invalid-process', f'{label} process record must be an object')
    assert isinstance(proc, dict)
    _require(proc.get('complete') is True, 'incomplete-process', f'{label} process did not complete')
    _require(proc.get('timed_out') is not True, 'process-timeout', f'{label} process timed out')
    _require(
        proc.get('remaining_group_terminated') is False,
        'descendant-leak',
        f'{label} left remaining group descendants',
    )
    _require(
        type(proc.get('exit_code')) is int and proc.get('exit_code') == expected_exit,
        'process-exit-mismatch',
        f'{label} exit_code={proc.get("exit_code")!r} != {expected_exit}',
    )


def _validate_profile_registry(products: object, label: str) -> dict[str, object]:
    _require(
        isinstance(products, list) and len(products) == EXPECTED_PROFILES,
        'invalid-profile-count',
        f'{label} must contain exactly {EXPECTED_PROFILES} profiles',
    )
    assert isinstance(products, list)
    seen_ids: set[str] = set()
    for index, row in enumerate(products):
        _require(isinstance(row, dict), 'invalid-profile', f'{label}[{index}] must be an object')
        assert isinstance(row, dict)
        pid = row.get('id')
        _require(_is_nonempty_str(pid) and pid not in seen_ids, 'invalid-profile', f'{label}[{index}] bad/dup id')
        assert isinstance(pid, str)
        seen_ids.add(pid)
        _require('_harness' not in row, 'reporting-override', f'{label}[{index}] contains reporting _harness override')
        _require(
            type(row.get('native')) is bool
            and type(row.get('observer')) is bool
            and type(row.get('test_hooks')) is bool
            and row.get('compiler_leg') in ('normal', 'checking')
            and row.get('collector') in ('off', 'major', 'young')
            and _is_nonempty_str(row.get('program'))
            and _is_nonempty_str(row.get('dump'))
            and isinstance(row.get('resource_roots'), dict)
            and bool(row.get('resource_roots')),
            'invalid-profile-fields',
            f'{label}[{index}] ({pid}) has malformed profile fields',
        )
    ref = products[0]
    _require(
        ref.get('id') == 'installed-reference'
        and ref.get('kind') == 'reference'
        and ref.get('compiler_leg') == 'normal'
        and ref.get('observer') is False
        and ref.get('test_hooks') is False
        and ref.get('collector') == 'off'
        and set(ref['resource_roots']) == {'lisp', 'data'},
        'invalid-reference-profile',
        f'{label}[0] must be the canonical installed-reference profile',
    )
    candidates = products[1:]
    _require(
        len(candidates) == EXPECTED_CANDIDATES
        and all(c.get('kind') in ('terminal', 'cocoa') for c in candidates),
        'invalid-candidate-profiles',
        f'{label} must have {EXPECTED_CANDIDATES} terminal/cocoa candidates after reference',
    )
    ordinary = [c['id'] for c in candidates if not c['test_hooks']]
    diagnostic = [c['id'] for c in candidates if c['test_hooks']]
    native_ids = [c['id'] for c in candidates if c['native']]
    dump_ids = [c['id'] for c in candidates if c['native'] or c['collector'] != 'off']
    _require(
        len(ordinary) == EXPECTED_ORDINARY_CANDIDATES
        and len(diagnostic) == EXPECTED_DIAGNOSTIC_CANDIDATES
        and len(native_ids) == EXPECTED_NATIVE_CANDIDATES
        and len(dump_ids) == EXPECTED_DUMP_CANDIDATES,
        'invalid-matrix-shape',
        f'{label} matrix partition mismatch (ordinary={len(ordinary)}, '
        f'diagnostic={len(diagnostic)}, native={len(native_ids)}, dump={len(dump_ids)})',
    )
    return {
        'profile_order': [p['id'] for p in products],
        'by_id': {p['id']: p for p in products},
        'ordinary_ids': ordinary,
        'diagnostic_ids': diagnostic,
        'native_ids': sorted(native_ids),
        'dump_ids': sorted(dump_ids),
        'candidate_ids': [c['id'] for c in candidates],
    }


def _resolve_build_source_owner(
    registry: InputPinRegistry,
    build_root: Path,
    build_doc: dict[str, object],
) -> tuple[Path, dict[str, object]]:
    source_export = build_doc.get('source_export')
    _require(isinstance(source_export, dict), 'invalid-source-export', 'build missing source_export object')
    assert isinstance(source_export, dict)
    _require(
        source_export.get('passed') is True
        and _is_nonempty_str(source_export.get('source'))
        and type(source_export.get('file_count')) is int
        and source_export['file_count'] > 0
        and _is_hex64(source_export.get('source_manifest_sha256')),
        'invalid-source-export',
        'build source_export fields are incomplete',
    )
    source_path = _normalize_path_text(source_export['source'], 'build.source_export.source')
    _require(
        str(source_path) == build_doc.get('source_root'),
        'source-root-mismatch',
        'build source_root disagrees with source_export.source',
    )
    current_root = build_root
    current_report: dict[str, object] = build_doc
    lineage: list[Path] = []
    repo_build = Path(os.path.abspath(REPO / 'build'))
    while True:
        _require(
            current_root.is_relative_to(repo_build)
            and current_root != repo_build
            and not any(
                current_root.is_relative_to(prior) or prior.is_relative_to(current_root)
                for prior in lineage
            )
            and not any(current_root.name == prior.name for prior in lineage),
            'invalid-source-lineage',
            f'invalid recovery build lineage at {current_root}',
        )
        _require(
            current_report.get('source_export') == source_export,
            'source-export-drift',
            'recovery lineage source_export differs from build',
        )
        if lineage or 'original_build' in current_report:
            if 'source_root' in current_report:
                _require(
                    current_report['source_root'] == str(source_path),
                    'source-root-drift',
                    'recovery lineage source_root differs from build',
                )
        lineage.append(current_root)
        if 'original_build' not in current_report:
            _require(
                source_path == current_root / 'source',
                'source-owner-mismatch',
                'frozen source path does not belong to terminal lineage owner',
            )
            _, _, manifest_doc = registry.read_json(
                current_root / 'source-inputs.json',
                max_bytes=MAX_SMALL_JSON_BYTES,
                expected=source_export['source_manifest_sha256'],
                label='build.source-inputs.json',
            )
            _require(
                isinstance(manifest_doc, dict)
                and manifest_doc.get('schema') == 1
                and isinstance(manifest_doc.get('files'), dict)
                and len(manifest_doc['files']) == source_export['file_count']
                and _is_hex64(manifest_doc.get('corpus_manifest_sha256')),
                'invalid-source-inputs',
                'source-inputs.json schema or file count mismatch',
            )
            assert isinstance(manifest_doc, dict)
            files_map = manifest_doc['files']
            assert isinstance(files_map, dict)
            corpus_entry = files_map.get('compatibility/stable-31.1/sources.json')
            _require(
                isinstance(corpus_entry, dict)
                and corpus_entry.get('sha256') == manifest_doc['corpus_manifest_sha256'],
                'corpus-manifest-mismatch',
                'source-inputs.json corpus_manifest_sha256 disagrees with pinned manifest entry',
            )
            return current_root, manifest_doc
        orig_root = _assert_physical_directory(current_report['original_build'], 'build.original_build')
        expected_partial = current_report.get('original_partial_sha256')
        _require(_is_hex64(expected_partial), 'invalid-partial-sha', 'missing original_partial_sha256')
        _, _, partial_doc = registry.read_json(
            orig_root / 'partial.json',
            max_bytes=128 * 1024 * 1024,
            expected=expected_partial,
            label='build.lineage.partial.json',
        )
        _require(isinstance(partial_doc, dict), 'invalid-partial-doc', 'lineage partial.json must be object')
        assert isinstance(partial_doc, dict)
        current_report = partial_doc
        current_root = orig_root


def _audit_build(registry: InputPinRegistry, build_arg: Path) -> dict[str, object]:
    build_root = _assert_physical_directory(build_arg, '--build')
    results_path, results_pin, build_doc = registry.read_json(
        build_root / 'results.json',
        max_bytes=128 * 1024 * 1024,
        label='build/results.json',
    )
    _require(isinstance(build_doc, dict), 'invalid-build', 'build/results.json must be a JSON object')
    assert isinstance(build_doc, dict)
    _require(
        build_doc.get('schema') == STAGE_SCHEMA
        and build_doc.get('action') == 'build'
        and build_doc.get('passed') is True
        and build_doc.get('goal_complete') is False,
        'build-not-passed',
        'build receipt must be a passing femacs-runtime-stage-v1 build document',
    )
    source_owner, source_inputs = _resolve_build_source_owner(registry, build_root, build_doc)
    cohorts = build_doc.get('source_cohorts')
    _require(
        isinstance(cohorts, list)
        and bool(cohorts)
        and build_doc['source_export'] in cohorts
        and all(
            isinstance(c, dict) and _is_hex64(c.get('source_manifest_sha256'))
            for c in cohorts
        ),
        'invalid-source-cohorts',
        'build source_cohorts missing or does not contain source_export',
    )
    assert isinstance(cohorts, list)
    cohort_digests = {c['source_manifest_sha256'] for c in cohorts}
    builds = build_doc.get('builds')
    _require(
        isinstance(builds, list) and len(builds) == EXPECTED_BUILDS,
        'invalid-build-count',
        f'build must record {EXPECTED_BUILDS} compiler configurations',
    )
    assert isinstance(builds, list)
    seen_builds: set[str] = set()
    for index, row in enumerate(builds):
        _require(isinstance(row, dict), 'invalid-build-row', f'build.builds[{index}] must be object')
        assert isinstance(row, dict)
        bid = row.get('id')
        _require(_is_nonempty_str(bid) and bid not in seen_builds, 'invalid-build-row', 'duplicate/empty build id')
        assert isinstance(bid, str)
        seen_builds.add(bid)
        _require(row.get('passed') is True, 'failed-build-row', f'build configuration {bid} did not pass')
        _require(
            row.get('source_cohort') in cohort_digests,
            'unknown-build-cohort',
            f'build configuration {bid} references unknown source_cohort',
        )
        _validate_process_record(row.get('process'), f'build[{bid}]')
    ref_identity = build_doc.get('reference_identity')
    _require(
        isinstance(ref_identity, dict) and set(ref_identity) == {'program', 'dump', 'resources'},
        'invalid-reference-identity',
        'build missing reference_identity {program, dump, resources}',
    )
    expected_products_path = build_root / 'products.json'
    _require(
        build_doc.get('product_list') == str(expected_products_path),
        'product-list-mismatch',
        'build product_list must point to <build>/products.json',
    )
    products_path, products_pin, products_doc = registry.read_json(
        expected_products_path,
        max_bytes=MAX_SMALL_JSON_BYTES,
        label='build/products.json',
    )
    _require(
        isinstance(products_doc, dict)
        and set(products_doc) == {'schema', 'products', 'source_cohorts'}
        and products_doc.get('schema') == 1
        and products_doc.get('products') == build_doc.get('products')
        and products_doc.get('source_cohorts') == cohorts,
        'invalid-products-doc',
        'build/products.json disagrees with build/results.json',
    )
    assert isinstance(products_doc, dict)
    matrix = _validate_profile_registry(products_doc['products'], 'build/products.json')
    return {
        'build_root': build_root,
        'results_path': str(results_path),
        'results_pin': results_pin,
        'products_path': products_path,
        'products_pin': products_pin,
        'build_doc': build_doc,
        'products_doc': products_doc,
        'source_owner': source_owner,
        'source_inputs': source_inputs,
        'source_root': str(build_doc['source_root']),
        'source_export': build_doc['source_export'],
        'source_cohorts': cohorts,
        'corpus_manifest_sha256': source_inputs['corpus_manifest_sha256'],
        'matrix': matrix,
    }


def _audit_reference_selection(
    registry: InputPinRegistry,
    build_ctx: dict[str, object],
    stage_summaries: list[tuple[str, object, bool]],
) -> dict[str, object]:
    present = [(name, item) for name, item, _ in stage_summaries if item is not None]
    if not present:
        products = build_ctx['products_doc']['products']  # type: ignore[index]
        return {
            'active': False,
            'epoch': None,
            'historical_epoch': None,
            'summary': None,
            'active_products': products,
            'active_products_path': str(build_ctx['products_path']),
            'active_products_sha256': build_ctx['products_pin']['sha256'],  # type: ignore[index]
            'matrix': build_ctx['matrix'],
        }
    canonical_name, canonical_summary = present[0]
    _require(
        isinstance(canonical_summary, dict)
        and _is_nonempty_str(canonical_summary.get('path'))
        and _is_hex64(canonical_summary.get('sha256'))
        and _is_nonempty_str(canonical_summary.get('epoch'))
        and _is_nonempty_str(canonical_summary.get('historical_epoch'))
        and _is_hex64(canonical_summary.get('selected_products_sha256'))
        and isinstance(canonical_summary.get('upstream'), dict)
        and isinstance(canonical_summary.get('capabilities'), dict),
        'invalid-reference-selection-summary',
        f'{canonical_name} has malformed reference_selection summary',
    )
    assert isinstance(canonical_summary, dict)
    for stage_name, item, required_when_active in stage_summaries:
        if item is None:
            if required_when_active:
                raise AuditError(
                    'reference-epoch-mismatch',
                    f'{stage_name} omitted reference_selection while {canonical_name} '
                    f'requires epoch {canonical_summary["epoch"]!r}',
                )
            continue
        if item != canonical_summary:
            other_epoch = item.get('epoch') if isinstance(item, dict) else None
            raise AuditError(
                'reference-epoch-mismatch',
                f'{stage_name} reference_selection (epoch={other_epoch!r}) differs '
                f'from {canonical_name} (epoch={canonical_summary["epoch"]!r})',
            )
    sel_path, sel_pin, sel_doc = registry.read_json(
        canonical_summary['path'],
        max_bytes=MAX_SMALL_JSON_BYTES,
        expected=canonical_summary['sha256'],
        label='reference-selection.json',
    )
    repo_build = Path(os.path.abspath(REPO / 'build'))
    sel_dir = sel_path.parent
    _require(
        sel_path.name == 'reference-selection.json'
        and sel_dir.is_relative_to(repo_build)
        and sel_dir != repo_build,
        'invalid-reference-selection-path',
        'reference-selection.json must reside in an owned build directory',
    )
    _require(isinstance(sel_doc, dict), 'invalid-reference-selection', 'reference-selection.json must be object')
    assert isinstance(sel_doc, dict)
    build_root = build_ctx['build_root']
    build_doc = build_ctx['build_doc']
    assert isinstance(build_doc, dict)
    _require(
        sel_doc.get('schema') == REFERENCE_SELECTION_SCHEMA
        and sel_doc.get('scope') == REFERENCE_SELECTION_SCOPE
        and sel_doc.get('epoch') == canonical_summary['epoch']
        and sel_doc.get('build_root') == str(build_root)
        and sel_doc.get('build_result_sha256') == build_ctx['results_pin']['sha256']  # type: ignore[index]
        and sel_doc.get('source_manifest_sha256') == build_doc['source_export']['source_manifest_sha256']
        and sel_doc.get('original_products_path') == str(build_ctx['products_path'])
        and sel_doc.get('original_products_sha256') == build_ctx['products_pin']['sha256']  # type: ignore[index]
        and sel_doc.get('profile_count') == EXPECTED_PROFILES
        and sel_doc.get('candidate_count') == EXPECTED_CANDIDATES,
        'reference-selection-build-mismatch',
        'reference-selection.json does not bind the audited build/source/products',
    )
    for flag in REFERENCE_SELECTION_FALSE_FLAGS:
        _require(
            sel_doc.get(flag) is False and canonical_summary.get(flag) is False,
            'invalid-reference-selection-flag',
            f'reference selection cannot set {flag}=True',
        )
    selected_products_expected = sel_dir / 'products.json'
    _require(
        sel_doc.get('selected_products_path') == str(selected_products_expected)
        and sel_doc.get('selected_products_sha256') == canonical_summary['selected_products_sha256'],
        'selected-products-path-mismatch',
        'reference-selection selected_products_path/sha256 mismatch',
    )
    _, selected_pin, selected_doc = registry.read_json(
        selected_products_expected,
        max_bytes=MAX_SMALL_JSON_BYTES,
        expected=canonical_summary['selected_products_sha256'],
        label='reference-selection/products.json',
    )
    orig_products_doc = build_ctx['products_doc']
    assert isinstance(orig_products_doc, dict)
    _require(
        isinstance(selected_doc, dict)
        and set(selected_doc) == {'schema', 'products', 'source_cohorts'}
        and selected_doc.get('schema') == 1
        and selected_doc.get('source_cohorts') == orig_products_doc['source_cohorts']
        and isinstance(selected_doc.get('products'), list)
        and len(selected_doc['products']) == EXPECTED_PROFILES
        and selected_doc['products'][1:] == orig_products_doc['products'][1:],
        'selected-candidates-changed',
        'reference selection altered compiled candidate entries or source_cohorts',
    )
    assert isinstance(selected_doc, dict)
    matrix = _validate_profile_registry(selected_doc['products'], 'reference-selection/products.json')
    _require(
        matrix['profile_order']
        == build_ctx['matrix']['profile_order']  # type: ignore[index]
        == sel_doc.get('profile_order'),
        'profile-order-changed',
        'reference selection altered 61-profile ordering',
    )
    orig_ref = sel_doc.get('original_reference')
    selected_ref = sel_doc.get('selected_reference')
    _require(
        isinstance(orig_ref, dict)
        and orig_ref.get('product') == orig_products_doc['products'][0]
        and orig_ref.get('identity') == build_doc['reference_identity']
        and orig_ref.get('historical_observations_retained') is True
        and orig_ref.get('historical_epoch') == canonical_summary['historical_epoch'],
        'original-reference-history-mismatch',
        'reference-selection original_reference history is invalid or relabeled',
    )
    reference_replaced = selected_ref.get('identity') != build_doc['reference_identity']
    _require(
        isinstance(selected_ref, dict)
        and selected_ref.get('product') == selected_doc['products'][0]
        and isinstance(selected_ref.get('identity'), dict)
        and set(selected_ref['identity']) == {'program', 'dump', 'resources'}
        and (orig_ref['historical_epoch'] != sel_doc['epoch']) is reference_replaced
        and (
            'reference_replaced' not in canonical_summary
            or canonical_summary.get('reference_replaced') is reference_replaced
        )
        and selected_ref.get('upstream') == canonical_summary['upstream']
        and selected_ref.get('capabilities') == canonical_summary['capabilities'],
        'selected-reference-mismatch',
        'reference-selection selected_reference or epoch transition disagrees with summary',
    )
    evidence_inputs = sel_doc.get('evidence_inputs')
    expected_evidence_names = {
        'recipe_info': 'recipe-info.json',
        'reference_probe': 'reference-probe.json',
        'upstream_tree_comparison': 'upstream-tree-comparison.json',
    }
    _require(
        isinstance(evidence_inputs, dict) and set(evidence_inputs) == set(expected_evidence_names),
        'incomplete-reference-evidence',
        'reference-selection missing adjacent evidence_inputs',
    )
    assert isinstance(evidence_inputs, dict)
    evidence_docs: dict[str, dict[str, object]] = {}
    for key, filename in expected_evidence_names.items():
        binding = evidence_inputs.get(key)
        _require(
            isinstance(binding, dict)
            and binding.get('path') == str(sel_dir / filename)
            and isinstance(binding.get('identity'), dict),
            'invalid-reference-evidence-binding',
            f'reference-selection {key} must bind adjacent {filename}',
        )
        assert isinstance(binding, dict)
        ident = binding['identity']
        assert isinstance(ident, dict)
        _, _, ev_doc = registry.read_json(
            sel_dir / filename,
            max_bytes=MAX_SMALL_JSON_BYTES,
            expected={'bytes': ident.get('bytes'), 'sha256': ident.get('sha256')},
            label=f'reference-selection/{filename}',
        )
        _require(isinstance(ev_doc, dict), 'invalid-reference-evidence', f'{filename} must be JSON object')
        assert isinstance(ev_doc, dict)
        evidence_docs[key] = ev_doc
    probe_doc = evidence_docs['reference_probe']
    tree_doc = evidence_docs['upstream_tree_comparison']
    _require(
        probe_doc.get('schema') == 'femacs-reference-install-probe-v1'
        and probe_doc.get('passed') is True,
        'failed-reference-probe',
        'reference-probe.json did not pass',
    )
    _validate_process_record(probe_doc.get('process'), 'reference-probe')
    _require(
        tree_doc.get('schema') == 'femacs-upstream-data-git-tree-comparison-v1'
        and tree_doc == canonical_summary.get('upstream_tree_comparison'),
        'reference-tree-comparison-mismatch',
        'upstream-tree-comparison.json disagrees with reference_selection summary',
    )
    patch_files = probe_doc.get('patch_files', {})
    _require(isinstance(patch_files, dict), 'invalid-reference-patches', 'patch_files must be object')
    assert isinstance(patch_files, dict)
    for url, patch_binding in sorted(patch_files.items()):
        _require(
            isinstance(patch_binding, dict)
            and _is_nonempty_str(patch_binding.get('path'))
            and isinstance(patch_binding.get('identity'), dict),
            'invalid-reference-patch',
            f'invalid captured patch binding for {url}',
        )
        assert isinstance(patch_binding, dict)
        patch_path = _normalize_path_text(patch_binding['path'], 'reference-patch')
        _require(
            patch_path.is_relative_to(sel_dir / 'patches'),
            'invalid-reference-patch-path',
            'captured reference patch must stay inside selection patches/',
        )
        ident = patch_binding['identity']
        assert isinstance(ident, dict)
        registry.stream_pin(
            patch_path,
            expected={'bytes': ident.get('bytes'), 'sha256': ident.get('sha256')},
            max_bytes=MAX_SMALL_JSON_BYTES,
            label='reference-selection/patch',
        )
    return {
        'active': True,
        'epoch': sel_doc['epoch'],
        'historical_epoch': orig_ref['historical_epoch'],
        'summary': canonical_summary,
        'selection_pin': sel_pin,
        'selected_products_pin': selected_pin,
        'active_products': selected_doc['products'],
        'active_products_path': str(selected_products_expected),
        'active_products_sha256': selected_pin['sha256'],
        'matrix': matrix,
    }


def _verify_execution_completion_dir(
    registry: InputPinRegistry,
    action_dir: Path,
    expected_action: str,
    active_products_sha256: str,
    *,
    expected_completion_sha: object = None,
    expected_detailed_sha: object = None,
    stream_detailed_only: bool = True,
    expected_fallback: bool | None = None,
) -> dict[str, object]:
    _, completion_pin, completion = registry.read_json(
        action_dir / 'completion.json',
        max_bytes=4 * 1024 * 1024,
        expected=expected_completion_sha,
        label=f'{expected_action}/completion.json',
    )
    _require(isinstance(completion, dict), 'invalid-completion', f'{expected_action} completion must be object')
    assert isinstance(completion, dict)
    _require(
        completion.get('schema') == COMPLETION_SCHEMA
        and completion.get('result_schema') == EXECUTION_SCHEMA
        and completion.get('action') == expected_action
        and completion.get('execution_complete') is True
        and completion.get('passed') is True
        and completion.get('post_hoc') is False
        and completion.get('full_product_inputs_unchanged') is True
        and completion.get('fixtures_unchanged') is True
        and completion.get('goal_complete') is False
        and completion.get('compatibility_accepted') is False
        and completion.get('performance_accepted') is False,
        'invalid-completion-fields',
        f'{expected_action}/completion.json failed completion invariants',
    )
    _, inputs_pin, inputs_doc = registry.read_json(
        action_dir / 'inputs.json',
        max_bytes=MAX_SMALL_JSON_BYTES,
        expected=completion.get('inputs_sha256'),
        label=f'{expected_action}/inputs.json',
    )
    _require(
        isinstance(inputs_doc, dict)
        and inputs_doc == completion.get('input_binding')
        and inputs_doc.get('action') == expected_action
        and inputs_doc.get('product_list_sha256') == active_products_sha256
        and inputs_doc.get('requested_groups') is None,
        'execution-input-mismatch',
        f'{expected_action}/inputs.json disagrees with active 61-profile registry or full group scope',
    )
    assert isinstance(inputs_doc, dict)
    if expected_action == 'performance':
        assert expected_fallback is not None
        control = completion.get('performance_control')
        _require(
            inputs_doc.get('permanent_full_fallback') is expected_fallback
            and control == {
                'permanent_full_fallback': expected_fallback,
                'startup_measured': not expected_fallback,
            },
            'performance-mode-mismatch',
            f'performance ({expected_fallback=}) completion/inputs mode binding mismatch',
        )
    retirement_ref = completion.get('retirement')
    _require(
        isinstance(retirement_ref, dict) and retirement_ref.get('path') == 'state-retirement.json',
        'invalid-retirement-ref',
        f'{expected_action} completion missing state-retirement.json binding',
    )
    assert isinstance(retirement_ref, dict)
    _, _, retirement_doc = registry.read_json(
        action_dir / 'state-retirement.json',
        max_bytes=MAX_SMALL_JSON_BYTES,
        expected={'bytes': retirement_ref.get('bytes'), 'sha256': retirement_ref.get('sha256')},
        label=f'{expected_action}/state-retirement.json',
    )
    _require(
        isinstance(retirement_doc, dict) and retirement_doc.get('complete') is True,
        'incomplete-state-retirement',
        f'{expected_action} state-retirement.json is not complete',
    )
    case_index_ref = completion.get('case_index')
    _require(
        isinstance(case_index_ref, dict)
        and case_index_ref.get('path') == 'case-index.jsonl'
        and type(case_index_ref.get('count')) is int
        and case_index_ref['count'] > 0,
        'invalid-case-index-ref',
        f'{expected_action} completion missing non-empty case-index.jsonl binding',
    )
    assert isinstance(case_index_ref, dict)
    _, _, newline_count = registry.stream_pin(
        action_dir / 'case-index.jsonl',
        expected={'bytes': case_index_ref.get('bytes'), 'sha256': case_index_ref.get('sha256')},
        count_newlines=True,
        label=f'{expected_action}/case-index.jsonl',
    )
    _require(
        newline_count == case_index_ref['count'],
        'case-index-count-mismatch',
        f'{expected_action} case-index.jsonl line count {newline_count} != {case_index_ref["count"]}',
    )
    detailed_ref = completion.get('detailed')
    _require(
        isinstance(detailed_ref, dict) and detailed_ref.get('path') == 'results.json',
        'invalid-detailed-ref',
        f'{expected_action} completion missing detailed results.json binding',
    )
    assert isinstance(detailed_ref, dict)
    if expected_detailed_sha is not None:
        _require(
            detailed_ref.get('sha256') == expected_detailed_sha,
            'detailed-sha-mismatch',
            f'{expected_action} stage receipt_sha256 disagrees with completion detailed.sha256',
        )
    detailed_pin = None
    if stream_detailed_only:
        _, detailed_pin, _ = registry.stream_pin(
            action_dir / 'results.json',
            expected={'bytes': detailed_ref.get('bytes'), 'sha256': detailed_ref.get('sha256')},
            label=f'{expected_action}/results.json',
        )
    return {
        'completion': completion,
        'completion_pin': completion_pin,
        'inputs_pin': inputs_pin,
        'detailed_ref': detailed_ref,
        'detailed_pin': detailed_pin,
    }


def _audit_prepare(
    registry: InputPinRegistry,
    prepare_doc: dict[str, object],
    prepare_pin: dict[str, object],
    build_ctx: dict[str, object],
    ref_ctx: dict[str, object],
) -> dict[str, object]:
    _require(
        prepare_doc.get('schema') == STAGE_SCHEMA
        and prepare_doc.get('action') == 'prepare'
        and prepare_doc.get('passed') is True
        and prepare_doc.get('goal_complete') is False
        and prepare_doc.get('execution_selection') is None,
        'prepare-not-passed',
        'prepare stage receipt must be a passing femacs-runtime-stage-v1 prepare document',
    )
    expected_prepare_products_sha = (
        str(ref_ctx['active_products_sha256'])
        if prepare_doc.get('reference_selection') is not None
        else str(build_ctx['products_pin']['sha256'])  # type: ignore[index]
    )
    if prepare_doc.get('reference_selection') is not None:
        _require(
            prepare_doc.get('reference_selection') == ref_ctx['summary'],
            'prepare-reference-epoch-mismatch',
            'prepare stage reference_selection disagrees with active epoch',
        )
    _require(
        prepare_doc.get('build_receipt') == str(build_ctx['build_root'])
        and prepare_doc.get('build_complete') is True
        and prepare_doc.get('source_root') == build_ctx['source_root']
        and prepare_doc.get('product_list_sha256') == expected_prepare_products_sha,
        'prepare-provenance-mismatch',
        'prepare stage does not bind the audited build, source_root, or 61-profile SHA',
    )
    expected_native = ref_ctx['matrix']['native_ids']  # type: ignore[index]
    _require(
        prepare_doc.get('native_profiles') == expected_native
        and len(expected_native) == EXPECTED_NATIVE_CANDIDATES,
        'prepare-native-profiles-mismatch',
        f'prepare stage must cover all {EXPECTED_NATIVE_CANDIDATES} native candidate profiles',
    )
    _validate_process_record(prepare_doc.get('process'), 'prepare.process')
    prep_receipt_path = _normalize_path_text(
        prepare_doc.get('preparation_receipt'), 'prepare.preparation_receipt'
    )
    neg_root = _assert_physical_directory(prepare_doc.get('negative_root'), 'prepare.negative_root')
    _require(
        prep_receipt_path == neg_root / 'results.json'
        and _is_hex64(prepare_doc.get('preparation_receipt_sha256')),
        'prepare-receipt-locator-mismatch',
        'prepare preparation_receipt must be <negative_root>/results.json with 64-hex sha256',
    )
    _verify_execution_completion_dir(
        registry,
        neg_root,
        'prepare-negative',
        expected_prepare_products_sha,
        expected_detailed_sha=prepare_doc['preparation_receipt_sha256'],
        stream_detailed_only=False,
    )
    _, prep_exec_pin, prep_exec_doc = registry.read_json(
        prep_receipt_path,
        max_bytes=MAX_SMALL_JSON_BYTES,
        expected=prepare_doc['preparation_receipt_sha256'],
        label='prepare-negative/results.json',
    )
    _require(
        isinstance(prep_exec_doc, dict)
        and prep_exec_doc.get('schema') == EXECUTION_SCHEMA
        and prep_exec_doc.get('action') == 'prepare-negative'
        and prep_exec_doc.get('passed') is True
        and prep_exec_doc.get('full_product_inputs_unchanged') is True
        and prep_exec_doc.get('fixtures_unchanged') is True
        and prep_exec_doc.get('driver_sources_unchanged') is True
        and prep_exec_doc.get('state_retirement_complete') is True
        and prep_exec_doc.get('goal_complete') is False,
        'prepare-negative-not-passed',
        'prepare-negative execution results.json failed schema/passing invariants',
    )
    assert isinstance(prep_exec_doc, dict)
    rows = prep_exec_doc.get('products')
    _require(
        isinstance(rows, list)
        and len(rows) == EXPECTED_NATIVE_CANDIDATES
        and {r.get('profile') for r in rows if isinstance(r, dict)} == set(expected_native),
        'prepare-negative-coverage-mismatch',
        f'prepare-negative must verify all {EXPECTED_NATIVE_CANDIDATES} native profiles',
    )
    assert isinstance(rows, list)
    unit_counts: dict[str, int] = {}
    for row in rows:
        assert isinstance(row, dict)
        prof = str(row['profile'])
        _require(
            row.get('passed') is True
            and isinstance(row.get('units'), list)
            and len(row['units']) >= 2
            and all(isinstance(u, dict) and u.get('passed') is True for u in row['units']),
            'prepare-negative-profile-failed',
            f'prepare-negative profile {prof} failed or has incomplete format units',
        )
        _, _, manifest_doc = registry.read_json(
            neg_root / prof / 'manifest.json',
            max_bytes=4 * 1024 * 1024,
            label=f'prepare-negative/{prof}/manifest.json',
        )
        _, _, prep_profile_doc = registry.read_json(
            neg_root / prof / 'preparation.json',
            max_bytes=16 * 1024 * 1024,
            label=f'prepare-negative/{prof}/preparation.json',
        )
        _require(
            isinstance(manifest_doc, dict)
            and manifest_doc.get('schema') == 'femacs-runtime-native-negative-v1'
            and isinstance(manifest_doc.get('control'), dict)
            and isinstance(manifest_doc.get('units'), list)
            and bool(manifest_doc['units'])
            and isinstance(prep_profile_doc, dict)
            and prep_profile_doc.get('passed') is True,
            'prepare-negative-manifest-invalid',
            f'prepare-negative {prof} manifest/preparation receipt invalid',
        )
        unit_counts[prof] = len(row['units'])
    return {
        'passed': True,
        'receipt': prepare_pin,
        'preparation_receipt': prep_exec_pin,
        'native_profile_count': len(expected_native),
        'unit_counts_per_profile': unit_counts,
    }


def _verify_authored_harness(registry, stage_path, stage_doc, build_ctx):
    """Bind the captured current evaluator without opening compiled products."""
    root = stage_path.parent / 'authored-harness'
    selected = stage_doc.get('authored_harness_selection')
    _require(isinstance(selected, dict) and selected.get('unchanged') is True
             and stage_doc.get('harness_root') == str(root),
             'authored-harness-unverified', 'current authored harness was not captured and verified')
    _, _, receipt = registry.read_json(root / 'authored-harness-selection.json',
        max_bytes=4 * 1024 * 1024, label='authored-harness-selection')
    _require(isinstance(receipt, dict)
             and receipt == {key: value for key, value in selected.items() if key != 'unchanged'}
             and receipt.get('schema') == 'femacs-authored-python-harness-v1'
             and receipt.get('compiled_source_root') == build_ctx['source_root']
             and receipt.get('compiled_products_unchanged') is True
             and receipt.get('frozen_source_unchanged') is True,
             'authored-harness-binding-mismatch', 'authored harness receipt differs from its stage')
    files = receipt.get('files')
    required = {'tools/'+name for name in (
        'runtime-acceptance.py', 'runtime-execution.py', 'runtime-retire.py',
        'runtime-profile.py', 'runtime-language-contract.py', 'reference-corpus.py')}
    required.update({'tools/fixtures/runtime-process-runner.el', 'tools/fixtures/runtime-language.el',
                     'tools/fixtures/xml-acceptance.el', 'tools/fixtures/sqlite-acceptance.el'})
    _require(isinstance(files, dict) and required <= set(files) and len(files) <= 256,
             'authored-harness-inputs-missing', 'authored harness lacks required driver/fixture inputs')
    for name, row in files.items():
        relative = Path(name)
        _require(not relative.is_absolute() and '..' not in relative.parts
                 and relative.parts[0] == 'tools' and isinstance(row, dict)
                 and isinstance(row.get('source'), dict) and isinstance(row.get('captured'), dict),
                 'authored-harness-input-invalid', 'invalid captured authored input')
        original, captured = row['source'], row['captured']
        identity = captured.get('identity')
        path = root / relative
        _require(isinstance(identity, dict) and identity == original.get('identity')
                 and captured.get('path') == str(path)
                 and identity.get('kind') == 'file' and type(identity.get('mode')) is int,
                 'authored-harness-identity-mismatch', 'captured authored input identity differs')
        registry.stream_pin(path, expected=identity, max_bytes=1024*1024,
                            label='authored-harness/'+name)
        _require(stat.S_IMODE(path.lstat().st_mode) == identity['mode'],
                 'authored-harness-mode-changed', 'captured authored input mode changed')


def _verify_stage_postflight_and_context(
    registry: InputPinRegistry,
    stage_path: Path,
    stage_doc: dict[str, object],
    build_ctx: dict[str, object],
    ref_ctx: dict[str, object],
    label: str,
) -> dict[str, object]:
    _require(
        stage_doc.get('execution_selection') is None,
        'reporting-override-rejected',
        f'{label} uses reporting-only execution_selection override',
    )
    _require(
        stage_doc.get('reference_selection') == ref_ctx['summary'],
        'reference-epoch-mismatch',
        f'{label} reference_selection disagrees with active epoch',
    )
    _require(
        stage_doc.get('build_receipt') == str(build_ctx['build_root'])
        and stage_doc.get('build_complete') is True
        and stage_doc.get('execution_complete') is True
        and stage_doc.get('product_identity_unchanged') is True
        and stage_doc.get('complete_receipts_valid') is True
        and stage_doc.get('goal_complete') is False
        and stage_doc.get('compatibility_accepted') is False
        and stage_doc.get('performance_accepted') is False,
        'stage-postflight-failed',
        f'{label} failed stage completion/product-preservation postflight invariants',
    )
    _, identity_pin, _ = registry.stream_pin(
        stage_doc.get('identity_receipt'),
        label=f'{label}/product-identities.json',
    )
    context_file = stage_path.parent / 'stable-stage-context.json'
    if os.path.lexists(context_file):
        _, _, ctx_doc = registry.read_json(
            context_file,
            max_bytes=4 * 1024 * 1024,
            label=f'{label}/stable-stage-context.json',
        )
        _require(
            isinstance(ctx_doc, dict)
            and ctx_doc.get('schema') == 'femacs-stable-stage-context-v1'
            and ctx_doc.get('build_receipt') == str(build_ctx['build_root'])
            and ctx_doc.get('source_manifest_sha256')
            == build_ctx['source_export']['source_manifest_sha256']  # type: ignore[index]
            and ctx_doc.get('products_sha256') == ref_ctx['active_products_sha256']
            and ctx_doc.get('reference_selection') == ref_ctx['summary'],
            'stage-context-mismatch',
            f'{label}/stable-stage-context.json disagrees with build/source/products/epoch',
        )
    if label in {'semantics', 'performance'}:
        _verify_authored_harness(registry, stage_path, stage_doc, build_ctx)
    return identity_pin


def _audit_semantics(
    registry: InputPinRegistry,
    semantics_path: Path,
    semantics_doc: dict[str, object],
    semantics_pin: dict[str, object],
    build_ctx: dict[str, object],
    ref_ctx: dict[str, object],
) -> dict[str, object]:
    _require(
        semantics_doc.get('schema') == STAGE_SCHEMA
        and semantics_doc.get('action') == 'execute'
        and semantics_doc.get('execution_phase') in ('semantics', 'all')
        and semantics_doc.get('original_ert_reused') is False
        and semantics_doc.get('passed') is True,
        'semantics-not-passed',
        'semantics must be a full non-followup passing execute stage',
    )
    identity_pin = _verify_stage_postflight_and_context(
        registry, semantics_path, semantics_doc, build_ctx, ref_ctx, 'semantics'
    )
    cases = semantics_doc.get('cases')
    _require(isinstance(cases, list) and len(cases) >= 4, 'semantics-missing-actions', 'semantics missing action rows')
    assert isinstance(cases, list)
    by_action: dict[str, dict[str, object]] = {}
    ordered_semantics: list[str] = []
    for row in cases:
        _require(isinstance(row, dict), 'invalid-semantics-row', 'semantics case row must be object')
        assert isinstance(row, dict)
        row_id = row.get('id')
        if row_id in SEMANTICS_ACTIONS:
            _require(row_id not in by_action, 'duplicate-semantics-action', f'duplicate semantics action {row_id}')
            assert isinstance(row_id, str)
            by_action[row_id] = row
            ordered_semantics.append(row_id)
    _require(
        tuple(ordered_semantics) == SEMANTICS_ACTIONS,
        'semantics-action-order-mismatch',
        f'semantics must execute exact actions {SEMANTICS_ACTIONS}, got {ordered_semantics}',
    )
    action_summaries: dict[str, object] = {}
    for action_id in SEMANTICS_ACTIONS:
        row = by_action[action_id]
        _require(
            row.get('passed') is True
            and row.get('execution_complete') is True
            and 'receipt_validation_error' not in row
            and 'affected_merge' not in row
            and _is_hex64(row.get('completion_sha256'))
            and _is_hex64(row.get('receipt_sha256')),
            'semantics-action-failed',
            f'semantics action {action_id} failed, reused partial ERT, or lacks receipt digests',
        )
        _validate_process_record(row.get('process'), f'semantics.{action_id}')
        receipt_file = _normalize_path_text(row.get('receipt'), f'semantics.{action_id}.receipt')
        completion_file = _normalize_path_text(
            row.get('completion_receipt'), f'semantics.{action_id}.completion_receipt'
        )
        action_dir = receipt_file.parent
        _require(
            receipt_file.name == 'results.json'
            and completion_file == action_dir / 'completion.json',
            'semantics-action-locator-mismatch',
            f'semantics.{action_id} completion/results paths must share owned action directory',
        )
        verified = _verify_execution_completion_dir(
            registry,
            action_dir,
            action_id,
            str(ref_ctx['active_products_sha256']),
            expected_completion_sha=row['completion_sha256'],
            expected_detailed_sha=row['receipt_sha256'],
            stream_detailed_only=True,
        )
        counts = verified['completion'].get('counts')  # type: ignore[union-attr]
        _require(
            isinstance(counts, dict) and counts == row.get('counts'),
            'semantics-counts-mismatch',
            f'semantics.{action_id} counts disagree between stage and completion.json',
        )
        assert isinstance(counts, dict)
        if action_id == 'ert':
            for group_key in ('cases', 'discoveries', 'ert_tests'):
                bucket = counts.get(group_key)
                _require(
                    isinstance(bucket, dict)
                    and type(bucket.get('total')) is int
                    and bucket['total'] > 0
                    and bucket.get('passed') == bucket['total']
                    and bucket.get('failed') == 0,
                    'ert-counts-failed',
                    f'semantics.ert {group_key} must be non-empty with zero failures',
                )
        elif action_id == 'negative':
            bucket = counts.get('cases')
            _require(
                isinstance(bucket, dict)
                and type(bucket.get('total')) is int
                and bucket['total'] >= EXPECTED_NATIVE_CANDIDATES * 2
                and bucket.get('passed') == bucket['total']
                and bucket.get('failed') == 0,
                'negative-counts-failed',
                'semantics.negative cases incomplete or failed',
            )
        elif action_id == 'dump':
            bucket = counts.get('products')
            _require(
                isinstance(bucket, dict)
                and bucket == {
                    'total': EXPECTED_DUMP_CANDIDATES,
                    'passed': EXPECTED_DUMP_CANDIDATES,
                    'failed': 0,
                },
                'dump-counts-failed',
                f'semantics.dump must pass all {EXPECTED_DUMP_CANDIDATES} collector/native products',
            )
        elif action_id == 'relocation':
            bucket = counts.get('products')
            _require(
                isinstance(bucket, dict)
                and bucket == {
                    'total': EXPECTED_CANDIDATES,
                    'passed': EXPECTED_CANDIDATES,
                    'failed': 0,
                },
                'relocation-counts-failed',
                f'semantics.relocation must pass all {EXPECTED_CANDIDATES} candidate products',
            )
        action_summaries[action_id] = {
            'counts': counts,
            'completion_sha256': row['completion_sha256'],
            'detailed_sha256': row['receipt_sha256'],
        }
    return {
        'passed': True,
        'receipt': semantics_pin,
        'identity_receipt': identity_pin,
        'actions': action_summaries,
    }


def _expected_stable_env(product: dict[str, object], explicit_libraries: bool) -> dict[str, str]:
    env = {'FEMACS_GC_OBSERVATION': 'off'}
    if product['kind'] != 'reference':
        collector = str(product['collector'])
        env['FEMACS_GC_COLLECTOR'] = 'major' if collector == 'off' else collector
    if explicit_libraries:
        roots = product['resource_roots']
        assert isinstance(roots, dict)
        if 'lisp' in roots:
            lisp_root = str(Path(os.path.abspath(str(roots['lisp']))))
        elif product['kind'] == 'cocoa' and 'bundle' in roots:
            lisp_root = str(Path(os.path.abspath(str(Path(str(roots['bundle'])) / 'lisp'))))
        else:
            raise AuditError('missing-lisp-root', f'profile {product["id"]} lacks Lisp resource root')
        env['FEMACS_STABLE_PRODUCT_LISP'] = lisp_root
    return env


def _audit_stable_raw_stages(
    registry: InputPinRegistry,
    stable_stages: list[tuple[Path, dict[str, object], dict[str, object]]],
    build_ctx: dict[str, object],
    ref_ctx: dict[str, object],
    pending: list[str],
) -> dict[str, object]:
    if not stable_stages:
        pending.append('stable_raw_stage_results_missing')
        raise AuditError('missing-stable-stages', 'at least one --stable raw stage receipt is required')
    ordinary_set = set(ref_ctx['matrix']['ordinary_ids'])  # type: ignore[index]
    by_id = ref_ctx['matrix']['by_id']  # type: ignore[index]
    assert isinstance(by_id, dict)
    reference_profile = by_id['installed-reference']
    assert isinstance(reference_profile, dict)
    expected_manifest_sha = str(build_ctx['corpus_manifest_sha256'])
    stage_pin_map: dict[str, dict[str, int | str]] = {}
    pair_records: dict[str, dict[str, object]] = {}
    for stage_path, stage_pin, stage_doc in stable_stages:
        key_str = str(stage_path)
        _require(key_str not in stage_pin_map, 'duplicate-stable-stage', f'duplicate --stable input {key_str}')
        stage_pin_map[key_str] = {
            'bytes': int(stage_pin['bytes']),  # type: ignore[arg-type]
            'sha256': str(stage_pin['sha256']),
        }
        _require(
            stage_doc.get('schema') == STAGE_SCHEMA
            and stage_doc.get('action') == 'execute'
            and stage_doc.get('execution_phase') in ('stable', 'all'),
            'invalid-stable-stage',
            f'stable stage {key_str} must be execute phase stable/all',
        )
        _verify_stage_postflight_and_context(
            registry, stage_path, stage_doc, build_ctx, ref_ctx, f'stable({stage_path.parent.name})'
        )
        explicit_libs = stage_doc.get('stable_product_libraries', False)
        _require(type(explicit_libs) is bool, 'invalid-stable-libs-flag', 'stable_product_libraries must be bool')
        if explicit_libs:
            harness_sel = stage_doc.get('stable_harness_selection')
            _require(
                isinstance(harness_sel, dict) and harness_sel.get('unchanged') is True,
                'stable-harness-changed',
                f'{key_str} explicit stable harness selection missing or changed',
            )
        runner_sel = stage_doc.get('stable_runner_selection')
        if runner_sel is not None:
            _require(
                isinstance(runner_sel, dict) and runner_sel.get('unchanged') is True,
                'stable-runner-changed',
                f'{key_str} stable_runner_selection changed during execution',
            )
        reuse_sel = stage_doc.get('stable_file_reuse')
        if reuse_sel is not None:
            _require(
                isinstance(reuse_sel, dict)
                and reuse_sel.get('unchanged') is True
                and _is_nonempty_str(reuse_sel.get('path')),
                'stable-reuse-changed',
                f'{key_str} stable_file_reuse selection changed during execution',
            )
            assert isinstance(reuse_sel, dict)
            _, _, reuse_doc = registry.read_json(
                reuse_sel['path'],
                max_bytes=MAX_SMALL_JSON_BYTES,
                label='stable-file-reuse-selection.json',
            )
            _require(
                isinstance(reuse_doc, dict)
                and reuse_doc.get('schema') == 'femacs-stable-file-reuse-admission-v1'
                and reuse_doc.get('reference_selection') == ref_ctx['summary']
                and reuse_doc.get('raw_results_modified') is False,
                'invalid-stable-reuse-admission',
                f'{key_str} stable-file-reuse-selection.json invalid or wrong epoch',
            )
        cases = stage_doc.get('cases')
        _require(isinstance(cases, list) and bool(cases), 'empty-stable-stage', f'{key_str} has no cases')
        assert isinstance(cases, list)
        stage_pairs = 0
        for row in cases:
            _require(isinstance(row, dict), 'invalid-stable-row', f'{key_str} case must be object')
            assert isinstance(row, dict)
            row_id = row.get('id', '')
            if not isinstance(row_id, str) or not row_id.startswith('stable-'):
                continue
            stage_pairs += 1
            profile_id = row_id[len('stable-'):]
            _require(
                profile_id in ordinary_set and profile_id not in pair_records,
                'duplicate-or-invalid-stable-profile',
                f'duplicate, unknown, or diagnostic stable profile {profile_id!r}',
            )
            _require(
                row.get('execution_complete') is True
                and 'receipt_validation_error' not in row
                and _is_hex64(row.get('completion_sha256'))
                and _is_hex64(row.get('receipt_sha256'))
                and type(row.get('corpus_complete')) is bool
                and all(type(row.get(k)) is int and row[k] >= 0 for k in ('case_count', 'passed_count', 'failure_count')),
                'incomplete-stable-pair',
                f'stable-{profile_id} incomplete or missing completion/detailed digests',
            )
            corpus_complete = bool(row['corpus_complete'])
            expected_exit = 0 if corpus_complete else 1
            _validate_process_record(row.get('process'), f'stable-{profile_id}', expected_exit=expected_exit)
            _require(
                row.get('passed') is corpus_complete,
                'stable-row-pass-flag-mismatch',
                f'stable-{profile_id} passed flag disagrees with raw corpus_complete',
            )
            detailed_path = _normalize_path_text(row.get('receipt'), f'stable-{profile_id}.receipt')
            pair_root = detailed_path.parent
            _require(
                detailed_path.name == 'stable-suite-run.json',
                'invalid-stable-detailed-name',
                f'stable-{profile_id} receipt must point to stable-suite-run.json',
            )
            _, comp_pin, comp_doc = registry.read_json(
                pair_root / 'stable-suite-completion.json',
                max_bytes=16 * 1024 * 1024,
                expected=row['completion_sha256'],
                label=f'stable-{profile_id}/stable-suite-completion.json',
            )
            _require(isinstance(comp_doc, dict), 'invalid-stable-completion', f'stable-{profile_id} completion not object')
            assert isinstance(comp_doc, dict)
            candidate_profile = by_id[profile_id]
            assert isinstance(candidate_profile, dict)
            expected_envs = {
                'reference': _expected_stable_env(reference_profile, explicit_libs),
                'candidate': _expected_stable_env(candidate_profile, explicit_libs),
            }
            _require(
                comp_doc.get('schema') == STABLE_COMPLETION_SCHEMA
                and comp_doc.get('result_schema') == STABLE_RUN_SCHEMA
                and comp_doc.get('selector') == 'all-discovered-ids'
                and comp_doc.get('manifest_sha256') == expected_manifest_sha
                and comp_doc.get('execution_complete') is True
                and comp_doc.get('identities_unchanged') is True
                and comp_doc.get('corpus_inputs_unchanged') is True
                and comp_doc.get('full_compatibility_claim') is False
                and comp_doc.get('entrypoint_count') == EXPECTED_STABLE_ENTRYPOINTS
                and comp_doc.get('file_count') == EXPECTED_STABLE_ENTRYPOINTS
                and isinstance(comp_doc.get('products_before'), dict)
                and comp_doc.get('products_before') == comp_doc.get('products_after')
                and comp_doc.get('runtime_environment') == expected_envs
                and comp_doc.get('corpus_complete') is corpus_complete
                and comp_doc.get('case_count') == row['case_count']
                and comp_doc.get('passed_count') == row['passed_count']
                and comp_doc.get('failure_count') == row['failure_count']
                and row['case_count'] > 0,
                'stable-completion-mismatch',
                f'stable-{profile_id} completion failed 555-entrypoint, identity, env, or raw count invariants',
            )
            if corpus_complete:
                _require(
                    row['failure_count'] == 0 and row['passed_count'] == row['case_count'],
                    'false-stable-corpus-complete',
                    f'stable-{profile_id} claims corpus_complete with failures or non-passing cases',
                )
            detailed_ref = comp_doc.get('detailed')
            _require(
                isinstance(detailed_ref, dict)
                and detailed_ref.get('path') == 'stable-suite-run.json'
                and detailed_ref.get('sha256') == row['receipt_sha256']
                and type(detailed_ref.get('bytes')) is int
                and detailed_ref['bytes'] > 0,
                'stable-detailed-ref-mismatch',
                f'stable-{profile_id} detailed binding disagrees with stage receipt_sha256',
            )
            assert isinstance(detailed_ref, dict)
            _, detailed_pin, _ = registry.stream_pin(
                detailed_path,
                expected={'bytes': detailed_ref['bytes'], 'sha256': detailed_ref['sha256']},
                label=f'stable-{profile_id}/stable-suite-run.json',
            )
            file_reuse_ref = row.get('file_reuse')
            if file_reuse_ref is not None:
                _require(
                    isinstance(file_reuse_ref, dict)
                    and _normalize_path_text(
                        file_reuse_ref.get('path'), f'stable-{profile_id}.file_reuse.path'
                    )
                    == pair_root / 'stable-file-reuse.json'
                    and _is_hex64(file_reuse_ref.get('sha256')),
                    'invalid-stable-file-reuse-ref',
                    f'stable-{profile_id} file_reuse reference invalid',
                )
                assert isinstance(file_reuse_ref, dict)
                _, _, pair_reuse_doc = registry.read_json(
                    pair_root / 'stable-file-reuse.json',
                    max_bytes=16 * 1024 * 1024,
                    expected=file_reuse_ref['sha256'],
                    label=f'stable-{profile_id}/stable-file-reuse.json',
                )
                expected_sides = 2 * EXPECTED_STABLE_ENTRYPOINTS
                _require(
                    isinstance(pair_reuse_doc, dict)
                    and pair_reuse_doc.get('schema') == 'femacs-stable-file-reuse-v1'
                    and pair_reuse_doc.get('raw_results_modified') is False
                    and pair_reuse_doc.get('physical_retirement_performed_by_this_run') is False
                    and pair_reuse_doc.get('retired_logical_bytes_attributed_to_this_run') == 0
                    and file_reuse_ref.get('retired_logical_bytes_attributed_to_this_run') == 0
                    and pair_reuse_doc.get('module_file_always_executed_fresh') is True
                    and type(pair_reuse_doc.get('count')) is int
                    and pair_reuse_doc['count'] >= 0
                    and type(pair_reuse_doc.get('fresh_file_executions')) is int
                    and pair_reuse_doc['fresh_file_executions'] >= 0
                    and pair_reuse_doc.get('covered_file_sides') == expected_sides
                    and file_reuse_ref.get('covered_file_sides') == expected_sides
                    and pair_reuse_doc['count'] == file_reuse_ref.get('reused_file_sides')
                    and pair_reuse_doc['fresh_file_executions'] == file_reuse_ref.get('fresh_file_executions')
                    and pair_reuse_doc['count'] + pair_reuse_doc['fresh_file_executions'] == expected_sides
                    and (reuse_sel is not None or pair_reuse_doc['count'] == 0),
                    'invalid-stable-pair-reuse-accounting',
                    f'stable-{profile_id} stable-file-reuse.json failed side-count or zero-reclaim invariants',
                )
            pair_records[profile_id] = {
                'stage_path': key_str,
                'completion_pin': comp_pin,
                'detailed_pin': detailed_pin,
                'completion_sha256': comp_pin['sha256'],
                'detailed_sha256': detailed_pin['sha256'],
                'corpus_complete': corpus_complete,
                'case_count': row['case_count'],
                'passed_count': row['passed_count'],
                'failure_count': row['failure_count'],
            }
        _require(stage_pairs > 0, 'no-stable-pairs-in-stage', f'{key_str} contained zero stable-* pairs')
    missing_profiles = sorted(ordinary_set - set(pair_records))
    if missing_profiles:
        pending.append(
            f'stable_raw_missing_profiles:{len(missing_profiles)}/{EXPECTED_ORDINARY_CANDIDATES}'
        )
        raise AuditError(
            'incomplete-stable-coverage',
            f'raw --stable stages cover {len(pair_records)}/{EXPECTED_ORDINARY_CANDIDATES} '
            f'ordinary candidate profiles; missing: {", ".join(missing_profiles[:8])}',
        )
    return {
        'passed': True,
        'stage_inputs': stage_pin_map,
        'profile_count': len(pair_records),
        'entrypoints_per_profile': EXPECTED_STABLE_ENTRYPOINTS,
        'pairs': pair_records,
    }


def _audit_classified_stable(
    registry: InputPinRegistry,
    classified_doc: dict[str, object],
    classified_pin: dict[str, object],
    build_ctx: dict[str, object],
    ref_ctx: dict[str, object],
    stable_ctx: dict[str, object] | None,
) -> dict[str, object]:
    if classified_doc.get('schema') == 'femacs-stable-catalog-assembly-v1':
        raise AuditError(
            'classified-stable-wrong-receipt-layer',
            'received catalog-assembly receipt; pass accounting/results.json (femacs-stable-classified-stage-v1)',
        )
    _require(
        classified_doc.get('schema') == CLASSIFIED_STAGE_SCHEMA
        and classified_doc.get('passed') is True
        and classified_doc.get('requested_scope_accepted') is True
        and classified_doc.get('classified_stable_phase_complete') is True
        and classified_doc.get('required_candidate_count') == EXPECTED_ORDINARY_CANDIDATES
        and classified_doc.get('remaining_profiles') == []
        and classified_doc.get('postflight_errors') == []
        and classified_doc.get('compiler_processes_started') == 0
        and classified_doc.get('editor_processes_started') == 0
        and classified_doc.get('original_results_modified') is False
        and classified_doc.get('raw_failures_promoted') is False
        and classified_doc.get('compatibility_accepted') is False
        and classified_doc.get('goal_complete') is False,
        'classified-stable-not-passed',
        'classified_stable stage failed full-48 completion or non-promotion invariants',
    )
    _require(
        ref_ctx['active'] is True and classified_doc.get('reference_selection') == ref_ctx['summary'],
        'classified-stable-epoch-mismatch',
        'classified_stable requires an active matching reference_selection epoch',
    )
    checks = classified_doc.get('checks')
    _require(
        isinstance(checks, dict)
        and set(checks) == set(CLASSIFIED_POSTFLIGHT_CHECKS)
        and all(checks.get(k) is True for k in CLASSIFIED_POSTFLIGHT_CHECKS),
        'classified-stable-checks-failed',
        'classified_stable postflight checks are incomplete or failed',
    )
    _require(
        stable_ctx is not None,
        'classified-stable-missing-raw-stages',
        'classified_stable cannot be verified without matching --stable raw stage inputs',
    )
    assert stable_ctx is not None
    expected_stage_inputs = stable_ctx['stage_inputs']
    actual_stage_inputs = classified_doc.get('original_stage_inputs')
    _require(
        isinstance(actual_stage_inputs, dict) and actual_stage_inputs == expected_stage_inputs,
        'classified-stage-inputs-mismatch',
        'classified_stable original_stage_inputs does not exactly match supplied --stable file SHA/bytes',
    )
    catalog_ref = classified_doc.get('classification_catalog')
    _require(
        isinstance(catalog_ref, dict) and _is_nonempty_str(catalog_ref.get('path')),
        'missing-classification-catalog',
        'classified_stable missing classification_catalog binding',
    )
    assert isinstance(catalog_ref, dict)
    _, catalog_pin, catalog_doc = registry.read_json(
        catalog_ref['path'],
        max_bytes=64 * 1024 * 1024,
        expected={'bytes': catalog_ref.get('bytes'), 'sha256': catalog_ref.get('sha256')},
        label='classification-catalog.json',
    )
    _require(
        isinstance(catalog_doc, dict)
        and set(catalog_doc) == {'schema', 'manifest_sha256', 'reference_selection_sha256', 'contexts', 'rules'}
        and catalog_doc.get('schema') == CLASSIFICATION_CATALOG_SCHEMA
        and catalog_doc.get('manifest_sha256') == build_ctx['corpus_manifest_sha256']
        and catalog_doc.get('reference_selection_sha256') == ref_ctx['summary']['sha256'],  # type: ignore[index]
        'invalid-classification-catalog',
        'classification catalog schema, manifest_sha256, or reference_selection_sha256 mismatch',
    )
    expected_ordinary = ref_ctx['matrix']['ordinary_ids']  # type: ignore[index]
    requested_profiles = classified_doc.get('requested_profiles')
    _require(
        isinstance(requested_profiles, list)
        and len(requested_profiles) == EXPECTED_ORDINARY_CANDIDATES
        and set(requested_profiles) == set(expected_ordinary),
        'classified-profile-set-mismatch',
        'classified_stable requested_profiles does not match the 48 ordinary candidates',
    )
    cases = classified_doc.get('cases')
    _require(
        isinstance(cases, list) and len(cases) == EXPECTED_ORDINARY_CANDIDATES,
        'classified-case-count-mismatch',
        f'classified_stable must contain {EXPECTED_ORDINARY_CANDIDATES} profile rows',
    )
    assert isinstance(cases, list)
    raw_pairs = stable_ctx['pairs']
    assert isinstance(raw_pairs, dict)
    seen_profiles: set[str] = set()
    total_raw_cases = 0
    total_raw_passed = 0
    total_raw_failures = 0
    total_classified_cases = 0
    for row in cases:
        _require(isinstance(row, dict), 'invalid-classified-row', 'classified_stable row must be object')
        assert isinstance(row, dict)
        prof = row.get('profile')
        _require(
            isinstance(prof, str) and prof in raw_pairs and prof not in seen_profiles,
            'invalid-classified-profile',
            f'duplicate or unexpected classified profile {prof!r}',
        )
        assert isinstance(prof, str)
        seen_profiles.add(prof)
        raw_info = raw_pairs[prof]
        assert isinstance(raw_info, dict)
        _require(
            row.get('passed') is True
            and 'error' not in row
            and row.get('raw_corpus_complete') is raw_info['corpus_complete']
            and row.get('raw_passed_count') == raw_info['passed_count']
            and row.get('raw_failure_count') == raw_info['failure_count'],
            'classified-row-raw-drift',
            f'classified_stable row {prof} failed or altered raw stable counts',
        )
        raw_embedded = row.get('raw_result')
        _require(
            isinstance(raw_embedded, dict)
            and raw_embedded.get('completion_sha256') == raw_info['completion_sha256']
            and raw_embedded.get('receipt_sha256') == raw_info['detailed_sha256'],
            'classified-row-receipt-mismatch',
            f'classified_stable row {prof} raw_result disagrees with supplied --stable SHA',
        )
        ledger_ref = row.get('classification')
        _require(
            isinstance(ledger_ref, dict) and _is_nonempty_str(ledger_ref.get('path')),
            'missing-classified-ledger',
            f'classified_stable row {prof} missing per-profile ledger reference',
        )
        assert isinstance(ledger_ref, dict)
        _, _, ledger = registry.read_json(
            ledger_ref['path'],
            max_bytes=MAX_SMALL_JSON_BYTES,
            expected={'bytes': ledger_ref.get('bytes'), 'sha256': ledger_ref.get('sha256')},
            label=f'classified-stable/{prof}.json',
        )
        _require(isinstance(ledger, dict), 'invalid-classified-ledger', f'{prof}.json must be object')
        assert isinstance(ledger, dict)
        classified_count = ledger.get('classified_case_count')
        _require(
            ledger.get('schema') == CLASSIFIED_LEDGER_SCHEMA
            and ledger.get('catalog_identity') == {
                'bytes': catalog_pin['bytes'],
                'sha256': catalog_pin['sha256'],
            }
            and ledger.get('profile') == prof
            and ledger.get('reference_epoch') == ref_ctx['epoch']
            and ledger.get('raw_completion_sha256') == raw_info['completion_sha256']
            and ledger.get('raw_detailed_sha256') == raw_info['detailed_sha256']
            and ledger.get('original_file_count') == EXPECTED_STABLE_ENTRYPOINTS
            and ledger.get('raw_case_count') == raw_info['case_count']
            and ledger.get('raw_passed_count') == raw_info['passed_count']
            and ledger.get('raw_failure_count') == raw_info['failure_count']
            and type(classified_count) is int
            and classified_count >= 0
            and ledger.get('blocking_case_failure_count') == 0
            and ledger.get('structural_failure_count') == 0
            and ledger.get('blocking_cases') == []
            and ledger.get('structural_failures') == []
            and isinstance(ledger.get('classified_cases'), list)
            and len(ledger['classified_cases']) == classified_count
            and raw_info['case_count'] == raw_info['passed_count'] + classified_count
            and raw_info['failure_count'] == classified_count
            and ledger.get('classified_corpus_accepted') is True
            and ledger.get('raw_results_modified') is False
            and ledger.get('classified_cases_counted_as_passed') is False
            and ledger.get('retained_failure_state_removed') is False
            and ledger.get('compatibility_accepted') is False
            and ledger.get('goal_complete') is False,
            'classified-ledger-failed',
            f'classified ledger {prof}.json failed conservation, zero-blocking, or immutability invariants',
        )
        assert isinstance(classified_count, int)
        total_raw_cases += int(raw_info['case_count'])  # type: ignore[arg-type]
        total_raw_passed += int(raw_info['passed_count'])  # type: ignore[arg-type]
        total_raw_failures += int(raw_info['failure_count'])  # type: ignore[arg-type]
        total_classified_cases += classified_count
    return {
        'passed': True,
        'receipt': classified_pin,
        'catalog': catalog_pin,
        'profile_count': len(seen_profiles),
        'totals': {
            'raw_case_count': total_raw_cases,
            'raw_passed_count': total_raw_passed,
            'raw_failure_count': total_raw_failures,
            'classified_case_count': total_classified_cases,
        },
    }


def _expected_performance_controls(
    products: list[dict[str, object]],
    *,
    permanent_full_fallback: bool,
) -> list[tuple[str, str, str]]:
    controls: list[tuple[str, str, str]] = []
    for product in products:
        if product['test_hooks']:
            continue
        modes = (
            ('interpreted', 'bytecode', 'native')
            if (product['native'] and not permanent_full_fallback)
            else ('interpreted', 'bytecode')
        )
        observers = (
            ('off', 'phases', 'survival', 'all')
            if (product['observer'] and not permanent_full_fallback)
            else ('off',)
        )
        for mode in modes:
            for obs in observers:
                controls.append((str(product['id']), mode, obs))
    return controls


def _audit_single_performance_mode(
    registry: InputPinRegistry,
    action_dir: Path,
    stage_row: dict[str, object],
    ref_ctx: dict[str, object],
    *,
    permanent_full_fallback: bool,
    pending: list[str],
) -> dict[str, object]:
    mode_label = 'performance-fallback' if permanent_full_fallback else 'performance'
    verified = _verify_execution_completion_dir(
        registry,
        action_dir,
        'performance',
        str(ref_ctx['active_products_sha256']),
        expected_completion_sha=stage_row['completion_sha256'],
        expected_detailed_sha=stage_row['receipt_sha256'],
        stream_detailed_only=False,
        expected_fallback=permanent_full_fallback,
    )
    detailed_ref = verified['detailed_ref']
    assert isinstance(detailed_ref, dict)
    active_products = ref_ctx['active_products']
    assert isinstance(active_products, list)
    by_id = ref_ctx['matrix']['by_id']  # type: ignore[index]
    assert isinstance(by_id, dict)
    expected_diagnostic = set(ref_ctx['matrix']['diagnostic_ids'])  # type: ignore[index]
    non_hook_ids = {p['id'] for p in active_products if not p['test_hooks']}
    _require(len(non_hook_ids) == EXPECTED_NON_HOOK_PROFILES, 'invalid-non-hook-count', 'expected 49 non-hook profiles')
    expected_controls = _expected_performance_controls(
        active_products, permanent_full_fallback=permanent_full_fallback
    )
    expected_process_slots = {
        (prof, mode, obs, rnd, rnd <= 0)
        for prof, mode, obs in expected_controls
        for rnd in range(-1, 8)
    }
    expected_startup_slots = (
        set()
        if permanent_full_fallback
        else {(str(pid), rnd, rnd <= 0) for pid in non_hook_ids for rnd in range(-1, 8)}
    )
    seen_process_slots: set[tuple[str, str, str, int, bool]] = set()
    seen_startup_slots: set[tuple[str, int, bool]] = set()
    excluded_profiles: list[str] = []
    process_count = 0
    process_warmup_count = 0
    sample_count = 0
    startup_count = 0
    error_count = 0
    comparison_count = 0
    eligible_comparison_count = 0
    fields: dict[str, object] = {}
    detailed_pin: dict[str, object] | None = None
    stream_arrays = frozenset({'processes', 'samples', 'startups', 'comparisons', 'errors', 'excluded'})
    for kind, key, item in registry.stream_json_object(
        action_dir / 'results.json',
        stream_arrays=stream_arrays,
        max_bytes=MAX_PERFORMANCE_BYTES,
        expected={'bytes': detailed_ref.get('bytes'), 'sha256': detailed_ref.get('sha256')},
        label=f'{mode_label}/results.json',
    ):
        if kind == 'pin':
            assert isinstance(item, dict)
            detailed_pin = item
            continue
        if kind == 'field':
            fields[key] = item
            continue
        if kind == 'array-end':
            continue
        if key == 'errors':
            error_count += 1
        elif key == 'excluded':
            _require(
                isinstance(item, dict)
                and set(item) == {'profile', 'reason'}
                and isinstance(item.get('profile'), str)
                and item.get('reason') == 'diagnostic test-hooks product',
                'invalid-performance-excluded',
                f'{mode_label} malformed excluded diagnostic entry',
            )
            assert isinstance(item, dict)
            excluded_profiles.append(str(item['profile']))
        elif key == 'processes':
            process_count += 1
            _require(isinstance(item, dict), 'invalid-performance-process', f'{mode_label} process row not object')
            assert isinstance(item, dict)
            _validate_process_record(item, f'{mode_label}.processes[{process_count - 1}]')
            prof = item.get('profile')
            mode = item.get('mode')
            obs = item.get('observer')
            rnd = item.get('round')
            warmup = item.get('warmup')
            _require(
                isinstance(prof, str)
                and isinstance(mode, str)
                and isinstance(obs, str)
                and type(rnd) is int
                and type(warmup) is bool
                and item.get('passed') is True
                and type(item.get('pid')) is int
                and item['pid'] > 0
                and item.get('execution_identity') == f'{prof}:{obs}'
                and item.get('memory_sampling_complete') is True
                and type(item.get('peak_accounted_bytes')) is int
                and item['peak_accounted_bytes'] > 0
                and type(item.get('peak_rss_bytes')) is int
                and item['peak_rss_bytes'] > 0
                and item.get('source_identity_before') == item.get('source_identity_after')
                and isinstance(item.get('source_identity_before'), dict),
                'synthetic-or-incomplete-performance-process',
                f'{mode_label} process row {process_count - 1} lacks real PID, memory sampling, or unchanged product identity',
            )
            assert isinstance(prof, str) and isinstance(mode, str) and isinstance(obs, str)
            assert type(rnd) is int and type(warmup) is bool
            slot = (prof, mode, obs, rnd, warmup)
            _require(
                slot in expected_process_slots and slot not in seen_process_slots,
                'unexpected-performance-process-slot',
                f'{mode_label} unexpected or duplicate process slot {slot}',
            )
            seen_process_slots.add(slot)
            if warmup:
                process_warmup_count += 1
            obs_meta = item.get('runtime_observations')
            workloads = item.get('workloads')
            _require(
                isinstance(obs_meta, dict)
                and obs_meta.get('schema') == 'femacs-runtime-performance-observation-v1'
                and obs_meta.get('mode') == mode
                and isinstance(workloads, list)
                and bool(workloads)
                and all(
                    isinstance(w, dict)
                    and w.get('execution_mode') == mode
                    and w.get('actual_function_kind') == mode
                    for w in workloads
                ),
                'invalid-performance-workloads',
                f'{mode_label} process {slot} missing verified runtime_observations/workloads',
            )
            if permanent_full_fallback:
                fb_ctx = item.get('fallback_context')
                mod_err = fb_ctx.get('module_error') if isinstance(fb_ctx, dict) else None
                _require(
                    isinstance(fb_ctx, dict)
                    and fb_ctx.get('schema') == 'femacs-runtime-performance-fallback-v1'
                    and isinstance(mod_err, dict)
                    and mod_err.get('symbol') == 'module-open-failed'
                    and mod_err.get('path_matched') is True
                    and mod_err.get('message_nonempty') is True,
                    'missing-fallback-context',
                    f'{mode_label} process {slot} missing verified failed-module-load fallback_context',
                )
                assert isinstance(fb_ctx, dict)
                product_meta = by_id[prof]
                assert isinstance(product_meta, dict)
                if product_meta['collector'] != 'off':
                    after_status = fb_ctx.get('after')
                    latched = fb_ctx.get('latched')
                    _require(
                        isinstance(after_status, list)
                        and len(after_status) == 3
                        and isinstance(after_status[1], list)
                        and len(after_status[1]) == 24
                        and after_status[1][14] == 1
                        and after_status[1][1] == 0
                        and isinstance(latched, list)
                        and len(latched) == 3
                        and isinstance(latched[1], list)
                        and len(latched[1]) == 24
                        and after_status[1][4] == latched[1][4],
                        'collector-fallback-not-retained',
                        f'{mode_label} collector profile {prof} did not retain permanent Full fallback',
                    )
        elif key == 'startups':
            startup_count += 1
            _require(not permanent_full_fallback, 'unexpected-fallback-startup', 'fallback mode must not measure startups')
            _require(isinstance(item, dict), 'invalid-startup-row', 'startup row must be object')
            assert isinstance(item, dict)
            _validate_process_record(item, f'{mode_label}.startups[{startup_count - 1}]')
            prof = item.get('profile')
            rnd = item.get('round')
            warmup = item.get('warmup')
            _require(
                isinstance(prof, str)
                and type(rnd) is int
                and type(warmup) is bool
                and item.get('passed') is True
                and item.get('mode') == 'startup'
                and item.get('observer') == 'off'
                and item.get('execution_identity') == f'{prof}:off'
                and type(item.get('pid')) is int
                and item['pid'] > 0
                and item.get('memory_sampling_complete') is True
                and type(item.get('peak_accounted_bytes')) is int
                and item['peak_accounted_bytes'] > 0
                and item.get('exit_timing_complete') is True
                and item.get('exit_timing_method') == 'spawn-to-reaped-exit-waiter'
                and _is_finite_nonneg_num(item.get('process_lifetime_seconds'))
                and float(item['process_lifetime_seconds']) > 0.0
                and item.get('source_identity_before') == item.get('source_identity_after'),
                'invalid-startup-measurement',
                f'{mode_label} startup row {startup_count - 1} failed exit-timing or memory invariants',
            )
            assert isinstance(prof, str) and type(rnd) is int and type(warmup) is bool
            s_slot = (prof, rnd, warmup)
            _require(
                s_slot in expected_startup_slots and s_slot not in seen_startup_slots,
                'unexpected-startup-slot',
                f'{mode_label} duplicate or unexpected startup slot {s_slot}',
            )
            seen_startup_slots.add(s_slot)
        elif key == 'samples':
            sample_count += 1
            _require(
                isinstance(item, dict)
                and item.get('correctness') is True
                and _is_finite_nonneg_num(item.get('wall_ms'))
                and type(item.get('round')) is int
                and 1 <= item['round'] <= EXPECTED_MEASURED_ROUNDS
                and type(item.get('peak_accounted_bytes')) is int
                and item['peak_accounted_bytes'] > 0,
                'invalid-performance-sample',
                f'{mode_label} sample[{sample_count - 1}] failed correctness/wall_ms/round invariants',
            )
            assert isinstance(item, dict)
            if permanent_full_fallback:
                _require(
                    item.get('mode') in ('interpreted', 'bytecode')
                    and isinstance(item.get('execution_identity'), str)
                    and item['execution_identity'].endswith(':off'),
                    'invalid-fallback-sample-mode',
                    'fallback sample recorded non-fallback mode or observer',
                )
        elif key == 'comparisons':
            comparison_count += 1
            _require(
                isinstance(item, dict) and type(item.get('eligible')) is bool,
                'invalid-performance-comparison',
                f'{mode_label} comparison[{comparison_count - 1}] malformed',
            )
            assert isinstance(item, dict)
            if item['eligible'] is True:
                eligible_comparison_count += 1
    expected_top = (PERFORMANCE_TOP_KEYS | PERFORMANCE_FALLBACK_KEYS) - stream_arrays
    _require(
        set(fields) == expected_top
        and fields.get('schema') == EXECUTION_SCHEMA
        and fields.get('action') == 'performance'
        and fields.get('passed') is True
        and fields.get('fixtures_unchanged') is True
        and fields.get('full_product_inputs_unchanged') is True
        and fields.get('driver_sources_unchanged') is True
        and fields.get('state_retirement_complete') is True
        and fields.get('performance_accepted') is False
        and fields.get('compatibility_accepted') is False
        and fields.get('goal_complete') is False
        and fields.get('permanent_full_fallback') is permanent_full_fallback
        and fields.get('startup_measured') is (not permanent_full_fallback)
        and fields.get('limitation') == PERFORMANCE_LIMITATION,
        'invalid-performance-top-fields',
        f'{mode_label}/results.json failed top-level schema, mode, or non-promotion invariants',
    )
    _require(error_count == 0, 'performance-errors-present', f'{mode_label} recorded {error_count} errors')
    _require(
        len(excluded_profiles) == EXPECTED_DIAGNOSTIC_CANDIDATES
        and set(excluded_profiles) == expected_diagnostic,
        'performance-excluded-mismatch',
        f'{mode_label} excluded profiles do not match the 12 diagnostic test_hooks products',
    )
    _require(
        seen_process_slots == expected_process_slots
        and process_count == len(expected_controls) * EXPECTED_PERFORMANCE_ROUNDS
        and process_warmup_count == len(expected_controls) * EXPECTED_WARMUP_ROUNDS,
        'performance-process-matrix-incomplete',
        f'{mode_label} did not execute the complete {len(expected_controls)}x9 real process matrix',
    )
    _require(
        seen_startup_slots == expected_startup_slots
        and startup_count == (0 if permanent_full_fallback else EXPECTED_NON_HOOK_PROFILES * EXPECTED_PERFORMANCE_ROUNDS),
        'performance-startup-matrix-incomplete',
        f'{mode_label} startup process matrix incomplete ({startup_count})',
    )
    _require(sample_count > 0 and comparison_count > 0, 'empty-performance-samples', f'{mode_label} missing samples/comparisons')
    summary = fields.get('summary')
    _require(
        isinstance(summary, dict)
        and set(summary) == {'samples', 'cases', 'failures', 'inconclusive'}
        and isinstance(summary.get('samples'), list)
        and len(summary['samples']) == sample_count
        and isinstance(summary.get('cases'), list)
        and bool(summary['cases'])
        and summary.get('failures') == []
        and isinstance(summary.get('inconclusive'), list),
        'invalid-performance-summary',
        f'{mode_label} summary failed sample count or zero-failure invariants',
    )
    assert isinstance(summary, dict)
    inconclusive_list = summary['inconclusive']
    assert isinstance(inconclusive_list, list)
    if inconclusive_list:
        pending.append(f'{mode_label}_inconclusive_cases:{len(inconclusive_list)}')
    return {
        'passed': True,
        'inconclusive_count': len(inconclusive_list),
        'process_count': process_count,
        'sample_count': sample_count,
        'startup_count': startup_count,
        'comparison_count': comparison_count,
        'eligible_comparison_count': eligible_comparison_count,
        'completion_sha256': stage_row['completion_sha256'],
        'detailed_pin': detailed_pin,
    }


def _audit_performance(
    registry: InputPinRegistry,
    perf_path: Path,
    perf_doc: dict[str, object],
    perf_pin: dict[str, object],
    build_ctx: dict[str, object],
    ref_ctx: dict[str, object],
    pending: list[str],
) -> dict[str, object]:
    _require(
        perf_doc.get('schema') == STAGE_SCHEMA
        and perf_doc.get('action') == 'execute'
        and perf_doc.get('execution_phase') in ('performance', 'all')
        and perf_doc.get('passed') is True,
        'performance-stage-not-passed',
        'performance stage receipt must be a passing execute performance/all document',
    )
    identity_pin = _verify_stage_postflight_and_context(
        registry, perf_path, perf_doc, build_ctx, ref_ctx, 'performance'
    )
    cases = perf_doc.get('cases')
    _require(isinstance(cases, list) and len(cases) >= 2, 'performance-missing-modes', 'performance stage missing mode rows')
    assert isinstance(cases, list)
    by_mode: dict[str, dict[str, object]] = {}
    ordered_modes: list[str] = []
    for row in cases:
        _require(isinstance(row, dict), 'invalid-performance-row', 'performance stage row must be object')
        assert isinstance(row, dict)
        row_id = row.get('id')
        if row_id in PERFORMANCE_ACTIONS:
            _require(row_id not in by_mode, 'duplicate-performance-mode', f'duplicate {row_id} row')
            assert isinstance(row_id, str)
            by_mode[row_id] = row
            ordered_modes.append(row_id)
    _require(
        tuple(ordered_modes) == PERFORMANCE_ACTIONS,
        'performance-modes-incomplete',
        f'performance stage must contain {PERFORMANCE_ACTIONS} in order, got {ordered_modes}',
    )
    mode_results: dict[str, object] = {}
    for action_id, is_fallback in (('performance', False), ('performance-fallback', True)):
        row = by_mode[action_id]
        _require(
            row.get('passed') is True
            and row.get('execution_complete') is True
            and 'receipt_validation_error' not in row
            and _is_hex64(row.get('completion_sha256'))
            and _is_hex64(row.get('receipt_sha256')),
            'performance-mode-row-failed',
            f'{action_id} stage row failed or lacks completion/detailed SHA256',
        )
        _validate_process_record(row.get('process'), f'performance.{action_id}')
        receipt_file = _normalize_path_text(row.get('receipt'), f'{action_id}.receipt')
        comp_file = _normalize_path_text(row.get('completion_receipt'), f'{action_id}.completion_receipt')
        action_dir = receipt_file.parent
        _require(
            receipt_file.name == 'results.json' and comp_file == action_dir / 'completion.json',
            'performance-locator-mismatch',
            f'{action_id} completion/results paths must share owned action directory',
        )
        mode_results[action_id] = _audit_single_performance_mode(
            registry,
            action_dir,
            row,
            ref_ctx,
            permanent_full_fallback=is_fallback,
            pending=pending,
        )
    return {
        'passed': True,
        'receipt': perf_pin,
        'identity_receipt': identity_pin,
        'modes': mode_results,
    }


def _extract_flag_value(argv: object, flag: str) -> str | None:
    if not isinstance(argv, list):
        return None
    for idx, token in enumerate(argv):
        if token == flag and idx + 1 < len(argv) and isinstance(argv[idx + 1], str):
            return argv[idx + 1]
    return None


def _audit_components(
    registry: InputPinRegistry,
    comp_doc: dict[str, object],
    comp_pin: dict[str, object],
    build_ctx: dict[str, object],
) -> dict[str, object]:
    _require(
        comp_doc.get('schema') == STAGE_SCHEMA
        and comp_doc.get('action') == 'components'
        and comp_doc.get('passed') is True
        and comp_doc.get('goal_complete') is False
        and comp_doc.get('execution_selection') is None
        and comp_doc.get('build_receipt') == str(build_ctx['build_root'])
        and comp_doc.get('build_complete') is True
        and comp_doc.get('source_root') == build_ctx['source_root'],
        'components-not-passed',
        'components stage receipt failed schema, build/source provenance, or passing invariants',
    )
    cases = comp_doc.get('cases')
    _require(
        isinstance(cases, list)
        and [r.get('id') for r in cases if isinstance(r, dict)] == list(COMPONENT_COMMANDS),
        'components-commands-mismatch',
        f'components must run exact commands {COMPONENT_COMMANDS}',
    )
    assert isinstance(cases, list)
    by_cmd = {str(r['id']): r for r in cases if isinstance(r, dict)}
    for cmd_id in COMPONENT_COMMANDS:
        row = by_cmd[cmd_id]
        _require(row.get('passed') is True, 'component-command-failed', f'components.{cmd_id} did not pass')
        _validate_process_record(row.get('process'), f'components.{cmd_id}')
    _validate_process_record(comp_doc.get('xtask_compile'), 'components.xtask_compile')
    xtask_cases = comp_doc.get('xtask_cases')
    _require(
        isinstance(xtask_cases, list) and bool(xtask_cases),
        'components-xtask-empty',
        'components.xtask_cases must be non-empty',
    )
    assert isinstance(xtask_cases, list)
    for idx, xrow in enumerate(xtask_cases):
        _require(
            isinstance(xrow, dict)
            and _is_nonempty_str(xrow.get('id'))
            and _is_hex64(xrow.get('binary_sha256'))
            and xrow.get('passed') is True,
            'components-xtask-failed',
            f'components.xtask_cases[{idx}] failed or malformed',
        )
        assert isinstance(xrow, dict)
        _validate_process_record(xrow.get('process'), f'components.xtask[{xrow["id"]}]')
    build_proc = by_cmd['actual-source-build']['process']
    contracts_proc = by_cmd['actual-source-contracts']['process']
    assert isinstance(build_proc, dict) and isinstance(contracts_proc, dict)
    expected_comp_dir = Path(str(build_ctx['source_root'])) / 'build/acceptance/runtime-components'
    expected_results_dir = Path(str(build_ctx['source_root'])) / 'build/acceptance/runtime-component-results'
    _require(
        _extract_flag_value(build_proc.get('argv'), '--out') == str(expected_comp_dir)
        and _extract_flag_value(contracts_proc.get('argv'), '--receipt') == str(expected_comp_dir / 'receipt.json')
        and _extract_flag_value(contracts_proc.get('argv'), '--out') == str(expected_results_dir),
        'components-actual-source-argv-mismatch',
        'actual-source-build/contracts argv do not bind the frozen source component receipt paths',
    )
    build_ev_dir = _assert_physical_directory(
        build_proc.get('evidence_directory'), 'components.actual-source-build.evidence_directory'
    )
    _, _, build_stdout_doc = registry.read_json(
        build_ev_dir / 'stdout.txt',
        max_bytes=1024 * 1024,
        label='actual-source-build/stdout.txt',
    )
    _, adapter_receipt_pin, adapter_receipt = registry.read_json(
        expected_comp_dir / 'receipt.json',
        max_bytes=4 * 1024 * 1024,
        label='runtime-components/receipt.json',
    )
    source_files = build_ctx['source_inputs']['files']  # type: ignore[index]
    assert isinstance(source_files, dict)
    expected_primary_shas = {name: source_files[name]['sha256'] for name in COMPONENT_PRIMARY_SOURCES}
    expected_support_shas = {name: source_files[name]['sha256'] for name in COMPONENT_SUPPORT_SOURCES}
    _require(
        isinstance(adapter_receipt, dict)
        and adapter_receipt.get('schema') == 1
        and adapter_receipt.get('provider') == 'actual-source-components'
        and adapter_receipt.get('sources') == expected_primary_shas
        and isinstance(adapter_receipt.get('supporting_source'), dict)
        and adapter_receipt['supporting_source'].get('sources') == expected_support_shas
        and adapter_receipt.get('binary') == 'target/release/runtime-component-adapter'
        and _is_hex64(adapter_receipt.get('binary_sha256'))
        and isinstance(adapter_receipt.get('stubs'), list)
        and bool(adapter_receipt['stubs'])
        and isinstance(build_stdout_doc, dict)
        and build_stdout_doc.get('receipt') == 'receipt.json'
        and build_stdout_doc.get('binary_sha256') == adapter_receipt.get('binary_sha256'),
        'invalid-actual-source-component-receipt',
        'runtime-components/receipt.json does not match frozen actual Rust sources or build stdout',
    )
    assert isinstance(adapter_receipt, dict)
    _, contract_results_pin, contract_results = registry.read_json(
        expected_results_dir / 'results.json',
        max_bytes=MAX_SMALL_JSON_BYTES,
        label='runtime-component-results/results.json',
    )
    _require(
        isinstance(contract_results, dict)
        and contract_results.get('passed') is True
        and contract_results.get('complete') is True
        and type(contract_results.get('requested')) is int
        and contract_results['requested'] > 0
        and contract_results.get('observed') == contract_results['requested']
        and isinstance(contract_results.get('cases'), list)
        and len(contract_results['cases']) == contract_results['requested'],
        'actual-source-contracts-failed',
        'runtime-component-results/results.json failed completeness or passing invariants',
    )
    assert isinstance(contract_results, dict)
    seen_contract_ids: set[str] = set()
    for row in contract_results['cases']:
        _require(
            isinstance(row, dict)
            and _is_nonempty_str(row.get('case_id'))
            and row['case_id'] not in seen_contract_ids
            and row.get('passed') is True
            and row.get('status') == 'passed'
            and row.get('sources') == expected_primary_shas
            and row.get('binary_sha256_before') == adapter_receipt['binary_sha256']
            and row.get('binary_sha256_after') == adapter_receipt['binary_sha256'],
            'actual-source-contract-case-failed',
            'component contract case failed or drifted from actual-source adapter binary/sources',
        )
        assert isinstance(row, dict)
        seen_contract_ids.add(str(row['case_id']))
    return {
        'passed': True,
        'receipt': comp_pin,
        'adapter_receipt': adapter_receipt_pin,
        'contract_results': contract_results_pin,
        'contract_case_count': len(seen_contract_ids),
        'xtask_case_count': len(xtask_cases),
    }


def _audit_local_receipt(
    registry: InputPinRegistry,
    local_arg: Path | None,
    pending: list[str],
) -> tuple[dict[str, object], bool]:
    if local_arg is None:
        pending.append('local_acceptance_receipt_missing')
        return {'status': 'pending', 'provided': False, 'passed': False}, False
    _, pin, doc = registry.read_json(local_arg, max_bytes=MAX_SMALL_JSON_BYTES, label='--local')
    _require(isinstance(doc, dict), 'invalid-local-receipt', '--local receipt must be a JSON object')
    assert isinstance(doc, dict)
    checks = doc.get('checks')
    products = doc.get('products')
    unrequested = doc.get('unrequested_layers')
    _require(
        doc.get('passed') is True
        and isinstance(checks, list)
        and bool(checks)
        and all(isinstance(c, dict) and c.get('passed') is True for c in checks)
        and isinstance(products, dict)
        and {'terminal', 'terminal_dump', 'gui', 'gui_dump'} <= set(products)
        and isinstance(doc.get('source_head'), str)
        and HEX40.fullmatch(doc['source_head']) is not None
        and _is_hex64(doc.get('tracked_patch_sha256'))
        and type(doc.get('requested_gui')) is bool
        and type(doc.get('requested_ime')) is bool
        and isinstance(unrequested, list),
        'local-acceptance-failed',
        '--local receipt did not pass or is missing local-acceptance.py fields',
    )
    assert isinstance(checks, list) and isinstance(unrequested, list)
    needs_workflow_review = False
    if unrequested:
        needs_workflow_review = True
        joined = ','.join(str(x) for x in unrequested)
        pending.append(f'local_acceptance_unrequested_layers:{joined}')
    return (
        {
            'status': 'needs-workflow-review' if needs_workflow_review else 'verified',
            'provided': True,
            'passed': True,
            'receipt': pin,
            'check_count': len(checks),
            'requested_gui': doc['requested_gui'],
            'requested_ime': doc['requested_ime'],
            'unrequested_layers': unrequested,
        },
        needs_workflow_review,
    )


def _audit_tty_receipt(
    registry: InputPinRegistry,
    tty_arg: Path | None,
    pending: list[str],
) -> dict[str, object]:
    if tty_arg is None:
        pending.append('tty_restoration_receipt_missing')
        return {'status': 'pending', 'provided': False, 'passed': False}
    _, pin, doc = registry.read_json(tty_arg, max_bytes=16 * 1024 * 1024, label='--tty')
    _require(isinstance(doc, dict), 'invalid-tty-receipt', '--tty receipt must be a JSON object')
    assert isinstance(doc, dict)
    ident_before = doc.get('identities_before')
    ident_after = doc.get('identities_after')
    _require(
        doc.get('id') == 'normal-terminal-mode-restoration'
        and doc.get('passed') is True
        and doc.get('exit_code') == 0
        and doc.get('active_modes_differed') is True
        and doc.get('settings_without_pending_equal') is True
        and doc.get('functional_canonical_input') is True
        and doc.get('after_input_modes_restored') is True
        and doc.get('echo_observed') is True
        and isinstance(ident_before, dict)
        and bool(ident_before)
        and ident_before == ident_after,
        'tty-restoration-failed',
        '--tty receipt failed test-terminal-modes.py mode-restoration invariants',
    )
    return {'status': 'verified', 'provided': True, 'passed': True, 'receipt': pin}


def _audit_cocoa_receipt(
    registry: InputPinRegistry,
    cocoa_arg: Path | None,
    pending: list[str],
) -> tuple[dict[str, object], bool]:
    # GUI / Finder / multiframe Cocoa / physical IME local sign-off has no
    # stable versioned machine schema in Root/tools. Never invent fields; pin
    # any supplied receipt, reject observed failures, and require explicit
    # workflow review before evidence_complete can ever be asserted.
    if cocoa_arg is None:
        pending.append('cocoa_workflow_receipt_missing')
        pending.append('cocoa_gui_ime_requires_explicit_workflow_review')
        return (
            {
                'status': 'pending',
                'provided': False,
                'passed': False,
                'stable_machine_schema': False,
                'needs_explicit_workflow_review': True,
            },
            True,
        )
    _, pin, doc = registry.read_json(cocoa_arg, max_bytes=MAX_SMALL_JSON_BYTES, label='--cocoa')
    if isinstance(doc, dict):
        if 'passed' in doc:
            _require(doc.get('passed') is True, 'cocoa-receipt-failed', '--cocoa receipt reported passed != True')
        observed_passed = doc.get('passed') is True
    elif isinstance(doc, list) and doc and all(isinstance(row, dict) for row in doc):
        _require(
            all(row.get('passed') is True for row in doc),
            'cocoa-receipt-failed',
            '--cocoa scenario list contains non-passing row',
        )
        observed_passed = True
    else:
        raise AuditError('invalid-cocoa-receipt', '--cocoa receipt must be a JSON object or non-empty row array')
    pending.append('cocoa_gui_ime_requires_explicit_workflow_review')
    return (
        {
            'status': 'needs-explicit-workflow-review',
            'provided': True,
            'observed_passed': observed_passed,
            'stable_machine_schema': False,
            'needs_explicit_workflow_review': True,
            'receipt': pin,
        },
        True,
    )


def _load_required_stage_json(
    registry: InputPinRegistry,
    path: Path,
    stage_name: str,
    pending: list[str],
    errors: list[str],
) -> tuple[Path, dict[str, object], dict[str, object]] | None:
    try:
        resolved, pin, doc = registry.read_json(path, max_bytes=MAX_SMALL_JSON_BYTES, label=f'--{stage_name}')
        _require(isinstance(doc, dict), 'invalid-stage-json', f'--{stage_name} must be a JSON object')
        assert isinstance(doc, dict)
        return resolved, pin, doc
    except AuditError as exc:
        if exc.code in ('missing-file', 'missing-directory'):
            pending.append(f'{stage_name}_receipt_missing')
        errors.append(f'{stage_name}: {exc}')
        return None


def _load_functional_receipts(registry, functional_paths, protection_paths,
                               pending, errors):
    """Verify the official-scale functional cohort receipts.

    The daily lane is the functional corpus (555 original files against one
    build, judged against the closed reference pair); the 48-candidate matrix is
    an audit node and is not required here.  A receipt only passes when it names
    its baseline and program, and when no file is unresolved.
    """
    if not functional_paths:
        if protection_paths:
            errors.append('functional: protection receipts supplied without a functional cohort')
        return None
    unresolved_fields = ('port-worse', 'incomplete', 'baseline-unavailable',
                         'registration-mismatch', 'pending-condition-diff')
    receipts = []
    passed = True
    file_count = 0
    for path in functional_paths:
        try:
            _, pin, doc = registry.read_json(path, max_bytes=64 * 1024 * 1024,
                                             label='functional-summary.json')
        except AuditError as exc:
            errors.append(f'functional: {exc}')
            pending.append('functional_receipt_missing')
            passed = False
            continue
        if not isinstance(doc, dict) or doc.get('schema') != 'femacs-functional-corpus-v1':
            errors.append(f'functional: invalid receipt schema in {path}')
            passed = False
            continue
        counts = doc.get('counts') if isinstance(doc.get('counts'), dict) else {}
        unresolved = sum(int(counts.get(name, 0)) for name in unresolved_fields
                         if isinstance(counts.get(name, 0), int))
        rows = doc.get('files') if isinstance(doc.get('files'), list) else []
        verdicts = {row.get('verdict') for row in rows if isinstance(row, dict)}
        program = doc.get('program') if isinstance(doc.get('program'), dict) else {}
        dump = doc.get('dump') if isinstance(doc.get('dump'), dict) else {}
        row = dict(path=str(path), sha256=pin['sha256'], label=doc.get('label'),
                   baseline=doc.get('baseline'), counts=counts, unresolved=unresolved,
                   files=len(rows), verdicts=sorted(v for v in verdicts if isinstance(v, str)),
                   program=program.get('sha256'), dump=dump.get('sha256'))
        if unresolved != 0:
            errors.append(f'functional: {path} has {unresolved} unresolved file(s)')
            passed = False
        if not doc.get('baseline'):
            errors.append(f'functional: {path} has no closed reference baseline')
            passed = False
        if not rows or not verdicts <= {'same', 'resolved'}:
            errors.append(f'functional: {path} carries a non-passing verdict set {sorted(verdicts)}')
            passed = False
        file_count += len(rows)
        receipts.append(row)
    protections = []
    for path in protection_paths:
        try:
            _, pin, doc = registry.read_json(path, max_bytes=64 * 1024 * 1024,
                                             label='protection-results.json')
        except AuditError as exc:
            errors.append(f'protection: {exc}')
            pending.append('protection_receipt_missing')
            passed = False
            continue
        ok = (isinstance(doc, dict)
              and doc.get('schema') == 'femacs-image-loader-protection-v1'
              and doc.get('complete') is True
              and isinstance(doc.get('program', {}).get('sha256'), str)
              and len(doc.get('program', {}).get('sha256', '')) == 64)
        if not ok:
            errors.append(f'protection: {path} is not a complete protection receipt')
            passed = False
        protections.append(dict(path=str(path), sha256=pin['sha256'], complete=bool(ok)))
    return dict(passed=passed, summary=dict(
        schema='femacs-functional-cohort-v1', file_count=file_count,
        receipts=receipts, protections=protections,
        scope='official-scale functional cohort; 48-candidate matrix is an audit node'))


def run_audit(args: argparse.Namespace) -> int:
    out_dir = _prepare_fresh_output(args.out)
    started = time.monotonic()
    registry = InputPinRegistry()
    pending: list[str] = []
    errors: list[str] = []
    stages_report: dict[str, object] = {}
    needs_explicit_workflow_review = True
    inputs_unchanged_at_exit = True
    machine_matrix_complete = False
    try:
        build_ctx: dict[str, object] | None = None
        try:
            build_ctx = _audit_build(registry, args.build)
            stages_report['build'] = {
                'passed': True,
                'build_root': str(build_ctx['build_root']),
                'results_receipt': build_ctx['results_pin'],
                'products_receipt': build_ctx['products_pin'],
                'source_root': build_ctx['source_root'],
                'source_manifest_sha256': build_ctx['source_export']['source_manifest_sha256'],  # type: ignore[index]
                'corpus_manifest_sha256': build_ctx['corpus_manifest_sha256'],
                'build_count': EXPECTED_BUILDS,
                'profile_count': EXPECTED_PROFILES,
            }
        except AuditError as exc:
            if exc.code in ('missing-file', 'missing-directory'):
                pending.append('build_receipt_missing')
            errors.append(f'build: {exc}')
            stages_report['build'] = {'passed': False, 'error': str(exc)}

        prepare_loaded = _load_required_stage_json(registry, args.prepare, 'prepare', pending, errors)
        semantics_loaded = _load_required_stage_json(registry, args.semantics, 'semantics', pending, errors)
        classified_loaded = None
        if args.classified_stable is not None:
            classified_loaded = _load_required_stage_json(
                registry, args.classified_stable, 'classified-stable', pending, errors
            )
        functional_loaded = _load_functional_receipts(registry, args.functional, args.protection,
                                                      pending, errors)
        if functional_loaded is not None:
            stages_report['functional'] = dict(functional_loaded['summary'],
                                               passed=functional_loaded['passed'])
        performance_loaded = _load_required_stage_json(registry, args.performance, 'performance', pending, errors)
        components_loaded = _load_required_stage_json(registry, args.components, 'components', pending, errors)

        stable_loaded: list[tuple[Path, dict[str, object], dict[str, object]]] = []
        if not args.stable:
            if args.classified_stable is not None:
                pending.append('stable_raw_stage_results_missing')
                stages_report['stable'] = {'passed': False, 'status': 'pending',
                                           'reason': 'no --stable inputs supplied'}
            else:
                stages_report['stable'] = {
                    'status': 'audit-node-not-run',
                    'reason': 'the 48-candidate matrix is an audit node, not the daily cohort gate'}
                stages_report['classified_stable'] = {
                    'status': 'audit-node-not-run',
                    'reason': 'classified_stable belongs to the matrix audit node'}
        else:
            for idx, stable_arg in enumerate(args.stable):
                loaded = _load_required_stage_json(registry, stable_arg, f'stable[{idx}]', pending, errors)
                if loaded is not None:
                    stable_loaded.append(loaded)

        ref_ctx: dict[str, object] | None = None
        if build_ctx is not None:
            stage_summaries: list[tuple[str, object, bool]] = []
            for label, loaded_item, required_when_active in (
                ('prepare', prepare_loaded, False),
                ('semantics', semantics_loaded, True),
                ('classified_stable', classified_loaded, classified_loaded is not None),
                ('performance', performance_loaded, True),
                ('components', components_loaded, False),
            ):
                if loaded_item is not None:
                    stage_summaries.append((label, loaded_item[2].get('reference_selection'), required_when_active))
            for idx, (_, _, st_doc) in enumerate(stable_loaded):
                stage_summaries.append((f'stable[{idx}]', st_doc.get('reference_selection'), True))
            try:
                ref_ctx = _audit_reference_selection(registry, build_ctx, stage_summaries)
                stages_report['reference_selection'] = {
                    'passed': True,
                    'active': ref_ctx['active'],
                    'epoch': ref_ctx['epoch'],
                    'historical_epoch': ref_ctx['historical_epoch'],
                    'active_products_path': ref_ctx['active_products_path'],
                    'active_products_sha256': ref_ctx['active_products_sha256'],
                }
            except AuditError as exc:
                errors.append(f'reference_selection: {exc}')
                stages_report['reference_selection'] = {'passed': False, 'error': str(exc)}

        if build_ctx is not None and ref_ctx is not None:
            if prepare_loaded is not None:
                try:
                    stages_report['prepare'] = _audit_prepare(
                        registry, prepare_loaded[2], prepare_loaded[1], build_ctx, ref_ctx
                    )
                except AuditError as exc:
                    errors.append(f'prepare: {exc}')
                    stages_report['prepare'] = {'passed': False, 'error': str(exc)}

            if semantics_loaded is not None:
                try:
                    stages_report['semantics'] = _audit_semantics(
                        registry, semantics_loaded[0], semantics_loaded[2], semantics_loaded[1], build_ctx, ref_ctx
                    )
                except AuditError as exc:
                    errors.append(f'semantics: {exc}')
                    stages_report['semantics'] = {'passed': False, 'error': str(exc)}

            stable_ctx: dict[str, object] | None = None
            if args.stable and len(stable_loaded) == len(args.stable):
                try:
                    stable_ctx = _audit_stable_raw_stages(
                        registry, stable_loaded, build_ctx, ref_ctx, pending
                    )
                    stages_report['stable'] = {
                        'passed': True,
                        'stage_count': len(stable_loaded),
                        'profile_count': stable_ctx['profile_count'],
                        'entrypoints_per_profile': stable_ctx['entrypoints_per_profile'],
                        'stage_inputs': stable_ctx['stage_inputs'],
                    }
                except AuditError as exc:
                    errors.append(f'stable: {exc}')
                    stages_report['stable'] = {'passed': False, 'error': str(exc)}
            elif args.stable:
                stages_report['stable'] = {'passed': False, 'error': 'one or more --stable receipts failed to load'}

            if classified_loaded is not None:
                try:
                    stages_report['classified_stable'] = _audit_classified_stable(
                        registry, classified_loaded[2], classified_loaded[1], build_ctx, ref_ctx, stable_ctx
                    )
                except AuditError as exc:
                    errors.append(f'classified_stable: {exc}')
                    stages_report['classified_stable'] = {'passed': False, 'error': str(exc)}

            if performance_loaded is not None:
                try:
                    stages_report['performance'] = _audit_performance(
                        registry,
                        performance_loaded[0],
                        performance_loaded[2],
                        performance_loaded[1],
                        build_ctx,
                        ref_ctx,
                        pending,
                    )
                except AuditError as exc:
                    errors.append(f'performance: {exc}')
                    stages_report['performance'] = {'passed': False, 'error': str(exc)}

            if components_loaded is not None:
                try:
                    stages_report['components'] = _audit_components(
                        registry, components_loaded[2], components_loaded[1], build_ctx
                    )
                except AuditError as exc:
                    errors.append(f'components: {exc}')
                    stages_report['components'] = {'passed': False, 'error': str(exc)}

        local_needs_review = False
        try:
            local_summary, local_needs_review = _audit_local_receipt(registry, args.local, pending)
            stages_report['local'] = local_summary
        except AuditError as exc:
            if exc.code in ('missing-file', 'missing-directory'):
                pending.append('local_acceptance_receipt_missing')
            errors.append(f'local: {exc}')
            stages_report['local'] = {'status': 'failed', 'provided': True, 'passed': False, 'error': str(exc)}

        try:
            stages_report['tty'] = _audit_tty_receipt(registry, args.tty, pending)
        except AuditError as exc:
            if exc.code in ('missing-file', 'missing-directory'):
                pending.append('tty_restoration_receipt_missing')
            errors.append(f'tty: {exc}')
            stages_report['tty'] = {'status': 'failed', 'provided': True, 'passed': False, 'error': str(exc)}

        cocoa_needs_review = True
        try:
            cocoa_summary, cocoa_needs_review = _audit_cocoa_receipt(registry, args.cocoa, pending)
            stages_report['cocoa'] = cocoa_summary
        except AuditError as exc:
            if exc.code in ('missing-file', 'missing-directory'):
                pending.append('cocoa_workflow_receipt_missing')
            pending.append('cocoa_gui_ime_requires_explicit_workflow_review')
            errors.append(f'cocoa: {exc}')
            stages_report['cocoa'] = {
                'status': 'failed',
                'provided': True,
                'passed': False,
                'stable_machine_schema': False,
                'needs_explicit_workflow_review': True,
                'error': str(exc),
            }

        needs_explicit_workflow_review = bool(local_needs_review or cocoa_needs_review)
        required_machine_stages = [
            'build',
            'reference_selection',
            'prepare',
            'semantics',
            'performance',
            'components',
        ]
        if functional_loaded is not None:
            required_machine_stages.append('functional')
        if args.classified_stable is not None:
            required_machine_stages.extend(['stable', 'classified_stable'])
        if functional_loaded is None and args.classified_stable is None:
            pending.append('cohort_receipts_missing')
        required_machine_stages = tuple(required_machine_stages)
        machine_matrix_complete = (
            not errors
            and all(
                isinstance(stages_report.get(name), dict)
                and stages_report[name].get('passed') is True  # type: ignore[index]
                for name in required_machine_stages
            )
            and stages_report.get('performance', {}).get('modes', {}).get('performance', {}).get('inconclusive_count') == 0  # type: ignore[union-attr]
            and stages_report.get('performance', {}).get('modes', {}).get('performance-fallback', {}).get('inconclusive_count') == 0  # type: ignore[union-attr]
        )
    except Exception as exc:
        errors.append(f'unexpected-audit-exception: {type(exc).__name__}: {exc}')
    finally:
        unchanged, reverify_errors = registry.verify_all_unchanged()
        inputs_unchanged_at_exit = unchanged
        if reverify_errors:
            errors.extend(reverify_errors)
            machine_matrix_complete = False
        dedup_pending = list(dict.fromkeys(pending))
        dedup_errors = list(dict.fromkeys(errors))
        evidence_complete = (
            machine_matrix_complete
            and not dedup_errors
            and not dedup_pending
            and not needs_explicit_workflow_review
            and inputs_unchanged_at_exit
        )
        ready_for_independent_final_review = bool(evidence_complete)
        report: dict[str, object] = {
            'schema': SCHEMA,
            'evidence_complete': evidence_complete,
            'ready_for_independent_final_review': ready_for_independent_final_review,
            'machine_matrix_complete': machine_matrix_complete,
            'cohort': ('matrix' if args.classified_stable is not None
                       else 'functional' if functional_loaded is not None else 'unbound'),
            'matrix_status': ('closed-audit-node' if args.classified_stable is not None
                              else 'audit-node-not-run'),
            'needs_explicit_workflow_review': needs_explicit_workflow_review,
            'inputs_unchanged_at_exit': inputs_unchanged_at_exit,
            'compatibility_accepted': False,
            'performance_accepted': False,
            'goal_complete': False,
            'compiler_processes_started': 0,
            'editor_processes_started': 0,
            'original_results_modified': False,
            'pending': dedup_pending,
            'errors': dedup_errors,
            'stages': stages_report,
            'input_file_count': len(registry.snapshot_pins()),
            'input_files': registry.snapshot_pins(),
            'seconds': time.monotonic() - started,
        }
        _write_durable_json(out_dir / 'results.json', report)
    return 0 if report['evidence_complete'] is True else 1


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build', type=Path, required=True, help='Completed runtime-stage build root directory')
    parser.add_argument('--prepare', type=Path, required=True, help='runtime-stage prepare results.json')
    parser.add_argument('--semantics', type=Path, required=True, help='runtime-stage semantics results.json')
    parser.add_argument(
        '--stable',
        type=Path,
        action='append',
        default=[],
        help='Closed raw runtime-stage stable results.json (repeatable across disjoint profile scopes)',
    )
    parser.add_argument(
        '--classified-stable',
        dest='classified_stable',
        type=Path,
        default=None,
        help='Closed stable-classification.py stage results.json (matrix audit node)',
    )
    parser.add_argument(
        '--functional',
        type=Path,
        action='append',
        default=[],
        help='Closed tools/functional-corpus.py summary.json (repeatable; the daily cohort lane)',
    )
    parser.add_argument(
        '--protection',
        type=Path,
        action='append',
        default=[],
        help='Committed probe results.json bound to this cohort (repeatable)',
    )
    parser.add_argument('--performance', type=Path, required=True, help='runtime-stage performance results.json')
    parser.add_argument('--components', type=Path, required=True, help='runtime-stage components results.json')
    parser.add_argument('--local', type=Path, default=None, help='Optional local-acceptance.py results.json')
    parser.add_argument('--cocoa', type=Path, default=None, help='Optional Cocoa GUI/IME workflow receipt JSON')
    parser.add_argument('--tty', type=Path, default=None, help='Optional test-terminal-modes.py mode-result.json')
    parser.add_argument('--out', type=Path, required=True, help='Fresh non-existing audit output directory')
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        return run_audit(args)
    except OutputDirectoryError as exc:
        print(f'runtime-final-audit: {exc}', file=sys.stderr)
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
