"""Real timed input/timer regression with closed stdin; use an external runtime controller."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import subprocess
import time

EXPRESSION = '''(let ((batch-event-timer-fired nil) timer)
  (unwind-protect
      (progn
        (setq timer (run-at-time 0.01 nil
                      (lambda () (setq batch-event-timer-fired t))))
        (when (read-event nil nil 0.05)
          (error "Unexpected input from closed stdin"))
        (unless batch-event-timer-fired
          (error "Timer did not fire during timed input"))
        (prin1 '(batch-timeout timer-fired)))
    (when timer (cancel-timer timer))))'''


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--program', type=Path, required=True)
    parser.add_argument('--dump', type=Path)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    program = args.program.resolve(strict=True)
    dump = args.dump.resolve(strict=True) if args.dump else None
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    inputs = [program, *([dump] if dump else [])]
    before = {str(path): digest(path) for path in inputs}
    env = dict(os.environ)
    for variable, name in [('HOME', 'home'), ('TMPDIR', 'tmp'),
                           ('XDG_CACHE_HOME', 'cache')]:
        directory = out / name
        directory.mkdir()
        env[variable] = str(directory)
    for variable in ('EMACSLOADPATH', 'EMACSDATA', 'EMACSDOC'):
        env.pop(variable, None)
    command = [str(program), *(['--dump-file', str(dump)] if dump else []),
               '-Q', '--batch', '--eval', EXPRESSION]
    started = time.monotonic()
    timed_out = False
    exit_code = None
    with (out / 'stdout.txt').open('wb') as stdout, (out / 'stderr.txt').open('wb') as stderr:
        try:
            result = subprocess.run(command, stdin=subprocess.DEVNULL,
                                    stdout=stdout, stderr=stderr, env=env,
                                    cwd=out, timeout=30)
            exit_code = result.returncode
        except subprocess.TimeoutExpired:
            timed_out = True
    seconds = time.monotonic() - started
    unchanged = before == {str(path): digest(path) for path in inputs}
    output = (out / 'stdout.txt').read_text(errors='replace').strip()
    passed = (exit_code == 0 and not timed_out and unchanged
              and output == '(batch-timeout timer-fired)')
    report = dict(scope='editor timed-input/timer integration; not UI or full compatibility',
                  passed=passed, stdin='closed', inputs=before,
                  inputs_unchanged=unchanged, exit_code=exit_code,
                  timed_out=timed_out, cases=[dict(id='closed-stdin-timer',
                                                passed=passed, seconds=seconds)])
    (out / 'results.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(dict(passed=passed, exit_code=exit_code, seconds=seconds)))
    return 0 if passed else 1


if __name__ == '__main__':
    raise SystemExit(main())
