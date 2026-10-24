#!/usr/bin/env python3
"""Optional strict reference audit. Run under the external resource controller.

No build or reference checkout is required by normal delivery. This explicitly
selected audit requires two existing programs and preserves every form's evidence.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent.parent
SMOKE = [
    '(list 1 2 3)',
    '(with-temp-buffer (insert "abc") (buffer-string))',
    '(mapcar (function 1+) (list 1 2 3))',
    '(sort (list 3 1 2) (function <))',
    '(string-match "a*b" "aaab")',
    '(replace-regexp-in-string "x+" "-" "axxbxx")',
    '(let ((f (make-temp-file "smoke"))) (with-temp-file f (insert "x")) (with-temp-buffer (insert-file-contents f) (delete-file f) (buffer-string)))',
    '(subrp (symbol-function (quote car)))',
    '(length (garbage-collect))',
    '(coding-system-p (quote utf-8-unix))',
    '(char-width ?\\u00e5)',
]


def new_output(path: Path) -> Path:
    path = Path(path).resolve()
    path.mkdir(parents=True, exist_ok=False)
    return path


def read_forms(path: Path) -> list[str]:
    forms = [line.strip() for line in Path(path).read_text().splitlines()
             if line.strip() and not line.lstrip().startswith(('#', ';'))]
    if not forms:
        raise ValueError('empty differential corpus')
    return forms


def compare(port: subprocess.CompletedProcess, ref: subprocess.CompletedProcess, stderr: bool = False) -> bool:
    return (port.returncode == ref.returncode == 0 and port.stdout == ref.stdout
            and (not stderr or port.stderr == ref.stderr))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--prog', type=Path, default=ROOT / 'build/terminal/port/src/emacs')
    parser.add_argument('--ref', type=Path, default=ROOT / 'build/emacs-ref/src/emacs')
    parser.add_argument('--forms', type=Path, default=ROOT / 'docs/baseline/differential-forms.el')
    parser.add_argument('--smoke', action='store_true')
    parser.add_argument('--stderr', action='store_true')
    parser.add_argument('--keep', action='store_true', help='compatibility flag: evidence is always retained')
    parser.add_argument('--out', type=Path, default=ROOT / 'build/acceptance/differential' / str(time.monotonic_ns()))
    parser.add_argument('--timeout', type=float, default=60)
    parser.add_argument('--memory-mib', type=int, help='deprecated hint; protect the whole audit externally')
    args = parser.parse_args(argv)
    if not 0 < args.timeout < float('inf') or (args.memory_mib is not None and args.memory_mib <= 0):
        parser.error('timeout/memory hint must be finite and positive')
    out = None
    report = dict(passed=False, forms=[])
    try:
        out = new_output(args.out)
        forms = [f'(prin1 {f})' for f in SMOKE] if args.smoke else read_forms(args.forms)
        binaries = dict(port=args.prog.resolve(), ref=args.ref.resolve())
        report['programs'] = {}
        for side, binary in binaries.items():
            if not binary.is_file() or not os.access(binary, os.X_OK):
                raise ValueError(f'missing executable: {binary}')
            digest = hashlib.sha256()
            with binary.open('rb') as source:
                for chunk in iter(lambda: source.read(1024 * 1024), b''):
                    digest.update(chunk)
            report['programs'][side] = dict(path=str(binary), sha256=digest.hexdigest())
        for number, form in enumerate(forms, 1):
            work = out / f'form-{number}'
            work.mkdir()
            fixture = work / 'input.el'
            fixture.write_text(form + '\n')
            home = work / 'home'
            results = {}
            row = dict(number=number, form=form, runs={}, passed=False)
            for side, binary in binaries.items():
                if home.exists():
                    shutil.rmtree(home)  # Only the just-created per-form owned HOME.
                home.mkdir()
                env = dict(HOME=str(home), TMPDIR=str(home), LC_ALL='C', PATH=os.defpath)
                started = time.monotonic()
                stdout = work / f'{side}.stdout'
                stderr = work / f'{side}.stderr'
                with stdout.open('xb') as output, stderr.open('xb') as errors:
                    try:
                        process = subprocess.run([str(binary), '--batch', '-Q', '-l', str(fixture)],
                                                 cwd=ROOT, env=env, stdout=output, stderr=errors, timeout=args.timeout)
                        code = process.returncode
                    except (OSError, subprocess.TimeoutExpired) as e:
                        errors.write(f'process failure: {type(e).__name__}\n'.encode())
                        code = 124 if isinstance(e, subprocess.TimeoutExpired) else 127
                # Bound comparison memory; preserve oversized original output as a failure.
                oversized = max(stdout.stat().st_size, stderr.stat().st_size) > 16 * 1024 * 1024
                result = subprocess.CompletedProcess([str(binary)], code if not oversized else 125,
                                                     stdout.read_bytes() if not oversized else b'',
                                                     stderr.read_bytes() if not oversized else b'')
                results[side] = result
                row['runs'][side] = dict(exit_code=result.returncode, seconds=time.monotonic() - started,
                                         stdout=str(stdout), stderr=str(stderr), oversized=oversized)
            row['passed'] = compare(results['port'], results['ref'], args.stderr)
            report['forms'].append(row)
            print(f'{"ok" if row["passed"] else "MISMATCH"} form {number}')
        report['passed'] = all(r['passed'] for r in report['forms'])
        print(f'differential: {len(forms)} form(s), {sum(not r["passed"] for r in report["forms"])} mismatch(es)')
        return 0 if report['passed'] else 1
    except (OSError, ValueError) as e:
        report['error'] = str(e)
        print(str(e), file=sys.stderr)
        return 2
    finally:
        if out:
            (out / 'results.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    raise SystemExit(main())
