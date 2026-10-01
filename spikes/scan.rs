// Spike 3: conservative stack scanning.
//
// alloc.c finds Lisp objects by reading the stack as raw words
// (mark_memory / mark_stack) after flushing registers with a setjmp.  This spike
// asks whether the same trick works for Rust frames at -O0/-O2/-O3: a pointer to
// a heap object is held in a local deep in the call stack, and the scanner walks
// the stack range between main and the scanner looking for that word.
#![allow(static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::hint::black_box;

#[inline(never)]
fn scan_range(a: usize, b: usize, target: usize) -> bool {
    // The stack grows downwards on this target, so the frames sit between min
    // and max; alloc.c decides the direction from `stack_bottom < &sentry`.
    // Scan the whole interval in both directions.
    let (mut p, hi) = if a < b { (a, b) } else { (b, a) };
    while p + std::mem::size_of::<usize>() <= hi {
        // SAFETY: the range is the live stack of this thread, captured from
        // frames that are still on it; reading it as words is exactly what
        // alloc.c's mark_memory does.
        let word = unsafe { std::ptr::read_volatile(p as *const usize) };
        if word == target {
            return true;
        }
        p += std::mem::align_of::<usize>();
    }
    false
}

#[inline(never)]
fn hold_and_scan(lo: usize, target: usize) -> bool {
    let held = target;          // the "Lisp object" lives only here
    black_box(&held);
    let here = std::ptr::addr_of!(held) as usize;
    scan_range(lo, here, target)
}

#[inline(never)]
fn flush(lo: usize, target: usize, buf: *mut u8) -> bool {
    unsafe extern "C" { fn _setjmp(b: *mut std::ffi::c_void) -> i32; }
    // C flushes callee-saved registers into the stack frame with a setjmp before
    // it scans (SET_STACK_TOP_ADDRESS); mirror that here.
    let r = unsafe { _setjmp(buf as *mut std::ffi::c_void) };
    black_box(r);
    hold_and_scan(lo, target)
}

fn main() {
    #[repr(C, align(16))] struct JmpBuf([u8; 256]);
    let mut jb = JmpBuf([0; 256]);
    let mut anchor = 0usize;
    let lo = std::ptr::addr_of!(anchor) as usize;
    let heap: Box<[usize; 2]> = Box::new([0x1234_5678_9abc_def0, 1]);
    let target = heap.as_ptr() as usize;      // a heap pointer, like a Lisp_Object
    black_box(&heap);
    black_box(&mut anchor);
    let found = flush(lo, target, jb.0.as_mut_ptr());
    println!("stack scan found heap pointer {}: {}", target, found);
    println!("opt level {} : {}", option_env!("SPIKE_OPT").unwrap_or("?"), found);
    black_box(heap);
}
