#!/usr/bin/env python3
"""Actual Darwin XML ABI and isolated editor/libxml2 functional acceptance.

Use one external controlled-run owner: build for --action abi, runtime for
--action functional.  This driver never builds an editor or fetches a source
tree.  Functional inputs use the existing complete product/resource receipts;
the original stable test files are verified against their retained manifest.
"""
from __future__ import annotations

import argparse
from dataclasses import asdict
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import sys


ROOT = Path(__file__).resolve().parent.parent
FIXTURE = ROOT / 'tools/fixtures/xml-acceptance.el'
STABLE = ROOT / 'compatibility/stable-31.1/sources.json'
STABLE_FILES = ('test/src/xml-tests.el', 'test/lisp/xml-tests.el')
TEST_NAME = re.compile(r'^\(ert-deftest\s+([A-Za-z0-9_-]+)\s', re.MULTILINE)
CASE_PREFIX = 'FE-XML-CASE:'
FIELDS = {
    'xmlNode': ('_private', 'type', 'name', 'children', 'last', 'parent', 'next',
                'prev', 'doc', 'ns', 'content', 'properties', 'nsDef', 'psvi',
                'line', 'extra'),
    'xmlAttr': ('_private', 'type', 'name', 'children', 'last', 'parent', 'next',
                'prev', 'doc', 'ns', 'atype', 'psvi'),
    'xmlDoc': ('_private', 'type', 'name', 'children', 'last', 'parent', 'next',
               'prev', 'doc', 'compression', 'standalone', 'intSubset',
               'extSubset', 'oldNs', 'version', 'encoding', 'ids', 'refs',
               'URL', 'charset', 'dict', 'psvi', 'parseFlags', 'properties'),
}
CONSTANTS = ('LIBXML_VERSION', 'XML_ELEMENT_NODE', 'XML_TEXT_NODE',
             'XML_CDATA_SECTION_NODE', 'XML_COMMENT_NODE', 'XML_PARSE_NOERROR',
             'XML_PARSE_NOWARNING', 'XML_PARSE_NOBLANKS', 'XML_PARSE_NONET',
             'HTML_PARSE_RECOVER', 'HTML_PARSE_NOERROR', 'HTML_PARSE_NOWARNING',
             'HTML_PARSE_NOBLANKS', 'HTML_PARSE_NONET')
XML = '<root a="值"><leaf>你好&amp;世界</leaf><!--kept--></root>'


def load_tool(name, relative):
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


execution = load_tool('fe_xml_execution', 'tools/runtime-execution.py')
ert = load_tool('fe_xml_ert', 'tools/check-ert.py')


def identity(path):
    return dict(path=str(path), **execution.file_identity(path))


def complete(process):
    return (process['complete'] and process['exit_code'] == 0
            and not process['timed_out'] and not process['remaining_group_terminated'])


def plain_environment():
    return {key: value for key, value in os.environ.items()
            if key not in execution.FORBIDDEN_ENV and not key.startswith('FEMACS_')}


def c_probe():
    lines = ['#include <stddef.h>', '#include <stdio.h>', '#include <string.h>',
             '#include <libxml/tree.h>', '#include <libxml/parser.h>',
             '#include <libxml/HTMLparser.h>', 'int main(void) {']
    for kind, fields in FIELDS.items():
        lines.extend((f'printf("{kind}.size=%zu\\n", sizeof({kind}));',
                      f'printf("{kind}.align=%zu\\n", _Alignof({kind}));'))
        lines.extend(f'printf("{kind}.{field}=%zu\\n", offsetof({kind}, {field}));'
                     for field in fields)
    lines.extend(f'printf("{name}=%d\\n", (int){name});' for name in CONSTANTS)
    lines.extend([
        'xmlCheckVersion(LIBXML_VERSION);',
        f'const char *input = {json.dumps(XML, ensure_ascii=False)};',
        'xmlDocPtr doc = xmlReadMemory(input, (int)strlen(input), "", "utf-8",',
        '  XML_PARSE_NONET | XML_PARSE_NOBLANKS | XML_PARSE_NOERROR | XML_PARSE_NOWARNING);',
        'xmlNodePtr root = doc ? xmlDocGetRootElement(doc) : NULL;',
        'xmlAttrPtr attr = root ? root->properties : NULL;',
        'xmlNodePtr leaf = root ? root->children : NULL;',
        'int ok = root && attr && leaf && leaf->children && root->last',
        '  && root->type == XML_ELEMENT_NODE && !strcmp((char *)root->name, "root")',
        '  && root->doc == doc && attr->parent == root && attr->children',
        '  && attr->children->content && !strcmp((char *)attr->children->content, "值")',
        '  && leaf->parent == root && leaf->children->type == XML_TEXT_NODE',
        '  && leaf->children->content && !strcmp((char *)leaf->children->content, "你好&世界")',
        '  && root->last->type == XML_COMMENT_NODE && root->last->content',
        '  && !strcmp((char *)root->last->content, "kept");',
        'printf("real_dom=%d\\n", ok ? 1 : 0);',
        'if (doc) xmlFreeDoc(doc);', 'xmlCleanupParser();', 'return ok ? 0 : 1;', '}'])
    return '\n'.join(lines) + '\n'


def rust_probe(header):
    lines = ['#![allow(non_snake_case, dead_code)]',
             f'include!({json.dumps(str(header))});', 'use libxml2_ffi::*;',
             'use std::ffi::CStr;',
             'unsafe fn equal(p: *const u8, s: &str) -> bool {',
             ' !p.is_null() && unsafe { CStr::from_ptr(p.cast()).to_bytes() == s.as_bytes() }',
             '}', 'fn main() {']
    for kind, fields in FIELDS.items():
        lines.extend((f'println!("{kind}.size={{}}", std::mem::size_of::<{kind}>());',
                      f'println!("{kind}.align={{}}", std::mem::align_of::<{kind}>());'))
        lines.extend(f'println!("{kind}.{field}={{}}", std::mem::offset_of!({kind}, '
                     f'{"r#type" if field == "type" else field}));' for field in fields)
    lines.extend(f'println!("{name}={{}}", {name});' for name in CONSTANTS)
    lines.extend([
        'let ok = unsafe {', 'xmlCheckVersion(LIBXML_VERSION);',
        f'let input = {json.dumps(XML, ensure_ascii=False)}.as_bytes();',
        'let doc = xmlReadMemory(input.as_ptr().cast(), input.len() as i32,',
        ' c"".as_ptr(), c"utf-8".as_ptr(),',
        ' XML_PARSE_NONET | XML_PARSE_NOBLANKS | XML_PARSE_NOERROR | XML_PARSE_NOWARNING);',
        'let root = if doc.is_null() { std::ptr::null_mut() } else { xmlDocGetRootElement(doc) };',
        'let good = if root.is_null() { false } else {',
        ' let attr = (*root).properties; let leaf = (*root).children;',
        ' !attr.is_null() && !leaf.is_null() && !(*leaf).children.is_null()',
        ' && !(*root).last.is_null() && (*root).r#type == XML_ELEMENT_NODE',
        ' && equal((*root).name, "root") && (*root).doc == doc',
        ' && (*attr).parent == root && !(*attr).children.is_null()',
        ' && equal((*(*attr).children).content, "值") && (*leaf).parent == root',
        ' && (*(*leaf).children).r#type == XML_TEXT_NODE',
        ' && equal((*(*leaf).children).content, "你好&世界")',
        ' && (*(*root).last).r#type == XML_COMMENT_NODE',
        ' && equal((*(*root).last).content, "kept")',
        '};', 'if !doc.is_null() { xmlFreeDoc(doc); }', 'xmlCleanupParser(); good',
        '};', 'println!("real_dom={}", u8::from(ok));',
        'if !ok { std::process::exit(1); }', '}'])
    return '\n'.join(lines) + '\n'


def observations(path):
    result = {}
    for line in path.read_text(encoding='utf-8').splitlines():
        key, separator, value = line.partition('=')
        if not separator or key in result or not re.fullmatch(r'[A-Za-z0-9_.]+', key):
            raise ValueError('invalid or duplicate ABI observation')
        result[key] = int(value)
    expected = {f'{kind}.{field}' for kind, fields in FIELDS.items()
                for field in (*fields, 'size', 'align')} | set(CONSTANTS) | {'real_dom'}
    if set(result) != expected:
        raise ValueError('missing or extra ABI observations')
    return result


def abi(args, out):
    if not all((args.cc, args.rustc, args.sdk_root)):
        raise ValueError('ABI requires explicit installed cc, rustc and SDK')
    cc, rustc, sdk = (path.absolute() for path in (args.cc, args.rustc, args.sdk_root))
    header = ROOT / 'src/include/libxml2.rs'
    sdk_headers = [sdk / 'usr/include/libxml2/libxml' / name
                   for name in ('tree.h', 'parser.h', 'HTMLparser.h', 'xmlversion.h')]
    inputs = [Path(__file__), ROOT / 'tools/runtime-execution.py', header, cc, rustc,
              *sdk_headers]
    before = [identity(path) for path in inputs]
    execution.write_json(out / 'inputs.json', dict(files=before, scope='Darwin SDK ABI and actual system XML DOM'))
    c, rust = out / 'xml-system.c', out / 'xml-owned.rs'
    c.write_text(c_probe(), encoding='utf-8')
    rust.write_text(rust_probe(header), encoding='utf-8')
    commands = {
        'C': [cc, '-arch', 'arm64', '-isysroot', sdk, '-I', sdk / 'usr/include/libxml2', '-std=c11',
              '-Wall', '-Wextra', '-Werror', '-MD', '-MF', out / 'c-dependencies.d',
              c, '-lxml2', '-o', out / 'xml-c'],
        'Rust': [rustc, '--target', 'aarch64-apple-darwin', '--edition=2024', '--cfg', 'feature="have_libxml2"',
                 rust, '-lxml2', '-C', 'link-arg=-isysroot', '-C', f'link-arg={sdk}',
                 '-o', out / 'xml-rust'],
    }
    rows, values = [], {}
    for name, command in commands.items():
        process = execution.run_process(command, out / (name + '-compile'), plain_environment(), 120, cwd=out)
        row = dict(id=name, compile=process, passed=False)
        if complete(process):
            program = out / ('xml-c' if name == 'C' else 'xml-rust')
            run = execution.run_process([program], out / (name + '-run'), plain_environment(), 30, cwd=out)
            row['run'] = run
            if complete(run):
                values[name] = observations(out / (name + '-run/stdout.txt'))
                row['observations'] = values[name]
                row['passed'] = values[name]['real_dom'] == 1
        rows.append(row)
    # Bind actual transitive SDK inputs as well as the four primary headers.
    dependencies = []
    dep = out / 'c-dependencies.d'
    if dep.exists():
        text = dep.read_text().replace('\\\n', ' ')
        for token in shlex.split(text.split(':', 1)[1]):
            path = Path(token)
            dependencies.append(identity(path if path.is_absolute() else out / path))
    same = [identity(path) for path in inputs] == before
    equal = len(values) == 2 and values['C'] == values['Rust']
    result = dict(action='abi', passed=same and equal and all(row['passed'] for row in rows),
                  inputs_unchanged=same, independent_observations_equal=equal,
                  cases=rows, actual_C_dependencies=dependencies,
                  scope='SDK C and actual owned Rust types/layout/constants plus real system parser traversal/free; not whole-editor acceptance')
    execution.write_json(out / 'results.json', result)
    return result


def functional(args, out):
    if not args.products:
        raise ValueError('functional acceptance requires an explicit complete product list')
    products_path = args.products.absolute()
    products = execution.load_products(products_path)
    if ({product['kind'] for product in products} != {'reference', 'terminal', 'cocoa'}
            or len([p for p in products if p['kind'] == 'reference']) != 1):
        raise ValueError('include the installed reference and both actual macOS products')
    baseline_legs = {(p['kind'], p['compiler_leg']) for p in products
                     if p['kind'] != 'reference' and not p['native']
                     and not p['observer'] and p['collector'] == 'off'}
    if baseline_legs != {(kind, leg) for kind in ('terminal', 'cocoa')
                         for leg in ('normal', 'checking')}:
        raise ValueError('include both normal/checking baseline legs of both products')
    manifest = execution.bounded_json(STABLE)
    if manifest['source']['release'] != '31.1':
        raise ValueError('explicit stable corpus release changed; review scope before running')
    entries = {entry['path']: entry for entry in manifest['entries']}
    sources = [('owned-XML', FIXTURE, None)]
    for logical in STABLE_FILES:
        entry = entries[logical]
        relative = Path(entry['source_path'])
        if relative.is_absolute() or '..' in relative.parts:
            raise ValueError('stable source escapes repository')
        path = ROOT / relative
        if path.stat().st_size != entry['size'] or execution.digest(path) != entry['sha256']:
            raise ValueError('stable XML source identity changed')
        sources.append((logical.replace('/', '-'), path, entry))
    harness = [Path(__file__), ROOT / 'tools/runtime-execution.py',
               ROOT / 'tools/check-ert.py', STABLE, products_path,
               *(path for _, path, _ in sources)]
    before = [identity(path) for path in harness]
    products_before = {p['id']: execution.product_identity(p, resources=True) for p in products}
    staged = []
    for label, path, _ in sources:
        target = out / (label + '.el')
        execution.copy_owned_data(path, target)
        names = TEST_NAME.findall(target.read_text(encoding='utf-8'))
        if not names or len(set(names)) != len(names):
            raise ValueError('fixture needs distinct explicit XML test IDs')
        if label == 'owned-XML' and len(names) != 13:
            raise ValueError('owned XML functional scope changed; review expected cases')
        staged.append((label, target, sorted(names)))
    execution.write_json(out / 'inputs.json', dict(files=before, products=products_before,
        staged=[dict(file=label, sha256=execution.digest(path), declared_ids=names)
                for label, path, names in staged],
        scope='real XML/HTML editor integration and both unchanged stable XML ERT files; no tag filters or accepted skips'))
    rows, discoveries = [], []
    for product in products:
        for label, fixture, names in staged:
            discovery = out / product['id'] / label / 'discovery'
            discovery.mkdir(parents=True)
            env = execution.owned_environment(discovery, product)
            env['FEMACS_XML_TEST_DIR'] = str(discovery / 'out')
            command = [product['program'], '--dump-file=' + product['dump'], '-Q', '--batch',
                       '-l', fixture]
            expression = '(dolist (test (ert-select-tests t t)) (princ (format "' + CASE_PREFIX + '%s\\n" (ert-test-name test))))'
            process = execution.run_process([*command, '--eval', expression], discovery, env, args.timeout, cwd=discovery)
            actual = [line[len(CASE_PREFIX):] for line in (discovery / 'stdout.txt').read_text(encoding='utf-8').splitlines()
                      if line.startswith(CASE_PREFIX)]
            discovered = complete(process) and sorted(actual) == names and len(set(actual)) == len(actual)
            discoveries.append(dict(product=product['id'], file=label, expected_ids=names,
                                    actual_ids=actual, process=process, passed=discovered))
            for name in names:
                case = out / product['id'] / label / name
                if not discovered:
                    rows.append(dict(product=product['id'], file=label, id=name,
                                     passed=False, status='not-run', reason='actual ERT discovery incomplete'))
                    continue
                case.mkdir(parents=True)
                env = execution.owned_environment(case, product)
                env['FEMACS_XML_TEST_DIR'] = str(case / 'out')
                process = execution.run_process([*command, '--eval',
                    "(ert-run-tests-batch-and-exit '(member " + name + '))'],
                    case, env, args.timeout, cwd=case)
                text = '\n'.join((case / (stream + '.txt')).read_text(encoding='utf-8', errors='replace')
                                 for stream in ('stdout', 'stderr'))
                native = (ert.parse_result(text, process['exit_code'], process['seconds'])
                          if process['exit_code'] is not None else
                          ert.RunResult(status='crashed', note=process['status'],
                                        elapsed_seconds=process['seconds']))
                passed = (complete(process) and native.status == 'ok' and native.ran == 1
                          and native.expected == 1 and native.unexpected == 0
                          and native.results == {name: 'passed'})
                row = dict(product=product['id'], file=label, id=name, passed=passed,
                           process=process, native_ERT=asdict(native))
                execution.write_json(case / 'result.json', row)
                rows.append(row)
            execution.write_json(out / 'partial.json', dict(discoveries=discoveries, cases=rows, complete=False))
    products_after = {p['id']: execution.product_identity(p, resources=True) for p in products}
    same = ([identity(path) for path in harness] == before and products_before == products_after)
    # Recheck the canonical complete product/resource receipts after execution.
    execution.load_products(products_path)
    result = dict(action='functional', passed=same and bool(rows) and all(row['passed'] for row in rows)
                  and all(row['passed'] for row in discoveries), complete=True,
                  inputs_products_resources_unchanged=same, discoveries=discoveries, cases=rows,
                  scope='listed current products, actual library DOM/GC/region/file workflow and both stable XML ERT files; not complete compatibility or performance acceptance')
    execution.write_json(out / 'results.json', result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--action', required=True, choices=('abi', 'functional'))
    parser.add_argument('--out', required=True, type=Path)
    parser.add_argument('--cc', type=Path)
    parser.add_argument('--rustc', type=Path)
    parser.add_argument('--sdk-root', type=Path)
    parser.add_argument('--products', type=Path)
    parser.add_argument('--timeout', type=float, default=120)
    args = parser.parse_args()
    out = args.out.absolute()
    if out.exists() or out.resolve() != out or not out.is_relative_to(ROOT / 'build'):
        parser.error('fresh owned build output required')
    if args.timeout <= 0 or not args.timeout < float('inf'):
        parser.error('positive finite per-case timeout required')
    out.mkdir(parents=True)
    result = abi(args, out) if args.action == 'abi' else functional(args, out)
    print(json.dumps(dict(action=args.action, passed=result['passed'], results=str(out / 'results.json'))))
    return 0 if result['passed'] else 1


if __name__ == '__main__':
    sys.exit(main())
