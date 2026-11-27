#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]
mod m {
include!("../../../src/include/config.rs");
include!("../../../crates/lisp-vm/src/ported/src/include/lisp.rs");
include!("../../../crates/lisp-vm/src/ported/src/include/itree.rs");
include!("../../../crates/lisp-vm/src/ported/src/include/character.rs");
include!("../../../crates/lisp-vm/src/ported/src/include/buffer.rs");
}
use m::*;
use core::ffi::c_void;
#[repr(C, align(8))] pub struct Arena (pub [u8; 101664]);
#[unsafe(no_mangle)] pub static mut lispsym: Arena = Arena ([0; 101664]);
#[repr(C, align(8))] struct VecBuf { hdr: vectorlike_header, tail: [Lisp_Object; 4] }
static mut VB: VecBuf = VecBuf { hdr: vectorlike_header { size: 0 }, tail: [lisp_h_Qnil (); 4] };
#[repr(C, align(8))] struct CtBuf { bytes: [u8; 24576] }
static mut CB: CtBuf = CtBuf { bytes: [0; 24576] };
fn main () {
  unsafe {
    VB.hdr.size = 4;
    let v = make_lisp_ptr ((&raw mut VB) as *mut c_void, Lisp_Type::Lisp_Vectorlike);
    ASET (v, 0, make_fixnum (11));
    println! ("read back {}", XFIXNUM (AREF (v, 0)));
    *aref_addr (v, 3) = make_fixnum (44);
    println! ("addr write {}", XFIXNUM (AREF (v, 3)));
    let ct = (&raw mut CB) as *mut Lisp_Char_Table;
    (*ct).header.size = PVECHEADERSIZE (pvec_type::PVEC_CHAR_TABLE, CHAR_TABLE_STANDARD_SLOTS + 8, 0);
    let cto = make_lisp_ptr (ct as *mut c_void, Lisp_Type::Lisp_Vectorlike);
    set_char_table_extras (cto, 2, make_fixnum (3));
    set_char_table_contents (cto, 5, make_fixnum (7));
    println! ("char table {} {}",
              XFIXNUM (FAM_ELEM! ((*XCHAR_TABLE (cto)).extras, 2)),
              XFIXNUM (FAM_ELEM! ((*XCHAR_TABLE (cto)).contents, 5)));
  }
}
