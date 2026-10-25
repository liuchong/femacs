#!/usr/bin/env python3
"""Generate JNI audit inputs from the actual ordered NDK and owned mirror declarations."""
import re
import sys
from pathlib import Path


def jni(header: Path, mirror: Path, out: Path) -> None:
    source = header.read_text().splitlines()
    start = next(i for i, line in enumerate(source) if line.startswith('struct JNINativeInterface'))
    end = next(i for i in range(start + 1, len(source)) if source[i].startswith('};'))
    body = re.sub(r'/\*.*?\*/', ' ', '\n'.join(source[start + 1:end]), flags=re.S)
    names = []
    for declaration in body.split(';'):
        text = ' '.join(declaration.split())
        if not text:
            continue
        match = re.match(r'^[A-Za-z_][\w ]*\**\s*\(\s*\*\s*([A-Za-z_]\w*)\s*\)', text)
        reserved = re.fullmatch(r'void\s*\*\s*(reserved\d+)', text)
        if not (match or reserved):
            raise ValueError('unparsed JNI header field: ' + text)
        names.append((match or reserved).group(1))
    rust = mirror.read_text()
    block = rust.split('pub struct JNINativeInterface', 1)[1].split('\n}', 1)[0]
    fields = re.findall(r'^  pub ([A-Za-z_]\w*):', block, re.M)
    if not names or names != fields:
        raise ValueError('NDK and mirror JNI field membership/order differ')
    lines = ['#include <jni.h>', '#include <stddef.h>']
    lines += [f'_Static_assert(offsetof(struct JNINativeInterface, {name}) == {8 * i}, "{name}");' for i, name in enumerate(names)]
    lines += [f'_Static_assert(sizeof(struct JNINativeInterface) == {8 * len(names)}, "size");', 'int main(void) { return 0; }']
    with (out / 'verify.c').open('x') as file:
        file.write('\n'.join(lines) + '\n')
    with (out / 'rust-jni.rs').open('x') as file:
        file.write('#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables)]\nuse std::ffi::{c_char,c_int,c_void};\n')
        import json
        file.write(f'include!({json.dumps(str(mirror.resolve()))});\nfn main() {{}}\n')
    print(f'JNI ordered entries: {len(names)}; C offset assertions: {len(names) + 1}')


if __name__ == '__main__':
    if len(sys.argv) != 5 or sys.argv[1] != 'jni':
        sys.exit('usage: probe-inputs.py jni HEADER MIRROR OWNED_OUT')
    jni(*(Path(arg) for arg in sys.argv[2:]))
