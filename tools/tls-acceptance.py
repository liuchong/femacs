#!/usr/bin/env python3
"""Real GnuTLS integration acceptance for explicit complete products.

Run under one external controlled-run owner; this driver never builds a product
and never nests another controller.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import os
from pathlib import Path
import shutil
import socket
import ssl
import sys
import threading
import time
import traceback


ROOT = Path(__file__).resolve().parent.parent
DRIVER = Path(__file__).resolve()
FIXTURE = ROOT / 'tools/fixtures/tls-acceptance.el'
STABLE = ROOT / 'compatibility/stable-31.1/sources.json'
ORIGINAL_REL = 'test/lisp/net/gnutls-tests.el'
ORIGINAL_IDS = (
    'test-gnutls-000-availability',
    'test-gnutls-000-data-extractions',
    'test-gnutls-001-hashes-internal-digests',
    'test-gnutls-002-hashes-digests',
    'test-gnutls-003-hashes-hmacs',
    'test-gnutls-004-symmetric-ciphers',
    'test-gnutls-005-aead-ciphers',
)
OWN_IDS = (
    'fe-tls-foundation',
    'fe-tls-error-contract',
    'fe-tls-known-vectors',
    'fe-tls-buffer-region',
    'fe-tls-local-lifecycle',
    'fe-tls-local-hostname-rejection',
    'fe-tls-local-wrong-ca',
    'fe-tls-local-async',
)
EXPECTED_IDS = tuple(sorted(ORIGINAL_IDS + OWN_IDS))
LOCAL_IDS = {
    'fe-tls-local-lifecycle',
    'fe-tls-local-hostname-rejection',
    'fe-tls-local-wrong-ca',
    'fe-tls-local-async',
}
ECHO_IDS = {'fe-tls-local-lifecycle', 'fe-tls-local-async'}
SELECTOR = r'^\(?:test-gnutls-\|fe-tls-\)'
MAX_OUTPUT_BYTES = 32 * 1024 * 1024


def load_tool(name, relative):
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


execution = load_tool('fe_tls_execution', 'tools/runtime-execution.py')


def identity(path):
    return dict(path=str(path), **execution.file_identity(path))


def complete(process):
    return (process['complete'] and process['exit_code'] == 0
            and not process['timed_out'] and not process['remaining_group_terminated'])


def plain_environment():
    return {key: value for key, value in os.environ.items()
            if key not in execution.FORBIDDEN_ENV and not key.startswith('FEMACS_')}


def find_openssl(value):
    candidate = value or 'openssl'
    found = shutil.which(candidate) if os.sep not in candidate else candidate
    if not found or not os.path.isfile(found) or not os.access(found, os.X_OK):
        raise ValueError('an executable OpenSSL is required')
    return Path(found).resolve()


def write_text(path, text, mode=0o600):
    path.write_text(text, encoding='utf-8')
    path.chmod(mode)
    return path


def process_text(process, name):
    path = Path(process['evidence_directory']) / name
    return path.read_text(encoding='utf-8', errors='replace') if path.is_file() else ''


def run_checked(command, directory, timeout, *, cwd=None):
    row = execution.run_process(command, directory, plain_environment(), timeout, cwd=cwd or directory)
    if not complete(row):
        detail = process_text(row, 'stderr.txt')[-4000:]
        raise RuntimeError(f"command failed: {command[0]} ({row['status']}): {detail}")
    return row


def generate_material(out, openssl, timeout):
    material = out / 'tls-material'
    material.mkdir(mode=0o700)
    ca_config = write_text(material / 'ca.cnf', """[req]
prompt = no
distinguished_name = dn
x509_extensions = v3_ca
[dn]
CN = Femacs TLS Acceptance CA
[v3_ca]
basicConstraints = critical,CA:TRUE
keyUsage = critical,keyCertSign,cRLSign
subjectKeyIdentifier = hash
""")
    wrong_ca_config = write_text(material / 'wrong-ca.cnf', """[req]
prompt = no
distinguished_name = dn
x509_extensions = v3_ca
[dn]
CN = Femacs TLS Acceptance Wrong CA
[v3_ca]
basicConstraints = critical,CA:TRUE
keyUsage = critical,keyCertSign,cRLSign
subjectKeyIdentifier = hash
""")
    leaf_request = write_text(material / 'leaf-request.cnf', """[req]
prompt = no
distinguished_name = dn
[dn]
CN = localhost
""")
    leaf_extensions = write_text(material / 'leaf-extensions.cnf', """[v3_leaf]
basicConstraints = critical,CA:FALSE
keyUsage = critical,digitalSignature,keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = @alt_names
[alt_names]
DNS.1 = localhost
IP.1 = 127.0.0.1
""")
    commands = []
    for label, command in (
        ('ca', [openssl, 'req', '-x509', '-newkey', 'rsa:2048', '-sha256',
                '-nodes', '-days', '2', '-keyout', 'ca.key', '-out', 'ca.crt',
                '-config', ca_config]),
        ('wrong-ca', [openssl, 'req', '-x509', '-newkey', 'rsa:2048', '-sha256',
                      '-nodes', '-days', '2', '-keyout', 'wrong-ca.key',
                      '-out', 'wrong-ca.crt', '-config', wrong_ca_config]),
        ('leaf-request', [openssl, 'req', '-new', '-newkey', 'rsa:2048', '-sha256',
                          '-nodes', '-keyout', 'leaf.key', '-out', 'leaf.csr',
                          '-config', leaf_request]),
        ('leaf-sign', [openssl, 'x509', '-req', '-in', 'leaf.csr', '-CA', 'ca.crt',
                       '-CAkey', 'ca.key', '-CAcreateserial', '-out', 'leaf.crt',
                       '-days', '2', '-sha256', '-extfile', leaf_extensions,
                       '-extensions', 'v3_leaf']),
    ):
        commands.append(dict(label=label, process=run_checked(
            command, material / ('command-' + label), timeout, cwd=material)))
    names = ('ca.crt', 'ca.key', 'wrong-ca.crt', 'wrong-ca.key',
             'leaf.crt', 'leaf.key', 'leaf.csr')
    files = {name: execution.file_identity(material / name) for name in names}
    return dict(directory=material, files=files, commands=commands,
                openssl=identity(openssl),
                ca=material / 'ca.crt', wrong_ca=material / 'wrong-ca.crt',
                cert=material / 'leaf.crt', key=material / 'leaf.key')


class TLSServer:
    def __init__(self, certificate, key, echo, timeout):
        self.context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
        self.context.load_cert_chain(certfile=str(certificate), keyfile=str(key))
        self.echo = echo
        self.timeout = timeout
        self.listener = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.listener.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        self.listener.bind(('127.0.0.1', 0))
        self.listener.listen(4)
        self.listener.settimeout(0.2)
        self.port = self.listener.getsockname()[1]
        self.stop_event = threading.Event()
        self.thread = None
        self.active = None
        self.records = []
        self.error = None

    def start(self):
        self.thread = threading.Thread(target=self._serve, name='fe-tls-server', daemon=True)
        self.thread.start()

    def _serve(self):
        deadline = time.monotonic() + self.timeout
        try:
            while (not self.stop_event.is_set() and time.monotonic() < deadline
                   and len(self.records) < 1):
                try:
                    connection, address = self.listener.accept()
                except socket.timeout:
                    continue
                except OSError as error:
                    if not self.stop_event.is_set():
                        self.error = f'{type(error).__name__}: {error}'
                    break
                record = dict(peer=address[0], peer_port=address[1], accepted=True,
                              handshake_ok=False, sent=False, received_text=None,
                              received_hex=None)
                self.active = connection
                tls = None
                try:
                    connection.settimeout(max(5, min(self.timeout, 30)))
                    tls = self.context.wrap_socket(connection, server_side=True,
                                                   suppress_ragged_eofs=False)
                    self.active = tls
                    record['handshake_ok'] = True
                    if self.echo:
                        data = b''
                        while b'\n' not in data and len(data) < 4096:
                            part = tls.recv(256)
                            if not part:
                                break
                            data += part
                        record['received_hex'] = data.hex()
                        record['received_text'] = data.decode('utf-8', errors='replace')
                        tls.sendall(b'pong\n')
                        record['sent'] = True
                        try:
                            # Keep the session alive until the client requests
                            # shutdown.  Proactively unwrapping after pong can
                            # retire its process session before its bye call.
                            trailing = tls.recv(1)
                            if trailing:
                                raise ValueError('unexpected application data after ping')
                            record['client_close_notify'] = True
                            plain = tls.unwrap()
                            record['server_close_notify'] = True
                            plain.close()
                        except (OSError, ssl.SSLError) as error:
                            record['close_error'] = f'{type(error).__name__}: {error}'
                    else:
                        tls.settimeout(1)
                        try:
                            tls.recv(1)
                        except (OSError, ssl.SSLError):
                            pass
                except (OSError, ssl.SSLError, ValueError) as error:
                    record['tls_error'] = f'{type(error).__name__}: {error}'
                finally:
                    if tls is not None:
                        try:
                            tls.close()
                        except OSError:
                            pass
                    try:
                        connection.close()
                    except OSError:
                        pass
                    if self.active is connection or self.active is tls:
                        self.active = None
                self.records.append(record)
        except Exception as error:
            self.error = f'{type(error).__name__}: {error}'
        finally:
            try:
                self.listener.close()
            except OSError:
                pass

    def stop(self):
        self.stop_event.set()
        try:
            self.listener.close()
        except OSError:
            pass
        if self.thread is not None and self.echo:
            # The client has returned: let an in-flight close_notify finish
            # before the existing forced socket/thread cleanup path.
            self.thread.join(timeout=1)
        active = self.active
        if active is not None:
            try:
                active.shutdown(socket.SHUT_RDWR)
            except OSError:
                pass
            try:
                active.close()
            except OSError:
                pass
        if self.thread is not None:
            self.thread.join(timeout=5)

    def record(self):
        return dict(host='127.0.0.1', port=self.port, echo=self.echo,
                    records=list(self.records), error=self.error,
                    thread_alive=bool(self.thread and self.thread.is_alive()))


def validate_server(record, echo):
    if record.get('error'):
        return False, 'TLS server failed: ' + record['error']
    if record.get('thread_alive'):
        return False, 'TLS server thread did not stop'
    records = record.get('records', [])
    if len(records) != 1:
        return False, f"expected one accepted connection, got {len(records)}"
    item = records[0]
    if not item.get('accepted'):
        return False, 'TLS server did not accept a connection'
    if echo and not item.get('handshake_ok'):
        return False, 'TLS server did not complete a handshake'
    if echo and (not item.get('sent') or item.get('received_text') != 'ping\n'):
        return False, 'TLS server did not receive and echo the client payload'
    if echo and (not item.get('client_close_notify')
                 or not item.get('server_close_notify') or item.get('close_error')
                 or item.get('tls_error')):
        return False, 'TLS close_notify exchange did not complete cleanly'
    return True, None


def output_bytes(row):
    directory = Path(row.get('evidence_directory', ''))
    total = 0
    for name in ('stdout.txt', 'stderr.txt', 'response.json'):
        path = directory / name
        if path.is_file():
            total += path.stat().st_size
    return total


def invoke_case(product, directory, wrapper, extra_env, timeout, test_id):
    request = dict(operation='ert', fixtures=[str(wrapper)], id=test_id)
    row = execution.invoke(product, directory, request, timeout, extra_env=extra_env)
    response = row.get('response', {})
    status = response.get('status')
    size = output_bytes(row)
    row.update(case_id=test_id, test_status=status, output_bytes=size)
    row['passed'] = bool(row.get('passed') and status == 'passed'
                         and response.get('id') == test_id
                         and size <= MAX_OUTPUT_BYTES)
    if size > MAX_OUTPUT_BYTES:
        row['output_oversized'] = True
    if status == 'skipped':
        row['failure'] = 'skipped test is not acceptance'
    elif status != 'passed':
        row['failure'] = f'test status is {status!r}'
    execution.write_json(directory / 'result.json', row)
    return row


def discover_case(product, directory, wrapper, extra_env, timeout):
    request = dict(operation='discover', fixtures=[str(wrapper)], selector=SELECTOR)
    row = execution.invoke(product, directory, request, timeout, extra_env=extra_env)
    response = row.get('response', {})
    ids = response.get('ids')
    row['discovered_ids'] = ids
    row['expected_ids'] = list(EXPECTED_IDS)
    row['version'] = response.get('metadata', {}).get('version')
    row['passed'] = bool(row.get('passed') and isinstance(ids, list)
                         and ids == list(EXPECTED_IDS))
    if ids != list(EXPECTED_IDS):
        row['registration_error'] = 'discovered test registration differs from the exact acceptance set'
    execution.write_json(directory / 'result.json', row)
    return row


def validate_products(products):
    references = [product for product in products if product['kind'] == 'reference']
    if len(references) != 1:
        raise ValueError('include exactly one installed reference product')
    baseline = {(product['kind'], product['compiler_leg'])
                for product in products
                if product['kind'] != 'reference' and not product['native']
                and not product['observer'] and product['collector'] == 'off'}
    expected = {(kind, leg) for kind in ('terminal', 'cocoa')
                for leg in ('normal', 'checking')}
    if baseline != expected:
        raise ValueError('include normal/checking baseline terminal and cocoa products')
    return references[0]


def base_env(original, material, directory):
    return {
        'FEMACS_TLS_ORIGINAL': str(original),
        'FEMACS_TLS_CA': str(material['ca']),
        'FEMACS_TLS_WRONG_CA': str(material['wrong_ca']),
        'FEMACS_TLS_CERT': str(material['cert']),
        'FEMACS_TLS_KEY': str(material['key']),
        'FEMACS_TLS_TEST_DIR': str(directory / 'out'),
    }


def run_case(product, directory, wrapper, original, material, timeout, test_id):
    env = base_env(original, material, directory)
    server = None
    row = None
    try:
        if test_id in LOCAL_IDS:
            server = TLSServer(material['cert'], material['key'],
                               test_id in ECHO_IDS, timeout)
            server.start()
            env['FEMACS_TLS_PORT'] = str(server.port)
        row = invoke_case(product, directory, wrapper, env, timeout, test_id)
    except Exception as error:
        directory.mkdir(parents=True, exist_ok=True)
        row = dict(case_id=test_id, passed=False, complete=False, exit_code=None,
                   failure=f'{type(error).__name__}: {error}',
                   evidence_directory=str(directory), test_status='not-run')
        execution.write_json(directory / 'case-error.json',
                             dict(case_id=test_id, error=f'{type(error).__name__}: {error}',
                                  traceback=traceback.format_exc()))
    finally:
        if server is not None:
            server.stop()
            record = server.record()
            if row is None:
                row = dict(case_id=test_id, passed=False, complete=False, exit_code=None,
                           evidence_directory=str(directory), test_status='not-run')
            row['tls_server'] = record
            server_ok, reason = validate_server(record, test_id in ECHO_IDS)
            if not server_ok:
                row['passed'] = False
                row['server_failure'] = reason
    if row is not None and directory.is_dir():
        execution.write_json(directory / 'result.json', row)
    return row


def run(args):
    out = execution.new_output(args.out)
    products_path = Path(args.products).absolute()
    products = execution.load_products(products_path)
    reference = validate_products(products)
    openssl = find_openssl(args.openssl)
    manifest = execution.bounded_json(STABLE)
    if manifest['source']['release'] != '31.1':
        raise ValueError('stable source release changed')
    entry = {row['path']: row for row in manifest['entries']}.get(ORIGINAL_REL)
    if entry is None:
        raise ValueError('stable GnuTLS test source is not in the manifest')
    relative = Path(entry['source_path'])
    if relative.is_absolute() or '..' in relative.parts:
        raise ValueError('stable source path escapes the repository')
    original_source = ROOT / relative
    if (original_source.stat().st_size != entry['size']
            or execution.digest(original_source) != entry['sha256']):
        raise ValueError('registered GnuTLS test source identity changed')

    wrapper = execution.copy_owned_data(FIXTURE, out / 'tls-acceptance.el')
    original = execution.copy_owned_data(original_source, out / 'gnutls-tests.el')
    fixture_before = {'wrapper': identity(wrapper), 'original': identity(original)}
    material = generate_material(out, openssl, args.openssl_timeout)

    input_paths = [DRIVER, FIXTURE, STABLE, products_path, original_source, openssl,
                   ROOT / 'tools/runtime-execution.py', execution.HARNESS]
    before_inputs = [identity(path) for path in input_paths]
    product_before = {product['id']: execution.product_identity(product, resources=True)
                      for product in products}
    cases, discoveries, not_run, product_rows = [], [], [], []
    reference_version = None
    started = time.time()

    for product in products:
        root = out / product['id']
        discovery = discover_case(product, root / 'discovery', wrapper,
                                  base_env(original, material, root / 'discovery'),
                                  args.timeout)
        discoveries.append(discovery)
        if product['id'] == reference['id']:
            reference_version = discovery.get('version')
        product_row = dict(id=product['id'], kind=product['kind'],
                           compiler_leg=product['compiler_leg'],
                           native=product['native'], observer=product['observer'],
                           collector=product['collector'], version=discovery.get('version'),
                           discovery_passed=discovery['passed'])
        product_rows.append(product_row)
        if not discovery['passed']:
            for test_id in EXPECTED_IDS:
                not_run.append(dict(profile=product['id'], id=test_id,
                                    reason='test discovery did not expose the exact set'))
            continue
        for index, test_id in enumerate(EXPECTED_IDS):
            directory = root / f'case-{index:02d}-{test_id}'
            row = run_case(product, directory, wrapper, original, material,
                           args.timeout, test_id)
            cases.append(row)
            execution.write_json(out / 'partial.json', dict(
                schema='femacs-tls-progress-v1', passed=False,
                profile=product['id'], latest=test_id,
                cases=len(cases), failed=sum(
                    item.get('passed') is False and item.get('test_status') != 'skipped'
                    for item in cases),
                not_run=len(not_run), final=False))

    after_inputs = [identity(path) for path in input_paths]
    fixture_after = {'wrapper': identity(wrapper), 'original': identity(original)}
    product_after = {product['id']: execution.product_identity(product, resources=True)
                     for product in products}
    final_receipts_valid = execution.load_products(products_path) == products
    inputs_unchanged = before_inputs == after_inputs
    fixtures_unchanged = fixture_before == fixture_after
    products_unchanged = product_before == product_after
    reference_version_ok = reference_version == '31.1'
    expected_cases = len(products) * len(EXPECTED_IDS)
    skipped = sum(row.get('test_status') == 'skipped' for row in cases)
    executed = len(cases)
    passed_cases = sum(row.get('passed') is True for row in cases)
    failed_cases = sum(row.get('passed') is False and row.get('test_status') != 'skipped'
                       for row in cases)
    all_discoveries = all(row['passed'] for row in discoveries)
    all_cases = executed == expected_cases and failed_cases == 0
    passed = bool(all_discoveries and all_cases and skipped == 0 and not not_run
                  and inputs_unchanged and fixtures_unchanged
                  and products_unchanged and reference_version_ok and final_receipts_valid)
    result = dict(
        schema='femacs-tls-acceptance-v1', action='tls-functional',
        passed=passed, started_utc=time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime(started)),
        scope='real GnuTLS crypto, registered stable cases, and isolated loopback PKI lifecycle; '
              'not whole stable corpus, native UI, or Goal completion',
        inputs=dict(files=before_inputs, final_files=after_inputs,
                    products=product_before, final_products=product_after,
                    stable=entry, products_path=str(products_path),
                    openssl=str(openssl)),
        inputs_unchanged=inputs_unchanged, fixtures_unchanged=fixtures_unchanged,
        products_unchanged=products_unchanged,
        final_product_receipts_valid=final_receipts_valid,
        reference_version=reference_version, reference_version_ok=reference_version_ok,
        expected_ids=list(EXPECTED_IDS), original_ids=list(ORIGINAL_IDS),
        owned_ids=list(OWN_IDS), local_ids=sorted(LOCAL_IDS),
        fixtures=dict(wrapper=fixture_before['wrapper'], original=fixture_before['original'],
                      wrapper_final=fixture_after['wrapper'],
                      original_final=fixture_after['original'],
                      wrapper_source=identity(FIXTURE)),
        certificates=dict(files=material['files'], commands=material['commands'],
                          openssl=material['openssl'], private_directory=str(material['directory'])),
        products=product_rows, discoveries=discoveries, cases=cases, not_run=not_run,
        counts=dict(expected=expected_cases, executed=executed, passed=passed_cases,
                    failed=failed_cases, skipped=skipped, not_run=len(not_run)),
        limitation='TLS acceptance only; this receipt does not establish stable-corpus, '
                   'native UI, performance, or overall Goal acceptance.')
    execution.write_json(out / 'results.json', result)
    execution.write_json(out / 'partial.json', dict(
        schema='femacs-tls-progress-v1', passed=passed, final=True,
        cases=executed, failed=failed_cases, not_run=len(not_run)))
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--products', required=True, help='explicit schema-1 product list')
    parser.add_argument('--out', required=True, help='fresh owned build output directory')
    parser.add_argument('--timeout', type=int, default=120, help='seconds per product process')
    parser.add_argument('--openssl-timeout', type=int, default=60,
                        help='seconds per certificate command')
    parser.add_argument('--openssl', help='explicit OpenSSL executable')
    args = parser.parse_args(argv)
    try:
        result = run(args)
    except Exception as error:
        out = Path(args.out).absolute()
        if out.is_dir():
            try:
                execution.write_json(out / 'failure.json', dict(
                    schema='femacs-tls-acceptance-v1', passed=False,
                    error=f'{type(error).__name__}: {error}',
                    traceback=traceback.format_exc()))
            except Exception:
                pass
        print(f'tls acceptance failed: {type(error).__name__}: {error}', file=sys.stderr)
        traceback.print_exc(file=sys.stderr)
        return 2
    print(json.dumps(dict(passed=result['passed'], out=str(Path(args.out).absolute()),
                          counts=result['counts'], reference_version=result['reference_version']),
                     sort_keys=True))
    return 0 if result['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
