// Spike 1 (Rust side): mirror the C structs with #[repr(C)] and print the same
// sizes and offsets, so the two reports can be diffed line by line.
#![allow(non_camel_case_types, non_snake_case, dead_code)]
use std::mem::{align_of, size_of, offset_of};

pub type EMACS_INT = isize;
#[repr(C)] pub struct Lisp_X { _private: [u8; 0] }
pub type Lisp_Word = *mut Lisp_X;
pub type Lisp_Object = Lisp_Word;
pub type INTERVAL = *mut std::ffi::c_void;

#[repr(C, align(8))] pub union vectorlike_header { pub size: isize, _align: [u8; 0] }

#[repr(C, align(8))] pub struct Lisp_String { pub u: Lisp_String_u }
#[repr(C, align(8))] pub union Lisp_String_u { pub s: Lisp_String_s, pub next: *mut Lisp_String }
#[repr(C)] #[derive(Clone, Copy)]
pub struct Lisp_String_s {
    pub size: isize, pub size_byte: isize, pub intervals: INTERVAL, pub data: *mut u8,
}

#[repr(C, align(8))] pub struct Lisp_Cons { pub u: Lisp_Cons_u }
#[repr(C, align(8))] pub union Lisp_Cons_u { pub s: Lisp_Cons_s, _align: [u8; 0] }
#[repr(C)] #[derive(Clone, Copy)]
pub struct Lisp_Cons_s { pub car: Lisp_Object, pub u: Lisp_Cons_s_u }
#[repr(C)] #[derive(Clone, Copy)]
pub union Lisp_Cons_s_u { pub cdr: Lisp_Object, pub chain: *mut Lisp_Cons }

#[repr(C, align(8))] pub struct Lisp_Vector { pub header: vectorlike_header, pub contents: [Lisp_Object; 0] }

#[repr(C, align(8))] pub struct Lisp_Symbol { pub u: Lisp_Symbol_u }
#[repr(C, align(8))] pub union Lisp_Symbol_u { pub s: Lisp_Symbol_s, _align: [u8; 0] }
#[repr(C)] #[derive(Clone, Copy)]
pub struct Lisp_Symbol_s {
    /// gcmarkbit:1, redirect:2, trapped_write:2, interned:2, declared_special:1
    pub bits: u32, pub _pad: u32,
    pub name: Lisp_Object, pub val: Lisp_Symbol_val, pub function: Lisp_Object,
    pub plist: Lisp_Object, pub next: *mut Lisp_Symbol,
}
#[repr(C)] #[derive(Clone, Copy)]
pub union Lisp_Symbol_val {
    pub value: Lisp_Object, pub alias: *mut Lisp_Symbol,
    pub blv: *mut std::ffi::c_void, pub fwd: *mut std::ffi::c_void,
}

#[repr(C, align(8))] pub struct Lisp_Subr {
    pub header: vectorlike_header,
    pub function: Lisp_Subr_function,
    pub min_args: i16, pub max_args: i16, pub symbol_name: *const i8,
    pub intspec: Lisp_Subr_intspec, pub command_modes: Lisp_Object, pub doc: EMACS_INT,
}
#[repr(C, align(8))] #[derive(Clone, Copy)]
pub union Lisp_Subr_function { pub a0: Option<unsafe extern "C" fn() -> Lisp_Object>, pub aMANY: Option<unsafe extern "C" fn(isize, *mut Lisp_Object) -> Lisp_Object> }
#[repr(C)] #[derive(Clone, Copy)]
pub union Lisp_Subr_intspec { pub string: *const i8, pub native: Lisp_Object }

// bignum.h's mirror carries the gmp.h subset (P-49); the twin keeps the same shape.
#[repr(C)] pub struct __mpz_struct { pub _mp_alloc: i32, pub _mp_size: i32, pub _mp_d: *mut u64 }
pub type mpz_t = [__mpz_struct; 1];
#[repr(C, align(8))] pub struct Lisp_Bignum { pub header: vectorlike_header, pub value: mpz_t }

fn main() {
    macro_rules! sz { ($t:ty) => { println!("sizeof {:<34} {}", stringify!($t), size_of::<$t>()) } }
    macro_rules! al { ($t:ty) => { println!("alignof {:<33} {}", stringify!($t), align_of::<$t>()) } }
    macro_rules! off { ($t:ty, $f:tt) => { println!("offsetof {:<32} {}", concat!(stringify!($t), ".", stringify!($f)), offset_of!($t, $f)) } }
    macro_rules! off2 { ($t:ty, $a:tt, $b:tt) => { println!("offsetof {:<32} {}", concat!(stringify!($t), ".", stringify!($a), ".", stringify!($b)), offset_of!($t, $a.$b)) } }
    macro_rules! off3 { ($t:ty, $a:tt, $b:tt, $c:tt) => { println!("offsetof {:<32} {}", concat!(stringify!($t), ".", stringify!($a), ".", stringify!($b), ".", stringify!($c)), offset_of!($t, $a.$b.$c)) } }

    println!("spike layout; system {}; EMACS_INT {} bits", std::env::consts::OS, 8 * size_of::<EMACS_INT>());
    sz!(__mpz_struct); al!(__mpz_struct);
    off!(__mpz_struct, _mp_alloc);
    off!(__mpz_struct, _mp_size);
    off!(__mpz_struct, _mp_d);
    sz!(mpz_t);
    println!("sizeof {:<34} {}", "mpz", size_of::<[mpz_t; 5]>());
    sz!(Lisp_Bignum); al!(Lisp_Bignum);
    off!(Lisp_Bignum, header);
    off!(Lisp_Bignum, value);
    sz!(Lisp_Object); al!(Lisp_Object);
    sz!(vectorlike_header); al!(vectorlike_header);
    sz!(Lisp_String); al!(Lisp_String);
    sz!(Lisp_Cons); al!(Lisp_Cons);
    sz!(Lisp_Vector); al!(Lisp_Vector);
    sz!(Lisp_Symbol); al!(Lisp_Symbol);
    sz!(Lisp_Subr); al!(Lisp_Subr);
    off!(vectorlike_header, size);
    off3!(Lisp_String, u, s, size);
    off3!(Lisp_String, u, s, size_byte);
    off3!(Lisp_String, u, s, intervals);
    off3!(Lisp_String, u, s, data);
    off2!(Lisp_String, u, next);
    off3!(Lisp_Cons, u, s, car);
    off!(Lisp_Vector, header);
    off!(Lisp_Vector, contents);
    off3!(Lisp_Symbol, u, s, name);
    off3!(Lisp_Symbol, u, s, val);
    off3!(Lisp_Symbol, u, s, function);
    off3!(Lisp_Symbol, u, s, plist);
    off3!(Lisp_Symbol, u, s, next);
    off3!(Lisp_Cons, u, s, car);
    off!(Lisp_Subr, function);
    off!(Lisp_Subr, max_args);
    off!(Lisp_Subr, min_args);
    off!(Lisp_Subr, symbol_name);
    off!(Lisp_Subr, intspec);
    off!(Lisp_Subr, command_modes);
    off!(Lisp_Subr, doc);
}
