#!/usr/bin/env python3
"""Protect the production message count format after real-editor acceptance.

The printf statement, integer alias and buffer width come from production.
Guard bytes and a local C-ABI declaration are component substitutes. This is
not a whole-message-logger, editor or general ABI proof.
"""
from pathlib import Path
import argparse
import importlib.util
import json
import re
import sys
import time

ROOT = Path(__file__).resolve().parents[1]


def extracted_source():
    source = (ROOT / 'src/xdisp.rs').read_text()
    start = source.index('pub unsafe extern "C" fn message_dolog ')
    end = source.index('unsafe fn message_log_check_duplicate ', start)
    body = source[start:end]
    declaration = re.search(
        r'let mut dupstr:\s*\[c_char;\s*(\d+)\]\s*=\s*\[0;\s*(\d+)\];', body)
    call = re.search(
        r'let duplen:\s*c_int\s*=\s*sprintf\s*\(dupstr\.as_mut_ptr\s*\(\s*\),[\s\S]*?dups\);', body)
    alias = re.search(r'^pub type intmax_t = [^;]+;',
                      (ROOT / 'src/include/lisp.rs').read_text(), re.M)
    if not declaration or not call or not alias:
        raise ValueError('production count format declarations not found')
    if declaration[1] != declaration[2]:
        raise ValueError('production buffer type/initialization lengths differ')
    capacity = int(declaration[1])
    return dict(buffer_declaration=declaration[0], capacity=capacity,
                statement=call[0], integer_alias=alias[0])


def harness(parts):
    return r'''
#![allow(non_camel_case_types)]
use std::ffi::{c_char, c_int, CStr};
''' + parts['integer_alias'] + r'''
unsafe extern "C" { fn sprintf(out: *mut c_char, format: *const c_char, ...) -> c_int; }
const CAPACITY: usize = ''' + str(parts['capacity']) + r''';
#[repr(C)]
struct Guarded { before: [u8; 16], value: [c_char; CAPACITY], after: [u8; 16] }
fn formatted(dups: intmax_t) -> String {
    let mut guarded = Guarded { before: [0xA5; 16], value: [0; CAPACITY], after: [0x5A; 16] };
    assert_eq!(guarded.after.as_ptr() as usize - guarded.value.as_ptr() as usize, CAPACITY);
    let dupstr = &mut guarded.value;
    let duplen = unsafe {
''' + parts['statement'] + r'''
        duplen
    };
    assert!(duplen >= 0 && (duplen as usize) < CAPACITY);
    assert_eq!(guarded.value[duplen as usize], 0);
    assert_eq!(guarded.before, [0xA5; 16]);
    assert_eq!(guarded.after, [0x5A; 16]);
    let actual = unsafe { CStr::from_ptr(guarded.value.as_ptr()) };
    assert_eq!(actual.to_bytes().len(), duplen as usize);
    actual.to_str().expect("count suffix is ASCII").to_owned()
}
#[cfg(test)] mod contract {
    use super::*;
    #[test] fn two_messages() { assert_eq!(formatted(2), " [2 times]"); }
    #[test] fn three_messages() { assert_eq!(formatted(3), " [3 times]"); }
    #[test] fn maximum_count_fits() {
        assert_eq!(formatted(intmax_t::MAX), " [9223372036854775807 times]");
    }
}
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    options = parser.parse_args()
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location('message_count_component_execution',
                                                  ROOT / 'tools/runtime-execution.py')
    api = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = api
    spec.loader.exec_module(api)
    out = api.new_output(options.out)
    paths = [Path(__file__), ROOT / 'src/xdisp.rs', ROOT / 'src/include/lisp.rs',
             ROOT / 'tools/runtime-execution.py']
    report = dict(schema='femacs-message-count-component-v1', passed=False,
                  execution_complete=False, component_only=True, cases=[],
                  inputs_before={str(p): api.file_identity(p) for p in paths},
                  limits='Production format call/alias/width; canaries/local FFI declaration substitute the outer buffer and editor. No VM, general ABI or performance acceptance.')
    started = time.monotonic()

    def save():
        report['seconds'] = time.monotonic() - started
        api.write_json(out / 'results.json', report)

    save()
    try:
        if sys.platform != 'darwin':
            raise ValueError('this current integer width contract requires the bound macOS target')
        parts = extracted_source()
        source = out / 'count-contract.rs'
        source.write_text(harness(parts))
        report['extraction'] = parts
        report['generated_source'] = api.file_identity(source)
        binary = out / 'count-contract'
        environment = {k: v for k, v in __import__('os').environ.items()
                       if not k.startswith('FEMACS_')}
        compile_result = api.run_process(
            ['rustc', '--edition=2024', '--test', source, '-o', binary],
            out / 'compile', environment, 30, cwd=ROOT)
        report['compile'] = compile_result
        save()
        if (not compile_result['complete'] or compile_result['exit_code'] != 0
                or compile_result['remaining_group_terminated']):
            raise ValueError('count component compile failed')
        for name in ('two_messages', 'three_messages', 'maximum_count_fits'):
            identity = 'contract::' + name
            result = api.run_process([binary, '--exact', identity, '--nocapture'],
                                     out / name, environment, 5, cwd=ROOT)
            stdout = (out / name / 'stdout.txt').read_text()
            good = (result['complete'] and result['exit_code'] == 0
                    and not result['remaining_group_terminated']
                    and ('test ' + identity + ' ... ok') in stdout
                    and '1 passed; 0 failed; 0 ignored' in stdout)
            report['cases'].append(dict(id=identity, process=result, passed=good))
            save()
        report['inputs_after'] = {str(p): api.file_identity(p) for p in paths}
        report['inputs_unchanged'] = report['inputs_before'] == report['inputs_after']
        report['execution_complete'] = len(report['cases']) == 3
        report['passed'] = (report['execution_complete'] and report['inputs_unchanged']
                            and all(row['passed'] for row in report['cases']))
    except Exception as error:
        report['error'] = repr(error)
    finally:
        save()
    print(json.dumps({k: report.get(k) for k in
                     ('passed', 'execution_complete', 'seconds', 'error')}), flush=True)
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
