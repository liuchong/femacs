"""Real Python/file integration for exact closed-file copying and deferred retirement.

The miniature editor emits the real progress protocol without executing Lisp.
The outer controller envelope is explicitly synthetic; process birth/reaping,
artifact ownership, CLI children, event checks, hashes and lsof are real.
"""
from pathlib import Path
import argparse
import hashlib
import importlib.util
import json
import os
import shutil
import subprocess
import sys
import time
from types import SimpleNamespace
from unittest import mock

REPO = Path(__file__).resolve().parent.parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


SEED_CHILD = '''from pathlib import Path
import importlib.util, json, sys
from types import SimpleNamespace
from unittest import mock
repo, fixture, output, profile = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]), sys.argv[4]
spec=importlib.util.spec_from_file_location('reuse_fixture',repo/'tools/test-stable-library-roots.py')
f=importlib.util.module_from_spec(spec);spec.loader.exec_module(f)
stage=f.stage;e=f.e
c=SimpleNamespace(**json.loads((fixture/'context.json').read_text()))
source=Path(c.source);products=c.products
before={p['id']:e.product_identity(p,resources=True) for p in products}
base=source/'build/acceptance'/output.name;base.mkdir(parents=True)
output.mkdir()
with mock.patch.object(stage,'REPO',Path(c.synthetic_repo)), f.owner.ArtifactRun(repo,output) as artifacts:
 harness,hs,hi=stage.stage_stable_harness(output,source,artifacts)
 runner,rs,ri=stage.stage_stable_runner(Path(c.synthetic_repo)/'tools/fixtures/stable-suite-runner.el',output,source,artifacts)
 rows=stage._execute_profiles(SimpleNamespace(execution_phase='stable',stable_profile=[profile],timeout=10.0),
  output,source,fixture/'products.json',products,base,stage.clean_environment(),harness=source,
  stable_runner=runner,stable_harness=harness,explicit_libraries=True)
e.write_json(output/'product-identities.json',dict(before=before,after=before,unchanged=True,complete_receipts_valid=True))
e.write_json(output/'stable-stage-context.json',dict(build_receipt=c.original,
 source_manifest_sha256='c'*64,products_sha256=e.digest(fixture/'products.json'),reference_selection=c.selection))
e.write_json(output/'results.json',dict(cases=rows,execution_complete=True,product_identity_unchanged=True,
 complete_receipts_valid=True,build_receipt=c.original,reference_selection=c.selection,stable_product_libraries=True,
 stable_runner_selection=dict(rs,unchanged=True),stable_harness_selection=dict(hs,unchanged=True)))
assert len(rows)==1 and rows[0]['execution_complete'],rows
'''


def integration(out):
    fixture_api = load('reuse_library_fixture', REPO/'tools/test-stable-library-roots.py')
    stage, suite, e = fixture_api.stage, fixture_api.suite, fixture_api.e
    work = out/'owned-fixtures'
    rows, failures = [], []
    started = time.monotonic()
    try:
        c = fixture_api._fixture(work)
        encoded = {key: str(getattr(c, key)) for key in ('source', 'original', 'synthetic_repo')}
        encoded.update(products=c.products, selection=c.selection)
        e.write_json(work/'context.json', encoded)
        e.write_json(work/'products.json', dict(schema=1, products=c.products))
        child = work/'seed-child.py'
        child.write_text(SEED_CHILD)
        batch = suite._load_batch()
        for profile in (c.cocoa_profile['id'], c.terminal_profile['id']):
            t = time.monotonic()
            donor = work/('donor-'+profile)
            command = [sys.executable, '-B', child, REPO, work, donor, profile]
            process = e.run_process(command, work/('process-'+profile), stage.clean_environment(), 60, cwd=REPO)
            assert process['complete'] and process['exit_code'] == 0 and not process['remaining_group_terminated'], process
            journal = json.loads((donor/'artifact-preservation.json').read_text())
            assert journal['owner_pid'] == process['pid'] and journal['state'] == 'released'
            # The standalone guard is supplied by the caller. This synthetic
            # envelope exercises admission fields without claiming a second guard.
            controller = work/('controller-'+profile+'.txt')
            controller.write_text('controlled-run: '+json.dumps(dict(reason='exited', exit_code=0,
                cleanup_complete=True, root_pid=process['pid'], argv_sha256=hashlib.sha256(
                    json.dumps([str(x) for x in command], ensure_ascii=True).encode()).hexdigest()))+'\n')
            spec = work/('reuse-'+profile+'.json')
            e.write_json(spec, dict(schema='femacs-stable-file-reuse-v1', donors=[dict(stage=str(donor),
                controller=str(controller), controller_sha256=e.digest(controller),
                argv=[str(x) for x in command], profile=profile, mode='closed')]))
            target = work/('target-'+profile)
            target.mkdir()
            base = c.source/'build/acceptance'/target.name
            base.mkdir(parents=True)
            options = SimpleNamespace(stable_file_reuse=spec,
                _reference_selection_summary=c.selection, _stable_reuse_source_manifest='c'*64,
                _stable_reuse_products_sha256=e.digest(work/'products.json'),
                execution_phase='stable', stable_profile=[profile], timeout=10.0)
            before = {p['id']: e.product_identity(p, resources=True) for p in c.products}
            with mock.patch.object(stage, 'REPO', c.synthetic_repo), fixture_api.owner.ArtifactRun(REPO, target) as artifacts:
                harness, _, _ = stage.stage_stable_harness(target, c.source, artifacts)
                runner, _, _ = stage.stage_stable_runner(c.synthetic_repo/'tools/fixtures/stable-suite-runner.el',
                    target, c.source, artifacts)
                # Product paths belong to the actual fixture repository. The
                # six-file capture uses its synthetic source-context provider.
                with mock.patch.object(stage, 'REPO', REPO):
                    admission, pins = stage.stage_stable_file_reuse(options, target, c.source,
                        c.products, before, harness, runner, c.original, artifacts)
                actual = stage._execute_profiles(options, target, c.source, work/'products.json',
                    c.products, base, stage.clean_environment(), harness=c.source, stable_runner=runner,
                    stable_harness=harness, explicit_libraries=True, stable_file_reuse=admission)
                assert all(e.file_identity(Path(name)) == pin for name, pin in pins.items())
            assert len(actual) == 1 and actual[0]['execution_complete'], actual
            record = actual[0]
            expected_raw_pass = profile == c.cocoa_profile['id']
            assert record['corpus_complete'] is expected_raw_pass
            assert record['process']['exit_code'] == (0 if expected_raw_pass else 1)
            summary = record['file_reuse']
            assert summary['reused_file_sides'] == 2 and summary['fresh_file_executions'] == 0, summary
            assert summary['retired_logical_bytes_attributed_to_this_run'] == 0
            old = c.source/'build/acceptance'/donor.name/('stable-'+profile)
            new = Path(record['receipt']).parent
            for label in ('reference', 'candidate'):
                name = suite._safe_component(c.entry['path'])
                for suffix in ('.json', '.events.jsonl', '.owner-retirement.json', '.environment.json'):
                    assert (old/'results'/label/(name+suffix)).read_bytes() == (new/'results'/label/(name+suffix)).read_bytes()
            rows.append(dict(id='closed-'+('passed' if expected_raw_pass else 'failed')+'-file-copy',
                passed=True, seconds=time.monotonic()-t, reused_file_sides=2, new_editor_standins=0,
                original_failure_count=record['failure_count']))

        t = time.monotonic()
        container = work/'occupied-case'
        root = container/'case'
        root.mkdir(parents=True)
        for name in batch.STATE_NAMES:
            (root/name).mkdir()
        held = root/'cache/held'
        writer = subprocess.run([sys.executable, '-B', '-c',
            'from pathlib import Path;import sys;Path(sys.argv[1]).write_bytes(b"retained content")', str(held)],
            capture_output=True, timeout=5)
        assert writer.returncode == 0
        holder = subprocess.Popen([sys.executable, '-B', '-c',
            'import sys;f=open(sys.argv[1],"rb");print("ready",flush=True);sys.stdin.buffer.read(1);f.close()', str(held)],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
        try:
            import select
            assert select.select([holder.stdout], [], [], 5)[0] and holder.stdout.readline() == b'ready\n'
            final = work/'occupancy-evidence.json'
            value = dict(complete=True, passed=True, exit_code=writer.returncode,
                         remaining_group_terminated=False)
            e.write_json(final, value)
            value['file_result'] = dict(path=final.name, bytes=final.stat().st_size, sha256=e.digest(final))
            value['state_retirement_path'] = final.stem+'.owner-retirement.json'
            queue = batch._DeferredRetirements(suite)
            queue.submit(final, value, root, ((root, root.lstat()), (container, container.lstat())))
            assert len(queue.pending) == 1 and root.exists() and not final.with_name(final.stem+'.owner-retirement.json').exists()
            assert value['state_retirement']['attempt_observation']['clear'] is False
            holder.stdin.write(b'x'); holder.stdin.flush()
            holder.communicate(timeout=5)
            assert holder.returncode == 0
            queue.sweep(final=True)
            assert value['state_retirement']['status'] == 'complete' and not container.exists()
            assert value['state_retirement']['expected_state_verified'] is True
            assert e.digest(final) == value['file_result']['sha256']
        finally:
            if holder.poll() is None:
                holder.terminate(); holder.communicate(timeout=5)
        rows.append(dict(id='real-holder-release-and-single-sealed-retirement', passed=True,
                         seconds=time.monotonic()-t))
    except Exception as error:
        failures.append(f'{type(error).__name__}: {error}')
    finally:
        short = fixture_api._retire_short_containers(work) if work.exists() else 0
        if work.exists():
            shutil.rmtree(work)
        result = dict(schema='femacs-stable-file-reuse-integration-v1', passed=not failures and len(rows) == 3,
            cases=rows, failures=failures, seconds=time.monotonic()-started,
            actual_editor_execution=False, original_corpus_execution=False,
            synthetic_controller_envelope=True, source_context_provider_is_synthetic=True,
            owned_fixture_retired=not work.exists(), short_containers_retired=short,
            compatibility_accepted=False, goal_complete=False)
        e.write_json(out/'results.json', result)
    return 0 if result['passed'] else 1


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args(argv)
    out = args.out.absolute()
    if out.resolve() != out or out.exists() or not out.is_relative_to(REPO/'build/acceptance'):
        parser.error('fresh owned acceptance output required')
    out.mkdir(parents=True)
    return integration(out)


if __name__ == '__main__':
    raise SystemExit(main())
