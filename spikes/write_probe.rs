// The Rust side of spikes/write_probe.c: the mirror's macros are place expressions, so
// the statements C writes to `GPT`, `Z`, `MODIFF`, `&MODIFF`, `FETCH_BYTE (n)`, … port
// literally (`NAME! ()`), and the four + two packed `bool_bf` bits are written through
// the named masks instead of C's bitfields.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]
mod m {
    include!("../src/include/config.rs");
    include!("../lib/include/stdio.rs");
    include!("../src/include/lisp.rs");
    include!("../src/include/itree.rs");
    include!("../src/include/character.rs");
    include!("../src/include/buffer.rs");
}
use m::*;
#[repr(C, align(8))]
pub struct Arena (pub [u8; 101664]);
#[unsafe(no_mangle)]
pub static mut lispsym: Arena = Arena ([0; 101664]);
use core::ffi::c_void;
static mut TS: thread_state = unsafe { core::mem::zeroed () };
#[unsafe(no_mangle)]
pub static mut current_thread: *mut thread_state = core::ptr::addr_of_mut! (TS);
static mut B: buffer = unsafe { core::mem::zeroed () };
static mut T: buffer_text = unsafe { core::mem::zeroed () };
static mut ARR: [u8; 64] = [0; 64];
static mut N: itree_node = unsafe { core::mem::zeroed () };
fn main () {
  unsafe {
    (*current_thread).m_current_buffer = core::ptr::addr_of_mut! (B) as *mut c_void;
    B.text = &raw mut T;
    T.beg = (&raw mut ARR) as *mut u8;
    GPT! () = 7;
    ZV! () += 3;
    ZV_BYTE! () = 9;
    GAP_SIZE! () = 4;
    MODIFF! () = 5;
    let m: *mut modiff_count = &raw mut MODIFF! ();
    *m += 1;
    SAVE_MODIFF! () = MODIFF! ();
    CHARS_MODIFF! () = 2;
    BEGV! () = 2;
    Z! () = ZV! ();
    Z_BYTE! () = ZV_BYTE! ();
    BEG_UNCHANGED! () = 1;
    END_UNCHANGED! () = 3;
    FETCH_BYTE! (1) = 0x41;
    println! ("places {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
              GPT! (), ZV! (), ZV_BYTE! (), GAP_SIZE! (), MODIFF! (), SAVE_MODIFF! (),
              CHARS_MODIFF! (), BEGV! (), Z! (), Z_BYTE! (),
              BEG_UNCHANGED! () + END_UNCHANGED! (),
              BUF_GPT! (current_buffer! ()), FETCH_BYTE! (1), *BYTE_POS_ADDR (1));
    B.bits = 0; T.bits = 0;
    B.bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P | BUFFER_CLIP_CHANGED
            | BUFFER_INHIBIT_BUFFER_HOOKS | BUFFER_LONG_LINE_OPTIMIZATIONS_P;
    T.bits |= BUFFER_TEXT_INHIBIT_SHRINKING | BUFFER_TEXT_REDISPLAY;
    let bb = &raw const B as *const u8;
    let tb = &raw const T as *const u8;
    println! ("bits {} {}", *bb.add (960), *tb.add (136));
    B.bits = 0; T.bits = 0;
    println! ("cleared {} {}", *bb.add (960), *tb.add (136));
    let nb = &raw const N as *const u8;
    N.bits |= ITREE_RED | ITREE_REAR_ADVANCE | ITREE_FRONT_ADVANCE;
    println! ("itree {}", *nb.add (72));
    N.bits = 0;
  }
}
