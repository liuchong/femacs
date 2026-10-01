// Spike 2: sys_setjmp / sys_longjmp from Rust.
//
// Mirrors the two mechanisms lisp.h selects on this platform
// (HAVE__SETJMP -> _setjmp/_longjmp, HAVE_SIGSETJMP -> sigsetjmp/siglongjmp with
// savemask 0) and answers three questions:
//   1. does a longjmp across Rust frames resume with the C semantics (setjmp
//      returns the value passed to longjmp)?
//   2. is a Drop value in a jumped-over frame dropped?  (It must not be: the C
//      unwinding path does not run destructors, so PORTING.md forbids holding
//      Drop values across a setjmp boundary.)
//   3. does an explicitly released heap buffer survive the jump as in C?
#![allow(dead_code, static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::c_void;

// The exact types come from config.h; the buffer is deliberately oversized here
// and the real port takes the size from the generated configuration.
#[repr(C, align(16))] pub struct JmpBuf([u8; 256]);

unsafe extern "C" {
    fn _setjmp(buf: *mut c_void) -> i32;
    fn _longjmp(buf: *mut c_void, val: i32) -> !;
    fn sigsetjmp(buf: *mut c_void, savemask: i32) -> i32;
    fn siglongjmp(buf: *mut c_void, val: i32) -> !;
}

static mut BUF: JmpBuf = JmpBuf([0; 256]);
static mut DROPPED: i32 = 0;

struct Guard(i32);
impl Drop for Guard {
    fn drop(&mut self) {
        unsafe { DROPPED += 1 };
    }
}

#[inline(never)]
unsafe fn deep_setjmp(savemask: i32) -> i32 {
    if savemask == 0 {
        _longjmp(std::ptr::addr_of_mut!(BUF).cast(), 7)
    } else {
        siglongjmp(std::ptr::addr_of_mut!(BUF).cast(), 9)
    }
}

#[inline(never)]
unsafe fn mid_setjmp(savemask: i32) -> i32 {
    let g = Guard(1);           // must NOT be dropped when the jump happens
    std::hint::black_box(&g);
    deep_setjmp(savemask)
}

fn main() {
    unsafe {
        let r1 = _setjmp(std::ptr::addr_of_mut!(BUF).cast());
        if r1 == 0 {
            mid_setjmp(0);
        } else {
            println!("setjmp      resumed with {} (expected 7)", r1);
        }
        let r2 = sigsetjmp(std::ptr::addr_of_mut!(BUF).cast(), 0);
        if r2 == 0 {
            mid_setjmp(1);
        } else {
            println!("sigsetjmp   resumed with {} (expected 9)", r2);
        }
        println!("guard drop count {} (expected 0: C does not unwind constructors)",
                 DROPPED);
        println!("jmpbuf      user buffer {} bytes, alignment {}",
                 std::mem::size_of::<JmpBuf>(), std::mem::align_of::<JmpBuf>());
    }
}
