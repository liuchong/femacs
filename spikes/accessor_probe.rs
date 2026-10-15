// The Rust side of spikes/accessor_probe.c: the read accessors of buffer.h, over the
// same buffer the C probe builds.  The two marker.c conversions are stubbed to the
// identity on both sides; everything else is the mirror.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]
mod m {
    include!("../femacs/src/include/config.rs");
    include!("../femacs/lib/include/stdio.rs");
    include!("../femacs/src/include/lisp.rs");
    include!("../femacs/src/include/itree.rs");
    include!("../femacs/src/include/character.rs");
    include!("../femacs/src/include/buffer.rs");
}
use m::*;
use core::ffi::{c_int, c_uint, c_void};
#[repr(C, align(8))]
pub struct Arena (pub [u8; 101664]);
#[unsafe(no_mangle)]
pub static mut lispsym: Arena = Arena ([0; 101664]);
static mut TS: thread_state = unsafe { core::mem::zeroed () };
#[unsafe(no_mangle)]
pub static mut current_thread: *mut thread_state = core::ptr::addr_of_mut! (TS);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn buf_charpos_to_bytepos (_b: *mut c_void, c: isize) -> isize { c }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn buf_bytepos_to_charpos (_b: *mut c_void, c: isize) -> isize { c }
static mut B: buffer = unsafe { core::mem::zeroed () };
static mut T: buffer_text = unsafe { core::mem::zeroed () };
static mut ARR: [u8; 64] = [0; 64];
fn main () {
  unsafe {
    (*current_thread).m_current_buffer = core::ptr::addr_of_mut! (B) as *mut c_void;
    B.text = &raw mut T;
    T.beg = (&raw mut ARR) as *mut u8;
    for i in 0..64 { ARR[i] = (0x40 + i) as u8; }
    B.begv = 2; B.pt = 3; B.zv = 5; B.begv_byte = 2; B.pt_byte = 3; B.zv_byte = 5;
    T.gpt = 4; T.gpt_byte = 4; T.z = 6; T.z_byte = 6; T.gap_size = 3;
    println! ("pos {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
              BEGV! (), PT! (), GPT! (), GPT_BYTE! (), ZV! (), ZV_BYTE! (), Z! (), Z_BYTE! (),
              GAP_SIZE! (), BUF_BEG (current_buffer! ()), BUF_BEG_BYTE (current_buffer! ()),
              BUF_PT (current_buffer! ()), BUF_PT_BYTE (current_buffer! ()),
              BUF_ZV (current_buffer! ()), BUF_ZV_BYTE (current_buffer! ()), BUF_COMPACT! (current_buffer! ()));
    for n in 1..=6 {
      println! ("addr {} {} {} {} {} {} {} {} {} {}",
                n,
                BYTE_POS_ADDR (n).offset_from (BEG_ADDR! ()),
                *BYTE_POS_ADDR (n) as c_int,
                PTR_BYTE_POS (BYTE_POS_ADDR (n)),
                CHAR_POS_ADDR (n).offset_from (BEG_ADDR! ()),
                BUF_PTR_BYTE_POS (current_buffer! (), BYTE_POS_ADDR (n)),
                FETCH_BYTE! (n) as c_uint,
                BUF_FETCH_BYTE (current_buffer! (), n) as c_uint,
                PTR_BYTE_POS (BUF_BYTE_ADDRESS (current_buffer! (), n)),
                CHAR_TO_BYTE (n));
    }
    println! ("gap {} {} {}",
              BUF_GPT_ADDR (current_buffer! ()).offset_from (BEG_ADDR! ()),
              BUF_Z_ADDR (current_buffer! ()).offset_from (BEG_ADDR! ()),
              BUF_GAP_END_ADDR (current_buffer! ()).offset_from (BEG_ADDR! ()));
    T.modiff = 10; T.unchanged_modified = 10;
    T.overlay_modiff = 20; T.overlay_unchanged_modified = 20;
    T.beg_unchanged = 0; T.end_unchanged = 0;
    BUF_COMPUTE_UNCHANGED (current_buffer! (), 2, 2);
    println! ("unch eq {} {}", BUF_BEG_UNCHANGED! (current_buffer! ()), BUF_END_UNCHANGED! (current_buffer! ()));
    T.overlay_unchanged_modified = 21;
    T.beg_unchanged = 0; T.end_unchanged = 0;
    BUF_COMPUTE_UNCHANGED (current_buffer! (), 2, 2);
    println! ("unch ne {} {}", BUF_BEG_UNCHANGED! (current_buffer! ()), BUF_END_UNCHANGED! (current_buffer! ()));
    T.beg_unchanged = 5; T.end_unchanged = 9;
    BUF_COMPUTE_UNCHANGED (current_buffer! (), 3, 4);
    println! ("unch min {} {}", BUF_BEG_UNCHANGED! (current_buffer! ()), BUF_END_UNCHANGED! (current_buffer! ()));
  }
}
