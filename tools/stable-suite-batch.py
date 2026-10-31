"""Original per-file ERT execution with bounded, per-case progress accounting."""
from __future__ import annotations

import hashlib
import json
import math
import os
from pathlib import Path
import signal
import subprocess
import tempfile
import time
import uuid


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


def _run_file(api, out, paths, runner, test_root, logical, timeout, label):
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
    short_container = ((socket_case and _process_socket_path_bytes(planned_tmp) + 8 >= 104)
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
        if package_case and short_container:
            # This container belongs to exactly one file execution.  Keeping
            # another 21-byte case hash would still overflow auxiliary sockets.
            environment = api.case_environment(logical, container, compact=True)
        else:
            environment = api.case_environment(logical, container)
        case_root = Path(environment['FEMACS_RUNTIME_CASE_ROOT'])
        case_node = case_root.lstat()
        if socket_case and _process_socket_path_bytes(environment['TMPDIR']) + 8 >= 104:
            raise api.SuiteError('owned temporary path cannot fit the original local socket')
        if package_case and _package_socket_path_bytes(environment['TMPDIR']) >= 104:
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
    environment.update(PATH=os.environ.get('PATH', os.defpath), LC_ALL='C', LANG='C',
        TERM='dumb', PWD=str(cwd), EMACS_TEST_DIRECTORY=str(test_root),
        FEMACS_RUNTIME_TEST_ROOT=str(case_root), STABLE_SUITE_MODE='batch',
        STABLE_SUITE_FILE=str(out / logical), STABLE_SUITE_FILE_ID=logical,
        STABLE_SUITE_PROGRESS=str(progress_path), STABLE_SUITE_RESULT=str(final_path))
    environment.update(paths.get('runtime_env', {}))
    environment.update(STABLE_SUITE_SOURCE_CONTEXT=str(paths['test_context']),
                       STABLE_SUITE_PRODUCT_LISP=str(paths['lisp_root']))
    argv = [str(paths['program']), *paths['dump_args'], '--batch', '-Q',
            '-L', str(test_root), '-l', str(runner), '-f', 'stable-suite-main']
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
        deadline = time.monotonic() + timeout

    try:
        with progress_path.open('xb'):
            pass
        with progress_path.open('rb') as progress, stdout_path.open('xb') as stdout, stderr_path.open('xb') as stderr:
            process = subprocess.Popen(argv, cwd=cwd, env=environment,
                stdin=subprocess.DEVNULL, stdout=stdout, stderr=stderr, start_new_session=True)
            while True:
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
                try:
                    process.wait(timeout=.05)
                except subprocess.TimeoutExpired:
                    pass
    except (OSError, ValueError, api.SuiteError) as error:
        protocol_error = str(error)
    finally:
        if process is not None:
            try:
                os.killpg(process.pid, signal.SIGKILL)
                # A deliberately timed-out/protocol-error process is not a
                # successful parent with unexplained surviving descendants.
                remaining_group_terminated = not timed_out and protocol_error is None
            except ProcessLookupError:
                pass
            process.wait()
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
        seconds=time.monotonic()-started, exit_code=process.returncode if process else None,
        remaining_group_terminated=remaining_group_terminated,
        stdout_log=str(stdout_path.relative_to(out)), stderr_log=str(stderr_path.relative_to(out)),
        progress=(dict(path=str(progress_path.relative_to(out)), bytes=progress_path.stat().st_size,
                       sha256=api._sha256_path(progress_path)) if progress_path.is_file() else None),
        owned_state_root=str(case_root), owned_container=str(container),
        environment_preparation=str(preparation_path.relative_to(out)),
        state_retirement_path=str(final_path.with_name(
            final_path.stem + '.owner-retirement.json').relative_to(out)))
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
    context_path = repo/'tools/stable-context.py'
    context_spec = importlib.util.spec_from_file_location('femacs_stable_context', context_path)
    context_provider = importlib.util.module_from_spec(context_spec)
    context_spec.loader.exec_module(context_provider)
    harness_sources[str(context_path)] = api._sha256_path(context_path)
    provider_path = repo/'tools/reference-corpus.py'
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
                    pair[label] = _run_file(api, out, product, runner, out/'test', logical, timeout, label)
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
