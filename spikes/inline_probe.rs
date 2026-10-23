// The Rust side of spikes/inline_probe.c: a differential for the header-inline
// functions of character.h (built by spikes/inline-probe.sh; the C side is
// compiled against the frozen configure output).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]
mod m {
    include!("../src/include/config.rs");
    include!("../lib/include/stdio.rs");
    include!("../src/include/lisp.rs");
    include!("../src/include/character.rs");
}
use m::*;
use std::ffi::{c_char, c_int};
#[repr(C, align(8))]
pub struct Arena (pub [u8; 101664]);
#[unsafe(no_mangle)]
pub static mut lispsym: Arena = Arena ([0; 101664]);

fn one (buf: &mut [u8; 6], n: usize) {
  unsafe {
    println! ("{:02x}{:02x}{:02x}{:02x}{:02x} {} {} {} {} {} {}",
              buf[0], buf[1], buf[2], buf[3], buf[4], n,
              multibyte_length (buf.as_ptr (), core::ptr::null (), false, true),
              multibyte_length (buf.as_ptr (), core::ptr::null (), false, false),
              multibyte_length (buf.as_ptr (), buf.as_ptr ().add (n), true, true),
              STRING_CHAR (buf.as_ptr ()),
              if n > 1 { raw_prev_char_len (buf.as_ptr ().add (n)) } else { 1 });
  }
}
fn main () {
  let v: [c_int; 9] = [0x00, 0x41, 0x80, 0x88, 0x9F, 0xA0, 0xBF, 0xC0, 0xFF];
  let mut b = [0u8; 6];
  for i in 0..9 { b[0] = v[i] as u8; one (&mut b, 1); }
  let mut a = 0xC0;
  while a <= 0xDF { for i in 0..9 { b[0] = a as u8; b[1] = v[i] as u8; one (&mut b, 2); } a += 0x1F; }
  let mut a = 0xE0;
  while a <= 0xEF { for i in 0..9 { for j in 0..9 {
      b[0] = a as u8; b[1] = v[i] as u8; b[2] = v[j] as u8; one (&mut b, 3); } } a += 0x0F; }
  let mut a = 0xF0;
  while a <= 0xF7 { let mut i = 0; while i < 9 { let mut j = 0; while j < 9 { let mut k = 0; while k < 9 {
      b[0] = a as u8; b[1] = v[i] as u8; b[2] = v[j] as u8; b[3] = v[k] as u8; b[4] = 0; one (&mut b, 4);
      k += 8; } j += 4; } i += 2; } a += 0x07; }
  for a in 0x88..=0x8F { for i in 0..9 { for j in 0..9 { for k in 0..9 {
      b[0] = 0xF8; b[1] = a as u8; b[2] = v[i] as u8; b[3] = v[j] as u8; b[4] = v[k] as u8; one (&mut b, 5);
  } } } }
}
