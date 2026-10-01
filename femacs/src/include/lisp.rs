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
