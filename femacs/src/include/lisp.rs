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

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_ushort, c_void};

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
    // Present only when the reference is configured with HAVE_NATIVE_COMP
    // (PORTING.md 9.2 structural condition).
    #[cfg(feature = "native_comp")]
    pub native_comp_u: Lisp_Object,
    #[cfg(feature = "native_comp")]
    pub native_c_name: *mut c_char,
    #[cfg(feature = "native_comp")]
    pub lambda_list: Lisp_Object,
    #[cfg(feature = "native_comp")]
    pub type_: Lisp_Object,
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
    pub fn strlen (s: *const c_char) -> usize;
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

// --- globals (src/lisp.h 912 includes the generated globals.h) --------------
// In the reference build this file comes from make-docfile; xtask gen-globals
// produces the port's version (Q symbols and the built-in symbol array).
include!("globals.rs");

// --- batch 2: numbers, conses, strings, vectors (src/lisp.h 1170-1750) ---
// Integer and pointer accessors, cons/string/vector accessors and the checks
// that go with them.

pub const INTMASK: EMACS_INT = EMACS_INT_MAX >> (INTTYPEBITS - 1);
pub const MOST_POSITIVE_FIXNUM: EMACS_INT = EMACS_INT_MAX >> INTTYPEBITS;
pub const MOST_NEGATIVE_FIXNUM: EMACS_INT = -1 - MOST_POSITIVE_FIXNUM;

/// FIXNUM_OVERFLOW_P (i) from src/lisp.h 1178.
#[inline(always)]
pub fn FIXNUM_OVERFLOW_P (i: EMACS_INT) -> bool {
    !((0 <= i || MOST_NEGATIVE_FIXNUM <= i) && i <= MOST_POSITIVE_FIXNUM)
}

/// `likely` from conf_post.h 377 was __builtin_expect; Rust has no equivalent
/// branch hint in stable, so this is the identity and the deviation is recorded.
#[inline(always)]
pub fn likely (cond: bool) -> bool { cond }

unsafe extern "C" {
    pub fn slow_eq (x: Lisp_Object, y: Lisp_Object) -> bool;
    pub fn memchr (s: *const c_void, c: c_int, n: usize) -> *mut c_void;
}

// --- numbers ---------------------------------------------------------------

#[inline(always)]
pub fn lisp_h_FIXNUMP (x: Lisp_Object) -> bool {
    (XTYPE (x) as isize & (Lisp_Type::Lisp_Int0 as isize | !(Lisp_Type::Lisp_Int1 as isize)))
        == Lisp_Type::Lisp_Int0 as isize
}

#[inline(always)]
pub fn FIXNUMP (x: Lisp_Object) -> bool { lisp_h_FIXNUMP (x) }

#[inline(always)]
pub fn make_fixnum (mut n: EMACS_INT) -> Lisp_Object {
    eassert! (! FIXNUM_OVERFLOW_P (n));
    let int0 = Lisp_Type::Lisp_Int0 as EMACS_INT;
    // USE_LSB_TAG is the only supported configuration (PORTING.md P-02).
    let u = n as EMACS_UINT;
    n = (u << INTTYPEBITS) as EMACS_INT;
    n += int0;
    XIL (n)
}

#[inline(always)]
pub fn make_ufixnum (mut n: EMACS_INT) -> Lisp_Object {
    eassert! (0 <= n && n <= INTMASK);
    let int0 = Lisp_Type::Lisp_Int0 as EMACS_INT;
    let u = n as EMACS_UINT;
    n = (u << INTTYPEBITS) as EMACS_INT;
    n += int0;
    XIL (n)
}

#[inline(always)]
pub fn XFIXNUM_RAW (a: Lisp_Object) -> EMACS_INT {
    let i = XLI (a);
    // ! USE_LSB_TAG would shift left first; not a supported configuration.
    i >> INTTYPEBITS
}

#[inline(always)]
pub fn XFIXNUM (a: Lisp_Object) -> EMACS_INT {
    eassert! (FIXNUMP (a));
    XFIXNUM_RAW (a)
}

#[inline(always)]
pub fn XUFIXNUM_RAW (a: Lisp_Object) -> EMACS_UINT {
    let i = XLI (a) as EMACS_UINT;
    i >> INTTYPEBITS
}

#[inline(always)]
pub fn XUFIXNUM (a: Lisp_Object) -> EMACS_UINT {
    eassert! (FIXNUMP (a));
    XUFIXNUM_RAW (a)
}

#[inline(always)]
pub fn lisp_h_XHASH (a: Lisp_Object) -> EMACS_UINT { XUFIXNUM_RAW (a) }

#[inline(always)]
pub fn XHASH (a: Lisp_Object) -> EMACS_UINT { lisp_h_XHASH (a) }

#[inline(always)]
pub fn make_fixed_natnum (n: EMACS_INT) -> Lisp_Object {
    eassert! (0 <= n && n <= MOST_POSITIVE_FIXNUM);
    make_fixnum (n)
}

#[inline(always)]
pub fn XFIXNUMPTR (a: Lisp_Object) -> *mut c_char {
    XUNTAG! (a, Lisp_Type::Lisp_Int0, c_char)
}

#[inline(always)]
pub fn make_pointer_integer_unsafe (p: *mut c_void) -> Lisp_Object {
    TAG_PTR_INITIALLY (Lisp_Type::Lisp_Int0, p as EMACS_INT)
}

#[inline(always)]
pub fn make_pointer_integer (p: *mut c_void) -> Lisp_Object {
    let a = make_pointer_integer_unsafe (p);
    eassert! (FIXNUMP (a) && XFIXNUMPTR (a) == p as *mut c_char);
    a
}

#[inline(always)]
pub fn clip_to_bounds (lower: isize, num: isize, upper: isize) -> isize {
    lower.max (num.min (upper))
}

#[inline(always)]
pub fn dead_object () -> Lisp_Object {
    make_lisp_ptr (std::ptr::null_mut (), Lisp_Type::Lisp_String)
}

// --- symbols, conses, strings, vectors -------------------------------------

#[inline(always)]
pub fn lisp_h_BASE_EQ (x: Lisp_Object, y: Lisp_Object) -> bool {
    XLI (x) == XLI (y)
}

#[inline(always)]
pub fn BASE_EQ (x: Lisp_Object, y: Lisp_Object) -> bool { lisp_h_BASE_EQ (x, y) }

#[inline(always)]
pub fn EQ (x: Lisp_Object, y: Lisp_Object) -> bool {
    if BASE_EQ (x, y) {
        true
    } else if likely (! unsafe { symbols_with_pos_enabled }) {
        false
    } else {
        unsafe { slow_eq (x, y) }
    }
}

#[inline(always)]
pub fn lisp_h_CHECK_SYMBOL (x: Lisp_Object) {
    CHECK_TYPE (SYMBOLP (x), Qsymbolp(), x);
}

#[inline(always)]
pub fn CHECK_SYMBOL (x: Lisp_Object) { lisp_h_CHECK_SYMBOL (x) }

#[inline(always)]
pub fn lisp_h_NILP (x: Lisp_Object) -> bool {
    BASE_EQ (x, Qnil())
}

#[inline(always)]
pub fn NILP (x: Lisp_Object) -> bool { lisp_h_NILP (x) }

#[inline(always)]
pub fn lisp_h_CONSP (x: Lisp_Object) -> bool {
    TAGGEDP (x, Lisp_Type::Lisp_Cons)
}

#[inline(always)]
pub fn CONSP (x: Lisp_Object) -> bool { lisp_h_CONSP (x) }

#[inline(always)]
pub fn CHECK_CONS (x: Lisp_Object) {
    CHECK_TYPE (CONSP (x), Qconsp(), x);
}

#[inline(always)]
pub fn XCONS (a: Lisp_Object) -> *mut Lisp_Cons {
    eassert! (CONSP (a));
    XUNTAG! (a, Lisp_Type::Lisp_Cons, Lisp_Cons)
}

#[inline(always)]
pub fn xcar_addr (c: Lisp_Object) -> *mut Lisp_Object {
    unsafe { std::ptr::addr_of_mut! ((*XCONS (c)).u.s.car) }
}

#[inline(always)]
pub fn xcdr_addr (c: Lisp_Object) -> *mut Lisp_Object {
    unsafe { std::ptr::addr_of_mut! ((*XCONS (c)).u.s.u.cdr) }
}

#[inline(always)]
pub fn lisp_h_XCAR (c: Lisp_Object) -> Lisp_Object { unsafe { (*XCONS (c)).u.s.car } }

#[inline(always)]
pub fn lisp_h_XCDR (c: Lisp_Object) -> Lisp_Object { unsafe { (*XCONS (c)).u.s.u.cdr } }

#[inline(always)]
pub fn XCAR (c: Lisp_Object) -> Lisp_Object { lisp_h_XCAR (c) }

#[inline(always)]
pub fn XCDR (c: Lisp_Object) -> Lisp_Object { lisp_h_XCDR (c) }

#[inline(always)]
pub fn XSETCAR (c: Lisp_Object, n: Lisp_Object) { unsafe { *xcar_addr (c) = n } }

#[inline(always)]
pub fn XSETCDR (c: Lisp_Object, n: Lisp_Object) { unsafe { *xcdr_addr (c) = n } }

#[inline(always)]
pub fn CAR (c: Lisp_Object) -> Lisp_Object {
    if CONSP (c) { return XCAR (c); }
    if !NILP (c) { unsafe { wrong_type_argument (Qlistp(), c) } }
    Qnil()
}

#[inline(always)]
pub fn CDR (c: Lisp_Object) -> Lisp_Object {
    if CONSP (c) { return XCDR (c); }
    if !NILP (c) { unsafe { wrong_type_argument (Qlistp(), c) } }
    Qnil()
}

#[inline(always)]
pub fn CAR_SAFE (c: Lisp_Object) -> Lisp_Object { if CONSP (c) { XCAR (c) } else { Qnil() } }

#[inline(always)]
pub fn CDR_SAFE (c: Lisp_Object) -> Lisp_Object { if CONSP (c) { XCDR (c) } else { Qnil() } }

#[inline(always)]
pub fn lisp_h_FLOATP (x: Lisp_Object) -> bool { TAGGEDP (x, Lisp_Type::Lisp_Float) }

#[inline(always)]
pub fn FLOATP (x: Lisp_Object) -> bool { lisp_h_FLOATP (x) }

#[inline(always)]
pub fn STRINGP (x: Lisp_Object) -> bool { TAGGEDP (x, Lisp_Type::Lisp_String) }

#[inline(always)]
pub fn CHECK_STRING (x: Lisp_Object) {
    CHECK_TYPE (STRINGP (x), Qstringp(), x);
}

#[inline(always)]
pub fn XSTRING (a: Lisp_Object) -> *mut Lisp_String {
    eassert! (STRINGP (a));
    XUNTAG! (a, Lisp_Type::Lisp_String, Lisp_String)
}

#[inline(always)]
pub fn STRING_MULTIBYTE (str: Lisp_Object) -> bool {
    unsafe { 0 <= (*XSTRING (str)).u.s.size_byte }
}

#[inline(always)]
pub fn STRING_SET_MULTIBYTE (str: Lisp_Object) {
    // The 0-length strings are unique and shared, so they cannot be modified.
    eassert! (unsafe { (*XSTRING (str)).u.s.size } > 0);
    unsafe { (*XSTRING (str)).u.s.size_byte = (*XSTRING (str)).u.s.size };
}

#[inline(always)]
pub fn SDATA (string: Lisp_Object) -> *mut u8 { unsafe { (*XSTRING (string)).u.s.data } }

#[inline(always)]
pub fn SSDATA (string: Lisp_Object) -> *mut c_char { SDATA (string) as *mut c_char }

#[inline(always)]
pub fn SREF (string: Lisp_Object, index: usize) -> u8 { unsafe { *SDATA (string).add (index) } }

#[inline(always)]
pub fn SSET (string: Lisp_Object, index: usize, new: u8) { unsafe { *SDATA (string).add (index) = new } }

#[inline(always)]
pub fn SCHARS (string: Lisp_Object) -> isize {
    let nchars = unsafe { (*XSTRING (string)).u.s.size };
    eassume! (0 <= nchars);
    nchars
}

#[inline(always)]
pub fn STRING_BYTES (s: *mut Lisp_String) -> isize {
    let nbytes = unsafe {
        let sb = (*s).u.s.size_byte;
        if sb < 0 { (*s).u.s.size } else { sb }
    };
    eassume! (0 <= nbytes);
    nbytes
}

#[inline(always)]
pub fn SBYTES (string: Lisp_Object) -> isize { STRING_BYTES (XSTRING (string)) }

#[inline(always)]
pub fn STRING_SET_CHARS (string: Lisp_Object, newsize: isize) {
    eassert! (
        if STRING_MULTIBYTE (string) {
            0 <= newsize && newsize <= SBYTES (string)
        } else {
            newsize == SCHARS (string)
        }
    );
    unsafe { (*XSTRING (string)).u.s.size = newsize };
}

#[inline(always)]
pub fn CHECK_STRING_NULL_BYTES (string: Lisp_Object) {
    CHECK_TYPE (
        unsafe { memchr (SSDATA (string) as *const c_void, 0, SBYTES (string) as usize).is_null () },
        Qfilenamep(),
        string,
    );
}

#[inline(always)]
pub fn string_immovable_p (str: Lisp_Object) -> bool {
    unsafe { (*XSTRING (str)).u.s.size_byte == -3 }
}

#[inline(always)]
pub fn XVECTOR (a: Lisp_Object) -> *mut Lisp_Vector {
    eassert! (VECTORLIKEP (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Vector)
}

#[inline(always)]
pub fn ASIZE (array: Lisp_Object) -> isize {
    let size = unsafe { (*XVECTOR (array)).header.size };
    eassume! (0 <= size);
    size
}

// --- batch 3a: built-in symbols (src/lisp.h 1110-1170) ---
// The built-in symbols live in the array the generated globals.rs declares; a
// symbol's Lisp_Object is its offset from the start of that array.

#[inline(always)]
pub fn XBARE_SYMBOL (a: Lisp_Object) -> *mut Lisp_Symbol {
    eassert! (BARE_SYMBOL_P (a));
    let i = XUNTAG! (a, Lisp_Type::Lisp_Symbol, Lisp_Symbol) as isize;
    unsafe { (&raw mut lispsym as *mut u8).offset (i) as *mut Lisp_Symbol }
}

#[inline(always)]
pub fn make_lisp_symbol_internal (sym: *mut Lisp_Symbol) -> Lisp_Object {
    // No eassert here in C either, so that builtin symbols can be constant
    // folded by the C compiler; the port keeps the same body.
    let symoffset = unsafe { (sym as *mut u8).offset_from (&raw const lispsym as *const u8) };
    TAG_PTR_INITIALLY (Lisp_Type::Lisp_Symbol, symoffset as EMACS_INT)
}

#[inline(always)]
pub fn make_lisp_symbol (sym: *mut Lisp_Symbol) -> Lisp_Object {
    let a = make_lisp_symbol_internal (sym);
    eassert! (XBARE_SYMBOL (a) == sym);
    a
}

#[inline(always)]
pub fn builtin_lisp_symbol (index: usize) -> Lisp_Object {
    make_lisp_symbol_internal (unsafe { (&raw mut lispsym as *mut Lisp_Symbol).add (index) })
}

#[inline(always)]
pub fn c_symbol_p (sym: *mut Lisp_Symbol) -> bool {
    let bp = unsafe { &raw const lispsym as *const u8 };
    let sp = sym as *const u8;
    // PTRDIFF_MAX < INTPTR_MAX is false on this target, so the offset form is used.
    let offset = unsafe { sp.offset_from (bp) };
    0 <= offset && (offset as usize) < std::mem::size_of::<[Lisp_Symbol; lispsym_count]> ()
}

// --- batch 3: vectors, bool-vectors, char-tables, symbols (src/lisp.h 1740-2400) ---
pub type bits_word = usize;
pub const BITS_PER_BITS_WORD: usize = 64;      // SIZE_WIDTH from the configuration
pub const BITS_WORD_MAX: usize = usize::MAX;
pub const BOOL_VECTOR_BITS_PER_CHAR: usize = 8;
/// src/lisp.h 1999: iQnil == 0 && Lisp_Symbol == 0.
pub const NIL_IS_ZERO: bool = true;
pub const CHARTAB_SIZE_BITS_0: usize = 6;
pub const CHARTAB_SIZE_BITS_1: usize = 4;
pub const CHARTAB_SIZE_BITS_2: usize = 5;
pub const CHARTAB_SIZE_BITS_3: usize = 7;

// Symbol redirect/trapped-write/interned states (src/lisp.h 2000-2040).
pub const SYMBOL_PLAINVAL: u32 = 0;
pub const SYMBOL_VARALIAS: u32 = 1;
pub const SYMBOL_LOCALIZED: u32 = 2;
pub const SYMBOL_FORWARDED: u32 = 3;
pub const SYMBOL_UNTRAPPED_WRITE: u32 = 0;
pub const SYMBOL_NOWRITE: u32 = 1;
pub const SYMBOL_UNINTERNED: u32 = 0;
pub const SYMBOL_INTERNED: u32 = 1;
pub const SYMBOL_INTERNED_IN_INITIAL_OBARRAY: u32 = 2;

#[repr(C, align(8))]
pub struct Lisp_Bool_Vector {
    pub header: vectorlike_header,
    pub size: EMACS_INT,
    pub data: [bits_word; 0],      // FLEXIBLE_ARRAY_MEMBER
}

#[repr(C, align(8))]
pub struct Lisp_Char_Table {
    pub header: vectorlike_header,
    pub defalt: Lisp_Object,
    pub parent: Lisp_Object,
    pub purpose: Lisp_Object,
    pub ascii: Lisp_Object,
    pub contents: [Lisp_Object; 1 << CHARTAB_SIZE_BITS_0],
    pub extras: [Lisp_Object; 0],  // FLEXIBLE_ARRAY_MEMBER
}

#[repr(C, align(8))]
pub struct Lisp_Sub_Char_Table {
    pub header: vectorlike_header,
    pub depth: c_int,
    pub min_char: c_int,
    pub contents: [Lisp_Object; 0], // FLEXIBLE_ARRAY_MEMBER
}

unsafe extern "C" {
    pub fn char_table_ref (table: Lisp_Object, idx: c_int) -> Lisp_Object;
    pub fn char_table_set (table: Lisp_Object, idx: c_int, val: Lisp_Object);
}

#[inline(always)]
pub fn gc_asize (array: Lisp_Object) -> isize {
    // Like ASIZE, but also usable in the garbage collector.
    unsafe { (*XVECTOR (array)).header.size & !ARRAY_MARK_FLAG }
}

#[inline(always)]
pub fn PVSIZE (pv: Lisp_Object) -> isize { ASIZE (pv) & PSEUDOVECTOR_SIZE_MASK }

#[inline(always)]
pub fn VECTORP (x: Lisp_Object) -> bool {
    VECTORLIKEP (x) && (ASIZE (x) & PSEUDOVECTOR_FLAG) == 0
}

#[inline(always)]
pub fn CHECK_VECTOR (x: Lisp_Object) {
    CHECK_TYPE (VECTORP (x), Qvectorp (), x);
}

#[inline(always)]
pub fn PSEUDOVECTOR_TYPE (v: *mut vectorlike_header) -> pvec_type {
    let size = unsafe { (*v).size };
    if size & PSEUDOVECTOR_FLAG != 0 {
        match (size & PVEC_TYPE_MASK) >> PSEUDOVECTOR_AREA_BITS {
            0 => pvec_type::PVEC_NORMAL_VECTOR,
            1 => pvec_type::PVEC_FREE,
            2 => pvec_type::PVEC_BIGNUM,
            3 => pvec_type::PVEC_MARKER,
            4 => pvec_type::PVEC_OVERLAY,
            5 => pvec_type::PVEC_FINALIZER,
            6 => pvec_type::PVEC_SYMBOL_WITH_POS,
            7 => pvec_type::PVEC_MISC_PTR,
            8 => pvec_type::PVEC_USER_PTR,
            9 => pvec_type::PVEC_PROCESS,
            10 => pvec_type::PVEC_FRAME,
            11 => pvec_type::PVEC_WINDOW,
            12 => pvec_type::PVEC_BOOL_VECTOR,
            13 => pvec_type::PVEC_BUFFER,
            14 => pvec_type::PVEC_HASH_TABLE,
            15 => pvec_type::PVEC_OBARRAY,
            16 => pvec_type::PVEC_TERMINAL,
            17 => pvec_type::PVEC_WINDOW_CONFIGURATION,
            18 => pvec_type::PVEC_SUBR,
            19 => pvec_type::PVEC_OTHER,
            20 => pvec_type::PVEC_XWIDGET,
            21 => pvec_type::PVEC_XWIDGET_VIEW,
            22 => pvec_type::PVEC_THREAD,
            23 => pvec_type::PVEC_MUTEX,
            24 => pvec_type::PVEC_CONDVAR,
            25 => pvec_type::PVEC_MODULE_FUNCTION,
            26 => pvec_type::PVEC_NATIVE_COMP_UNIT,
            27 => pvec_type::PVEC_TS_PARSER,
            28 => pvec_type::PVEC_TS_NODE,
            29 => pvec_type::PVEC_TS_COMPILED_QUERY,
            30 => pvec_type::PVEC_SQLITE,
            31 => pvec_type::PVEC_CLOSURE,
            32 => pvec_type::PVEC_CHAR_TABLE,
            33 => pvec_type::PVEC_SUB_CHAR_TABLE,
            34 => pvec_type::PVEC_RECORD,
            _ => pvec_type::PVEC_FONT,
        }
    } else {
        pvec_type::PVEC_NORMAL_VECTOR
    }
}

#[inline(always)]
pub fn PSEUDOVECTOR_TYPEP (a: *mut vectorlike_header, code: pvec_type) -> bool {
    unsafe { ((*a).size & (PSEUDOVECTOR_FLAG | PVEC_TYPE_MASK))
        == (PSEUDOVECTOR_FLAG | ((code as isize) << PSEUDOVECTOR_AREA_BITS)) }
}

#[inline(always)]
pub fn bool_vector_words (size: EMACS_INT) -> EMACS_INT {
    eassume! (0 <= size && size <= EMACS_INT_MAX - (BITS_PER_BITS_WORD as EMACS_INT - 1));
    (size + (BITS_PER_BITS_WORD as EMACS_INT - 1)) / BITS_PER_BITS_WORD as EMACS_INT
}

#[inline(always)]
pub fn bool_vector_bytes (size: EMACS_INT) -> EMACS_INT {
    eassume! (0 <= size && size <= EMACS_INT_MAX - (BOOL_VECTOR_BITS_PER_CHAR as EMACS_INT - 1));
    (size + (BOOL_VECTOR_BITS_PER_CHAR as EMACS_INT - 1)) / BOOL_VECTOR_BITS_PER_CHAR as EMACS_INT
}

#[inline(always)]
pub fn bits_word_to_host_endian (val: bits_word) -> bits_word {
    // WORDS_BIGENDIAN is not defined on this target, so C returns val unchanged.
    val
}

#[inline(always)]
pub fn BOOL_VECTOR_P (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_BOOL_VECTOR) }

#[inline(always)]
pub fn CHECK_BOOL_VECTOR (x: Lisp_Object) {
    CHECK_TYPE (BOOL_VECTOR_P (x), Qbool_vector_p (), x);
}

#[inline(always)]
pub fn XBOOL_VECTOR (a: Lisp_Object) -> *mut Lisp_Bool_Vector {
    eassert! (BOOL_VECTOR_P (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Bool_Vector)
}

#[inline(always)]
pub fn bool_vector_size (a: Lisp_Object) -> EMACS_INT {
    let size = unsafe { (*XBOOL_VECTOR (a)).size };
    eassume! (0 <= size);
    size
}

#[inline(always)]
pub fn bool_vector_data (a: Lisp_Object) -> *mut bits_word {
    unsafe { std::ptr::addr_of_mut! ((*XBOOL_VECTOR (a)).data) as *mut bits_word }
}

#[inline(always)]
pub fn bool_vector_uchar_data (a: Lisp_Object) -> *mut u8 { bool_vector_data (a) as *mut u8 }

#[inline(always)]
pub fn bool_vector_bitref (a: Lisp_Object, i: EMACS_INT) -> bool {
    eassume! (0 <= i);
    eassert! (i < bool_vector_size (a));
    unsafe {
        *bool_vector_uchar_data (a).add ((i / BOOL_VECTOR_BITS_PER_CHAR as EMACS_INT) as usize)
            & (1 << (i % BOOL_VECTOR_BITS_PER_CHAR as EMACS_INT)) != 0
    }
}

#[inline(always)]
pub fn bool_vector_ref (a: Lisp_Object, i: EMACS_INT) -> Lisp_Object {
    if bool_vector_bitref (a, i) { Qt () } else { Qnil () }
}

#[inline(always)]
pub fn bool_vector_set (a: Lisp_Object, i: EMACS_INT, b: bool) {
    eassume! (0 <= i);
    eassert! (i < bool_vector_size (a));
    unsafe {
        let addr = bool_vector_uchar_data (a)
            .add ((i / BOOL_VECTOR_BITS_PER_CHAR as EMACS_INT) as usize);
        let bit = 1u8 << (i % BOOL_VECTOR_BITS_PER_CHAR as EMACS_INT);
        if b { *addr |= bit } else { *addr &= !bit }
    }
}

#[inline(always)]
pub fn AREF (array: Lisp_Object, idx: isize) -> Lisp_Object {
    eassert! (0 <= idx && idx < gc_asize (array));
    unsafe { (*XVECTOR (array)).contents[idx as usize] }
}

#[inline(always)]
pub fn aref_addr (array: Lisp_Object, idx: isize) -> *mut Lisp_Object {
    eassert! (0 <= idx && idx <= gc_asize (array));
    unsafe { std::ptr::addr_of_mut! ((*XVECTOR (array)).contents[idx as usize]) }
}

#[inline(always)]
pub fn ASET (array: Lisp_Object, idx: isize, val: Lisp_Object) {
    eassert! (0 <= idx && idx < ASIZE (array));
    unsafe { (*XVECTOR (array)).contents[idx as usize] = val }
}

#[inline(always)]
pub fn gc_aset (array: Lisp_Object, idx: isize, val: Lisp_Object) {
    eassert! (0 <= idx && idx < gc_asize (array));
    unsafe { (*XVECTOR (array)).contents[idx as usize] = val }
}

#[inline(always)]
pub fn memclear (p: *mut c_void, nbytes: isize) {
    eassert! (0 <= nbytes);
    // Since Qnil is zero, memset suffices.
    unsafe { std::ptr::write_bytes (p as *mut u8, 0, nbytes as usize) }
}

#[inline(always)]
pub fn ASCII_CHAR_P (c: isize) -> bool { 0 <= c && c < 0x80 }

#[inline(always)]
pub fn CHAR_TABLE_P (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_CHAR_TABLE) }

#[inline(always)]
pub fn XCHAR_TABLE (a: Lisp_Object) -> *mut Lisp_Char_Table {
    eassert! (CHAR_TABLE_P (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Char_Table)
}

#[inline(always)]
pub fn SUB_CHAR_TABLE_P (a: Lisp_Object) -> bool {
    PSEUDOVECTORP (a, pvec_type::PVEC_SUB_CHAR_TABLE)
}

#[inline(always)]
pub fn XSUB_CHAR_TABLE (a: Lisp_Object) -> *mut Lisp_Sub_Char_Table {
    eassert! (SUB_CHAR_TABLE_P (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Sub_Char_Table)
}

#[inline(always)]
pub fn CHAR_TABLE_REF_ASCII (ct: Lisp_Object, idx: isize) -> Lisp_Object {
    let mut tbl = XCHAR_TABLE (ct);
    loop {
        let val = unsafe {
            let ascii = (*tbl).ascii;
            if SUB_CHAR_TABLE_P (ascii) {
                (*XSUB_CHAR_TABLE (ascii)).contents[idx as usize]
            } else {
                ascii
            }
        };
        let val = if NILP (val) { unsafe { (*tbl).defalt } } else { val };
        if !NILP (val) || NILP (unsafe { (*tbl).parent }) {
            return val;
        }
        tbl = XCHAR_TABLE (unsafe { (*tbl).parent });
    }
}

#[inline(always)]
pub fn CHAR_TABLE_REF (ct: Lisp_Object, idx: isize) -> Lisp_Object {
    if ASCII_CHAR_P (idx) {
        CHAR_TABLE_REF_ASCII (ct, idx)
    } else {
        unsafe { char_table_ref (ct, idx as c_int) }
    }
}

#[inline(always)]
pub fn CHAR_TABLE_SET (ct: Lisp_Object, idx: isize, val: Lisp_Object) {
    if ASCII_CHAR_P (idx) && SUB_CHAR_TABLE_P (unsafe { (*XCHAR_TABLE (ct)).ascii }) {
        unsafe { set_sub_char_table_contents ((*XCHAR_TABLE (ct)).ascii, idx, val) };
    } else {
        unsafe { char_table_set (ct, idx as c_int, val) }
    }
}

#[inline(always)]
pub fn CHAR_TABLE_EXTRA_SLOTS (ct: *mut Lisp_Char_Table) -> isize {
    unsafe { ((*ct).header.size & PSEUDOVECTOR_SIZE_MASK) - CHAR_TABLE_STANDARD_SLOTS }
}

/// src/lisp.h 2243: PSEUDOVECSIZE (struct Lisp_Char_Table, contents) - 1 +
/// (1 << CHARTAB_SIZE_BITS_0); the four preceding slots are defalt, parent,
/// purpose and ascii after the header.
pub const CHAR_TABLE_STANDARD_SLOTS: isize = 4 + (1 << CHARTAB_SIZE_BITS_0);

#[inline(always)]
pub fn SUBRP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_SUBR) }

#[inline(always)]
pub fn XSUBR (a: Lisp_Object) -> *mut Lisp_Subr {
    eassert! (SUBRP (a));
    unsafe { std::ptr::addr_of_mut! ((*XUNTAG! (a, Lisp_Type::Lisp_Vectorlike,
                                                Aligned_Lisp_Subr)).s) }
}

#[inline(always)]
pub fn VALID_DOCSTRING_P (doc: Lisp_Object) -> bool {
    FIXNUMP (doc) || STRINGP (doc)
        || (CONSP (doc) && STRINGP (XCAR (doc)) && FIXNUMP (XCDR (doc)))
}

// --- symbol fields (src/lisp.h 2290-2390) ----------------------------------

#[inline(always)]
pub fn SYMBOL_VAL (sym: *mut Lisp_Symbol) -> Lisp_Object {
    eassert! (unsafe { (*sym).u.s.bits & SYMBOL_REDIRECT_MASK } >> SYMBOL_REDIRECT_SHIFT == SYMBOL_PLAINVAL);
    unsafe { (*sym).u.s.val.value }
}

#[inline(always)]
pub fn SYMBOL_ALIAS (sym: *mut Lisp_Symbol) -> *mut Lisp_Symbol {
    eassume! (unsafe { ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT } == SYMBOL_VARALIAS
        && ! unsafe { (*sym).u.s.val.alias }.is_null ());
    unsafe { (*sym).u.s.val.alias }
}

#[inline(always)]
pub fn SYMBOL_BLV (sym: *mut Lisp_Symbol) -> *mut c_void {
    eassume! (unsafe { ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT } == SYMBOL_LOCALIZED
        && ! unsafe { (*sym).u.s.val.blv }.is_null ());
    unsafe { (*sym).u.s.val.blv }
}

#[inline(always)]
pub fn SYMBOL_FWD (sym: *mut Lisp_Symbol) -> *mut c_void {
    eassume! (unsafe { ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT } == SYMBOL_FORWARDED
        && ! unsafe { (*sym).u.s.val.fwd }.is_null ());
    unsafe { (*sym).u.s.val.fwd }
}

#[inline(always)]
pub fn SET_SYMBOL_VAL (sym: *mut Lisp_Symbol, v: Lisp_Object) {
    eassert! (unsafe { ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT } == SYMBOL_PLAINVAL);
    unsafe { (*sym).u.s.val.value = v }
}

#[inline(always)]
pub fn SET_SYMBOL_ALIAS (sym: *mut Lisp_Symbol, v: *mut Lisp_Symbol) {
    eassume! (unsafe { ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT } == SYMBOL_VARALIAS
        && ! v.is_null ());
    unsafe { (*sym).u.s.val.alias = v }
}

#[inline(always)]
pub fn SET_SYMBOL_BLV (sym: *mut Lisp_Symbol, v: *mut c_void) {
    eassume! (unsafe { ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT } == SYMBOL_LOCALIZED
        && ! v.is_null ());
    unsafe { (*sym).u.s.val.blv = v }
}

#[inline(always)]
pub fn SET_SYMBOL_FWD (sym: *mut Lisp_Symbol, fwd: *mut c_void) {
    eassume! (unsafe { ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT } == SYMBOL_FORWARDED
        && ! fwd.is_null ());
    unsafe { (*sym).u.s.val.fwd = fwd }
}

#[inline(always)]
pub fn SYMBOL_NAME (sym: Lisp_Object) -> Lisp_Object { unsafe { (*XSYMBOL (sym)).u.s.name } }

#[inline(always)]
pub fn lisp_h_SYMBOL_TRAPPED_WRITE_P (sym: Lisp_Object) -> u32 {
    unsafe { ((*XSYMBOL (sym)).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK) >> SYMBOL_TRAPPED_WRITE_SHIFT }
}

#[inline(always)]
pub fn SYMBOL_TRAPPED_WRITE_P (sym: Lisp_Object) -> u32 {
    lisp_h_SYMBOL_TRAPPED_WRITE_P (sym)
}

#[inline(always)]
pub fn lisp_h_SYMBOL_CONSTANT_P (sym: Lisp_Object) -> bool {
    lisp_h_SYMBOL_TRAPPED_WRITE_P (sym) == SYMBOL_NOWRITE
}

#[inline(always)]
pub fn SYMBOL_CONSTANT_P (sym: Lisp_Object) -> bool { lisp_h_SYMBOL_CONSTANT_P (sym) }

#[inline(always)]
pub fn SYMBOL_INTERNED_P (sym: Lisp_Object) -> bool {
    unsafe { ((*XSYMBOL (sym)).u.s.bits & SYMBOL_INTERNED_MASK) >> SYMBOL_INTERNED_SHIFT
        != SYMBOL_UNINTERNED }
}

#[inline(always)]
pub fn SYMBOL_INTERNED_IN_INITIAL_OBARRAY_P (sym: Lisp_Object) -> bool {
    unsafe { ((*XSYMBOL (sym)).u.s.bits & SYMBOL_INTERNED_MASK) >> SYMBOL_INTERNED_SHIFT
        == SYMBOL_INTERNED_IN_INITIAL_OBARRAY }
}

unsafe extern "C" {
    pub fn set_sub_char_table_contents (table: Lisp_Object, idx: isize, val: Lisp_Object);
}

// --- batch 4: misc pointers, overlays, finalizers, fwd objects (src/lisp.h 2890-3390) ---
// Fwd object types (src/lisp.h 528) and the buffer-obj predicate (3120).
pub const Lisp_Fwd_Int: u32 = 0;
pub const Lisp_Fwd_Bool: u32 = 1;
pub const Lisp_Fwd_Obj: u32 = 2;
pub const Lisp_Fwd_Buffer_Obj: u32 = 3;
pub const Lisp_Fwd_Kboard_Obj: u32 = 4;

// enum Lisp_Fwd_Predicate (src/lisp.h 3099), in the C member order.
pub const FWDPRED_Qnil: u32 = 0;
pub const FWDPRED_Qintegerp: u32 = 1;
pub const FWDPRED_Qsymbolp: u32 = 2;
pub const FWDPRED_Qstringp: u32 = 3;
pub const FWDPRED_Qnumberp: u32 = 4;
pub const FWDPRED_Qfraction: u32 = 5;
pub const FWDPRED_Qvertical_scroll_bar: u32 = 6;
pub const FWDPRED_Qoverwrite_mode: u32 = 7;

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Overlay {
    pub header: vectorlike_header,
    pub plist: Lisp_Object,
    pub buffer: *mut c_void,        // struct buffer *
    pub interval: *mut c_void,      // struct itree_node *
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Misc_Ptr {
    pub header: vectorlike_header,
    pub pointer: *mut c_void,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_User_Ptr {
    pub header: vectorlike_header,
    pub finalizer: Option<unsafe extern "C" fn(*mut c_void)>,
    pub p: *mut c_void,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Finalizer {
    pub header: vectorlike_header,
    pub function: Lisp_Object,
    pub prev: *mut Lisp_Finalizer,
    pub next: *mut Lisp_Finalizer,
}

/// struct Lisp_Fwd (src/lisp.h 3124): one enum bitfield followed by a union.
#[repr(C)]
#[derive(Clone, Copy)]
pub union Lisp_Fwd_u {
    pub intvar: *mut i64,
    pub boolvar: *mut bool,
    pub objvar: *mut Lisp_Object,
    pub buf: Lisp_Fwd_buf,
    pub kbdoffset: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_Fwd_buf {
    pub offset: u16,
    pub predicate: u8,
    pub _pad: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_Fwd {
    pub type_: u32,               // ENUM_BF (Lisp_Fwd_Type) type : 8 in C's first word
    pub u: Lisp_Fwd_u,
}

unsafe extern "C" {
    pub fn make_misc_ptr (a: *mut c_void) -> Lisp_Object;
    pub fn bignum_to_double (n: Lisp_Object) -> f64;
    pub fn make_bigint (n: i64) -> Lisp_Object;
    pub fn make_biguint (n: u64) -> Lisp_Object;
}

#[inline(always)]
pub fn FINALIZERP (x: Lisp_Object) -> bool { PSEUDOVECTORP (x, pvec_type::PVEC_FINALIZER) }

#[inline(always)]
pub fn XFINALIZER (a: Lisp_Object) -> *mut Lisp_Finalizer {
    eassert! (FINALIZERP (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Finalizer)
}

#[inline(always)]
pub fn MARKERP (x: Lisp_Object) -> bool { PSEUDOVECTORP (x, pvec_type::PVEC_MARKER) }

#[inline(always)]
pub fn XMARKER (a: Lisp_Object) -> *mut Lisp_Marker {
    eassert! (MARKERP (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Marker)
}

#[inline(always)]
pub fn OVERLAYP (x: Lisp_Object) -> bool { PSEUDOVECTORP (x, pvec_type::PVEC_OVERLAY) }

#[inline(always)]
pub fn XOVERLAY (a: Lisp_Object) -> *mut Lisp_Overlay {
    eassert! (OVERLAYP (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Overlay)
}

#[inline(always)]
pub fn USER_PTRP (x: Lisp_Object) -> bool { PSEUDOVECTORP (x, pvec_type::PVEC_USER_PTR) }

#[inline(always)]
pub fn XUSER_PTR (a: Lisp_Object) -> *mut Lisp_User_Ptr {
    eassert! (USER_PTRP (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_User_Ptr)
}

#[inline(always)]
pub fn make_mint_ptr (a: *mut c_void) -> Lisp_Object {
    let val = TAG_PTR_INITIALLY (Lisp_Type::Lisp_Int0, a as EMACS_INT);
    if FIXNUMP (val) && XFIXNUMPTR (val) == a as *mut c_char {
        val
    } else {
        unsafe { make_misc_ptr (a) }
    }
}

#[inline(always)]
pub fn mint_ptrp (x: Lisp_Object) -> bool {
    FIXNUMP (x) || PSEUDOVECTORP (x, pvec_type::PVEC_MISC_PTR)
}

#[inline(always)]
pub fn xmint_pointer (a: Lisp_Object) -> *mut c_void {
    eassert! (mint_ptrp (a));
    if FIXNUMP (a) {
        XFIXNUMPTR (a) as *mut c_void
    } else {
        unsafe { (*XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Misc_Ptr)).pointer }
    }
}

#[inline(always)]
pub fn XFWDTYPE (a: *mut Lisp_Fwd) -> u32 { unsafe { (*a).type_ } }

#[inline(always)]
pub fn BUFFER_OBJFWDP (a: *mut Lisp_Fwd) -> bool { XFWDTYPE (a) == Lisp_Fwd_Buffer_Obj }

#[inline(always)]
pub fn XBUFFER_OFFSET (a: *mut Lisp_Fwd) -> u16 {
    eassert! (BUFFER_OBJFWDP (a));
    unsafe { (*a).u.buf.offset }
}

#[inline(always)]
pub fn KBOARD_OBJFWDP (a: *mut Lisp_Fwd) -> bool { XFWDTYPE (a) == Lisp_Fwd_Kboard_Obj }

#[inline(always)]
pub fn XFLOAT (a: Lisp_Object) -> *mut Lisp_Float {
    eassert! (FLOATP (a));
    XUNTAG! (a, Lisp_Type::Lisp_Float, Lisp_Float)
}

#[inline(always)]
pub fn XFLOAT_DATA (f: Lisp_Object) -> f64 { unsafe { (*XFLOAT (f)).u.data } }

#[inline(always)]
pub fn lisp_h_CHECK_FIXNUM (x: Lisp_Object) {
    CHECK_TYPE (FIXNUMP (x), Qfixnump (), x);
}

#[inline(always)]
pub fn CHECK_FIXNUM (x: Lisp_Object) { lisp_h_CHECK_FIXNUM (x) }

#[inline(always)]
pub fn FIXNATP (x: Lisp_Object) -> bool { FIXNUMP (x) && 0 <= XFIXNUM (x) }

#[inline(always)]
pub fn XFIXNAT (a: Lisp_Object) -> EMACS_INT {
    eassert! (FIXNUMP (a));
    let result = XFIXNUM (a);
    eassume! (0 <= result);
    result
}

#[inline(always)]
pub fn NUMBERP (x: Lisp_Object) -> bool { INTEGERP (x) || FLOATP (x) }

#[inline(always)]
pub fn RANGED_FIXNUMP (lo: EMACS_INT, x: Lisp_Object, hi: EMACS_INT) -> bool {
    FIXNUMP (x) && lo <= XFIXNUM (x) && XFIXNUM (x) <= hi
}

#[inline(always)]
pub fn XFLOATINT (n: Lisp_Object) -> f64 {
    if FIXNUMP (n) {
        XFIXNUM (n) as f64
    } else if FLOATP (n) {
        XFLOAT_DATA (n)
    } else {
        unsafe { bignum_to_double (n) }
    }
}

#[inline(always)]
pub fn CHECK_NUMBER (x: Lisp_Object) { CHECK_TYPE (NUMBERP (x), Qnumberp (), x) }

#[inline(always)]
pub fn CHECK_INTEGER (x: Lisp_Object) { CHECK_TYPE (INTEGERP (x), Qintegerp (), x) }

#[inline(always)]
pub fn CHECK_FIXNAT (x: Lisp_Object) { CHECK_TYPE (FIXNATP (x), Qwholenump (), x) }

#[inline(always)]
pub fn make_int (n: EMACS_INT) -> Lisp_Object {
    if FIXNUM_OVERFLOW_P (n) { unsafe { make_bigint (n as i64) } } else { make_fixnum (n) }
}

#[inline(always)]
pub fn make_uint (n: EMACS_INT) -> Lisp_Object {
    if FIXNUM_OVERFLOW_P (n) { unsafe { make_biguint (n as u64) } } else { make_fixnum (n) }
}

#[inline(always)]
pub fn BIGNUMP (x: Lisp_Object) -> bool { PSEUDOVECTORP (x, pvec_type::PVEC_BIGNUM) }

#[inline(always)]
pub fn INTEGERP (x: Lisp_Object) -> bool { FIXNUMP (x) || BIGNUMP (x) }

#[inline(always)]
pub fn ARRAYP (x: Lisp_Object) -> bool {
    VECTORP (x) || STRINGP (x) || CHAR_TABLE_P (x) || BOOL_VECTOR_P (x)
}

#[inline(always)]
pub fn IMAGEP (x: Lisp_Object) -> bool { CONSP (x) && EQ (XCAR (x), Qimage ()) }

#[inline(always)]
pub fn RECORDP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_RECORD) }

#[inline(always)]
pub fn CHECK_RECORD (x: Lisp_Object) { CHECK_TYPE (RECORDP (x), Qrecordp (), x) }

#[inline(always)]
pub fn FRAMEP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_FRAME) }

#[inline(always)]
pub fn CLOSUREP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_CLOSURE) }

#[inline(always)]
pub fn AUTOLOADP (x: Lisp_Object) -> bool { CONSP (x) && EQ (Qautoload (), XCAR (x)) }

#[inline(always)]
pub fn WINDOW_CONFIGURATIONP (a: Lisp_Object) -> bool {
    PSEUDOVECTORP (a, pvec_type::PVEC_WINDOW_CONFIGURATION)
}

// --- batch 5: specpdl, buffer-local values, constructors, setters (src/lisp.h 3540-3900) ---
// --- specpdl (src/lisp.h 3540-3790) ----------------------------------------
// specpdl_ref is the wrapped form on 64-bit targets (WRAP_SPECPDL_REF is 1).

pub const SPECPDL_UNWIND: u32 = 0;
pub const SPECPDL_UNWIND_ARRAY: u32 = 1;
pub const SPECPDL_UNWIND_PTR: u32 = 2;
pub const SPECPDL_UNWIND_INT: u32 = 3;
pub const SPECPDL_UNWIND_INTMAX: u32 = 4;
pub const SPECPDL_UNWIND_EXCURSION: u32 = 5;
pub const SPECPDL_UNWIND_VOID: u32 = 6;
pub const SPECPDL_BACKTRACE: u32 = 7;
pub const SPECPDL_NOP: u32 = 8;
pub const SPECPDL_LET: u32 = 9;
pub const SPECPDL_LET_LOCAL: u32 = 10;
pub const SPECPDL_LET_DEFAULT: u32 = 11;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct specpdl_ref {
    pub bytes: isize,
}

// Each variant of the union starts with an 8-bit `kind` bitfield followed by
// pointer-aligned members, so the mirror keeps one byte and pads to the word.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct specbinding_unwind {
    pub kind: u8, pub _pad: [u8; 7],
    pub func: Option<unsafe extern "C" fn(Lisp_Object)>,
    pub arg: Lisp_Object,
    pub eval_depth: EMACS_INT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct specbinding_unwind_array {
    pub kind: u8, pub _pad: [u8; 7],
    pub nelts: isize,
    pub array: *mut Lisp_Object,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct specbinding_unwind_ptr {
    pub kind: u8, pub _pad: [u8; 7],
    pub func: Option<unsafe extern "C" fn(*mut c_void)>,
    pub arg: *mut c_void,
    pub mark: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct specbinding_unwind_int {
    pub kind: u8, pub _pad: [u8; 7],
    pub func: Option<unsafe extern "C" fn(c_int)>,
    pub arg: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct specbinding_unwind_intmax {
    pub kind: u8, pub _pad: [u8; 7],
    pub func: Option<unsafe extern "C" fn(i64)>,
    pub arg: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct specbinding_unwind_excursion {
    pub kind: u8, pub _pad: [u8; 7],
    pub marker: Lisp_Object,
    pub window: Lisp_Object,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct specbinding_unwind_void {
    pub kind: u8, pub _pad: [u8; 7],
    pub func: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union specbinding_let_where {
    pub kbd: *mut c_void,          // KBOARD *
    pub buf: Lisp_Object,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct specbinding_let {
    pub kind: u8, pub _pad: [u8; 7],
    pub symbol: Lisp_Object,
    pub old_value: Lisp_Object,
    pub where_: specbinding_let_where,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct specbinding_bt {
    pub kind: u8, pub debug_on_exit: u8, pub _pad: [u8; 6],
    pub function: Lisp_Object,
    pub args: *mut Lisp_Object,
    pub nargs: isize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union specbinding {
    pub kind: u8,
    pub unwind: specbinding_unwind,
    pub unwind_array: specbinding_unwind_array,
    pub unwind_ptr: specbinding_unwind_ptr,
    pub unwind_int: specbinding_unwind_int,
    pub unwind_intmax: specbinding_unwind_intmax,
    pub unwind_excursion: specbinding_unwind_excursion,
    pub unwind_void: specbinding_unwind_void,
    pub let_: specbinding_let,
    pub bt: specbinding_bt,
}

/// The specpdl itself is per-thread state in C (`#define specpdl
/// (current_thread->m_specpdl)`); the mirror declares the storage here and the
/// thread accessor arrives with thread.rs.
unsafe extern "C" {
    pub static mut specpdl: *mut specbinding;
    pub static mut specpdl_ptr: *mut specbinding;
    pub static mut specpdl_end: *mut specbinding;
}

#[inline(always)]
pub fn wrap_specpdl_ref (bytes: isize) -> specpdl_ref { specpdl_ref { bytes } }

#[inline(always)]
pub fn unwrap_specpdl_ref (r: specpdl_ref) -> isize { r.bytes }

#[inline(always)]
pub fn specpdl_count_to_ref (count: isize) -> specpdl_ref {
    wrap_specpdl_ref (count * std::mem::size_of::<specbinding>() as isize)
}

#[inline(always)]
pub fn specpdl_ref_to_count (r: specpdl_ref) -> isize {
    unwrap_specpdl_ref (r) / std::mem::size_of::<specbinding>() as isize
}

#[inline(always)]
pub fn specpdl_ref_eq (a: specpdl_ref, b: specpdl_ref) -> bool {
    unwrap_specpdl_ref (a) == unwrap_specpdl_ref (b)
}

#[inline(always)]
pub fn specpdl_ref_lt (a: specpdl_ref, b: specpdl_ref) -> bool {
    unwrap_specpdl_ref (a) < unwrap_specpdl_ref (b)
}

#[inline(always)]
pub fn specpdl_ref_valid_p (r: specpdl_ref) -> bool { unwrap_specpdl_ref (r) >= 0 }

#[inline(always)]
pub fn make_invalid_specpdl_ref () -> specpdl_ref { wrap_specpdl_ref (-1) }

#[inline(always)]
pub fn specpdl_ref_add (r: specpdl_ref, delta: isize) -> specpdl_ref {
    wrap_specpdl_ref (unwrap_specpdl_ref (r) + delta * std::mem::size_of::<specbinding>() as isize)
}

#[inline(always)]
pub fn backtrace_debug_on_exit (pdl: *mut specbinding) -> bool {
    eassert! (unsafe { (*pdl).kind } == SPECPDL_BACKTRACE as u8);
    unsafe { (*pdl).bt.debug_on_exit != 0 }
}

// --- buffer-local values (src/lisp.h 3110) ---------------------------------

/// lispfwd is `struct Lisp_Fwd *`.
pub type lispfwd = *mut Lisp_Fwd;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_Buffer_Local_Value {
    pub bits: u32,                 // local_if_set:1, found:1 (bool_bf bitfields)
    pub _pad: u32,
    pub fwd: lispfwd,
    pub where_: Lisp_Object,
    pub defcell: Lisp_Object,
    pub valcell: Lisp_Object,
}

pub const BLV_LOCAL_IF_SET: u32 = 1 << 0;
pub const BLV_FOUND: u32 = 1 << 1;

#[inline(always)]
pub fn blv_found (blv: *mut Lisp_Buffer_Local_Value) -> bool {
    eassert! ((unsafe { (*blv).bits } & BLV_FOUND != 0)
        == !BASE_EQ (unsafe { (*blv).defcell }, unsafe { (*blv).valcell }));
    (unsafe { (*blv).bits } & BLV_FOUND) != 0
}

// --- constructors and helpers ----------------------------------------------

unsafe extern "C" {
    pub fn list1 (a: Lisp_Object) -> Lisp_Object;
    pub fn list2 (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
    pub fn list3 (a: Lisp_Object, b: Lisp_Object, c: Lisp_Object) -> Lisp_Object;
    pub fn list4 (a: Lisp_Object, b: Lisp_Object, c: Lisp_Object, d: Lisp_Object) -> Lisp_Object;
    pub fn allocate_nil_vector (size: isize) -> *mut Lisp_Vector;
    pub fn vectorlike_nbytes (h: *const vectorlike_header) -> isize;
    pub fn bignum_to_intmax (num: Lisp_Object) -> i64;
    pub fn bignum_to_uintmax (num: Lisp_Object) -> u64;
    pub fn flush_stack_call_func1 (func: Option<unsafe extern "C" fn(*mut c_void)>, arg: *mut c_void);
}

#[inline(always)]
pub fn xvector_contents_addr (a: Lisp_Object, i: isize) -> *mut c_void {
    let offset = (std::mem::size_of::<vectorlike_header>()
        + i as usize * std::mem::size_of::<Lisp_Object>()) as isize;
    unsafe { (XVECTOR (a) as *mut u8).offset (offset) as *mut c_void }
}

#[inline(always)]
pub fn xvector_contents (a: Lisp_Object) -> *mut c_void { xvector_contents_addr (a, 0) }

#[inline(always)]
pub fn vcopy (v: Lisp_Object, offset: isize, args: *const Lisp_Object, count: isize) {
    eassert! (0 <= offset && 0 <= count && offset + count <= ASIZE (v));
    unsafe {
        std::ptr::copy_nonoverlapping (args, xvector_contents_addr (v, offset) as *mut Lisp_Object,
                                       count as usize)
    }
}

#[inline(always)]
pub fn vector_nbytes (v: *mut Lisp_Vector) -> isize {
    unsafe { vectorlike_nbytes (&raw const (*v).header) }
}

#[inline(always)]
pub fn make_nil_vector (size: isize) -> Lisp_Object {
    make_lisp_ptr (unsafe { allocate_nil_vector (size) } as *mut c_void, Lisp_Type::Lisp_Vectorlike)
}

#[inline(always)]
pub fn list1i (a: EMACS_INT) -> Lisp_Object { unsafe { list1 (make_int (a)) } }

#[inline(always)]
pub fn list2i (a: EMACS_INT, b: EMACS_INT) -> Lisp_Object {
    unsafe { list2 (make_int (a), make_int (b)) }
}

#[inline(always)]
pub fn list3i (a: EMACS_INT, b: EMACS_INT, c: EMACS_INT) -> Lisp_Object {
    unsafe { list3 (make_int (a), make_int (b), make_int (c)) }
}

#[inline(always)]
pub fn list4i (a: EMACS_INT, b: EMACS_INT, c: EMACS_INT, d: EMACS_INT) -> Lisp_Object {
    unsafe { list4 (make_int (a), make_int (b), make_int (c), make_int (d)) }
}

/// elogb: the smallest power of two exponent, from stdc_bit_width.
#[inline(always)]
pub fn elogb (n: u64) -> c_int {
    let width = 64 - n.leading_zeros();
    (width - 1) as c_int
}

#[inline(always)]
pub fn integer_to_intmax (num: Lisp_Object, n: *mut i64) -> bool {
    if FIXNUMP (num) {
        unsafe { *n = XFIXNUM (num) as i64 };
        true
    } else {
        let i = unsafe { bignum_to_intmax (num) };
        unsafe { *n = i };
        i != 0 || INTEGERP (num)
    }
}

#[inline(always)]
pub fn integer_to_uintmax (num: Lisp_Object, n: *mut u64) -> bool {
    if FIXNUMP (num) {
        let v = XFIXNUM (num);
        unsafe { *n = v as u64 };
        0 <= v
    } else {
        let i = unsafe { bignum_to_uintmax (num) };
        unsafe { *n = i };
        i != 0 || INTEGERP (num)
    }
}

#[inline(always)]
pub fn modiff_incr (a: *mut i64, len: isize) {
    let a0 = unsafe { *a };
    // Increase more for a large modification, logarithmically.
    let incr: i64 = if len == 0 { 1 } else { elogb (len as u64) as i64 + 1 };
    let (v, overflow) = a0.overflowing_add (incr);
    eassert! (!overflow && v >> 30 >> 30 == 0);
    unsafe { *a = v };
}

#[inline(always)]
pub fn modiff_to_integer (a: i64) -> Lisp_Object {
    eassume! (0 <= a && a >> 30 >> 30 == 0);
    make_int (a as EMACS_INT)
}

#[inline(always)]
pub fn flush_stack_call_func (func: Option<unsafe extern "C" fn(*mut c_void)>, arg: *mut c_void) {
    // C calls __builtin_unwind_init() here so the GC can see the registers; the
    // port uses a setjmp-based barrier (PORTING.md 4.3) and forbids tail calls
    // in this path (the C adds an empty asm for the same reason, bug#65727).
    flush_stack_barrier ();
    unsafe { flush_stack_call_func1 (func, arg) };
    flush_stack_barrier ();
}

#[inline(never)]
fn flush_stack_barrier () {
    // SAFETY: a setjmp writes the callee-saved registers to this frame, which is
    // the mechanism alloc.c relies on before it scans the stack.
    unsafe {
        let mut buf = std::mem::MaybeUninit::<[u8; 256]>::uninit ();
        setjmp_for_barrier (buf.as_mut_ptr () as *mut c_void);
        std::hint::black_box (&mut buf);
    }
}

unsafe extern "C" {
    #[link_name = "_setjmp"]
    fn setjmp_for_barrier (b: *mut c_void) -> c_int;
}

// --- setters ---------------------------------------------------------------

#[inline(always)]
pub fn set_symbol_function (sym: Lisp_Object, function: Lisp_Object) {
    unsafe { (*XSYMBOL (sym)).u.s.function = function }
}

#[inline(always)]
pub fn set_symbol_plist (sym: Lisp_Object, plist: Lisp_Object) {
    unsafe { (*XSYMBOL (sym)).u.s.plist = plist }
}

#[inline(always)]
pub fn set_symbol_next (sym: Lisp_Object, next: *mut Lisp_Symbol) {
    unsafe { (*XSYMBOL (sym)).u.s.next = next }
}

#[inline(always)]
pub fn set_symbol_trapped_write (sym: Lisp_Object, trapped_write: u32) {
    unsafe {
        (*XSYMBOL (sym)).u.s.bits =
            ((*XSYMBOL (sym)).u.s.bits & !SYMBOL_TRAPPED_WRITE_MASK)
                | ((trapped_write << SYMBOL_TRAPPED_WRITE_SHIFT) & SYMBOL_TRAPPED_WRITE_MASK)
    }
}

#[inline(always)]
pub fn make_symbol_constant (sym: Lisp_Object) {
    set_symbol_trapped_write (sym, SYMBOL_NOWRITE);
}

#[inline(always)]
pub fn set_overlay_plist (overlay: Lisp_Object, plist: Lisp_Object) {
    unsafe { (*XOVERLAY (overlay)).plist = plist }
}

#[inline(always)]
pub fn string_intervals (s: Lisp_Object) -> *mut c_void {
    unsafe { (*XSTRING (s)).u.s.intervals }
}

#[inline(always)]
pub fn set_string_intervals (s: Lisp_Object, i: *mut c_void) {
    unsafe { (*XSTRING (s)).u.s.intervals = i }
}

#[inline(always)]
pub fn set_char_table_defalt (table: Lisp_Object, val: Lisp_Object) {
    unsafe { (*XCHAR_TABLE (table)).defalt = val }
}

#[inline(always)]
pub fn set_char_table_purpose (table: Lisp_Object, val: Lisp_Object) {
    unsafe { (*XCHAR_TABLE (table)).purpose = val }
}

#[inline(always)]
pub fn set_char_table_extras (table: Lisp_Object, idx: isize, val: Lisp_Object) {
    eassert! (0 <= idx && idx < CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (table)));
    unsafe { (*XCHAR_TABLE (table)).extras[idx as usize] = val }
}

#[inline(always)]
pub fn set_char_table_contents (table: Lisp_Object, idx: isize, val: Lisp_Object) {
    eassert! (0 <= idx && idx < (1 << CHARTAB_SIZE_BITS_0));
    unsafe { (*XCHAR_TABLE (table)).contents[idx as usize] = val }
}

// --- batch 6: obarrays, hash tables, sqlite, checks, constructors (src/lisp.h 2400-3400) ---
pub type hash_hash_t = c_uint;
pub type hash_idx_t = isize;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct hash_table_test {
    pub hashfn: Option<unsafe extern "C" fn(Lisp_Object, *mut Lisp_Hash_Table) -> hash_hash_t>,
    pub cmpfn: Lisp_Object,
    pub user_hash_function: Lisp_Object,
    pub user_cmp_function: Lisp_Object,
    pub name: Lisp_Object,
}

#[repr(C, align(8))]
pub struct Lisp_Obarray {
    pub header: vectorlike_header,
    pub buckets: *mut Lisp_Object,
    pub size_bits: c_uint,
    pub count: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct obarray_iter_t {
    pub o: *mut Lisp_Obarray,
    pub idx: isize,
    pub symbol: *mut Lisp_Symbol,
}

#[repr(C, align(8))]
pub struct Lisp_Hash_Table {
    pub header: vectorlike_header,
    pub index: *mut hash_idx_t,
    pub hash: *mut hash_hash_t,
    pub key_and_value: *mut Lisp_Object,
    pub test: *const hash_table_test,
    pub next: *mut hash_idx_t,
    pub count: hash_idx_t,
    pub next_free: hash_idx_t,
    pub table_size: hash_idx_t,
    pub index_bits: u8,
    pub _pad: [u8; 3],
    /// weakness:3, frozen_test:2, mutable:1 (ENUM_BF/bool_bf bitfields)
    pub bits: u32,
    pub next_weak: *mut Lisp_Hash_Table,
}

pub const HASH_WEAKNESS_KEY: u32 = 1;
pub const HASH_WEAKNESS_VALUE: u32 = 2;
pub const HASH_WEAKNESS_KEY_OR_VALUE: u32 = 3;
pub const HASH_WEAKNESS_NONE: u32 = 0;
pub const HASH_TABLE_MUTABLE: u32 = 1 << 5;

#[repr(C, align(8))]
pub struct Lisp_Sqlite {
    pub header: vectorlike_header,
    pub db: *mut c_void,
    pub stmt: *mut c_void,
    pub name: *mut c_char,
    pub finalizer: Option<unsafe extern "C" fn(*mut c_void)>,
    pub eof: bool,
    pub is_statement: bool,
}

/// struct Lisp_Module_Function is defined in emacs-module.c; only pointers to
/// it appear here.
#[repr(C)]
pub struct Lisp_Module_Function {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn make_unibyte_string (s: *const c_char, len: usize) -> Lisp_Object;
    pub fn make_string (s: *const c_char, len: usize) -> Lisp_Object;
    pub fn allocate_vector (size: isize) -> *mut Lisp_Vector;
    pub fn intern_1 (s: *const c_char, len: usize) -> Lisp_Object;
    pub fn intern_c_string_1 (s: *const c_char, len: usize) -> Lisp_Object;
    pub fn Fsignal (error_symbol: Lisp_Object, data: Lisp_Object) -> !;
    pub fn xsignal0 (error_symbol: Lisp_Object) -> !;
    pub fn fast_string_match_internal (regexp: Lisp_Object, string: Lisp_Object,
                                       table: Lisp_Object) -> isize;
    pub fn fast_c_string_match_internal (regexp: Lisp_Object, string: *const c_char,
                                         len: usize, table: Lisp_Object) -> isize;
    pub fn check_obarray_slow (obarray: Lisp_Object) -> Lisp_Object;
    pub fn grow_specpdl_allocation ();
    pub fn unbind_to (count: specpdl_ref, value: Lisp_Object) -> Lisp_Object;
    pub fn maybe_garbage_collect ();
    pub fn probably_quit () -> !;
    pub fn xfree (p: *mut c_void);
    pub fn Fcons (car: Lisp_Object, cdr: Lisp_Object) -> Lisp_Object;
    pub fn calln (fn_: Lisp_Object) -> Lisp_Object;
    pub fn egetenv_internal (var: *const c_char, len: usize) -> *mut c_char;
    pub static mut Vquit_flag: Lisp_Object;
    pub static mut pending_signals: bool;
    pub static mut Vascii_canon_table: Lisp_Object;
    pub static mut Vcurrent_load_list: Lisp_Object;
    pub static mut consing_until_gc: EMACS_INT;
    pub static mut chartab_size: [c_int; 4];
}

// --- obarrays (src/lisp.h 2400-2500) ---------------------------------------

#[inline(always)]
pub fn OBARRAYP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_OBARRAY) }

#[inline(always)]
pub fn XOBARRAY (a: Lisp_Object) -> *mut Lisp_Obarray {
    eassert! (OBARRAYP (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Obarray)
}

#[inline(always)]
pub fn CHECK_OBARRAY (x: Lisp_Object) { CHECK_TYPE (OBARRAYP (x), Qobarrayp (), x) }

#[inline(always)]
pub fn make_lisp_obarray (o: *mut Lisp_Obarray) -> Lisp_Object {
    eassert! (PSEUDOVECTOR_TYPEP (&raw mut (*o).header, pvec_type::PVEC_OBARRAY));
    make_lisp_ptr (o as *mut c_void, Lisp_Type::Lisp_Vectorlike)
}

#[inline(always)]
pub fn obarray_size (o: *mut Lisp_Obarray) -> isize {
    1isize << unsafe { (*o).size_bits }
}

#[inline(always)]
pub fn check_obarray (obarray: Lisp_Object) -> Lisp_Object {
    if OBARRAYP (obarray) { obarray } else { unsafe { check_obarray_slow (obarray) } }
}

#[inline(always)]
pub fn make_obarray_iter (oa: *mut Lisp_Obarray) -> obarray_iter_t {
    obarray_iter_t { o: oa, idx: -1, symbol: std::ptr::null_mut () }
}

#[inline(always)]
pub fn obarray_iter_at_end (it: *mut obarray_iter_t) -> bool {
    if ! unsafe { (*it).symbol }.is_null () {
        return false;
    }
    let size = obarray_size (unsafe { (*it).o });
    loop {
        unsafe { (*it).idx += 1 };
        if unsafe { (*it).idx } >= size {
            return true;
        }
        let obj = unsafe { *(*((*it).o)).buckets.add ((*it).idx as usize) };
        if !NILP (obj) {
            unsafe { (*it).symbol = XSYMBOL (obj) };
            return false;
        }
    }
}

#[inline(always)]
pub fn obarray_iter_step (it: *mut obarray_iter_t) {
    unsafe { (*it).symbol = (*(*it).symbol).u.s.next };
}

#[inline(always)]
pub fn obarray_iter_symbol (it: *mut obarray_iter_t) -> Lisp_Object {
    make_lisp_symbol (unsafe { (*it).symbol })
}

// --- hash tables (src/lisp.h 2510-2700) ------------------------------------

#[inline(always)]
pub fn HASH_TABLE_P (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_HASH_TABLE) }

#[inline(always)]
pub fn XHASH_TABLE (a: Lisp_Object) -> *mut Lisp_Hash_Table {
    eassert! (HASH_TABLE_P (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Hash_Table)
}

#[inline(always)]
pub fn make_lisp_hash_table (h: *mut Lisp_Hash_Table) -> Lisp_Object {
    eassert! (PSEUDOVECTOR_TYPEP (&raw mut (*h).header, pvec_type::PVEC_HASH_TABLE));
    make_lisp_ptr (h as *mut c_void, Lisp_Type::Lisp_Vectorlike)
}

#[inline(always)]
pub fn HASH_KEY (h: *mut Lisp_Hash_Table, idx: hash_idx_t) -> Lisp_Object {
    eassert! (idx >= 0 && idx < unsafe { (*h).table_size });
    unsafe { *(*h).key_and_value.add (2 * idx as usize) }
}

#[inline(always)]
pub fn HASH_VALUE (h: *mut Lisp_Hash_Table, idx: hash_idx_t) -> Lisp_Object {
    eassert! (idx >= 0 && idx < unsafe { (*h).table_size });
    unsafe { *(*h).key_and_value.add (2 * idx as usize + 1) }
}

#[inline(always)]
pub fn HASH_HASH (h: *mut Lisp_Hash_Table, idx: hash_idx_t) -> hash_hash_t {
    eassert! (idx >= 0 && idx < unsafe { (*h).table_size });
    unsafe { *(*h).hash.add (idx as usize) }
}

#[inline(always)]
pub fn HASH_TABLE_SIZE (h: *mut Lisp_Hash_Table) -> hash_idx_t { unsafe { (*h).table_size } }

#[inline(always)]
pub fn hash_table_index_size (h: *mut Lisp_Hash_Table) -> isize {
    1isize << unsafe { (*h).index_bits }
}

#[inline(always)]
pub fn hash_from_key (h: *mut Lisp_Hash_Table, key: Lisp_Object) -> hash_hash_t {
    unsafe { ((*(*h).test).hashfn.unwrap_unchecked ()) (key, h) }
}

#[inline(always)]
pub fn hash_unused_entry_key_p (key: Lisp_Object) -> bool {
    BASE_EQ (key, HASH_UNUSED_ENTRY_KEY ())
}

/// src/lisp.h 2654: #define HASH_UNUSED_ENTRY_KEY INVALID_LISP_VALUE.
#[inline(always)]
pub fn HASH_UNUSED_ENTRY_KEY () -> Lisp_Object { INVALID_LISP_VALUE () }

#[inline(always)]
pub fn INVALID_LISP_VALUE () -> Lisp_Object {
    // The C constant is a tag value that no real object can have.
    TAG_PTR (Lisp_Type::Lisp_Int1, std::ptr::null_mut ())
}

#[inline(always)]
pub fn set_hash_key_slot (h: *mut Lisp_Hash_Table, idx: hash_idx_t, val: Lisp_Object) {
    eassert! (idx >= 0 && idx < unsafe { (*h).table_size });
    unsafe { *(*h).key_and_value.add (2 * idx as usize) = val }
}

#[inline(always)]
pub fn set_hash_value_slot (h: *mut Lisp_Hash_Table, idx: hash_idx_t, val: Lisp_Object) {
    eassert! (idx >= 0 && idx < unsafe { (*h).table_size });
    unsafe { *(*h).key_and_value.add (2 * idx as usize + 1) = val }
}

#[inline(always)]
pub fn sxhash_combine (x: EMACS_UINT, y: EMACS_UINT) -> EMACS_UINT {
    (x << 4).wrapping_add (x >> (EMACS_INT_WIDTH - 4)).wrapping_add (y)
}

#[inline(always)]
pub fn SXHASH_REDUCE (x: EMACS_INT) -> EMACS_INT {
    (x ^ x >> (EMACS_INT_WIDTH - FIXNUM_BITS)) & INTMASK
}

#[inline(always)]
pub fn reduce_emacs_uint_to_hash_hash (x: EMACS_UINT) -> hash_hash_t {
    // sizeof x <= 2 * sizeof (hash_hash_t): the C folds the upper half in.
    (x ^ (x >> (8 * (std::mem::size_of::<EMACS_UINT>() - std::mem::size_of::<hash_hash_t>()))))
        as hash_hash_t
}

#[inline(always)]
pub fn knuth_hash (hash: hash_hash_t) -> c_uint {
    let mut h = hash;
    let alpha: c_uint = 2_654_435_769; // 2**32/phi
    h = h.wrapping_mul (alpha);
    h
}

pub const FIXNUM_BITS: usize = INTTYPEBITS + 1;

// --- sqlite objects (src/lisp.h 3400) --------------------------------------

#[inline(always)]
pub fn SQLITEP (x: Lisp_Object) -> bool { PSEUDOVECTORP (x, pvec_type::PVEC_SQLITE) }

#[inline(always)]
pub fn SQLITE (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_SQLITE) }

#[inline(always)]
pub fn CHECK_SQLITE (x: Lisp_Object) { CHECK_TYPE (SQLITE (x), Qsqlitep (), x) }

#[inline(always)]
pub fn XSQLITE (a: Lisp_Object) -> *mut Lisp_Sqlite {
    eassert! (SQLITEP (a));
    XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Sqlite)
}

// --- argument checks -------------------------------------------------------

#[inline(always)]
pub fn CHECK_LIST (x: Lisp_Object) {
    CHECK_TYPE (CONSP (x) || NILP (x), Qlistp (), x)
}

#[inline(always)]
pub fn CHECK_LIST_END (x: Lisp_Object, y: Lisp_Object) {
    CHECK_TYPE (NILP (x), Qlistp (), y)
}

#[inline(always)]
pub fn CHECK_STRING_CAR (x: Lisp_Object) {
    CHECK_TYPE (STRINGP (XCAR (x)), Qstringp (), XCAR (x))
}

#[inline(always)]
pub fn CHECK_VECTOR_OR_STRING (x: Lisp_Object) -> isize {
    if VECTORP (x) {
        return ASIZE (x);
    }
    if STRINGP (x) {
        return SCHARS (x);
    }
    unsafe { wrong_type_argument (Qarrayp (), x) }
}

#[inline(always)]
pub fn CHECK_ARRAY (x: Lisp_Object, predicate: Lisp_Object) {
    CHECK_TYPE (ARRAYP (x), predicate, x)
}

#[inline(always)]
pub fn CHECK_SUBR (x: Lisp_Object) { CHECK_TYPE (SUBRP (x), Qsubrp (), x) }

#[inline(always)]
pub fn call0 (fn_: Lisp_Object) -> Lisp_Object { unsafe { calln (fn_) } }

// --- specpdl helpers that use the storage -------------------------------

#[inline(always)]
pub fn specpdl_ref_to_ptr (r: specpdl_ref) -> *mut specbinding {
    unsafe { (specpdl as *mut u8).offset (unwrap_specpdl_ref (r)) as *mut specbinding }
}

#[inline(always)]
pub fn SPECPDL_INDEX () -> specpdl_ref {
    unsafe { wrap_specpdl_ref (specpdl_ptr as isize - specpdl as isize) }
}

#[inline(always)]
pub fn grow_specpdl () {
    unsafe {
        specpdl_ptr = specpdl_ptr.add (1);
        if specpdl_ptr == specpdl_end {
            grow_specpdl_allocation ();
        }
    }
}

#[inline(always)]
pub fn record_in_backtrace (function: Lisp_Object, args: *mut Lisp_Object, nargs: isize) {
    let count = SPECPDL_INDEX ();
    eassert! (nargs >= UNEVALLED);
    unsafe {
        (*specpdl_ptr).bt.kind = SPECPDL_BACKTRACE as u8;
        (*specpdl_ptr).bt.debug_on_exit = 0;
        (*specpdl_ptr).bt.function = function;
        (*specpdl_ptr).bt.args = args;
        (*specpdl_ptr).bt.nargs = nargs;
        grow_specpdl ();
    }
    let _ = count;
}

pub const UNEVALLED: isize = -2;
pub const MANY: isize = -1;

// --- quitting and collection ----------------------------------------------

#[inline(always)]
pub fn maybe_quit () {
    if !NILP (unsafe { Vquit_flag }) || unsafe { pending_signals } {
        unsafe { probably_quit () }
    }
}

#[inline(always)]
pub fn rarely_quit (count: u64) {
    if count == 0 {
        maybe_quit ()
    }
}

#[inline(always)]
pub fn maybe_gc () {
    if unsafe { consing_until_gc } < 0 {
        unsafe { maybe_garbage_collect () }
    }
}

// --- strings and vectors ---------------------------------------------------

#[inline(always)]
pub fn build_unibyte_string (str_: *const c_char) -> Lisp_Object {
    unsafe { make_unibyte_string (str_, strlen (str_)) }
}

#[inline(always)]
pub fn build_string (str_: *const c_char) -> Lisp_Object {
    unsafe { make_string (str_, strlen (str_)) }
}

#[inline(always)]
pub fn make_uninit_vector (size: isize) -> Lisp_Object {
    make_lisp_ptr (unsafe { allocate_vector (size) } as *mut c_void, Lisp_Type::Lisp_Vectorlike)
}

#[inline(always)]
pub fn make_uninit_sub_char_table (depth: c_int) -> Lisp_Object {
    let slots = SUB_CHAR_TABLE_OFFSET + unsafe { chartab_size[depth as usize] } as isize;
    let v = make_uninit_vector (slots);
    unsafe {
        let size = std::ptr::addr_of_mut! ((*XVECTOR (v)).header.size);
        *size |= PSEUDOVECTOR_FLAG
            | ((pvec_type::PVEC_SUB_CHAR_TABLE as isize) << PSEUDOVECTOR_AREA_BITS);
    }
    unsafe {
        (*XSUB_CHAR_TABLE (v)).depth = depth;
        (*XSUB_CHAR_TABLE (v)).min_char = 0;
    }
    v
}

/// src/lisp.h 2245: SUB_CHAR_TABLE_OFFSET = PSEUDOVECSIZE (struct
/// Lisp_Sub_Char_Table, contents) - 1; the three preceding slots are header,
/// depth and min_char.
pub const SUB_CHAR_TABLE_OFFSET: isize = 2;

#[inline(always)]
pub fn LOADHIST_ATTACH (x: Lisp_Object) {
    if unsafe { initialized } {
        unsafe { Vcurrent_load_list = Fcons (x, Vcurrent_load_list) }
    }
}

#[inline(always)]
pub fn intern (str_: *const c_char) -> Lisp_Object {
    unsafe { intern_1 (str_, strlen (str_)) }
}

#[inline(always)]
pub fn intern_c_string (str_: *const c_char) -> Lisp_Object {
    unsafe { intern_c_string_1 (str_, strlen (str_)) }
}

#[inline(always)]
pub fn xsignal (error_symbol: Lisp_Object, data: Lisp_Object) -> ! {
    unsafe { Fsignal (error_symbol, data) }
}

#[inline(always)]
pub fn CHECK_KEYWORD_ARGS (nargs: isize) {
    // A list of keyword/value pairs must not be missing a value.
    if nargs & 1 != 0 {
        unsafe { xsignal0 (Qmalformed_keyword_arg_list ()) }
    }
}

#[inline(always)]
pub fn fast_string_match (regexp: Lisp_Object, string: Lisp_Object) -> isize {
    unsafe { fast_string_match_internal (regexp, string, Qnil ()) }
}

#[inline(always)]
pub fn fast_string_match_ignore_case (regexp: Lisp_Object, string: Lisp_Object) -> isize {
    unsafe { fast_string_match_internal (regexp, string, Vascii_canon_table) }
}

#[inline(always)]
pub fn fast_c_string_match (regexp: Lisp_Object, string: *const c_char, len: usize) -> isize {
    unsafe { fast_c_string_match_internal (regexp, string, len, Qnil ()) }
}

#[inline(always)]
pub fn fast_c_string_match_ignore_case (regexp: Lisp_Object, string: *const c_char,
                                        len: usize) -> isize {
    unsafe { fast_c_string_match_internal (regexp, string, len, Vascii_canon_table) }
}

#[inline(always)]
pub fn maybe_disable_address_randomization (argc: c_int) -> c_int { argc }

#[inline(always)]
pub fn lispstpcpy (dest: *mut c_char, string: Lisp_Object) -> *mut c_char {
    let len = SBYTES (string);
    unsafe {
        std::ptr::copy_nonoverlapping (SDATA (string), dest as *mut u8, len as usize + 1);
        dest.add (len as usize)
    }
}

/// In C this is a no-op when LeakSanitizer support is not compiled in.
#[inline(always)]
pub fn __lsan_ignore_object (_p: *const c_void) {}

#[inline(always)]
pub fn egetenv (var: *const c_char) -> *mut c_char {
    unsafe { egetenv_internal (var, strlen (var)) }
}

#[inline(always)]
pub fn safe_free (sa_count: specpdl_ref) {
    while unsafe { specpdl_ptr } != specpdl_ref_to_ptr (sa_count) {
        unsafe {
            specpdl_ptr = specpdl_ptr.sub (1);
            eassert! ((*specpdl_ptr).kind == SPECPDL_UNWIND_PTR as u8);
            xfree ((*specpdl_ptr).unwind_ptr.arg);
        }
    }
}

#[inline(always)]
pub fn safe_free_unbind_to (count: specpdl_ref, sa_count: specpdl_ref, val: Lisp_Object) -> Lisp_Object {
    eassert! (!specpdl_ref_lt (sa_count, count));
    unsafe { unbind_to (count, val) }
}

// --- native compilation and modules (feature-disabled configurations) ------
// The frozen configuration has neither HAVE_NATIVE_COMP nor HAVE_MODULES, so
// these live behind the same structural conditions the C uses (PORTING.md 9.2).

#[cfg(feature = "native_comp")]
#[inline(always)]
pub fn NATIVE_COMP_FUNCTIONP (a: Lisp_Object) -> bool {
    PSEUDOVECTORP (a, pvec_type::PVEC_NATIVE_COMP_UNIT)
}

#[cfg(feature = "native_comp")]
#[inline(always)]
pub fn NATIVE_COMP_FUNCTION_DYNP (f: Lisp_Object) -> bool {
    SUBRP (f) && unsafe { (*XSUBR (f)).native_comp_u } != 0
}

#[cfg(not(feature = "native_comp"))]
#[inline(always)]
pub fn NATIVE_COMP_FUNCTIONP (_a: Lisp_Object) -> bool { false }

#[cfg(not(feature = "native_comp"))]
#[inline(always)]
pub fn NATIVE_COMP_FUNCTION_DYNP (_f: Lisp_Object) -> bool { false }

#[cfg(feature = "native_comp")]
#[inline(always)]
pub fn SUBR_TYPE (a: Lisp_Object) -> Lisp_Object {
    unsafe { (*XSUBR (a)).type_ }
}

// allocate_native_comp_unit is not mirrored yet: it allocates a
// struct Lisp_Native_Comp_Unit, which lives in comp.h and arrives with comp.rs.
// It stays on the missing list rather than shipping a placeholder (PORTING.md 0.3).

#[cfg(feature = "modules")]
#[inline(always)]
pub fn MODULE_FUNCTIONP (o: Lisp_Object) -> bool {
    PSEUDOVECTORP (o, pvec_type::PVEC_MODULE_FUNCTION)
}

#[cfg(feature = "modules")]
#[inline(always)]
pub fn XMODULE_FUNCTION (o: Lisp_Object) -> *mut Lisp_Module_Function {
    eassert! (MODULE_FUNCTIONP (o));
    XUNTAG! (o, Lisp_Type::Lisp_Vectorlike, Lisp_Module_Function)
}

#[cfg(not(feature = "modules"))]
#[inline(always)]
pub fn MODULE_FUNCTIONP (_o: Lisp_Object) -> bool { false }
