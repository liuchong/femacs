"""Actual isolated product/native/dump execution, invoked only in final D.

Run the whole command under controlled-run; do not nest another controller.
prepare-negative invokes a compiler and needs build protection. Other actions
execute the selected products and need runtime protection. No action is a Goal
completion claim. Products, dumps and resource trees are explicit inputs.
"""
from __future__ import annotations

import argparse
import ctypes
import ctypes.util
import functools
import gzip
import stat
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import threading
import time

REPO = Path(__file__).resolve().parent.parent
TOOL_ROOT = Path(__file__).resolve().parent
HARNESS = REPO / "tools/fixtures/runtime-process-runner.el"
FIXTURES = REPO / "tools/fixtures"
SAFE_ID = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]*$")
HEX = re.compile(r"^[0-9a-f]{64}$")
FORBIDDEN_ENV = {"EMACSLOADPATH", "EMACSDATA", "EMACSDOC", "EMACSPATH", "EMACSNATIVELOADPATH",
                 "DYLD_LIBRARY_PATH", "DYLD_FALLBACK_LIBRARY_PATH", "GCC_EXEC_PREFIX",
                 "COMPILER_PATH", "LIBRARY_PATH", "FEMACS_RUNTIME_BINDING"}
_RUN_OWNER_PID, _RUN_OWNER_PARENT_PID = os.getpid(), os.getppid()
_OWNER_LOSS_GRACE_SECONDS = 30


def _run_owner_present():
    # Each coordinator imports its own helper. Forked or orphaned copies must
    # not silently adopt a new parent and continue an old controlled operation.
    return (_RUN_OWNER_PARENT_PID > 1 and os.getpid() == _RUN_OWNER_PID
            and os.getppid() == _RUN_OWNER_PARENT_PID)


def digest(path):
    result = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            result.update(block)
    return result.hexdigest()


def write_json(path, payload):
    # Replace only this owner's evolving receipt, never an input/product.
    path = Path(path)
    temporary = path.with_name(path.name + ".writing")
    with temporary.open("x", encoding="utf-8") as stream:
        json.dump(payload, stream, indent=2, ensure_ascii=False, allow_nan=False)
        stream.write("\n")
    os.replace(temporary, path)


def bounded_json(path, limit=16 * 1024 * 1024):
    path = Path(path)
    if path.stat().st_size > limit:
        raise ValueError("oversized JSON input: " + path.name)
    return json.loads(path.read_text(encoding="utf-8"))


def write_completion(out, report):
    """Bind complete detailed evidence without making its array an input gate."""
    out = Path(out)
    inputs = bounded_json(out / 'inputs.json')
    index = out / 'case-index.jsonl'
    seen = set()
    count = 0

    def processes(value):
        if isinstance(value, dict):
            if {'argv', 'pid', 'cwd', 'complete', 'exit_code'} <= value.keys():
                yield value
            else:
                for child in value.values():
                    yield from processes(child)
        elif isinstance(value, list):
            for child in value:
                yield from processes(child)

    with index.open('x', encoding='utf-8') as stream:
        for row in processes(report):
            directory = Path(row['evidence_directory'])
            if not directory.is_relative_to(out):
                raise ValueError('case evidence escaped its owned execution root')
            path = directory / 'result.json'
            if not path.is_file():
                path = directory / 'process.json'
            if path in seen:
                continue
            if not path.is_file():
                raise ValueError('completed process has no actual case evidence')
            seen.add(path)
            stream.write(json.dumps(dict(path=str(path.relative_to(out)),
                sha256=digest(path), bytes=path.stat().st_size,
                complete=row['complete'], exit_code=row['exit_code']),
                separators=(',', ':')) + '\n')
            count += 1
        stream.flush()
        os.fsync(stream.fileno())
    counts = {}
    for key in ('cases', 'products', 'processes', 'startups'):
        rows = report.get(key)
        if isinstance(rows, list):
            counts[key] = dict(total=len(rows),
                passed=sum(row.get('passed') is True for row in rows if isinstance(row, dict)),
                failed=sum(row.get('passed') is False for row in rows if isinstance(row, dict)))
    if report['action'] == 'ert':
        for operation, key in (('discover', 'discoveries'), ('ert', 'ert_tests')):
            rows = [row for row in report['cases'] if row.get('request', {}).get('operation') == operation]
            counts[key] = dict(total=len(rows), passed=sum(row.get('passed') is True for row in rows),
                               failed=sum(row.get('passed') is False for row in rows))
    result = out / 'results.json'
    completion = dict(schema='femacs-runtime-completion-v1', action=report['action'],
        result_schema=report['schema'], execution_complete=True,
        passed=report['passed'] is True, post_hoc=False, counts=counts,
        full_product_inputs_unchanged=report.get('full_product_inputs_unchanged') is True,
        fixtures_unchanged=report.get('fixtures_unchanged') is True,
        inputs_sha256=digest(out / 'inputs.json'), input_binding=inputs,
        detailed=dict(path='results.json', bytes=result.stat().st_size, sha256=digest(result)),
        case_index=dict(path=index.name, bytes=index.stat().st_size, sha256=digest(index), count=count),
        retirement=dict(path='state-retirement.json', bytes=(out/'state-retirement.json').stat().st_size,
                        sha256=digest(out/'state-retirement.json')),
        goal_complete=False, compatibility_accepted=False, performance_accepted=False)
    if report['action'] == 'performance':
        fallback = inputs.get('permanent_full_fallback')
        if (type(fallback) is not bool or report.get('permanent_full_fallback') is not fallback
                or report.get('startup_measured') is not (not fallback)):
            raise ValueError('performance input and actual measurement mode differ')
        completion['performance_control'] = dict(permanent_full_fallback=fallback,
                                                  startup_measured=not fallback)
    write_json(out / 'completion.json', completion)
    return completion


def read_completion(root, action, products_path):
    """Validate the small receipt, then stream-bind its full immutable evidence."""
    root = safe_path(root)
    receipt = bounded_json(root / 'completion.json', 1024 * 1024)
    if (receipt.get('schema') != 'femacs-runtime-completion-v1'
            or receipt.get('result_schema') != 'femacs-runtime-execution-v1'
            or receipt.get('action') != action
            or receipt.get('execution_complete') is not True
            or receipt.get('post_hoc') is not False
            or type(receipt.get('passed')) is not bool):
        raise ValueError('invalid completed execution receipt')
    inputs = bounded_json(root / 'inputs.json')
    if (digest(root / 'inputs.json') != receipt.get('inputs_sha256')
            or inputs != receipt.get('input_binding')
            or inputs.get('product_list_sha256') != digest(products_path)
            or inputs.get('action') != action):
        raise ValueError('completed execution input identity mismatch')
    if action == 'performance':
        fallback = inputs.get('permanent_full_fallback')
        control = receipt.get('performance_control')
        if (type(fallback) is not bool or type(control) is not dict
                or set(control) != {'permanent_full_fallback', 'startup_measured'}
                or control['permanent_full_fallback'] is not fallback
                or control['startup_measured'] is not (not fallback)):
            raise ValueError('completed performance mode binding mismatch')
    for key, name in (('detailed', 'results.json'), ('case_index', 'case-index.jsonl')):
        identity = receipt.get(key, {})
        path = root / name
        if (identity.get('path') != name or path.is_symlink()
                or type(identity.get('bytes')) is not int
                or not isinstance(identity.get('sha256'), str)
                or not HEX.fullmatch(identity['sha256'])
                or path.stat().st_size != identity.get('bytes')
                or digest(path) != identity.get('sha256')):
            raise ValueError('completed execution evidence identity mismatch: ' + key)
    retirement = receipt.get('retirement', {})
    path = root/'state-retirement.json'
    if (retirement.get('path') != path.name or path.is_symlink()
            or path.stat().st_size != retirement.get('bytes')
            or digest(path) != retirement.get('sha256')):
        raise ValueError('completed execution retirement evidence changed')
    with (root / 'case-index.jsonl').open('rb') as stream:
        actual_count = sum(block.count(b'\n') for block in iter(lambda: stream.read(262144), b''))
    if (type(receipt['case_index'].get('count')) is not int
            or actual_count != receipt['case_index']['count']):
        raise ValueError('completed execution case index count mismatch')
    if receipt['passed'] and not (receipt['full_product_inputs_unchanged'] and receipt['fixtures_unchanged']):
        raise ValueError('completed execution incorrectly passed changed inputs')
    return receipt


def safe_path(path, *, must_exist=True):
    absolute = Path(os.path.abspath(path))
    current = Path(absolute.anchor)
    for part in absolute.parts[1:]:
        current /= part
        if current.is_symlink():
            raise ValueError("symlink component in owned/input path: " + str(current))
    if must_exist and not absolute.exists():
        raise ValueError("missing input: " + str(absolute))
    return absolute


def new_output(path):
    absolute = safe_path(path, must_exist=False)
    if not absolute.is_relative_to(REPO / "build") or absolute == REPO / "build":
        raise ValueError("fresh output must be an owned build child")
    absolute.parent.mkdir(parents=True, exist_ok=True)
    absolute.mkdir(mode=0o700)
    return absolute


def freeze_source(out):
    """Materialize current tracked and new inputs, not HEAD's older archive.

    This is a final-D preparation action. Ignored source overlays named by the
    exact stable manifest are included explicitly. Build/cache/reference trees
    and private native conversation logs are never inputs.
    """
    top = subprocess.run(["git", "rev-parse", "--show-toplevel"], cwd=REPO,
                         check=True, stdout=subprocess.PIPE, text=True).stdout.strip()
    if Path(top).resolve() != REPO:
        raise ValueError("source inventory must belong to this repository")
    listing = subprocess.run(["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"],
                             cwd=REPO, check=True, stdout=subprocess.PIPE).stdout
    names = {name.decode("utf-8") for name in listing.split(b"\0") if name}
    provider_spec = importlib.util.spec_from_file_location("femacs_snapshot_corpus", REPO / "tools/reference-corpus.py")
    provider = importlib.util.module_from_spec(provider_spec)
    provider_spec.loader.exec_module(provider)
    corpus_inputs = provider.export_inputs(REPO / "compatibility/stable-31.1/sources.json",
                                          REPO / ".femacs-cache/stable-corpus", repo=REPO)
    corpus = corpus_inputs["manifest"]
    corpus_entries = {entry["source_path"]: entry for entry in corpus["entries"]}
    names.update(corpus_inputs["sources"])
    excluded = {"emacs", "build", "dist", "target", ".femacs-cache", ".git"}
    names = sorted(name for name in names if Path(name).parts[0] not in excluded)
    if not names:
        raise ValueError("empty current source inventory")
    destination = out / "source"
    destination.mkdir()
    identities = {}
    for name in names:
        relative = Path(name)
        if relative.is_absolute() or any(part in ("..", ".") for part in relative.parts):
            raise ValueError("unsafe source snapshot path")
        source = corpus_inputs["sources"].get(name, REPO / relative)
        # Permit a source symlink itself, but never follow a symlinked parent
        # into another checkout when copying tracked/new source bytes.
        safe_path(source.parent)
        before = file_identity(source)
        if name in corpus_entries:
            entry = corpus_entries[name]
            expected = dict(kind="file", bytes=entry["size"], sha256=entry["sha256"])
            if any(before.get(key)!=value for key,value in expected.items()):
                raise ValueError("manifest corpus input changed before snapshot: " + name)
            provider.suite._check_mode(before['mode'], entry['mode'], 'snapshot source')
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        if before["kind"] == "symlink":
            link = Path(before["target"])
            if link.is_absolute() or not (source.parent / link).resolve().is_relative_to(REPO):
                raise ValueError("source symlink leaves the snapshot")
            target.symlink_to(link)
        else:
            with source.open("rb") as reader, target.open("xb") as writer:
                shutil.copyfileobj(reader, writer, length=1024 * 1024)
            target.chmod(before["mode"])
        if before != file_identity(source) or before != file_identity(target):
            raise ValueError("source changed during frozen export: " + name)
        identities[name] = before
    write_json(destination / ".femacs-runtime-sources.json", dict(schema=1, files=names))
    write_json(out / "source-inputs.json", dict(schema=1, files=identities,
                                               inventory="current tracked and new source plus verified manifest/cache corpus inputs",
                                               corpus_manifest_sha256=corpus_inputs["manifest_sha256"],
                                               corpus_acquisition=corpus_inputs["acquisition"],
                                               tests_executed=False))
    return dict(passed=True, source=str(destination), file_count=len(names),
                source_manifest_sha256=digest(out / "source-inputs.json"), tests_executed=False)


def file_identity(path):
    path = Path(path)
    meta = path.lstat()
    if path.is_symlink():
        return {"kind": "symlink", "target": os.readlink(path), "mode": meta.st_mode & 0o777}
    if not path.is_file():
        raise ValueError("identity input is not a regular file")
    return {"kind": "file", "bytes": meta.st_size, "mode": meta.st_mode & 0o777,
            "sha256": digest(path)}


def verify_source_export(original, source_export):
    """Verify the exact snapshot inventory, not a newly computed source label."""
    original = safe_path(original)
    source = safe_path(source_export['source'])
    if source != original/'source':
        raise ValueError('source must be the named immutable export')
    manifest_path = original/'source-inputs.json'
    if digest(manifest_path) != source_export['source_manifest_sha256']:
        raise ValueError('frozen source inventory changed')
    manifest = bounded_json(manifest_path)
    exported = bounded_json(source/'.femacs-runtime-sources.json')
    if exported.get('schema')!=1 or exported.get('files')!=sorted(manifest['files']):
        raise ValueError('exported source list changed')
    if len(manifest['files'])!=source_export['file_count']:
        raise ValueError('frozen source count changed')
    for name, expected in manifest['files'].items():
        relative = Path(name)
        if relative.is_absolute() or '..' in relative.parts or not relative.parts:
            raise ValueError('unsafe frozen source input')
        safe_path((source/relative).parent)
        if file_identity(source/relative)!=expected:
            raise ValueError('frozen source input changed: '+name)
    return source


def tree_identity(root):
    root = safe_path(root)
    if not root.is_dir():
        raise ValueError("resource root must be a directory")
    result = {}
    for directory, subdirs, names in os.walk(root, followlinks=False):
        for name in sorted(list(subdirs)):
            item = Path(directory) / name
            if item.is_symlink():
                result[item.relative_to(root).as_posix()] = file_identity(item)
                subdirs.remove(name)
        for name in sorted(names):
            item = Path(directory) / name
            result[item.relative_to(root).as_posix()] = file_identity(item)
    if not result:
        raise ValueError("empty resource tree is not a product identity")
    return result


def copy_owned_data(source, destination):
    """Copy bytes/mode without propagating an input owner's immutable flags."""
    shutil.copyfile(source, destination)
    shutil.copymode(source, destination)
    return destination


def product_receipt_layout(value, root):
    """Require the complete recorded product, including its resource groups."""
    binding = value['binding']
    kind, variant = binding.get('product'), binding.get('variant')
    if kind not in ('terminal', 'cocoa') or variant not in ('baseline', 'native', 'collector', 'native-collector'):
        raise ValueError('unknown completed product kind or variant')
    if kind == 'cocoa':
        package = root/'dist/Femacs.app'
        program, dump = 'dist/Femacs.app/Contents/MacOS/Femacs', 'dist/Femacs.app/Contents/Resources/libexec/Femacs.pdmp'
        resources = {'generated-config':root/'config', 'bundle':package}
        roots = {'bundle':str(package/'Contents/Resources')}
    else:
        package = root/'dist/terminal'
        program, dump = 'dist/terminal/src/emacs', 'dist/terminal/src/emacs.pdmp'
        info = safe_path(bounded_json(root/'config/manual-set.json')['info'])
        if not info.is_relative_to(REPO/'build'):
            raise ValueError('manual identity outside owned build output')
        resources = {'generated-config':root/'config', 'terminal-package':package, 'info':info}
        roots = {name:str(package/name) for name in ('lisp', 'etc', 'info')}
    if 'native' in variant:
        resources.update({'runtime-native':root/'runtime-native', 'licenses':root/'licenses'})
        roots.update({name:str(root/name) for name in ('runtime-native', 'licenses')})
    selected = bounded_json(root / 'runtime-config.json')
    tls_settings = [value for section in ('macros', 'features')
                    for key, value in selected.get(section, {}).items()
                    if key.upper() == 'HAVE_GNUTLS']
    tls_enabled = any(value is True or value == 1 or value in ('1', 'true')
                      for value in tls_settings)
    if tls_enabled:
        resources['tls'] = root / 'tls'
        if kind == 'cocoa':
            roots['tls-runtime'] = str(package / 'Contents/Frameworks/runtime-tls')
            roots['tls-licenses'] = str(package / 'Contents/Resources/licenses/tls')
        else:
            roots['tls-runtime'] = str(package / 'runtime-tls')
            roots['tls-licenses'] = str(package / 'licenses/tls')
    lcms_settings = [value for section in ('macros', 'features')
                     for key, value in selected.get(section, {}).items()
                     if key.upper() == 'HAVE_LCMS2']
    lcms_enabled = any(value is True or value == 1 or value in ('1', 'true')
                       for value in lcms_settings)
    if lcms_enabled:
        resources['lcms'] = root / 'lcms'
        if kind == 'cocoa':
            roots['lcms-runtime'] = str(package / 'Contents/Frameworks/runtime-lcms')
            roots['lcms-licenses'] = str(package / 'Contents/Resources/licenses/lcms')
        else:
            roots['lcms-runtime'] = str(package / 'runtime-lcms')
            roots['lcms-licenses'] = str(package / 'licenses/lcms')
    treesit_settings = [value for section in ('macros', 'features')
                        for key, value in selected.get(section, {}).items()
                        if key.upper() == 'HAVE_TREE_SITTER']
    if any(value is True or value == 1 or value in ('1', 'true') for value in treesit_settings):
        resources['treesit'] = root / 'treesit'
        if kind == 'cocoa':
            roots['treesit-runtime'] = str(package / 'Contents/Frameworks/runtime-treesit')
            roots['treesit-licenses'] = str(package / 'Contents/Resources/licenses/treesit')
        else:
            roots['treesit-runtime'] = str(package / 'runtime-treesit')
            roots['treesit-licenses'] = str(package / 'licenses/treesit')
    image_settings = [value for section in ('macros', 'features')
                      for key, value in selected.get(section, {}).items()
                      if key.upper() in ('HAVE_PNG', 'HAVE_JPEG')]
    if any(value is True or value == 1 or value in ('1', 'true') for value in image_settings):
        resources['image'] = root / 'image'
        if kind == 'cocoa':
            roots['image-runtime'] = str(package / 'Contents/Frameworks/runtime-image')
            roots['image-licenses'] = str(package / 'Contents/Resources/licenses/image')
        else:
            roots['image-runtime'] = str(package / 'runtime-image')
            roots['image-licenses'] = str(package / 'licenses/image')
    files = {'port/src/temacs', 'port/src/emacs', 'port/src/emacs.pdmp', 'port/src/bootstrap-emacs.pdmp', program, dump}
    identity_version = value.get('identity_version', 1)
    if type(identity_version) is not int or identity_version not in (1, 2, 3):
        raise ValueError('unknown product file identity version')
    if identity_version == 3:
        files.add('port/lib-src/emacsclient')
        files.add('dist/Femacs.app/Contents/MacOS/bin/emacsclient' if kind == 'cocoa'
                  else 'dist/terminal/lib-src/emacsclient')
    if value.get('schema') != 1 or set(value.get('files', {})) != files or set(value.get('resources', {})) != set(resources):
        raise ValueError('completed product file/resource identity set is incomplete or unexpected')
    return program, dump, resources, roots


def load_products(path):
    document = bounded_json(path)
    if document.get("schema") != 1 or not document.get("products"):
        raise ValueError("explicit schema1 product list required")
    result, seen, verified_resources = [], set(), set()
    for original in document["products"]:
        product = dict(original)
        identity = product.get("id", "")
        if not SAFE_ID.fullmatch(identity) or identity in seen:
            raise ValueError("invalid/duplicate product profile ID")
        seen.add(identity)
        if product.get("kind") not in ("reference", "terminal", "cocoa"):
            raise ValueError("explicit reference/terminal/cocoa product kind required")
        if product.get("compiler_leg") not in ("normal", "checking"):
            raise ValueError("explicit normal/checking product leg required")
        for flag in ("native", "observer", "test_hooks"):
            if type(product.get(flag)) is not bool:
                raise ValueError("explicit boolean product feature required: " + flag)
        if product.get("test_hooks") and not product["observer"]:
            raise ValueError("test-hooks product must include observer")
        if product.get("collector") not in ("off", "major", "young"):
            raise ValueError("explicit collector mode required")
        product["program"] = str(safe_path(product["program"]))
        product["dump"] = str(safe_path(product["dump"]))
        roots = product.get("resource_roots")
        if not isinstance(roots, dict) or not roots or any(not SAFE_ID.fullmatch(key) for key in roots):
            raise ValueError("named actual product resource roots required")
        product["resource_roots"] = {key: str(safe_path(value)) for key, value in roots.items()}
        if product["kind"] != "reference":
            receipt_path = safe_path(product["product_receipt"])
            receipt = bounded_json(receipt_path, 64 * 1024 * 1024)
            binding = receipt.get("binding", {})
            variant = ("native-collector" if product["native"] and product["collector"] != "off"
                       else "native" if product["native"] else "collector"
                       if product["collector"] != "off" else "baseline")
            if (receipt.get("schema") != 1 or receipt.get("build_complete") is not True
                    or binding.get("product") != product["kind"]
                    or binding.get("compiler_leg") != product["compiler_leg"]
                    or binding.get("variant") != variant
                    or binding.get("observer") != product["observer"]
                    or binding.get("test_hooks") != product["test_hooks"]):
                raise ValueError("product profile does not match whole-product build receipt")
            root = receipt_path.parent
            program_rel, dump_rel, resources, expected_roots = product_receipt_layout(receipt, root)
            if receipt.get('identity_version') in (2, 3):
                metadata = receipt.get('file_identities', {})
                if set(metadata) != set(receipt['files']):
                    raise ValueError('completed product file metadata is incomplete')
                for relative, identity in metadata.items():
                    if (identity.get('sha256') != receipt['files'][relative]
                            or file_identity(safe_path(root / relative)) != identity):
                        raise ValueError('completed product file metadata changed')
            if product['resource_roots'] != expected_roots:
                raise ValueError('selected resources do not belong to product receipt')
            resource_key = (str(receipt_path), digest(receipt_path))
            if resource_key not in verified_resources:
                for name, resource in resources.items():
                    if tree_identity(resource) != receipt['resources'][name]:
                        raise ValueError('product resources changed from build receipt: ' + name)
                verified_resources.add(resource_key)
            for relative, expected in receipt.get("files", {}).items():
                if Path(relative).is_absolute() or ".." in Path(relative).parts:
                    raise ValueError("unsafe product receipt path")
                if digest(safe_path(root / relative)) != expected:
                    raise ValueError("product bytes changed from build receipt")
            if (Path(product["program"]) != root / program_rel
                    or Path(product["dump"]) != root / dump_rel):
                raise ValueError("selected executable/dump does not belong to product receipt")
            product["product_receipt"] = str(receipt_path)
        result.append(product)
    return result


def product_identity(product, *, resources=False):
    result = {"program": file_identity(product["program"]), "dump": file_identity(product["dump"])}
    if resources:
        result["resources"] = {key: tree_identity(path) for key, path in product["resource_roots"].items()}
    if product.get("product_receipt"):
        result["build_receipt"] = file_identity(product["product_receipt"])
    return result


def stage_negative_inputs(origin, directory):
    origin = safe_path(origin)
    manifest = bounded_json(origin / "manifest.json")
    if manifest.get("schema") != "femacs-runtime-native-negative-v1" or not manifest.get("control"):
        raise ValueError("missing positive native format control")
    prepared = dict(schema=manifest["schema"], units=[])
    destination = directory / "negative"
    destination.mkdir()
    seen = set()
    for source in [manifest["control"], *manifest.get("units", [])]:
        relative = Path(source["path"])
        if relative.is_absolute() or ".." in relative.parts or str(relative) in seen:
            raise ValueError("unsafe/duplicate native format input")
        seen.add(str(relative))
        input_path = safe_path(origin / relative)
        if digest(input_path) != source["sha256"]:
            raise ValueError("native format input changed from preparation receipt")
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        copy_owned_data(input_path, target)
        item = dict(source, path=(Path("negative") / relative).as_posix())
        if source is manifest["control"]:
            prepared["control"] = item
        else:
            prepared["units"].append(item)
    write_json(destination / "manifest.json", prepared)
    return destination / "manifest.json"


def owned_environment(directory, product, observation="off"):
    env = {key: value for key, value in os.environ.items()
           if key not in FORBIDDEN_ENV and not key.startswith("FEMACS_")}
    paths = {"HOME": "home", "XDG_CACHE_HOME": "cache", "XDG_CONFIG_HOME": "config",
             "XDG_STATE_HOME": "state", "TMPDIR": "tmp", "FEMACS_RUNTIME_NATIVE_CACHE": "eln",
             "EMACSNATIVELOADPATH": "eln"}
    for key, name in paths.items():
        path = directory / name
        path.mkdir(exist_ok=True)
        env[key] = str(path)
    env.update(LC_ALL="C", FEMACS_RUNTIME_TEST_ROOT=str(directory),
               FEMACS_RUNTIME_OUT=str(directory / "out"),
               FEMACS_XML_TEST_DIR=str(directory / "out"),
               FEMACS_SQLITE_TEST_DIR=str(directory / "out"),
               FEMACS_GC_OBSERVATION=observation,
               FEMACS_GC_COLLECTOR=product["collector"] if product["collector"] != "off" else "major")
    (directory / "out").mkdir()
    return env


@functools.lru_cache(maxsize=1)
def prepare_memory_sampling():
    """Retain native bindings, never process identities or measurement values."""
    import psutil
    class RusageV1(ctypes.Structure):
        _fields_ = [("uuid", ctypes.c_ubyte * 16)] + [(name, ctypes.c_uint64) for name in (
            "user_time", "system_time", "pkg_idle_wkups", "interrupt_wkups", "pageins",
            "wired_size", "resident_size", "phys_footprint", "proc_start_abstime", "proc_exit_abstime",
            "child_user_time", "child_system_time", "child_pkg_idle_wkups", "child_interrupt_wkups",
            "child_pageins", "child_elapsed_abstime")]
    library = function = None
    if sys.platform == 'darwin':
        library = ctypes.CDLL(ctypes.util.find_library('proc'), use_errno=True)
        function = library.proc_pid_rusage
        function.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.c_void_p]
        function.restype = ctypes.c_int
    return psutil, RusageV1, library, function


def sample_memory(pid):
    # Measurement only. The external controlled-run owner enforces limits and
    # reaps its complete tree. Never inspect unrelated processes or budgets.
    psutil, RusageV1, _library, function = prepare_memory_sampling()
    rss = footprint = accounted = 0
    observed = 0
    try:
        parent = psutil.Process(pid)
        owned = [parent, *parent.children(recursive=True)]
    except psutil.NoSuchProcess:
        return None
    for process in owned:
        try:
            created = process.create_time()
            resident = process.memory_info().rss
            physical = resident
            if sys.platform == "darwin":
                observation = RusageV1()
                if function(process.pid, 1, ctypes.byref(observation)) != 0:
                    if not process.is_running() or process.status() == psutil.STATUS_ZOMBIE:
                        continue
                    raise OSError(ctypes.get_errno(), "cannot sample owned physical footprint")
                physical = observation.phys_footprint
            # psutil caches create_time on a Process instance. Reopen the PID
            # after both native reads so the identity check is actually fresh.
            if psutil.Process(process.pid).create_time() != created:
                raise RuntimeError("owned sampling PID identity changed")
            rss += resident
            footprint += physical
            accounted += max(resident, physical)
            observed += 1
        except psutil.NoSuchProcess:
            continue
    return (rss, footprint, accounted) if observed else None


class _ProcessCancelled(Exception):
    """The coordinator cancelled this owned child, not its original test."""


def _signal_child_group(child, number):
    try:
        os.killpg(child.pid, number)
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        # Darwin can report EPERM for an exited, unreaped group leader.
        # Reap only our own child, then retry; a live child or persistent
        # permission failure remains an error rather than an empty group.
        if child.poll() is None:
            raise
        try:
            os.killpg(child.pid, number)
            return True
        except ProcessLookupError:
            return False


class _OwnedProcessTree:
    """Keep birth identities across child-created sessions and reparenting."""
    def __init__(self):
        import psutil
        self.psutil = psutil
        self.root = None
        self.known = {}
        self.observed = 0
        self.errors = set()
        self.next_refresh = 0.0

    def _current(self, pid, birth):
        try:
            process = self.psutil.Process(pid)
            if (process.create_time() != birth
                    or process.status() == self.psutil.STATUS_ZOMBIE):
                return None
            return process
        except self.psutil.NoSuchProcess:
            return None
        except self.psutil.AccessDenied:
            self.errors.add('process-identity-access-denied')
            return None

    def refresh(self, *, force=False):
        now = time.monotonic()
        if not force and now < self.next_refresh:
            return
        self.next_refresh = now + .1
        if self.root is None:
            return
        visited = set()
        # Previously observed children can now belong to PID1 or a fresh
        # session. Their recorded birth, not their current parent, owns them.
        for pid, birth in list(self.known.items()):
            process = self._current(pid, birth)
            if process is None:
                self.known.pop(pid, None)
                continue
            if pid in visited:
                continue
            try:
                descendants = process.children(recursive=True)
                for item in [process, *descendants]:
                    current = self.psutil.Process(item.pid)
                    created = current.create_time()
                    if created != item.create_time():
                        continue
                    if item.pid not in self.known or self.known[item.pid] != created:
                        self.observed += 1
                    self.known[item.pid] = created
                    visited.add(item.pid)
            except self.psutil.NoSuchProcess:
                continue
            except self.psutil.AccessDenied:
                self.errors.add('process-descendants-access-denied')

    def attach(self, child):
        self.root = child.pid
        try:
            self.known[child.pid] = self.psutil.Process(child.pid).create_time()
            self.observed += 1
        except self.psutil.NoSuchProcess:
            # A short, already-reaped command has no live root to enumerate.
            return

    def live(self):
        return [process for pid, birth in list(self.known.items())
                if (process := self._current(pid, birth)) is not None]

    def signal(self, number, *, exclude_group=None):
        self.refresh(force=True)
        # Signal the root last so a live parent remains observable while its
        # separately sessioned children are collected. Never signal a reused PID.
        for process in sorted(self.live(), key=lambda item: item.pid == self.root):
            current = self._current(process.pid, self.known[process.pid])
            if current is None:
                continue
            try:
                if exclude_group is not None and os.getpgid(current.pid) == exclude_group:
                    continue
                current.send_signal(number)
            except (self.psutil.NoSuchProcess, ProcessLookupError):
                continue
            except (self.psutil.AccessDenied, PermissionError):
                self.errors.add('owned-process-signal-access-denied')

    def finish(self):
        deadline = time.monotonic() + 2
        while self.live() and time.monotonic() < deadline:
            self.signal(signal.SIGKILL)
            time.sleep(.01)
        remaining = self.live()
        return dict(complete=not remaining and not self.errors,
                    observed_processes=self.observed,
                    remaining=[dict(pid=p.pid, birth=self.known[p.pid]) for p in remaining],
                    errors=sorted(self.errors))


def run_process(argv, directory, env, timeout, *, stdin=None, metrics=False, cwd=None,
                observe_exit=False, cancel_event=None):
    directory.mkdir(parents=True, exist_ok=True)
    started = time.monotonic()
    working_directory = Path(cwd or directory).resolve()
    # Popen(cwd=...) changes the actual cwd but does not update an inherited
    # PWD string. Bind both so child-owned diagnostics use the real case root.
    child_env = dict(env, PWD=str(working_directory))
    row = dict(argv=[str(part) for part in argv], complete=False, timed_out=False,
               cwd=str(working_directory), evidence_directory=str(directory.resolve()),
               owner_pid=_RUN_OWNER_PID, owner_parent_pid=_RUN_OWNER_PARENT_PID,
               exit_code=None, status="not-started")
    with (directory / "stdout.txt").open("xb") as stdout, (directory / "stderr.txt").open("xb") as stderr:
        try:
            if not _run_owner_present():
                row.update(cancelled=True, owner_parent_lost=True, status='cancelled')
                raise SystemExit(125)
            if cancel_event is not None and cancel_event.is_set():
                row.update(cancelled=True, status='cancelled')
                return row
            if metrics:
                preparation_started = time.monotonic()
                prepare_memory_sampling()
                row['sampler_preparation_seconds'] = time.monotonic()-preparation_started
            tree = _OwnedProcessTree()
            if not _run_owner_present():
                row.update(cancelled=True, owner_parent_lost=True, status='cancelled')
                raise SystemExit(125)
            spawn_started = time.monotonic()
            child = subprocess.Popen([str(part) for part in argv], cwd=working_directory,
                                     env=child_env, stdin=subprocess.DEVNULL if stdin is None else stdin,
                                     stdout=stdout, stderr=stderr, start_new_session=True)
            row["pid"] = child.pid
            exit_event, exit_info, exit_thread = None, {}, None
            try:
                tree.attach(child)
                if observe_exit:
                    exit_event = threading.Event()
                    def observe_child_exit():
                        try:
                            child.wait()
                            exit_info['ended'] = time.monotonic()
                        except Exception as error:
                            exit_info['error'] = f'{type(error).__name__}: {error}'
                        finally:
                            exit_event.set()
                    exit_thread = threading.Thread(target=observe_child_exit,
                                                    name='runtime-owned-child-exit')
                    exit_thread.start()
                tree.refresh(force=True)
                if metrics:
                    row.update(peak_rss_bytes=0, peak_footprint_bytes=0, peak_accounted_bytes=0,
                               memory_samples=0, sample_errors=[], sampling_interval_seconds=.02,
                               load_per_core_before=os.getloadavg()[0] / max(1, os.cpu_count() or 1))
                    while child.poll() is None:
                        tree.refresh()
                        if not _run_owner_present():
                            row['owner_parent_lost'] = True
                            raise _ProcessCancelled()
                        if cancel_event is not None and cancel_event.is_set():
                            raise _ProcessCancelled()
                        if time.monotonic() - spawn_started > timeout:
                            raise subprocess.TimeoutExpired(argv, timeout)
                        try:
                            sample = sample_memory(child.pid)
                            if sample is not None:
                                row["memory_samples"] += 1
                                for key, amount in zip(("peak_rss_bytes", "peak_footprint_bytes", "peak_accounted_bytes"), sample):
                                    row[key] = max(row[key], amount)
                        except Exception as error:
                            row["sample_errors"].append(f"{type(error).__name__}: {error}")
                        if exit_event is None:
                            time.sleep(.02)
                        else:
                            exit_event.wait(.02)
                    row["load_per_core_after"] = os.getloadavg()[0] / max(1, os.cpu_count() or 1)
                    row["memory_sampling_complete"] = (row["memory_samples"] > 0
                        and row['peak_accounted_bytes'] > 0 and not row["sample_errors"])
                else:
                    deadline = None if timeout is None else spawn_started + timeout
                    while child.poll() is None:
                        tree.refresh()
                        if not _run_owner_present():
                            row['owner_parent_lost'] = True
                            raise _ProcessCancelled()
                        if cancel_event is not None and cancel_event.is_set():
                            raise _ProcessCancelled()
                        remaining = None if deadline is None else deadline - time.monotonic()
                        if remaining is not None and remaining <= 0:
                            raise subprocess.TimeoutExpired(argv, timeout)
                        try:
                            child.wait(timeout=.1 if remaining is None else min(.1, remaining))
                        except subprocess.TimeoutExpired:
                            # This bounded poll is not an extension of the
                            # original process deadline.
                            pass
            except _ProcessCancelled:
                row['cancelled'] = True
                tree.refresh(force=True)
                if row.get('owner_parent_lost'):
                    # Cooperative owners can unwind on TERM. Other Python CLIs
                    # may exit immediately; retain their already observed child
                    # identities even when those children have their own session.
                    tree.signal(signal.SIGTERM, exclude_group=child.pid)
                    _signal_child_group(child, signal.SIGTERM)
                    deadline = time.monotonic() + _OWNER_LOSS_GRACE_SECONDS
                    while tree.live() and time.monotonic() < deadline:
                        tree.refresh()
                        child.poll()
                        time.sleep(.02)
                tree.signal(signal.SIGKILL)
                _signal_child_group(child, signal.SIGKILL)
                child.wait()
            except subprocess.TimeoutExpired:
                row["timed_out"] = True
                tree.signal(signal.SIGKILL)
                _signal_child_group(child, signal.SIGKILL)
                child.wait()
            finally:
                tree.refresh(force=True)
                descendants_remained = any(p.pid != child.pid for p in tree.live())
                tree.signal(signal.SIGKILL)
                # The group is unique to this owner; collect children still
                # alive after their parent exits. Never use process names.
                row['remaining_group_terminated'] = _signal_child_group(child, signal.SIGKILL)
                if descendants_remained and not row.get('cancelled') and not row['timed_out']:
                    row['remaining_group_terminated'] = True
                # Unwind the real child before outer artifact guards release;
                # SIGINT/SIGTERM exceptions must not leave its last writes racing.
                child.wait()
                row['owned_process_cleanup'] = tree.finish()
                if observe_exit:
                    if exit_thread is not None and exit_thread.ident is not None:
                        exit_thread.join(timeout=1)
                    row['exit_timing_method'] = 'spawn-to-reaped-exit-waiter'
                    row['exit_timing_complete'] = (exit_thread is not None
                        and exit_thread.ident is not None and not exit_thread.is_alive()
                        and exit_event.is_set() and 'ended' in exit_info and 'error' not in exit_info)
                    row['process_lifetime_seconds'] = (exit_info['ended'] - spawn_started
                        if row['exit_timing_complete'] else None)
                    if 'error' in exit_info:
                        row['exit_timing_error'] = exit_info['error']
            if not _run_owner_present():
                row.update(cancelled=True, owner_parent_lost=True)
            row.update(exit_code=child.returncode,
                       complete=not row["timed_out"] and not row.get('cancelled', False)
                       and row['owned_process_cleanup']['complete'],
                       status=('cancelled' if row.get('cancelled') else
                               'timeout' if row['timed_out'] else 'exited'))
        except (OSError, ValueError) as error:
            row.update(status='process-error' if 'pid' in row else 'spawn-error', error=str(error))
        finally:
            row["seconds"] = time.monotonic() - started
            write_json(directory / "process.json", row)
    if row.get('owner_parent_lost'):
        raise SystemExit(125)
    return row


def invoke(product, directory, request, timeout, observation="off", *, extra_env=None):
    directory.mkdir(parents=True)
    env = owned_environment(directory, product, observation)
    env.update(extra_env or {})
    request = dict(request, schema=1)
    fixture_sources = {str(Path(path)): digest(path) for path in request.get('fixtures', [])}
    negative = request.pop("negative_source", None)
    if negative is not None:
        env["FEMACS_RUNTIME_NEGATIVE_MANIFEST"] = str(stage_negative_inputs(negative, directory))
    write_json(directory / "request.json", request)
    env.update(FEMACS_RUNTIME_REQUEST=str(directory / "request.json"),
               FEMACS_RUNTIME_RESULT=str(directory / "response.json"))
    before = product_identity(product)
    harness = Path(product.get("_harness", HARNESS))
    harness_before = digest(harness)
    row = run_process([*product.get("_command_prefix", []), product["program"], "--dump-file=" + product["dump"],
                       "-Q", "--batch", "--load", harness], directory, env, timeout)
    row.update(profile=product["id"], request=request, source_identity_before=before,
               source_identity_after=product_identity(product), harness_sha256=harness_before,
               fixture_sources=fixture_sources,
               fixture_sources_after={path: digest(path) for path in fixture_sources})
    try:
        row["response"] = bounded_json(directory / "response.json")
        if row["response"].get("schema") != "femacs-runtime-process-v1":
            raise ValueError("invalid runtime product response schema")
    except (OSError, ValueError) as error:
        row["response_error"] = str(error)
    row["passed"] = (row["exit_code"] == 0 and row["complete"]
                     and row.get("response", {}).get("complete") is True
                     and row["source_identity_before"] == row["source_identity_after"]
                     and row['fixture_sources'] == row['fixture_sources_after']
                     and digest(harness) == harness_before)
    if row["remaining_group_terminated"]:
        row["passed"] = False
        row["child_cleanup_error"] = "owned descendants outlived the product process"
    write_json(directory / "result.json", row)
    return row


def fixture_groups(product, observation):
    groups = [("language", ["runtime-language.el", "runtime-symbol-intern-functional.el"], "^rtl"),
              ("xml", ["xml-acceptance.el"], "^fe-xml-"),
              ("sqlite", ["sqlite-acceptance.el"], "^fe-sqlite-")]
    if product["kind"] == "reference":
        return groups
    if product["observer"]:
        groups.append(("observer", ["runtime-observer.el"], "^femacs-runtime-observer/"))
        if product["test_hooks"]:
            groups.append(("observer-fault", ["runtime-observer.el"], "^femacs-runtime-observer-fault/"))
    if product["collector"] != "off":
        groups.append(("collector", ["runtime-gc.el"], "^rtg"))
    if product["native"]:
        groups.append(("native", ["runtime-native.el", "runtime-native-lifecycle.el"], "^rtn"))
    return groups


def run_ert(products, out, timeout, negative_roots, only_groups=None):
    rows, registrations = [], {}
    for product in products:
        modes = ["off", "phases", "survival", "all"] if product["observer"] else ["off"]
        for mode in modes:
            for name, fixtures, selector in fixture_groups(product, mode):
                if only_groups is not None and name not in only_groups:
                    continue
                root = out / product["id"] / mode / name
                loaded = [str(FIXTURES / fixture) for fixture in fixtures]
                request = dict(operation="discover", fixtures=loaded, selector=selector)
                discovery = invoke(product, root / "discovery", request, timeout, mode)
                discovery['observer_mode'] = mode
                rows.append(discovery)
                ids = discovery.get("response", {}).get("ids", [])
                if not isinstance(ids, list) or not ids or len(set(ids)) != len(ids):
                    discovery.update(passed=False, registration_error="missing/duplicate actual ERT IDs")
                    write_json(root / "discovery/result.json", discovery)
                    continue
                registrations[(product["id"], mode, name)] = ids
                for index, identity in enumerate(ids):
                    case_root = root / ("case-%05d" % index)
                    request = dict(operation="ert", fixtures=loaded, id=identity)
                    if identity == "rtn10-format-negative-manifest" and product["id"] in negative_roots:
                        request["negative_source"] = str(negative_roots[product["id"]])
                    row = invoke(product, case_root, request, timeout, mode)
                    row['observer_mode'] = mode
                    response = row.get("response", {})
                    row["passed"] = row["passed"] and response.get("status") == "passed" and response.get("id") == identity
                    rows.append(row)
                    write_json(case_root / "result.json", row)
                write_json(out / "partial.json", dict(schema='femacs-runtime-progress-v1',
                    process_count=len(rows), groups_completed=len(registrations),
                    failed_processes=sum(row['passed'] is False for row in rows),
                    latest=dict(profile=product['id'], mode=mode, group=name), final=False))
    common_registration = []
    for group in ('language', 'xml', 'sqlite'):
        reference_sets = [set(ids) for (profile, mode, name), ids in registrations.items()
                          if name == group and any(p['id'] == profile and p['kind'] == 'reference'
                                                   for p in products)]
        reference = (reference_sets[0] if reference_sets
                     and all(s == reference_sets[0] for s in reference_sets) else None)
        for (profile, mode, name), ids in registrations.items():
            if name == group:
                common_registration.append(dict(profile=profile, observer_mode=mode, group=group,
                    missing=sorted(reference - set(ids)) if reference is not None else None,
                    extra=sorted(set(ids) - reference) if reference is not None else None,
                    passed=reference is not None and set(ids) == reference))
    differences = [row for row in common_registration if row['group'] == 'language']
    language_contract = load_tool('runtime-language-contract.py')
    language_observations, reference_observations, observation_differences = [], {}, []
    for row in rows:
        request = row.get("request", {})
        if request.get("operation") != "ert" or not request.get("id", "").startswith("rtl"):
            continue
        observed = row.get("response", {}).get("observations", {}).get("language")
        identity = request["id"]
        item = dict(profile=row["profile"], observer_mode=row['observer_mode'], id=identity, observations=observed,
                    passed=row["passed"] and isinstance(observed, list) and bool(observed))
        language_observations.append(item)
        if any(p["id"] == row["profile"] and p["kind"] == "reference" for p in products):
            if (identity in reference_observations and not language_contract.compare(
                    identity, reference_observations[identity], observed)['passed']):
                item["passed"] = False
            reference_observations[identity] = observed
    for item in language_observations:
        comparison = language_contract.compare(item['id'],
            reference_observations.get(item['id']), item['observations'])
        observation_differences.append(dict(profile=item["profile"], id=item["id"],
                                           passed=item["passed"] and item["id"] in reference_observations
                                           and comparison['passed'], comparison=comparison))
    language_requested = only_groups is None or 'language' in only_groups
    common_requested = ({'language', 'xml', 'sqlite'} if only_groups is None
                        else set(only_groups) & {'language', 'xml', 'sqlite'})
    common_complete = all(any(row['group'] == group for row in common_registration)
                          for group in common_requested)
    return dict(cases=rows, requested_groups=sorted(only_groups) if only_groups else None,
                registrations=[dict(profile=k[0], observer_mode=k[1], group=k[2], ids=v)
                                         for k, v in registrations.items()],
                language_registration=differences, shared_registration=common_registration,
                language_observations=language_observations,
                language_observation_comparisons=observation_differences,
                passed=bool(rows) and all(row["passed"] for row in rows)
                and common_complete and all(row['passed'] for row in common_registration)
                and (not language_requested or (bool(differences)
                    and all(row["passed"] for row in differences)
                    and bool(observation_differences)
                    and all(row["passed"] for row in observation_differences))),
                compatibility_accepted=False, limitation="additional runtime cases; full stable corpus and native UI remain separate")


def prepare_negative(products, out, timeout):
    plan = bounded_json(FIXTURES / "runtime-native-negative-cases.json")
    rows = []
    for product in products:
        if not product["native"] or product["kind"] == "reference":
            continue
        root = out / product["id"]
        row = invoke(product, root / "abi", dict(operation="abi", fixtures=[]), timeout)
        entry = dict(profile=product["id"], abi_process=row, units=[], passed=False)
        rows.append(entry)
        if not row["passed"]:
            continue
        literal = row["response"].get("abi_literal")
        if not isinstance(literal, str) or not literal.startswith('"') or not literal.endswith('"'):
            entry["error"] = "product did not provide an actual serialized ABI string"
            continue
        source = FIXTURES / "runtime-native-unit.rs"
        abi = root / "abi-literal.txt"
        abi.write_bytes(literal.encode("utf-8"))
        manifest = dict(schema="femacs-runtime-native-negative-v1", units=[])
        for unit in [plan["control"], *plan["units"]]:
            kind = unit["kind"]
            directory = root / ("build-" + kind)
            directory.mkdir()
            library = root / (kind + ".eln")
            env = {key: value for key, value in os.environ.items() if key not in FORBIDDEN_ENV}
            env["FEMACS_RUNTIME_ABI_LITERAL_INPUT"] = str(abi)
            built = run_process(["rustc", "--edition=2024", "--crate-type=cdylib", "-C", "panic=abort",
                                 "-C", "opt-level=2", "--cfg", 'negative="' + kind + '"',
                                 source, "-o", library], directory, env, timeout)
            control = dict(kind=kind, compile=built, source_sha256=digest(source), abi_sha256=digest(abi), passed=False)
            entry["units"].append(control)
            if built["exit_code"] != 0 or not built["complete"] or not library.is_file():
                continue
            signed = run_process(["codesign", "--force", "--sign", "-", library], directory / "sign", env, timeout)
            verified = run_process(["codesign", "--verify", "--strict", library], directory / "verify", env, timeout)
            control.update(sign=signed, verify=verified)
            if signed["exit_code"] != 0 or verified["exit_code"] != 0:
                continue
            item = dict(kind=kind, path=library.name, sha256=digest(library))
            if "expect_error" in unit:
                item["expect_error"] = unit["expect_error"]
                manifest["units"].append(item)
            else:
                manifest["control"] = item
            control.update(passed=True, library=item)
        entry["passed"] = len(entry["units"]) == 1 + len(plan["units"]) and all(unit["passed"] for unit in entry["units"])
        write_json(root / "manifest.json", manifest)
        write_json(root / "preparation.json", entry)
        write_json(out / "partial.json", dict(products=rows, final=False))
    return dict(products=rows, passed=bool(rows) and all(row["passed"] for row in rows),
                limitation="signed format-control construction only; no real Lisp compiler acceptance")


def run_dump(products, out, timeout):
    rows = []
    for product in products:
        if product["kind"] == "reference" or not (product["native"] or product["collector"] != "off"):
            continue
        root = out / product["id"]
        root.mkdir(parents=True)
        env = owned_environment(root, product)
        env["FEMACS_RUNTIME_NATIVE_REQUIRED"] = "1" if product["native"] else "0"
        env["FEMACS_RUNTIME_COLLECTOR_REQUIRED"] = "1" if product["collector"] != "off" else "0"
        before = product_identity(product)
        fixture = FIXTURES / "runtime-dump.el"
        fixture_before = digest(fixture)
        prepare = run_process([product["program"], "--dump-file=" + product["dump"], "-Q", "--batch",
                               "--load", fixture, "--funcall", "femacs-runtime-dump-prepare"], root / "prepare", env, timeout)
        row = dict(profile=product["id"], prepare=prepare, passed=False, source_identity_before=before)
        rows.append(row)
        image = root / "runtime-image.pdmp"
        if prepare["exit_code"] == 0 and prepare["complete"] and image.is_file():
            row["image_sha256"] = digest(image)
            native_unit = root / "dump-native.eln"
            if product["native"]:
                try:
                    row["native_unit_before_restore"] = file_identity(native_unit)
                except (OSError, ValueError) as error:
                    row["native_unit_error"] = str(error)
            # Process2 uses a NEW HOME/cache but the same exact immutable new
            # image and owned image fixture directory. It does not prepare it.
            second_root = root / "second-environment"
            second_root.mkdir()
            second_env = owned_environment(second_root, product)
            second_env.update(FEMACS_RUNTIME_TEST_ROOT=str(root),
                              FEMACS_RUNTIME_NATIVE_REQUIRED=env["FEMACS_RUNTIME_NATIVE_REQUIRED"],
                              FEMACS_RUNTIME_COLLECTOR_REQUIRED=env["FEMACS_RUNTIME_COLLECTOR_REQUIRED"])
            restored = run_process([product["program"], "--dump-file=" + str(image), "-Q", "--batch",
                                    "--load", fixture, "--funcall", "femacs-runtime-dump-assert"],
                                   root / "restore", second_env, timeout)
            row["restore"] = restored
            try:
                row["prepared_report"] = bounded_json(root / "dump-prepared.json")
                row["restored_report"] = bounded_json(root / "dump-restored.json")
                row["passed"] = (restored["exit_code"] == 0 and restored["complete"]
                                 and row["prepared_report"][0:2] == [1, "prepared"]
                                 and row["restored_report"][0:2] == [1, "restored"]
                                 and digest(image) == row["image_sha256"])
            except (OSError, ValueError, TypeError) as error:
                row["report_error"] = str(error)
        if product["native"] and "native_unit_before_restore" in row:
            try:
                row["native_unit_after_restore"] = file_identity(native_unit)
                row["passed"] = (row["passed"] and
                                 row["native_unit_before_restore"] == row["native_unit_after_restore"])
            except (OSError, ValueError) as error:
                row.update(passed=False, native_unit_error=str(error))
        row["source_identity_after"] = product_identity(product)
        row["passed"] = row["passed"] and before == row["source_identity_after"] and digest(fixture) == fixture_before
        write_json(root / "result.json", row)
        write_json(out / "partial.json", dict(products=rows, final=False))
    return dict(products=rows, passed=bool(rows) and all(row["passed"] for row in rows))


def run_negative(products, out, timeout, origins):
    rows = []
    plan = bounded_json(FIXTURES / "runtime-native-negative-cases.json")
    expected = [plan["control"], *plan["units"]]
    for product in products:
        if not product["native"] or product["kind"] == "reference":
            continue
        origin = origins.get(product["id"])
        try:
            if origin is None:
                raise ValueError("missing separately built negative-control directory")
            receipt = bounded_json(origin / "preparation.json", 64 * 1024 * 1024)
            if (receipt.get("passed") is not True
                    or receipt.get("abi_process", {}).get("source_identity_before") != product_identity(product)):
                raise ValueError("format controls are not bound to this actual product")
            manifest = bounded_json(origin / "manifest.json")
            units = {item["kind"]: item for item in [manifest["control"], *manifest["units"]]}
            if len(units) != len(expected) or set(units) != {item["kind"] for item in expected}:
                raise ValueError("incomplete/duplicate format-control identities")
        except (OSError, ValueError, KeyError) as error:
            rows.append(dict(profile=product["id"], passed=False, status="preparation-error", error=str(error)))
            continue
        for unit in expected:
            kind = unit["kind"]
            root = out / product["id"] / kind
            request = dict(operation="native-unit", fixtures=[], negative_source=str(origin),
                           path=str(root / "negative" / units[kind]["path"]))
            try:
                row = invoke(product, root, request, timeout)
                response = row.get("response", {})
                matched = (response.get("loaded") is True if kind == "valid-format" else
                           response.get("loaded") is False and response.get("error_symbol") == unit["expect_error"])
                row.update(kind=kind, expected=unit, passed=row["passed"] and matched)
            except (OSError, ValueError, KeyError) as error:
                row = dict(profile=product["id"], kind=kind, passed=False, status="input-error", error=str(error))
            rows.append(row)
            write_json(out / "partial.json", dict(cases=rows, final=False))
    return dict(cases=rows, passed=bool(rows) and all(row["passed"] for row in rows),
                limitation="format positive/negative controls; real Lisp native lowering is a separate required layer")


def seatbelt_profile(hidden_roots, hidden_files, relocated):
    # These strings are input locators, never shell fragments. Exclude only
    # the owned relocated subtree if it happens to be nested in a denied root.
    # No original tree is renamed, uninstalled or made globally unavailable.
    def quote(value):
        return json.dumps(str(value), ensure_ascii=False)
    lines = ["(version 1)", "(allow default)"]
    for root in sorted(set(Path(value).resolve() for value in hidden_roots)):
        predicate = '(subpath ' + quote(root) + ')'
        if relocated.is_relative_to(root):
            predicate = '(require-all ' + predicate + ' (require-not (subpath ' + quote(relocated) + ')))'
        lines.append('(deny file-read* ' + predicate + ')')
    for file in sorted(set(Path(value).resolve() for value in hidden_files)):
        lines.append('(deny file-read* (literal ' + quote(file) + '))')
    return "\n".join(lines) + "\n"


def sync_record_directory(directory):
    descriptor = os.open(directory, os.O_RDONLY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def sync_case_record(path):
    with Path(path).open('rb') as stream:
        os.fsync(stream.fileno())
    sync_record_directory(Path(path).parent)


def retire_relocation_copy(package, case_root, expected_root, *, evidence_root=None, expected_tree=None):
    """Retire listed owned members only, with durable identities and intents."""
    package, case_root = Path(package), safe_path(case_root)
    evidence_root = safe_path(evidence_root) if evidence_root is not None else case_root
    if package.parent != case_root or package.name not in ("Femacs.app", "terminal"):
        raise ValueError("cleanup is not the exact owned relocation package")
    if not package.exists() and not package.is_symlink():
        return dict(state="no-copy", complete=True, files=0, logical_bytes=0)
    safe_path(package)

    def node(path):
        info = path.lstat()
        return (info.st_dev, info.st_ino, getattr(info, 'st_birthtime', None),
                stat.S_IFMT(info.st_mode), stat.S_IMODE(info.st_mode), info.st_flags)

    root_node = node(package)
    if root_node[:2] != expected_root or root_node[3] != stat.S_IFDIR:
        raise ValueError("relocation package root ownership changed")

    def occupancy(name):
        observed = subprocess.run(["lsof", "+D", str(package)], capture_output=True, text=True)
        write_json(evidence_root/name, dict(returncode=observed.returncode,
                   stdout=observed.stdout, stderr=observed.stderr))
        if observed.returncode != 1 or observed.stdout.strip() or observed.stderr.strip():
            raise ValueError("relocation package has an open or unknown holder")

    def inventory():
        files, nodes, directories = {}, {}, {'.': node(package)}
        member_mtimes, directory_mtimes = {}, {'.': package.lstat().st_mtime_ns}
        for parent, dirs, names in os.walk(package, followlinks=False):
            links = [name for name in dirs if (Path(parent)/name).is_symlink()]
            dirs[:] = [name for name in dirs if name not in links]
            for name in dirs:
                path = Path(parent)/name
                directories[path.relative_to(package).as_posix()] = node(path)
                directory_mtimes[path.relative_to(package).as_posix()] = path.lstat().st_mtime_ns
            for name in sorted(names+links):
                path = Path(parent)/name
                relative = path.relative_to(package).as_posix()
                current = node(path)
                if current[5] & stat.UF_IMMUTABLE:
                    raise ValueError("immutable input unexpectedly appears in owned copy")
                files[relative], nodes[relative] = file_identity(path), current
                member_mtimes[relative] = path.lstat().st_mtime_ns
        return files, nodes, directories, member_mtimes, directory_mtimes

    occupancy('package-copy-occupancy.json')
    observed, member_nodes, directories, member_mtimes, directory_mtimes = inventory()
    expected = {name: dict(value) for name, value in
                (observed if expected_tree is None else expected_tree).items()}
    if observed != expected:
        raise ValueError('relocation copy differs from its planned tree')
    for name in expected:
        relative = Path(name)
        if relative.is_absolute() or '..' in relative.parts or not relative.parts:
            raise ValueError('unsafe planned relocation member')
    manifest = evidence_root/'consumed-package.jsonl.gz'
    logical = allocated = 0
    with manifest.open('xb') as raw:
        with gzip.GzipFile(filename='', mode='wb', fileobj=raw, mtime=0) as stream:
            stream.write((json.dumps(dict(schema='femacs-consumed-relocation-v1', package=package.name))+'\n').encode())
            for name, identity in sorted(expected.items()):
                info = (package/name).lstat()
                stream.write((json.dumps(dict(path=name, identity=identity, allocated_bytes=info.st_blocks*512),
                              ensure_ascii=False, separators=(',', ':'))+'\n').encode())
                logical += identity.get('bytes', 0)
                allocated += info.st_blocks*512
        raw.flush()
        os.fsync(raw.fileno())
    sync_record_directory(manifest.parent)
    private = evidence_root/'package-copy-node-identities.private.json'
    write_json(private, dict(members=member_nodes, directories=directories,
                            member_mtime_ns=member_mtimes, directory_mtime_ns=directory_mtimes))
    sync_case_record(private)
    receipt = dict(state='identity-recorded', complete=False, files=len(expected), logical_bytes=logical,
                   allocated_bytes=allocated, manifest=manifest.name, manifest_sha256=digest(manifest),
                   expected_tree_sha256=hashlib.sha256(json.dumps(expected, sort_keys=True, separators=(',', ':')).encode()).hexdigest(),
                   deleted_files=0, restore='copy the bound original product package; not an inode/xattr restore')
    lifecycle = evidence_root/'package-copy-lifecycle.json'
    write_json(lifecycle, receipt)
    sync_case_record(lifecycle)
    try:
        occupancy('package-copy-occupancy-after-manifest.json')
        current_files, current_nodes, current_dirs, current_mtimes, current_dir_mtimes = inventory()
        if (current_files != expected or current_nodes != member_nodes or current_dirs != directories
                or current_mtimes != member_mtimes or current_dir_mtimes != directory_mtimes):
            raise ValueError('relocation tree changed after its durable manifest')
        with (evidence_root/'package-copy-operations.jsonl').open('x') as operations:
            for name, identity in sorted(expected.items()):
                path = package/name
                operations.write(json.dumps(dict(action='unlink-intent', path=name, identity=identity))+'\n')
                operations.flush()
                os.fsync(operations.fileno())
                if (node(path) != member_nodes[name] or file_identity(path) != identity
                        or path.lstat().st_mtime_ns != member_mtimes[name]):
                    raise ValueError('planned relocation member changed before unlink')
                parent = path.parent.relative_to(package).as_posix()
                if (node(path.parent) != directories[parent] or node(package) != root_node
                        or node(path) != member_nodes[name]):
                    raise ValueError('relocation parent ownership changed before unlink')
                path.unlink()
                receipt['deleted_files'] += 1
                operations.write(json.dumps(dict(action='unlinked', path=name))+'\n')
                operations.flush()
                os.fsync(operations.fileno())
            for name in sorted(directories, key=lambda value: len(Path(value).parts), reverse=True):
                directory = package if name == '.' else package/name
                operations.write(json.dumps(dict(action='rmdir-intent', path=name, node=directories[name]))+'\n')
                operations.flush()
                os.fsync(operations.fileno())
                if node(directory) != directories[name]:
                    raise ValueError('relocation directory ownership changed before rmdir')
                directory.rmdir()
                operations.write(json.dumps(dict(action='rmdir-complete', path=name))+'\n')
                operations.flush()
                os.fsync(operations.fileno())
        sync_record_directory(case_root)
        if package.exists() or package.is_symlink():
            raise ValueError('consumed relocation package remains')
    except BaseException as error:
        receipt.update(state='retained-error', complete=False, error_type=type(error).__name__, error=str(error))
        write_json(lifecycle, receipt)
        sync_case_record(lifecycle)
        raise
    receipt.update(state='retired', complete=True)
    write_json(lifecycle, receipt)
    sync_case_record(lifecycle)
    return receipt


def run_relocation(products, out, timeout):
    sandbox = shutil.which("sandbox-exec")
    rows = []
    for product in products:
        if product["kind"] == "reference":
            continue
        root = out / product["id"]
        root.mkdir(parents=True)
        row = dict(profile=product["id"], passed=False, native_required=product["native"])
        rows.append(row)
        relocated = None
        owned_root = None
        work_case = None
        copied = None
        try:
            if sandbox is None or sys.platform != "darwin":
                raise ValueError("source-denied relocation requires actual Darwin sandbox-exec; no inferred pass")
            build_root = Path(product["product_receipt"]).parent
            inputs = bounded_json(build_root / "runtime-inputs.json")
            source_root = safe_path(inputs["source_root"])
            # Probe files must exist and be readable by the parent first.
            # Child file-error alone could otherwise be an ordinary missing file.
            probes = [source_root / "src/alloc.rs", Path(product["program"]), Path(product["dump"])]
            hidden_roots = [source_root, build_root]
            hidden_files = []
            if product["native"]:
                original = bounded_json(build_root / "backend-sources.json")
                hidden_files = [safe_path(item["source"]) for item in original["libraries"]]
                if not hidden_files:
                    raise ValueError("native relocation has no actual original backend identity")
                prefix = inputs.get("backend_prefix")
                if not prefix:
                    raise ValueError("native relocation has no captured backend prefix")
                hidden_roots.append(safe_path(prefix))
                probes.extend(hidden_files)
            original_cache = product.get("original_cache") or inputs.get("native_cache")
            if original_cache:
                cache = safe_path(original_cache)
                if not cache.is_dir():
                    raise ValueError("original cache locator is not an actual directory")
                hidden_roots.append(cache)
                # Empty cache is still unavailable; expose it explicitly,
                # without inventing a file-read probe for a nonexistent input.
                row["original_cache_identity"] = {"path":str(cache), "files":list(tree_identity(cache))} if any(cache.iterdir()) else {"path":str(cache), "empty":True}
            row["probe_files_before"] = {str(path):file_identity(path) for path in probes}
            source_package = Path(product["program"]).parents[2] if product["kind"] == "cocoa" else Path(product["program"]).parents[1]
            before = tree_identity(source_package)
            # realpath of a moved terminal executable traverses its ancestors.
            # Keep the actual package OUTSIDE the denied source tree instead of
            # weakening the source-denied sandbox to expose ancestor metadata.
            # This sibling belongs to the same explicit frozen-build owner.
            source_owner = safe_path(source_root.parent)
            if (source_root.name != "source" or
                    not (source_owner / "source-inputs.json").is_file() or
                    not (source_root / ".femacs-runtime-sources.json").is_file()):
                raise ValueError("relocation work requires the actual owned frozen export")
            work_case = source_owner / "relocation-work.noindex" / hashlib.sha256(str(out).encode()).hexdigest() / product["id"]
            safe_path(work_case, must_exist=False)
            if any(work_case.is_relative_to(Path(hidden)) for hidden in hidden_roots):
                raise ValueError("owned relocation package would still be inside a denied input root")
            work_case.mkdir(parents=True)
            row["relocation_work_root"] = str(work_case)
            parent_nodes = []
            for directory in (work_case, work_case.parent, work_case.parent.parent):
                metadata = directory.lstat()
                if (directory.resolve() != directory or not stat.S_ISDIR(metadata.st_mode)
                        or getattr(metadata, 'st_flags', 0)):
                    raise ValueError("relocation parent is not an unprotected owned directory")
                parent_nodes.append(dict(path=str(directory), device=metadata.st_dev,
                    inode=metadata.st_ino, mode=stat.S_IMODE(metadata.st_mode),
                    mtime_ns=metadata.st_mtime_ns, birthtime=getattr(metadata, 'st_birthtime', None)))
            parent_record = root/'relocation-parent-identities.private.json'
            write_json(parent_record, dict(schema='femacs-relocation-parents-v1', directories=parent_nodes))
            sync_case_record(parent_record)
            row['parent_directory_retention'] = dict(state='retained-for-owned-cleanup',
                identity_record=parent_record.name, sha256=digest(parent_record))
            relocated = work_case / ("Femacs.app" if product["kind"] == "cocoa" else "terminal")
            relocated.mkdir()
            ownership = relocated.lstat()
            owned_root = (ownership.st_dev, ownership.st_ino)
            shutil.copytree(source_package, relocated, symlinks=True,
                            copy_function=copy_owned_data, dirs_exist_ok=True)
            copied = tree_identity(relocated)
            if before != copied:
                raise ValueError("relocated package does not preserve all original bytes/modes/links")
            # Reject source links rather than allowing them through the one
            # owned-subtree exception in the source-denied sandbox.
            for name, value in copied.items():
                if value["kind"] == "symlink":
                    target = (relocated / name).parent / value["target"]
                    if not target.resolve().is_relative_to(relocated):
                        raise ValueError("relocated package contains an external symlink: " + name)
            harness_root = root / "harness"
            harness_root.mkdir()
            source_fixtures = [HARNESS, FIXTURES / "runtime-language.el", FIXTURES / "runtime-native.el", FIXTURES / "runtime-native-lifecycle.el"]
            for fixture in source_fixtures:
                copy_owned_data(fixture, harness_root / fixture.name)
                if digest(fixture) != digest(harness_root / fixture.name):
                    raise ValueError("copied relocation harness changed")
            profile = root / "source-unavailable.sb"
            # The exception covers the whole owned case subtree, including
            # its harness, policy and isolated compilation/output directories.
            profile.write_text(seatbelt_profile(hidden_roots, hidden_files, root), encoding="utf-8")
            moved = dict(product, _command_prefix=[sandbox, "-f", str(profile)],
                         _harness=str(harness_root / HARNESS.name))
            if product["kind"] == "cocoa":
                moved.update(program=str(relocated / "Contents/MacOS/Femacs"),
                             dump=str(relocated / "Contents/Resources/libexec/Femacs.pdmp"),
                             resource_roots={"bundle":str(relocated / "Contents/Resources")})
            else:
                moved.update(program=str(relocated / "src/emacs"), dump=str(relocated / "src/emacs.pdmp"),
                             resource_roots={name:str(relocated / name) for name in ("lisp", "etc", "info")})
            if 'tls-runtime' in product['resource_roots']:
                if product['kind'] == 'cocoa':
                    moved['resource_roots'].update({
                        'tls-runtime': str(relocated / 'Contents/Frameworks/runtime-tls'),
                        'tls-licenses': str(relocated / 'Contents/Resources/licenses/tls')})
                else:
                    moved['resource_roots'].update({
                        'tls-runtime': str(relocated / 'runtime-tls'),
                        'tls-licenses': str(relocated / 'licenses/tls')})
            if 'lcms-runtime' in product['resource_roots']:
                if product['kind'] == 'cocoa':
                    moved['resource_roots'].update({
                        'lcms-runtime': str(relocated / 'Contents/Frameworks/runtime-lcms'),
                        'lcms-licenses': str(relocated / 'Contents/Resources/licenses/lcms')})
                else:
                    moved['resource_roots'].update({
                        'lcms-runtime': str(relocated / 'runtime-lcms'),
                        'lcms-licenses': str(relocated / 'licenses/lcms')})
            if 'treesit-runtime' in product['resource_roots']:
                if product['kind'] == 'cocoa':
                    moved['resource_roots'].update({
                        'treesit-runtime': str(relocated / 'Contents/Frameworks/runtime-treesit'),
                        'treesit-licenses': str(relocated / 'Contents/Resources/licenses/treesit')})
                else:
                    moved['resource_roots'].update({
                        'treesit-runtime': str(relocated / 'runtime-treesit'),
                        'treesit-licenses': str(relocated / 'licenses/treesit')})
            signature_env = {key:value for key,value in os.environ.items() if key not in FORBIDDEN_ENV}
            signatures = []
            signed_inputs = [relocated if product["kind"] == "cocoa" else Path(moved["program"]), *sorted(relocated.rglob("*.dylib"))]
            for index, signed in enumerate(signed_inputs):
                signatures.append(run_process(["codesign", "--verify", "--deep", "--strict", signed],
                                              root / ("signature-%03d" % index), signature_env, timeout))
            row["signatures"] = signatures
            denial = invoke(moved, root / "denial-probes", dict(operation="paths-unavailable", fixtures=[],
                                                                 paths=[str(path) for path in probes]), timeout)
            row["denial_probes"] = denial
            denied = denial.get("response", {}).get("paths", [])
            if (not denial["passed"] or len(denied) != len(probes)
                    or any(item.get("readable") is not False for item in denied)
                    or [item.get("path") for item in denied] != [str(path) for path in probes]):
                raise ValueError("child has not demonstrated actual denial of all existing original inputs")
            cases = [("rtl01-lexical-closure-and-redefinition", ["runtime-language.el"])]
            if product["native"]:
                cases.extend([("rtn02-real-compile-file-and-native-identity", ["runtime-native.el"]),
                              ("rtn07-native-car-cdr-mutation-and-collector", ["runtime-native.el"])])
            row["cases"] = []
            for identity, fixtures in cases:
                observed = invoke(moved, root / identity, dict(operation="ert", id=identity,
                                                              fixtures=[str(harness_root / name) for name in fixtures]), timeout)
                response = observed.get("response", {})
                observed["passed"] = observed["passed"] and response.get("status") == "passed" and response.get("id") == identity
                row["cases"].append(observed)
            row["original_package_unchanged"] = before == tree_identity(source_package)
            row["relocated_package_unchanged"] = copied == tree_identity(relocated)
            row["probe_files_after"] = {str(path):file_identity(path) for path in probes}
            row["passed"] = (all(item["exit_code"] == 0 and item["complete"] for item in signatures)
                             and all(item["passed"] for item in row["cases"])
                             and row["original_package_unchanged"] and row["relocated_package_unchanged"]
                             and row["probe_files_before"] == row["probe_files_after"])
            row["editing_and_load_scope"] = "batch language/closure and actual native load; physical UI remains separate"
            row["native_compilation_prerequisites"] = "captured developer SDK/assembler/linker remain installed; original source/build/backend paths denied only to the child"
        except (OSError, ValueError, KeyError, TypeError) as error:
            row.update(passed=False, error=str(error))
        finally:
            if relocated is not None and owned_root is not None:
                try:
                    row["package_cleanup"] = retire_relocation_copy(relocated, work_case, owned_root,
                                                                              evidence_root=root, expected_tree=copied)
                except (OSError, ValueError) as error:
                    row.update(passed=False, package_cleanup=dict(state="retained-error", complete=False,
                                                                 error=str(error)))
            write_json(root / "result.json", row)
            write_json(out / "partial.json", dict(products=rows, final=False))
    return dict(products=rows, passed=bool(rows) and all(row["passed"] for row in rows),
                limitation="source-denied relocated batch execution; actual interactive Cocoa/Finder/IME and whole compatibility are separate")


def load_tool(name):
    path = TOOL_ROOT / name
    spec = importlib.util.spec_from_file_location("femacs_" + path.stem.replace("-", "_"), path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def retire_consumed_state(report, out):
    """Run only after the complete action has consumed its isolated case state."""
    tool = load_tool('runtime-retire.py')
    rows, seen, errors, pending = [], set(), [], []
    names = ('home', 'cache', 'config', 'state', 'tmp', 'eln')
    def attempt(item):
        item['attempts'] += 1
        try:
            snapshot = None
            if item.get('snapshot'):
                if digest(item['snapshot']) != item['snapshot_sha256']:
                    raise ValueError('action closure-time state proof changed')
                snapshot = bounded_json(item['snapshot'], 32*1024*1024)
            result = tool.retire(item['root'], names, item['evidence'], item['process'],
                expected_state=snapshot, attempt_index=item['attempts'],
                prior_occupancy_attempts=item['observations'], record_occupancy=True)
            observation = result.get('attempt_observation')
            if observation:
                item['observations'].append(observation)
            if (result.get('status') != 'complete' and not item.get('snapshot')
                    and observation is not None and observation.get('clear') is False):
                proof = tool.snapshot_owned_state(item['root'], names)
                path = item['evidence'].with_name(item['evidence'].stem+'.closure-state.private.json')
                with path.open('x') as stream:
                    json.dump(proof, stream); stream.flush(); os.fsync(stream.fileno())
                item.update(snapshot=path, snapshot_sha256=digest(path))
            item['locked'] = (result.get('state_drift') is True or observation is None
                              or (observation.get('clear') is True and result.get('status') != 'complete'))
        except (OSError, ValueError) as error:
            item['locked'] = True
            result = dict(status='retained', error=f'{type(error).__name__}: {error}',
                          prior_occupancy_attempts=list(item['observations']))
        rows[item['index']] = dict(case=str(item['root'].relative_to(out)), **result)
        return result.get('status') == 'complete'
    def walk(value):
        if isinstance(value, dict):
            if 'evidence_directory' in value and 'pid' in value:
                yield value
            else:
                for child in value.values():
                    yield from walk(child)
        elif isinstance(value, list):
            for child in value:
                yield from walk(child)
    for process in walk(report):
        root = Path(process['evidence_directory'])
        if root in seen or process.get('passed') is not True:
            continue
        seen.add(root)
        if not root.is_relative_to(out) or root.resolve() != root:
            raise ValueError('case retirement outside owned action')
        evidence = root/'result.json'
        if not evidence.is_file():
            evidence = root/'process.json'
        item = dict(root=root, evidence=evidence, process=process, index=len(rows), attempts=0,
                    observations=[], locked=False)
        rows.append(None)
        complete = attempt(item)
        for delay in (0.15, 0.50):
            if complete or item['locked']:
                break
            time.sleep(delay)
            complete = attempt(item)
        if not complete and not item['locked']:
            pending.append(item)
    deadline = time.monotonic()+60.0
    for round_index in range(2):
        if round_index and pending and time.monotonic()+11.0 < deadline:
            time.sleep(1.0)
        for item in list(pending):
            if time.monotonic()+11.0 >= deadline:
                break
            with tool.retirement_deadline(max(0.1, deadline-time.monotonic()-3.0)):
                complete = attempt(item)
            if complete or item['locked']:
                pending.remove(item)
    errors.extend(row['case'] for row in rows if row['status'] != 'complete')
    summary = dict(schema='femacs-runtime-state-retirement-v1', cases=rows, errors=errors,
        complete=not errors, logical_bytes=sum(row.get('logical_bytes',0) for row in rows),
        files=sum(row.get('files',0) for row in rows),
        failed_cases_retained=True, raw_case_evidence_retained=True)
    write_json(out/'state-retirement.json', summary)
    return summary


def write_performance_progress(out, processes, samples, errors, latest):
    # Full rows stay in their individual results and the final detailed report.
    # Rewriting their accumulated phase arrays here would perturb later rounds.
    write_json(out / "partial.json", dict(schema="femacs-runtime-progress-v1",
        process_count=len(processes), sample_count=len(samples), error_count=len(errors),
        latest={key: latest.get(key) for key in
                ("profile", "mode", "observer", "round", "warmup", "passed")},
        final=False))


def run_performance(products, out, timeout, *, permanent_full_fallback=False):
    profile_tool = load_tool("runtime-profile.py")
    accounting = load_tool("runtime-acceptance.py")
    base_fixture = FIXTURES / "runtime-profile.el"
    fixture = FIXTURES / "runtime-profile-fallback.el" if permanent_full_fallback else base_fixture
    runner = FIXTURES / "runtime-performance-runner.el"
    sources = {str(path.relative_to(TOOL_ROOT.parent)): digest(path)
               for path in (base_fixture, fixture, runner)}
    # A hook-enabled diagnostic build is not a performance control. Keep the
    # explicit omission with reason rather than relabelling it uninstrumented.
    excluded = [dict(profile=p["id"], reason="diagnostic test-hooks product") for p in products if p["test_hooks"]]
    controls = []
    for product in products:
        if product["test_hooks"]:
            continue
        modes = (["interpreted", "bytecode", "native"] if product["native"]
                 and not permanent_full_fallback else ["interpreted", "bytecode"])
        observations = (["off", "phases", "survival", "all"] if product["observer"]
                        and not permanent_full_fallback else ["off"])
        for mode in modes:
            for observer in observations:
                controls.append((product, mode, observer))
    processes, samples, startup_samples, errors = [], [], [], []
    for round_index in range(-2, 7):
        warmup = round_index < 0
        label = ("warmup-" + str(round_index + 3)) if warmup else ("round-" + str(round_index + 1))
        shift = (round_index + 2) % max(1, len(controls))
        order = controls[shift:] + controls[:shift]
        for product, mode, observer in order:
            execution_id = product["id"] + ":" + observer
            root = out / product["id"] / observer / mode / label
            root.mkdir(parents=True)
            env = owned_environment(root, product, observer)
            env.update(FEMACS_RUNTIME_EXECUTION_MODE=mode, FEMACS_RUNTIME_PROFILE_FIXTURE=str(fixture))
            before = product_identity(product)
            row = run_process([product["program"], "--dump-file=" + product["dump"], "-Q", "--batch",
                               "--load", runner], root, env, timeout, metrics=True)
            row.update(profile=product["id"], mode=mode, observer=observer, warmup=warmup,
                       round=round_index + 1, execution_identity=execution_id, source_identity_before=before)
            processes.append(row)
            try:
                row["source_identity_after"] = product_identity(product)
                if row["exit_code"] != 0 or not row["complete"] or row["remaining_group_terminated"]:
                    raise ValueError("workload process failed/incomplete/left descendants")
                if before != row["source_identity_after"]:
                    raise ValueError("product executable/dump changed during workload")
                if row.get("memory_sampling_complete") is not True:
                    raise ValueError("incomplete actual RSS/physical-footprint samples")
                output = root / "stdout.txt"
                if output.stat().st_size > 16 * 1024 * 1024:
                    raise ValueError("oversized profile output retained; not parsed as complete")
                observed = profile_tool.parse_profile(output.read_text(encoding="utf-8"))
                metadata = bounded_json(root / "profile-observations.json")
                if metadata.get("schema") != "femacs-runtime-performance-observation-v1" or metadata.get("mode") != mode:
                    raise ValueError("missing/wrong actual execution-mode observations")
                row["runtime_observations"] = metadata
                if permanent_full_fallback:
                    context = bounded_json(root / "fallback-context.json", 128 * 1024)
                    if (context.get("schema") != "femacs-runtime-performance-fallback-v1"
                            or context.get("module_error", {}).get("symbol") != "module-open-failed"
                            or context.get("module_error", {}).get("path_matched") is not True
                            or context.get("module_error", {}).get("message_nonempty") is not True):
                        raise ValueError("actual permanent-fallback input/error was not verified")
                    status = context.get("after")
                    if product["collector"] != "off" and (not isinstance(status, list)
                            or len(status) != 3 or len(status[1]) != 24
                            or status[1][14] != 1 or status[1][1] != 0
                            or status[1][4] != context["latched"][1][4]):
                        raise ValueError("collector did not retain permanent Full fallback")
                    row["fallback_context"] = context
                row["workloads"] = observed
                for measured in observed:
                    if measured.get("execution_mode") != mode or measured.get("actual_function_kind") != mode:
                        raise ValueError("workload did not execute the requested function representation")
                    if not warmup:
                        sample = dict(measured, mode=mode, execution_identity=execution_id,
                                      round=round_index + 1, correctness=True,
                                      peak_rss_bytes=row["peak_rss_bytes"],
                                      peak_footprint_bytes=row["peak_footprint_bytes"],
                                      peak_accounted_bytes=row["peak_accounted_bytes"],
                                      load_per_core=max(row["load_per_core_before"], row["load_per_core_after"]))
                        samples.append(sample)
                row["passed"] = True
            except (OSError, ValueError, KeyError, TypeError) as error:
                row.update(passed=False, error=str(error))
                errors.append(dict(profile=product["id"], mode=mode, observer=observer,
                                   round=round_index + 1, warmup=warmup, error=str(error)))
            write_json(root / "result.json", row)
            write_performance_progress(out, processes, samples, errors, row)
        # Startup is its own fresh-process endpoint, not the compiler/setup
        # duration hidden inside a workload. Retain two warmups + seven rounds.
        # Permanent fallback is a post-startup condition. Do not replay
        # unchanged startup or native-only controls in that explicit mode.
        startup_products = [] if permanent_full_fallback else [p for p in products if not p["test_hooks"]]
        rotate = (round_index + 2) % max(1, len(startup_products))
        for product in startup_products[rotate:] + startup_products[:rotate]:
            root = out / product["id"] / "startup" / label
            root.mkdir(parents=True)
            env = owned_environment(root, product)
            before = product_identity(product)
            row = run_process([product["program"], "--dump-file=" + product["dump"], "-Q", "--batch",
                               "--eval", '(princ "FEMACS-RUNTIME-STARTUP-READY\\n")'],
                              root, env, timeout, metrics=True, observe_exit=True)
            row.update(profile=product["id"], warmup=warmup, round=round_index + 1,
                       mode='startup', observer='off', execution_identity=product['id']+':off',
                       source_identity_before=before)
            try:
                row['source_identity_after'] = product_identity(product)
                if (row['exit_code'] != 0 or not row['complete'] or row['remaining_group_terminated']
                        or before != row['source_identity_after']
                        or row.get('memory_sampling_complete') is not True
                        or row.get('peak_accounted_bytes', 0) <= 0
                        or row.get('exit_timing_complete') is not True
                        or row.get('exit_timing_method') != 'spawn-to-reaped-exit-waiter'
                        or type(row.get('process_lifetime_seconds')) not in (int, float)
                        or not math.isfinite(row['process_lifetime_seconds'])
                        or row['process_lifetime_seconds'] <= 0
                        or (root/'stdout.txt').stat().st_size > 128
                        or (root/'stdout.txt').read_text(encoding='utf-8') != 'FEMACS-RUNTIME-STARTUP-READY\n'):
                    raise ValueError('startup process, exit timing, memory, output or product identity incomplete')
                row['passed'] = True
                if not warmup:
                    samples.append(dict(case='startup', mode='startup',
                        execution_identity=row['execution_identity'], round=round_index+1,
                        wall_ms=row['process_lifetime_seconds']*1000, pauses_ms=[], correctness=True,
                        peak_rss_bytes=row['peak_rss_bytes'], peak_footprint_bytes=row['peak_footprint_bytes'],
                        peak_accounted_bytes=row['peak_accounted_bytes'],
                        load_per_core=max(row['load_per_core_before'],row['load_per_core_after'])))
            except (OSError, ValueError, KeyError, TypeError) as error:
                row.update(passed=False,error=str(error))
                errors.append(dict(profile=product['id'],mode='startup',observer='off',
                    round=round_index+1,warmup=warmup,error=str(error)))
            startup_samples.append(row)
            write_json(root / "result.json", row)
    summary = accounting.summarize_performance(samples)
    comparisons = []
    groups = {(row["execution_identity"], row["mode"], row["case"]): row for row in summary["cases"]}
    references = [p for p in products if p["kind"] == "reference"]
    for reference in references:
        for candidate in products:
            if candidate["kind"] == "reference" or candidate["test_hooks"]:
                continue
            for mode in ("interpreted", "bytecode", "native", "startup"):
                for case in (('startup',) if mode == 'startup' else profile_tool.EXPECTED):
                    reference_key = (reference["id"] + ":off", mode, case)
                    candidate_key = (candidate["id"] + ":off", mode, case)
                    if reference_key not in groups or candidate_key not in groups:
                        continue
                    left = [sample for sample in samples if (sample["execution_identity"], sample["mode"], sample["case"]) == reference_key]
                    right = [sample for sample in samples if (sample["execution_identity"], sample["mode"], sample["case"]) == candidate_key]
                    by_round = {sample["round"]: sample for sample in right}
                    paired = [(sample, by_round[sample["round"]]) for sample in left if sample["round"] in by_round]
                    interval = (accounting.paired_speed_interval([a["wall_ms"] for a, b in paired],
                                                                 [b["wall_ms"] for a, b in paired], seed=0, resamples=10000)
                                if paired else None)
                    comparisons.append(dict(reference=reference["id"], candidate=candidate["id"], mode=mode, case=case,
                                            paired_rounds=len(paired), interval=interval,
                                            eligible=(len(paired) == 7 and groups[reference_key]["quiet"]
                                                      and groups[candidate_key]["quiet"] and not errors)))
    for major in products:
        if major["collector"] != "major":
            continue
        for young in products:
            if (young["collector"] != "young" or major["program"] != young["program"]
                    or major["dump"] != young["dump"] or major["compiler_leg"] != young["compiler_leg"]):
                continue
            for mode in ("interpreted", "bytecode", "native", "startup"):
                for case in (('startup',) if mode == 'startup' else profile_tool.EXPECTED):
                    key1, key2 = (major["id"] + ":off", mode, case), (young["id"] + ":off", mode, case)
                    if key1 not in groups or key2 not in groups:
                        continue
                    left = {row["round"]: row["wall_ms"] for row in samples
                            if (row["execution_identity"], row["mode"], row["case"]) == key1}
                    right = {row["round"]: row["wall_ms"] for row in samples
                             if (row["execution_identity"], row["mode"], row["case"]) == key2}
                    rounds = sorted(set(left) & set(right))
                    interval = (accounting.paired_speed_interval([left[n] for n in rounds], [right[n] for n in rounds],
                                                                 seed=0, resamples=10000) if rounds else None)
                    comparisons.append(dict(kind="same-layout-major-young", major=major["id"], young=young["id"],
                                            case=case, mode=mode, paired_rounds=len(rounds), interval=interval,
                                            eligible=len(rounds) == 7 and groups[key1]["quiet"] and groups[key2]["quiet"] and not errors))
    unchanged = sources == {name: digest(TOOL_ROOT.parent / name) for name in sources}
    return dict(processes=processes, samples=samples, startups=startup_samples, summary=summary,
                comparisons=comparisons, errors=errors, excluded=excluded, fixture_sources=sources,
                permanent_full_fallback=permanent_full_fallback,
                startup_measured=not permanent_full_fallback,
                fixtures_unchanged=unchanged,
                passed=bool(samples) and not errors and unchanged and not summary["failures"]
                and len(processes) == len(controls) * 9 and all(row["passed"] for row in startup_samples)
                and len(startup_samples) == (0 if permanent_full_fallback else
                    len([p for p in products if not p['test_hooks']])*9),
                performance_accepted=False,
                limitation="measurement collection only; busy/inconclusive data, unresolved compatibility and required layout comparisons prevent promotion")


def main(argv=None):
    global REPO
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--action", choices=("freeze-source", "ert", "prepare-negative", "negative", "dump", "performance", "relocation"), required=True)
    parser.add_argument("--products", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--negative-root", type=Path)
    parser.add_argument("--timeout", type=float, default=180)
    parser.add_argument('--repository', type=Path,
                        help='Owned product repository for a separately bound harness snapshot')
    parser.add_argument('--groups', type=str, help='Explicit affected ERT groups; not full acceptance')
    parser.add_argument('--permanent-full-fallback', action='store_true',
                        help='Performance-only real failed-module-load prelude; matched interpreted/bytecode, observer off, no startup replay')
    options = parser.parse_args(argv)
    if options.permanent_full_fallback and options.action != 'performance':
        parser.error('permanent Full fallback applies only to performance measurement')
    if options.repository is not None:
        REPO = safe_path(options.repository)
        if not (REPO/'.git').exists():
            parser.error('explicit product repository must be a Git worktree')
    groups = None
    if options.groups is not None:
        groups = options.groups.split(',')
        if (options.action != 'ert' or not groups or len(set(groups)) != len(groups)
                or set(groups)-{'language', 'xml', 'sqlite', 'native', 'observer', 'observer-fault', 'collector'}):
            parser.error('explicit known ERT groups apply only to affected ERT execution')
    if not math.isfinite(options.timeout) or options.timeout <= 0:
        parser.error("positive finite per-process timeout required")
    if options.action != "freeze-source" and options.products is None:
        parser.error("explicit --products required for product execution")
    products = load_products(options.products) if options.products else []
    out = new_output(options.out)
    fixtures = {str(path): digest(path) for path in sorted(FIXTURES.glob('*.el'))}
    driver_sources = {str(path): digest(path) for path in TOOL_ROOT.glob('*.py')}
    write_json(out / "inputs.json", dict(products=products, action=options.action,
                                         product_list_sha256=digest(options.products) if options.products else None,
                                         driver_sha256=digest(__file__), harness_sha256=digest(HARNESS),
                                         fixture_sources=fixtures, driver_sources=driver_sources,
                                         requested_groups=groups,
                                         permanent_full_fallback=options.permanent_full_fallback))
    try:
        before = {product["id"]: product_identity(product, resources=True) for product in products}
        write_json(out / "product-identities-before.json", before)
        if options.action == "freeze-source":
            report = freeze_source(out)
        elif options.action == "prepare-negative":
            report = prepare_negative(products, out, options.timeout)
        elif options.action == "dump":
            report = run_dump(products, out, options.timeout)
        elif options.action == "performance":
            report = run_performance(products, out, options.timeout,
                                     permanent_full_fallback=options.permanent_full_fallback)
        elif options.action == "relocation":
            report = run_relocation(products, out, options.timeout)
        else:
            negative = {p["id"]: safe_path(options.negative_root / p["id"])
                        for p in products if p["native"] and p["kind"] != "reference"} if options.negative_root else {}
            report = (run_negative(products, out, options.timeout, negative)
                      if options.action == "negative" else run_ert(products, out, options.timeout, negative, groups))
        after = {product["id"]: product_identity(product, resources=True) for product in products}
        write_json(out / "product-identities-after.json", after)
        report["full_product_inputs_unchanged"] = before == after
        report['fixtures_unchanged'] = fixtures == {path: digest(path) for path in fixtures}
        report['driver_sources_unchanged'] = all(digest(Path(path)) == value for path, value in driver_sources.items())
        report["passed"] = (report["passed"] and before == after and report['fixtures_unchanged']
                            and report['driver_sources_unchanged'])
    except BaseException as error:
        write_json(out / "fatal.json", dict(complete=False, passed=False, error=f"{type(error).__name__}: {error}"))
        raise
    report.update(schema="femacs-runtime-execution-v1", action=options.action,
                  goal_complete=False, compatibility_accepted=False, performance_accepted=False)
    retirements = retire_consumed_state(report, out)
    report['state_retirement_complete'] = retirements['complete']
    report['passed'] = report['passed'] and retirements['complete']
    write_json(out / "results.json", report)
    write_completion(out, report)
    print(json.dumps(dict(action=options.action, passed=report["passed"], results=str(out / "results.json"))))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
