#!/usr/bin/env python3
"""Protect the extracted parent/child location checks after real GC integration.

The memory-tree resolver, object tagging, dump predicate and owner are explicit
substitutes. This is a component contract, not a VM, allocator or ABI proof.
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
    start = text.index(declaration)
    end = text.index('{', start) + 1
    depth = 1
    while depth:
        depth += (text[end] == '{') - (text[end] == '}')
        end += 1
    return text[start:end]


PRELUDE = r'''
#![allow(non_camel_case_types, non_upper_case_globals, dead_code, static_mut_refs, unused_unsafe)]
use core::ffi::c_void;
type Lisp_Object = usize;
type uintptr_t = usize;
type bits_word = u64;
const BITS_PER_BITS_WORD: usize = 64;
const CONS_BLOCK_SIZE: usize = 1998;
const BLOCK_ALIGN: usize = 32768;
const TAG: usize = 1usize << 63;
#[repr(C)] #[derive(Clone, Copy)] struct ConsCdr { cdr: Lisp_Object }
#[repr(C)] #[derive(Clone, Copy)] struct ConsS { car: Lisp_Object, u: ConsCdr }
#[repr(C)] #[derive(Clone, Copy)] struct ConsU { s: ConsS }
#[repr(C)] #[derive(Clone, Copy)] pub struct Lisp_Cons { u: ConsU }
#[derive(Clone, Copy, PartialEq, Eq)] enum mem_type { MEM_TYPE_CONS, OTHER }
#[derive(Clone, Copy)] struct mem_node { start: *mut c_void, end: *mut c_void, r#type: mem_type }
const EMPTY: mem_node = mem_node { start: core::ptr::null_mut(), end: core::ptr::null_mut(), r#type: mem_type::OTHER };
static mut NODES: [mem_node; 2] = [EMPTY; 2];
static mut LOOKUPS: usize = 0;
static mut OWNER: bool = true;
static mut DUMP: usize = 0x10000;
static mut cons_block: *mut cons_block = core::ptr::null_mut();
static mut cons_block_index: usize = 0;
macro_rules! MEM_NIL { () => { core::ptr::null_mut::<mem_node>() }; }
macro_rules! eassert { ($x:expr) => { assert!($x) }; }
unsafe fn young_owner() -> bool { unsafe { OWNER } }
fn CONSP(object: Lisp_Object) -> bool { object & TAG != 0 }
fn lisp_h_XCONS_untracked(object: Lisp_Object) -> *mut Lisp_Cons { (object & !TAG) as *mut Lisp_Cons }
unsafe fn pdumper_object_p(pointer: *mut c_void) -> bool { unsafe { pointer as usize == DUMP } }
unsafe fn mem_find(pointer: *mut c_void) -> *mut mem_node {
    unsafe {
        LOOKUPS += 1;
        for i in 0..2 {
            let node = &raw mut NODES[i];
            if (*node).start as usize <= pointer as usize && (pointer as usize) < (*node).end as usize { return node; }
        }
        core::ptr::null_mut()
    }
}
'''

TESTS = r'''
#[cfg(test)] mod protection {
    use super::*;
    struct Timer(&'static str, std::time::Instant);
    impl Drop for Timer { fn drop(&mut self) { println!("CASE_TIME\t{}\t{}", self.0, self.1.elapsed().as_nanos()); } }
    fn begin(name: &'static str) -> Timer {
        unsafe { NODES = [EMPTY; 2]; LOOKUPS = 0; OWNER = true; DUMP = 0x10000;
            cons_block = core::ptr::null_mut(); cons_block_index = 0; }
        Timer(name, std::time::Instant::now())
    }
    struct Block { pointer: *mut cons_block, slot: usize }
    impl Block {
        fn new(slot: usize) -> Self {
            unsafe {
                let layout = std::alloc::Layout::from_size_align(BLOCK_ALIGN, BLOCK_ALIGN).unwrap();
                let pointer = std::alloc::alloc_zeroed(layout).cast::<cons_block>();
                assert!(!pointer.is_null());
                NODES[slot] = mem_node { start: pointer.cast(), end: pointer.cast::<u8>().add(core::mem::size_of::<cons_block>()).cast(), r#type: mem_type::MEM_TYPE_CONS };
                Self { pointer, slot }
            }
        }
        fn cell(&self, index: usize) -> *mut Lisp_Cons { unsafe { self.pointer.cast::<Lisp_Cons>().add(index) } }
        unsafe fn set(&self, index: usize, car: Lisp_Object, cdr: Lisp_Object) {
            unsafe { (*self.cell(index)).u.s.car = car; (*self.cell(index)).u.s.u.cdr = cdr; }
        }
        fn object(&self, index: usize) -> Lisp_Object { TAG | self.cell(index) as usize }
        unsafe fn check(&self, parent: usize, car: Lisp_Object, cdr: Lisp_Object, dirty: bool, lookups: usize) {
            unsafe {
                self.set(parent, car, cdr);
                (*self.pointer).dirtybits.fill(0xaaaaaaaaaaaaaaaa);
                (*self.pointer).oldbits.fill(0x5555555555555555);
                (*self.pointer).gcmarkbits.fill(0xf0f0f0f0f0f0f0f0);
                (*self.pointer).raw_tainted = 17;
                let mut expected = (*self.pointer).dirtybits;
                if dirty { expected[parent / 64] |= 1u64 << (parent % 64); }
                LOOKUPS = 0;
                gc_collector::note_cons_marked(self.cell(parent));
                assert_eq!(LOOKUPS, lookups);
                assert_eq!((*self.pointer).dirtybits, expected);
                assert_eq!((*self.pointer).oldbits, [0x5555555555555555; 32]);
                assert_eq!((*self.pointer).gcmarkbits, [0xf0f0f0f0f0f0f0f0; 32]);
                assert_eq!((*self.pointer).raw_tainted, 17);
                assert_eq!((*self.cell(parent)).u.s.car, car);
                assert_eq!((*self.cell(parent)).u.s.u.cdr, cdr);
            }
        }
    }
    impl Drop for Block { fn drop(&mut self) { unsafe {
        NODES[self.slot] = EMPTY;
        std::alloc::dealloc(self.pointer.cast(), std::alloc::Layout::from_size_align(BLOCK_ALIGN, BLOCK_ALIGN).unwrap());
    } } }

    #[test] fn same_block_prefix_omits_only_child_lookups() {
        let _timer = begin("same_block_prefix_omits_only_child_lookups");
        assert_eq!(core::mem::size_of::<Lisp_Cons>(), 16);
        assert_eq!(core::mem::size_of::<cons_block>(), 32752);
        assert_eq!(core::mem::offset_of!(cons_block, gcmarkbits), 31968);
        let b = Block::new(0);
        unsafe { for parent in [0, 63, 64, 1997] { for child in [0, 63, 64, 1997] {
            b.check(parent, b.object(child), b.object(1997), false, 1);
        } } }
    }

    #[test] fn partial_head_and_metadata_keep_original_fallback() {
        let _timer = begin("partial_head_and_metadata_keep_original_fallback");
        let b = Block::new(0);
        unsafe {
            cons_block = b.pointer; cons_block_index = 65;
            b.check(64, b.object(64), 0, false, 1);
            for index in [65, 1997, 1998] { b.check(0, b.object(index), 0, true, 2); }
            cons_block = core::ptr::null_mut();
            b.check(0, b.object(1997), 0, false, 1);
            b.check(0, b.object(1998), 0, true, 2);
        }
    }

    #[test] fn other_block_dump_null_and_misalignment_keep_checks() {
        let _timer = begin("other_block_dump_null_and_misalignment_keep_checks");
        let b = Block::new(0); let other = Block::new(1);
        unsafe {
            b.check(0, other.object(1997), other.object(0), false, 3);
            b.check(0, TAG | DUMP, other.object(0), true, 1); // car dump stops cdr
            b.check(0, other.object(0), TAG | DUMP, true, 2);
            b.check(0, TAG, other.object(0), true, 1);
            b.check(0, TAG | (b.cell(0) as usize + 1), 0, true, 2);
            b.check(0, TAG | (b.pointer as usize - 16), 0, true, 2);
            b.check(0, 0, 7, false, 1);
        }
    }

    #[test] fn parent_validation_and_owner_gate_are_preserved() {
        let _timer = begin("parent_validation_and_owner_gate_are_preserved");
        let b = Block::new(0);
        unsafe {
            b.set(0, TAG | DUMP, TAG);
            let original = (*b.pointer).dirtybits;
            OWNER = false; LOOKUPS = 0;
            gc_collector::note_cons_marked(b.cell(0));
            assert_eq!(LOOKUPS, 0); assert_eq!((*b.pointer).dirtybits, original);
            OWNER = true;
            for parent in [core::ptr::null_mut(), DUMP as *mut Lisp_Cons,
                (b.cell(0) as usize + 1) as *mut Lisp_Cons, b.cell(1998)] {
                gc_collector::note_cons_marked(parent);
                assert_eq!((*b.pointer).dirtybits, original);
            }
            cons_block = b.pointer; cons_block_index = 0;
            gc_collector::note_cons_marked(b.cell(0));
            assert_eq!((*b.pointer).dirtybits, original);
            cons_block = core::ptr::null_mut();
            NODES[0].r#type = mem_type::OTHER;
            gc_collector::note_cons_marked(b.cell(0));
            assert_eq!((*b.pointer).dirtybits, original);
            NODES[0].r#type = mem_type::MEM_TYPE_CONS;
            NODES[0].end = b.pointer.cast::<u8>().add(32751).cast();
            gc_collector::note_cons_marked(b.cell(0));
            assert_eq!((*b.pointer).dirtybits, original);
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
    text, allocation = source.read_text(), alloc.read_text()
    assert '#[repr(C)]\npub struct cons_block\n' in allocation
    extracted = '\n'.join(item(allocation, declaration) for declaration in
                          ('macro_rules! CONS_BLOCK ', 'macro_rules! CONS_INDEX '))
    extracted += '\n#[repr(C)]\n' + item(allocation, 'pub struct cons_block\n')
    hooks = '\n'.join(item(text, declaration) for declaration in
                      ('    unsafe fn heap_cell(', '    pub(super) unsafe fn note_cons_marked('))
    generated = PRELUDE + extracted + '\nmod gc_collector { use super::*;\n' + hooks + '\n}\n' + TESTS
    out.mkdir(parents=True)
    (out / 'hooks.rs').write_text(generated)
    started = time.monotonic()
    report = dict(passed=False, execution_complete=False,
                  source_sha256={str(p.relative_to(ROOT)): digest(p) for p in (source, alloc)},
                  driver_sha256=digest(Path(__file__)), generated_sha256=digest(out / 'hooks.rs'),
                  boundary='Unmodified heap_cell/note_cons_marked/cons block/macros with explicit tree resolver, tags, dump and owner substitutes; not VM, allocator or ABI acceptance',
                  budget=dict(whole_seconds=100, compile_seconds=60, execute_seconds=20,
                              scale='Four serial component tests, two 32768-byte aligned blocks, fixed boundary cases; no runtime or external service'))

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
        assert len(report['cases']) == len({row['id'] for row in report['cases']}) == 4
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
