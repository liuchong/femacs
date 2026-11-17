#!/usr/bin/env python3
"""Run the committed PNG/JPEG protection fixture against one graphical product.

The original image test files skip every decode when no window-system frame is
available, so this lane starts the product as a real Cocoa application and runs
`tools/fixtures/image-loader-protection.el' in its first frame.  The fixture
covers the file and memory arms of both readers, damaged/truncated input through
the loaders' error recovery, and that both readers still work afterwards.

The evidence written to `--out' names every input by sha256, including the
product, its dump, the fixture and the repository root, so the run can be
reproduced without a scratch tree.  Nothing is written inside the repository.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent


def digest(path: Path) -> str:
    hasher = hashlib.sha256()
    with path.open('rb') as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b''):
            hasher.update(block)
    return hasher.hexdigest()


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--program', type=Path, required=True)
    parser.add_argument('--dump', type=Path, required=True)
    parser.add_argument('--root', type=Path, default=REPO)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--label', required=True)
    parser.add_argument('--timeout', type=float, default=300)
    args = parser.parse_args(argv)

    program = args.program.resolve()
    dump = args.dump.resolve()
    root = args.root.resolve()
    fixture = REPO / 'tools/fixtures/image-loader-protection.el'
    for path in (program, dump, fixture):
        if not path.is_file():
            parser.error('missing probe input: %s' % path)
    for name in ('lisp', 'etc'):
        if not (root / name).is_dir():
            parser.error('probe root has no %s directory: %s' % (name, root))

    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    result = out / 'result.txt'
    stdout = out / 'stdout.log'
    stderr = out / 'stderr.log'
    env = {key: value for key, value in os.environ.items() if not key.startswith('FEMACS_')}
    env.update(EMACSLOADPATH=str(root / 'lisp'), EMACSDATA=str(root / 'etc'),
               IMAGE_LOADER_ROOT=str(root), IMAGE_LOADER_RESULT=str(result),
               LC_ALL='C', LANG='C')
    argv_product = [str(program), '-Q', '-l', str(fixture)]
    started = time.time()
    timed_out = False
    exit_code = None
    with stdout.open('xb') as out_stream, stderr.open('xb') as err_stream:
        try:
            proc = subprocess.run(argv_product, env=env, stdin=subprocess.DEVNULL,
                                  stdout=out_stream, stderr=err_stream,
                                  timeout=args.timeout)
            exit_code = proc.returncode
        except subprocess.TimeoutExpired:
            timed_out = True
    elapsed = time.time() - started
    text = result.read_text(errors='replace') if result.is_file() else ''
    report = dict(
        schema='femacs-image-loader-protection-v1', label=args.label,
        complete=(exit_code == 0 and not timed_out and 'failed 0' in text),
        exit_code=exit_code, timed_out=timed_out, elapsed_seconds=round(elapsed, 3),
        program=dict(path=str(program), sha256=digest(program)),
        dump=dict(path=str(dump), sha256=digest(dump)),
        fixture=dict(path=str(fixture), sha256=digest(fixture)),
        root=dict(path=str(root), lisp=digest(root / 'lisp/loadup.el') if (root / 'lisp/loadup.el').is_file() else None),
        command=argv_product,
        environment={key: env[key] for key in ('EMACSLOADPATH', 'EMACSDATA', 'IMAGE_LOADER_ROOT')},
        result=text,
        logs=dict(stdout='stdout.log', stderr='stderr.log'))
    (out / 'results.json').write_text(json.dumps(report, indent=1) + '\n')
    print(json.dumps(dict(complete=report['complete'], exit_code=exit_code,
                          timed_out=timed_out)))
    return 0 if report['complete'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
