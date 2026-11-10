"""Final-D build/execution matrix owner. Never a whole-Goal completion tool.

build, prepare and components invoke compilers: use build protection. execute invokes
products and corpus/performance processes: use runtime protection. Reuse one
external controlled-run owner; this tool never nests resource controllers.
Every output is fresh and earlier failures remain. No remote CI is used.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import shutil
import sys

REPO = Path(__file__).resolve().parent.parent
COORDINATOR_SHA256 = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
spec = importlib.util.spec_from_file_location("femacs_runtime_execution", REPO / "tools/runtime-execution.py")
execution = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = execution
spec.loader.exec_module(execution)
owner_spec = importlib.util.spec_from_file_location("femacs_runtime_owner", REPO / "tools/runtime-owner.py")
owner = importlib.util.module_from_spec(owner_spec)
owner_spec.loader.exec_module(owner)

EXECUTION_PHASES = {
    'all': ('ert', 'negative', 'dump', 'relocation', 'performance'),
    'semantics': ('ert', 'negative', 'dump', 'relocation'),
    'stable': (),
    'performance': ('performance',),
}


def execution_plan(phase, products, stable_profiles=()):
    """Select complete pairs only; compiled products and corpus scope stay intact."""
    if phase not in EXECUTION_PHASES:
        raise ValueError('unknown execution phase')
    if stable_profiles and phase != 'stable':
        raise ValueError('stable profiles require the stable execution phase')
    pairs = []
    if phase in ('all', 'stable'):
        pairs = [(reference, candidate) for reference in products if reference['kind'] == 'reference'
                 for candidate in products if candidate['kind'] != 'reference' and not candidate['test_hooks']]
    actions = EXECUTION_PHASES[phase]
    ids = [*actions, *('stable-' + candidate['id'] for _, candidate in pairs)]
    if not ids or len(ids) != len(set(ids)):
        raise ValueError('empty or duplicate execution scope')
    if stable_profiles:
        if len(stable_profiles) != len(set(stable_profiles)):
            raise ValueError('duplicate stable profile selection')
        available = {candidate['id'] for _, candidate in pairs}
        if set(stable_profiles) - available:
            raise ValueError('unknown or diagnostic stable profile selection')
        pairs = [(reference, candidate) for reference, candidate in pairs
                 if candidate['id'] in stable_profiles]
        ids = ['stable-' + candidate['id'] for _, candidate in pairs]
    return actions, pairs, ids


def clean_environment():
    return {key:value for key,value in os.environ.items()
            if not key.startswith("FEMACS_") and key not in execution.FORBIDDEN_ENV}


def verify_build_owner(path, identity):
    try:
        path = execution.safe_path(path)
        if not path.is_file() or not os.access(path, os.X_OK) or execution.digest(path) != identity:
            raise ValueError("runtime build owner missing, unsafe or changed")
    except (OSError, ValueError) as error:
        raise ValueError("runtime build owner missing, unsafe or changed") from error


def stage_build_owner(source, out):
    source = execution.safe_path(source)
    out = execution.safe_path(out)
    if not source.is_file() or not os.access(source, os.X_OK):
        raise ValueError("runtime build owner input must be an executable regular file")
    repository = execution.safe_path(REPO)
    try:
        relative = out.relative_to(repository)
    except ValueError as error:
        raise ValueError("runtime build owner requires an owned build output") from error
    if len(relative.parts) < 2 or relative.parts[0] != "build":
        raise ValueError("runtime build owner requires an owned build output")
    owner_id = hashlib.sha256(relative.as_posix().encode("utf-8")).hexdigest()
    root = execution.safe_path(repository / ".femacs-cache/runtime-build-owners" / owner_id, must_exist=False)
    root.mkdir(parents=True)
    identity = execution.digest(source)
    owner = root / "runtime-build-driver"
    with source.open("rb") as original, owner.open("xb") as retained:
        shutil.copyfileobj(original, retained, length=1024 * 1024)
    owner.chmod(source.stat().st_mode & 0o777)
    if execution.digest(source) != identity:
        raise ValueError("runtime build owner input changed during capture")
    verify_build_owner(owner, identity)
    receipt = dict(schema="femacs-runtime-build-owner-v1", path=owner.name,
                   owner_root=str(root.relative_to(repository)), sha256=identity, bytes=owner.stat().st_size,
                   original_cache_locator=str(source.relative_to(out)))
    execution.write_json(root / "receipt.json", receipt)
    execution.write_json(out / "build-owner-receipt.json", receipt)
    return owner, identity


def build_matrix(options, out, artifacts=None):
    source_export = execution.freeze_source(out)
    source = execution.verify_source_export(out, source_export)
    if artifacts is not None:
        artifacts.select_source(out, source_export)
    reference = dict(id="installed-reference", kind="reference", compiler_leg="normal",
                     native=options.reference_native, observer=False, test_hooks=False, collector="off",
                     program=str(execution.safe_path(options.reference)), dump=str(execution.safe_path(options.reference_dump)),
                     resource_roots={"lisp":str(execution.safe_path(options.reference_lisp)),
                                     "data":str(execution.safe_path(options.reference_data))})
    reference_identity = execution.product_identity(reference, resources=True)
    carried, cocoa_carried = None, None
    if (getattr(options, "carry_terminal_from", None) is not None
            or getattr(options, "carry_native_cocoa_from", None) is not None):
        recovery_spec = importlib.util.spec_from_file_location("femacs_carry_recovery", REPO / "tools/runtime-resume.py")
        recovery = importlib.util.module_from_spec(recovery_spec)
        recovery_spec.loader.exec_module(recovery)
        if getattr(options, "carry_terminal_from", None) is not None:
            carried = recovery.carry_terminals(options.carry_terminal_from, options.carry_controller_result,
                                              source_export, out, reference,
                                              retention_proof=getattr(options, 'terminal_retention_proof', None))
        if getattr(options, "carry_native_cocoa_from", None) is not None:
            cocoa_carried = recovery.carry_native_cocoa(options.carry_native_cocoa_from,
                                                       options.carry_native_cocoa_controller_result,
                                                       source_export, out, reference)
    env = clean_environment()
    if artifacts is not None:
        env[owner.ARTIFACT_ENV] = str(artifacts.journal)
        env['FEMACS_RUNTIME_HELPER_PYTHON'] = sys.executable
    cargo_target = out / "cargo"
    env["CARGO_TARGET_DIR"] = str(cargo_target)
    compiled = execution.run_process(["cargo", "build", "--release", "--manifest-path", source / "Cargo.toml",
                                      "-p", "xtask", "-j", "2"], out / "xtask-build", env, None, cwd=source)
    report = dict(source_export=source_export, xtask_build=compiled, xtask_profile="release",
                  builds=carried["builds"] if carried else [],
                  products=[reference, *(carried["products"] if carried else [])],
                  reference_identity=reference_identity, passed=False,
                  source_cohorts=[source_export], uniform_source_snapshot=not bool(carried))
    if carried:
        report.update(original_history=carried["original_history"],
                      terminal_source_certificate=carried["certificate"])
        report["source_cohorts"].insert(0, carried["original_source"])
    retained_cocoa_ids = set()
    if cocoa_carried:
        report["builds"].extend(cocoa_carried["builds"])
        report["products"].extend(cocoa_carried["products"])
        report["cocoa_original_history"] = cocoa_carried["original_history"]
        report["native_cocoa_source_certificate"] = cocoa_carried["certificate"]
        report["source_cohorts"].insert(0, cocoa_carried["original_source"])
        report["uniform_source_snapshot"] = False
        retained_cocoa_ids = {row["id"] for row in cocoa_carried["builds"]}
    # Preserve completed input/compiler evidence even if capture itself fails.
    execution.write_json(out / "partial.json", report)
    if compiled["exit_code"] != 0 or not compiled["complete"]:
        return report
    compiled_xtask = cargo_target / "release/xtask"
    if not compiled_xtask.is_file():
        report["error"] = "build returned without actual xtask executable"
        return report
    xtask, identity = stage_build_owner(compiled_xtask, out)
    report["xtask_sha256"] = identity
    report["build_owner_receipt"] = str(xtask.parent / "receipt.json")
    current_host = dict(role="fresh", sha256=identity, source_manifest_sha256=source_export["source_manifest_sha256"])
    with owner.protect(xtask, identity, out, REPO):
        execution.write_json(out / "partial.json", report)
        # Exercise both product representatives before their remaining matrix.
        selected = [("native-collector", True, True)]
        selected.extend((variant, observer, False) for variant in ("baseline", "native", "collector", "native-collector")
                        for observer in (False, True))
        selected.append(("baseline", True, True))
        for variant, observer, hooks in selected:
            for product in ("terminal", "cocoa"):
                if carried and product == "terminal":
                    continue
                verify_build_owner(xtask, identity)
                execution.verify_source_export(out, source_export)
                name = product + "-" + variant + ("-observer" if observer else "") + ("-diagnostic" if hooks else "")
                if name in retained_cocoa_ids:
                    continue
                root = source / "build/runtime" / name
                argv = [xtask, "runtime-build", "--product", product, "--variant", variant, "--out", root]
                if observer:
                    argv.append("--observer")
                if hooks:
                    argv.append("--test-hooks")
                if "native" in variant:
                    argv.extend(["--backend-prefix", options.backend_prefix])
                result = execution.run_process(argv, out / "builds" / name, env, None, cwd=source)
                row = dict(id=name, product=product, variant=variant, observer=observer, test_hooks=hooks, process=result,
                           passed=result["exit_code"] == 0 and result["complete"] and not result["remaining_group_terminated"],
                           source_cohort=source_export["source_manifest_sha256"], host_provenance=current_host)
                report["builds"].append(row)
                # Preserve a completed leg even if its independent sibling failed.
                # It does not turn the whole matrix green or hide the failed leg.
                for leg in ("normal", "checking"):
                    leg_root = root if leg == "normal" else root / "checking"
                    receipt_path = leg_root / "runtime-product.json"
                    if not receipt_path.is_file():
                        continue
                    receipt = execution.bounded_json(receipt_path, 64 * 1024 * 1024)
                    if receipt.get("build_complete") is not True:
                        continue
                    program_rel, dump_rel, _, resources = execution.product_receipt_layout(receipt, leg_root)
                    program, dump = leg_root / program_rel, leg_root / dump_rel
                    for collector in (("major", "young") if "collector" in variant else ("off",)):
                        profile_id = name + "-" + leg + ("-" + collector if collector != "off" else "")
                        report["products"].append(dict(id=profile_id, kind=product, compiler_leg=leg,
                                                       native="native" in variant, observer=observer, test_hooks=hooks,
                                                       collector=collector, program=str(program), dump=str(dump),
                                                       resource_roots=resources, product_receipt=str(receipt_path),
                                                       source_cohort=source_export["source_manifest_sha256"],
                                                       host_provenance=current_host))
                execution.write_json(out / "partial.json", report)
                for leg in ("normal", "checking"):
                    leg_root = root if leg == 'normal' else root / 'checking'
                    outcome = leg_root / 'runtime-build-outcome.json'
                    if outcome.is_file():
                        completed = execution.bounded_json(outcome)
                        if (completed.get('artifact_preservation_required') is True
                                and completed.get('artifact_preservation_complete') is False
                                and (leg_root / 'runtime-product.json').is_file()):
                            report['error'] = 'completed compiler leg preservation failed; remaining matrix not started'
                            return report
                verify_build_owner(xtask, identity)
                execution.verify_source_export(out, source_export)
                if not row["passed"]:
                    # Preserve both compiler legs and their diagnostics before
                    # correcting the frozen source in a fresh build. Later
                    # configurations cannot certify this failed source.
                    report["error"] = "compiler configuration failed; remaining matrix not started"
                    report["source_root"] = str(source)
                    return report
                retirement_spec = importlib.util.spec_from_file_location(
                    "femacs_runtime_case_retirement", source / "tools/runtime-retire.py")
                retirement = importlib.util.module_from_spec(retirement_spec)
                retirement_spec.loader.exec_module(retirement)
                report.setdefault("compiler_copy_retirements", {})[name] = {
                    leg: retirement.retire(leg_root / "objects", ["units"],
                        leg_root / "runtime-build-outcome.json", result)
                    for leg, leg_root in (("normal", root), ("checking", root / "checking"))}
                execution.write_json(out / "partial.json", report)
        if execution.product_identity(reference, resources=True) != reference_identity:
            raise ValueError("installed reference changed during matrix build")
        execution.write_json(out / "products.json", dict(schema=1, products=report["products"],
                                                        source_cohorts=report["source_cohorts"]))
        execution.load_products(out / "products.json")
        report["passed"] = (len(report["builds"]) == 20 and all(row["passed"] for row in report["builds"])
                            and len(report["products"]) == 61)
        # 4 ordinary product/variant/observer matrices per product with collector
        # mode duplication plus the two diagnostic variants: 60 profiles + ref.
        report["source_root"] = str(source)
        report["product_list"] = str(out / "products.json")
        report["semantic_tests_executed"] = False
        return report


def build_source_lineage(root, report):
    """Resolve every pinned recovery link to the one physical frozen export."""
    root = execution.safe_path(root)
    exported = report['source_export']
    source = execution.safe_path(exported['source'])
    lineage = []
    while True:
        if (not root.is_relative_to(REPO / 'build') or root == REPO / 'build'
                or any(root.is_relative_to(prior) or prior.is_relative_to(root)
                       for prior in lineage)):
            raise ValueError('recovery lineage requires separate owned build roots without cycles')
        if any(root.name == prior.name for prior in lineage):
            raise ValueError('recovery producer names must be unique within their source lineage')
        if (report.get('source_export') != exported
                or ((lineage or 'original_build' in report)
                    and 'source_root' in report and report['source_root'] != str(source))):
            raise ValueError('recovery source export differs from its pinned original')
        lineage.append(root)
        if 'original_build' not in report:
            inventory = execution.safe_path(root / 'source-inputs.json')
            if (source != root / 'source' or not inventory.is_file()
                    or execution.digest(inventory) != exported['source_manifest_sha256']):
                raise ValueError('recovery lineage does not end at its exact frozen source inventory')
            return lineage
        original = execution.safe_path(report['original_build'])
        partial = execution.safe_path(original / 'partial.json')
        if not partial.is_file() or execution.digest(partial) != report.get('original_partial_sha256'):
            raise ValueError('recovery original partial receipt changed')
        report = execution.bounded_json(partial, 128 * 1024 * 1024)
        root = original


def build_source_owner(root, report):
    """Resolve a successful or interrupted recovered report without relabelling source."""
    return build_source_lineage(root, report)[-1]


def read_build(path):
    root = execution.safe_path(path)
    report = execution.bounded_json(root / "results.json", 128 * 1024 * 1024)
    if (report.get("action") != "build" or report.get('passed') is not True
            or not report.get("source_root")):
        raise ValueError("actual runtime-stage build receipt required")
    source = execution.safe_path(report["source_root"])
    if execution.verify_source_export(build_source_owner(root, report), report['source_export']) != source:
        raise ValueError('selected source does not match the completed build export')
    product_list = execution.safe_path(report["product_list"])
    products = execution.load_products(product_list)
    return root, source, product_list, products, report


def read_action_build(options):
    """Bind a reporting-only process bridge without relabelling compiled inputs."""
    original, source, products_path, products, build = read_build(options.build)
    selected_path = getattr(options, 'execution_products', None)
    if selected_path is None:
        return original, source, products_path, products, build
    selected_path = execution.safe_path(selected_path)
    if not selected_path.is_relative_to(source/'build'):
        raise ValueError('execution selection must be an owned source build input')
    selected = execution.load_products(selected_path)
    if (any('_harness' in product for product in products)
            or [{key:value for key,value in product.items() if key != '_harness'}
                for product in selected] != products):
        raise ValueError('execution selection changed compiled products or their ordering')
    harnesses = {product.get('_harness') for product in selected}
    if len(harnesses) != 1 or not all(isinstance(path, str) for path in harnesses):
        raise ValueError('one explicit common execution bridge is required')
    harness = execution.safe_path(next(iter(harnesses)))
    if harness != selected_path.parent/'runtime-process-runner.el' or not harness.is_file():
        raise ValueError('execution bridge must be adjacent ordinary input')
    binding_path = execution.safe_path(selected_path.parent/'execution-inputs.json')
    if not binding_path.is_file():
        raise ValueError('execution binding must be an ordinary file')
    binding_digest = execution.digest(binding_path)
    binding = execution.bounded_json(binding_path)
    if (binding.get('schema') != 'femacs-runtime-execution-products-v1'
            or binding.get('build_result_sha256') != execution.digest(original/'results.json')
            or binding.get('original_products_sha256') != execution.digest(products_path)
            or binding.get('selected_products_sha256') != execution.digest(selected_path)
            or binding.get('original_harness_sha256') != execution.digest(source/'tools/fixtures/runtime-process-runner.el')
            or binding.get('harness_sha256') != execution.digest(harness)
            or binding.get('scope') != 'bounded-diagnostic-reporting-only'):
        raise ValueError('execution bridge provenance changed')
    identities = {str(path): execution.file_identity(path)
                  for path in (selected_path, binding_path, harness)}
    if (identities[str(binding_path)]['sha256'] != binding_digest
            or identities[str(selected_path)]['sha256'] != binding['selected_products_sha256']
            or identities[str(harness)]['sha256'] != binding['harness_sha256']):
        raise ValueError('execution inputs changed during initial validation')
    previous = getattr(options, '_execution_input_identities', None)
    if previous is not None and previous != identities:
        raise ValueError('execution inputs changed since initial validation')
    options._execution_input_identities = identities
    return original, source, selected_path, selected, build


def prepare_matrix(options, out):
    """Build functional native fixtures without running protective unit cases."""
    original, source, products_path, products, build = read_action_build(options)
    if build.get('passed') is not True:
        raise ValueError('functional fixture preparation requires a completed product matrix')
    negative_root = source/'build/acceptance/runtime-negative-units'
    if negative_root.exists():
        raise ValueError('native fixture output already exists; inspect its original receipt, do not rebuild blindly')
    env = clean_environment()
    command = [sys.executable, source/'tools/runtime-acceptance.py', '--action', 'prepare-negative',
               '--products', products_path, '--out', negative_root, '--timeout', str(options.timeout)]
    result = execution.run_process(command, out/'negative-unit-build', env, None, cwd=source)
    path = negative_root/'results.json'
    receipt = execution.bounded_json(path, 64*1024*1024) if path.is_file() else {}
    inputs_path = negative_root/'inputs.json'
    inputs = execution.bounded_json(inputs_path) if inputs_path.is_file() else {}
    products_sha256 = execution.digest(products_path)
    expected = {p['id'] for p in products if p['native'] and p['kind'] != 'reference'}
    rows = receipt.get('products', [])
    passed = (bool(expected) and result['exit_code'] == 0 and result['complete']
              and not result['remaining_group_terminated'] and receipt.get('passed') is True
              and receipt.get('schema') == 'femacs-runtime-execution-v1'
              and receipt.get('action') == 'prepare-negative'
              and receipt.get('full_product_inputs_unchanged') is True
              and inputs.get('product_list_sha256') == products_sha256
              and len(rows) == len(expected) and {r.get('profile') for r in rows} == expected
              and all(r.get('passed') is True for r in rows))
    return dict(passed=passed, process=result, preparation_receipt=str(path),
                preparation_receipt_sha256=execution.digest(path) if path.is_file() else None,
                product_list_sha256=products_sha256,
                negative_root=str(negative_root), native_profiles=sorted(expected),
                source_root=str(source), build_receipt=str(original), build_complete=True,
                protective_component_tests_executed=False, semantic_tests_executed=False)


def component_matrix(options, out):
    original, source, products_path, products, build = read_action_build(options)
    env = clean_environment()
    env["CARGO_TARGET_DIR"] = str(out / "cargo")
    rows = []
    commands = [
        ("build-owner-accounting", [sys.executable, source / "tools/test-runtime-stage.py"]),
        ("stable-accounting", [sys.executable, source / "tools/test-stable-suite.py"]),
        ("runtime-accounting", [sys.executable, source / "tools/test-runtime-acceptance.py"]),
        ("actual-source-build", [sys.executable, source / "tools/runtime-components.py", "--out", source / "build/acceptance/runtime-components"]),
        ("actual-source-contracts", [sys.executable, source / "tools/test-runtime-contracts.py", "--receipt",
                                     source / "build/acceptance/runtime-components/receipt.json", "--out",
                                     source / "build/acceptance/runtime-component-results"]),
    ]
    for name, argv in commands:
        case_env = dict(env, FEMACS_RUNTIME_TEST_ROOT=str(out / name / "evidence"))
        result = execution.run_process(argv, out / name, case_env, None, cwd=source)
        rows.append(dict(id=name, process=result, passed=result["exit_code"] == 0 and result["complete"]
                         and not result["remaining_group_terminated"]))
        execution.write_json(out / "partial.json", dict(cases=rows, final=False))
    # Compile all current xtask component cases once, then execute each exact
    # ID separately to retain per-case wall time rather than a cargo total.
    compiled = execution.run_process(["cargo", "test", "--manifest-path", source / "Cargo.toml", "-p", "xtask",
                                      "--no-run", "--message-format=json", "-j", "2"],
                                     out / "xtask-test-build", env, None, cwd=source)
    cases = []
    artifacts = []
    if compiled["exit_code"] == 0:
        with (out / "xtask-test-build/stdout.txt").open(encoding="utf-8") as stream:
            for line in stream:
                try:
                    item = json.loads(line)
                except ValueError:
                    continue
                if item.get("reason") == "compiler-artifact" and item.get("profile", {}).get("test") is True and item.get("executable"):
                    artifacts.append(execution.safe_path(item["executable"]))
    for index, binary in enumerate(artifacts):
        listed = execution.run_process([binary, "--list", "--format", "terse"], out / ("xtask-list-%d" % index), env, None, cwd=source)
        names = [line[:-6] for line in (out / ("xtask-list-%d" % index) / "stdout.txt").read_text().splitlines()
                 if line.endswith(": test")]
        for number, name in enumerate(names):
            case = execution.run_process([binary, "--exact", name, "--nocapture"],
                                         out / ("xtask-%d-%05d" % (index, number)), env, None, cwd=source)
            cases.append(dict(id=name, binary_sha256=execution.digest(binary), process=case,
                              passed=case["exit_code"] == 0 and case["complete"] and not case["remaining_group_terminated"]))
    return dict(cases=rows, xtask_compile=compiled, xtask_cases=cases,
                passed=bool(cases) and all(row["passed"] for row in cases + rows),
                negative_root=str(source / "build/acceptance/runtime-negative-units"),
                source_root=str(source), build_receipt=str(original), build_complete=build.get("passed") is True)


def _execute_profiles(options, out, source, products_path, products, base, env,
                      *, harness=None, original_rows=None):
    harness = harness or source
    actions, pairs, _ = execution_plan(getattr(options, 'execution_phase', 'all'), products,
                                     getattr(options, 'stable_profile', ()))
    rows = []
    for action in actions:
        command = [sys.executable, harness / "tools/runtime-acceptance.py", "--action", action,
                   "--repository", REPO, "--products", products_path,
                   "--out", base / action, "--timeout", str(options.timeout)]
        if action == 'ert' and original_rows is not None:
            command.extend(['--groups', 'collector'])
        if action in ("ert", "negative"):
            command.extend(["--negative-root", source / "build/acceptance/runtime-negative-units"])
        result = execution.run_process(command, out / action, env, None, cwd=source)
        receipt_path = base / action / "results.json"
        row = dict(id=action, process=result, receipt=str(receipt_path), passed=False)
        if receipt_path.is_file():
            try:
                receipt = execution.read_completion(base / action, action, products_path)
                row['execution_complete'] = receipt['execution_complete']
                row['counts'] = receipt['counts']
                row['completion_receipt'] = str(base / action / 'completion.json')
                row['completion_sha256'] = execution.digest(base / action / 'completion.json')
                row["passed"] = (result["exit_code"] == 0 and result['complete']
                    and not result['remaining_group_terminated'] and receipt['passed'])
                row["receipt_sha256"] = receipt['detailed']['sha256']
                if action == 'ert' and original_rows is not None:
                    checkpoint = execution.load_tool('runtime-checkpoint.py')
                    merged = checkpoint.merge(execution, original_rows, base/action, receipt, out)
                    row['affected_merge'] = merged
                    row['passed'] = row['passed'] and merged['passed']
            except (OSError, ValueError, KeyError, TypeError) as error:
                row['receipt_validation_error'] = f'{type(error).__name__}: {error}'
        rows.append(row)
        execution.write_json(out / "partial.json", dict(cases=rows, final=False))
    for reference, candidate in pairs:
        name = "stable-" + candidate["id"]
        receipt_root = base / name
        command = [sys.executable, harness / "tools/stable-suite.py", "--repo", source,
                   "--manifest", source / "compatibility/stable-31.1/sources.json", "--out", receipt_root,
                   "--reference", reference["program"], "--candidate", candidate["program"],
                   "--reference-dump-args", json.dumps(["--dump-file=" + reference["dump"]]),
                   "--candidate-dump-args", json.dumps(["--dump-file=" + candidate["dump"]]),
                   "--reference-env", json.dumps({"FEMACS_GC_OBSERVATION":"off"}),
                   "--candidate-env", json.dumps({"FEMACS_GC_OBSERVATION":"off",
                                                   "FEMACS_GC_COLLECTOR":"major" if candidate["collector"] == "off" else candidate["collector"]}),
                   "--timeout", str(options.timeout),
                   '--runner', harness/'tools/fixtures/stable-suite-runner.el']
        case_env = dict(env)
        result = execution.run_process(command, out / name, case_env, None, cwd=source)
        receipt = receipt_root / "stable-suite-run.json"
        row = dict(id=name, process=result, receipt=str(receipt),
                         receipt_sha256=execution.digest(receipt) if receipt.is_file() else None,
                         passed=False)
        try:
            suite = execution.load_tool('stable-suite.py')
            summary = suite.read_completion(receipt_root,
                source/'compatibility/stable-31.1/sources.json', reference['program'], candidate['program'],
                ['--dump-file='+reference['dump']], ['--dump-file='+candidate['dump']],
                {'FEMACS_GC_OBSERVATION': 'off'}, {'FEMACS_GC_OBSERVATION': 'off',
                    'FEMACS_GC_COLLECTOR': 'major' if candidate['collector'] == 'off' else candidate['collector']})
            row['completion_sha256'] = execution.digest(receipt_root/'stable-suite-completion.json')
            row['execution_complete'] = summary['execution_complete']
            row['case_count'] = summary['case_count']
            row['failure_count'] = summary['failure_count']
            row['passed'] = (result['exit_code'] == 0 and result['complete']
                and not result['remaining_group_terminated'] and summary['corpus_complete'])
        except Exception as error:
            row['receipt_validation_error'] = f'{type(error).__name__}: {error}'
        rows.append(row)
        execution.write_json(out / "partial.json", dict(cases=rows, final=False))
    return rows


def snapshot_harness(out):
    root = out/'harness'
    root.mkdir()
    files = [*sorted((REPO/'tools').glob('*.py')),
             *sorted(path for path in (REPO/'tools/fixtures').rglob('*') if path.is_file())]
    identities = {}
    for path in files:
        path = execution.safe_path(path)
        relative = path.relative_to(REPO)
        destination = root/relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(path, destination)
        shutil.copymode(path, destination)
        if execution.digest(path) != execution.digest(destination):
            raise ValueError('harness source changed during snapshot')
        identities[str(destination)] = execution.file_identity(destination)
    execution.write_json(out/'harness-inputs.json', identities)
    return root, identities


def execute_matrix(options, out, artifacts=None):
    original, source, products_path, products, build = read_action_build(options)
    phase = getattr(options, 'execution_phase', 'all')
    actions, pairs, expected_ids = execution_plan(phase, products,
                                                getattr(options, 'stable_profile', ()))
    available_pairs = execution_plan('stable', products)[1] if phase in ('all', 'stable') else []
    requested_profiles = [candidate['id'] for _, candidate in pairs]
    remaining_profiles = [candidate['id'] for _, candidate in available_pairs
                          if candidate['id'] not in requested_profiles]
    # Include the complete stable-corpus loop in the same immutable-product boundary.
    before = {product["id"]: execution.product_identity(product, resources=True) for product in products}
    identity_path = out / "product-identities.json"
    execution.write_json(identity_path, dict(before=before, after=None, unchanged=False))
    base = source / "build/acceptance" / out.name
    base.mkdir(parents=True)
    env = clean_environment()
    harness, original_rows = source, None
    if options.action == 'followup':
        harness, identities = snapshot_harness(out)
        if artifacts is not None:
            owner.capture_artifacts(artifacts.journal, identities)
        # Old valid VM results can only be reused with the original source and
        # same bytecode bridge. A fixture change may replace its entire group.
        if execution.digest(harness/'tools/fixtures/runtime-process-runner.el') != execution.digest(source/'tools/fixtures/runtime-process-runner.el'):
            raise ValueError('changed process bridge requires a wider replay')
        active_fixtures = {fixture for product in products
            for mode in (('off','phases','survival','all') if product['observer'] else ('off',))
            for _,fixtures,_ in execution.fixture_groups(product,mode) for fixture in fixtures}
        changed = []
        for name in sorted(active_fixtures):
            path = source/'tools/fixtures'/name
            if execution.digest(path) != execution.digest(harness/'tools/fixtures'/path.name):
                changed.append(path.name)
        if changed != ['runtime-gc.el']:
            raise ValueError('affected replay fixture scope changed: '+repr(changed))
        checkpoint = execution.load_tool('runtime-checkpoint.py')
        original_rows, prior = checkpoint.verify(execution, source, products_path, products,
            options.original_ert, options.original_index, options.original_audit, out)
        execution.write_json(out/'progress.json', dict(stage='original-ert-revalidated',
            counts=prior['counts'], requested_next='full affected collector group'))
    try:
        rows = _execute_profiles(options, out, source, products_path, products, base, env,
                                 harness=harness, original_rows=original_rows)
    finally:
        after, errors = {}, {}
        for product in products:
            try:
                after[product["id"]] = execution.product_identity(product, resources=True)
            except (OSError, ValueError) as error:
                errors[product["id"]] = str(error)
        complete_receipts_valid = False
        try:
            # Includes generated config and Cocoa outer-bundle metadata, which
            # are wider than the execution driver's resource_roots alone.
            execution.load_products(products_path)
            complete_receipts_valid = True
        except Exception as error:
            errors['complete-product-receipts'] = f'{type(error).__name__}: {error}'
        unchanged = not errors and before == after
        execution.write_json(identity_path, dict(before=before, after=after,
                                                identity_errors=errors, unchanged=unchanged,
                                                complete_receipts_valid=complete_receipts_valid))
    scope_complete = ([row['id'] for row in rows] == expected_ids
                      and all(row.get('execution_complete') is True for row in rows)
                      and unchanged and complete_receipts_valid)
    return dict(cases=rows, build_receipt=str(original), build_complete=build.get("passed") is True,
                harness_root=str(harness), original_ert_reused=original_rows is not None,
                execution_phase=phase, requested_actions=list(actions), requested_stable_pairs=len(pairs),
                requested_stable_profiles=requested_profiles,
                available_stable_pairs=len(available_pairs), remaining_stable_profiles=remaining_profiles,
                stable_phase_complete=phase in ('all', 'stable') and scope_complete and not remaining_profiles,
                execution_complete=scope_complete,
                full_matrix_execution_complete=phase == 'all' and scope_complete,
                remaining_execution_phases=([name for name in ('semantics', 'stable', 'performance')
                                             if name != phase or (name == 'stable' and remaining_profiles)]
                                            if phase != 'all' else []),
                passed=scope_complete and all(row["passed"] for row in rows),
                product_identity_unchanged=unchanged, complete_receipts_valid=complete_receipts_valid,
                identity_receipt=str(identity_path),
                goal_complete=False, compatibility_accepted=False, performance_accepted=False,
                remaining=["actual source/product review", "native Finder/multiframe IME/TTY workflows",
                           "required scope and performance judgment", "signed delivery and cleanup evidence"])


def select_execution_inputs(artifacts, original, source, products_path, products, build, options):
    """Preserve finished products, source cohorts and top-level build metadata for execution."""
    cohorts = list(build.get('source_cohorts') or [build['source_export']])
    if build['source_export'] not in cohorts:
        cohorts.append(build['source_export'])
    source_roots = []
    seen_sources = set()
    for cohort in cohorts:
        cohort_source = execution.safe_path(cohort['source'])
        if str(cohort_source) in seen_sources:
            continue
        seen_sources.add(str(cohort_source))
        cohort_owner = (build_source_owner(original, build)
                        if cohort == build['source_export']
                        else execution.safe_path(cohort_source.parent))
        artifacts.select_source(cohort_owner, cohort)
        source_roots.append(cohort_source)
    top_metadata = [execution.safe_path(original / 'results.json'),
                    execution.safe_path(build['product_list'])]
    value = owner.artifact_read(artifacts.journal)
    for path in top_metadata:
        if str(path) not in value['allowed_roots']:
            value['allowed_roots'].append(str(path))
    owner.artifact_write(artifacts.journal, value)
    owner.capture_artifacts(artifacts.journal, {
        str(path): execution.file_identity(path) for path in top_metadata})
    if getattr(options, 'execution_products', None) is not None:
        # Preserve the initially verified content, not fresh hashes
        # that could silently adopt a pre-capture replacement.
        owner.capture_artifacts(artifacts.journal, options._execution_input_identities)
        for path, identity in options._execution_input_identities.items():
            if execution.file_identity(execution.safe_path(path)) != identity:
                raise ValueError('execution inputs changed before preservation completed')
    seen = set()
    for product in products:
        if product['kind'] != 'reference' and product['product_receipt'] not in seen:
            receipt_path = execution.safe_path(product['product_receipt'])
            product_source = next(
                (root for root in source_roots if receipt_path.is_relative_to(root / 'build/runtime')),
                source)
            owner.capture_product_files(artifacts.journal, receipt_path, product_source)
            seen.add(product['product_receipt'])


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--action", choices=("build", "prepare", "execute", "followup", "components"), required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument('--retention-mode', choices=('archive', 'files'), default='archive',
                        help='archive selected content or verify and retain its physical original files')
    parser.add_argument("--build", type=Path)
    parser.add_argument('--execution-products', type=Path,
                        help='explicit reporting-only process bridge selection for fixed products')
    parser.add_argument("--backend-prefix", type=Path)
    parser.add_argument("--reference", type=Path)
    parser.add_argument("--reference-dump", type=Path)
    parser.add_argument("--reference-lisp", type=Path)
    parser.add_argument("--reference-data", type=Path)
    parser.add_argument("--reference-native", action="store_true")
    parser.add_argument("--carry-terminal-from", type=Path)
    parser.add_argument("--carry-controller-result", type=Path)
    parser.add_argument("--terminal-retention-proof", type=Path)
    parser.add_argument("--carry-native-cocoa-from", type=Path)
    parser.add_argument("--carry-native-cocoa-controller-result", type=Path)
    parser.add_argument("--timeout", type=float, default=300)
    parser.add_argument('--execution-phase', choices=tuple(EXECUTION_PHASES), default='all',
                        help='execute one complete phase without claiming the complete matrix')
    parser.add_argument('--stable-profile', action='append', default=[],
                        help='execute this complete stable pair; repeat for an explicit profile scope')
    parser.add_argument('--original-ert', type=Path)
    parser.add_argument('--original-index', type=Path)
    parser.add_argument('--original-audit', type=Path)
    options = parser.parse_args(argv)
    if options.action != 'execute' and options.execution_phase != 'all':
        parser.error('execution-phase applies only to execute')
    if options.stable_profile and (options.action != 'execute' or options.execution_phase != 'stable'):
        parser.error('stable-profile applies only to execute --execution-phase stable')
    if len(options.stable_profile) != len(set(options.stable_profile)):
        parser.error('duplicate stable profile selection')
    if options.execution_products is not None and options.action not in ('prepare', 'execute', 'components'):
        parser.error('execution-products applies only to prepare/execute/components')
    if not math.isfinite(options.timeout) or options.timeout <= 0:
        parser.error("positive finite per-case deadline required")
    if options.action == 'followup' and any(getattr(options,key) is None
            for key in ('original_ert','original_index','original_audit')):
        parser.error('followup requires original ERT root, exact per-case index and independent audit')
    if options.action != 'followup' and any(getattr(options,key) is not None
            for key in ('original_ert','original_index','original_audit')):
        parser.error('original execution reuse applies only to explicit followup')
    if options.action == "build":
        for field in ("backend_prefix", "reference", "reference_dump", "reference_lisp", "reference_data"):
            if getattr(options, field) is None:
                parser.error("build requires --" + field.replace("_", "-"))
        if bool(options.carry_terminal_from) != bool(options.carry_controller_result):
            parser.error("terminal reuse requires both prior build and its stopped controller receipt")
        if options.terminal_retention_proof is not None and options.carry_terminal_from is None:
            parser.error("historical terminal retention proof requires explicit prior terminal reuse")
        if bool(options.carry_native_cocoa_from) != bool(options.carry_native_cocoa_controller_result):
            parser.error("native Cocoa reuse requires both prior build and its stopped controller receipt")
    elif options.build is None:
        parser.error("prepare/execute/components require --build receipt directory")
    elif (options.carry_terminal_from is not None or options.carry_controller_result is not None
          or options.carry_native_cocoa_from is not None or options.carry_native_cocoa_controller_result is not None
          or options.terminal_retention_proof is not None):
        parser.error("terminal reuse options apply only to build")
    out = execution.new_output(options.out)
    try:
        with owner.ArtifactRun(REPO, out) as artifacts:
            if options.action == "build":
                result = build_matrix(options, out, artifacts)
            else:
                original, source, products_path, products, build = read_action_build(options)
                select_execution_inputs(artifacts, original, source, products_path, products, build, options)
                if options.action == 'followup':
                    prepared = source/'build/acceptance/runtime-negative-units'
                    owner.capture_artifacts(artifacts.journal, {
                        str(prepared/name): identity
                        for name,identity in execution.tree_identity(prepared).items()})
                try:
                    if options.action == "prepare":
                        result = prepare_matrix(options, out)
                        if result['passed']:
                            root = Path(result['negative_root'])
                            identities = {str(root / name): identity
                                          for name, identity in execution.tree_identity(root).items()}
                            owner.capture_artifacts(artifacts.journal, identities)
                    elif options.action == "components":
                        result = component_matrix(options, out)
                    else:
                        result = execute_matrix(options, out, artifacts)
                finally:
                    if execution.verify_source_export(build_source_owner(original, build), build['source_export']) != source:
                        raise ValueError('frozen action source changed')
            if options.execution_products is not None:
                result['execution_selection'] = dict(path=str(products_path),
                    sha256=execution.digest(products_path),
                    binding_sha256=execution.digest(products_path.parent/'execution-inputs.json'),
                    harness_sha256=execution.digest(products[0]['_harness']),
                    selection_preserves_compiled_product_fields=True)
            # Deduplicated exact content is retained before releasing flags.
            # It is selected source/product/link evidence, not a compiler cache.
            if options.retention_mode == 'files':
                result['artifact_retention'] = artifacts.retain_current_files()
            elif options.action != 'followup':
                artifacts.archive()
            if options.action == 'followup':
                # Keep the selected originals, new harness and exact ledgers;
                # their verified retention mode is explicit in this run.
                result['recovery'] = dict(new_harness_retained=str(out/'harness'),
                    original_source_and_products_retained=True,
                    new_duplicate_content_archive_created=False)
    except BaseException as error:
        execution.write_json(out / "fatal.json", dict(passed=False, complete=False, error=f"{type(error).__name__}: {error}"))
        raise
    result.update(schema="femacs-runtime-stage-v1", action=options.action, goal_complete=False,
                  coordinator_sha256=COORDINATOR_SHA256)
    execution.write_json(out / "results.json", result)
    print(json.dumps(dict(action=options.action, passed=result["passed"], results=str(out / "results.json"))))
    return 0 if result["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
