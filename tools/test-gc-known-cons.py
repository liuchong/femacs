#!/usr/bin/env python3
"""Protect the real sweep hooks with explicit isolated primitive substitutes.

Run after real product integration, through the build controller. This extracts
the production hooks, bitmap helper, counter method and cons-block definition;
it does not reproduce a marker, memory tree, Lisp runtime or owner-thread check.
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


def item(text, declaration):
    """Current selected items contain no brace-bearing string literals."""
    start = text.index(declaration)
    opening = text.index('{', start)
    depth = 1
    end = opening + 1
    while depth:
        depth += (text[end] == '{') - (text[end] == '}')
        end += 1
    return text[start:end]


TESTS = r'''
#[cfg(test)]
mod protection {
    use super::*;
    struct Timer(&'static str, std::time::Instant);
    impl Drop for Timer {
        fn drop(&mut self) {
            println!("CASE_TIME\t{}\t{}", self.0, self.1.elapsed().as_nanos());
        }
    }
    fn begin(name: &'static str) -> Timer {
        unsafe { OWNER = true; STATE = State::new(); }
        Timer(name, std::time::Instant::now())
    }
    fn block() -> Box<cons_block> {
        Box::new(cons_block {
            conses: [Lisp_Cons { u: ConsU { s: ConsS { car: dead_object(), cdr: 37 } } }; CONS_BLOCK_SIZE],
            gcmarkbits: [0; 32], oldbits: [0; 32], dirtybits: [0; 32],
            raw_tainted: 17, next: std::ptr::null_mut(),
        })
    }
    const INDICES: [usize; 4] = [0, 63, 64, CONS_BLOCK_SIZE - 1];

    #[test]
    fn disabled_owner_preserves_metadata_and_counters() {
        let _timer = begin("disabled_owner_preserves_metadata_and_counters");
        let mut b = block();
        b.oldbits.fill(0x5555555555555555); b.dirtybits.fill(u64::MAX);
        let old = b.oldbits; let dirty = b.dirtybits;
        unsafe {
            OWNER = false;
            for i in INDICES { gc_collector::survivor(&mut *b, i); gc_collector::swept_dead(&mut *b, i); }
            assert_eq!(state().totals, [0; 24]);
        }
        assert_eq!(b.oldbits, old); assert_eq!(b.dirtybits, dirty);
        assert_eq!(b.raw_tainted, 17);
    }

    #[test]
    fn partial_word_survivors_promote_once_without_clearing_dirty_bits() {
        let _timer = begin("partial_word_survivors_promote_once_without_clearing_dirty_bits");
        let mut b = block(); b.dirtybits.fill(u64::MAX); b.gcmarkbits.fill(0xaaaaaaaaaaaaaaaa);
        let dirty = b.dirtybits; let marks = b.gcmarkbits;
        let mut expected = [0; 32];
        unsafe {
            for i in INDICES {
                gc_collector::survivor(&mut *b, i);
                gc_collector::survivor(&mut *b, i);
                expected[i / 64] |= 1u64 << (i % 64);
            }
            assert_eq!(state().totals[7], 4); assert_eq!(state().totals[8], 0);
        }
        assert_eq!(b.oldbits, expected); assert_eq!(b.dirtybits, dirty);
        assert_eq!(b.gcmarkbits, marks); assert_eq!(b.conses[0].u.s.car, dead_object());
    }

    #[test]
    fn dead_cells_clear_only_their_age_and_count_only_newly_dead_young() {
        let _timer = begin("dead_cells_clear_only_their_age_and_count_only_newly_dead_young");
        let mut b = block(); let mut expected_dead = 0;
        unsafe {
            for i in INDICES {
                let word = i / 64; let mask = 1u64 << (i % 64);
                for old in [false, true] {
                    for dirty in [false, true] {
                        for already_dead in [false, true] {
                            b.oldbits.fill(u64::MAX); b.dirtybits.fill(u64::MAX);
                            if !old { b.oldbits[word] &= !mask; }
                            if !dirty { b.dirtybits[word] &= !mask; }
                            b.conses[i].u.s.car = if already_dead { dead_object() } else { 99 };
                            let mut expected_old = b.oldbits; expected_old[word] &= !mask;
                            let mut expected_dirty = b.dirtybits; expected_dirty[word] &= !mask;
                            let car = b.conses[i].u.s.car;
                            gc_collector::swept_dead(&mut *b, i);
                            if !old && !already_dead { expected_dead += 1; }
                            assert_eq!(state().totals[8], expected_dead);
                            assert_eq!(b.oldbits, expected_old); assert_eq!(b.dirtybits, expected_dirty);
                            assert_eq!(b.conses[i].u.s.car, car); assert_eq!(b.conses[i].u.s.cdr, 37);
                            // The real caller installs dead_object after the hook.
                            b.conses[i].u.s.car = dead_object();
                            gc_collector::swept_dead(&mut *b, i);
                            assert_eq!(state().totals[8], expected_dead);
                            gc_collector::survivor(&mut *b, i);
                            assert_ne!(b.oldbits[word] & mask, 0); assert_eq!(b.dirtybits[word] & mask, 0);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn exhausted_counter_still_updates_age_and_invalidates_cycle() {
        let _timer = begin("exhausted_counter_still_updates_age_and_invalidates_cycle");
        let mut b = block(); b.dirtybits[0] = u64::MAX;
        unsafe {
            state().totals[7] = u64::MAX;
            gc_collector::survivor(&mut *b, 0);
            assert_eq!(state().totals[7], u64::MAX); assert_eq!(b.oldbits[0], 1);
            assert!(state().counters_exhausted); assert!(!state().registry.valid);
            assert!(!state().cycle_valid); assert_eq!(state().invalid_reason, 11);
            state().totals[8] = u64::MAX; b.conses[1].u.s.car = 99;
            gc_collector::swept_dead(&mut *b, 1);
            assert_eq!(state().totals[8], u64::MAX); assert_eq!(b.dirtybits[0], u64::MAX & !2);
        }
    }
}
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    out = args.out.resolve()
    if out.exists() or out == ROOT or out.is_relative_to(ROOT / 'emacs'):
        parser.error('Use a fresh owned output outside the readonly reference')
    source = ROOT / 'src/runtime_gc_collector.rs'
    alloc = ROOT / 'src/alloc.rs'
    text = source.read_text()
    hooks = '\n'.join(item(text, '    pub(super) unsafe fn ' + name + '(')
                      for name in ('survivor', 'swept_dead'))
    helper = item(text, '        pub fn clear_age(')
    count = item(text, '        fn count(')
    block = item(alloc.read_text(), 'pub struct cons_block\n')
    prelude = '''
#![allow(non_camel_case_types, dead_code)]
type Lisp_Object = u64;
type bits_word = u64;
const CONS_BLOCK_SIZE: usize = 1998;
const BITS_PER_BITS_WORD: usize = 64;
#[derive(Clone, Copy)] struct ConsS { car: Lisp_Object, cdr: Lisp_Object }
#[derive(Clone, Copy)] struct ConsU { s: ConsS }
#[derive(Clone, Copy)] struct Lisp_Cons { u: ConsU }
const fn dead_object() -> u64 { u64::MAX }
struct RegistryState { valid: bool }
struct State { totals: [u64; 24], registry: RegistryState, counters_exhausted: bool,
               cycle_valid: bool, invalid_reason: u32 }
impl State {
    const fn new() -> Self { Self { totals: [0; 24], registry: RegistryState { valid: true },
        counters_exhausted: false, cycle_valid: true, invalid_reason: 0 } }
'''
    generated = prelude + count + '''
}
static mut STATE: State = State::new();
static mut OWNER: bool = true;
unsafe fn state() -> &'static mut State { unsafe { &mut *(&raw mut STATE) } }
unsafe fn young_owner() -> bool { unsafe { OWNER } }
''' + block + '\nmod gc_collector {\nuse super::*;\nmod numeric {use super::*;\n' + helper + '\n}\n' + hooks + '\n}\n' + TESTS
    out.mkdir(parents=True)
    (out / 'hooks.rs').write_text(generated)
    started = time.monotonic()
    report = dict(passed=False, execution_complete=False,
                  source_sha256={str(p.relative_to(ROOT)): digest(p) for p in (source, alloc)},
                  driver_sha256=digest(Path(__file__)), generated_sha256=digest(out / 'hooks.rs'),
                  boundary='Unmodified source hooks/counter/helper/block with explicit u64 object, cons-field and owner/state substitutes; isolated component, not ABI or real GC acceptance',
                  budget=dict(whole_seconds=100, compile_seconds=60, execute_seconds=20,
                              scale='Four serial tests, four boundary indices and eight dead-cell states; 1998-cell block, no tree/VM'))
    def save():
        report['seconds'] = time.monotonic() - started
        (out / 'results.json').write_text(json.dumps(report, indent=2) + '\n')
    save()
    try:
        for phase, argv, timeout in (
            ('compile', ['rustc', '--edition=2024', '--test', '--cfg', 'feature="femacs_gc_generational"',
                         '-C', 'opt-level=1', '-C', 'overflow-checks=on', 'hooks.rs', '-o', 'hooks-test'], 60),
            ('execute', [str(out / 'hooks-test'), '--test-threads=1', '--nocapture'], 20)):
            begin = time.monotonic()
            with (out / (phase + '.stdout')).open('w') as stdout, (out / (phase + '.stderr')).open('w') as stderr:
                result = subprocess.run(argv, cwd=out, stdout=stdout, stderr=stderr, timeout=timeout)
            report[phase] = dict(exit_code=result.returncode, seconds=time.monotonic() - begin)
            save()
            assert result.returncode == 0, phase
        output = (out / 'execute.stdout').read_text()
        report['cases'] = [dict(id=name, seconds=int(ns) / 1e9)
                           for name, ns in re.findall(r'CASE_TIME\t([^\t\n]+)\t([0-9]+)', output)]
        assert len(report['cases']) == 4 and len({r['id'] for r in report['cases']}) == 4
        assert 'test result: ok. 4 passed; 0 failed;' in output
        assert report['source_sha256'] == {str(p.relative_to(ROOT)): digest(p) for p in (source, alloc)}
        assert report['driver_sha256'] == digest(Path(__file__))
        report.update(passed=True, execution_complete=True, sources_unchanged=True)
        save()
    except BaseException as error:
        report.update(error_type=type(error).__name__, error=str(error)[:500])
        save()
        raise
    print(json.dumps(report))


if __name__ == '__main__':
    main()
