#!/usr/bin/env python3
"""Report which duplicate declarations may be withdrawn from a unit (PORTING.md P-323).

`tools/check-mirror-dupes.py` matches a name against the include graph.  That is enough to see
a duplicate, and not enough to withdraw one: the mirror's declaration may sit behind a `cfg`
the unit does not have (measured: 17 names in 16 units, status 4.995), and the unit's copy may
spell a different type than the mirror's (measured: `code_convert_string`'s `c_int` against the
mirror's `bool`).  This tool answers the withdrawal question per hit:

    python3 tools/check-decl-withdraw.py            # per-hit verdict, with the reason
    python3 tools/check-decl-withdraw.py --safe     # only the hits that may be withdrawn

A hit is `safe` only when both hold:

  * **visible**: neither the mirror's declaration nor any module enclosing it carries a `#[cfg]`.
    The tool does not try to evaluate cfg against a unit's configuration; a gate anywhere above
    the declaration means "not proven visible here", and the copy stays.
  * **same signature**: the two declarations' parameter types and return type are the same after
    whitespace and `pub` are normalised.  Parameter names are not compared - C keeps them only
    for readers.

Anything else is `keep`, with the reason printed.  The tool reads; it never edits.
"""
import argparse
import re
import os
import subprocess
import sys

DECL = re.compile(r'(?:pub )?fn ([a-z_]\w*)\s*\(([^;]*?)\)\s*(?:->\s*([^;]+?))?;')


def dupes():
    out = subprocess.run([sys.executable, 'tools/check-mirror-dupes.py'],
                         capture_output=True, text=True).stdout
    hits = []
    for line in out.splitlines():
        if not line.startswith('FAIL '):
            continue
        rest = line[len('FAIL '):]
        path, tail = rest.split(': ', 1)
        mirror = tail.split(' is already declared by ')[1].split(' ')[0]
        hits.append((path, mirror, tail.split(' is already declared by ')[0]))
    return hits


def signature(params, ret):
    parts = []
    depth = 0
    cur = ''
    for ch in params:
        if ch in '(<[':
            depth += 1
        elif ch in ')>]':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(cur)
            cur = ''
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    types = []
    for part in parts:
        part = part.strip()
        if not part:
            continue
        # drop a parameter name: the last `name:` at depth 0
        m = re.match(r'(?:mut\s+)?(?:r#)?[a-z_]\w*\s*:\s*(.+)$', part, re.S)
        types.append(re.sub(r'\s+', ' ', (m.group(1) if m else part)).strip())
    r = re.sub(r'\s+', ' ', (ret or '()')).strip()
    return '(' + ', '.join(types) + ') -> ' + r



def direct_includes(unit):
    """The mirrors a unit `include!`s itself, resolved to repository-relative paths."""
    out = set()
    unit_dir = os.path.dirname(unit)
    for m in re.finditer(r'include!\("([^"]+)"\)', open(unit, encoding='utf-8', errors='ignore').read()):
        out.add(os.path.normpath(os.path.join(unit_dir, m.group(1))))
    return out

def decls_in(text):
    found = {}
    for m in DECL.finditer(text):
        found.setdefault(m.group(1), signature(m.group(2), m.group(3)))
    return found


def gated_decls(path):
    """name -> True when the declaration or an enclosing module carries a #[cfg]."""
    gated = {}
    stack = []
    pending = []
    for line in open(path, encoding='utf-8', errors='ignore'):
        st = line.strip()
        if st.startswith('#['):
            pending.append(st)
            continue
        if re.match(r'(?:pub )?mod \w+ \{', st):
            stack.append(any('cfg' in a for a in pending))
            pending = []
            continue
        if st == '}':
            if stack:
                stack.pop()
            pending = []
            continue
        m = re.match(r'(?:pub )?fn ([a-z_]\w*)\s*\(', st)
        if m:
            gated[m.group(1)] = any(stack) or any('cfg' in a for a in pending)
            pending = []
            continue
        if st and not st.startswith('//'):
            pending = []
    return gated


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--safe', action='store_true', help='print only withdrawable hits')
    args = ap.parse_args()
    hits = dupes()
    cache = {}
    safe = 0
    for unit, mirror, name in hits:
        if mirror not in cache:
            cache[mirror] = (decls_in(open(mirror, encoding='utf-8').read()), gated_decls(mirror))
        mirror_decls, mirror_gated = cache[mirror]
        unit_text = open(unit, encoding='utf-8').read()
        unit_decls = decls_in(unit_text)
        reason = None
        if mirror not in direct_includes(unit):
            reason = 'the unit does not include that mirror'
        elif mirror_gated.get(name):
            reason = 'mirror declaration carries a cfg'
        elif name not in mirror_decls:
            reason = 'mirror declaration not found'
        elif name not in unit_decls:
            reason = 'unit declaration not found'
        elif mirror_decls[name] != unit_decls[name]:
            reason = f'signature differs: unit {unit_decls[name]} vs mirror {mirror_decls[name]}'
        if reason is None:
            safe += 1
            if args.safe:
                print(f'{unit}\t{mirror}\t{name}')
        elif not args.safe:
            print(f'keep\t{unit}\t{name}\t{reason}')
    if not args.safe:
        print(f'{len(hits)} duplicate(s), {safe} safely withdrawable, {len(hits) - safe} keep')
    return 0


if __name__ == '__main__':
    sys.exit(main())
