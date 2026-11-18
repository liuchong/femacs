"""Functional integration for P464 explicit product Lisp roots and owned stable harness.

Tests real file/process composition for explicit product library selection on
an App-shaped reference with no Resources/lisp, six-file owned harness capture
and Darwin UF_IMMUTABLE lifecycle, helper imports from the captured snapshot,
and a real Python CLI subprocess with unchanged stable writer/reader and
post-run classification. A tiny synthetic executable emits the progress
protocol instead of running Lisp (actual_editor_execution=False).
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
import re
from pathlib import Path
import shutil
import stat
import sys
import time
from types import SimpleNamespace
from unittest import mock

REPO = Path(__file__).resolve().parent.parent


def _load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


stage = _load_module('library_roots_test_stage', REPO / 'tools/runtime-stage.py')
e = stage.execution
owner = stage.owner
suite = e.load_tool('stable-suite.py')
accounting = _load_module('library_roots_test_classification', REPO / 'tools/stable-classification.py')

HARNESS_NAMES = (
    'stable-suite.py',
    'stable-suite-batch.py',
    'stable-context.py',
    'runtime-retire.py',
    'reference-corpus.py',
    'fixtures/module-thread-config.h',
)

SYNTHETIC_PROGRAM = """#!{python}
import json, os, sys
from pathlib import Path

assert os.environ.get("STABLE_SUITE_MODE") == "batch"
explicit_lisp = os.environ.get("FEMACS_STABLE_PRODUCT_LISP")
product_lisp = os.environ.get("STABLE_SUITE_PRODUCT_LISP")
context_root = os.environ.get("STABLE_SUITE_SOURCE_CONTEXT")
assert explicit_lisp and explicit_lisp == product_lisp
lisp_path = Path(product_lisp)
assert lisp_path.is_dir() and (lisp_path / "simple.el").is_file()
assert context_root and (Path(context_root) / "lisp").is_symlink()
assert (Path(context_root) / "lisp").resolve() == lisp_path.resolve()
program = Path(sys.argv[0]).resolve()
if program.parent.name == "MacOS" and "Reference.app" in str(program):
    assert not (program.parent.parent / "Resources" / "lisp").exists()

logical = os.environ["STABLE_SUITE_FILE_ID"]
progress = Path(os.environ["STABLE_SUITE_PROGRESS"])
failing = "terminal-collector-normal-young" in str(progress)
status = "failed" if failing else "passed"
diagnostic = "synthetic failed evidence" if failing else ""
test_id = "synthetic-library-root-case"
events = [
    dict(
        event="loaded",
        file=logical,
        selector="all-discovered-ids",
        ids=[test_id],
        tests=[dict(id=test_id, tags=[])],
        emacs_version="32.0.50",
        system_configuration="aarch64-apple-darwin",
        system_type="darwin",
        window_system=None,
        features=["threads"],
    ),
    dict(event="test-started", file=logical, test_id=test_id),
    dict(
        event="test-ended",
        file=logical,
        test_id=test_id,
        status=status,
        duration=0.001,
        diagnostic=diagnostic,
        expected=(status == "passed"),
        skipped=False,
        condition="synthetic-condition" if failing else None,
    ),
    dict(
        event="finished",
        file=logical,
        status="complete",
        requested_count=1,
        ended_count=1,
    ),
]
with progress.open("ab") as stream:
    for row in events:
        stream.write(json.dumps(row, sort_keys=True).encode("utf-8") + b"\\n")
    stream.flush()
    os.fsync(stream.fileno())
raise SystemExit(0)
"""

SYNTHETIC_CONTEXT = """from __future__ import annotations
import json, os
from pathlib import Path

SELECTION = "f23d2dde45c9bb0eb3dcd5757cbfc90a71eeeb36b40697590a7fc4ce1f65e999"


def prepare(api, repo, out, corpus, products):
    provider = api._corpus_provider(repo)
    expected_provider = Path(__file__).with_name("reference-corpus.py").resolve()
    if Path(provider.__file__).resolve() != expected_provider:
        raise api.SuiteError("corpus provider did not resolve inside captured harness")
    manifest_path = api._absolute(
        repo / "compatibility/stable-31.1/source-context.json",
        "source context manifest",
    )
    manifest_sha = api._sha256_path(manifest_path)
    context = json.loads(manifest_path.read_text(encoding="utf-8"))
    if context.get("source") != corpus["source"]:
        raise api.SuiteError("synthetic source context version mismatch")
    data = out / "source-context-data"
    for directory in ("src", "admin", "nextstep"):
        (data / directory).mkdir(parents=True, exist_ok=True)
    readme = data / "README"
    readme.write_text("synthetic source context readme\\n", encoding="utf-8")
    contexts = {}
    for label, paths in products.items():
        root = out / "source-contexts" / label
        root.mkdir(parents=True)
        links = {name: data / name for name in ("src", "admin", "nextstep", "README")}
        links.update(lisp=paths["lisp_root"], test=out / "test")
        for name, target in links.items():
            (root / name).symlink_to(
                os.path.relpath(target, root),
                target_is_directory=target.is_dir(),
            )
        paths["test_context"] = root
        contexts[label] = dict(
            root=str(root),
            links={name: os.readlink(root / name) for name in links},
            resolved_lisp=str(Path(paths["lisp_root"]).resolve()),
        )
    record = dict(
        manifest_sha256=manifest_sha,
        selection_sha256=SELECTION,
        file_count=1,
        bytes=readme.stat().st_size,
        acquisition=dict(synthetic_context=True, provider=str(expected_provider)),
        contexts=contexts,
    )
    (out / "source-context.json").write_text(
        json.dumps(record, indent=2, sort_keys=True) + "\\n",
        encoding="utf-8",
    )
    return context, record


def verify(api, repo, out, context, record):
    manifest_path = repo / "compatibility/stable-31.1/source-context.json"
    if api._sha256_path(manifest_path) != record["manifest_sha256"]:
        raise api.SuiteError("source context manifest changed during execution")
    for value in record["contexts"].values():
        root = Path(value["root"])
        if {path.name for path in root.iterdir()} != set(value["links"]):
            raise api.SuiteError("source context link set changed")
        for name, target in value["links"].items():
            link = root / name
            if not link.is_symlink() or os.readlink(link) != target:
                raise api.SuiteError("source context link changed")
        if (root / "lisp").resolve() != Path(value["resolved_lisp"]):
            raise api.SuiteError("source context Lisp target changed")
"""


def _write_lisp_tree(root, marker):
    (root / 'emacs-lisp').mkdir(parents=True, exist_ok=True)
    (root / 'simple.el').write_text(f';; {marker} simple.el\n', encoding='utf-8')
    (root / 'emacs-lisp/comp.el').write_text(f';; {marker} comp.el\n', encoding='utf-8')
    (root / 'emacs-lisp/comp.elc').write_bytes(f';; {marker} comp.elc\n'.encode('utf-8'))


def _write_executable(path):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(SYNTHETIC_PROGRAM.format(python=sys.executable), encoding='utf-8')
    path.chmod(0o755)


def _anchor(path):
    data = path.read_bytes()
    first = data.replace(b'\r\n', b'\n').split(b'\n')[0] + b'\n'
    return dict(
        path=str(path),
        **accounting.bound_file(path),
        start_line=1,
        end_line=1,
        lf_sha256=hashlib.sha256(first).hexdigest(),
    )


def _build_candidates(reference_program, reference_dump, reference_lisp,
                      cocoa_program, cocoa_dump, cocoa_lisp,
                      terminal_program, terminal_dump, terminal_lisp):
    reference = dict(
        id='installed-reference',
        kind='reference',
        compiler_leg='normal',
        native=True,
        observer=False,
        test_hooks=False,
        collector='off',
        program=str(reference_program),
        dump=str(reference_dump),
        resource_roots=dict(lisp=str(reference_lisp)),
    )
    selected = [('native-collector', True, True)]
    selected.extend(
        (variant, observer, False)
        for variant in ('baseline', 'native', 'collector', 'native-collector')
        for observer in (False, True)
    )
    selected.append(('baseline', True, True))
    candidates = []
    for variant, observer, hooks in selected:
        for product in ('terminal', 'cocoa'):
            name = (
                product
                + '-'
                + variant
                + ('-observer' if observer else '')
                + ('-diagnostic' if hooks else '')
            )
            for leg in ('normal', 'checking'):
                for collector in (('major', 'young') if 'collector' in variant else ('off',)):
                    profile_id = name + '-' + leg + ('-' + collector if collector != 'off' else '')
                    is_cocoa = product == 'cocoa'
                    candidates.append(
                        dict(
                            id=profile_id,
                            kind=product,
                            compiler_leg=leg,
                            native='native' in variant,
                            observer=observer,
                            test_hooks=hooks,
                            collector=collector,
                            program=str(cocoa_program if is_cocoa else terminal_program),
                            dump=str(cocoa_dump if is_cocoa else terminal_dump),
                            resource_roots=(dict(bundle=str(cocoa_lisp.parent)) if is_cocoa
                                            else dict(lisp=str(terminal_lisp))),
                        )
                    )
    return [reference, *candidates]


def _fixture(root):
    root.mkdir()
    reference_program = root / 'apps/Reference.app/Contents/MacOS/Reference'
    (root / 'apps/Reference.app/Contents/Resources').mkdir(parents=True)
    _write_executable(reference_program)
    reference_dump = root / 'cellar/libexec/reference/32.0.50/aarch64-apple-darwin/Reference.pdmp'
    reference_dump.parent.mkdir(parents=True)
    reference_dump.write_bytes(b'synthetic reference dump\n')
    reference_lisp = root / 'cellar/share/reference/32.0.50/lisp'
    _write_lisp_tree(reference_lisp, 'reference-32.0.50')

    cocoa_program = root / 'candidates/Candidate.app/Contents/MacOS/Candidate'
    _write_executable(cocoa_program)
    cocoa_dump = root / 'candidates/Candidate.app/Contents/MacOS/Candidate.pdmp'
    cocoa_dump.write_bytes(b'synthetic cocoa candidate dump\n')
    cocoa_lisp = root / 'candidates/Candidate.app/Contents/Resources/lisp'
    _write_lisp_tree(cocoa_lisp, 'candidate-cocoa')

    terminal_program = root / 'candidates/terminal/bin/program'
    _write_executable(terminal_program)
    terminal_dump = root / 'candidates/terminal/bin/program.pdmp'
    terminal_dump.write_bytes(b'synthetic terminal candidate dump\n')
    terminal_lisp = root / 'candidates/terminal/share/reference/32.0.50/lisp'
    _write_lisp_tree(terminal_lisp, 'candidate-terminal')

    original = root / 'original'
    source = original / 'source'
    for name in (*HARNESS_NAMES, 'fixtures/stable-suite-runner.el'):
        frozen = source / 'tools' / name
        frozen.parent.mkdir(parents=True, exist_ok=True)
        frozen.write_text(f'# frozen source placeholder for {name}\n', encoding='utf-8')

    rel = 'test/lisp/synthetic-library-root-tests.el'
    test_path = source / rel
    test_path.parent.mkdir(parents=True, exist_ok=True)
    raw = b';; synthetic owned test input; Root/test is untouched\n'
    test_path.write_bytes(raw)
    test_path.chmod(0o644)
    entry = dict(
        path=rel,
        source_path=rel,
        input='tracked',
        mode='100644',
        size=len(raw),
        sha256=e.digest(test_path),
        blob=hashlib.sha1(b'blob ' + str(len(raw)).encode('ascii') + b'\0' + raw).hexdigest(),
    )
    manifest = source / 'compatibility/stable-31.1/sources.json'
    manifest.parent.mkdir(parents=True, exist_ok=True)
    e.write_json(
        manifest,
        dict(
            schema=1,
            source=dict(release=suite.RELEASE, commit=suite.COMMIT),
            file_count=1,
            lisp_file_count=1,
            total_bytes=len(raw),
            entries=[entry],
        ),
    )
    e.write_json(
        source / 'compatibility/stable-31.1/source-context.json',
        dict(
            schema=1,
            source=dict(release=suite.RELEASE, commit=suite.COMMIT),
            selection_sha256='f23d2dde45c9bb0eb3dcd5757cbfc90a71eeeb36b40697590a7fc4ce1f65e999',
            file_count=1,
            total_bytes=32,
            entries=[],
        ),
    )

    synthetic_repo = root / 'synthetic-repo'
    for name in (
        'stable-suite.py',
        'stable-suite-batch.py',
        'runtime-retire.py',
        'reference-corpus.py',
        'fixtures/module-thread-config.h',
        'fixtures/stable-suite-runner.el',
    ):
        real_file = REPO / 'tools' / name
        staged_file = synthetic_repo / 'tools' / name
        staged_file.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(real_file, staged_file)
        shutil.copymode(real_file, staged_file)
        assert e.digest(staged_file) == e.digest(real_file), name
    synthetic_context = synthetic_repo / 'tools/stable-context.py'
    synthetic_context.write_text(SYNTHETIC_CONTEXT, encoding='utf-8')
    synthetic_context.chmod(0o644)

    products = _build_candidates(
        reference_program, reference_dump, reference_lisp,
        cocoa_program, cocoa_dump, cocoa_lisp,
        terminal_program, terminal_dump, terminal_lisp,
    )
    by_id = {product['id']: product for product in products}
    selection = dict(
        path=str(root / 'reference-selection.json'),
        sha256='a' * 64,
        epoch='synthetic-32',
        upstream=dict(version='32.0.50', commit='b' * 40),
    )
    return SimpleNamespace(
        root=root,
        original=original,
        source=source,
        synthetic_repo=synthetic_repo,
        manifest=manifest,
        entry=entry,
        products=products,
        by_id=by_id,
        reference=products[0],
        cocoa_profile=by_id['cocoa-native-normal'],
        terminal_profile=by_id['terminal-collector-normal-young'],
        reference_program=reference_program,
        reference_dump=reference_dump,
        reference_lisp=reference_lisp,
        cocoa_lisp=cocoa_lisp,
        terminal_lisp=terminal_lisp,
        selection=selection,
    )


def _retire_short_containers(root):
    if not root.exists():
        return 0
    retired = 0
    cache_root = (Path.home() / 'AI' / 'caches' / '.noindex').resolve()
    for env_file in root.rglob('*.environment.json'):
        try:
            payload = json.loads(env_file.read_text(encoding='utf-8'))
            container = Path(payload.get('container', ''))
        except Exception:
            continue
        if not container.exists():
            continue
        assert container.resolve() == container and container.parent == cache_root
        assert re.fullmatch(r'f[a-z0-9_]{8}', container.name) and not container.is_symlink()
        node = container.lstat()
        assert (node.st_dev, node.st_ino) == (payload['device'], payload['inode'])
        assert node.st_uid == os.getuid() and stat.S_ISDIR(node.st_mode)
        case_root = Path(payload['case_root'])
        assert case_root.parent == container and case_root.name == 'c'
        final = env_file.with_name(env_file.name.removesuffix('.environment.json')+'.json')
        actual = json.loads(final.read_text(encoding='utf-8'))
        assert actual['owned_container'] == str(container) and actual['owned_state_root'] == str(case_root)
        # This is the actual reaped exit0 process, even when its synthetic ERT
        # outcome failed. Preserve that outcome; only consumed state is retired.
        result = suite._retire_state(case_root,
            ('home','cache','config','state','tmp','native-cache','out','cwd'), final, actual,
            empty_parents=((case_root, case_root.lstat()), (container, node)))
        assert result['status'] == 'complete' and result['all_selected_absent'] is True
        assert not container.exists()
        retired += 1
    return retired


def integration(out):
    fixture = out / 'owned-fixtures'
    started = time.monotonic()
    rows = []
    try:
        context = _fixture(fixture)

        # Case 1: Explicit library selection for App-shaped program without Resources/lisp
        # and full 48-candidate scope admission across terminal and cocoa profiles.
        start = time.monotonic()
        missing_resources_lisp = context.reference_program.parent.parent / 'Resources' / 'lisp'
        assert not missing_resources_lisp.exists()
        try:
            suite._product_paths(
                context.reference_program,
                ['--dump-file=' + str(context.reference_dump)],
                {'FEMACS_GC_OBSERVATION': 'off'},
            )
        except suite.SuiteError as error:
            assert 'product library root is missing' in str(error)
            assert str(missing_resources_lisp) in str(error)
        else:
            raise AssertionError('expected missing Resources/lisp failure without explicit library env')

        actions, pairs, ids = stage.execution_plan('stable', context.products)
        assert actions == () and len(pairs) == 48 and len(ids) == 48
        assert sum(1 for _, cand in pairs if cand['kind'] == 'terminal') == 24
        assert sum(1 for _, cand in pairs if cand['kind'] == 'cocoa') == 24
        for _, cand in pairs:
            env = stage.stable_product_environment(cand, explicit_libraries=True)
            expected_root = (Path(cand['resource_roots']['bundle'])/'lisp' if cand['kind']=='cocoa'
                             else Path(cand['resource_roots']['lisp']))
            assert Path(env['FEMACS_STABLE_PRODUCT_LISP']) == expected_root
            assert env['FEMACS_GC_OBSERVATION'] == 'off'
            assert env['FEMACS_GC_COLLECTOR'] in {'major', 'young'}

        ref_env = stage.stable_product_environment(context.reference, explicit_libraries=True)
        assert ref_env == {
            'FEMACS_GC_OBSERVATION': 'off',
            'FEMACS_STABLE_PRODUCT_LISP': str(context.reference_lisp),
        }
        ref_paths = suite._product_paths(
            context.reference_program,
            ['--dump-file=' + str(context.reference_dump)],
            ref_env,
        )
        assert ref_paths['lisp_root'] == context.reference_lisp.resolve()
        ref_identity = suite._public_product_identity(ref_paths)
        assert ref_identity['lisp'] == suite._tree_identity(context.reference_lisp)

        batch = suite._load_batch()
        comp_case = fixture / 'comp-library-case'
        (comp_case / 'out').mkdir(parents=True)
        comp_layout = batch._prepare_product_library_fixture(
            suite, ref_paths, comp_case, 'test/src/comp-tests.el'
        )
        assert comp_layout['product_lisp'] == str(context.reference_lisp.resolve())
        assert (comp_case / 'out/lisp/emacs-lisp/comp.el').is_file()
        assert (comp_case / 'out/lisp/emacs-lisp/comp.elc').is_file()
        batch._finish_product_library_fixture(suite, comp_layout)
        assert comp_layout['product_sources_unchanged'] is True
        rows.append(
            dict(
                id='app-shaped-missing-resources-lisp-and-48-candidate-scope',
                passed=True,
                seconds=time.monotonic() - start,
            )
        )

        # Case 2: Actual owned harness 6-file capture from Root, Darwin UF_IMMUTABLE
        # lifecycle, and all 5 helper imports + header resolution within the snapshot.
        start = time.monotonic()
        capture_stage = fixture / 'capture-stage'
        capture_stage.mkdir()
        with owner.ArtifactRun(REPO, capture_stage) as artifacts:
            captured_root, selection, identities = stage.stage_stable_harness(
                capture_stage, context.source, artifacts
            )
            assert selection['schema'] == 'femacs-stable-python-harness-v1'
            assert selection['compiled_products_unchanged'] is True
            assert selection['frozen_source_unchanged'] is True
            assert set(selection['files']) == set(HARNESS_NAMES)
            receipt_file = captured_root / 'stable-harness-selection.json'
            assert receipt_file.is_file()
            assert bool(receipt_file.stat().st_flags & stat.UF_IMMUTABLE)

            for name in HARNESS_NAMES:
                real_path = REPO / 'tools' / name
                captured_path = captured_root / 'tools' / name
                frozen_path = context.source / 'tools' / name
                row = selection['files'][name]
                assert captured_path.is_file()
                assert e.digest(captured_path) == e.digest(real_path)
                assert row['source']['identity'] == e.file_identity(real_path)
                assert row['captured']['identity'] == e.file_identity(real_path)
                assert row['frozen']['identity'] == e.file_identity(frozen_path)
                assert row['captured']['identity'] != row['frozen']['identity']
                assert bool(captured_path.stat().st_flags & stat.UF_IMMUTABLE)
                try:
                    captured_path.write_bytes(b'overwrite forbidden\n')
                except OSError:
                    pass
                else:
                    raise AssertionError(f'UF_IMMUTABLE did not block write on {captured_path}')

            captured_suite = _load_module(
                'captured_stable_suite_check', captured_root / 'tools/stable-suite.py'
            )
            captured_batch = captured_suite._load_batch()
            captured_provider = captured_suite._corpus_provider(context.source)
            assert Path(captured_batch.__file__).resolve() == (
                captured_root / 'tools/stable-suite-batch.py'
            ).resolve()
            assert Path(captured_provider.__file__).resolve() == (
                captured_root / 'tools/reference-corpus.py'
            ).resolve()
            assert Path(captured_provider.suite.__file__).resolve() == (
                captured_root / 'tools/stable-suite.py'
            ).resolve()
            header_in_snapshot = (
                Path(captured_batch.__file__).resolve().parent.parent
                / 'tools/fixtures/module-thread-config.h'
            )
            assert header_in_snapshot.is_file()
            assert e.digest(header_in_snapshot) == e.digest(
                REPO / 'tools/fixtures/module-thread-config.h'
            )
            retire_case = fixture / 'retire-check'
            for subdir in ('home', 'cache'):
                (retire_case / subdir).mkdir(parents=True)
            retire_final = fixture / 'retire-final.json'
            e.write_json(retire_final, dict(passed=True))
            retire_result = captured_suite._retire_state(
                retire_case, ('home', 'cache'), retire_final,
                dict(complete=True, exit_code=0, remaining_group_terminated=False)
            )
            assert retire_result['status'] == 'complete'

        journal = json.loads((capture_stage / 'artifact-preservation.json').read_text(encoding='utf-8'))
        assert journal['state'] == 'released' and journal['release_failures'] == []
        for name in HARNESS_NAMES:
            assert not ((captured_root / 'tools' / name).stat().st_flags & stat.UF_IMMUTABLE)
        assert not ((captured_root / 'stable-harness-selection.json').stat().st_flags & stat.UF_IMMUTABLE)
        rows.append(
            dict(
                id='owned-six-file-capture-immutable-lifecycle-and-imports',
                passed=True,
                seconds=time.monotonic() - start,
            )
        )

        # Case 3: Real Python CLI subprocess via runtime-stage._execute_profiles using
        # the captured stable harness, explicit FEMACS_STABLE_PRODUCT_LISP environments,
        # and the unique non-.py runner invariant.
        start = time.monotonic()
        cli_stage = fixture / 'cli-stage'
        cli_stage.mkdir()
        base = context.source / 'build/acceptance' / cli_stage.name
        base.mkdir(parents=True)
        requested_ids = [context.cocoa_profile['id'], context.terminal_profile['id']]
        options = SimpleNamespace(
            execution_phase='stable',
            stable_profile=requested_ids,
            timeout=30.0,
        )
        with mock.patch.object(stage, 'REPO', context.synthetic_repo), \
             owner.ArtifactRun(REPO, cli_stage) as artifacts:
            captured_harness, harness_selection, harness_identities = stage.stage_stable_harness(
                cli_stage, context.source, artifacts
            )
            captured_runner, runner_selection, runner_identities = stage.stage_stable_runner(
                context.synthetic_repo / 'tools/fixtures/stable-suite-runner.el',
                cli_stage,
                context.source,
                artifacts,
            )
            for name in (
                'stable-suite.py',
                'stable-suite-batch.py',
                'runtime-retire.py',
                'reference-corpus.py',
                'fixtures/module-thread-config.h',
            ):
                assert e.digest(captured_harness / 'tools' / name) == e.digest(REPO / 'tools' / name)
            assert e.digest(captured_harness / 'tools/stable-context.py') == e.digest(
                context.synthetic_repo / 'tools/stable-context.py'
            )
            assert bool(captured_runner.stat().st_flags & stat.UF_IMMUTABLE)
            stage_rows = stage._execute_profiles(
                options,
                cli_stage,
                context.source,
                fixture / 'products.json',
                context.products,
                base,
                stage.clean_environment(),
                harness=context.source,
                stable_runner=captured_runner,
                stable_harness=captured_harness,
                explicit_libraries=True,
            )

        assert len(stage_rows) == 2
        by_row_id = {row['id']: row for row in stage_rows}
        cocoa_row = by_row_id['stable-' + context.cocoa_profile['id']]
        terminal_row = by_row_id['stable-' + context.terminal_profile['id']]
        for expected_exit, row in ((0, cocoa_row), (1, terminal_row)):
            proc = row['process']
            if not (proc['complete'] and not proc['remaining_group_terminated'] and proc['exit_code'] == expected_exit):
                logs = cli_stage / row['id']
                e.write_json(
                    out / 'failed-child.json',
                    dict(
                        id=row['id'],
                        process=proc,
                        receipt_validation_error=row.get('receipt_validation_error'),
                        stdout=(logs / 'stdout.txt').read_text(encoding='utf-8')[:16384]
                        if (logs / 'stdout.txt').is_file() else '',
                        stderr=(logs / 'stderr.txt').read_text(encoding='utf-8')[:16384]
                        if (logs / 'stderr.txt').is_file() else '',
                    ),
                )
                raise AssertionError('captured stable-suite CLI child exited unexpectedly; failed-child.json retained')

        assert cocoa_row['passed'] is True and cocoa_row['execution_complete'] is True
        assert cocoa_row['corpus_complete'] is True
        assert cocoa_row['case_count'] == 1 and cocoa_row['passed_count'] == 1 and cocoa_row['failure_count'] == 0

        assert terminal_row['passed'] is False and terminal_row['execution_complete'] is True
        assert terminal_row['corpus_complete'] is False
        assert terminal_row['case_count'] == 1 and terminal_row['passed_count'] == 0 and terminal_row['failure_count'] == 1

        expected_py_sources = {
            str((captured_harness / 'tools' / name).resolve()): e.digest(captured_harness / 'tools' / name)
            for name in (
                'stable-suite.py',
                'stable-suite-batch.py',
                'stable-context.py',
                'runtime-retire.py',
                'reference-corpus.py',
            )
        }
        for profile, row in ((context.cocoa_profile, cocoa_row), (context.terminal_profile, terminal_row)):
            receipt_root = Path(row['receipt']).parent
            completion = json.loads(
                (receipt_root / 'stable-suite-completion.json').read_text(encoding='utf-8')
            )
            assert completion['runtime_environment'] == {
                'reference': stage.stable_product_environment(context.reference, True),
                'candidate': stage.stable_product_environment(profile, True),
            }
            assert completion['products_before']['reference']['lisp'] == suite._tree_identity(
                context.reference_lisp
            )
            assert completion['products_before']['candidate']['lisp'] == suite._tree_identity(
                stage.stable_product_lisp_root(profile)
            )
            sources = completion['harness_sources']
            assert [name for name in sources if Path(name).suffix != '.py'] == [str(captured_runner)]
            py_sources = {name: digest for name, digest in sources.items() if Path(name).suffix == '.py'}
            assert py_sources == expected_py_sources

        rows.append(
            dict(
                id='captured-cli-subprocess-explicit-libraries-and-runner-invariant',
                passed=True,
                seconds=time.monotonic() - start,
            )
        )

        # Case 4: Post-run classifier reconstructs explicit FEMACS_STABLE_PRODUCT_LISP
        # environments for closed stable_product_libraries stages and preserves raw outcomes.
        start = time.monotonic()
        cocoa_root = Path(cocoa_row['receipt']).parent
        terminal_root = Path(terminal_row['receipt']).parent
        cocoa_completion = suite.read_completion(
            cocoa_root,
            context.manifest,
            context.reference['program'],
            context.cocoa_profile['program'],
            ['--dump-file=' + context.reference['dump']],
            ['--dump-file=' + context.cocoa_profile['dump']],
            stage.stable_product_environment(context.reference, True),
            stage.stable_product_environment(context.cocoa_profile, True),
            runner=captured_runner,
        )
        terminal_completion = suite.read_completion(
            terminal_root,
            context.manifest,
            context.reference['program'],
            context.terminal_profile['program'],
            ['--dump-file=' + context.reference['dump']],
            ['--dump-file=' + context.terminal_profile['dump']],
            stage.stable_product_environment(context.reference, True),
            stage.stable_product_environment(context.terminal_profile, True),
            runner=captured_runner,
        )
        terminal_detailed = json.loads(Path(terminal_row['receipt']).read_text(encoding='utf-8'))
        failed_case = terminal_detailed['cases'][0]
        rule = dict(
            profile=context.terminal_profile['id'],
            file=failed_case['file'],
            id=failed_case['id'],
            category='upstream-invalid-test',
            reason='Synthetic contract case verifying explicit product Lisp root reconstruction',
            input_sha256=context.entry['sha256'],
            source_evidence=[_anchor(context.source / context.entry['path'])],
            observations={
                label: accounting.observation_signature(failed_case[label])
                for label in ('reference', 'candidate')
            },
        )
        catalog_path = fixture / 'classification-catalog.json'
        e.write_json(
            catalog_path,
            dict(
                schema=accounting.SCHEMA,
                manifest_sha256=e.digest(context.manifest),
                reference_selection_sha256=context.selection['sha256'],
                contexts={
                    context.cocoa_profile['id']: dict(
                        context_sha256=accounting.context_identity(
                            cocoa_completion, context.cocoa_profile, context.selection
                        ),
                        run_sha256=cocoa_completion['detailed']['sha256'],
                        completion_sha256=cocoa_row['completion_sha256'],
                    ),
                    context.terminal_profile['id']: dict(
                        context_sha256=accounting.context_identity(
                            terminal_completion, context.terminal_profile, context.selection
                        ),
                        run_sha256=terminal_completion['detailed']['sha256'],
                        completion_sha256=terminal_row['completion_sha256'],
                    ),
                },
                rules=[rule],
            ),
        )
        stage_result = fixture / 'stage-results.json'
        e.write_json(
            stage_result,
            dict(
                schema='femacs-runtime-stage-v1',
                action='execute',
                execution_phase='stable',
                execution_complete=True,
                product_identity_unchanged=True,
                complete_receipts_valid=True,
                build_receipt=str(context.original),
                reference_selection=context.selection,
                stable_product_libraries=True,
                stable_harness_selection=dict(harness_selection, unchanged=True),
                stable_runner_selection=dict(runner_selection, unchanged=True),
                cases=stage_rows,
                passed=False,
            ),
        )
        build_report = dict(source_export=dict(synthetic=True))

        def _admission(opts):
            opts._reference_selection_summary = context.selection
            return (
                context.original,
                context.source,
                fixture / 'products.json',
                context.products,
                build_report,
            )

        classify_args = SimpleNamespace(
            classification=catalog_path,
            reference_selection=fixture / 'selection.json',
            build=context.original,
            run_results=[stage_result],
            out=out / 'classified-stage',
        )
        with mock.patch.object(stage, 'read_action_build', side_effect=_admission), \
             mock.patch.object(stage, 'verify_reference_selection', return_value=None), \
             mock.patch.object(stage, 'build_source_owner', return_value=context.original), \
             mock.patch.object(e, 'load_products', return_value=context.products), \
             mock.patch.object(e, 'verify_source_export', return_value=context.source):
            assert accounting.run(stage, classify_args) == 0

        classified = json.loads((classify_args.out / 'results.json').read_text(encoding='utf-8'))
        assert classified['passed'] is True
        assert classified['requested_profiles'] == [
            context.cocoa_profile['id'],
            context.terminal_profile['id'],
        ]
        assert len(classified['remaining_profiles']) == 46
        assert classified['classified_stable_phase_complete'] is False
        assert all(classified['checks'].values())
        by_profile = {row['profile']: row for row in classified['cases']}
        assert by_profile[context.cocoa_profile['id']]['raw_corpus_complete'] is True
        assert by_profile[context.cocoa_profile['id']]['raw_failure_count'] == 0
        assert by_profile[context.terminal_profile['id']]['raw_corpus_complete'] is False
        assert by_profile[context.terminal_profile['id']]['raw_failure_count'] == 1
        assert classified['raw_failures_promoted'] is False
        assert classified['compatibility_accepted'] is False and classified['goal_complete'] is False
        retired_containers = _retire_short_containers(fixture)
        assert retired_containers == 2
        rows.append(
            dict(
                id='closed-stage-classification-reconstructs-explicit-library-environments',
                passed=True,
                seconds=time.monotonic() - start,
                whole_product_admission_isolated=True,
            )
        )

        return dict(
            passed=all(row['passed'] for row in rows),
            cases=rows,
            actual_python_children=6,
            actual_python_cli_children=2,
            synthetic_editor_processes=4,
            original_files_per_synthetic_run=1,
            synthetic_case_observations=4,
            short_containers_retired=retired_containers,
            actual_editor_execution=False,
            original_corpus_execution=False,
            evidence_limitations=[
                'Synthetic executable emits loaded/test-started/test-ended/finished progress without running Lisp (actual_editor_execution=False).',
                'CLI subprocess fixture isolates only original source-context archive acquisition via a pre-capture synthetic stable-context.py module while keeping actual stable-suite, stable-suite-batch, runtime-retire, reference-corpus and module-thread-config.h bytes.',
                'Whole-product build admission is isolated; real editor execution across admitted products and original help-fns/module corpus files is covered by the separate D-only real-editor functional run.',
            ],
            seconds=time.monotonic() - started,
            goal_complete=False,
        )
    finally:
        _retire_short_containers(fixture)
        shutil.rmtree(fixture, ignore_errors=True)


def _verify_functional_inputs(functional, real_functional=None):
    if functional is None:
        raise ValueError('protection requires accepted functional receipts')
    first_path = e.safe_path(functional)
    first = e.bounded_json(first_path, 16 * 1024 * 1024)
    if real_functional is None and first.get('schema') == 'femacs-p464-closed-functional-review-v1':
        pins = {
            Path(row['path']).parent.name: row
            for row in first.get('source_pins', [])
            if isinstance(row, dict) and isinstance(row.get('path'), str)
        }
        syn_pin = pins.get('p464-library-roots-integration-2.noindex')
        real_pin = pins.get('p464-real-product-libraries-functional-2.noindex')
        if not syn_pin or not real_pin:
            raise ValueError('closed functional package missing required synthetic/real pins')
        syn_path = e.safe_path(syn_pin['path'])
        real_path = e.safe_path(real_pin['path'])
        if e.digest(syn_path) != syn_pin['sha256'] or e.digest(real_path) != real_pin['sha256']:
            raise ValueError('closed functional package pin digest mismatch')
        syn_data = e.bounded_json(syn_path, 16 * 1024 * 1024)
        real_data = e.bounded_json(real_path, 16 * 1024 * 1024)
    else:
        if real_functional is None:
            raise ValueError('protection requires both --functional and --real-functional receipts')
        syn_path = first_path
        real_path = e.safe_path(real_functional)
        syn_data = first
        real_data = e.bounded_json(real_path, 16 * 1024 * 1024)
    if (syn_data.get('passed') is not True
            or syn_data.get('actual_editor_execution') is not False
            or syn_data.get('owned_fixture_retired') is not True
            or len(syn_data.get('cases', [])) != 4
            or not all(row.get('passed') is True for row in syn_data['cases'])
            or syn_data.get('actual_python_children') != 6
            or syn_data.get('actual_python_cli_children') != 2
            or syn_data.get('synthetic_editor_processes') != 4):
        raise ValueError('synthetic functional integration receipt is not a passing P464 receipt')
    checks = real_data.get('checks')
    if (real_data.get('schema') != 'femacs-p464-real-product-library-functional-v1'
            or real_data.get('passed') is not True
            or real_data.get('actual_editor_execution') is not True
            or real_data.get('registered_ids_equal') is not True
            or real_data.get('status_maps_equal') is not True
            or len(real_data.get('cases', [])) != 7
            or not all(row.get('passed') is True for row in real_data['cases'])
            or not isinstance(checks, dict) or len(checks) != 8
            or not all(value is True for value in checks.values())):
        raise ValueError('real functional integration receipt is not a passing P464 receipt')
    syn_sha = e.digest(syn_path)
    real_sha = e.digest(real_path)
    return dict(
        synthetic_path=str(syn_path),
        synthetic_sha256=syn_sha,
        real_path=str(real_path),
        real_sha256=real_sha,
    )


def protection(out, functional, real_functional=None):
    bound_functional = _verify_functional_inputs(functional, real_functional)
    fixture = out / 'owned-fixtures'
    started = time.monotonic()
    rows = []
    try:
        context = _fixture(fixture)

        def check(name, operation, rejected=True, expected=(ValueError, suite.SuiteError, KeyError, TypeError, SystemExit)):
            start = time.monotonic()
            try:
                result = operation()
            except expected as error:
                if isinstance(error, SystemExit):
                    assert rejected and error.code != 0, name
                else:
                    assert rejected, name
            else:
                assert not rejected and result is True, name
            rows.append(dict(id=name, passed=True, seconds=time.monotonic() - start))

        def all_rejected(operations):
            for operation in operations:
                try:
                    operation()
                except (ValueError, suite.SuiteError, KeyError, TypeError, OSError):
                    continue
                raise AssertionError('each independent invalid input must be rejected')
            return True

        bad_syn = fixture / 'bad-synthetic-functional.json'
        e.write_json(bad_syn, dict(passed=False, actual_editor_execution=False))
        bad_real = fixture / 'bad-real-functional.json'
        e.write_json(
            bad_real,
            dict(
                schema='femacs-p464-real-product-library-functional-v1',
                passed=True,
                actual_editor_execution=False,
            ),
        )
        check(
            'functional-prerequisite-gate-rejects-missing-or-failed-inputs',
            lambda: all_rejected([
                lambda: _verify_functional_inputs(None, bound_functional['real_path']),
                lambda: _verify_functional_inputs(bad_syn, bound_functional['real_path']),
                lambda: _verify_functional_inputs(bound_functional['synthetic_path'], bad_real),
            ]),
            rejected=False,
        )

        check(
            'app-shaped-reference-without-resources-lisp-rejected-unless-explicit',
            lambda: bool(
                suite._product_paths(
                    context.reference_program,
                    ['--dump-file=' + str(context.reference_dump)],
                    {'FEMACS_GC_OBSERVATION': 'off'},
                )
            ),
        )
        check(
            'direct-lisp-vs-cocoa-bundle-lisp-resolution',
            lambda: (
                stage.stable_product_lisp_root(context.reference) == context.reference_lisp.resolve()
                and stage.stable_product_lisp_root(context.terminal_profile) == context.terminal_lisp.resolve()
                and stage.stable_product_lisp_root(context.cocoa_profile) == context.cocoa_lisp.resolve()
                and 'lisp' not in context.cocoa_profile['resource_roots']
                and 'bundle' in context.cocoa_profile['resource_roots']
            ),
            rejected=False,
        )

        empty_bundle = fixture / 'empty-cocoa-bundle/Contents/Resources'
        empty_bundle.mkdir(parents=True)
        check(
            'cocoa-missing-bundle-lisp-directory-rejected',
            lambda: bool(
                stage.stable_product_lisp_root(
                    dict(context.cocoa_profile, resource_roots=dict(bundle=str(empty_bundle)))
                )
            ),
        )
        check(
            'terminal-without-direct-lisp-key-rejected',
            lambda: bool(
                stage.stable_product_lisp_root(
                    dict(context.terminal_profile, resource_roots=dict(bundle=str(context.cocoa_lisp.parent)))
                )
            ),
        )
        check(
            'relative-product-lisp-root-rejected',
            lambda: bool(
                suite._product_paths(
                    context.reference_program,
                    ['--dump-file=' + str(context.reference_dump)],
                    {'FEMACS_GC_OBSERVATION': 'off', 'FEMACS_STABLE_PRODUCT_LISP': 'relative/lisp'},
                )
            ),
        )
        check(
            'nul-byte-product-lisp-root-rejected',
            lambda: bool(
                suite._product_paths(
                    context.reference_program,
                    ['--dump-file=' + str(context.reference_dump)],
                    {'FEMACS_GC_OBSERVATION': 'off', 'FEMACS_STABLE_PRODUCT_LISP': str(context.reference_lisp) + '\x00bad'},
                )
            ),
        )
        check(
            'missing-product-lisp-root-rejected',
            lambda: bool(
                suite._product_paths(
                    context.reference_program,
                    ['--dump-file=' + str(context.reference_dump)],
                    {'FEMACS_GC_OBSERVATION': 'off', 'FEMACS_STABLE_PRODUCT_LISP': str(fixture / 'absent-lisp')},
                )
            ),
        )
        regular_file_root = fixture / 'not-a-directory.el'
        regular_file_root.write_text(';; file, not directory\n', encoding='utf-8')
        check(
            'regular-file-product-lisp-root-rejected',
            lambda: bool(
                suite._product_paths(
                    context.reference_program,
                    ['--dump-file=' + str(context.reference_dump)],
                    {'FEMACS_GC_OBSERVATION': 'off', 'FEMACS_STABLE_PRODUCT_LISP': str(regular_file_root)},
                )
            ),
        )
        symlink_lisp = fixture / 'symlink-lisp'
        symlink_lisp.symlink_to(context.reference_lisp, target_is_directory=True)
        check(
            'symlink-product-lisp-root-rejected',
            lambda: all_rejected([
                lambda: suite._product_paths(context.reference_program,
                    ['--dump-file=' + str(context.reference_dump)],
                    {'FEMACS_GC_OBSERVATION': 'off', 'FEMACS_STABLE_PRODUCT_LISP': str(symlink_lisp)}),
                lambda: stage.stable_product_lisp_root(
                    dict(context.reference, resource_roots=dict(lisp=str(symlink_lisp)))),
            ]),
            rejected=False,
        )

        def run_suite_env(ref_env_obj, cand_env_obj, tag):
            unused = fixture / f'unused-suite-{tag}'
            rc = suite.main([
                '--repo', str(context.source),
                '--manifest', str(context.manifest),
                '--out', str(unused),
                '--reference', str(context.reference_program),
                '--candidate', str(context.cocoa_profile['program']),
                '--reference-dump-args', json.dumps(['--dump-file=' + str(context.reference_dump)]),
                '--candidate-dump-args', json.dumps(['--dump-file=' + str(context.cocoa_profile['dump'])]),
                '--reference-env', json.dumps(ref_env_obj),
                '--candidate-env', json.dumps(cand_env_obj),
            ])
            assert rc == 2 and not unused.exists()
            return True

        check(
            'env-allowlist-rejects-unknown-key-before-output',
            lambda: run_suite_env(
                {'FEMACS_GC_OBSERVATION': 'off', 'FEMACS_PRODUCT_LISP_ROOT': str(context.reference_lisp)},
                stage.stable_product_environment(context.cocoa_profile, True),
                'unknown-key',
            ),
            rejected=False,
        )
        check(
            'env-allowlist-rejects-invalid-collector-and-observation-before-output',
            lambda: (
                run_suite_env(
                    {'FEMACS_GC_OBSERVATION': 'verbose', 'FEMACS_STABLE_PRODUCT_LISP': str(context.reference_lisp)},
                    stage.stable_product_environment(context.cocoa_profile, True),
                    'bad-obs',
                )
                and run_suite_env(
                    stage.stable_product_environment(context.reference, True),
                    {'FEMACS_GC_OBSERVATION': 'off', 'FEMACS_GC_COLLECTOR': 'generational',
                     'FEMACS_STABLE_PRODUCT_LISP': str(context.cocoa_lisp)},
                    'bad-collector',
                )
            ),
            rejected=False,
        )
        check(
            'env-allowlist-rejects-invalid-explicit-lisp-root-before-output',
            lambda: (
                run_suite_env(
                    {'FEMACS_GC_OBSERVATION': 'off', 'FEMACS_STABLE_PRODUCT_LISP': 'relative/lisp'},
                    stage.stable_product_environment(context.cocoa_profile, True),
                    'rel-root',
                )
                and run_suite_env(
                    {'FEMACS_GC_OBSERVATION': 'off', 'FEMACS_STABLE_PRODUCT_LISP': str(symlink_lisp)},
                    stage.stable_product_environment(context.cocoa_profile, True),
                    'sym-root',
                )
            ),
            rejected=False,
        )

        def assert_cli_rejected(argv, tag):
            unused = fixture / f'unused-cli-{tag}'
            try:
                stage.main([*argv, '--out', str(unused)])
            except SystemExit as error:
                assert error.code == 2 and not unused.exists()
                return True
            raise AssertionError(f'expected CLI rejection for {tag}')

        check(
            'cli-rejects-non-execute-action-before-output',
            lambda: (
                assert_cli_rejected(
                    ['--action', 'prepare', '--build', str(context.original),
                     '--reference-selection', str(fixture / 'sel.json'), '--stable-product-libraries'],
                    'prepare',
                )
                and assert_cli_rejected(
                    ['--action', 'components', '--build', str(context.original),
                     '--reference-selection', str(fixture / 'sel.json'), '--stable-product-libraries'],
                    'components',
                )
            ),
            rejected=False,
        )
        check(
            'cli-rejects-semantics-and-performance-phases-before-output',
            lambda: (
                assert_cli_rejected(
                    ['--action', 'execute', '--execution-phase', 'semantics', '--build', str(context.original),
                     '--reference-selection', str(fixture / 'sel.json'), '--stable-product-libraries'],
                    'semantics',
                )
                and assert_cli_rejected(
                    ['--action', 'execute', '--execution-phase', 'performance', '--build', str(context.original),
                     '--reference-selection', str(fixture / 'sel.json'), '--stable-product-libraries'],
                    'performance',
                )
            ),
            rejected=False,
        )
        check(
            'cli-rejects-missing-reference-selection-before-output',
            lambda: assert_cli_rejected(
                ['--action', 'execute', '--execution-phase', 'stable', '--build', str(context.original),
                 '--stable-product-libraries'],
                'no-ref-sel',
            ),
            rejected=False,
        )
        check(
            'execution-plan-preserves-48-candidates-and-rejects-diagnostic',
            lambda: (
                len(stage.execution_plan('stable', context.products)[1]) == 48
                and all(
                    stage.execution_plan('stable', context.products, [cand['id']])[2] == ['stable-' + cand['id']]
                    for _, cand in stage.execution_plan('stable', context.products)[1]
                )
            ),
            rejected=False,
        )
        check(
            'execution-plan-rejects-diagnostic-profile-selection',
            lambda: bool(
                stage.execution_plan(
                    'stable',
                    context.products,
                    ['cocoa-native-collector-observer-diagnostic-normal-major'],
                )
            ),
        )

        snap_out = fixture / 'snapshot-check'
        snap_out.mkdir()
        snap_root, snap_sel, _ = stage.stage_stable_harness(snap_out, context.source)
        snap_suite = _load_module('prot_snap_suite', snap_root / 'tools/stable-suite.py')
        snap_batch = snap_suite._load_batch()
        snap_provider = snap_suite._corpus_provider(context.source)
        check(
            'six-file-snapshot-header-and-same-directory-imports',
            lambda: (
                set(snap_sel['files']) == set(HARNESS_NAMES)
                and Path(snap_batch.__file__).resolve() == (snap_root / 'tools/stable-suite-batch.py').resolve()
                and Path(snap_provider.__file__).resolve() == (snap_root / 'tools/reference-corpus.py').resolve()
                and Path(snap_provider.suite.__file__).resolve() == (snap_root / 'tools/stable-suite.py').resolve()
                and (Path(snap_batch.__file__).resolve().parent.parent / 'tools/fixtures/module-thread-config.h').is_file()
            ),
            rejected=False,
        )

        header_target = context.synthetic_repo / 'tools/fixtures/module-thread-config.h'
        header_bytes = header_target.read_bytes()
        header_target.unlink()
        try:
            with mock.patch.object(stage, 'REPO', context.synthetic_repo):
                missing_snap = fixture/'missing-header-snapshot'
                missing_snap.mkdir()
                all_rejected([lambda: stage.stage_stable_harness(missing_snap, context.source)])
                header_target.symlink_to(REPO / 'tools/fixtures/module-thread-config.h')
                bad_snap = fixture / 'bad-snapshot'
                bad_snap.mkdir()
                check(
                    'capture-rejects-missing-or-symlink-header-input',
                    lambda: all_rejected([lambda: stage.stage_stable_harness(bad_snap, context.source)]),
                    rejected=False,
                )
        finally:
            header_target.unlink()
            header_target.write_bytes(header_bytes)
            header_target.chmod(0o644)

        def run_isolated_postflight(tag, mutator):
            stage_out = fixture / f'postflight-{tag}'
            stage_out.mkdir()
            build_report = dict(passed=True, source_export=dict(synthetic=True))

            def _admission(opts):
                opts._reference_selection_summary = context.selection
                return (
                    context.original,
                    context.source,
                    fixture / 'products.json',
                    context.products,
                    build_report,
                )

            def _fake_execute_profiles(opts, out_dir, src, prod_path, prods, base_dir, env, **kwargs):
                mutator(out_dir, kwargs['stable_harness'])
                return [
                    dict(
                        id='stable-' + context.cocoa_profile['id'],
                        execution_complete=True,
                        passed=True,
                    )
                ]

            opts = SimpleNamespace(
                action='execute',
                execution_phase='stable',
                stable_profile=[context.cocoa_profile['id']],
                stable_runner=None,
                stable_product_libraries=True,
                reference_selection=fixture / 'sel.json',
                timeout=10.0,
            )
            with mock.patch.object(stage, 'REPO', context.synthetic_repo), \
                 mock.patch.object(stage, 'read_action_build', side_effect=_admission), \
                 mock.patch.object(stage, 'verify_reference_selection', return_value=None), \
                 mock.patch.object(e, 'product_identity', side_effect=lambda p, resources=True: {'id': p['id']}), \
                 mock.patch.object(e, 'load_products', return_value=context.products), \
                 mock.patch.object(stage, '_execute_profiles', side_effect=_fake_execute_profiles):
                result = stage.execute_matrix(opts, stage_out, artifacts=None)
            identities_doc = json.loads((stage_out / 'product-identities.json').read_text(encoding='utf-8'))
            return (
                result['stable_product_libraries'] is True
                and result['stable_harness_selection']['unchanged'] is False
                and result['product_identity_unchanged'] is False
                and result['execution_complete'] is False
                and result['passed'] is False
                and 'stable-harness-selection' in identities_doc['identity_errors']
            )

        src_suite = context.synthetic_repo / 'tools/stable-suite.py'
        src_suite_orig = src_suite.read_bytes()
        try:
            check(
                'postflight-detects-source-harness-mutation',
                lambda: run_isolated_postflight(
                    'source-py',
                    lambda _out, _harness: src_suite.write_bytes(src_suite_orig + b'# mutated\n'),
                ),
                rejected=False,
            )
        finally:
            src_suite.write_bytes(src_suite_orig)

        check(
            'postflight-detects-captured-harness-mutation',
            lambda: run_isolated_postflight(
                'captured-py',
                lambda _out, harness: (harness / 'tools/stable-suite-batch.py').write_bytes(b'# mutated\n'),
            ),
            rejected=False,
        )
        check(
            'postflight-detects-captured-header-mutation',
            lambda: run_isolated_postflight(
                'captured-header',
                lambda _out, harness: (harness / 'tools/fixtures/module-thread-config.h').write_bytes(b'/* mutated */\n'),
            ),
            rejected=False,
        )
        check(
            'postflight-detects-selection-receipt-mutation',
            lambda: run_isolated_postflight(
                'selection-json',
                lambda _out, harness: (harness / 'stable-harness-selection.json').write_text('{"mutated":true}\n', encoding='utf-8'),
            ),
            rejected=False,
        )
        frozen_corpus = context.source / 'tools/reference-corpus.py'
        frozen_orig = frozen_corpus.read_bytes()
        try:
            check(
                'postflight-detects-frozen-source-harness-mutation',
                lambda: run_isolated_postflight(
                    'frozen-py',
                    lambda _out, _harness: frozen_corpus.write_bytes(frozen_orig + b'# mutated\n'),
                ),
                rejected=False,
            )
        finally:
            frozen_corpus.write_bytes(frozen_orig)

        # Classifier protections for unverified harness selection and mismatched explicit library env.
        raw_root = fixture / 'classifier-raw'
        raw_root.mkdir()
        runner = REPO / 'tools/fixtures/stable-suite-runner.el'
        ref_env_without_lisp = {'FEMACS_GC_OBSERVATION': 'off'}
        cand_env_without_lisp = {'FEMACS_GC_OBSERVATION': 'off', 'FEMACS_GC_COLLECTOR': 'major'}
        prog = fixture / 'candidates/terminal/bin/program'
        dump = fixture / 'candidates/terminal/bin/program.pdmp'
        (fixture / 'candidates/terminal/lisp').mkdir(parents=True, exist_ok=True)
        _write_lisp_tree(fixture / 'candidates/terminal/lisp', 'fallback-sibling-lisp')
        prods_without_explicit = {
            'reference': suite._public_product_identity(
                suite._product_paths(prog, ['--dump-file=' + str(dump)], ref_env_without_lisp)
            ),
            'candidate': suite._public_product_identity(
                suite._product_paths(prog, ['--dump-file=' + str(dump)], cand_env_without_lisp)
            ),
        }
        event_raw = json.dumps(
            dict(
                event='test-ended',
                file=context.entry['path'],
                test_id='synthetic-case',
                status='passed',
                duration=0.001,
                diagnostic='',
                expected=True,
                skipped=False,
                condition=None,
            ),
            sort_keys=True,
        ).encode('utf-8')
        (raw_root / '0-reference.events.jsonl').write_bytes(event_raw + b'\n')
        (raw_root / '0-candidate.events.jsonl').write_bytes(event_raw + b'\n')
        case_obj = dict(
            test_id='synthetic-case',
            status='passed',
            complete=True,
            exit_code=0,
            seconds=0.001,
            diagnostic='',
            assessment=dict(passed=True, status='passed', seconds=0.001, diagnostic=''),
            evidence=dict(
                path='0-reference.events.jsonl',
                event_sha256=hashlib.sha256(event_raw).hexdigest(),
                event_ordinal=0,
            ),
        )
        file_side = dict(
            complete=True,
            passed=True,
            timed_out=False,
            remaining_group_terminated=False,
            exit_code=0,
            argv=[str(prog), '--dump-file=' + str(dump)],
            metadata=dict(features=[], emacs_version='32.0.50'),
            progress=dict(path='0-reference.events.jsonl', **accounting.bound_file(raw_root / '0-reference.events.jsonl')),
        )
        report = dict(
            schema=suite.RESULT_SCHEMA,
            source=dict(release=suite.RELEASE, commit=suite.COMMIT),
            manifest_sha256=e.digest(context.manifest),
            selector='all-discovered-ids',
            entrypoint_count=1,
            resource_only_lisp_inputs=0,
            cases=[
                dict(
                    file=context.entry['path'],
                    id='synthetic-case',
                    passed=True,
                    reference=case_obj,
                    candidate=case_obj,
                )
            ],
            files=[
                dict(
                    file=context.entry['path'],
                    registration=dict(passed=True, reference_count=1, missing=[], extra=[], duplicates=[]),
                    reference_id_count=1,
                    candidate_id_count=1,
                    reference=file_side,
                    candidate=file_side,
                )
            ],
            failures=[],
            corpus_complete=True,
            products_before=prods_without_explicit,
            products_after=prods_without_explicit,
            identities_unchanged=True,
            corpus_inputs_unchanged=True,
            source_context_unchanged=True,
            registration=dict(passed=True, reference_count=1, candidate_count=1),
            runtime_environment=dict(reference=ref_env_without_lisp, candidate=cand_env_without_lisp),
            harness_sources={str(runner): e.digest(runner), str(Path(__file__)): e.digest(Path(__file__))},
        )
        suite._write_run_report(raw_root, report)
        raw_completion = suite.read_completion(
            raw_root,
            context.manifest,
            prog,
            prog,
            ['--dump-file=' + str(dump)],
            ['--dump-file=' + str(dump)],
            ref_env_without_lisp,
            cand_env_without_lisp,
            runner=runner,
        )
        cat_path = fixture / 'prot-catalog.json'
        e.write_json(
            cat_path,
            dict(
                schema=accounting.SCHEMA,
                manifest_sha256=e.digest(context.manifest),
                reference_selection_sha256=context.selection['sha256'],
                contexts={
                    context.cocoa_profile['id']: dict(
                        context_sha256=accounting.context_identity(
                            raw_completion, context.cocoa_profile, context.selection
                        ),
                        run_sha256=raw_completion['detailed']['sha256'],
                        completion_sha256=e.digest(raw_root / 'stable-suite-completion.json'),
                    )
                },
                rules=[],
            ),
        )
        bad_stage_unverified = fixture / 'stage-unverified-harness.json'
        raw_case_row = dict(
            id='stable-' + context.cocoa_profile['id'],
            receipt=str(raw_root / 'stable-suite-run.json'),
            receipt_sha256=e.digest(raw_root / 'stable-suite-run.json'),
            completion_sha256=e.digest(raw_root / 'stable-suite-completion.json'),
            execution_complete=True,
            passed=True,
            process=dict(complete=True, exit_code=0, remaining_group_terminated=False),
        )
        e.write_json(
            bad_stage_unverified,
            dict(
                schema='femacs-runtime-stage-v1',
                action='execute',
                execution_phase='stable',
                execution_complete=True,
                product_identity_unchanged=True,
                complete_receipts_valid=True,
                build_receipt=str(context.original),
                reference_selection=context.selection,
                stable_product_libraries=True,
                stable_harness_selection=dict(unchanged=False),
                cases=[raw_case_row],
                passed=True,
            ),
        )

        def _prot_admission(opts):
            opts._reference_selection_summary = context.selection
            return (
                context.original,
                context.source,
                fixture / 'products.json',
                context.products,
                dict(source_export=dict(synthetic=True)),
            )

        with mock.patch.object(stage, 'read_action_build', side_effect=_prot_admission):
            check(
                'classifier-rejects-unverified-harness-selection',
                lambda: bool(
                    accounting.run(
                        stage,
                        SimpleNamespace(
                            classification=cat_path,
                            reference_selection=fixture / 'sel.json',
                            build=context.original,
                            run_results=[bad_stage_unverified],
                            out=fixture / 'unused-classified-1',
                        ),
                    )
                ),
            )

        bad_stage_env = fixture / 'stage-mismatched-env.json'
        e.write_json(
            bad_stage_env,
            dict(
                schema='femacs-runtime-stage-v1',
                action='execute',
                execution_phase='stable',
                execution_complete=True,
                product_identity_unchanged=True,
                complete_receipts_valid=True,
                build_receipt=str(context.original),
                reference_selection=context.selection,
                stable_product_libraries=True,
                stable_harness_selection=dict(unchanged=True),
                cases=[raw_case_row],
                passed=True,
            ),
        )
        with mock.patch.object(stage, 'read_action_build', side_effect=_prot_admission), \
             mock.patch.object(stage, 'verify_reference_selection', return_value=None), \
             mock.patch.object(stage, 'build_source_owner', return_value=context.original), \
             mock.patch.object(e, 'load_products', return_value=context.products), \
             mock.patch.object(e, 'verify_source_export', return_value=context.source):
            check(
                'classifier-rejects-mismatched-explicit-library-environment',
                lambda: (
                    accounting.run(
                        stage,
                        SimpleNamespace(
                            classification=cat_path,
                            reference_selection=fixture / 'sel.json',
                            build=context.original,
                            run_results=[bad_stage_env],
                            out=fixture / 'classified-mismatch-out',
                        ),
                    ) == 1
                    and json.loads(
                        (fixture / 'classified-mismatch-out/results.json').read_text(encoding='utf-8')
                    )['passed'] is False
                ),
                rejected=False,
            )

        return dict(
            passed=all(row['passed'] for row in rows),
            cases=rows,
            synthetic_functional_acceptance_sha256=bound_functional['synthetic_sha256'],
            real_functional_acceptance_sha256=bound_functional['real_sha256'],
            functional_acceptance_sha256=dict(
                synthetic=bound_functional['synthetic_sha256'],
                real=bound_functional['real_sha256'],
            ),
            actual_editor_execution=False,
            original_corpus_execution=False,
            evidence_limitations=[
                'Protective unit and component cases run in-process against synthetic product descriptors and files without starting editors or compilers (actual_editor_execution=False).',
                'Binds both passing synthetic (a0eb1b80...) and real-editor (6c4daa64...) functional integration receipts before executing protective checks.',
            ],
            seconds=time.monotonic() - started,
            goal_complete=False,
        )
    finally:
        _retire_short_containers(fixture)
        shutil.rmtree(fixture, ignore_errors=True)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mode', choices=('integration', 'protection'), default='integration')
    parser.add_argument('--functional', '--synthetic-functional', '--functional-acceptance',
                        dest='functional', type=Path)
    parser.add_argument('--real-functional', '--actual-functional',
                        dest='real_functional', type=Path)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args(argv)
    if args.mode == 'protection' and args.functional is None:
        parser.error('protection requires --functional (and --real-functional or a closed functional package)')
    out = e.new_output(args.out)
    result = (
        integration(out)
        if args.mode == 'integration'
        else protection(out, args.functional, args.real_functional)
    )
    result['owned_fixture_retired'] = not (out / 'owned-fixtures').exists()
    e.write_json(out / 'results.json', result)
    return 0 if result['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
