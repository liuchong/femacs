#!/usr/bin/env python3
"""Protective unit contracts for tools/functional-strict-probe.py tool pinning."""
from __future__ import annotations

import importlib.util
import json
import os
from pathlib import Path
import stat
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parent.parent


def load_probe_module():
    spec = importlib.util.spec_from_file_location(
        'femacs_functional_strict_probe', ROOT / 'tools/functional-strict-probe.py')
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


class StrictProbePinTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.probe = load_probe_module()

    def setUp(self) -> None:
        base = ROOT / 'build/strict-probe-unit'
        base.mkdir(parents=True, exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(dir=base)
        self.work = Path(self.temp.name)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def _make_executable(self, name: str, body: str) -> Path:
        target = self.work / name
        target.write_text(body, encoding='utf-8')
        target.chmod(0o755)
        return target

    def test_resolve_and_stage_pinned_tools_records_sha256_and_version(self) -> None:
        fake_clangd = self._make_executable(
            'fake-clangd',
            '#!/bin/sh\nif [ "$1" = "--version" ]; then\n  echo "fake clangd version 99.1.0"\n  exit 0\nfi\nexit 0\n',
        )
        fake_stty = self._make_executable(
            'fake-stty',
            '#!/bin/sh\nif [ "$1" = "--version" ]; then\n  echo "stty: illegal option" >&2\n  exit 1\nfi\nexit 0\n',
        )
        pins = self.probe.resolve_pinned_tools([
            f'clangd={fake_clangd}',
            f'stty={fake_stty}',
        ])
        self.assertEqual(set(pins), {'clangd', 'stty'})
        self.assertEqual(pins['clangd']['version_exit_code'], 0)
        self.assertEqual(pins['clangd']['version_first_line'], 'fake clangd version 99.1.0')
        self.assertEqual(pins['stty']['version_exit_code'], 1)
        self.assertIn('illegal option', pins['stty']['version_first_line'])

        staged_dir = self.work / 'staged'
        effective_path = self.probe.build_pinned_bin(staged_dir, pins, '/usr/bin:/bin')
        self.assertTrue((staged_dir / 'clangd').is_symlink())
        self.assertEqual((staged_dir / 'clangd').resolve(), fake_clangd.resolve())
        self.assertEqual(effective_path, f'{staged_dir}:/usr/bin:/bin')

    def test_rejects_duplicate_missing_directory_or_non_executable_tool(self) -> None:
        tool = self._make_executable('ok-tool', '#!/bin/sh\necho ok\n')
        non_exec = self.work / 'plain.txt'
        non_exec.write_text('not executable\n', encoding='utf-8')
        non_exec.chmod(stat.S_IRUSR | stat.S_IWUSR)
        sub_dir = self.work / 'dir-tool'
        sub_dir.mkdir()

        for invalid in (
            ['missing_equals'],
            [f'bad/name={tool}'],
            [f'tool={self.work / "nonexistent"}'],
            [f'tool={non_exec}'],
            [f'tool={sub_dir}'],
            [f'tool={tool}', f'tool={tool}'],
            [f'tool=relative/path'],
        ):
            with self.subTest(spec=invalid):
                with self.assertRaises(ValueError):
                    self.probe.resolve_pinned_tools(invalid)

    def test_read_server_observations_parses_jsonl(self) -> None:
        log = self.work / 'eglot-servers.jsonl'
        rows = [
            {
                'test_id': 'eglot-test-tramp-test',
                'remote_method': 'mock',
                'process_command': ['/bin/sh', '-i'],
                'running_command': '/bin/sh -c stty raw > /dev/null; /usr/bin/clangd',
                'server_info': {'name': 'clangd', 'version': 'Apple clangd version 21.0.0'},
            }
        ]
        log.write_text(''.join(json.dumps(r) + '\n' for r in rows), encoding='utf-8')
        parsed = self.probe.read_server_observations(log)
        self.assertEqual(len(parsed), 1)
        self.assertEqual(parsed[0]['remote_method'], 'mock')
        self.assertEqual(parsed[0]['server_info']['name'], 'clangd')


if __name__ == '__main__':
    unittest.main()
