#!/usr/bin/env python3
"""Protect the actual numeric Registry in isolation; not VM acceptance.

Run through the build controller. The generated Rust uses the source module
unchanged, with explicit 64-bit Lisp/tag and bitmap primitive substitutes.
Real product/lifecycle integration is a separate prerequisite, not simulated here.
"""
from pathlib import Path
import argparse
import hashlib
import json
import re
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


TESTS = r'''
#[cfg(test)]
mod protection {
    use super::*;
    // Approximately 4 MiB: initialize in static storage, never on a test stack.
    static mut REGISTRY: Registry = Registry::new();

    unsafe fn registry() -> &'static mut Registry { unsafe { &mut *(&raw mut REGISTRY) } }
    fn reset(r: &Registry) {
        assert_eq!(r.len, 0);
        assert_eq!(r.frozen_len, 0);
        assert!(!r.valid);
        assert!(!r.overflowed);
        assert!(r.index.iter().all(|&slot| slot == 0));
    }

    #[test]
    fn initial_static_registry_is_empty() {
        static INITIAL: Registry = Registry::new();
        let r = &INITIAL;
        reset(r);
        assert!(r.entries.iter().all(|&object| object == lisp_h_Qnil()));
    }

    #[test]
    fn populated_freeze_duplicate_and_clear() {
        let r = unsafe { registry() };
        r.clear();
        for object in [1, 7, 63, 1024, u64::MAX] { assert!(r.insert(object)); }
        assert_eq!(r.len, 5);
        assert!(r.index.iter().any(|&slot| slot != 0));
        r.freeze(); r.valid = true;
        assert_eq!(r.frozen_len, 5);
        assert!(r.insert(63));
        assert_eq!(r.len, 5);
        r.clear();
        reset(r);
        // Entries are not roots when length is zero; clear need not erase them.
        assert_eq!(&r.entries[..5], &[1, 7, 63, 1024, u64::MAX]);
    }

    #[test]
    fn repeated_empty_clear_still_resets_all_metadata() {
        let r = unsafe { registry() };
        r.clear();
        for _ in 0..4 {
            r.valid = true;
            r.overflowed = true;
            r.frozen_len = 3;
            r.clear();
            reset(r);
        }
    }

    #[test]
    fn reinsertion_after_populated_and_empty_clear() {
        let r = unsafe { registry() };
        r.clear();
        for _ in 0..3 {
            assert!(r.insert(17)); assert!(r.insert(29));
            assert!(r.insert(17)); assert_eq!(r.len, 2);
            assert_eq!(&r.entries[..2], &[17, 29]);
            r.freeze(); assert_eq!(r.frozen_len, 2);
            r.clear(); reset(r);
            r.clear(); reset(r);
        }
    }

    #[test]
    fn full_capacity_failure_clear_and_reuse() {
        let r = unsafe { registry() };
        r.clear();
        // Sequential values at the real 0.5 table load, no adversarial collisions.
        for object in 1..=REGISTRY_CAPACITY as u64 { assert!(r.insert(object)); }
        assert_eq!(r.len, REGISTRY_CAPACITY);
        r.freeze(); r.valid = true;
        assert!(r.insert(REGISTRY_CAPACITY as u64)); // Existing item at capacity.
        assert!(!r.insert(REGISTRY_CAPACITY as u64 + 1));
        assert!(r.overflowed); assert!(!r.valid);
        assert_eq!(r.frozen_len, REGISTRY_CAPACITY);
        r.clear(); reset(r);
        assert!(r.insert(REGISTRY_CAPACITY as u64 + 1));
        assert_eq!(r.len, 1);
        assert!(!r.overflowed);
        r.clear(); reset(r);
    }
}
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    out = args.out.resolve()
    if out.exists():
        parser.error('Output exists; read its original result before retrying')
    reference = ROOT / 'emacs'
    if out == reference or reference in out.parents:
        parser.error('Output must not be inside the readonly reference')
    source = ROOT / 'src/runtime_gc_collector.rs'
    text = source.read_text()
    # Exact current module boundaries; fail closed if the source structure changes.
    start = text.index('    pub(super) mod numeric {')
    end = text.index('\n    struct State {', start)
    module = text[start:end]
    assert module.rstrip().endswith('}')
    module = module.rstrip()[:-1] + TESTS + '\n}\n'
    constants = []
    for name in ('REGISTRY_CAPACITY', 'INDEX_CAPACITY', 'MAX_PROBES'):
        matches = re.findall(r'^    const ' + name + r': usize = [0-9]+;', text, re.M)
        assert len(matches) == 1, name
        constants.append(matches[0].strip())
    prelude = '\n'.join(constants) + '''
#[allow(non_camel_case_types)] type Lisp_Object = u64;
#[allow(non_camel_case_types)] type bits_word = u64;
const BITS_PER_BITS_WORD: usize = 64;
const BITS_WORD_MAX: u64 = u64::MAX;
#[allow(non_snake_case)] const fn lisp_h_Qnil() -> u64 { 0 }
#[allow(non_snake_case)] fn XLI(object: u64) -> u64 { object }
'''
    generated = prelude + '\nmod gc_collector {\nuse super::*;\n' + module + '\n}\n'
    out.mkdir(parents=True)
    (out / 'registry.rs').write_text(generated)
    started = time.monotonic()
    report = dict(passed=False, execution_complete=False,
                  collector_sha256=digest(source), driver_sha256=digest(Path(__file__)),
                  generated_sha256=digest(out / 'registry.rs'),
                  boundary='Actual numeric source with explicit u64 Lisp/tag/bitmap substitutes; isolated component only, no real GC roots or VM acceptance',
                  scale='Five serial tests, one 262144-entry fill, real 524288-slot index; bounded 1024 probes per insertion and static registry storage',
                  budget=dict(compile_seconds=60, execution_seconds=20, whole_seconds=100))
    def save():
        report['seconds'] = time.monotonic() - started
        (out / 'results.json').write_text(json.dumps(report, indent=2) + '\n')
    save()
    try:
        report['rustc_version'] = subprocess.run(['rustc', '--version'], check=True,
            capture_output=True, text=True, timeout=5).stdout.strip()
        for phase, argv, timeout in (
            ('compile', ['rustc', '--edition=2024', '--test', '-C', 'opt-level=1',
                         '-C', 'overflow-checks=on', 'registry.rs', '-o', 'registry-test'], 60),
            ('execute', [str(out / 'registry-test'), '--test-threads=1', '--nocapture'], 20)):
            begin = time.monotonic()
            with (out / (phase + '.stdout')).open('w') as stdout, (out / (phase + '.stderr')).open('w') as stderr:
                result = subprocess.run(argv, cwd=out, stdout=stdout, stderr=stderr, timeout=timeout)
            report[phase] = dict(exit_code=result.returncode, seconds=time.monotonic()-begin)
            save()
            assert result.returncode == 0, phase
        output = (out / 'execute.stdout').read_text()
        assert 'test result: ok. 5 passed; 0 failed;' in output
        report['source_unchanged'] = report['collector_sha256'] == digest(source)
        report['driver_unchanged'] = report['driver_sha256'] == digest(Path(__file__))
        report.update(execution_complete=True, passed=report['source_unchanged'] and report['driver_unchanged'])
        save()
    except BaseException as error:
        report.update(error_type=type(error).__name__, error=str(error)[:1000])
        save()
        raise
    print(json.dumps(report))


if __name__ == '__main__':
    main()
