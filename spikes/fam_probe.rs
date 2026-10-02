// Guard for the flexible-array-member idiom (PORTING.md 1.3).  The mirror declares C's
// flexible array members as `[T; 0]` so `sizeof` and the following fields' offsets match
// C, but a Rust array of length 0 cannot be indexed — `arr[i]` is a bounds check that
// panics in debug *and* release.  `FAM_ELEM!` reaches the elements by address instead.
// This probe runs the four C-named accessors that used to index a `[T; 0]` directly, on
// buffers laid out like the real ones.
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
use core::ffi::c_void;
#[repr(C, align(8))]
pub struct Arena (pub [u8; 101664]);
#[unsafe(no_mangle)]
pub static mut lispsym: Arena = Arena ([0; 101664]);

/// A vector of four elements: the fixed header plus the flexible tail.
#[repr(C, align(8))]
struct VecBuf { hdr: vectorlike_header, tail: [Lisp_Object; 4] }
static mut VB: VecBuf = VecBuf { hdr: vectorlike_header { size: 0 }, tail: [lisp_h_Qnil (); 4] };

/// A char table with room for the standard slots and eight extras.
#[repr(C, align(8))]
struct CtBuf { bytes: [u8; 24576] }
static mut CB: CtBuf = CtBuf { bytes: [0; 24576] };

fn main () {
  unsafe {
    VB.hdr.size = 4;
    let v = make_lisp_ptr ((&raw mut VB) as *mut c_void, Lisp_Type::Lisp_Vectorlike);
    ASET (v, 0, make_fixnum (11));
    ASET (v, 3, make_fixnum (44));
    println! ("fam vector {} {} {}", XFIXNUM (AREF (v, 0)), XFIXNUM (AREF (v, 3)),
              *aref_addr (v, 1) as usize);
    *aref_addr (v, 2) = make_fixnum (33);
    println! ("fam vector addr {}", XFIXNUM (*aref_addr (v, 2)));

    let ct = (&raw mut CB) as *mut Lisp_Char_Table;
    (*ct).header.size = PVECHEADERSIZE (pvec_type::PVEC_CHAR_TABLE,
                                        CHAR_TABLE_STANDARD_SLOTS + 8, 0);
    let cto = make_lisp_ptr (ct as *mut c_void, Lisp_Type::Lisp_Vectorlike);
    set_char_table_extras (cto, 0, make_fixnum (1));
    set_char_table_extras (cto, 2, make_fixnum (3));
    set_char_table_contents (cto, 5, make_fixnum (7));
    println! ("fam char-table {} {} {} {}",
              XFIXNUM (FAM_ELEM! ((*XCHAR_TABLE (cto)).extras, 0)),
              XFIXNUM (FAM_ELEM! ((*XCHAR_TABLE (cto)).extras, 2)),
              XFIXNUM (FAM_ELEM! ((*XCHAR_TABLE (cto)).contents, 5)),
              CHAR_TABLE_EXTRA_SLOTS (ct));
    gc_aset (v, 1, make_fixnum (22));
    println! ("fam gc_aset {}", XFIXNUM (AREF (v, 1)));
  }
}
