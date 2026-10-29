#!/usr/bin/env python3
"""Compare actual Tree-sitter SDK and Lisp layouts; not whole-editor acceptance.

The reference headers/config are explicit read-only differential inputs, never
production build dependencies. Run under the existing build controller after
the complete component implementation has been written.
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
spec = importlib.util.spec_from_file_location('treesit_abi_execution', ROOT/'tools/runtime-execution.py')
execution = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = execution
spec.loader.exec_module(execution)

FIELDS = {
    'Lisp_TS_Parser': ('header', 'language_symbol', 'after_change_functions', 'tag',
        'last_set_ranges', 'embed_level', 'buffer', 'parser', 'tree', 'input',
        'need_reparse', 'visible_beg', 'visible_end', 'visi_beg_linecol',
        'visi_end_linecol', 'timestamp', 'deleted', 'need_to_gc_buffer', 'within_reparse'),
    'Lisp_TS_Node': ('header', 'parser', 'node', 'timestamp'),
    'Lisp_TS_Query': ('header', 'language', 'source', 'query', 'cursor'),
    'ts_linecol': ('bytepos', 'line', 'col'),
    'buffer': ('ts_parser_list_', 'ts_linecol_begv', 'ts_linecol_point', 'ts_linecol_zv'),
    'TSPoint': ('row', 'column'),
    'TSRange': ('start_point', 'end_point', 'start_byte', 'end_byte'),
    'TSInput': ('payload', 'read', 'encoding', 'decode'),
    'TSLogger': ('payload', 'log'),
    'TSInputEdit': ('start_byte', 'old_end_byte', 'new_end_byte', 'start_point', 'old_end_point', 'new_end_point'),
    'TSNode': ('context', 'id', 'tree'),
    'TSTreeCursor': ('tree', 'id', 'context'),
    'TSQueryCapture': ('node', 'index'),
    'TSQueryMatch': ('id', 'pattern_index', 'capture_count', 'captures'),
    'TSQueryPredicateStep': ('type', 'value_id'),
}
ENUMS = ('TSInputEncoding', 'TSLogType', 'TSQueryError', 'TSQueryPredicateStepType')
CONSTANTS = ('TREE_SITTER_LANGUAGE_VERSION', 'TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION',
    'TSInputEncodingUTF8', 'TSInputEncodingUTF16LE', 'TSInputEncodingUTF16BE', 'TSInputEncodingCustom',
    'TSLogTypeParse', 'TSLogTypeLex', 'TSQueryErrorNone', 'TSQueryErrorSyntax',
    'TSQueryErrorNodeType', 'TSQueryErrorField', 'TSQueryErrorCapture', 'TSQueryErrorStructure',
    'TSQueryErrorLanguage', 'TSQueryPredicateStepTypeDone', 'TSQueryPredicateStepTypeCapture',
    'TSQueryPredicateStepTypeString')


def split_parameters(text):
    depth, start, result = 0, 0, []
    for index, char in enumerate(text):
        if char in '(<[': depth += 1
        elif char in ')>]':
            if char != '>' or index == 0 or text[index-1] != '-': depth -= 1
        elif char == ',' and depth == 0:
            result.append(text[start:index].strip()); start = index+1
    if text[start:].strip(): result.append(text[start:].strip())
    return result


def c_type(value):
    value = value.strip().replace('core::ffi::', '')
    if value.startswith('*const '): return 'const '+c_type(value[7:])+' *'
    if value.startswith('*mut '): return c_type(value[5:])+' *'
    if value.startswith('Option<unsafe extern "C" fn '):
        text = value[len('Option<unsafe extern "C" fn '):-1].strip()
        depth = 0
        for index, char in enumerate(text):
            if char == '(': depth += 1
            elif char == ')':
                depth -= 1
                if depth == 0: break
        parameters = ', '.join(c_type(item) for item in split_parameters(text[1:index])) or 'void'
        suffix = text[index+1:].strip()
        result = c_type(suffix[2:]) if suffix.startswith('->') else 'void'
        return result+' (*)('+parameters+')'
    aliases = {'c_char':'char', 'c_void':'void', 'c_int':'int', 'c_uint':'unsigned int',
        'u8':'uint8_t', 'u16':'uint16_t', 'u32':'uint32_t', 'i32':'int32_t',
        'u64':'uint64_t', 'i64':'int64_t', 'usize':'size_t', 'isize':'ptrdiff_t', '()':'void'}
    if value in aliases: return aliases[value]
    if not re.fullmatch(r'\w+', value): raise ValueError('unsupported foreign type: '+value)
    return value


def function_contracts():
    source = (ROOT/'src/include/tree_sitter_api.rs').read_text()
    rows = []
    for match in re.finditer(r'pub fn (ts_\w+)\s*\(', source):
        name, depth, index = match[1], 1, match.end()
        while depth:
            if source[index] == '(': depth += 1
            elif source[index] == ')': depth -= 1
            index += 1
        parameters = source[match.end():index-1]
        suffix = source[index:source.index(';', index)].strip()
        if suffix and not suffix.startswith('->'): raise ValueError('invalid foreign declaration suffix')
        result = suffix[2:].strip() if suffix else '()'
        arguments = [c_type(item.split(':', 1)[1]) for item in split_parameters(parameters)]
        signature = c_type(result or '()')+' (*)('+(', '.join(arguments) or 'void')+')'
        rows.append((name, '_Static_assert(__builtin_types_compatible_p(__typeof__(&'+name+'), '+signature+'), "'+name+'");'))
    if len(rows) != 55 or len({name for name, _ in rows}) != len(rows):
        raise ValueError('unexpected used SDK function declaration set')
    return rows


def callback_contracts():
    source = (ROOT/'src/include/tree_sitter_api.rs').read_text()
    read = re.search(r'pub read:\s*([^\n]+)', source).group(1).strip().rstrip(',')
    decode = re.search(r'pub type TSDecodeFunction\s*=\s*([^;]+);', source).group(1)
    log = re.search(r'pub log:\s*(Option<unsafe extern "C" fn\s*\([^\n]+?\)>)', source).group(1)
    return ['_Static_assert(__builtin_types_compatible_p(__typeof__((('+kind+' *)0)->'+field+'), '
        +c_type(value)+'), "callback '+kind+'.'+field+'");'
        for kind, field, value in [('TSInput', 'read', read), ('TSInput', 'decode', decode), ('TSLogger', 'log', log)]]


def c_probe():
    lines = ['#include <config.h>', '#undef HAVE_TREE_SITTER', '#define HAVE_TREE_SITTER 1',
        '#include <stddef.h>', '#include <stdint.h>', '#include <stdio.h>',
        '#include <tree_sitter/api.h>', '#include "lisp.h"', '#include "buffer.h"',
        '#include "treesit.h"', *[row for _, row in function_contracts()], *callback_contracts(), 'int main(void) {']
    for kind, fields in FIELDS.items():
        ckind = 'struct '+kind if kind in ('buffer', 'ts_linecol') or kind.startswith('Lisp_') else kind
        lines.extend([f'printf("{kind}.size=%zu\\n", sizeof({ckind}));',
            f'printf("{kind}.align=%zu\\n", _Alignof({ckind}));'])
        lines.extend(f'printf("{kind}.{field}=%zu\\n", offsetof({ckind}, {field}));' for field in fields)
    for kind in ENUMS:
        lines.extend([f'printf("{kind}.size=%zu\\n", sizeof({kind}));',
            f'printf("{kind}.signed=%d\\n", (({kind})-1 < ({kind})0));'])
    lines.extend(f'printf("constant.{name}=%lld\\n", (long long){name});' for name in CONSTANTS)
    return '\n'.join([*lines, 'return 0;', '}'])+'\n'


def rust_probe():
    lines = ['#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_macros, unused_variables, static_mut_refs, unsafe_op_in_unsafe_fn)]',
        *[f'include!({json.dumps(str(ROOT/path))});' for path in
          ('src/include/config.rs', 'src/include/lisp.rs', 'src/include/buffer.rs', 'src/include/treesit.rs')], 'fn main() {']
    for kind, fields in FIELDS.items():
        lines.extend([f'println!("{kind}.size={{}}", std::mem::size_of::<{kind}>());',
            f'println!("{kind}.align={{}}", std::mem::align_of::<{kind}>());'])
        lines.extend(f'println!("{kind}.{field}={{}}", std::mem::offset_of!({kind}, {"r#type" if field == "type" else field}));'
            for field in fields)
    for kind in ENUMS:
        lines.extend([f'println!("{kind}.size={{}}", std::mem::size_of::<{kind}>());',
            f'println!("{kind}.signed={{}}", ((-1_i64 as {kind}) < (0 as {kind})) as u8);'])
    lines.extend(f'println!("constant.{name}={{}}", {name} as i64);' for name in CONSTANTS)
    return '\n'.join([*lines, '}'])+'\n'


def complete(row):
    return row['complete'] and row['exit_code'] == 0 and not row['timed_out'] and not row['remaining_group_terminated']


def dependencies(path):
    text = path.read_text().replace('\\\n', ' ')
    return sorted({(Path(token) if Path(token).is_absolute() else ROOT/token).resolve()
        for token in shlex.split(text.split(':', 1)[1])})


def rust_inputs(product):
    overrides = {ROOT/'src/include/config.rs':product/'config/config.rs'}
    if (product/'config/globals.rs').is_file(): overrides[ROOT/'src/include/globals.rs'] = product/'config/globals.rs'
    pending = [ROOT/path for path in ('src/include/config.rs', 'src/include/lisp.rs', 'src/include/buffer.rs', 'src/include/treesit.rs')]
    seen, result = set(), set()
    while pending:
        path = pending.pop().resolve()
        if path in seen: continue
        seen.add(path); result.add(path)
        selected = overrides.get(path, path).resolve(); result.add(selected)
        for match in re.finditer(r'include!\(\s*"([^"]+)"\s*\)', selected.read_text()):
            included = (path.parent/match[1]).resolve()
            if included.is_file(): pending.append(included)
    return result


def observations(path):
    expected = {kind+'.'+field for kind, fields in FIELDS.items() for field in (*fields, 'size', 'align')}
    expected |= {kind+'.'+field for kind in ENUMS for field in ('size', 'signed')}
    expected |= {'constant.'+name for name in CONSTANTS}
    values = {}
    for line in path.read_text().splitlines():
        name, separator, value = line.partition('=')
        if not separator or name in values or name not in expected or not re.fullmatch(r'-?\d+', value):
            raise ValueError('invalid actual layout observation')
        values[name] = int(value)
    if set(values) != expected: raise ValueError('incomplete actual layout observations')
    return values


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('out', 'cc', 'sdk-root', 'c-source', 'c-config', 'treesit-prefix', 'product-root', 'xtask'):
        parser.add_argument('--'+name, type=Path, required=True)
    args = parser.parse_args()
    out, product = args.out.absolute(), args.product_root.absolute()
    if not out.is_relative_to(ROOT/'build') or out.resolve() != out or out.exists(): parser.error('fresh owned output required')
    selected = product/'config/darwin-cocoa.json'
    if not selected.is_file(): selected = product/'config/darwin-terminal.json'
    configured = execution.bounded_json(selected)
    values = {key.upper():value for section in ('macros', 'features') for key, value in configured[section].items()}
    if values.get('HAVE_TREE_SITTER') != '1': parser.error('built Tree-sitter enabled product required')
    receipt = execution.bounded_json(product/'treesit/build-receipt.json')
    header = args.treesit_prefix.absolute()/'include/tree_sitter/api.h'
    if execution.digest(header) != receipt['header_sha256']: raise ValueError('SDK differs from built product')
    out.mkdir(parents=True)
    c, rust = out/'treesit-c.c', out/'treesit-rust.rs'
    c.write_text(c_probe()); rust.write_text(rust_probe())
    env = {key:value for key,value in os.environ.items() if not key.startswith('FEMACS_') and key not in execution.FORBIDDEN_ENV}
    command = [args.cc.absolute(), '-arch', 'arm64', '-isysroot', args.sdk_root.absolute(), '-std=gnu2x', '-O2',
        '-I', args.c_config.absolute(), '-I', args.c_config.absolute().parent/'lib',
        '-I', args.c_source.absolute(), '-I', args.c_source.absolute().parent/'lib',
        '-I', ROOT/'build/generation/module-api', '-I', args.treesit_prefix.absolute()/'include']
    discovery = execution.run_process([*command, '-M', '-MF', out/'C-before.d', c], out/'C-dependencies', env, 180, cwd=ROOT)
    if not complete(discovery):
        execution.write_json(out/'results.json', dict(passed=False, C_dependency_discovery=discovery)); return 1
    closure = dependencies(out/'C-before.d')
    inputs = sorted({*closure, *rust_inputs(product), Path(__file__).resolve(), ROOT/'tools/runtime-execution.py',
        args.xtask.absolute(), args.cc.absolute(), header, c, rust, selected, product/'treesit/build-receipt.json'})
    before = {str(path):execution.file_identity(path) for path in inputs}
    execution.write_json(out/'inputs.json', before)
    env['FEMACS_CONFIG_RS'] = str(product/'config/config.rs'); env['FEMACS_CONFIG_FILE'] = str(selected)
    for name in ('globals.rs', 'symbol-names.rs'):
        if (product/'config'/name).is_file(): env['FEMACS_GLOBALS_RS' if name == 'globals.rs' else 'FEMACS_SYMBOL_NAMES_RS'] = str(product/'config'/name)
    flags = []
    for key, value in sorted(values.items()):
        flags.extend(('--cfg', 'feature="'+('' if value in ('1', 'true', True) else 'not_')+key.lower()+'"'))
    commands = {'C':[*command, '-MD', '-MF', out/'C.d', c, '-o', out/'treesit-c'],
        'Rust':[args.xtask.absolute(), 'probe', rust, out/'treesit-rust', '--target', 'aarch64-apple-darwin', *flags]}
    rows, actual = [], {}
    for name, command in commands.items():
        built = execution.run_process(command, out/(name+'-compile'), env, 180, cwd=ROOT)
        row = dict(id=name, compile=built, passed=False)
        if complete(built):
            ran = execution.run_process([out/('treesit-c' if name == 'C' else 'treesit-rust')], out/(name+'-run'), env, 30, cwd=ROOT)
            row['run'] = ran
            if complete(ran): actual[name] = observations(out/(name+'-run/stdout.txt')); row['passed'] = True
        rows.append(row)
    unchanged = before == {str(path):execution.file_identity(path) for path in inputs}
    closure_unchanged = (out/'C.d').is_file() and dependencies(out/'C.d') == closure
    differences = {name:{language:data.get(name) for language,data in actual.items()}
        for name in set().union(*(data.keys() for data in actual.values()))
        if len({data.get(name) for data in actual.values()}) > 1}
    passed = unchanged and closure_unchanged and len(actual) == 2 and not differences and all(row['passed'] for row in rows)
    result = dict(passed=passed, inputs_unchanged=unchanged, C_dependency_closure_unchanged=closure_unchanged,
        C_dependency_discovery=discovery, cases=rows, observations=actual, differences=differences,
        foreign_functions=[name for name,_ in function_contracts()], callback_fields=['TSInput.read', 'TSInput.decode', 'TSLogger.log'], whole_editor_accepted=False,
        scope='actual selected SDK function declarations, SDK/Lisp object and buffer layouts; runtime acceptance separate')
    execution.write_json(out/'results.json', result)
    print(json.dumps(dict(passed=passed, observations=len(actual.get('C', {})), foreign_functions=len(result['foreign_functions']))))
    return 0 if passed else 1


if __name__ == '__main__': raise SystemExit(main())
