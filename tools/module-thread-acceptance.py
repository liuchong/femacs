#!/usr/bin/env python3
"""Owned stable-module and thread acceptance evidence, without a second runner."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import sys
import time

REPO = Path(__file__).resolve().parent.parent
FIXTURES = REPO / "tools/fixtures"
RUNNER = FIXTURES / "module-thread-runner.el"
CONFIG_FIXTURE = FIXTURES / "module-thread-config.h"
STABLE_MANIFEST = REPO / "compatibility/stable-31.1/sources.json"
STABLE_HEADER_RE = re.compile(r"^[ \t]*#[ \t]*define[ \t]+EMACS_MAJOR_VERSION[ \t]+([0-9]+)[ \t]*$", re.MULTILINE)
TEST_FILES = (
    ("test/lisp/thread-tests.el", "thread-lisp"),
    ("test/src/thread-tests.el", "thread-src"),
    ("test/src/emacs-module-tests.el", "module-src"),
)
REQUIRED_SOURCES = (
    "test/lisp/thread-tests.el",
    "test/src/thread-tests.el",
    "test/src/emacs-module-tests.el",
    "test/src/emacs-module-resources/mod-test.c",
)


def load_execution():
    path = REPO / "tools/runtime-execution.py"
    spec = importlib.util.spec_from_file_location("femacs_module_thread_execution", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


execution = load_execution()


class AcceptanceError(Exception):
    pass


def file_record(path):
    path = execution.safe_path(path)
    value = execution.file_identity(path)
    return dict(value, path=str(path))


def sha(path):
    return execution.digest(path)


def read_json(path, limit=16 * 1024 * 1024):
    return execution.bounded_json(path, limit)


def read_header_major(path):
    text = path.read_text(encoding="utf-8")
    match = STABLE_HEADER_RE.search(text)
    if match is None:
        raise AcceptanceError(f"module header has no EMACS_MAJOR_VERSION: {path}")
    return int(match.group(1))


def copy_checked(source, destination, expected=None):
    source = execution.safe_path(source)
    before = execution.file_identity(source)
    if expected is not None:
        if before.get("kind") != "file":
            raise AcceptanceError(f"expected regular source file: {source}")
        if before.get("bytes") != expected["size"]:
            raise AcceptanceError(f"source size changed: {source}")
        if before.get("sha256") != expected["sha256"]:
            raise AcceptanceError(f"source hash changed: {source}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    execution.copy_owned_data(source, destination)
    after = execution.file_identity(destination)
    if before.get("sha256") != after.get("sha256") or before.get("mode") != after.get("mode"):
        raise AcceptanceError(f"copied source identity changed: {source}")
    return dict(path=str(destination), destination=str(destination), source=str(source), **after)


def stable_manifest_entries(path):
    document = read_json(path, 64 * 1024 * 1024)
    if document.get("schema") != 1:
        raise AcceptanceError("stable source manifest schema is not 1")
    entries = {entry.get("path"): entry for entry in document.get("entries", [])}
    selected = {}
    for relative in REQUIRED_SOURCES:
        entry = entries.get(relative)
        if entry is None:
            raise AcceptanceError(f"stable source manifest is missing {relative}")
        source = (REPO / entry["source_path"]).resolve()
        if not source.is_relative_to(REPO):
            raise AcceptanceError(f"stable source escapes repository: {source}")
        selected[relative] = dict(entry, source_path=str(source))
    return selected


def copy_stable_inputs(selected, root):
    records = {}
    source_root = root / "stable-source"
    for relative, entry in selected.items():
        if relative.startswith("test/"):
            destination = source_root / relative
        else:
            destination = source_root / relative
        records[relative] = copy_checked(Path(entry["source_path"]), destination, entry)
    return records


def copy_header(path, root, expected_major, label):
    source = execution.safe_path(path)
    if source.is_symlink() or not source.is_file():
        raise AcceptanceError(f"{label} must be a regular header file")
    major = read_header_major(source)
    if major != expected_major:
        raise AcceptanceError(f"{label} major is {major}, expected {expected_major}")
    destination = root / "stable" / "emacs-module.h" if expected_major == 31 else root / "candidate" / "emacs-module.h"
    record = copy_checked(source, destination)
    record["major"] = major
    return record


def copy_config_header(path, root):
    source = execution.safe_path(path)
    if source.is_symlink() or not source.is_file():
        raise AcceptanceError("stable config header must be a regular file")
    destination = root / "config" / "config.h"
    return copy_checked(source, destination)


def make_build_environment(root):
    env = {key: value for key, value in os.environ.items()
           if key not in execution.FORBIDDEN_ENV}
    for key, name in {
        "HOME": "home",
        "XDG_CACHE_HOME": "cache",
        "XDG_CONFIG_HOME": "config",
        "XDG_STATE_HOME": "state",
        "TMPDIR": "tmp",
        "FEMACS_RUNTIME_NATIVE_CACHE": "eln",
        "EMACSNATIVELOADPATH": "eln",
    }.items():
        path = root / name
        path.mkdir(parents=True, exist_ok=True)
        env[key] = str(path)
    env.update(LC_ALL="C", LANG="C", TERM="dumb")
    return env


def compile_module(source, header_dir, config_dir, root, cc, gmp_cflags, gmp_ldflags, timeout, sdk_root):
    build_root = root / "module-build"
    build_root.mkdir(parents=True, exist_ok=True)
    module_directory = root / "compiled-module"
    module_directory.mkdir()
    output = module_directory / "mod-test.dylib"
    argv = list(cc) + [
        "-std=gnu2x",
        "-shared",
        "-fPIC",
        "-I", str(header_dir),
        "-I", str(config_dir),
    ] + list(gmp_cflags) + [
        str(source),
        "-o", str(output),
    ] + list(gmp_ldflags)
    if sys.platform == "darwin":
        if sdk_root is None or not sdk_root.is_dir():
            raise AcceptanceError("an explicit existing macOS SDK root is required")
        argv.extend(["-isysroot", str(sdk_root)])
        argv.extend(["-Wl,-undefined,dynamic_lookup"])
    process = execution.run_process(
        argv,
        build_root / "compile",
        make_build_environment(build_root),
        timeout,
        cwd=build_root,
    )
    if not process["complete"] or process["exit_code"] != 0 or process["remaining_group_terminated"]:
        raise AcceptanceError("stable module compilation failed; inspect compile evidence")
    if not output.is_file():
        raise AcceptanceError("stable module compiler produced no output")
    return dict(path=str(output), argv=[str(part) for part in argv], process=process,
                identity=file_record(output))


def wrapper_script(program, dump, invocation_directory):
    eval_value = json.dumps(str(invocation_directory) + os.sep)
    argv = [str(program), "--dump-file=" + str(dump), "--eval",
            f"(setq invocation-directory {eval_value})"]
    return ("#!" + sys.executable + "\nimport os, sys\n"
            + "os.execv(" + repr(str(program)) + ", " + repr(argv) + " + sys.argv[1:])\n")


def create_harness(product, root, stable_records, module_record, timeout):
    product_id = product["id"]
    harness = root / "harness" / product_id
    binary = harness / "bin"
    tests = harness / "test"
    binary.mkdir(parents=True, exist_ok=True)
    tests.mkdir(parents=True, exist_ok=True)
    copied = {}
    for relative, record in stable_records.items():
        if not relative.startswith("test/"):
            continue
        destination = tests / relative[len("test/"):]
        copied[relative] = copy_checked(Path(record["path"]), destination)
    module_target = tests / "src/emacs-module-resources/mod-test.dylib"
    copied["test/src/emacs-module-resources/mod-test.dylib"] = copy_checked(
        Path(module_record["path"]), module_target)
    invocation_name = Path(product["program"]).name
    wrapper = binary / invocation_name
    wrapper.write_text(
        wrapper_script(product["program"], product["dump"], binary),
        encoding="utf-8",
    )
    wrapper.chmod(0o755)
    return dict(
        path=str(harness),
        binary=str(binary) + os.sep,
        tests=str(tests),
        invocation_name=invocation_name,
        wrapper=str(wrapper),
        wrapper_sha256=sha(wrapper),
        copied=copied,
        module_suffix=".dylib",
    )


def safe_component(value):
    value = re.sub(r"[^A-Za-z0-9._-]+", "-", value).strip(".-") or "case"
    return value + "-" + hashlib.sha256(value.encode("utf-8")).hexdigest()[:12]


def write_case_result(root, row):
    execution.write_json(root / "result.json", row)


def output_identity(path):
    if path is None:
        return None
    path = Path(path)
    if not path.is_file():
        return None
    return dict(bytes=path.stat().st_size, sha256=sha(path))


def run_case(product, harness, test_file, operation, test_id, root, timeout,
             module_record, stable_header, candidate_header, config_header):
    label = operation if test_id is None else f"{operation}-{test_id}"
    case_root = root / "cases" / product["id"] / safe_component(str(test_file)) / safe_component(label)
    case_root.mkdir(parents=True, exist_ok=False)
    relative_test = Path(test_file).relative_to(harness['tests'])
    original_records = {name:record for name,record in harness['copied'].items() if name in REQUIRED_SOURCES}
    harness = create_harness(product, case_root, original_records, module_record, timeout)
    test_file = Path(harness['tests']) / relative_test
    request = dict(
        schema=1,
        operation=operation,
        file=str(test_file),
        invocation_directory=harness["binary"],
        invocation_name=harness["invocation_name"],
    )
    if test_id is not None:
        request["id"] = test_id
    execution.write_json(case_root / "request.json", request)
    env = execution.owned_environment(case_root, product, "off")
    env["FEMACS_MODULE_THREAD_REQUEST"] = str(case_root / "request.json")
    env["FEMACS_MODULE_THREAD_RESULT"] = str(case_root / "response.json")
    before = execution.product_identity(product)
    module_path = Path(harness["tests"]) / "src/emacs-module-resources/mod-test.dylib"
    fixture_before = dict(test=file_record(test_file), module=file_record(module_path),
                          runner=file_record(RUNNER), wrapper=sha(harness["wrapper"]))
    argv = [
        harness["wrapper"],
        "-Q",
        "--batch",
        "--load",
        str(RUNNER),
    ]
    process = execution.run_process(
        argv,
        case_root / "process",
        env,
        timeout,
        metrics=True,
        cwd=harness["binary"],
    )
    after = execution.product_identity(product)
    response = None
    response_error = None
    response_path = case_root / "response.json"
    if response_path.is_file():
        try:
            response = read_json(response_path, 16 * 1024 * 1024)
        except (OSError, ValueError) as error:
            response_error = f"{type(error).__name__}: {error}"
    else:
        response_error = "no module/thread response"
    test_sha = sha(test_file)
    module_sha = sha(module_path)
    fixture_unchanged = fixture_before == dict(test=file_record(test_file), module=file_record(module_path),
        runner=file_record(RUNNER), wrapper=sha(harness["wrapper"]))
    passed = (
        process["complete"] is True
        and process["exit_code"] == 0
        and process.get("remaining_group_terminated") is False
        and isinstance(response, dict)
        and response.get("schema") == "femacs-module-thread-process-v1"
        and response.get("complete") is True
        and response.get("status") == "passed"
        and before == after
        and fixture_unchanged
        and test_sha == file_record(test_file)["sha256"]
        and module_sha == module_record["identity"]["sha256"]
        and harness["wrapper_sha256"] == sha(harness["wrapper"])
    )
    status = None
    if isinstance(response, dict):
        status = response.get("status")
    if status is None:
        status = process.get("status")
    if operation == "discover":
        ids_value = response.get("ids") if isinstance(response, dict) else None
        passed = (process["complete"] is True
                  and process["exit_code"] == 0
                  and process.get("remaining_group_terminated") is False
                  and isinstance(response, dict)
                  and response.get("complete") is True
                  and isinstance(ids_value, list)
                  and bool(ids_value)
                  and before == after and fixture_unchanged)
    row = dict(
        schema="femacs-module-thread-case-v1",
        product=product["id"],
        kind=product["kind"],
        operation=operation,
        file=str(test_file),
        id=test_id,
        status=status,
        complete=bool(isinstance(response, dict) and response.get("complete") is True),
        passed=passed,
        skipped=status == "skipped",
        timed_out=process["timed_out"],
        exit_code=process["exit_code"],
        duration=None if not isinstance(response, dict) else response.get("seconds"),
        process=process,
        request=request,
        response=response,
        response_error=response_error,
        source_identity_before=before,
        source_identity_after=after,
        source_identity_unchanged=before == after,
        fixture_inputs_unchanged=fixture_unchanged,
        command=dict(argv=argv, actual_program=product["program"], actual_dump=product["dump"]),
        inputs=dict(
            test_file=file_record(test_file),
            module_path=file_record(module_path),
            module=module_record["identity"],
            wrapper_sha256=harness["wrapper_sha256"],
            runner=file_record(RUNNER),
            stable_header=stable_header,
            candidate_header=candidate_header,
            config_header=config_header,
        ),
        stdout=output_identity(case_root / "process/stdout.txt"),
        stderr=output_identity(case_root / "process/stderr.txt"),
        response_identity=output_identity(response_path),
        harness=harness,
        record_path=None,
    )
    row["record_path"] = str(case_root / "result.json")
    write_case_result(case_root, row)
    if row['passed']:
        row['retirement'] = retire_harness(harness, case_root, 'successful reaped isolated case')
        row['cleanup_complete'] = row['retirement'].get('status') == 'complete'
        row['passed'] = row['passed'] and row['cleanup_complete']
    else:
        row['retirement'] = dict(state='retained-failure', harness=harness['path'])
        row['cleanup_complete'] = False
    write_case_result(case_root, row)
    return row, response


def ids_from_response(response):
    if not isinstance(response, dict) or response.get("complete") is not True:
        return []
    value = response.get("ids")
    if not isinstance(value, list) or any(not isinstance(item, str) or not item for item in value):
        return []
    if len(set(value)) != len(value):
        return []
    return sorted(value)


def missing_case(product, test_file, test_id, reason, before):
    return dict(
        schema="femacs-module-thread-case-v1",
        product=product["id"],
        kind=product["kind"],
        operation="ert",
        file=str(test_file),
        id=test_id,
        status="missing",
        complete=False,
        passed=False,
        skipped=False,
        timed_out=False,
        exit_code=None,
        duration=None,
        process=None,
        request=None,
        response=None,
        response_error=reason,
        source_identity_before=before,
        source_identity_after=before,
        source_identity_unchanged=True,
        command=None,
        inputs=None,
        stdout=None,
        stderr=None,
        response_identity=None,
        record_path=None,
    )


def retire_harness(harness, root, reason):
    base = execution.safe_path(harness["path"])
    evidence = root / "retirement-inputs" / (base.name + ".json")
    evidence.parent.mkdir(parents=True, exist_ok=True)
    execution.write_json(evidence, dict(harness=harness, reason=reason,
        scope="completed owned harness only; raw cases and canonical inputs retained"))
    return execution.load_tool('runtime-retire.py').retire(base.parent, (base.name,), evidence,
        dict(complete=True, exit_code=0, remaining_group_terminated=False))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--products", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--stable-module-header", required=True, type=Path)
    parser.add_argument("--candidate-module-header", type=Path)
    parser.add_argument("--stable-config-header", type=Path)
    parser.add_argument("--stable-manifest", type=Path, default=STABLE_MANIFEST)
    parser.add_argument("--timeout", type=float, default=300.0)
    parser.add_argument("--cc", default=os.environ.get("CC", "cc"))
    parser.add_argument("--sdk-root", type=Path)
    parser.add_argument("--gmp-cflags", default="")
    parser.add_argument("--gmp-ldflags", default="-lgmp")
    options = parser.parse_args(argv)

    out = execution.new_output(options.out)
    try:
        if not 0 < options.timeout < float('inf'):
            raise AcceptanceError("--timeout must be positive")
        products = execution.load_products(options.products)
        if not products:
            raise AcceptanceError("products list is empty")
        references = [product for product in products if product["kind"] == "reference"]
        candidates = [product for product in products if product["kind"] != "reference"]
        if len(references) != 1 or not candidates:
            raise AcceptanceError("exactly one real reference and at least one candidate required")
        reference = references[0]
        for product in candidates:
            if (Path(product["program"]).resolve(), Path(product["dump"]).resolve()) == (
                    Path(reference["program"]).resolve(), Path(reference["dump"]).resolve()):
                raise AcceptanceError("candidate aliases the reference program/dump")
        stable_manifest = file_record(options.stable_manifest)
        selected = stable_manifest_entries(options.stable_manifest)
        stable_copy = out / "inputs"
        stable_records = copy_stable_inputs(selected, stable_copy)
        stable_header = copy_header(options.stable_module_header, stable_copy, 31, "stable module header")
        candidate_header = None
        if options.candidate_module_header is not None:
            candidate_header = copy_header(options.candidate_module_header, stable_copy, 32,
                                           "candidate module header")
        elif (REPO / "build/generation/module-api/emacs-module.h").is_file():
            candidate_header = copy_header(REPO / "build/generation/module-api/emacs-module.h",
                                           stable_copy, 32, "generated candidate module header")
        if candidate_header is None:
            raise AcceptanceError("the actual generated candidate module header is required")
        packaged_headers = {}
        for product in products:
            if product["kind"] == "reference": continue
            receipt = Path(product["product_receipt"])
            relative = ("dist/Femacs.app/Contents/Resources/include/emacs-module.h"
                        if product["kind"] == "cocoa" else "dist/terminal/include/emacs-module.h")
            delivered = file_record(receipt.parent / relative)
            if delivered["sha256"] != candidate_header["sha256"]:
                raise AcceptanceError("packaged module header differs from the selected generated API")
            packaged_headers[product["id"]] = delivered
        provenance = options.stable_module_header.with_name('inputs.json')
        if provenance.is_file():
            source = read_json(provenance)
            original = read_json(options.stable_manifest)["source"]
            if (source.get('api_major') != 31 or source.get('source_ref') != original['commit']
                    or source.get('generated', {}).get('sha256') != stable_header['sha256']):
                raise AcceptanceError("stable module API provenance does not match the original corpus version")
            stable_header['source_provenance'] = file_record(provenance)
        config_source = options.stable_config_header or CONFIG_FIXTURE
        config_header = copy_config_header(config_source, stable_copy)
        module_source = Path(stable_records["test/src/emacs-module-resources/mod-test.c"]["path"])
        header_dir = Path(stable_header["path"]).parent
        config_dir = Path(config_header["path"]).parent
        gmp_cflags = shlex.split(options.gmp_cflags)
        gmp_ldflags = shlex.split(options.gmp_ldflags)
        cc = shlex.split(options.cc)
        compiled = compile_module(module_source, header_dir, config_dir, out, cc,
                                  gmp_cflags, gmp_ldflags, options.timeout, options.sdk_root)
        compile_process = compiled["process"]

        inputs = dict(
            schema="femacs-module-thread-inputs-v1",
            action="module-thread",
            products=dict(path=str(options.products), sha256=sha(options.products)),
            stable_manifest=stable_manifest,
            stable_sources=stable_records,
            stable_module_header=stable_header,
            candidate_module_header=candidate_header,
            packaged_module_headers=packaged_headers,
            stable_config_header=config_header,
            module_source=file_record(module_source),
            compiled_module=compiled["identity"],
            compile=dict(argv=compiled["argv"], process=compile_process),
            runner=file_record(RUNNER),
            config_fixture=file_record(CONFIG_FIXTURE),
            cc=cc,
            sdk_root=None if options.sdk_root is None else str(options.sdk_root),
            gmp_cflags=gmp_cflags,
            gmp_ldflags=gmp_ldflags,
            stable_manifest_path=str(options.stable_manifest),
            timeout=options.timeout,
        )
        execution.write_json(out / "inputs.json", inputs)

        all_cases = []
        all_discoveries = []
        all_unrun = []
        all_failures = []
        all_skipped = []
        product_rows = []
        before_products = {product["id"]: execution.product_identity(product, resources=True)
                           for product in products}
        harnesses = {}
        cleanup = {}

        for product in products:
            harness = create_harness(product, out, stable_records, compiled, options.timeout)
            harnesses[product["id"]] = harness

        reference_cache = {}
        reference_harness = harnesses[reference["id"]]
        for relative, _label in TEST_FILES:
            path = Path(reference_harness["tests"]) / relative[len("test/"):]
            discovery, response = run_case(reference, reference_harness, path, "discover", None,
                out, options.timeout, compiled, stable_header, candidate_header, config_header)
            all_discoveries.append(discovery)
            ids = ids_from_response(response)
            reference_cases = {}
            for test_id in ids:
                reference_cases[test_id], _ = run_case(reference, reference_harness, path,
                    "ert", test_id, out, options.timeout, compiled, stable_header,
                    candidate_header, config_header)
            reference_cache[relative] = dict(discovery=discovery, ids=ids, cases=reference_cases)
        for product in candidates:
            harness = harnesses[product["id"]]
            product_failures = []
            product_unrun = []
            product_skipped = []
            before = execution.product_identity(product)
            product_cases = []
            product_discoveries = []
            for relative, _label in TEST_FILES:
                test_path = Path(harness["tests"]) / relative[len("test/"):]
                reference_data = reference_cache[relative]
                reference_ids = reference_data["ids"]
                candidate_discovery, candidate_response = run_case(
                    product, harness, test_path, "discover", None, out, options.timeout,
                    compiled, stable_header, candidate_header, config_header)
                candidate_ids = ids_from_response(candidate_response)
                product_discoveries.append(candidate_discovery)
                all_discoveries.append(candidate_discovery)
                if not reference_data["discovery"]["passed"] or not reference_ids:
                    message = f"{product['id']} {relative}: reference discovery incomplete"
                    product_failures.append(message)
                    all_failures.append(message)
                if not candidate_discovery["passed"] or not candidate_ids:
                    message = f"{product['id']} {relative}: candidate discovery incomplete"
                    product_failures.append(message)
                    all_failures.append(message)
                for test_id in sorted(set(reference_ids) | set(candidate_ids)):
                    reference_case = reference_data["cases"].get(test_id)
                    if reference_case is None:
                        reference_case = missing_case(reference, relative, test_id,
                            "candidate-only ID absent from real reference discovery",
                            execution.product_identity(reference))
                    if test_id in candidate_ids:
                        candidate_case, _ = run_case(
                            product, harness, test_path, "ert", test_id, out, options.timeout,
                            compiled, stable_header, candidate_header, config_header)
                    else:
                        candidate_case = missing_case(
                            product, relative, test_id,
                            "candidate discovery did not contain expected reference ID", before)
                        product_unrun.append({"product": product["id"], "file": relative,
                                              "id": test_id, "reason": "missing candidate ID"})
                        all_unrun.append(product_unrun[-1])
                    reasons = []
                    if reference_case["status"] != "passed": reasons.append("reference did not pass")
                    if candidate_case["status"] != "passed": reasons.append("candidate did not pass")
                    if reference_case["skipped"] or candidate_case["skipped"]:
                        reasons.append("skip retained")
                        for side, row in (("reference", reference_case), ("candidate", candidate_case)):
                            if row["skipped"]:
                                item = {"product": product["id"], "file": relative,
                                        "id": test_id, "side": side}
                                product_skipped.append(item); all_skipped.append(item)
                    extra = test_id not in reference_ids
                    if extra: reasons.append("candidate extra ID")
                    case = dict(id=test_id, file=relative, reference=reference_case,
                        candidate=candidate_case,
                        passed=reference_case["passed"] is True and candidate_case["passed"] is True and not extra,
                        reasons=reasons, candidate_extra=extra)
                    product_cases.append(case); all_cases.append(case)
                    if not case["passed"]:
                        message = f"{product['id']} {relative} {test_id}: " + ", ".join(reasons)
                        product_failures.append(message); all_failures.append(message)
            after = execution.product_identity(product)
            unchanged = before == after
            product_row = dict(
                id=product["id"],
                kind=product["kind"],
                before=before,
                after=after,
                unchanged=unchanged,
                harness=harness,
                discoveries=product_discoveries,
                cases=product_cases,
                failures=product_failures,
                unrun=product_unrun,
                skipped=product_skipped,
                cleanup_eligible=unchanged and not product_failures and not product_unrun and not product_skipped,
            )
            product_rows.append(product_row)
            if not unchanged:
                message = f"{product['id']}: product identity changed"
                all_failures.append(message)
            if product_row["cleanup_eligible"]:
                cleanup[product["id"]] = retire_harness(harness, out, "all expected cases completed")
            else:
                cleanup[product["id"]] = dict(state="retained-failure", harness=harness["path"])

        reference_ready = all(data["ids"] and data["discovery"]["passed"]
                              and all(row["passed"] for row in data["cases"].values())
                              for data in reference_cache.values())
        if reference_ready:
            cleanup[reference["id"]] = retire_harness(reference_harness, out,
                                                       "all original reference cases completed")
        else:
            cleanup[reference["id"]] = dict(state="retained-failure", harness=reference_harness["path"])
        if reference_ready and all(product["cleanup_eligible"] for product in product_rows):
            evidence = out / "compiled-module-retirement-input.json"
            execution.write_json(evidence, dict(module=compiled["identity"], process=compile_process,
                reason="all comparisons completed; compile logs and original inputs retained"))
            cleanup["module-build"] = execution.load_tool('runtime-retire.py').retire(
                out, ('compiled-module',), evidence, compile_process)
        else:
            cleanup["module-build"] = dict(state="retained-failure",
                                           path=str(out / "module-build"))

        after_products = {product["id"]: execution.product_identity(product, resources=True)
                          for product in products}
        full_unchanged = before_products == after_products
        cleanup_complete = all(item.get('status') == 'complete' for item in cleanup.values())
        fixtures_unchanged = (
            sha(module_source) == file_record(module_source)["sha256"]
            and all(sha(Path(record["path"])) == record["sha256"]
                    for record in stable_records.values())
            and sha(Path(stable_header["path"])) == stable_header["sha256"]
            and (candidate_header is None or sha(Path(candidate_header["path"])) == candidate_header["sha256"])
            and sha(Path(config_header["path"])) == config_header["sha256"]
            and sha(RUNNER) == file_record(RUNNER)["sha256"]
            and sha(Path(stable_manifest["path"])) == stable_manifest["sha256"]
        )
        passed = (
            full_unchanged
            and fixtures_unchanged
            and bool(all_cases)
            and all(case["passed"] is True for case in all_cases)
            and not all_unrun
            and not all_skipped
            and not all_failures
            and cleanup_complete
        )
        report = dict(
            schema="femacs-module-thread-acceptance-v1",
            action="module-thread",
            products=product_rows,
            reference=dict(product=reference, files=reference_cache),
            product_identities=dict(before=before_products, after=after_products),
            full_product_inputs_unchanged=full_unchanged,
            fixtures_unchanged=fixtures_unchanged,
            discoveries=all_discoveries,
            cases=all_cases,
            unrun=all_unrun,
            failures=all_failures,
            skipped=all_skipped,
            passed=passed,
            cleanup=cleanup,
            cleanup_complete=cleanup_complete,
            goal_complete=False,
            compatibility_accepted=False,
            performance_accepted=False,
        )
        execution.write_json(out / "results.json", report)

        index_path = out / "case-index.jsonl"
        index_count = 0
        indexed_records = set()
        with index_path.open("x", encoding="utf-8") as stream:
            index_rows = list(all_discoveries)
            index_rows.extend(row for data in reference_cache.values() for row in data['cases'].values())
            index_rows.extend(case['candidate'] for case in all_cases)
            for row in index_rows:
                    record = row.get("record_path")
                    if record is None:
                        continue
                    path = Path(record)
                    if record in indexed_records: continue
                    indexed_records.add(record)
                    if not path.is_file():
                        raise AcceptanceError(f"case record disappeared: {path}")
                    stream.write(json.dumps(dict(
                        path=str(path.relative_to(out)),
                        bytes=path.stat().st_size,
                        sha256=sha(path),
                        complete=row["complete"],
                        exit_code=row["exit_code"],
                        status=row["status"],
                        passed=row["passed"],
                    ), separators=(",", ":")) + "\n")
                    index_count += 1
            stream.flush()
            os.fsync(stream.fileno())

        retirement = dict(
            schema="femacs-module-thread-retirement-v1",
            products=cleanup,
            module_source=file_record(module_source),
            compiled_module=compiled["identity"],
            stable_module_header=stable_header,
            candidate_module_header=candidate_header,
        )
        execution.write_json(out / "state-retirement.json", retirement)

        completion = dict(
            schema="femacs-module-thread-completion-v1",
            action="module-thread",
            execution_complete=True,
            passed=passed,
            post_hoc=False,
            counts=dict(
                products=dict(total=len(product_rows),
                              passed=sum(row["cleanup_eligible"] for row in product_rows)),
                discoveries=dict(total=len(all_discoveries),
                                 passed=sum(row["passed"] is True for row in all_discoveries),
                                 failed=sum(row["passed"] is False for row in all_discoveries)),
                cases=dict(total=len(all_cases),
                           passed=sum(case["passed"] is True for case in all_cases),
                           failed=sum(case["passed"] is False for case in all_cases)),
                unrun=len(all_unrun),
                skipped=len(all_skipped),
            ),
            full_product_inputs_unchanged=full_unchanged,
            fixtures_unchanged=fixtures_unchanged,
            inputs_sha256=sha(out / "inputs.json"),
            detailed=output_identity(out / "results.json"),
            case_index=dict(path=str(index_path.name), bytes=index_path.stat().st_size,
                            sha256=sha(index_path), count=index_count),
            retirement=dict(path=str((out / "state-retirement.json").name),
                            bytes=(out / "state-retirement.json").stat().st_size,
                            sha256=sha(out / "state-retirement.json")),
            goal_complete=False,
            compatibility_accepted=False,
            performance_accepted=False,
        )
        execution.write_json(out / "completion.json", completion)
        print(json.dumps(dict(action="module-thread", passed=passed,
                              results=str(out / "results.json"),
                              completion=str(out / "completion.json"))))
        return 0 if passed else 1
    except BaseException as error:
        execution.write_json(out / "fatal.json",
                             dict(passed=False, complete=False,
                                  error=f"{type(error).__name__}: {error}"))
        raise


if __name__ == "__main__":
    raise SystemExit(main())
