"""Actual owned-filesystem integration for planned retirement and late changes.

No compiler, editor, or reference tree is used. Run after implementation; the
output and retained negative controls are later retired with a cleanup report.
"""
from pathlib import Path
import argparse
import importlib.util
import json
import os
import shutil
import sys
import time
from unittest.mock import patch

ROOT = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('actual_retirement', ROOT/'tools/runtime-execution.py')
m = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = m
spec.loader.exec_module(m)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    options = parser.parse_args()
    out = m.new_output(options.out)
    rows = []

    def case(name, action):
        directory = out/name
        directory.mkdir()
        package, evidence = directory/'terminal', directory/'evidence'
        package.mkdir()
        evidence.mkdir()
        (package/'a.txt').write_bytes(b'planned a')
        (package/'nested').mkdir()
        (package/'nested/b.txt').write_bytes(b'planned b')
        (package/'nested/b.txt').chmod(0o640)
        (package/'z-alias').symlink_to('nested/b.txt')
        expected = m.tree_identity(package)
        info = package.stat()
        started = time.monotonic()
        try:
            details = action(package, evidence, (info.st_dev, info.st_ino), expected)
            result = dict(id=name, passed=True, details=details)
        except BaseException as error:
            result = dict(id=name, passed=False, error_type=type(error).__name__, error=str(error))
        result['seconds'] = time.monotonic()-started
        rows.append(result)
        m.write_json(directory/'result.json', result)
        print(json.dumps(result), flush=True)

    def retire(package, evidence, identity, expected):
        return m.retire_relocation_copy(package, package.parent, identity,
                                         evidence_root=evidence, expected_tree=expected)

    def rejected(action, phrase):
        try:
            action()
        except (OSError, ValueError) as error:
            if phrase not in str(error):
                raise
            return
        raise AssertionError('unsafe retirement was accepted')

    def positive(package, evidence, identity, expected):
        original = package.parent/'input'
        shutil.copytree(package, original, symlinks=True)
        receipt = retire(package, evidence, identity, expected)
        assert receipt['complete'] and receipt['deleted_files'] == len(expected) and not package.exists()
        restored = package.parent/'restored'
        shutil.copytree(original, restored, symlinks=True)
        assert m.tree_identity(restored) == expected
        return dict(files=receipt['files'], manifest_sha256=receipt['manifest_sha256'], restoration_verified=True)

    def wrong_plan(package, evidence, identity, expected):
        (package/'a.txt').write_bytes(b'changed before entry')
        rejected(lambda: retire(package, evidence, identity, expected), 'planned tree')
        assert (package/'a.txt').exists() and not (evidence/'consumed-package.jsonl.gz').exists()
        return dict(copy_retained=True)

    def after_manifest(package, evidence, identity, expected, mode):
        actual = m.sync_case_record
        injected = []
        held = []
        def boundary(path):
            actual(path)
            if Path(path).name == 'package-copy-lifecycle.json' and not injected:
                injected.append(True)
                if mode == 'change':
                    (package/'a.txt').write_bytes(b'changed after durable manifest')
                elif mode == 'new':
                    (package/'unlisted.txt').write_bytes(b'never remove an unlisted file')
                else:
                    held.append((package/'a.txt').open('rb'))
        try:
            with patch.object(m, 'sync_case_record', side_effect=boundary):
                rejected(lambda: retire(package, evidence, identity, expected),
                         'holder' if mode == 'holder' else 'after its durable manifest')
        finally:
            for handle in held:
                handle.close()
        receipt = json.loads((evidence/'package-copy-lifecycle.json').read_text())
        assert injected and not receipt['complete'] and receipt['deleted_files'] == 0
        assert all((package/name).exists() or (package/name).is_symlink() for name in expected)
        return dict(copy_retained=True, post_manifest_boundary_checked=True, mode=mode)

    def held_at_entry(package, evidence, identity, expected):
        with (package/'a.txt').open('rb'):
            rejected(lambda: retire(package, evidence, identity, expected), 'holder')
        assert m.tree_identity(package) == expected
        return dict(real_open_descriptor_detected=True)

    def changed_before_unlink(package, evidence, identity, expected):
        actual = os.fsync
        injected = []
        def boundary(descriptor):
            actual(descriptor)
            target = evidence/'package-copy-operations.jsonl'
            if target.exists() and not injected:
                a, b = os.fstat(descriptor), target.stat()
                if (a.st_dev, a.st_ino) == (b.st_dev, b.st_ino):
                    injected.append(True)
                    (package/'a.txt').write_bytes(b'changed at durable unlink intent')
        with patch.object(m.os, 'fsync', side_effect=boundary):
            rejected(lambda: retire(package, evidence, identity, expected), 'before unlink')
        receipt = json.loads((evidence/'package-copy-lifecycle.json').read_text())
        assert injected and receipt['deleted_files'] == 0 and (package/'a.txt').exists()
        return dict(changed_member_retained=True)

    def late_unlisted(package, evidence, identity, expected):
        actual = Path.unlink
        injected = []
        def boundary(path, *args, **kwargs):
            result = actual(path, *args, **kwargs)
            if path.is_relative_to(package) and not injected:
                injected.append(True)
                (package/'late-unlisted.txt').write_bytes(b'preserve this unplanned member')
            return result
        with patch.object(Path, 'unlink', boundary):
            try:
                retire(package, evidence, identity, expected)
            except OSError:
                pass
            else:
                raise AssertionError('unexpected late member was swept')
        receipt = json.loads((evidence/'package-copy-lifecycle.json').read_text())
        assert injected and not receipt['complete'] and receipt['deleted_files'] == len(expected)
        assert (package/'late-unlisted.txt').read_bytes() == b'preserve this unplanned member'
        return dict(unlisted_member_retained=True, partial_outcome_durable=True)

    case('complete-and-restore', positive)
    case('planned-tree-mismatch', wrong_plan)
    case('member-change-after-manifest', lambda *args: after_manifest(*args, 'change'))
    case('new-member-after-manifest', lambda *args: after_manifest(*args, 'new'))
    case('real-holder-at-entry', held_at_entry)
    case('real-holder-after-manifest', lambda *args: after_manifest(*args, 'holder'))
    case('member-change-at-unlink-intent', changed_before_unlink)
    case('late-unlisted-member-retained', late_unlisted)
    result = dict(scope='actual planned-retirement filesystem integration only', cases=rows,
                  passed=all(row['passed'] for row in rows), product_tests_run=False)
    m.write_json(out/'results.json', result)
    return 0 if result['passed'] else 1

if __name__ == '__main__':
    raise SystemExit(main())
