"""Post-integration client protocol protection with an isolated fake server.

Runs an actual supplied client, not an editor/server compatibility test. The
server supplies byte streams to protect line consumption, buffer growth,
legacy replies and error status. Run under the normal resource controller.
"""
from pathlib import Path
import argparse
import hashlib
import json
import os
import socket
import stat
import subprocess
import time
import uuid

REPO = Path(__file__).resolve().parent.parent


def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024*1024), b''):
            value.update(block)
    return value.hexdigest()


def run_case(client, name, chunks, expected, code, out):
    started = time.monotonic()
    directory = out/name
    directory.mkdir()
    socket_root = REPO/'.femacs-cache/client-protocol'/uuid.uuid4().hex[:8]
    socket_root.mkdir(parents=True, mode=0o700)
    address = socket_root/'s'
    if len(os.fsencode(address)) >= 104:
        raise ValueError('owned Unix socket path exceeds supported bound')
    socket_identity = None
    root_metadata = socket_root.lstat()
    process = None
    error = None
    stdout = stderr = b''
    result_code = None
    request = b''
    try:
        with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as listener:
            listener.bind(str(address))
            entry = address.lstat()
            socket_identity = (entry.st_dev, entry.st_ino, stat.S_IFMT(entry.st_mode))
            listener.listen(1)
            listener.settimeout(5)
            environment = {key:value for key,value in os.environ.items()
                           if not key.startswith(('FEMACS_', 'EMACS', 'ALTERNATE_EDITOR'))}
            environment.update(HOME=str(directory), TMPDIR=str(directory), LC_ALL='C')
            process = subprocess.Popen([str(client), '--socket-name', str(address),
                                       '--eval', '(+ 19 23)'], cwd=directory,
                                       env=environment, stdin=subprocess.DEVNULL,
                                       stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            with listener.accept()[0] as connection:
                connection.settimeout(5)
                while b'\n' not in request:
                    block = connection.recv(4096)
                    if not block or len(request)+len(block) > 65536:
                        raise ValueError('client request missing or exceeds bound')
                    request += block
                for block in chunks:
                    connection.sendall(block)
            stdout, stderr = process.communicate(timeout=10)
            result_code = process.returncode
    except (OSError, ValueError, subprocess.TimeoutExpired) as failure:
        error = type(failure).__name__+': '+str(failure)
    finally:
        if process is not None and process.poll() is None:
            process.kill()
            stdout, stderr = process.communicate(timeout=5)
            result_code = process.returncode
        cleanup = dict(socket_identity=socket_identity, socket_unlinked=False,
                       root_identity=[root_metadata.st_dev, root_metadata.st_ino])
        if socket_identity is not None:
            entry = address.lstat()
            if (entry.st_dev, entry.st_ino, stat.S_IFMT(entry.st_mode)) != socket_identity:
                raise ValueError('owned protocol socket identity changed; retained')
            cleanup['action'] = 'unlink-owned-socket-intent'
            evidence = directory/'socket-cleanup.json'
            with evidence.open('x') as stream:
                json.dump(cleanup, stream);stream.flush();os.fsync(stream.fileno())
            entry = address.lstat()
            if (entry.st_dev, entry.st_ino, stat.S_IFMT(entry.st_mode)) != socket_identity:
                raise ValueError('owned protocol socket changed after intent; retained')
            address.unlink()
            cleanup['socket_unlinked'] = True
        current = socket_root.lstat()
        if (current.st_dev, current.st_ino) != (root_metadata.st_dev, root_metadata.st_ino):
            raise ValueError('owned protocol socket directory changed; retained')
        socket_root.rmdir()
        cleanup['root_removed'] = True
        (directory/'socket-cleanup-result.json').write_text(json.dumps(cleanup)+'\n')
    (directory/'stdout.bin').write_bytes(stdout)
    (directory/'stderr.bin').write_bytes(stderr)
    (directory/'request.bin').write_bytes(request)
    passed = (error is None and result_code == code
              and (stdout.strip() == expected if code == 0 else expected in stderr)
              and b'-eval ' in request)
    return dict(id=name, seconds=time.monotonic()-started, exit_code=result_code,
                passed=passed, error=error, stdout_bytes=len(stdout), stderr_bytes=len(stderr))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--client', type=Path, action='append', required=True)
    parser.add_argument('--out', type=Path, required=True)
    options = parser.parse_args()
    out = options.out.resolve()
    if not out.is_relative_to(REPO/'build') or out.exists():
        raise SystemExit('fresh owned output required')
    out.mkdir(parents=True)
    pid = ('-emacs-pid '+str(os.getpid())+'\n').encode()
    cases = [
        ('multiple-lines', [pid+b'-print first\n-print second\n'], b'first\nsecond', 0),
        ('partial-large-line', [pid+b'-print '+b'x'*3000, b'x'*2000+b'\n'], b'x'*5000, 0),
        ('complete-and-partial-tail', [pid+b'-print first\n-print '+b'x'*3000,
                                      b'x'*2000+b'\n'], b'first\n'+b'x'*5000, 0),
        ('legacy-continuation', [pid+b'-print-nonl first\n-print-nonl second\n'], b'firstsecond', 0),
        ('server-error', [pid+b'-error forced-error\n'], b'forced-error', 1),
    ]
    report = dict(passed=False, scope='client protocol component, isolated fake server', clients=[])
    for number, original in enumerate(options.client):
        client = original.resolve()
        if not client.is_file() or not os.access(client, os.X_OK):
            raise ValueError('supplied client is not executable')
        identity = digest(client)
        directory = out/('client-'+str(number));directory.mkdir()
        results = [run_case(client, name, chunks, expected, code, directory)
                   for name, chunks, expected, code in cases]
        unchanged = identity == digest(client)
        report['clients'].append(dict(client=str(client), sha256=identity,
            unchanged=unchanged, cases=results, passed=unchanged and all(row['passed'] for row in results)))
        (out/'results.json').write_text(json.dumps(report, indent=2)+'\n')
    report['passed'] = all(row['passed'] for row in report['clients'])
    (out/'results.json').write_text(json.dumps(report, indent=2)+'\n')
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
