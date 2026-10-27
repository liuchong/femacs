"""Protect the production live-resize title formatter after native UI acceptance.

Compile the actual Swift helper with literal UTF-8 expectations. This is a
formatting component test, not a replacement for native live-resize acceptance.
Run through controlled-run's build profile with a fresh owned build output.
"""
from pathlib import Path
import argparse, hashlib, json, re, subprocess, time

ROOT = Path(__file__).resolve().parent.parent

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', required=True, type=Path)
    args = parser.parse_args()
    out = args.out.absolute()
    if (not out.is_relative_to(ROOT / 'build') or out == ROOT / 'build'
            or any(p.is_symlink() for p in [out, *out.parents])):
        raise ValueError('output must be a distinct nonsymlink owned build child')
    out.mkdir(parents=True, exist_ok=False)
    source = ROOT / 'platform/macos/nsterm.swift'
    binding = digest(source)
    text = source.read_text()
    match = re.search(r'private func ns_window_resize_title\(', text)
    if not match:
        raise ValueError('production title helper missing')
    opening = text.index('{', match.start())
    end, depth = opening + 1, 1
    while depth:
        if text[end] == '{': depth += 1
        if text[end] == '}': depth -= 1
        end += 1
    helper = text[match.start():end]
    assert '@_silgen_name("esprintf")' not in text
    cases = [
        ('ascii', 'Femacs', 65, 28, 'Femacs  —  (65 × 28)'),
        ('unicode', '图形界面🙂', 99, 40, '图形界面🙂  —  (99 × 40)'),
        ('combining-quotes', "e\u0301 `quote'", 1, 1, "e\u0301 `quote'  —  (1 × 1)"),
        ('int32-limits', 'limits', -2147483648, 2147483647,
         'limits  —  (-2147483648 × 2147483647)'),
        ('empty', '', 0, 0, '  —  (0 × 0)'),
        ('long-utf8', '界' * 4096, 123, 456, '界' * 4096 + '  —  (123 × 456)'),
    ]
    code = ['import Foundation', helper]
    for name, title, columns, rows, expected in cases:
        literal = lambda value: json.dumps(value, ensure_ascii=False)
        code += ['do {', '  let start = Date().timeIntervalSinceReferenceDate',
                 '  let actual = ' + literal(title) + '.withCString { old in',
                 f'    ns_window_resize_title(old, columns: {columns}, rows: {rows})',
                 '  }', '  precondition(Array(actual.utf8) == Array(' + literal(expected) + '.utf8))',
                 '  let row: [String: Any] = ["case": ' + literal(name) + ', "passed": true,',
                 '    "seconds": Date().timeIntervalSinceReferenceDate - start]',
                 '  print(String(data: try JSONSerialization.data(withJSONObject: row), encoding: .utf8)!)', '}']
    (out / 'probe.swift').write_text('\n'.join(code) + '\n')
    report = dict(schema='femacs-live-resize-title-components-v1',
                  source_sha256=binding, passed=False, cases=[])
    try:
        commands = [('compile', ['swiftc', '-O', str(out / 'probe.swift'), '-o', str(out / 'probe')]),
                    ('execute', [str(out / 'probe')])]
        for name, argv in commands:
            start = time.monotonic()
            with (out / (name + '.stdout')).open('xb') as stdout, (out / (name + '.stderr')).open('xb') as stderr:
                result = subprocess.run(argv, cwd=ROOT, stdout=stdout, stderr=stderr)
            report[name] = dict(exit_code=result.returncode, seconds=time.monotonic() - start)
            if result.returncode:
                return 1
        report['cases'] = [json.loads(line) for line in (out / 'execute.stdout').read_text().splitlines()]
        report['source_unchanged'] = binding == digest(source)
        report['passed'] = (len(report['cases']) == len(cases)
                            and {x['case'] for x in report['cases']} == {x[0] for x in cases}
                            and all(x['passed'] for x in report['cases']) and report['source_unchanged'])
        return 0 if report['passed'] else 1
    finally:
        (out / 'results.json').write_text(json.dumps(report, indent=2) + '\n')
        print(json.dumps(dict(passed=report['passed'], cases=len(report['cases']))))

if __name__ == '__main__':
    raise SystemExit(main())
