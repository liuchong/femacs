#!/usr/bin/env python3
"""Actual Darwin TLS process/library layout and flag comparison.

This optional differential diagnostic uses explicit read-only C source/config
inputs as an oracle. Production builds do not require those inputs. Execute
only after the entire TLS integration set has been authored, under one external
build controller. A passing result does not certify whole-editor behavior.
"""
from __future__ import annotations
import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import sys

ROOT = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('tls_abi_execution', ROOT / 'tools/runtime-execution.py')
execution = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = execution
spec.loader.exec_module(execution)

FIELDS = ('header', 'tty_name', 'name', 'command', 'filter', 'sentinel', 'log',
          'buffer', 'childp', 'plist', 'type', 'mark', 'status',
          'decode_coding_system', 'decoding_buf', 'encode_coding_system',
          'encoding_buf', 'write_queue', 'gnutls_cred_type',
          'gnutls_boot_parameters', 'stderrproc', 'thread', 'pid', 'infd',
          'nbytes_read', 'outfd', 'open_fd', 'tick', 'update_tick',
          'decoding_carryover', 'read_output_delay', 'readmax', 'raw_status',
          'backlog', 'port', 'socktype', 'gnutls_initstage', 'gnutls_state',
          'gnutls_x509_cred', 'gnutls_anon_cred', 'gnutls_certificates',
          'gnutls_certificates_length', 'gnutls_peer_verification',
          'gnutls_extra_peer_verification', 'gnutls_log_level',
          'gnutls_handshakes_tried')
FLAGS = ('adaptive_read_buffering', 'read_output_skip', 'kill_without_query',
         'pty_in', 'pty_out', 'inherit_coding_system_flag', 'alive',
         'raw_status_new', 'is_non_blocking_client', 'is_server',
         'gnutls_p', 'gnutls_complete_negotiation_p')
ENUMS = ('gnutls_initstage_t', 'gnutls_server_name_type_t',
         'gnutls_x509_crt_fmt_t', 'gnutls_close_request_t',
         'gnutls_alert_description_t', 'gnutls_credentials_type_t',
         'gnutls_cipher_algorithm_t', 'gnutls_mac_algorithm_t',
         'gnutls_digest_algorithm_t', 'gnutls_kx_algorithm_t',
         'gnutls_protocol_t', 'gnutls_compression_method_t',
         'gnutls_certificate_type_t', 'gnutls_sign_algorithm_t',
         'gnutls_sec_param_t', 'gnutls_pk_algorithm_t',
         'gnutls_certificate_print_formats_t')


def identity(path):
    return dict(path=str(path), **execution.file_identity(path))


def complete(row):
    return (row['complete'] and row['exit_code'] == 0 and not row['timed_out']
            and not row['remaining_group_terminated'])


def dependency_paths(path):
    text = path.read_text().replace('\\\n', ' ')
    return sorted({(Path(token) if Path(token).is_absolute() else ROOT / token).resolve()
                   for token in shlex.split(text.split(':', 1)[1])})


def rust_include_inputs(product):
    # Match xtask's literal include discovery, including gated arms, and its
    # generated-config/global substitutions. Bind both the mirror and override.
    selected = {'src/include/config.rs': product / 'config/config.rs'}
    if (product / 'config/globals.rs').is_file():
        selected['src/include/globals.rs'] = product / 'config/globals.rs'
    pending = [ROOT / path for path in ('src/include/config.rs', 'src/include/lisp.rs',
               'lib/include/sys_types.in.rs', 'src/include/process.rs')]
    seen = set()
    inputs = set()
    while pending:
        mirror = pending.pop().resolve()
        if mirror in seen:
            continue
        seen.add(mirror)
        inputs.add(mirror)
        chosen = next((replacement for name, replacement in selected.items()
                       if mirror == ROOT / name), mirror).resolve()
        inputs.add(chosen)
        for line in chosen.read_text().splitlines():
            match = re.search(r'include!\(\s*"([^"]+)"\s*\)', line)
            if match:
                target = (mirror.parent / match.group(1)).resolve()
                if target.is_file():
                    pending.append(target)
    return sorted(inputs)


def library_function_assertions():
    source = (ROOT / 'src/gnutls.rs').read_text()
    aliases = dict(c_char='char', c_uchar='unsigned char', c_int='int',
                   c_uint='unsigned int', c_void='void', c_long='long',
                   c_ulong='unsigned long')

    def c_type(value):
        value = value.strip()
        if value.startswith('*const '):
            return 'const ' + c_type(value[len('*const '):]) + ' *'
        if value.startswith('*mut '):
            return c_type(value[len('*mut '):]) + ' *'
        if value == '()':
            return 'void'
        if not re.fullmatch(r'[A-Za-z_][A-Za-z_0-9]*', value):
            raise ValueError('unsupported actual foreign type: ' + value)
        return aliases.get(value, value)

    expressions = []
    names = set()
    for match in re.finditer(r'pub fn (gnutls_\w+)\s*\(([^;]*?)\)\s*(?:->\s*([^;]+))?;', source):
        name, parameters, result = match.groups()
        if name in names:
            raise ValueError('duplicate directly linked library declaration: ' + name)
        names.add(name)
        arguments = []
        for parameter in parameters.split(','):
            if not parameter.strip():
                continue
            _, separator, actual_type = parameter.partition(':')
            if not separator:
                raise ValueError('unnamed actual Rust foreign parameter')
            arguments.append(c_type(actual_type))
        function_type = c_type(result or '()') + ' (*)(' + (', '.join(arguments) or 'void') + ')'
        expressions.append('_Static_assert(__builtin_types_compatible_p(__typeof__(&'
                           + name + '), ' + function_type + '), "foreign API: ' + name + '");')
    if not expressions:
        raise ValueError('missing actual directly linked TLS declarations')
    return expressions


def c_probe():
    lines = ['#include <config.h>', '#undef HAVE_GNUTLS', '#define HAVE_GNUTLS 1',
             '#undef THREADS_ENABLED', '#define THREADS_ENABLED 1',
             '#undef HAVE_PTHREAD', '#define HAVE_PTHREAD 1',
             '#undef HAVE_MODULES', '#define HAVE_MODULES 1',
             '#include <stddef.h>', '#include <stdio.h>', '#include <string.h>',
             '#include <gnutls/gnutls.h>', '#include <gnutls/crypto.h>',
             '#include <gnutls/x509.h>', '#include "lisp.h"', '#include "process.h"',
             *library_function_assertions(),
             'static void bytes(const char *name, const void *p, size_t n) {',
             ' printf("%s=", name); for (size_t i=0; i<n; ++i) printf("%02x", ((const unsigned char *)p)[i]);',
             ' putchar(10); }', 'int main(void) {',
             'printf("process.size=%zu\\n", sizeof(struct Lisp_Process));',
             'printf("process.align=%zu\\n", _Alignof(struct Lisp_Process));',
             'printf("datum.size=%zu\\n", sizeof(gnutls_datum_t));',
             'printf("datum.align=%zu\\n", _Alignof(gnutls_datum_t));',
             'printf("datum.data=%zu\\n", offsetof(gnutls_datum_t, data));',
             'printf("datum.size-field=%zu\\n", offsetof(gnutls_datum_t, size));']
    lines.extend(f'printf("process.{field}=%zu\\n", offsetof(struct Lisp_Process, {field}));'
                 for field in FIELDS)
    for kind in ENUMS:
        lines.extend((f'printf("{kind}.size=%zu\\n", sizeof({kind}));',
                      f'printf("{kind}.signed=%d\\n", ({kind})-1 < ({kind})0);'))
    lines.append('struct Lisp_Process p;')
    for flag in FLAGS:
        lines.extend(('memset(&p, 0, sizeof p);', f'p.{flag} = 1;',
                      f'bytes("flag.{flag}", &p, sizeof p);'))
    lines.extend(('memset(&p, 0, sizeof p);', 'p.gnutls_p = 1; p.gnutls_complete_negotiation_p = 1;',
                  'p.gnutls_p = 0; bytes("flag.tls-neighbor-preserved", &p, sizeof p);',
                  'return 0;', '}'))
    return '\n'.join(lines) + '\n'


def rust_probe():
    original = (ROOT / 'src/gnutls.rs').read_text()
    # Read the actual owned definitions, rather than inventing another ABI mirror.
    datum = re.search(r'pub struct gnutls_datum_t\s*\{[^}]*\}', original).group()
    types = []
    for kind in ENUMS:
        if kind == 'gnutls_initstage_t':
            continue  # owned process header imports its real shared definition
        match = re.search(r'pub type ' + kind + r'\s*=\s*[^;]+;', original)
        if not match:
            raise ValueError('missing actual owned TLS type: ' + kind)
        types.append(match.group())
    lines = ['#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_macros, unused_variables, static_mut_refs, unsafe_op_in_unsafe_fn)]',
             f'include!({json.dumps(str(ROOT / "src/include/config.rs"))});',
             f'include!({json.dumps(str(ROOT / "src/include/lisp.rs"))});',
             f'include!({json.dumps(str(ROOT / "lib/include/sys_types.in.rs"))});',
             f'include!({json.dumps(str(ROOT / "src/include/process.rs"))});',
             *types, '#[repr(C)]', datum,
             'fn bytes(name: &str, p: &Lisp_Process) {',
             ' print!("{}=", name); let data = unsafe { std::slice::from_raw_parts((p as *const Lisp_Process).cast::<u8>(), std::mem::size_of::<Lisp_Process>()) };',
             ' for b in data { print!("{:02x}", b); } println!();', '}',
             'fn main() {',
             'println!("process.size={}", std::mem::size_of::<Lisp_Process>());',
             'println!("process.align={}", std::mem::align_of::<Lisp_Process>());',
             'println!("datum.size={}", std::mem::size_of::<gnutls_datum_t>());',
             'println!("datum.align={}", std::mem::align_of::<gnutls_datum_t>());',
             'println!("datum.data={}", std::mem::offset_of!(gnutls_datum_t, data));',
             'println!("datum.size-field={}", std::mem::offset_of!(gnutls_datum_t, size));']
    lines.extend(f'println!("process.{field}={{}}", std::mem::offset_of!(Lisp_Process, {"r#type" if field == "type" else field}));'
                 for field in FIELDS)
    for kind in ENUMS:
        lines.extend((f'println!("{kind}.size={{}}", std::mem::size_of::<{kind}>());',
                      f'println!("{kind}.signed={{}}", ((-1_i64 as {kind}) < (0 as {kind})) as u8);'))
    for index, flag in enumerate(FLAGS):
        lines.extend(('let mut storage = std::mem::MaybeUninit::<Lisp_Process>::uninit();',
                      'unsafe { std::ptr::write_bytes(storage.as_mut_ptr().cast::<u8>(), 0, std::mem::size_of::<Lisp_Process>()); }',
                      'let p = unsafe { &mut *storage.as_mut_ptr() };'))
        if index == 0:
            lines.append('p.bits1 = 1 << PROCESS_ADAPTIVE_READ_BUFFERING_SHIFT;')
        elif index == 1:
            lines.append('p.bits1 = 1 << PROCESS_READ_OUTPUT_SKIP_SHIFT;')
        elif index < 10:
            lines.append(f'p.bits2 = 1 << {index - 2};')
        else:
            method = 'set_gnutls_p' if index == 10 else 'set_gnutls_complete_negotiation_p'
            lines.append(f'p.{method}(true);')
        lines.append(f'bytes("flag.{flag}", p);')
    lines.extend(('let mut storage = std::mem::MaybeUninit::<Lisp_Process>::uninit();',
                  'unsafe { std::ptr::write_bytes(storage.as_mut_ptr().cast::<u8>(), 0, std::mem::size_of::<Lisp_Process>()); }',
                  'let p = unsafe { &mut *storage.as_mut_ptr() };',
                  'p.set_gnutls_p(true); p.set_gnutls_complete_negotiation_p(true); p.set_gnutls_p(false);',
                  'bytes("flag.tls-neighbor-preserved", p);', '}'))
    return '\n'.join(lines) + '\n'


def observations(path):
    rows = {}
    for line in path.read_text().splitlines():
        key, separator, value = line.partition('=')
        if not separator or key in rows or not re.fullmatch(r'[A-Za-z0-9_.-]+', key):
            raise ValueError('invalid or duplicate actual ABI observation')
        if not re.fullmatch(r'[0-9a-f]+', value):
            raise ValueError('invalid ABI value')
        rows[key] = value
    expected = ({'process.size', 'process.align', 'datum.size', 'datum.align', 'datum.data', 'datum.size-field',
                 'flag.tls-neighbor-preserved'} | {'process.' + f for f in FIELDS}
                | {'flag.' + flag for flag in FLAGS}
                | {kind + '.' + key for kind in ENUMS for key in ('size', 'signed')})
    if rows.keys() != expected:
        raise ValueError('incomplete actual ABI observation set')
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('out', 'cc', 'sdk-root', 'c-source', 'c-config', 'gnutls-prefix', 'product-root', 'xtask'):
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    out = args.out.absolute()
    if not out.is_relative_to(ROOT / 'build') or out.resolve() != out or out.exists():
        parser.error('fresh owned build output required')
    product = execution.safe_path(args.product_root.absolute())
    configured = execution.bounded_json(product / 'config/darwin-cocoa.json'
        if (product / 'config/darwin-cocoa.json').exists() else product / 'config/darwin-terminal.json')
    values = {key.upper(): value for section in ('macros', 'features') for key, value in configured[section].items()}
    if values.get('HAVE_GNUTLS') != '1':
        parser.error('an actual TLS-enabled product configuration is required')
    receipt = execution.bounded_json(product / 'tls/build-receipt.json')
    inputs = [Path(__file__), ROOT / 'tools/runtime-execution.py', ROOT / 'src/gnutls.rs',
              ROOT / 'src/include/process.rs', ROOT / 'src/include/gnutls-types.rs',
              args.xtask.absolute(), args.cc.absolute(), args.c_config.absolute() / 'config.h',
              args.c_source.absolute() / 'process.h', args.c_source.absolute() / 'gnutls.h',
              product / 'config/config.rs', product / 'tls/build-receipt.json',
              *rust_include_inputs(product)]
    for name, expected in receipt['headers']:
        path = args.gnutls_prefix.absolute() / 'include/gnutls' / name
        if execution.digest(path) != expected:
            raise ValueError('actual TLS header differs from built product: ' + name)
        inputs.append(path)
    out.mkdir(parents=True)
    c, rust = out / 'process-c.c', out / 'process-rust.rs'
    c.write_text(c_probe())
    rust.write_text(rust_probe())
    env = {k:v for k,v in os.environ.items() if not k.startswith('FEMACS_') and k not in execution.FORBIDDEN_ENV}
    c_source, c_config = args.c_source.absolute(), args.c_config.absolute()
    c_command = [args.cc.absolute(), '-arch', 'arm64', '-isysroot', args.sdk_root.absolute(),
        '-std=gnu2x', '-O2',
        '-I', c_config, '-I', c_config.parent / 'lib', '-I', c_source, '-I', c_source.parent / 'lib',
        '-I', ROOT / 'build/generation/module-api', '-I', args.gnutls_prefix.absolute() / 'include',
        ]
    discovery = execution.run_process([*c_command, '-M', '-MF', out / 'C-before.d', c],
        out / 'C-dependencies', env, 180, cwd=ROOT)
    if not complete(discovery):
        execution.write_json(out / 'results.json', dict(passed=False,
            C_dependency_discovery=discovery, whole_editor_accepted=False,
            runtime_TLS_accepted=False))
        return 1
    c_inputs = dependency_paths(out / 'C-before.d')
    inputs = sorted({path.resolve() for path in [*inputs, *c_inputs, rust]})
    before = [identity(path) for path in inputs]
    execution.write_json(out / 'inputs.json', dict(files=before,
        scope='actual TLS-enabled process layout, packed flags, datum and enum representations; literal Rust include graph and actual C compiler dependency closure bound before and after; not whole-editor acceptance'))
    commands = {'C': [*c_command, '-MD', '-MF', out / 'C.d', c, '-o', out / 'process-c']}
    flags = []
    for key, value in sorted(values.items()):
        positive = value in ('1', 'true', True)
        flags.extend(('--cfg', 'feature="' + ('' if positive else 'not_') + key.lower() + '"'))
    commands['Rust'] = [args.xtask.absolute(), 'probe', rust, out / 'process-rust',
                        '--target', 'aarch64-apple-darwin', *flags]
    env['FEMACS_CONFIG_RS'] = str(product / 'config/config.rs')
    env['FEMACS_CONFIG_FILE'] = str(product / 'config/darwin-cocoa.json'
        if (product / 'config/darwin-cocoa.json').exists() else product / 'config/darwin-terminal.json')
    for name in ('globals.rs', 'symbol-names.rs'):
        path = product / 'config' / name
        if path.exists():
            env['FEMACS_GLOBALS_RS' if name == 'globals.rs' else 'FEMACS_SYMBOL_NAMES_RS'] = str(path)
    rows, data = [], {}
    for name, command in commands.items():
        built = execution.run_process(command, out / (name + '-compile'), env, 180, cwd=ROOT)
        row = dict(id=name, compile=built, passed=False)
        if complete(built):
            path = out / ('process-c' if name == 'C' else 'process-rust')
            ran = execution.run_process([path], out / (name + '-run'), env, 30, cwd=ROOT)
            row['run'] = ran
            if complete(ran):
                data[name] = observations(out / (name + '-run/stdout.txt'))
                row['observations'] = data[name]
                row['passed'] = True
        rows.append(row)
    actual_c_inputs = dependency_paths(out / 'C.d') if (out / 'C.d').exists() else []
    dependencies = [identity(path) for path in actual_c_inputs]
    closure_unchanged = actual_c_inputs == c_inputs
    unchanged = before == [identity(path) for path in inputs]
    equal = len(data) == 2 and data['C'] == data['Rust']
    differences = {key: {name: values.get(key) for name, values in data.items()}
                   for key in set().union(*(set(v) for v in data.values()))
                   if len({v.get(key) for v in data.values()}) > 1}
    result = dict(passed=unchanged and closure_unchanged and equal and all(row['passed'] for row in rows),
                  inputs_unchanged=unchanged, observations_equal=equal, differences=differences,
                  C_dependency_closure_unchanged=closure_unchanged,
                  C_dependency_discovery=discovery,
                  cases=rows, actual_C_dependencies=dependencies,
                  whole_editor_accepted=False, runtime_TLS_accepted=False)
    execution.write_json(out / 'results.json', result)
    print(json.dumps(dict(passed=result['passed'], out=str(out))))
    return 0 if result['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
