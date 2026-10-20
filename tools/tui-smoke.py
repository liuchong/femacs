#!/usr/bin/env python3
"""Drive the reference or the port in a terminal on a real pty and check that the frame is drawn.

Every other check in this repository runs `--batch`, which never builds a terminal frame.  This
one forks a pty (so the child has a controlling terminal, which `-nw` needs), waits for the first
frame, and reports whether the mode line is on it:

    python3 tools/tui-smoke.py build/port/src/emacs
    python3 tools/tui-smoke.py build/emacs-ref/src/emacs --rows 10

Exit 0 when the mode line (`*scratch*` and the major mode) reached the screen, 1 when it did not,
2 when the program could not be started.  `--keep` prints the captured stream.
"""
import argparse
import fcntl
import os
import pty
import re
import select
import struct
import sys
import termios
import time

ANSI = re.compile(r'\x1b\[[0-9;?]*[A-Za-z]|\x1b[()][A-Za-z0-9]|\x1b[=>]')


def run(prog, rows, cols, wait):
    pid, fd = pty.fork()
    if pid == 0:
        os.environ.update(TERM='xterm-256color', HOME='/tmp/emacs-tui-home', LC_ALL='C')
        try:
            os.execv(prog, [prog, '-nw', '-Q'])
        finally:
            os._exit(127)
    fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack('HHHH', rows, cols, 0, 0))
    buf = b''
    end = time.time() + wait
    while time.time() < end:
        readable, _, _ = select.select([fd], [], [], 0.2)
        if not readable:
            continue
        try:
            chunk = os.read(fd, 65536)
        except OSError:
            break
        if not chunk:
            break
        buf += chunk
    os.write(fd, b'\x18\x03')          # C-x C-c
    time.sleep(0.5)
    try:
        os.waitpid(pid, os.WNOHANG)
    except ChildProcessError:
        pass
    os.close(fd)
    return buf


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('program')
    ap.add_argument('--rows', type=int, default=24)
    ap.add_argument('--cols', type=int, default=80)
    ap.add_argument('--wait', type=float, default=6.0)
    ap.add_argument('--keep', action='store_true')
    args = ap.parse_args()
    os.makedirs('/tmp/emacs-tui-home', exist_ok=True)
    if not os.access(args.program, os.X_OK):
        print(f'tui-smoke: not executable: {args.program}', file=sys.stderr)
        return 2
    raw = run(args.program, args.rows, args.cols, args.wait)
    text = ANSI.sub('', raw.decode('utf-8', 'replace'))
    has_mode_line = '*scratch*' in text and 'Lisp Interaction' in text
    if args.keep:
        print(text)
    print(f'tui-smoke: {args.rows}x{args.cols}, {len(raw)} byte(s), '
          f'mode line {"present" if has_mode_line else "MISSING"}')
    return 0 if has_mode_line else 1


if __name__ == '__main__':
    sys.exit(main())
