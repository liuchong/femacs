// femacs: port of src/lisp.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// This is the header mirror: declarations only (PORTING.md 0.1).  Definitions of
// functions live in exactly one translation unit each; this file declares types,
// constants and accessors exactly as src/lisp.h does.
//
// Copyright (C) 1985-2026 Free Software Foundation, Inc.
//
// This file is part of GNU Emacs, and femacs is distributed under the same
// license: the GNU General Public License, version 3 or later.  See LICENSE.
//
// The layout of every type here is asserted against the frozen C layout in
// docs/baseline/layout/ by `xtask layout-check`; those numbers come from the C
// compiler, never from this file.

// Header mirrors are textually included (PORTING.md 0.1), so they carry no inner
// attributes and no crate-relative paths; the including unit supplies both.

use std::ffi::{c_char, c_double, c_int, c_void};

// --- integers --------------------------------------------------------------
// src/lisp.h: EMACS_INT is one word; the frozen configuration says 64 bits
// (docs/baseline/config/darwin-arm64.json: ALIGNOF_EMACS_INT = 8).
pub type EMACS_INT = isize;
pub type EMACS_UINT = usize;
// src/lisp.h 90-102 define these as enum members chosen from the integer widths;
// on this target EMACS_INT is long long.
pub const EMACS_INT_WIDTH: usize = 64;
pub const EMACS_UINT_WIDTH: usize = 64;
pub const EMACS_INT_MAX: EMACS_INT = EMACS_INT::MAX;
pub const EMACS_INT_MIN: EMACS_INT = EMACS_INT::MIN;

// --- tagged pointers (src/lisp.h 220-300) ----------------------------------
pub const GCTYPEBITS: usize = 3; // DEFINE_GDB_SYMBOL (int, GCTYPEBITS)
pub const INTTYPEBITS: usize = GCTYPEBITS - 1;
pub const VALBITS: usize = EMACS_INT_WIDTH - GCTYPEBITS;
pub const VAL_MAX: EMACS_INT = EMACS_INT_MAX >> (GCTYPEBITS - 1);
pub const USE_LSB_TAG: bool = true; // required configuration (PORTING.md P-02)
// VALMASK = USE_LSB_TAG ? -(1 << GCTYPEBITS) : VAL_MAX
pub const VALMASK: EMACS_INT = -(1 << GCTYPEBITS);
pub const INTTYPEBITS_MASK: EMACS_INT = (1 << INTTYPEBITS) - 1;

// In the size word of a vector: the vector has been marked.
pub const ARRAY_MARK_FLAG: isize = isize::MIN; // PTRDIFF_MIN
// In the size word of a struct Lisp_Vector: it is really a pseudovector.
pub const PSEUDOVECTOR_FLAG: isize = isize::MAX - isize::MAX / 2;

/// `struct Lisp_X` exists only so that Lisp_Object is a pointer type; the C
/// declaration is incomplete and so is this one (src/lisp.h 311-323).
#[repr(C)]
pub struct Lisp_X {
    _private: [u8; 0],
}
pub type Lisp_Word = *mut Lisp_X;
pub type Lisp_Object = Lisp_Word;

/// enum Lisp_Type, with the values this configuration selects (USE_LSB_TAG).
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lisp_Type {
    Lisp_Symbol = 0,
    Lisp_Type_Unused0 = 1,
    Lisp_Int0 = 2,
    Lisp_Cons = 3,
    Lisp_String = 4,
    Lisp_Vectorlike = 5,
    Lisp_Int1 = 6,
    Lisp_Float = 7,
}

// --- accessors (src/lisp.h 360-520) ----------------------------------------
// C defines these as macros; Rust cannot make them `const fn` because pointer to
// integer casts are not const-evaluable, so they are ordinary inline functions
// (registered objects are initialised at run time, as C does in syms_of_*).

#[inline(always)]
pub fn lisp_h_XLI(o: Lisp_Object) -> EMACS_INT {
    o as EMACS_INT
}

#[inline(always)]
pub fn lisp_h_XIL(i: EMACS_INT) -> Lisp_Object {
    i as Lisp_Object
}

#[inline(always)]
pub fn lisp_h_XLP(o: Lisp_Object) -> *mut c_void {
    o as *mut c_void
}

#[inline(always)]
pub fn lisp_h_XUNTAG(a: Lisp_Object, type_: Lisp_Type, _ctype: usize) -> *mut c_void {
    (lisp_h_XLI(a) - (type_ as EMACS_INT)) as *mut c_void
}

#[inline(always)]
pub fn lisp_h_XTYPE(a: Lisp_Object) -> Lisp_Type {
    match lisp_h_XLI(a) & !VALMASK {
        0 => Lisp_Type::Lisp_Symbol,
        1 => Lisp_Type::Lisp_Type_Unused0,
        2 => Lisp_Type::Lisp_Int0,
        3 => Lisp_Type::Lisp_Cons,
        4 => Lisp_Type::Lisp_String,
        5 => Lisp_Type::Lisp_Vectorlike,
        6 => Lisp_Type::Lisp_Int1,
        _ => Lisp_Type::Lisp_Float,
    }
}

#[inline(always)]
pub fn lisp_h_TAGGEDP(a: Lisp_Object, tag: Lisp_Type) -> bool {
    (lisp_h_XLI(a) & !VALMASK) == tag as EMACS_INT
}

#[inline(always)]
pub fn make_lisp_ptr(ptr: *mut c_void, type_: Lisp_Type) -> Lisp_Object {
    lisp_h_XIL((ptr as EMACS_INT) + type_ as EMACS_INT)
}

// --- vector-like header (src/lisp.h 923) -----------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub union vectorlike_header {
    pub size: isize,
}

// --- strings (src/lisp.h 1554) ---------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub union Lisp_String_u {
    pub s: Lisp_String_s,
    pub next: *mut Lisp_String,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_String_s {
    /// Number of characters in string; MSB is used as the mark bit.
    pub size: isize,
    /// Number of bytes, negative for a unibyte string (-1/-2/-3 variants).
    pub size_byte: isize,
    pub intervals: *mut c_void,
    /// The data is always followed by a NUL that is not counted in size.
    pub data: *mut u8,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_String {
    pub u: Lisp_String_u,
}

// --- conses (src/lisp.h 1425) ----------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub union Lisp_Cons_s_u {
    pub cdr: Lisp_Object,
    pub chain: *mut Lisp_Cons,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_Cons_s {
    pub car: Lisp_Object,
    pub u: Lisp_Cons_s_u,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub union Lisp_Cons_u {
    pub s: Lisp_Cons_s,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Cons {
    pub u: Lisp_Cons_u,
}

// --- vectors (src/lisp.h 1728) ---------------------------------------------

#[repr(C, align(8))]
pub struct Lisp_Vector {
    pub header: vectorlike_header,
    pub contents: [Lisp_Object; 0], // FLEXIBLE_ARRAY_MEMBER
}

// --- floats (src/lisp.h, struct Lisp_Float) --------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub union Lisp_Float_u {
    pub data: c_double,
    pub chain: *mut Lisp_Float,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Float {
    pub u: Lisp_Float_u,
}

// --- symbols (src/lisp.h 785) ----------------------------------------------
// The C bitfield word is unsigned int: gcmarkbit:1, redirect:2, trapped_write:2,
// interned:2, declared_special:1 (PORTING.md 1.3).  Accessors keep the bit
// positions and widths identical to C.

pub const SYMBOL_GC_MARK_BIT: u32 = 1 << 0;
pub const SYMBOL_REDIRECT_SHIFT: u32 = 1;
pub const SYMBOL_REDIRECT_MASK: u32 = 0b11 << SYMBOL_REDIRECT_SHIFT;
pub const SYMBOL_TRAPPED_WRITE_SHIFT: u32 = 3;
pub const SYMBOL_TRAPPED_WRITE_MASK: u32 = 0b11 << SYMBOL_TRAPPED_WRITE_SHIFT;
pub const SYMBOL_INTERNED_SHIFT: u32 = 5;
pub const SYMBOL_INTERNED_MASK: u32 = 0b11 << SYMBOL_INTERNED_SHIFT;
pub const SYMBOL_DECLARED_SPECIAL_BIT: u32 = 1 << 7;

#[repr(C)]
#[derive(Clone, Copy)]
pub union Lisp_Symbol_val {
    pub value: Lisp_Object,
    pub alias: *mut Lisp_Symbol,
    pub blv: *mut c_void,
    pub fwd: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_Symbol_s {
    pub bits: u32,
    pub _pad: u32,
    /// The symbol's name, as a Lisp string.
    pub name: Lisp_Object,
    pub val: Lisp_Symbol_val,
    pub function: Lisp_Object,
    pub plist: Lisp_Object,
    pub next: *mut Lisp_Symbol,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub union Lisp_Symbol_u {
    pub s: Lisp_Symbol_s,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Symbol {
    pub u: Lisp_Symbol_u,
}

// --- subrs (src/lisp.h 2174) -----------------------------------------------

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub union Lisp_Subr_function {
    pub a0: Option<unsafe extern "C" fn() -> Lisp_Object>,
    pub a1: Option<unsafe extern "C" fn(Lisp_Object) -> Lisp_Object>,
    pub a2: Option<unsafe extern "C" fn(Lisp_Object, Lisp_Object) -> Lisp_Object>,
    pub aMANY: Option<unsafe extern "C" fn(isize, *mut Lisp_Object) -> Lisp_Object>,
    pub aUNEVALLED: Option<unsafe extern "C" fn(Lisp_Object) -> Lisp_Object>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union Lisp_Subr_intspec {
    pub string: *const c_char,
    pub native: Lisp_Object,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Subr {
    pub header: vectorlike_header,
    pub function: Lisp_Subr_function,
    pub min_args: i16,
    pub max_args: i16,
    pub symbol_name: *const c_char,
    pub intspec: Lisp_Subr_intspec,
    pub command_modes: Lisp_Object,
    /// Positive: offset into etc/DOC; negative: native-comp doc index.
    pub doc: EMACS_INT,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub union Aligned_Lisp_Subr {
    pub s: Lisp_Subr,
}

// --- markers (src/lisp.h, struct Lisp_Marker) ------------------------------
// `buffer` is `struct buffer *`, which buffer.rs will declare; the layout key
// only fixes its offset, so an opaque pointer is faithful here.

#[repr(C, align(8))]
pub struct Lisp_Marker {
    pub header: vectorlike_header,
    pub buffer: *mut c_void,
    /// bool_bf need_adjustment : 1; bool_bf insertion_type : 1;
    pub bits: u32,
    pub _pad: u32,
    pub next: *mut Lisp_Marker,
    pub charpos: isize,
    pub bytepos: isize,
}

// --- support declarations and accessors (batch 1) ---
// Sources: src/lisp.h 180-198 (assertions), 620-650 (gflags), 700-760 (tag
// accessors), 869-880 (tag initialisers), 985-1050 (pseudovector bits).

// --- assertions (src/lisp.h 180-198) ---------------------------------------
// Three forms, exactly as C: with checking on, `suppress_checking` short
// circuits and a failure calls die; with checking off the condition is not even
// evaluated (PORTING.md 2.5).

unsafe extern "C" {
    pub fn die (msg: *const c_char, file: *const c_char, line: c_int) -> !;
    pub static mut suppress_checking: bool;
}

/// lib/verify.h `assume`: a compile-time promise, undefined behaviour if false.
macro_rules! assume {
    ($cond:expr) => {
        if !($cond) {
            unsafe { core::hint::unreachable_unchecked() }
        }
    };
}

macro_rules! eassert {
    ($cond:expr) => {
        #[cfg(enable_checking)]
        {
            if !suppress_checking && !($cond) {
                unsafe { die (c"eassert".as_ptr(), c"lisp.rs".as_ptr(), line!() as c_int) }
            }
        }
    };
}

macro_rules! eassume {
    ($cond:expr) => {
        #[cfg(enable_checking)]
        {
            if suppress_checking {
                assume!($cond);
            } else if !($cond) {
                unsafe { die (c"eassume".as_ptr(), c"lisp.rs".as_ptr(), line!() as c_int) }
            }
        }
        #[cfg(not(enable_checking))]
        {
            assume!($cond);
        }
    };
}

// --- gflags (src/lisp.h 620-645) -------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct gflags {
    pub will_dump: bool,
    pub will_bootstrap: bool,
    pub will_dump_with_pdumper: bool,
    pub dumped_with_pdumper: bool,
}

unsafe extern "C" {
    /// Set after Emacs has started up the first time (emacs.c).
    pub static mut initialized: bool;
    pub static mut gflags: gflags;
    pub static mut symbols_with_pos_enabled: bool;
}

#[inline(always)]
pub fn will_dump_p () -> bool {
    if HAVE_PDUMPER { unsafe { gflags.will_dump } } else { false }
}

#[inline(always)]
pub fn will_bootstrap_p () -> bool {
    if HAVE_PDUMPER { unsafe { gflags.will_bootstrap } } else { false }
}

#[inline(always)]
pub fn will_dump_with_pdumper_p () -> bool {
    if HAVE_PDUMPER { unsafe { gflags.will_dump_with_pdumper } } else { false }
}

#[inline(always)]
pub fn dumped_with_pdumper_p () -> bool {
    if HAVE_PDUMPER { unsafe { gflags.dumped_with_pdumper } } else { false }
}

// --- tag accessors (src/lisp.h 700-760) ------------------------------------
// C's TAG_PTR/LISP_WORD_TAG and XUNTAG are macros; they keep their names and
// their call shape here for the same reason the C keeps them as macros.

pub type Lisp_Word_tag = EMACS_UINT;

#[inline(always)]
pub fn LISP_WORD_TAG (tag: Lisp_Word_tag) -> Lisp_Word_tag {
    // USE_LSB_TAG is true in every supported configuration (PORTING.md P-02).
    tag << 0
}

#[inline(always)]
pub fn TAG_PTR (tag: Lisp_Type, p: *mut c_void) -> Lisp_Object {
    ((p as EMACS_UINT) + LISP_WORD_TAG (tag as Lisp_Word_tag)) as Lisp_Object
}

#[inline(always)]
pub fn TAG_PTR_INITIALLY (tag: Lisp_Type, p: EMACS_INT) -> Lisp_Object {
    ((p as EMACS_UINT) + LISP_WORD_TAG (tag as Lisp_Word_tag)) as Lisp_Object
}

/// XUNTAG (a, tag, ctype): the C macro subtracts the tag and casts.
macro_rules! XUNTAG {
    ($a:expr, $tag:expr, $ctype:ty) => {
        (($a as isize) - ($tag as isize)) as *mut $ctype
    };
}

#[inline(always)]
pub fn XLI (o: Lisp_Object) -> EMACS_INT { lisp_h_XLI (o) }

#[inline(always)]
pub fn XIL (i: EMACS_INT) -> Lisp_Object { lisp_h_XIL (i) }

#[inline(always)]
pub fn XLP (o: Lisp_Object) -> *mut c_void { lisp_h_XLP (o) }

#[inline(always)]
pub fn XTYPE (a: Lisp_Object) -> Lisp_Type { lisp_h_XTYPE (a) }

#[inline(always)]
pub fn TAGGEDP (a: Lisp_Object, tag: Lisp_Type) -> bool { lisp_h_TAGGEDP (a, tag) }

#[inline(always)]
pub fn lisp_h_CHECK_TYPE (ok: bool, predicate: Lisp_Object, x: Lisp_Object) {
    if !ok {
        unsafe { wrong_type_argument (predicate, x) }
    }
}

#[inline(always)]
pub fn CHECK_TYPE (ok: bool, predicate: Lisp_Object, x: Lisp_Object) {
    lisp_h_CHECK_TYPE (ok, predicate, x)
}

unsafe extern "C" {
    pub fn wrong_type_argument (predicate: Lisp_Object, value: Lisp_Object) -> !;
}

// --- pseudovector types (src/lisp.h 985-1050) ------------------------------

pub const PSEUDOVECTOR_SIZE_BITS: isize = 12;
pub const PSEUDOVECTOR_SIZE_MASK: isize = (1 << PSEUDOVECTOR_SIZE_BITS) - 1;
pub const PSEUDOVECTOR_REST_BITS: isize = 12;
pub const PSEUDOVECTOR_REST_MASK: isize =
    ((1 << PSEUDOVECTOR_REST_BITS) - 1) << PSEUDOVECTOR_SIZE_BITS;
pub const PSEUDOVECTOR_AREA_BITS: isize =
    PSEUDOVECTOR_SIZE_BITS + PSEUDOVECTOR_REST_BITS;
pub const PVEC_TYPE_MASK: isize = 0x3f << PSEUDOVECTOR_AREA_BITS;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum pvec_type {
    PVEC_NORMAL_VECTOR = 0,
    PVEC_FREE = 1,
    PVEC_BIGNUM = 2,
    PVEC_MARKER = 3,
    PVEC_OVERLAY = 4,
    PVEC_FINALIZER = 5,
    PVEC_SYMBOL_WITH_POS = 6,
    PVEC_MISC_PTR = 7,
    PVEC_USER_PTR = 8,
    PVEC_PROCESS = 9,
    PVEC_FRAME = 10,
    PVEC_WINDOW = 11,
    PVEC_BOOL_VECTOR = 12,
    PVEC_BUFFER = 13,
    PVEC_HASH_TABLE = 14,
    PVEC_OBARRAY = 15,
    PVEC_TERMINAL = 16,
    PVEC_WINDOW_CONFIGURATION = 17,
    PVEC_SUBR = 18,
    PVEC_OTHER = 19,
    PVEC_XWIDGET = 20,
    PVEC_XWIDGET_VIEW = 21,
    PVEC_THREAD = 22,
    PVEC_MUTEX = 23,
    PVEC_CONDVAR = 24,
    PVEC_MODULE_FUNCTION = 25,
    PVEC_NATIVE_COMP_UNIT = 26,
    PVEC_TS_PARSER = 27,
    PVEC_TS_NODE = 28,
    PVEC_TS_COMPILED_QUERY = 29,
    PVEC_SQLITE = 30,
    PVEC_CLOSURE = 31,
    PVEC_CHAR_TABLE = 32,
    PVEC_SUB_CHAR_TABLE = 33,
    PVEC_RECORD = 34,
    PVEC_FONT = 35,
}

/// Kept equal to the highest member (src/lisp.h: PVEC_TAG_MAX = PVEC_FONT).
pub const PVEC_TAG_MAX: pvec_type = pvec_type::PVEC_FONT;

#[inline(always)]
pub fn lisp_h_VECTORLIKEP (x: Lisp_Object) -> bool {
    TAGGEDP (x, Lisp_Type::Lisp_Vectorlike)
}

#[inline(always)]
pub fn VECTORLIKEP (x: Lisp_Object) -> bool { lisp_h_VECTORLIKEP (x) }

/// struct Lisp_Symbol_With_Pos (src/lisp.h 947).
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Symbol_With_Pos {
    pub header: vectorlike_header,
    pub sym: Lisp_Object,
    pub pos: Lisp_Object,
}

#[inline(always)]
pub fn PSEUDOVECTORP (a: Lisp_Object, code: pvec_type) -> bool {
    lisp_h_VECTORLIKEP (a)
        && unsafe {
            let header = XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, vectorlike_header);
            ((*header).size & (PSEUDOVECTOR_FLAG | PVEC_TYPE_MASK))
                == (PSEUDOVECTOR_FLAG | ((code as isize) << PSEUDOVECTOR_AREA_BITS))
        }
}

// --- symbols (src/lisp.h 1054-1160) ----------------------------------------

#[inline(always)]
pub fn lisp_h_SYMBOL_WITH_POS_P (x: Lisp_Object) -> bool {
    PSEUDOVECTORP (x, pvec_type::PVEC_SYMBOL_WITH_POS)
}

#[inline(always)]
pub fn lisp_h_BARE_SYMBOL_P (x: Lisp_Object) -> bool {
    TAGGEDP (x, Lisp_Type::Lisp_Symbol)
}

#[inline(always)]
pub fn BARE_SYMBOL_P (x: Lisp_Object) -> bool { lisp_h_BARE_SYMBOL_P (x) }

#[inline(always)]
pub fn SYMBOL_WITH_POS_P (x: Lisp_Object) -> bool { lisp_h_SYMBOL_WITH_POS_P (x) }

#[inline(always)]
pub fn SYMBOLP (x: Lisp_Object) -> bool {
    BARE_SYMBOL_P (x)
        || (unsafe { symbols_with_pos_enabled } && SYMBOL_WITH_POS_P (x))
}

#[inline(always)]
pub fn XSYMBOL_WITH_POS (a: Lisp_Object) -> *mut Lisp_Symbol_With_Pos {
    eassert! (SYMBOL_WITH_POS_P (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Symbol_With_Pos)
}

#[inline(always)]
pub fn XSYMBOL_WITH_POS_SYM (a: Lisp_Object) -> Lisp_Object {
    let sym = unsafe { (*XSYMBOL_WITH_POS (a)).sym };
    eassume! (BARE_SYMBOL_P (sym));
    sym
}

#[inline(always)]
pub fn XSYMBOL_WITH_POS_POS (a: Lisp_Object) -> Lisp_Object {
    unsafe { (*XSYMBOL_WITH_POS (a)).pos }
}

#[inline(always)]
pub fn maybe_remove_pos_from_symbol (x: Lisp_Object) -> Lisp_Object {
    if unsafe { symbols_with_pos_enabled } && SYMBOL_WITH_POS_P (x) {
        XSYMBOL_WITH_POS_SYM (x)
    } else {
        x
    }
}

#[inline(always)]
pub fn XSYMBOL (a: Lisp_Object) -> *mut Lisp_Symbol {
    let a = if !BARE_SYMBOL_P (a) {
        eassume! (unsafe { symbols_with_pos_enabled });
        XSYMBOL_WITH_POS_SYM (a)
    } else {
        a
    };
    eassert! (BARE_SYMBOL_P (a));
    XUNTAG! (a, Lisp_Type::Lisp_Symbol, Lisp_Symbol)
}
