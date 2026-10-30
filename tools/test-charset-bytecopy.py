"""Protective component tests for P-438 charset byte-copy generator recipes.

Reads the 10 real byte-copy recipes from build-support/generator-recipes.json,
places fixed known-byte inputs in a small isolated directory, sets macOS
UF_IMMUTABLE on the test's own raw inputs, executes the real recipe commands
twice, and verifies output bytes, non-propagation of UF_IMMUTABLE, and unchanged
input content/permissions/flags before restoring only this test's flags in
finally. Also verifies missing inputs fail with a non-zero exit code.
"""
from __future__ import annotations

import argparse
import ctypes
import hashlib
import io
import json
import os
from pathlib import Path
import shutil
import stat
import subprocess
import sys
import tempfile
import time
import unittest

REPO = Path(__file__).resolve().parent.parent
RECIPES_PATH = REPO / 'build-support/generator-recipes.json'

EXPECTED_BYTECOPY_PAIRS = (
    ('admin/charsets/mapfiles/CP720.map', 'etc/charsets/CP720.map'),
    ('admin/charsets/mapfiles/CP858.map', 'etc/charsets/CP858.map'),
    ('admin/charsets/mapfiles/JISX213A.map', 'etc/charsets/JISX213A.map'),
    ('admin/charsets/mapfiles/MULE-ethiopic.map', 'etc/charsets/MULE-ethiopic.map'),
    ('admin/charsets/mapfiles/MULE-ipa.map', 'etc/charsets/MULE-ipa.map'),
    ('admin/charsets/mapfiles/MULE-is13194.map', 'etc/charsets/MULE-is13194.map'),
    ('admin/charsets/mapfiles/MULE-lviscii.map', 'etc/charsets/MULE-lviscii.map'),
    ('admin/charsets/mapfiles/MULE-sisheng.map', 'etc/charsets/MULE-sisheng.map'),
    ('admin/charsets/mapfiles/MULE-tibetan.map', 'etc/charsets/MULE-tibetan.map'),
    ('admin/charsets/mapfiles/MULE-uviscii.map', 'etc/charsets/MULE-uviscii.map'),
)

def digest_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def digest_file(path: Path) -> str:
    return digest_bytes(Path(path).read_bytes())


def set_fd_flags(descriptor: int, flags: int) -> None:
    function = ctypes.CDLL(None, use_errno=True).fchflags
    function.argtypes = (ctypes.c_int, ctypes.c_uint32)
    function.restype = ctypes.c_int
    if function(descriptor, flags) != 0:
        error = ctypes.get_errno()
        raise OSError(error, os.strerror(error))


def load_bytecopy_recipes(path: Path = RECIPES_PATH) -> list[dict[str, str]]:
    document = json.loads(Path(path).read_text(encoding='utf-8'))
    charsets = document.get('charsets')
    if not isinstance(charsets, list) or not charsets:
        raise ValueError('generator-recipes.json must define a non-empty charsets list')
    selected: list[dict[str, str]] = []
    expected_outputs = {output: source for source, output in EXPECTED_BYTECOPY_PAIRS}
    for row in charsets:
        if not isinstance(row, dict):
            raise ValueError('charset recipe entry must be an object')
        output = row.get('output', '')
        shell = row.get('shell', '')
        if output in expected_outputs:
            if not isinstance(shell, str) or not shell.strip():
                raise ValueError(f'missing executable recipe for {output}')
            # Protect the observable byte/metadata contract, not today's choice
            # of shell utility or redirection spelling.
            selected.append({'source': expected_outputs[output],
                             'output': output, 'shell': shell})
    actual_pairs = tuple((row['source'], row['output']) for row in selected)
    if actual_pairs != EXPECTED_BYTECOPY_PAIRS:
        raise ValueError(
            f'unexpected byte-copy recipe set: got {actual_pairs}, expected {EXPECTED_BYTECOPY_PAIRS}'
        )
    return selected


def clean_recipe_env() -> dict[str, str]:
    env = dict(os.environ)
    env['LC_ALL'] = 'C'
    env.pop('EMACSDATA', None)
    env.pop('EMACSDOC', None)
    return env


class CharsetBytecopyProtectionTests(unittest.TestCase):
    def test_immutable_inputs_twice_without_flag_propagation(self) -> None:
        if (
            sys.platform != 'darwin'
            or not hasattr(os, 'chflags')
            or not hasattr(stat, 'UF_IMMUTABLE')
        ):
            self.fail(
                'macOS UF_IMMUTABLE support is required; non-macOS must not claim immutable coverage'
            )
        recipes = load_bytecopy_recipes(RECIPES_PATH)
        self.assertEqual(len(recipes), 10)
        work = Path(tempfile.mkdtemp(prefix='femacs-charset-bytecopy-imm-'))
        held: list[tuple[Path, tuple[int, int], int, bytes, str, int]] = []
        try:
            for index, row in enumerate(recipes):
                src = work / row['source']
                dst = work / row['output']
                src.parent.mkdir(parents=True, exist_ok=True)
                dst.parent.mkdir(parents=True, exist_ok=True)
                payload = (
                    f'# fixed known byte-copy fixture {index}: {row["source"]}\n'
                    f'0x00\t0x0000\n0x{index + 1:02x}\t0x00{index + 1:02x}\n'
                ).encode('utf-8')
                mode = 0o644 if index % 2 == 0 else 0o640
                src.write_bytes(payload)
                src.chmod(mode)
                fd = os.open(src, os.O_RDONLY | os.O_NOFOLLOW)
                try:
                    before = os.fstat(fd)
                    node = (before.st_dev, before.st_ino)
                    orig_flags = before.st_flags
                    self.assertEqual(orig_flags & stat.UF_IMMUTABLE, 0)
                    held.append((src, node, orig_flags, payload, digest_bytes(payload), mode))
                    set_fd_flags(fd, orig_flags | stat.UF_IMMUTABLE)
                    after_set = os.fstat(fd)
                    self.assertNotEqual(after_set.st_flags & stat.UF_IMMUTABLE, 0)
                finally:
                    os.close(fd)
                self.assertFalse(dst.exists())

            env = clean_recipe_env()
            for pass_number in (1, 2):
                for row, (src, node, orig_flags, payload, payload_sha, mode) in zip(
                    recipes, held
                ):
                    dst = work / row['output']
                    proc = subprocess.run(
                        ['bash', '-o', 'pipefail', '-c', row['shell']],
                        cwd=work,
                        env=env,
                        capture_output=True,
                        text=True,
                        timeout=10,
                        check=False,
                    )
                    self.assertEqual(
                        proc.returncode,
                        0,
                        f'pass {pass_number} failed for {row["output"]}: {proc.stderr}',
                    )
                    self.assertEqual(proc.stdout, '')
                    self.assertEqual(proc.stderr, '')
                    self.assertTrue(dst.is_file() and not dst.is_symlink())
                    dst_stat = dst.lstat()
                    self.assertEqual(
                        dst_stat.st_flags & stat.UF_IMMUTABLE,
                        0,
                        f'pass {pass_number} propagated UF_IMMUTABLE to {row["output"]}',
                    )
                    self.assertEqual(dst.read_bytes(), payload)
                    self.assertEqual(digest_file(dst), payload_sha)

                    src_stat = src.lstat()
                    self.assertEqual((src_stat.st_dev, src_stat.st_ino), node)
                    self.assertEqual(stat.S_IMODE(src_stat.st_mode), mode)
                    self.assertEqual(
                        src_stat.st_flags,
                        orig_flags | stat.UF_IMMUTABLE,
                        f'pass {pass_number} altered input flags on {row["source"]}',
                    )
                    self.assertEqual(src.read_bytes(), payload)
                    self.assertEqual(digest_file(src), payload_sha)
        finally:
            try:
                for src, node, orig_flags, payload, payload_sha, mode in reversed(held):
                    if not src.exists() and not src.is_symlink():
                        continue
                    fd = os.open(src, os.O_RDONLY | os.O_NOFOLLOW)
                    try:
                        current = os.fstat(fd)
                        if (current.st_dev, current.st_ino) != node:
                            raise RuntimeError(f'refuse to clear flags on replaced inode: {src}')
                        set_fd_flags(fd, orig_flags)
                        restored = os.fstat(fd)
                        if restored.st_flags != orig_flags:
                            raise RuntimeError(f'failed to restore original flags on {src}')
                    finally:
                        os.close(fd)
            finally:
                shutil.rmtree(work, ignore_errors=False)

    def test_missing_input_fails_without_swallowing_error(self) -> None:
        recipes = load_bytecopy_recipes(RECIPES_PATH)
        self.assertEqual(len(recipes), 10)
        work = Path(tempfile.mkdtemp(prefix='femacs-charset-bytecopy-missing-'))
        try:
            (work / 'admin/charsets/mapfiles').mkdir(parents=True, exist_ok=True)
            (work / 'etc/charsets').mkdir(parents=True, exist_ok=True)
            env = clean_recipe_env()
            for row in recipes:
                src = work / row['source']
                self.assertFalse(src.exists())
                proc = subprocess.run(
                    ['bash', '-o', 'pipefail', '-c', row['shell']],
                    cwd=work,
                    env=env,
                    capture_output=True,
                    text=True,
                    timeout=10,
                    check=False,
                )
                self.assertNotEqual(
                    proc.returncode,
                    0,
                    f'missing input unexpectedly succeeded for {row["output"]}',
                )
                self.assertTrue(
                    proc.stderr.strip(),
                    f'missing input produced empty stderr for {row["output"]}',
                )
                self.assertIn(row['source'], proc.stderr)
        finally:
            shutil.rmtree(work, ignore_errors=False)


class TimedResult(unittest.TextTestResult):
    def startTest(self, test: unittest.case.TestCase) -> None:
        self._started = time.monotonic()
        super().startTest(test)

    def stopTest(self, test: unittest.case.TestCase) -> None:
        elapsed = time.monotonic() - self._started
        failed_ids = {case.id() for case, _ in self.failures + self.errors + self.skipped}
        self.timings.append(
            {
                'id': test.id(),
                'seconds': elapsed,
                'passed': test.id() not in failed_ids,
            }
        )
        super().stopTest(test)


def main() -> int:
    global RECIPES_PATH
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, help='Optional fresh directory for test receipts')
    parser.add_argument('--recipes', type=Path, default=RECIPES_PATH)
    args = parser.parse_args()
    RECIPES_PATH = args.recipes.resolve()
    out = None
    if args.out is not None:
        out = args.out.absolute()
        if out.exists() or out.resolve() != out:
            parser.error('fresh owned --out directory required')
        out.mkdir(parents=True)

    recipes_before = digest_file(RECIPES_PATH)
    stream = io.StringIO()
    TimedResult.timings = []
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(CharsetBytecopyProtectionTests)
    result = unittest.TextTestRunner(
        stream=stream, verbosity=2, resultclass=TimedResult
    ).run(suite)
    recipes_after = digest_file(RECIPES_PATH)
    passed = (
        result.wasSuccessful()
        and len(result.skipped) == 0
        and result.testsRun == 2
        and recipes_before == recipes_after
    )
    summary = {
        'passed': passed,
        'tests': result.testsRun,
        'platform': sys.platform,
        'darwin_immutable_covered': sys.platform == 'darwin' and passed,
        'recipes_sha256': recipes_before,
        'recipes_unchanged': recipes_before == recipes_after,
        'test_sha256': digest_file(Path(__file__).resolve()),
        'cases': TimedResult.timings,
        'failures': [test.id() for test, _ in result.failures],
        'errors': [test.id() for test, _ in result.errors],
        'skipped': [test.id() for test, _ in result.skipped],
        'editor_accepted': False,
    }
    if out is not None:
        (out / 'unittest.txt').write_text(stream.getvalue(), encoding='utf-8')
        (out / 'results.json').write_text(
            json.dumps(summary, indent=2, ensure_ascii=False) + '\n',
            encoding='utf-8',
        )
    print(stream.getvalue(), end='')
    print(json.dumps(summary, ensure_ascii=False))
    return 0 if passed else 1


if __name__ == '__main__':
    raise SystemExit(main())
