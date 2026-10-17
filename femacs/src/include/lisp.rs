// femacs: port of src/lisp.h @ fd7afced403.
/* C lisp.h 4307 and 4906 declare `mark_fringe_data` (defined in `src/fringe.c`) and
   `module_finalize_function` (defined in `src/emacs-module.c`).  Both live inside `other_units`
   below: at this scope a unit that defines one read `E0428` (measured on `src/fringe.c` and
   `src/emacs-module.c`), and the module's own glob import (`use other_units::*;`, 4413) puts them
   back in scope for every reader while the defining unit's own item shadows the import - the
   shape P-39 prescribes and the module's header comment records.  */
//
// This is the header mirror: declarations only (PORTING.md 0.1).  Definitions of
// functions live in exactly one translation unit each; this file declares types,
// constants and accessors exactly as src/lisp.h does.
//
// The layout of every type here is asserted against the frozen C layout in
// docs/baseline/layout/ by `xtask layout-check`; those numbers come from the C
// compiler, never from this file.

// Header mirrors are textually included (PORTING.md 0.1), so they carry no inner
// attributes and no crate-relative paths; the including unit supplies both.

use std::ffi::{c_char, c_double, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong,
              c_ulonglong, c_ushort, c_void};

// src/lisp.h 26 includes <stdbit.h>, ahead of <float.h> (31) and <inttypes.h> (32);
// the port mirrors the C23 header cut down to the three functions data.c calls.
include!("../../lib/include/stdbit.in.rs");
// src/lisp.h 31 includes <float.h> first and <inttypes.h> (hence <stdint.h>) at 32;
// the port keeps that order (review D's F10 found the two swapped).  gnulib has no
// wrapper for <float.h>, so that mirror is a system mirror.
/* C 30: `#include <string.h>` - lisp.h's own list.  The port's chain carried it only when a
   unit happened to include the mirror itself, so units declared the names in place
   (`src/keyboard.rs` 486-500 is the worked example).  With this edge those in-place copies
   are `E0428` and move back to the mirror (P-88 (5), status 3.524).  */
include!("../../lib/include/string.rs");   // C 30: `#include <string.h>`
include!("../../lib/include/float.in.rs");
// src/lisp.h 32 includes <inttypes.h>, which includes <stdint.h>; the port keeps
// the same chain (lib/inttypes.in.rs will include this file once it exists).
include!("../../lib/include/stdint.in.rs");
// src/lisp.h 34 includes <stdio.h>, which is how C's `FILE` reaches every translation unit
// through this header.  The port used to leave it to each unit, and that was wrong for the
// mirrors: `regex-emacs.h` 199's checking-only `print_compiled_pattern (FILE *dest, …)` is
// declared downstream of lisp.h in C but had no `FILE` in a unit that includes lisp.h and
// regex-emacs.h without stdio - `src/atimer.rs` compiled under the default configuration and
// failed under `--cfg enable_checking`, which is the only mode that activates that declaration
// (docs/status.md 3.70).  `xtask materialize` dedupes by file path, so a unit that also
// includes stdio.rs directly still gets exactly one copy.
include!("../../lib/include/stdio.rs");
// src/lisp.h 44 includes <verify.h> (after <attribute.h> 39, <byteswap.h> 40,
// <intprops.h> 41 and <sys-limits.h> 42, whose mirrors are not in this list yet --
// P-26).  The mirror defines `assume` (verify.h 389), which `eassume!` below expands,
// and `static_assert` (verify.h 309, the C23 keyword PORTING.md 2.5 maps), so lisp.rs
// declares neither: one name, one place (PORTING.md 0.1-3).
include!("../../lib/include/verify.rs");

// --- integers --------------------------------------------------------------
// src/lisp.h: EMACS_INT is one word; the frozen configuration says 64 bits
// (docs/baseline/config/darwin-arm64.json: ALIGNOF_EMACS_INT = 8).
pub type EMACS_INT = isize;
pub type EMACS_UINT = usize;
/// C: `ptrdiff_t`/`size_t` come from <stddef.h> (src/lisp.h 29) and the system
/// <stdint.h>; the frozen 64-bit target maps both to the Rust machine words.
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type intmax_t = i64;
pub type uintmax_t = u64;
// src/lisp.h 90-102 define these as enum members chosen from the integer widths;
// on this target EMACS_INT is long long.
pub const EMACS_INT_WIDTH: usize = LONG_WIDTH as usize;   // lisp.h 96 (the live branch)
pub const EMACS_UINT_WIDTH: usize = ULONG_WIDTH as usize; // lisp.h 96
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

// src/lisp.h 2168 includes "comp.h", which this configuration compiles to its
// no-native-comp branch; the mirror carries the one entry point data.c calls.
/* C 2168's `#include "comp.h"` sits *after* lisp.h's `eassert` (181-189) and `XUNTAG`
   (750); this include moves below those macros for the same reason - comp.rs's accessors
   call both, and `macro_rules!` are textually scoped (measured: with the include above
   them, `xtask dead-arms` could not find `eassert`/`XUNTAG` in `src/alloc.rs`'s flipped
   build).  */

// --- subrs (src/lisp.h 2174) -----------------------------------------------

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub union Lisp_Subr_function {
    pub a0: Option<unsafe extern "C" fn() -> Lisp_Object>,
    pub a1: Option<unsafe extern "C" fn(Lisp_Object) -> Lisp_Object>,
    pub a2: Option<unsafe extern "C" fn(Lisp_Object, Lisp_Object) -> Lisp_Object>,
    pub a3: Option<unsafe extern "C" fn(Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object>,
    pub a4: Option<unsafe extern "C" fn(Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object>,
    pub a5: Option<unsafe extern "C" fn(Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object>,
    pub a6: Option<unsafe extern "C" fn(Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object>,
    pub a7: Option<unsafe extern "C" fn(Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object>,
    pub a8: Option<unsafe extern "C" fn(Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object>,
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
    #[cfg(feature = "have_native_comp")]
    pub native_comp_u: Lisp_Object,
    #[cfg(feature = "have_native_comp")]
    pub native_c_name: *mut c_char,
    #[cfg(feature = "have_native_comp")]
    pub lambda_list: Lisp_Object,
    #[cfg(feature = "have_native_comp")]
    pub r#type: Lisp_Object,
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

// lisp.h 313-314: `bool_bf need_adjustment : 1; bool_bf insertion_type : 1;` - the two
// 1-bit flags C reads as `m->insertion_type`.  The port keeps them in `bits`, LSB first,
// and reads the second through the accessor below; `docs/ports/token-aliases.tsv` carries
// the `insertion_type` family so the C body's token still has to be spelled (P-78).
pub const MARKER_NEED_ADJUSTMENT: u32 = 1 << 0;
pub const MARKER_INSERTION_TYPE: u32 = 1 << 1;

#[inline(always)]
pub fn MARKER_INSERTION_TYPE_P (m: *mut Lisp_Marker) -> bool {
    unsafe { ((*m).bits & MARKER_INSERTION_TYPE) != 0 }
}

// --- support declarations and accessors (batch 1) ---
// Sources: src/lisp.h 180-198 (assertions), 620-650 (gflags), 700-760 (tag
// accessors), 869-880 (tag initialisers), 985-1050 (pseudovector bits).

// --- assertions (src/lisp.h 180-198) ---------------------------------------
// Three forms, exactly as C: with checking on, `suppress_checking` short
// circuits and a failure calls die; with checking off the condition is not even
// evaluated (PORTING.md 2.5).


// lib/verify.h `assume`: a compile-time promise, undefined behaviour if false.  The
// definition is the mirror's, included above at this file's C 44 position.

/// C's `eassert (cond)` is `die (#cond, __FILE__, __LINE__)`: the message is the
/// condition *text* and the file is the unit that expanded it.  `concat!` folds
/// `stringify!`/`file!` into NUL-terminated static strings, so the port reports the
/// same message (C reports the `.c`/`.h` name, the port the mirror's path — the one
/// deviation) and allocates nothing on the failure path.
macro_rules! eassert {
    ($cond:expr) => {
        #[cfg(enable_checking)]
        {
            if !(unsafe { suppress_checking }) && !($cond) {
                unsafe {
                    die (concat! (stringify! ($cond), "\0").as_ptr () as *const c_char,
                         concat! (file! (), "\0").as_ptr () as *const c_char,
                         line!() as c_int)
                }
            }
        }
    };
}

macro_rules! eassume {
    ($cond:expr) => {
        #[cfg(enable_checking)]
        {
            if unsafe { suppress_checking } {
                assume!($cond);
            } else if !($cond) {
                unsafe {
                    die (concat! (stringify! ($cond), "\0").as_ptr () as *const c_char,
                         concat! (file! (), "\0").as_ptr () as *const c_char,
                         line!() as c_int)
                }
            }
        }
        #[cfg(not(enable_checking))]
        {
            assume!($cond);
        }
    };
}

// C23 spells `static_assert (cond)` as a keyword (src/lisp.h 2007); the port keeps the
// same name and evaluates it at compile time.  The definition is the verify.h mirror's
// (`femacs/lib/include/verify.rs`), included above at this file's C 44 position: PORTING.md
// 2.5's macro is reachable in lisp.rs's scope, and PORTING.md 0.1-3 keeps it to one
// definition for the whole tree.

// C defines PSEUDOVECSIZE at lisp.h 2022, *after* the char-table constants (2231-2247) that
// are built from it.  Rust's `macro_rules!` is textually scoped (P-57) and those constants
// are `const` initialisers that the macro has to be expanded in, so the port moves the
// definition here, next to `static_assert!`; the body is C's, unchanged.
/// C: offsetof (type, lastlispfield) + word_size < header_size ? 0 : ...
#[macro_export]
macro_rules! PSEUDOVECSIZE {
    ($type:ty, $field:tt) => {{
        let off = std::mem::offset_of!($type, $field) as isize;
        if off + word_size < header_size { 0 } else { (off + word_size - header_size) / word_size }
    }};
}

#[inline(always)]
pub fn PVECHEADERSIZE (code: pvec_type, lispsize: isize, restsize: isize) -> isize {
    PSEUDOVECTOR_FLAG | ((code as isize) << PSEUDOVECTOR_AREA_BITS)
        | (restsize << PSEUDOVECTOR_SIZE_BITS) | lispsize
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
include!("comp.rs");        // C 2168: `#include "comp.h"`, after `eassert`/`XUNTAG`

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
    XBARE_SYMBOL (a)
}

// --- globals (src/lisp.h 912 includes the generated globals.h) --------------
// In the reference build this file comes from make-docfile; xtask gen-globals
// produces the port's version (Q symbols and the built-in symbol array).
include!("globals.rs");
// C sees every EXFUN through lisp.h's `#include <globals.h>`; the mirror keeps the
// generated prototypes in `globals.rs`'s `pub mod subrs`, and `xtask gen-globals` ends
// that file with `use subrs::*;`, which `include!` above puts in this scope -- the same
// visibility by Rust's own mechanism (PORTING.md 0.1-14).  data.c is the first unit that
// depends on it (review B's M3: the line must not be repeated here).

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
pub const BITS_PER_BITS_WORD: usize = SIZE_WIDTH as usize;   // lisp.h 137, the live branch
pub const BITS_WORD_MAX: usize = usize::MAX;
pub const BOOL_VECTOR_BITS_PER_CHAR: usize = 8;
// Types the reference gets from <setjmp.h> and keyboard.h; the sizes come from
// the frozen C layout baseline (docs/baseline/layout/c-layout-darwin-arm64.tsv:
// sizeof jmp_buf 192, sizeof sigjmp_buf 196).
pub type jmp_buf = [u8; 192];
pub type sigjmp_buf = [u8; 196];

// lisp.h 3570's `typedef struct kboard KBOARD;` is not declared here: the tag's definition
// lives in keyboard.h, and a mirror may only name a type it declares itself or reaches through
// one of its own `include!`s (PORTING.md P-72).  `keyboard.rs` declares both the tag and the
// typedef; a unit that needs `KBOARD` includes that mirror (as C's code needs keyboard.h).
// The opaque `kboard` stand-in this line replaced was deleted with its real definition (P-70).

pub type intptr_t = isize;

/// src/lisp.h 1999: iQnil == 0 && Lisp_Symbol == 0.
/// src/lisp.h 4140: `typedef intmax_t modiff_count;`
pub type modiff_count = intmax_t;

/// src/lisp.h 1423: `typedef struct interval *INTERVAL;`.  `struct interval` is
/// incomplete in lisp.h (its definition lives in intervals.h, which this header
/// does not include), so the port keeps an opaque pointer here; naming the real
/// type would force every translation unit to include intervals.rs
/// (PORTING.md 0.1-2: C's incomplete `struct X *` becomes `*mut c_void` and the
/// deviation is recorded).
pub type INTERVAL = *mut c_void;

/// src/lisp.h 1999: `enum { NIL_IS_ZERO = iQnil == 0 && Lisp_Symbol == 0 };`
/// The port derives it from the generated symbol table, so an assertion that
/// relies on it stays meaningful.
pub const NIL_IS_ZERO: bool = iQnil == 0 && (Lisp_Type::Lisp_Symbol as isize) == 0;
pub const CHARTAB_SIZE_BITS_0: usize = 6;
pub const CHARTAB_SIZE_BITS_1: usize = 4;
pub const CHARTAB_SIZE_BITS_2: usize = 5;
pub const CHARTAB_SIZE_BITS_3: usize = 7;

// Symbol redirect/trapped-write/interned states (src/lisp.h 763-784: `enum
// symbol_interned`, `enum symbol_redirect`, `enum symbol_trapped_write`).
pub const SYMBOL_PLAINVAL: u32 = 0;
pub const SYMBOL_VARALIAS: u32 = 1;
pub const SYMBOL_LOCALIZED: u32 = 2;
pub const SYMBOL_FORWARDED: u32 = 3;
pub const SYMBOL_UNTRAPPED_WRITE: u32 = 0;
pub const SYMBOL_NOWRITE: u32 = 1;
pub const SYMBOL_TRAPPED_WRITE: u32 = 2;   // lisp.h 777-783 (`enum symbol_trapped_write`)
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
// lisp.h 1784: `PSEUDOVECTOR_TYPE (const struct Lisp_Vector *v)` - the parameter is the
// vector itself (whose first member is the header), which is what XVECTOR hands back.
pub fn PSEUDOVECTOR_TYPE (v: *mut Lisp_Vector) -> pvec_type {
    let size = unsafe { (*v).header.size };
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

// --- flexible array members (PORTING.md 1.3) --------------------------------
//
// C declares `Lisp_Vector.contents`, `Lisp_Char_Table.contents`/`.extras`,
// `Lisp_Sub_Char_Table.contents` and `Lisp_Bool_Vector.data` as flexible array members.
// The port keeps them as `[T; 0]` so `sizeof` and the following fields' offsets match C,
// but a Rust array of length 0 cannot be indexed: `arr[i]` is a bounds check that
// panics ("the len is 0 but the index is 0") in both debug and release.  The allocation
// really does hold the elements, so the port reaches them by address; `FAM_ELEM!` is a
// *place*, so C's `x->contents[i]`, `&x->contents[i]` and `x->contents[i] = v` all port
// to it unchanged in shape.
/// The base address of a flexible array member, without retagging: `as_mut_ptr ()` on a
/// `[T; 0]` field first takes a `&mut [T; 0]`, a zero-size retag whose tag exists nowhere,
/// so every `.add (i)` derived from it is undefined behaviour even for `i == 0` (Miri:
/// "attempting a write access using <544> … that tag does not exist in the borrow stack").
/// `addr_of_mut!` keeps the allocation's provenance, the idiom `bool_vector_data` uses.
/// `N` is generic because two callers reach a *sized* array (`Lisp_Sub_Char_Table.contents`,
/// `Lisp_Char_Table.contents`), not only a FAM.
#[inline(always)]
pub fn fam_base<T, const N: usize> (p: *mut [T; N]) -> *mut T { p as *mut T }

#[macro_export]
macro_rules! FAM_ELEM {
    ($fam:expr, $idx:expr) => { *fam_base (core::ptr::addr_of_mut! ($fam)).add ($idx as usize) };
}

/// `FAM_ELEM!`'s read-only twin: a `*const` base (`pdumper.rs`'s three reads of a dumped vector)
/// cannot go through `addr_of_mut!`, and C reads `x->contents[i]` through either qualifier.
#[inline(always)]
pub fn fam_base_ro<T, const N: usize> (p: *const [T; N]) -> *const T { p as *const T }

#[macro_export]
macro_rules! FAM_ELEM_RO {
    ($fam:expr, $idx:expr) => { *fam_base_ro (core::ptr::addr_of! ($fam)).add ($idx as usize) };
}

#[inline(always)]
pub fn AREF (array: Lisp_Object, idx: isize) -> Lisp_Object {
    eassert! (0 <= idx && idx < gc_asize (array));
    unsafe { FAM_ELEM! ((*XVECTOR (array)).contents, idx) }
}

#[inline(always)]
pub fn aref_addr (array: Lisp_Object, idx: isize) -> *mut Lisp_Object {
    eassert! (0 <= idx && idx <= gc_asize (array));
    unsafe { &raw mut FAM_ELEM! ((*XVECTOR (array)).contents, idx) }
}

#[inline(always)]
pub fn ASET (array: Lisp_Object, idx: isize, val: Lisp_Object) {
    eassert! (0 <= idx && idx < ASIZE (array));
    unsafe { FAM_ELEM! ((*XVECTOR (array)).contents, idx) = val }
}

#[inline(always)]
pub fn gc_aset (array: Lisp_Object, idx: isize, val: Lisp_Object) {
    eassert! (0 <= idx && idx < gc_asize (array));
    unsafe { FAM_ELEM! ((*XVECTOR (array)).contents, idx) = val }
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
                FAM_ELEM! ((*XSUB_CHAR_TABLE (ascii)).contents, idx)
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
pub const CHAR_TABLE_STANDARD_SLOTS: isize =
    PSEUDOVECSIZE! (Lisp_Char_Table, contents) - 1 + (1 << CHARTAB_SIZE_BITS_0);  // lisp.h 2231-2240

// lisp.h 2252-2258: the three sanity checks that bind the constants above to the layout.
// They were missing from this mirror, which is why `SUB_CHAR_TABLE_OFFSET` could sit at 2
// while C's is 1 (P-59/P-60): the C compiler proved it, the port did not.  C's `offsetof
// (struct X, f)` is `std::mem::offset_of! (X, f)` and `sizeof (Lisp_Object)` is
// `std::mem::size_of::<Lisp_Object> ()`, exactly as lisp.h 1831-1835 spells them.
static_assert! (std::mem::offset_of! (Lisp_Char_Table, defalt) as isize == header_size);
static_assert! (std::mem::offset_of! (Lisp_Char_Table, extras) as isize
                == header_size + CHAR_TABLE_STANDARD_SLOTS
                   * std::mem::size_of::<Lisp_Object> () as isize);
static_assert! (std::mem::offset_of! (Lisp_Sub_Char_Table, contents) as isize
                == header_size + SUB_CHAR_TABLE_OFFSET
                   * std::mem::size_of::<Lisp_Object> () as isize);

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

// src/lisp.h 2289 includes "thread.h" unconditionally; the mirror carries the three
// predicates data.c's type predicates use (docs/ports/include-thread.md).
include!("thread.rs");

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
    pub r#type: u32,               // ENUM_BF (Lisp_Fwd_Type) type : 8 in C's first word
    pub u: Lisp_Fwd_u,
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
pub fn XFWDTYPE (a: *mut Lisp_Fwd) -> u32 { unsafe { (*a).r#type } }

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
pub fn make_int (n: intmax_t) -> Lisp_Object {
    if FIXNUM_OVERFLOW_P (n as EMACS_INT) { unsafe { make_bigint (n) } } else { make_fixnum (n as EMACS_INT) }
}

#[inline(always)]
pub fn make_uint (n: uintmax_t) -> Lisp_Object {
    // C's `FIXNUM_OVERFLOW_P (n)` with an unsigned N compares in the common type: the
    // macro's lower-bound half is vacuous and only the upper bound decides.  The signed
    // helper is asked too, so the token stays and a value above i64::MAX (which truncates)
    // is caught by the unsigned bound (review A's A-4).
    if FIXNUM_OVERFLOW_P (n as EMACS_INT) || (MOST_POSITIVE_FIXNUM as uintmax_t) < n
      { unsafe { make_biguint (n) } }
    else { make_fixnum (n as EMACS_INT) }
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
/* C 3558-3562: the rest of the enumeration is target-dependent, because C 3560-3562's two
   `#ifdef HAVE_MODULES` members sit *between* `SPECPDL_NOP` and `SPECPDL_LET` and C's
   enumerators are numbered in order.  Android sets `HAVE_MODULES` (1), the frozen target does
   not (`/* #undef HAVE_MODULES */`), so the six values differ by two between the two arms -
   transcribed with the frozen target's numbers only, `src/eval.c` 3840-3847's module arm read
   `cannot find value SPECPDL_MODULE_RUNTIME` on Android (measured).  */
#[cfg (not (feature = "have_modules"))]
pub const SPECPDL_LET: u32 = 9;
#[cfg (not (feature = "have_modules"))]
pub const SPECPDL_LET_LOCAL: u32 = 10;
#[cfg (not (feature = "have_modules"))]
pub const SPECPDL_LET_DEFAULT: u32 = 11;
#[cfg (feature = "have_modules")]
pub const SPECPDL_MODULE_RUNTIME: u32 = 9;
#[cfg (feature = "have_modules")]
pub const SPECPDL_MODULE_ENVIRONMENT: u32 = 10;
#[cfg (feature = "have_modules")]
pub const SPECPDL_LET: u32 = 11;
#[cfg (feature = "have_modules")]
pub const SPECPDL_LET_LOCAL: u32 = 12;
#[cfg (feature = "have_modules")]
pub const SPECPDL_LET_DEFAULT: u32 = 13;

// lisp.h 4198-4205: `enum Set_Internal_Bind` (the fourth argument of `set_internal`).
pub type Set_Internal_Bind = c_int;
pub const SET_INTERNAL_SET: Set_Internal_Bind = 0;
pub const SET_INTERNAL_BIND: Set_Internal_Bind = 1;
pub const SET_INTERNAL_UNBIND: Set_Internal_Bind = 2;
pub const SET_INTERNAL_THREAD_SWITCH: Set_Internal_Bind = 3;

// lisp.h 3803-3826: `enum handlertype` (the kind of a `struct handler`).
pub type handlertype = c_int;
pub const CATCHER: handlertype = 0;
pub const CONDITION_CASE: handlertype = 1;
pub const CATCHER_ALL: handlertype = 2;
pub const CATCHER_ALL_DEBUGGABLE: handlertype = 3;
pub const HANDLER_BIND: handlertype = 4;
pub const SKIP_CONDITIONS: handlertype = 5;

// lisp.h 3833-3837: `enum nonlocal_exit`.
pub type nonlocal_exit = c_int;
pub const NONLOCAL_EXIT_SIGNAL: nonlocal_exit = 0;
pub const NONLOCAL_EXIT_THROW: nonlocal_exit = 1;

// lisp.h 4496-4502: `enum gc_root_type`.
pub type gc_root_type = c_int;
pub const GC_ROOT_STATICPRO: gc_root_type = 0;
pub const GC_ROOT_BUFFER_LOCAL_DEFAULT: gc_root_type = 1;
pub const GC_ROOT_BUFFER_LOCAL_NAME: gc_root_type = 2;

/* C 4504-4508: the visitor `visit_static_gc_roots` is handed (C 4509).  Its first member is
   `void (*visit) (Lisp_Object const *, enum gc_root_type, void *)`, spelled the way this file
   spells every function pointer (`Option<unsafe extern "C" fn …>`, PORTING.md 1.4); the record was
   missing from this mirror until the missing-type check asked for it (status 3.153) - the port had
   recorded it in a comment in `pdumper.rs` instead.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct gc_root_visitor
{
  pub visit: Option<unsafe extern "C" fn (*const Lisp_Object, gc_root_type, *mut c_void)>,
  pub data: *mut c_void,
}
pub const GC_ROOT_C_SYMBOL: gc_root_type = 3;

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
    pub r#where: specbinding_let_where,
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
    pub r#let: specbinding_let,
    pub bt: specbinding_bt,
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
    pub r#where: Lisp_Object,
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
pub fn list1i (a: EMACS_INT) -> Lisp_Object { unsafe { list1 (make_int (a as intmax_t)) } }

#[inline(always)]
pub fn list2i (a: EMACS_INT, b: EMACS_INT) -> Lisp_Object {
    unsafe { list2 (make_int (a as intmax_t), make_int (b as intmax_t)) }
}

#[inline(always)]
pub fn list3i (a: EMACS_INT, b: EMACS_INT, c: EMACS_INT) -> Lisp_Object {
    unsafe { list3 (make_int (a as intmax_t), make_int (b as intmax_t), make_int (c as intmax_t)) }
}

#[inline(always)]
pub fn list4i (a: EMACS_INT, b: EMACS_INT, c: EMACS_INT, d: EMACS_INT) -> Lisp_Object {
    unsafe { list4 (make_int (a as intmax_t), make_int (b as intmax_t), make_int (c as intmax_t), make_int (d as intmax_t)) }
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
        // lisp.h 4109: `i != 0` is the whole test -- `bignum_to_intmax` answers 0 both for
        // a value out of intmax range and for an impossible zero bignum, and *that* zero is
        // the overflow signal.  The port had added `|| INTEGERP (num)`, which is true for
        // every bignum, so the overflow branch of every caller became unreachable
        // (review A's D1: data.c's `store_symval_forwarding` wrote 0 instead of signalling
        // `overflow-error`, and bignum.rs's `check_integer_range` returned 0 silently).
        i != 0
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
        // lisp.h 4122: same test as integer_to_intmax (review A's D1).
        i != 0
    }
}

// C: lisp.h 4142-4154 returns the value the counter had *before* the increment
// (`return a0;`); the port used to drop it, which is invisible to every gate except by
// reading - `src/buffer.c`'s lane found it and worked around it by reading `MODIFF` first
// (docs/status.md 3.71).
#[inline(always)]
pub fn modiff_incr (a: *mut i64, len: isize) -> i64 {
    let a0 = unsafe { *a };
    // Increase more for a large modification, logarithmically.
    let incr: i64 = if len == 0 { 1 } else { elogb (len as u64) as i64 + 1 };
    let (v, overflow) = a0.overflowing_add (incr);
    eassert! (!overflow && v >> 30 >> 30 == 0);
    unsafe { *a = v };
    a0
}

#[inline(always)]
pub fn modiff_to_integer (a: i64) -> Lisp_Object {
    eassume! (0 <= a && a >> 30 >> 30 == 0);
    make_int (a as intmax_t)
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

// The setjmp used as the register barrier of `flush_stack_barrier` (P-18).  This is the
// port's own alias for `_setjmp`, not a name from lisp.h, so it stays at the crate root.
unsafe extern "C" {
    #[link_name = "_setjmp"]
    fn setjmp_for_barrier (b: *mut c_void) -> c_int;
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

// lisp.h 4009-4013: C writes the bitfield directly here.  The port used to route this
// through an invented `set_symbol_trapped_write` helper; C's function of that name is a
// *static in data.c* (1832-1840) with a `Qtrapping_constant` early exit the helper did
// not have, and having both in one module would be a duplicate definition once data.c
// 1832 lands (review A's F1).
#[inline(always)]
pub fn make_symbol_constant (sym: Lisp_Object) {
    unsafe {
        (*XSYMBOL (sym)).u.s.bits =
            ((*XSYMBOL (sym)).u.s.bits & !SYMBOL_TRAPPED_WRITE_MASK)
                | ((SYMBOL_NOWRITE << SYMBOL_TRAPPED_WRITE_SHIFT) & SYMBOL_TRAPPED_WRITE_MASK)
    }
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
    unsafe { FAM_ELEM! ((*XCHAR_TABLE (table)).extras, idx) = val }
}

// C defines this INLINE in lisp.h 4078; the mirror had declared it as an external symbol
// with no definition anywhere in the port (review A's F3 — the one such case among the
// 403 INLINE definitions of the four mirrored headers).
#[inline(always)]
pub fn set_sub_char_table_contents (table: Lisp_Object, idx: isize, val: Lisp_Object) {
    unsafe { FAM_ELEM! ((*XSUB_CHAR_TABLE (table)).contents, idx) = val }
}

#[inline(always)]
pub fn set_char_table_contents (table: Lisp_Object, idx: isize, val: Lisp_Object) {
    eassert! (0 <= idx && idx < (1 << CHARTAB_SIZE_BITS_0));
    unsafe { FAM_ELEM! ((*XCHAR_TABLE (table)).contents, idx) = val }
}

// --- batch 6: obarrays, hash tables, sqlite, checks, constructors (src/lisp.h 2400-3400) ---
pub type hash_hash_t = c_uint;
// lisp.h 2520-2525: `typedef enum hash_table_std_test_t { Test_eql, Test_eq, Test_equal }`.
pub type hash_table_std_test_t = c_int;
pub const Test_eql: hash_table_std_test_t = 0;
pub const Test_eq: hash_table_std_test_t = 1;
pub const Test_equal: hash_table_std_test_t = 2;

// lisp.h 2544-2552: `typedef enum hash_table_weakness_t`.
pub type hash_table_weakness_t = c_int;
pub const Weak_None: hash_table_weakness_t = 0;
pub const Weak_Key: hash_table_weakness_t = 1;
pub const Weak_Value: hash_table_weakness_t = 2;
pub const Weak_Key_Or_Value: hash_table_weakness_t = 3;
pub const Weak_Key_And_Value: hash_table_weakness_t = 4;
/// lisp.h 2559: `typedef int_least32_t hash_idx_t;` — a 32-bit index, *not* a
/// `ptrdiff_t` (the C comment above it says "signed and a subtype of ptrdiff_t", which the
/// port's `isize` read it as; review A's F1 measured `sizeof (struct Lisp_Hash_Table)` at 72
/// in C against 88 in the port, 2026-10-02).  `PRIdHASH_IDX` (lisp.h 2560) is `PRIdLEAST32`,
/// i.e. the `"d"` the port already uses.
pub type hash_idx_t = i32;

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

// C's bitfields of `struct Lisp_Hash_Table` (lisp.h 2620-2630) have no field names of their
// own; these are the port's names for the bits inside `bits`, spelled like `itree.rs`'s
// (PORTING.md 1.3).
pub const HASH_WEAKNESS_MASK: u8 = 0b111;        // weakness : 3, bits 0-2
pub const HASH_FROZEN_TEST_MASK: u8 = 0b11 << 3; // frozen_test : 2, bits 3-4
pub const HASH_MUTABLE_MASK: u8 = 1 << 5;        // mutable : 1, bit 5

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
    /// C's three bitfields (`weakness : 3`, `frozen_test : 2`, `mutable : 1`) live in the
    /// byte right after `index_bits`, which the frozen target's clang packs into the *same*
    /// 4-byte unit (bytes 60-63): weakness at bits 0-2, frozen_test at 3-4, mutable at 5,
    /// bytes 62-63 become padding.  Measured, not assumed:
    ///
    ///     t.index_bits = 0xAA; t.weakness = 5; t.frozen_test = 2; t.mutable = 1;
    ///     bytes 56..71: 00 00 00 00 aa 35 00 00 …   -> sizeof 72, index_bits 60,
    ///                                                  next_weak 64
    ///
    /// The port had `_pad: [u8; 3]` plus a `u32` here, which made the struct 80 bytes (and 88
    /// before `hash_idx_t` was corrected) with `next_weak` at 72 — reviewer A's F1 and this
    /// fix's probe.  PORTING.md 1.3 wants the storage unit recorded; the offsets around it and
    /// `sizeof` are asserted, the bit numbering itself is recorded (P-35).
    pub bits: u8,
    /// C's tail padding, the rest of that 4-byte unit (bytes 62-63).
    pub _pad: [u8; 2],
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
    eassert! (unsafe { PSEUDOVECTOR_TYPEP (&raw mut (*o).header, pvec_type::PVEC_OBARRAY) });
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
        /* C lisp.h 2477: `!BASE_EQ (obj, make_fixnum (0))` - an empty bucket holds the
           *fixnum* zero (C 5038/5231/5273 write it), which is not `Qnil`: reading it as nil
           is what sent the fixnum into `XSYMBOL` on the first run (status 3.614).  */
        if !BASE_EQ (obj, make_fixnum (0)) {
            unsafe { (*it).symbol = XBARE_SYMBOL (obj) };
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
    eassert! (unsafe { PSEUDOVECTOR_TYPEP (&raw mut (*h).header, pvec_type::PVEC_HASH_TABLE) });
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
// lisp.h 4254-4258: the hash-table primitives `coding.h`'s CODING_SYSTEM_ID needs (the rest
// arrive with the units that define them), plus lisp.h 4279's `string_make_unibyte`, which
// keyboard.h's ENCODE_MENU_STRING calls.
//
// They sit in a whole-import module because `src/fns.c` - the unit that *defines* all three -
// could not put its definitions at its crate root while these stood there: a same-named item at
// this scope is an `E0428`, so the unit wrapped them in a module of its own instead.  With the
// declarations here the definitions shadow the glob and are back where C has them (PORTING.md
// 0.1-10/P-21; the `buffer_point_*`/`intervals_decls` shape, status 3.87).
pub mod lisp_hash_decls {
  use super::*;

  unsafe extern "C" { pub fn hash_find (h: *mut Lisp_Hash_Table, key: Lisp_Object) -> ptrdiff_t; }
  unsafe extern "C" { pub fn hash_put (h: *mut Lisp_Hash_Table, key: Lisp_Object,
                                        value: Lisp_Object, hash: hash_hash_t) -> ptrdiff_t; }
  unsafe extern "C" { pub fn string_make_unibyte (string: Lisp_Object) -> Lisp_Object; }
}
use lisp_hash_decls::*;

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
    // C 2651: `make_lisp_ptr (NULL, Lisp_Float)` - Lisp_Float is 7 in *both* tag schemes
    // (lisp.h 522), and 7 is not the encoding of any Lisp value: fixnums are `(v << 2) + 2`
    // or `(v << 2) + 6` (lisp.h 506-511), so 7 is free.  The port had `Lisp_Int1`, i.e. 6,
    // which is exactly the fixnum 1: measured 2026-10-16, `hash_unused_entry_key_p (1)`
    // was therefore true, so every slot scan skipped the entry whose key was 1 -
    // `(maphash ...)`/`#s(hash-table ...)` print the second entry on, read one past the used
    // region into stale memory, and the byte compiler's `maphash` over a jump table lost its
    // first case (which is what the `bytecomp.elc` difference and the `print_object` crash
    // both came from).
    TAG_PTR (Lisp_Type::Lisp_Float, std::ptr::null_mut ())
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

/// src/lisp.h 2795-2808: `knuth_hash (hash_hash_t hash, unsigned bits)` - "reduce HASH to a
/// value BITS wide".  json.c's `symset_hash` is the first user, so the mirror now carries C's
/// signature and body; the earlier one-argument spelling here was C 2801-2805 alone.
#[inline(always)]
pub fn knuth_hash (hash: hash_hash_t, bits: c_uint) -> ptrdiff_t {
    /* Knuth multiplicative hashing, tailored for 32-bit indices
       (avoiding a 64-bit multiply on typical platforms).  */
    let h: c_uint = hash;
    let alpha: c_uint = 2_654_435_769;	/* 2**32/phi */
    /* Multiply with unsigned int, ANDing in case UINT_WIDTH exceeds 32.  */
    let product: c_uint = h.wrapping_mul (alpha) & 0xffff_ffff;
    /* Convert to a wider type, so that the shift works when BITS == 0.  */
    let wide_product: u64 = product as u64;
    return (wide_product >> (32 - bits)) as ptrdiff_t;
}

pub const FIXNUM_BITS: usize = VALBITS + 1;   // lisp.h 226: not INTTYPEBITS + 1 (=3)!  See P-59.

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

// --- specpdl helpers that use the storage -------------------------------

#[inline(always)]
pub fn specpdl_ref_to_ptr (r: specpdl_ref) -> *mut specbinding {
    unsafe { ((specpdl! ()) as *mut u8).offset (unwrap_specpdl_ref (r)) as *mut specbinding }
}

#[inline(always)]
pub fn SPECPDL_INDEX () -> specpdl_ref {
    unsafe { wrap_specpdl_ref ((specpdl_ptr! ()) as isize - (specpdl! ()) as isize) }
}

#[inline(always)]
pub fn grow_specpdl () {
    unsafe {
        (specpdl_ptr! ()) = (specpdl_ptr! ()).add (1);
        if (specpdl_ptr! ()) == (specpdl_end! ()) {
            grow_specpdl_allocation ();
        }
    }
}

#[inline(always)]
pub fn record_in_backtrace (function: Lisp_Object, args: *mut Lisp_Object, nargs: isize)
  -> specpdl_ref {
    let count = SPECPDL_INDEX ();
    eassert! (nargs >= UNEVALLED);
    unsafe {
        (*(specpdl_ptr! ())).bt.kind = SPECPDL_BACKTRACE as u8;
        (*(specpdl_ptr! ())).bt.debug_on_exit = 0;
        (*(specpdl_ptr! ())).bt.function = function;
        (*(specpdl_ptr! ())).bt.args = args;
        (*(specpdl_ptr! ())).bt.nargs = nargs;
        grow_specpdl ();
    }
    return count;
}

pub const MANY: isize = -2;
pub const UNEVALLED: isize = -1;

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
    unsafe { make_unibyte_string (str_, strlen (str_) as ptrdiff_t) }
}

#[inline(always)]
pub fn build_string (str_: *const c_char) -> Lisp_Object {
    unsafe { make_string (str_, strlen (str_) as ptrdiff_t) }
}

#[inline(always)]
pub fn make_uninit_vector (size: isize) -> Lisp_Object {
    make_lisp_ptr (unsafe { allocate_vector (size) } as *mut c_void, Lisp_Type::Lisp_Vectorlike)
}

#[inline(always)]
pub fn make_uninit_sub_char_table (depth: c_int, min_char: c_int) -> Lisp_Object {
    let slots = SUB_CHAR_TABLE_OFFSET + unsafe { chartab_size[depth as usize] } as isize;
    let v = make_uninit_vector (slots);
    unsafe {
        let size = std::ptr::addr_of_mut! ((*XVECTOR (v)).header.size);
        *size |= PSEUDOVECTOR_FLAG
            | ((pvec_type::PVEC_SUB_CHAR_TABLE as isize) << PSEUDOVECTOR_AREA_BITS);
    }
    unsafe {
        (*XSUB_CHAR_TABLE (v)).depth = depth;
        (*XSUB_CHAR_TABLE (v)).min_char = min_char;
    }
    v
}

/// src/lisp.h 2245: SUB_CHAR_TABLE_OFFSET = PSEUDOVECSIZE (struct
/// Lisp_Sub_Char_Table, contents) - 1; the three preceding slots are header,
/// depth and min_char.
pub const SUB_CHAR_TABLE_OFFSET: isize =
    PSEUDOVECSIZE! (Lisp_Sub_Char_Table, contents) - 1;  // lisp.h 2245-2247 (was 2; C is 1, P-59)

#[inline(always)]
pub fn LOADHIST_ATTACH (x: Lisp_Object) {
    if unsafe { initialized } {
        unsafe { Vcurrent_load_list = Fcons (x, Vcurrent_load_list) }
    }
}

#[inline(always)]
pub fn intern (str_: *const c_char) -> Lisp_Object {
    unsafe { intern_1 (str_, strlen (str_) as ptrdiff_t) }
}

#[inline(always)]
pub fn intern_c_string (str_: *const c_char) -> Lisp_Object {
    unsafe { intern_c_string_1 (str_, strlen (str_) as ptrdiff_t) }
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
    unsafe { fast_c_string_match_internal (regexp, string, len as ptrdiff_t, Qnil ()) }
}

#[inline(always)]
pub fn fast_c_string_match_ignore_case (regexp: Lisp_Object, string: *const c_char,
                                        len: usize) -> isize {
    unsafe { fast_c_string_match_internal (regexp, string, len as ptrdiff_t, Vascii_canon_table) }
}

/// C: lisp.h 5233-5237: `INLINE struct bc_frame *get_act_rec (struct thread_state *th)
/// { return th->bc.fp; }`.  `struct bc_frame` is a pointer-only type no mirror carries, so
/// the accessor speaks the untyped pointer (P-55), and `th->bc` is thread.h's
/// `bc_thread_state` (`include/thread.rs`, whose `fp` is at 0).  C calls this only from
/// eval.c's own `static inline` copy; the mirror carries it once (P-20).
#[inline(always)]
pub unsafe fn get_act_rec (th: *mut thread_state) -> *mut c_void {
    unsafe { (*th).bc.fp }
}

/// C: lisp.h 5239-5243: `INLINE void set_act_rec (struct thread_state *th, struct bc_frame
/// *act_rec) { th->bc.fp = act_rec; }` - the setter half of the pair above.
#[inline(always)]
pub unsafe fn set_act_rec (th: *mut thread_state, act_rec: *mut c_void) {
    unsafe { (*th).bc.fp = act_rec; }
}

/// C: lisp.h 5276-5280 (the live arm; `HAVE_PERSONALITY_ADDR_NO_RANDOMIZE` is `undef` in the
/// frozen configuration, so `sysdep.c`'s two-argument definition is dead text there):
/// `INLINE int maybe_disable_address_randomization (int argc, char **argv) { return argc; }`.
/// The second parameter is C's own and unused, exactly as in C - the port dropped it once and
/// the call site had to spell one argument (docs/status.md 3.66).
/// C 5272-5274's gate: the inline is the `#else` arm, and Android sets
/// `HAVE_PERSONALITY_ADDR_NO_RANDOMIZE` (1), where `sysdep.c` 162 defines the real one - with both
/// live the unit read `E0428 maybe_disable_address_randomization` (measured).
#[cfg (not (feature = "have_personality_addr_no_randomize"))]
#[inline(always)]
pub fn maybe_disable_address_randomization (argc: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = argv;
    argc
}

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
    unsafe { egetenv_internal (var, strlen (var) as ptrdiff_t) }
}

#[inline(always)]
pub fn safe_free (sa_count: specpdl_ref) {
    while unsafe { (specpdl_ptr! ()) } != specpdl_ref_to_ptr (sa_count) {
        unsafe {
            (specpdl_ptr! ()) = (specpdl_ptr! ()).sub (1);
            eassert! ((*(specpdl_ptr! ())).kind == SPECPDL_UNWIND_PTR as u8);
            xfree ((*(specpdl_ptr! ())).unwind_ptr.arg);
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

// The live pair comes first on purpose: `tools/extract-abi.py` reads the *first*
// definition of a function it finds, and the port wants its body compared with the body C
// compiles for this target (PORTING.md P-51's dead text follows).
#[cfg(not(feature = "have_native_comp"))]
#[inline(always)]
pub fn NATIVE_COMP_FUNCTIONP (a: Lisp_Object) -> bool { let _ = a; false }

#[cfg(not(feature = "have_native_comp"))]
#[inline(always)]
pub fn NATIVE_COMP_FUNCTION_DYNP (a: Lisp_Object) -> bool { let _ = a; false }

// lisp.h 5489-5530: the `#ifdef HAVE_NATIVE_COMP` pair is the *dead* one for this
// configuration, and the `#else` pair (`return false`) is what C compiles.  The port had the
// polarity backwards (the gate found it once the frozen configuration decided the branch).
// The bodies themselves were never through rustc until `dead-arms` learned to flip
// `native_comp` (review A's F6), and it found two spelling errors in them: the pointer was
// compared with `!= 0` instead of C's `!NILP`, and `type_` is `r#type` in the mirror.
#[cfg(feature = "have_native_comp")]
#[inline(always)]
pub fn NATIVE_COMP_FUNCTIONP (a: Lisp_Object) -> bool {
    SUBRP (a) && !NILP (unsafe { (*XSUBR (a)).native_comp_u })
}

#[cfg(feature = "have_native_comp")]
#[inline(always)]
pub fn NATIVE_COMP_FUNCTION_DYNP (a: Lisp_Object) -> bool {
    NATIVE_COMP_FUNCTIONP (a) && !NILP (unsafe { (*XSUBR (a)).lambda_list })
}

const _: () = core::assert! (!cfg! (feature = "have_native_comp") || HAVE_NATIVE_COMP);

#[cfg(feature = "have_native_comp")]
#[inline(always)]
pub fn SUBR_TYPE (a: Lisp_Object) -> Lisp_Object {
    unsafe { (*XSUBR (a)).r#type }
}

// allocate_native_comp_unit is not mirrored yet: it allocates a
// struct Lisp_Native_Comp_Unit, which lives in comp.h and arrives with comp.rs.
// It stays on the missing list rather than shipping a placeholder (PORTING.md 0.3).

// lisp.h 4872-4876 / 4878-4883: both are unconditional in C (the `#ifdef HAVE_MODULES`
// in that region opens *after* them, at lisp.h 4885).  The port had gated the pair on a
// `have_modules` feature nothing ever passed, so the frozen build compiled a `false` stub for
// `MODULE_FUNCTIONP` while C compiled the real `PSEUDOVECTORP`; review B's F2 (found while
// reviewing data.c's `module-function-p`, which calls it).
#[inline(always)]
pub fn MODULE_FUNCTIONP (o: Lisp_Object) -> bool {
    PSEUDOVECTORP (o, pvec_type::PVEC_MODULE_FUNCTION)
}

#[inline(always)]
pub fn XMODULE_FUNCTION (o: Lisp_Object) -> *mut Lisp_Module_Function {
    eassert! (MODULE_FUNCTIONP (o));
    XUNTAG! (o, Lisp_Type::Lisp_Vectorlike, Lisp_Module_Function)
}

// --- extern declarations for reference globals (src/lisp.h) ---
// The reference declares these objects extern in lisp.h; their definitions
// live in the ported units (alloc.c, emacs.c, fns.c, ...).



// --- batch 7: macrology (src/lisp.h 50-1400, 5600-5700) ---
// Everything in this batch is a C macro; each keeps its name.  Where C used a
// macro to spell a function, the mirror defines a function of the same name, so
// call sites read `QUITP()` instead of `QUITP` (PORTING.md P-22).

// --- sizes and alignment ---------------------------------------------------

pub const IDEAL_GCALIGNMENT: usize = 8;
pub const GCALIGNMENT: usize = 8;
pub const MAX_ALLOCA: isize = 16 * 1024;
pub const STRING_BYTES_BOUND: isize =
    if MOST_POSITIVE_FIXNUM < (isize::MAX - 1) { MOST_POSITIVE_FIXNUM } else { isize::MAX - 1 };
/// C: min (MOST_POSITIVE_FIXNUM, (min (PTRDIFF_MAX, SIZE_MAX) - bool_header_size)
/// * BOOL_VECTOR_BITS_PER_CHAR - (BITS_PER_BITS_WORD - 1)); MOST_POSITIVE_FIXNUM
/// is the smaller term on every supported target.
pub const BOOL_VECTOR_LENGTH_MAX: isize = MOST_POSITIVE_FIXNUM;
pub const FLOAT_TO_STRING_BUFSIZE: usize = 350;
pub const WAIT_READING_MAX: i64 = i64::MAX;
pub const CHILD_SETUP_ERROR_DESC: &str = "Doing vfork";   // lisp.h 5208 (`#else` of DOS_NT)
/* lisp.h 3930-3933: `#if defined (USE_X_TOOLKIT) || defined (HAVE_NTGUI) || defined (HAVE_NS) ||
   defined (USE_GTK) || defined (HAVE_HAIKU)` defines `HAVE_EXT_MENU_BAR true`, and lisp.h
   3935-3936 defines `HAVE_EXT_TOOL_BAR true` under `USE_GTK`/`HAVE_NS`.  The frozen
   darwin-arm64 configuration has none of those (`docs/baseline/config/darwin-arm64.macros.tsv`),
   so both names are **undefined** in C and their value is `false` - the constants used to say
   `true`, which no unit reads today (the arms are `#[cfg]`s) but which would take the wrong
   branch the first time one is written as a value (status 3.110).  */
pub const HAVE_EXT_MENU_BAR: bool = false;
pub const HAVE_EXT_TOOL_BAR: bool = false;
// lisp.h 3201 spells the comparison FLT_RADIX, FLT_MANT_DIG, FLT_MIN_EXP, FLT_MAX_EXP;
// the order is kept (review D's F9).
pub const IEEE_FLOATING_POINT: bool =
    FLT_RADIX == 2 && FLT_MANT_DIG == 24 && FLT_MIN_EXP == -125 && FLT_MAX_EXP == 128;
pub const pI: &str = "l";
pub const pD: &str = "t";
pub const PRIdHASH_IDX: &str = "d";           // PRIdLEAST32 on this target
pub const HASH_UNUSED_ENTRY_KEY_IS_INVALID: bool = true;
pub const GCALIGNED_MARKER: () = ();          // C's GCALIGNED* attributes are #[repr(align)]

#[inline(always)]
pub fn POWER_OF_2 (n: isize) -> bool { (n & (n - 1)) == 0 }

#[inline(always)]
pub fn ROUNDUP (x: isize, y: isize) -> isize {
    if POWER_OF_2 (y) { (y - 1 + x) & !(y - 1) } else { (y - 1 + x) - (y - 1 + x) % y }
}

/// lisp.h 1831-1835: `enum { header_size = offsetof (struct Lisp_Vector, contents),
/// bool_header_size = offsetof (struct Lisp_Bool_Vector, data),
/// word_size = sizeof (Lisp_Object) };` — three enum *constants*, not functions.  The port
/// had `word_size`/`header_size` as `const fn`s, which the constant gate could not compare
/// against C's enum members (C's members are only visible to the strengthened extractor
/// since P-38's `enum NAME` + newline + `{` form landed).  Their C type is `int`; the port
/// spells them `isize`/`usize` because every use site is signed arithmetic — value-identical
/// on the frozen target, recorded in PORTING.md P-63.
pub const header_size: isize = std::mem::offset_of!(Lisp_Vector, contents) as isize;
pub const bool_header_size: isize = std::mem::offset_of!(Lisp_Bool_Vector, data) as isize;
pub const word_size: isize = std::mem::size_of::<Lisp_Object>() as isize;


#[inline(always)]
pub const fn VECSIZE (size: usize) -> isize {
    (size as isize - header_size + word_size - 1) / word_size
}

#[macro_export]
macro_rules! ALLOCATE_PSEUDOVECTOR {
    ($type:ty, $field:tt, $tag:expr) => {
        unsafe { allocate_pseudovector (VECSIZE (std::mem::size_of::<$type> ()) as c_int, PSEUDOVECSIZE! ($type, $field) as c_int, PSEUDOVECSIZE! ($type, $field) as c_int, $tag) as *mut $type }
    };
}

#[macro_export]
macro_rules! ALLOCATE_ZEROED_PSEUDOVECTOR {
    ($type:ty, $field:tt, $tag:expr) => {
        unsafe { allocate_pseudovector (VECSIZE (std::mem::size_of::<$type> ()) as c_int, PSEUDOVECSIZE! ($type, $field) as c_int, VECSIZE (std::mem::size_of::<$type> ()) as c_int, $tag) as *mut $type }
    };
}

#[macro_export]
macro_rules! ALLOCATE_PLAIN_PSEUDOVECTOR {
    ($type:ty, $tag:expr) => {
        unsafe { allocate_pseudovector (VECSIZE (std::mem::size_of::<$type> ()) as c_int, 0, VECSIZE (std::mem::size_of::<$type> ()) as c_int, $tag) as *mut $type }
    };
}

// --- stack allocation (PORTING.md P-05) ------------------------------------
// C's AVAIL_ALLOCA relies on the alloca builtin.  The port has no alloca: the
// decision is a stack pool with the same sa_avail accounting, so the allocator
// is a function of the port (alloc.rs) rather than a C name.  record_xmalloc and
// friends keep their C names.


/// C: `#define USE_SAFE_ALLOCA ptrdiff_t sa_avail = MAX_ALLOCA; specpdl_ref
/// sa_count = SPECPDL_INDEX ()` — it *declares* the two names the rest of the block uses.
/// A Rust `macro_rules!` cannot: the `let`s it introduces are hygienic, so the caller
/// never sees `sa_avail`/`sa_count` (the macro was unusable).  The call site therefore
/// names them, the shape `SAFE_NALLOCA!` already takes: `USE_SAFE_ALLOCA! (sa_avail,
/// sa_count, sa_mark);` — the one deviation, and the C name is kept.
///
/// `sa_mark` is the third name, and it is the port's: C's `alloca` memory dies with the frame,
/// which Rust cannot name, so the *scope* carries the pool's height and `SAFE_FREE!` releases
/// back to it (PORTING.md P-05, P-117 ⑧).  The declaration order matters at the call site:
/// the height is read before anything is allocated in the scope.
#[macro_export]
macro_rules! USE_SAFE_ALLOCA {
    ($sa_avail:ident, $sa_count:ident, $sa_mark:ident) => {
        let mut $sa_avail: isize = MAX_ALLOCA;
        let $sa_count: specpdl_ref = SPECPDL_INDEX ();
        let $sa_mark: usize = unsafe { sa_stack_mark () };
    };
}

#[macro_export]
macro_rules! AVAIL_ALLOCA {
    ($sa_avail:ident, $size:expr) => {{
        $sa_avail -= $size as isize;
        unsafe { sa_stack_alloc ($size as usize) }
    }};
}

#[macro_export]
macro_rules! SAFE_ALLOCA {
    ($sa_avail:ident, $size:expr) => {
        if ($size as isize) <= $sa_avail {
            AVAIL_ALLOCA! ($sa_avail, $size)
        } else {
            unsafe { record_xmalloc ($size as usize) }
        }
    };
}

#[macro_export]
macro_rules! SAFE_ALLOCA_LISP {
    ($sa_avail:ident, $buf:ident, $nelt:expr) => {{
        let nelt_ = $nelt as isize;
        // C: `if (ckd_mul (&alloca_nbytes, nelt, word_size) || SIZE_MAX < alloca_nbytes)`;
        // the pair from `overflowing_mul` is ckd_mul's wrapped value and overflow flag
        // (P-15, the ckd_mul family in docs/ports/token-aliases.tsv).
        let (alloca_nbytes, overflow) = nelt_.overflowing_mul (word_size);
        if overflow || SIZE_MAX < alloca_nbytes as usize {
            unsafe { memory_full_up () }
        } else if alloca_nbytes <= $sa_avail {
            $buf = AVAIL_ALLOCA! ($sa_avail, alloca_nbytes) as *mut Lisp_Object;
        } else {
            $buf = unsafe { xzalloc (alloca_nbytes as usize) } as *mut Lisp_Object;
            unsafe { record_unwind_protect_array ($buf, nelt_) };
        }
    }};
}

/// C's `sizeof *(buf)` never evaluates `buf`, so it is well-defined for a pointer
/// that is NULL or not yet assigned (the real `GET_OVERLAYS_AT` call sites pass both).
/// `size_of_val (&*p)` would dereference it, so the port asks for the pointee type
/// instead: passing the pointer by value lets inference name `T` without reading the
/// pointee.  A call site whose pointer C leaves uninitialized must initialize it,
/// because Rust will not pass an uninitialized binding by value.
#[inline(always)]
pub fn pointee_size<T> (p: *mut T) -> usize { let _ = p; std::mem::size_of::<T> () }

#[macro_export]
macro_rules! SAFE_NALLOCA {
    ($sa_avail:ident, $buf:ident, $multiplier:expr, $nitems:expr) => {{
        let mult = $multiplier as isize;
        let nitems_ = $nitems as isize;
        if nitems_ <= $sa_avail / (pointee_size ($buf) as isize) / mult {
            let nbytes = pointee_size ($buf) as isize * mult * nitems_;
            $buf = AVAIL_ALLOCA! ($sa_avail, nbytes) as *mut _;
        } else {
            $buf = unsafe { xnmalloc (nitems_, pointee_size ($buf) as isize * mult) } as *mut _;
            unsafe { record_unwind_protect_ptr (Some (xfree_wrapper), $buf as *mut c_void) };
        }
    }};
}

pub unsafe extern "C" fn xfree_wrapper (p: *mut c_void) { unsafe { xfree (p) } }

#[macro_export]
macro_rules! SAFE_ALLOCA_STRING {
    ($sa_avail:ident, $ptr:ident, $string:expr) => {{
        $ptr = SAFE_ALLOCA! ($sa_avail, SBYTES ($string) + 1) as *mut c_char;
        unsafe { std::ptr::copy_nonoverlapping (SDATA ($string), $ptr as *mut u8, (SBYTES ($string) + 1) as usize) };
    }};
}

/// C: `#define SAFE_FREE() safe_free (sa_count)`.  The port adds the pool's release: this is
/// where the scope's requests die (its `sa_mark`), and it is *not* part of `safe_free`, whose
/// walk only covers what C itself pushed (P-117 ⑦).
#[macro_export]
macro_rules! SAFE_FREE {
    ($sa_count:expr, $sa_mark:expr) => {{
        safe_free ($sa_count);
        unsafe { sa_stack_release_to ($sa_mark) }
    }};
}

#[macro_export]
macro_rules! SAFE_FREE_UNBIND_TO {
    ($sa_count:expr, $count:expr, $val:expr, $sa_mark:expr) => {{
        let val_ = safe_free_unbind_to ($count, $sa_count, $val);
        unsafe { sa_stack_release_to ($sa_mark) };
        val_
    }};
}

// --- Lisp_Object setters (src/lisp.h 1330-1400) -----------------------------

#[inline(always)]
pub fn make_lisp_ptr(ptr: *mut c_void, type_: Lisp_Type) -> Lisp_Object {
    // C's body is `Lisp_Object a = TAG_PTR_INITIALLY (type, ptr);
    // eassert (TAGGEDP (a, type) && XUNTAG (a, type, char) == ptr); return a;` — the
    // port had folded it to the arithmetic and dropped the round-trip assertion.
    // It sits here, after `eassert!`, exactly where lisp.h 1334 has it (before the
    // XSET* setter macros that call it).
    let a = TAG_PTR_INITIALLY (type_, ptr as EMACS_INT);
    eassert! (TAGGEDP (a, type_) && XUNTAG! (a, type_, c_char) as *mut c_void == ptr);
    return a;
}

macro_rules! XSETCONS { ($a:expr, $b:expr) => { $a = make_lisp_ptr ($b as *mut c_void, Lisp_Type::Lisp_Cons) } }
macro_rules! XSETVECTOR { ($a:expr, $b:expr) => { $a = make_lisp_ptr ($b as *mut c_void, Lisp_Type::Lisp_Vectorlike) } }
macro_rules! XSETSTRING { ($a:expr, $b:expr) => { $a = make_lisp_ptr ($b as *mut c_void, Lisp_Type::Lisp_String) } }
macro_rules! XSETSYMBOL { ($a:expr, $b:expr) => { $a = make_lisp_symbol ($b) } }
macro_rules! XSETFLOAT { ($a:expr, $b:expr) => { $a = make_lisp_ptr ($b as *mut c_void, Lisp_Type::Lisp_Float) } }
macro_rules! XSETINT { ($a:expr, $b:expr) => { $a = make_fixnum ($b) } }
macro_rules! XSETFASTINT { ($a:expr, $b:expr) => { $a = make_fixed_natnum ($b) } }
macro_rules! XSETPVECTYPE { ($v:expr, $code:expr) => {
    unsafe { (*$v).header.size |= PSEUDOVECTOR_FLAG | (($code as isize) << PSEUDOVECTOR_AREA_BITS) }
} }
macro_rules! XSETPVECTYPESIZE { ($v:expr, $code:expr, $lispsize:expr, $restsize:expr) => {
    unsafe { (*$v).header.size = PVECHEADERSIZE ($code, $lispsize, $restsize) }
} }
macro_rules! XSETTYPED_PSEUDOVECTOR { ($a:expr, $b:expr, $size:expr, $code:expr) => {{
    XSETVECTOR! ($a, $b);
    eassert! ((($size) & (PSEUDOVECTOR_FLAG | PVEC_TYPE_MASK))
        == (PSEUDOVECTOR_FLAG | (($code as isize) << PSEUDOVECTOR_AREA_BITS)));
}} }
macro_rules! XSETPSEUDOVECTOR { ($a:expr, $b:expr, $code:expr) => {
    XSETTYPED_PSEUDOVECTOR! ($a, $b,
        unsafe { (*XUNTAG! ($a, Lisp_Type::Lisp_Vectorlike, vectorlike_header)).size }, $code)
} }
macro_rules! XSETSUBR { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_SUBR) } }
macro_rules! XSETBOOL_VECTOR { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_BOOL_VECTOR) } }
macro_rules! XSETCHAR_TABLE { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_CHAR_TABLE) } }
macro_rules! XSETSUB_CHAR_TABLE { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_SUB_CHAR_TABLE) } }
macro_rules! XSETBUFFER { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_BUFFER) } }
macro_rules! XSETPROCESS { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_PROCESS) } }
macro_rules! XSETWINDOW { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_WINDOW) } }
macro_rules! XSETWINDOW_CONFIGURATION { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_WINDOW_CONFIGURATION) } }
macro_rules! XSETTERMINAL { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_TERMINAL) } }
macro_rules! XSETTHREAD { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_THREAD) } }
macro_rules! XSETMUTEX { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_MUTEX) } }
macro_rules! XSETCONDVAR { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_CONDVAR) } }
macro_rules! XSETNATIVE_COMP_UNIT { ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_NATIVE_COMP_UNIT) } }

#[inline(always)]
pub fn lisp2size_passthrough (n: isize) -> isize { n }

// --- small utilities -------------------------------------------------------

/// C: lisp.h 71-72: `#define max(a, b) ((a) > (b) ? (a) : (b))` and
/// `#define min(a, b) ((a) < (b) ? (a) : (b))` - type-generic macros, so the port's rows are
/// generic too.  They were the `isize` pair until slice 76 (2026-10-07): C 16580/16589/16592's
/// float uses cannot call that pair (`error[E0308]: expected f32, found isize`, measured), which
/// is the shape the `eabs!` row below records for slice 74.  Every existing call site still
/// infers `isize` (its arguments are already cast, e.g. the `min (... as isize, ... as isize)`
/// rows the scroll-bar and glyph text use), so the widening is a superset - and P-87 1's first
/// leg is what says so (`check-all-units.sh`: 83 units, 0 failures).
#[inline(always)]
pub fn max<T: PartialOrd> (a: T, b: T) -> T { if a > b { a } else { b } }

#[inline(always)]
pub fn min<T: PartialOrd> (a: T, b: T) -> T { if a < b { a } else { b } }

/// eabs (x): C's macro (lisp.h 5597), which works for any signed type and — like C —
/// evaluates its argument twice when it is negative.  The port used to spell it as a
/// `f64` function, which floatfns.c's integer uses cannot call.
///
/// C's body compares against the integer literal `0`, which the compiler converts to `x`'s own
/// type; Rust's `0` is an integer literal and would not unify with an `f32`/`f64` argument
/// (`error[E0308]: expected f32, found integer`, measured on xterm.c 15950's float call -
/// docs/ports/xterm.md slice 74).  `Default::default ()` is the zero of whichever type the
/// argument has, which is C's conversion written where C relies on it; the note in
/// docs/ports/lisp.md records the token deviation.
macro_rules! eabs {
    ($x:expr) => { if $x < Default::default () { -$x } else { $x } };
}

/// lisp.h 5315-5319: the non-Android branch is `# define emacs_fclose fclose`, i.e. an
/// object-like macro aliasing the function.  The port keeps a macro with C's name (P-37), so
/// a call site reads `emacs_fclose! (f)`; a wrapper `fn` would be a *different* function (its
/// address would not be `fclose`'s, PORTING.md 0.1-6), and mirrors declare, they do not
/// define (0.1-2).  Recorded in docs/ports/lisp.md.
macro_rules! emacs_fclose { ($f:expr) => { unsafe { fclose ($f) } }; }


#[inline(always)]
pub fn QUITP () -> bool { !NILP (unsafe { Vquit_flag }) && NILP (unsafe { Vinhibit_quit }) }

#[inline(always)]
pub fn IS_DAEMON () -> bool { (unsafe { daemon_type }) != 0 }

#[inline(always)]
pub fn DAEMON_RUNNING () -> bool { (unsafe { daemon_type }) >= 0 }



#[inline(always)]
pub fn INT_TO_INTEGER (expr: EMACS_INT) -> Lisp_Object { make_int (expr as intmax_t) }

#[inline(always)]
pub fn CONS_TO_INTEGER_signed (cons: Lisp_Object, lo: i64, hi: i64) -> i64 {
    unsafe { cons_to_signed (cons, lo, hi) }
}

#[inline(always)]
pub fn CONS_TO_INTEGER_unsigned (cons: Lisp_Object, hi: u64) -> u64 {
    unsafe { cons_to_unsigned (cons, hi) }
}


#[inline(always)]
pub fn TYPE_RANGED_FIXNUMP (lo: EMACS_INT, x: Lisp_Object, hi: EMACS_INT) -> bool {
    RANGED_FIXNUMP (lo, x, hi)
}

#[inline(always)]
pub fn STRING_SET_UNIBYTE (str_: *mut Lisp_Object) {
    /* C: `XSTRING (STR)` (lisp.h 1624-1630), i.e. the object is *untagged* through XUNTAG; casting
       the tagged word straight to `*mut Lisp_String` leaves the tag bits in the address and Rust's
       alignment check aborts - the first run of the linked program did exactly that
       (`init_obarray_once` -> `define_symbol` -> `make_string` -> `make_clear_string`, status 3.611).  */
    unsafe {
        if (*XSTRING (*str_)).u.s.size == 0 {
            *str_ = empty_unibyte_string;
        } else {
            (*XSTRING (*str_)).u.s.size_byte = -1;
        }
    }
}



/// CALLMANY (f, array): call the subr with the array's element count.
#[macro_export]
macro_rules! CALLMANY { ($f:expr, $array:expr) => { unsafe { ($f) ($array.len () as isize, $array.as_ptr () as *mut Lisp_Object) } } }

#[macro_export]
macro_rules! CALLN { ($f:expr, $($args:expr),* $(,)?) => {{
    // ARRAYELTS has to be a constant expression, and evaluating an argument to
    // count it is not (C evaluates each argument exactly once, when the array is
    // initialised): stringify! counts without evaluating (PORTING.md 2.1).
    let arr: [Lisp_Object; 0 $(+ { let _ = stringify!($args); 1 })*] = [$($args),*];
    CALLMANY! ($f, arr)
}} }

#[macro_export]
macro_rules! calln { ($($args:expr),* $(,)?) => { CALLN! (Ffuncall, $($args),*) } }

// C defines `call0` right after the CALLN/calln macros (lisp.h 3462-3471), and the port
// keeps that order: a `macro_rules!` is only in textual scope after its definition, so a
// body that used `calln!` above it would work at a crate root but not where the mirror is
// included inside a module (PORTING.md 0.1-9, which wants position-independent mirrors).
#[inline(always)]
pub fn call0 (fn_: Lisp_Object) -> Lisp_Object { calln! (fn_) }


/// C: lisp.h 4845: `extern Lisp_Object safe_funcall (ptrdiff_t, Lisp_Object*);` - the
/// error-catching callee `safe_calln` reaches.  It sits in a module with a glob import because
/// `eval.c`'s port *defines* it, and a same-scope declaration would be E0428 against that
/// definition (PORTING.md 0.1-10/P-21, the `keymap_decls` shape).  It has to be declared here
/// rather than at each call site: a `macro_rules!` body resolves a bare path at its *definition*
/// site (mixed-site hygiene), so `safe_calln` below can only name what this file can see.
pub mod lisp_safe_decls {
  use super::*;

  unsafe extern "C" {
    pub fn safe_funcall (nargs: ptrdiff_t, args: *mut Lisp_Object) -> Lisp_Object;
  }
}
use lisp_safe_decls::*;

/// C: lisp.h 4845-4847: `#define safe_calln(...) CALLMANY (safe_funcall, ((Lisp_Object [])
/// {__VA_ARGS__}))`.  The port used to expand this to `calln!` (`Ffuncall`) *plus* a
/// `maybe_quit ()` C does not make, which lost the error catch and the `inhibit-redisplay`
/// handling at all six call sites (docs/status.md 3.69, found by `src/eval.c`'s lane).
#[macro_export]
macro_rules! safe_calln { ($($args:expr),* $(,)?) => {{
    let arr: [Lisp_Object; 0 $(+ { let _ = stringify!($args); 1 })*] = [$($args),*];
    CALLMANY! (safe_funcall, arr)
}} }

#[macro_export]
macro_rules! list { ($($args:expr),* $(,)?) => {{
    // C's `list` passes __VA_ARGS__ twice (countof, then the call), so each argument is
    // evaluated once for the array and once for the argument — the port keeps that.
    let arr = [$($args),*];
    unsafe { listn (arr.len () as isize, $($args),*) }
}} }

// --- batch 8: DEFUN/DEFVAR machinery, AUTO_* helpers, aliases ---
// --- configuration constants that the C spells as macros -------------------

pub const USE_STACK_LISP_OBJECTS: bool = true;
pub const USE_STACK_CONS: bool = USE_STACK_LISP_OBJECTS;      // lisp.h 5735
pub const USE_STACK_STRING: bool = USE_STACK_CONS && !defined_GC_CHECK_STRING_BYTES;  // lisp.h 5736
pub const WRAP_SPECPDL_REF: bool = true;
pub const DEFINE_NON_NIL_Q_SYMBOL_MACROS: bool = true;
pub const LISP_WORDS_ARE_POINTERS: bool = true;      // recorded: the port only supports this
pub const LISP_OBJECT_IS_STRUCT: bool = false;       // CHECK_LISP_OBJECT_TYPE is not supported
pub const EMACS_LISP_H: bool = true;                 // the include guard has no Rust equivalent

#[inline(always)]
pub fn LISP_INITIALLY (w: Lisp_Object) -> Lisp_Object { w }

#[inline(always)]
pub fn LISPSYM_INITIALLY (index: usize) -> Lisp_Object {
    TAG_PTR_INITIALLY (Lisp_Type::Lisp_Symbol,
        (index * std::mem::size_of::<Lisp_Symbol> ()) as EMACS_INT)
}

#[inline(always)]
pub fn SYMBOL_INDEX (index: usize) -> usize { index }

#[inline(always)]
pub const fn lisp_h_Qnil () -> Lisp_Object { std::ptr::null_mut () }   // NIL_IS_ZERO: index 0, tag 0

#[inline(always)]
pub fn lisp_h_XFIXNUM_RAW (a: Lisp_Object) -> EMACS_INT { XLI (a) >> INTTYPEBITS }

#[inline(always)]
pub fn lisp_h_make_fixnum_wrap (n: EMACS_INT) -> Lisp_Object {
    XIL ((((n as EMACS_UINT) << INTTYPEBITS) as EMACS_INT) + Lisp_Type::Lisp_Int0 as EMACS_INT)
}

#[inline(always)]
pub fn lisp_h_make_fixnum (n: EMACS_INT) -> Lisp_Object {
    eassert! (!FIXNUM_OVERFLOW_P (n));
    lisp_h_make_fixnum_wrap (n)
}


/// C's `intspec` is a string literal, `0` or `NULL` (src/lisp.h 3427-3431, and the
/// frozen table has exactly those three); the union's first member is the C
/// string, so a string becomes a NUL-terminated pointer and the other two are
/// the null pointer, as C's `{intspec}` initialiser gives.
#[macro_export]
macro_rules! intspec_string {
    (0) => { core::ptr::null () };
    (NULL) => { core::ptr::null () };
    ($s:literal) => { concat!($s, "\0").as_ptr () as *const core::ffi::c_char };
}

/// C's `DEFUN` stores the function's address in the union member whose type is
/// `Lisp_Object (*) (…)` (`{.a ## maxargs = fnname}`); `_Noreturn` is a specifier
/// on the *declaration*, not part of that type.  Rust's `-> !` *is* part of the
/// type, so a noreturn subr's pointer is punned exactly where C's union
/// initialiser puns it.  SAFETY: both sides are function pointers of the same
/// size and the function never returns, so the return type is never observed.
#[macro_export]
macro_rules! subr_function_pointer {
    (Lisp_Object, $cname:ident, ($($ty:ty),*)) => { Some ($cname) };
    (!, $cname:ident, ($($ty:ty),*)) => {
        Some (unsafe {
            core::mem::transmute::<unsafe extern "C" fn($($ty),*) -> !,
                                   unsafe extern "C" fn($($ty),*) -> Lisp_Object> (
                $cname as unsafe extern "C" fn($($ty),*) -> !)
        })
    };
}

/// src/lisp.h 3406-3440: `DEFUN` declares the Lisp-callable function and the
/// file-static record `defsubr` registers.  Both parts are kept, in C's field
/// order; `maxargs` selects the union member exactly as C's
/// `{ .a ## maxargs = fnname }` does, and the arity arms are C's
/// `DEFUN_ARGS_0 … DEFUN_ARGS_8` / `MANY` / `UNEVALLED` table.  The docstring
/// literal stays in the invocation on purpose: like `make-docfile`, the DOC
/// generator reads docstrings from the source text (PORTING.md 5.1) and
/// `extract-abi.py` checks its digest against the frozen `subrs.tsv`.
///
/// The invocation is C's DEFUN with the body as the last argument (a Rust macro
/// invocation cannot be followed by items):
///
/// ```ignore
/// DEFUN! ("cons", Fcons, Scons, 2, 2, 0,
///         doc: "Create a new cons, give it CAR and CDR as components, and return it.",
///         (car: Lisp_Object, cdr: Lisp_Object),
///         {
///             return CALLN! (Fcons, car, cdr);
///         });
/// ```
#[macro_export]
macro_rules! DEFUN {
    // Entry: C's field order, with C's optional `attributes:` list.
    ($lname:literal, $cname:ident, $sname:ident, $minargs:expr, $maxargs:tt, $intspec:tt,
     doc: $doc:literal, attributes: noreturn, ($($arg:ident : $ty:ty),* $(,)?), $body:block) => {
        DEFUN! (@arity $lname, $cname, $sname, $minargs, $maxargs, $intspec,
                ($($arg : $ty),*), $body, !);
    };
    ($lname:literal, $cname:ident, $sname:ident, $minargs:expr, $maxargs:tt, $intspec:tt,
     doc: $doc:literal, ($($arg:ident : $ty:ty),* $(,)?), $body:block) => {
        DEFUN! (@arity $lname, $cname, $sname, $minargs, $maxargs, $intspec,
                ($($arg : $ty),*), $body, Lisp_Object);
    };
    (@arity $lname:literal, $cname:ident, $sname:ident, $minargs:expr, 0, $intspec:tt,
     ($($arg:ident : $ty:ty),*), $body:block, $ret:tt) =>
        { DEFUN! (@record $lname, $cname, $sname, $minargs, 0, $intspec,
                 ($($arg : $ty),*), $body, a0, $ret); };
    (@arity $lname:literal, $cname:ident, $sname:ident, $minargs:expr, 1, $intspec:tt,
     ($($arg:ident : $ty:ty),*), $body:block, $ret:tt) =>
        { DEFUN! (@record $lname, $cname, $sname, $minargs, 1, $intspec,
                 ($($arg : $ty),*), $body, a1, $ret); };
    (@arity $lname:literal, $cname:ident, $sname:ident, $minargs:expr, 2, $intspec:tt,
     ($($arg:ident : $ty:ty),*), $body:block, $ret:tt) =>
        { DEFUN! (@record $lname, $cname, $sname, $minargs, 2, $intspec,
                 ($($arg : $ty),*), $body, a2, $ret); };
    (@arity $lname:literal, $cname:ident, $sname:ident, $minargs:expr, 3, $intspec:tt,
     ($($arg:ident : $ty:ty),*), $body:block, $ret:tt) =>
        { DEFUN! (@record $lname, $cname, $sname, $minargs, 3, $intspec,
                 ($($arg : $ty),*), $body, a3, $ret); };
    (@arity $lname:literal, $cname:ident, $sname:ident, $minargs:expr, 4, $intspec:tt,
     ($($arg:ident : $ty:ty),*), $body:block, $ret:tt) =>
        { DEFUN! (@record $lname, $cname, $sname, $minargs, 4, $intspec,
                 ($($arg : $ty),*), $body, a4, $ret); };
    (@arity $lname:literal, $cname:ident, $sname:ident, $minargs:expr, 5, $intspec:tt,
     ($($arg:ident : $ty:ty),*), $body:block, $ret:tt) =>
        { DEFUN! (@record $lname, $cname, $sname, $minargs, 5, $intspec,
                 ($($arg : $ty),*), $body, a5, $ret); };
    (@arity $lname:literal, $cname:ident, $sname:ident, $minargs:expr, 6, $intspec:tt,
     ($($arg:ident : $ty:ty),*), $body:block, $ret:tt) =>
        { DEFUN! (@record $lname, $cname, $sname, $minargs, 6, $intspec,
                 ($($arg : $ty),*), $body, a6, $ret); };
    (@arity $lname:literal, $cname:ident, $sname:ident, $minargs:expr, 7, $intspec:tt,
     ($($arg:ident : $ty:ty),*), $body:block, $ret:tt) =>
        { DEFUN! (@record $lname, $cname, $sname, $minargs, 7, $intspec,
                 ($($arg : $ty),*), $body, a7, $ret); };
    (@arity $lname:literal, $cname:ident, $sname:ident, $minargs:expr, 8, $intspec:tt,
     ($($arg:ident : $ty:ty),*), $body:block, $ret:tt) =>
        { DEFUN! (@record $lname, $cname, $sname, $minargs, 8, $intspec,
                 ($($arg : $ty),*), $body, a8, $ret); };
    (@arity $lname:literal, $cname:ident, $sname:ident, $minargs:expr, MANY, $intspec:tt,
     ($($arg:ident : $ty:ty),*), $body:block, $ret:tt) =>
        { DEFUN! (@record $lname, $cname, $sname, $minargs, MANY, $intspec,
                 ($($arg : $ty),*), $body, aMANY, $ret); };
    (@arity $lname:literal, $cname:ident, $sname:ident, $minargs:expr, UNEVALLED, $intspec:tt,
     ($($arg:ident : $ty:ty),*), $body:block, $ret:tt) =>
        { DEFUN! (@record $lname, $cname, $sname, $minargs, UNEVALLED, $intspec,
                 ($($arg : $ty),*), $body, aUNEVALLED, $ret); };
    (@record $lname:literal, $cname:ident, $sname:ident, $minargs:expr, $maxargs:expr,
     $intspec:tt, ($($arg:ident : $ty:ty),*), $body:block, $member:ident, $ret:tt) => {
        // C: SUBR_SECTION_ATTRIBUTE + `static union Aligned_Lisp_Subr sname = {{{…}}};`
        #[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,subrs"))]
        #[cfg_attr(not(target_os = "macos"), unsafe(link_section = ".subrs"))]
        static mut $sname: Aligned_Lisp_Subr = Aligned_Lisp_Subr {
            s: Lisp_Subr {
                header: vectorlike_header {
                    size: (pvec_type::PVEC_SUBR as isize) << PSEUDOVECTOR_AREA_BITS,
                },
                function: Lisp_Subr_function {
                    $member: subr_function_pointer! ($ret, $cname, ($($ty),*)),
                },
                min_args: $minargs as i16,
                max_args: $maxargs as i16,
                symbol_name: concat!($lname, "\0").as_ptr () as *const core::ffi::c_char,
                intspec: Lisp_Subr_intspec { string: intspec_string! ($intspec) },
                command_modes: lisp_h_Qnil (),
                doc: 0,
                // C's initialiser stops at `lisp_h_Qnil`, so under HAVE_NATIVE_COMP the
                // four fields after `doc` are zero-filled by the aggregate rule (lisp.h
                // 2201-2206); Rust has to spell them.  Turning this arm's compiler on
                // (`dead-arms` flips `have_native_comp`, review A's F6) is what found them.
                #[cfg(feature = "have_native_comp")]
                native_comp_u: lisp_h_Qnil (),
                #[cfg(feature = "have_native_comp")]
                native_c_name: std::ptr::null_mut (),
                #[cfg(feature = "have_native_comp")]
                lambda_list: lisp_h_Qnil (),
                #[cfg(feature = "have_native_comp")]
                r#type: lisp_h_Qnil (),
            },
        };

        // C: `Lisp_Object fnname` (external linkage, declared here, not among the
        // dump-only statics), plus the port's `#[unsafe(no_mangle)]` (PORTING.md 0.2).
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $cname ($($arg : $ty),*) -> $ret $body
    };
}

/* C's DEFVAR_* macros *statically* initialize the forwarding object - e.g. `static struct
   Lisp_Fwd const o_fwd = {Lisp_Fwd_Obj, .u.objvar = &globals.f_##vname}` (lisp.h 3495-3531) - so
   the pointer is a link-time constant that is correct in any process, including one that has just
   *loaded a dump*, where `syms_of_*` never runs again.  The five macros below used to assign the
   member at run time instead, which left every one of these forwarding objects with a NULL
   pointer after a dump load; `do_symval_forwarding` then dereferenced it (status 3.644).  */
macro_rules! DEFVAR_LISP {
    /* C spells the docstring `doc: /* … */` in the invocation itself, so the port accepts the
       label too (status 3.952, `src/xfns.rs`'s `syms_of_xfns`).  */
    ($lname:expr, $vname:ident, doc: $doc:expr) => { DEFVAR_LISP! ($lname, $vname, $doc) };
    ($lname:expr, $vname:ident, $doc:expr) => {{
        static mut O_FWD: Lisp_Fwd = Lisp_Fwd {
            r#type: Lisp_Fwd_Obj,
            u: Lisp_Fwd_u { objvar: std::ptr::addr_of_mut! ($vname) },
        };
        /* The name argument is C's NUL-terminated string: a plain Rust `&str` carries no
           terminating NUL, and `defvar_lisp`/`defvar_int`/... read one - 141 sites registered
           their variable under a name that ran into the next literal (status 3.617).  The
           binding below makes a plain `"..."` a compile error; use `c"..."`.  */
        let _: &core::ffi::CStr = $lname;
        unsafe {
            defvar_lisp (&raw const O_FWD, $lname.as_ptr () as *const c_char);
        }
    }};
}

macro_rules! DEFVAR_LISP_NOPRO {
    ($lname:expr, $vname:ident, $doc:expr) => {{
        static mut O_FWD: Lisp_Fwd = Lisp_Fwd {
            r#type: Lisp_Fwd_Obj,
            u: Lisp_Fwd_u { objvar: std::ptr::addr_of_mut! ($vname) },
        };
        /* The name argument is C's NUL-terminated string: a plain Rust `&str` carries no
           terminating NUL, and `defvar_lisp`/`defvar_int`/... read one - 141 sites registered
           their variable under a name that ran into the next literal (status 3.617).  The
           binding below makes a plain `"..."` a compile error; use `c"..."`.  */
        let _: &core::ffi::CStr = $lname;
        unsafe {
            defvar_lisp_nopro (&raw const O_FWD, $lname.as_ptr () as *const c_char);
        }
    }};
}

macro_rules! DEFVAR_INT {
    ($lname:expr, $vname:ident, $doc:expr) => {{
        static mut I_FWD: Lisp_Fwd = Lisp_Fwd {
            r#type: Lisp_Fwd_Int,
            u: Lisp_Fwd_u { intvar: std::ptr::addr_of_mut! ($vname) as *mut i64 },
        };
        /* The name argument is C's NUL-terminated string: a plain Rust `&str` carries no
           terminating NUL, and `defvar_lisp`/`defvar_int`/... read one - 141 sites registered
           their variable under a name that ran into the next literal (status 3.617).  The
           binding below makes a plain `"..."` a compile error; use `c"..."`.  */
        let _: &core::ffi::CStr = $lname;
        unsafe {
            // C: `.u.intvar = &V` with `intmax_t *intvar` and `EMACS_INT` = intmax_t; the
            // port spells the union with C's intmax_t (i64) and EMACS_INT as isize.
            defvar_int (&raw const I_FWD, $lname.as_ptr () as *const c_char);
        }
    }};
}

macro_rules! DEFVAR_BOOL {
    /* C spells the docstring `doc: /* … */` in the invocation itself, so the port accepts the
       label too (status 3.952, `src/xfns.rs`'s `syms_of_xfns`).  */
    ($lname:expr, $vname:ident, doc: $doc:expr) => { DEFVAR_BOOL! ($lname, $vname, $doc) };
    ($lname:expr, $vname:ident, $doc:expr) => {{
        static mut B_FWD: Lisp_Fwd = Lisp_Fwd {
            r#type: Lisp_Fwd_Bool,
            u: Lisp_Fwd_u { boolvar: std::ptr::addr_of_mut! ($vname) },
        };
        /* The name argument is C's NUL-terminated string: a plain Rust `&str` carries no
           terminating NUL, and `defvar_lisp`/`defvar_int`/... read one - 141 sites registered
           their variable under a name that ran into the next literal (status 3.617).  The
           binding below makes a plain `"..."` a compile error; use `c"..."`.  */
        let _: &core::ffi::CStr = $lname;
        unsafe {
            defvar_bool (&raw const B_FWD, $lname.as_ptr () as *const c_char);
        }
    }};
}

/// DEFVAR_KBOARD stores a KBOARD offset, which keyboard.rs provides.
macro_rules! DEFVAR_KBOARD {
    ($lname:expr, $vname:expr, $doc:expr) => {{
        static mut KO_FWD: Lisp_Fwd = Lisp_Fwd {
            r#type: Lisp_Fwd_Kboard_Obj,
            u: Lisp_Fwd_u { kbdoffset: $vname as c_int },
        };
        /* The name argument is C's NUL-terminated string: a plain Rust `&str` carries no
           terminating NUL, and `defvar_lisp`/`defvar_int`/... read one - 141 sites registered
           their variable under a name that ran into the next literal (status 3.617).  The
           binding below makes a plain `"..."` a compile error; use `c"..."`.  */
        let _: &core::ffi::CStr = $lname;
        unsafe {
            defvar_kboard (&raw const KO_FWD, $lname.as_ptr () as *const c_char);
        }
    }};
}

/// DEFSYM is empty in the reference as well (src/lisp.h 2394): the symbols come
/// from the generated globals, which the port generates in globals.rs.
macro_rules! DEFSYM { ($sym:ident, $name:expr) => {} }

macro_rules! EXFUN { ($fnname:ident, $maxargs:tt) => {} }
macro_rules! DEFUN_ARGS_1 { () => { (Lisp_Object) } }
macro_rules! DEFUN_ARGS_2 { () => { (Lisp_Object, Lisp_Object) } }
macro_rules! DEFUN_ARGS_3 { () => { (Lisp_Object, Lisp_Object, Lisp_Object) } }
macro_rules! DEFUN_ARGS_4 { () => { (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) } }
macro_rules! DEFUN_ARGS_5 { () => { (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) } }
macro_rules! DEFUN_ARGS_6 { () => { (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) } }
macro_rules! DEFUN_ARGS_7 { () => { (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) } }
macro_rules! DEFUN_ARGS_8 { () => { (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) } }
/// DEFINE_LISP_SYMBOL (src/lisp.h 883) exists so a debugger can see the symbol;
/// the port's globals.rs provides the Q symbols directly (P-23).
macro_rules! DEFINE_LISP_SYMBOL { ($name:ident) => {} }
macro_rules! DEFUN_ARGS_0 { () => { () } }
macro_rules! DEFUN_ARGS_MANY { () => { (isize, *mut Lisp_Object) } }
macro_rules! DEFUN_ARGS_UNEVALLED { () => { (Lisp_Object) } }

// --- AUTO_* stack objects (src/lisp.h 5760-5830) ---------------------------
//
// C's macro body is `make_lisp_ptr ((&(struct Lisp_String) {{…}}), Lisp_String)` (lisp.h
// 5820-5826): a *compound literal*, whose storage lasts to the end of the **enclosing block**
// (C99 6.5.2.5) - which is what lisp.h's own comment says, "the object has the lifetime of the
// containing block".  A Rust `let` inside a macro's branch block ends there instead, so the
// returned object would point at dead stack: ASan named exactly that (`stack-use-after-scope`
// in `Fstring_equal`, reached from `assoc_ignore_text_properties` under `get_minibuffer`) and
// that is the shape which corrupts malloc's metadata (PORTING.md P-113).
//
// Every macro below therefore declares its storage with a binding of the **caller's block** and
// hands the object back in a second binding of the same name, which shadows it: shadowing
// leaves the first binding's scope at the end of the block, exactly as C's compound literal
// does.  C's *expression* forms (`STACK_CONS`, `AUTO_CONS_EXPR`, `AUTO_STR`,
// `AUTO_STR_WITH_LEN`) cannot be written that way in Rust, so their call sites use the
// statement forms - a deviation P-113 ② records.

/// C: `#define STACK_CONS(a, b) make_lisp_ptr (&((struct Lisp_Cons) {{…}}), Lisp_Cons)`.
///
/// Rust has no compound literal, so there is no expression form whose storage lives to the end
/// of the enclosing block (PORTING.md P-113): every call site uses the statement macros
/// (`AUTO_CONS!`, `AUTO_LIST1..4!`), whose storage binding sits in the caller's block.  The name
/// stays in the mirror so its macro set matches lisp.h's, and a use that has no storage binding
/// fails at compile time instead of pointing at dead stack.
macro_rules! STACK_CONS {
    ($a:expr, $b:expr) => {
        compile_error! ("STACK_CONS has no Rust counterpart: use AUTO_CONS! (PORTING.md P-113)")
    };
}

/// C: `#define AUTO_CONS_EXPR(a, b) (USE_STACK_CONS ? STACK_CONS (a, b) : Fcons (a, b))` - the
/// same missing compound literal as `STACK_CONS` (PORTING.md P-113 ②); use `AUTO_CONS!`.
macro_rules! AUTO_CONS_EXPR {
    ($a:expr, $b:expr) => {
        compile_error! ("AUTO_CONS_EXPR has no Rust counterpart: use AUTO_CONS! (PORTING.md P-113)")
    };
}

/// C: `#define AUTO_STR_WITH_LEN(str, len) (USE_STACK_STRING ? (make_lisp_ptr …) :
/// make_unibyte_string (str, len))` - the same missing compound literal (PORTING.md P-113 ②);
/// use `AUTO_STRING_WITH_LEN!`, which binds the storage in the caller's block.
macro_rules! AUTO_STR_WITH_LEN {
    ($str:expr, $len:expr) => {
        compile_error! ("AUTO_STR_WITH_LEN has no Rust counterpart: use AUTO_STRING_WITH_LEN! \
                         (PORTING.md P-113)")
    };
}

/// C: `#define AUTO_STR(str) AUTO_STR_WITH_LEN (str, strlen (str))` (PORTING.md P-113 ②);
/// use `AUTO_STRING!`.
macro_rules! AUTO_STR {
    ($str:expr) => {
        compile_error! ("AUTO_STR has no Rust counterpart: use AUTO_STRING! (PORTING.md P-113)")
    };
}

#[macro_export]
macro_rules! AUTO_CONS { ($name:ident, $a:expr, $b:expr) => {
    let mut $name = Lisp_Cons { u: Lisp_Cons_u { s: Lisp_Cons_s {
        car: $a, u: Lisp_Cons_s_u { cdr: $b } } } };
    let $name = if USE_STACK_CONS {
        make_lisp_ptr (std::ptr::addr_of_mut! ($name) as *mut c_void, Lisp_Type::Lisp_Cons)
    } else {
        unsafe { Fcons ($name.u.s.car, $name.u.s.u.cdr) }
    };
} }

/// Link `n` stack cells (already carrying their cars) into a nil-terminated list and return the
/// head.  C's `AUTO_LIST2 (name, a, b)` is `STACK_CONS (a, STACK_CONS (b, Qnil ()))` - two
/// compound literals, each with the enclosing block's lifetime - so the port declares the cells
/// in one binding of the caller's block and links them here (PORTING.md P-113 ③).
#[inline(always)]
pub unsafe fn auto_list_link (cells: *mut Lisp_Cons, n: usize) -> Lisp_Object
{
  let mut i: usize = n;
  let mut head: Lisp_Object = Qnil ();
  while i > 0
    {
      i -= 1;
      (*cells.add (i)).u.s.u.cdr = head;
      head = make_lisp_ptr (cells.add (i) as *mut c_void, Lisp_Type::Lisp_Cons);
    }
  head
}

#[macro_export]
macro_rules! AUTO_LIST1 { ($name:ident, $a:expr) => {
    let mut $name: [Lisp_Cons; 1] = unsafe { std::mem::zeroed () };
    $name[0].u.s.car = $a;
    let $name = if USE_STACK_CONS { unsafe { auto_list_link ($name.as_mut_ptr (), 1) } }
                else { unsafe { list1 ($name[0].u.s.car) } };
} }

#[macro_export]
macro_rules! AUTO_LIST2 { ($name:ident, $a:expr, $b:expr) => {
    let mut $name: [Lisp_Cons; 2] = unsafe { std::mem::zeroed () };
    $name[0].u.s.car = $a;
    $name[1].u.s.car = $b;
    let $name = if USE_STACK_CONS { unsafe { auto_list_link ($name.as_mut_ptr (), 2) } }
                else { unsafe { list2 ($name[0].u.s.car, $name[1].u.s.car) } };
} }

#[macro_export]
macro_rules! AUTO_LIST3 { ($name:ident, $a:expr, $b:expr, $c:expr) => {
    let mut $name: [Lisp_Cons; 3] = unsafe { std::mem::zeroed () };
    $name[0].u.s.car = $a;
    $name[1].u.s.car = $b;
    $name[2].u.s.car = $c;
    let $name = if USE_STACK_CONS { unsafe { auto_list_link ($name.as_mut_ptr (), 3) } }
                else { unsafe { list3 ($name[0].u.s.car, $name[1].u.s.car, $name[2].u.s.car) } };
} }

#[macro_export]
macro_rules! AUTO_LIST4 { ($name:ident, $a:expr, $b:expr, $c:expr, $d:expr) => {
    let mut $name: [Lisp_Cons; 4] = unsafe { std::mem::zeroed () };
    $name[0].u.s.car = $a;
    $name[1].u.s.car = $b;
    $name[2].u.s.car = $c;
    $name[3].u.s.car = $d;
    let $name = if USE_STACK_CONS { unsafe { auto_list_link ($name.as_mut_ptr (), 4) } }
                else { unsafe { list4 ($name[0].u.s.car, $name[1].u.s.car,
                                       $name[2].u.s.car, $name[3].u.s.car) } };
} }

#[macro_export]
macro_rules! AUTO_STRING_WITH_LEN { ($name:ident, $str:expr, $len:expr) => {
    let mut $name = Lisp_String { u: Lisp_String_u { s: Lisp_String_s {
        size: $len, size_byte: -1, intervals: std::ptr::null_mut (),
        data: $str as *mut u8 } } };
    let $name = if USE_STACK_STRING {
        make_lisp_ptr (std::ptr::addr_of_mut! ($name) as *mut c_void, Lisp_Type::Lisp_String)
    } else {
        unsafe { make_unibyte_string ($name.u.s.data as *const c_char,
                                          $name.u.s.size as ptrdiff_t) }
    };
} }

#[macro_export]
macro_rules! AUTO_STRING { ($name:ident, $str:expr) => {
    AUTO_STRING_WITH_LEN! ($name, $str, unsafe { strlen ($str) } as isize)
} }


// --- setjmp wrappers (the HAVE__SETJMP branch of src/lisp.h 2271-2287) ------

#[inline(always)]
pub unsafe fn sys_setjmp (j: *mut jmp_buf) -> c_int {
    unsafe { _setjmp (j as *mut c_void) }
}

#[inline(always)]
pub unsafe fn sys_longjmp (j: *mut jmp_buf, v: c_int) -> ! {
    unsafe { _longjmp (j as *mut c_void, v) }
}


// --- CONS_TO_INTEGER (src/lisp.h 4188) -------------------------------------

/// The C macro picks the signed or unsigned helper from the target type; the
/// port does the same through a trait.
pub trait ConsToInteger: Sized {
    fn cons_to_integer (cons: Lisp_Object) -> Self;
}

impl ConsToInteger for i64 {
    fn cons_to_integer (cons: Lisp_Object) -> i64 {
        unsafe { cons_to_signed (cons, i64::MIN, i64::MAX) }
    }
}

impl ConsToInteger for u64 {
    fn cons_to_integer (cons: Lisp_Object) -> u64 {
        unsafe { cons_to_unsigned (cons, u64::MAX) }
    }
}

#[macro_export]
macro_rules! CONS_TO_INTEGER { ($cons:expr, $var:expr) => { $var = <_ as ConsToInteger>::cons_to_integer ($cons) } }

// --- declarations lisp.h makes for definitions that live in other units -----
// buffer.rs (which this mirror reaches the way buffer.h includes lisp.h) needs
// these; they are declared once, here, because a second declaration of the same
// name in one module is E0428.

// --- declarations lisp.h makes for definitions that live in other units ------------
// C declares these in lisp.h and defines them in one .c unit each (alloc.c, marker.c,
// emacs.c, chartab.c, …).  The port keeps them in a module and `use`s them, so that the
// defining unit's own definition shadows the import instead of colliding with it
// (E0428) — the shape P-21 uses for the `V*` globals and P-28 for the subr prototypes
// (PORTING.md P-39).  `buffer.rs` and the other mirrors reach them through this glob
// import, exactly as a translation unit reaching lisp.h does.

pub mod other_units {
    // the mirror's own types/constants live in the crate root, as lisp.h's do
    use super::*;

    /* C lisp.h 4307: `extern void mark_fringe_data (void);`  */
    #[cfg (all (feature = "have_window_system", feature = "have_android"))]
    unsafe extern "C" { pub fn mark_fringe_data (); }
    /* C lisp.h 4906: `extern void module_finalize_function (const struct Lisp_Module_Function *);`  */
    #[cfg (all (feature = "have_window_system", feature = "have_android"))]
    unsafe extern "C" { pub fn module_finalize_function (function: *const c_void); }

    /* C 4889-4906 (`#ifdef HAVE_MODULES`): the opaque module-function pointer type and the six
       entry points `emacs-module.c` defines.  Android sets `HAVE_MODULES`, so `src/print.c`
       2188/2199 reads them there (`cannot find type module_funcptr`, measured).  */
    #[cfg (feature = "have_modules")]
    pub type module_funcptr = unsafe extern "C" fn ();   // C 4889: a plain function pointer
    #[cfg (feature = "have_modules")]
    unsafe extern "C" {
      /* C 4895-4906.  */
      pub fn funcall_module (function: Lisp_Object, nargs: ptrdiff_t,
                             args: *mut Lisp_Object) -> Lisp_Object;
      pub fn module_function_arity (function: *const c_void) -> Lisp_Object;
      pub fn module_function_documentation (function: *const c_void) -> Lisp_Object;
      pub fn module_function_interactive_form (function: *const c_void) -> Lisp_Object;
      pub fn module_function_command_modes (function: *const c_void) -> Lisp_Object;
      pub fn module_function_address (function: *const c_void) -> module_funcptr;
      pub fn module_function_data (function: *const c_void) -> *mut c_void;
    }
    /* C 4280/5436: two more declarations for definitions in other units (`src/fns.c`,
       `src/xterm.c`/`androidterm.c`); `src/keyboard.c` reads both and includes no header that
       names them on this target.  */
    unsafe extern "C" {
      /// C: lisp.h 4280: `extern Lisp_Object plist_get (Lisp_Object plist, Lisp_Object prop);`
      pub fn plist_get (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object;
      /// C: lisp.h 5436: `extern char *get_keysym_name (int);`
      pub fn get_keysym_name (keysym: c_int) -> *mut c_char;
    }
    /* C 5485-5486 (`#ifdef HAVE_TEXT_CONVERSION`): two entry points `src/textconv.c` defines and
       `src/keyboard.c` calls; `textconv.h` does not declare them, `lisp.h` does.  */
    #[cfg (feature = "have_text_conversion")]
    unsafe extern "C" {
      pub fn disable_text_conversion ();
      pub fn resume_text_conversion ();
    }
    /// C 5273-5274: the `#ifdef HAVE_PERSONALITY_ADDR_NO_RANDOMIZE` arm declares this one and
    /// `sysdep.c` 162 defines it; the `#else` arm is the inline above (C 5276-5280).  It sits in
    /// the module rather than at the mirror's root because the defining unit includes this mirror
    /// (P-39); without it, `src/emacs.c` 1510 read `cannot find function` on Android (measured,
    /// after the inline was gated with C's own condition).
    #[cfg (feature = "have_personality_addr_no_randomize")]
    unsafe extern "C" {
      pub fn maybe_disable_address_randomization (argc: c_int, argv: *mut *mut c_char) -> c_int;
    }

    // src/lisp.h 4268, 4330, 4345, 4363, 5039, 5110: cmds.c's callees — the definitions live in
    // editfns.c (concat2, replace_range), insdel.c (del_range, insert_and_inherit), search.c
    // (scan_newline_from_point) and indent.c (current_column); lisp.h declares them here.
    unsafe extern "C" {
        /* src/lisp.h 4268.  */
        /// C: Lisp_Object concat2 (Lisp_Object, Lisp_Object)
        pub fn concat2 (s1: Lisp_Object, s2: Lisp_Object) -> Lisp_Object;
        /* src/lisp.h 4330.  */
        /* src/lisp.h 4338-4339 and 4919: the three emitters syntax.c's
           `internal-describe-syntax-value` calls.  */
        /// C: void insert_char (int)
        pub fn insert_char (c: c_int);
        /// C: void insert_string (const char *)
        pub fn insert_string (str_: *const c_char);
        /// C: void insert1 (Lisp_Object)
        pub fn insert1 (object: Lisp_Object);
        /// C: void insert_and_inherit (const char *, ptrdiff_t)
        pub fn insert_and_inherit (str_: *const c_char, nbytes: ptrdiff_t);
        /* src/lisp.h 4345.  */
        /// C: void insert (const char *, ptrdiff_t)
        pub fn insert (str_: *const c_char, nbytes: ptrdiff_t);
        /// C: void del_range (ptrdiff_t, ptrdiff_t)
        pub fn del_range (from: ptrdiff_t, to: ptrdiff_t);
        /* src/lisp.h 4363: the six parameters of the replacement.  */
        /// C: void replace_range (ptrdiff_t, ptrdiff_t, Lisp_Object, bool, bool, bool)
        pub fn replace_range (from: ptrdiff_t, to: ptrdiff_t, new_: Lisp_Object,
                              prepare: bool, inherit: bool, markers: bool);
        /* src/lisp.h 5037.  */
        /// C: void scan_newline (ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t, bool)
        pub fn scan_newline (from: ptrdiff_t, from_byte: ptrdiff_t, to: ptrdiff_t,
                             to_byte: ptrdiff_t, count: ptrdiff_t, allow_quit: bool);
        /* src/lisp.h 5039.  */
        /// C: ptrdiff_t scan_newline_from_point (ptrdiff_t, ptrdiff_t *, ptrdiff_t *)
        pub fn scan_newline_from_point (count: ptrdiff_t, pos: *mut ptrdiff_t,
                                        pos_byte: *mut ptrdiff_t) -> ptrdiff_t;
        /* src/lisp.h 5110.  */
        /// C: ptrdiff_t current_column (void)
        pub fn current_column () -> ptrdiff_t;
        /* src/lisp.h 5546: `extern void *xnrealloc (void *, ptrdiff_t, ptrdiff_t)`.  */
        /// C: void *xnrealloc (void *, ptrdiff_t, ptrdiff_t)
        pub fn xnrealloc (block: *mut c_void, nitems: ptrdiff_t, item_size: ptrdiff_t)
          -> *mut c_void;
    }

    // src/lisp.h 619, 5298, 5541: bignum.c's three remaining callees.
    unsafe extern "C" {
        pub fn args_out_of_range_3 (a: Lisp_Object, b: Lisp_Object, c: Lisp_Object) -> !;
        pub fn get_random_ulong () -> c_ulong;
        pub fn xrealloc (block: *mut c_void, size: usize) -> *mut c_void;
    }

    // src/lisp.h 4195, 4261, 4540, 4563-4564: character.c's callees.  `args_out_of_range`
    // is eval.c's keymap helper, `validate_subarray` editfns.c's, and the three allocation
    // entries (`string_overflow`, `make_uninit_string`, `make_uninit_multibyte_string`)
    // alloc.c's; `AVOID` is `-> !` (PORTING.md 5.2).
    unsafe extern "C" {
        pub fn args_out_of_range (a: Lisp_Object, b: Lisp_Object) -> !;
        pub fn validate_subarray (array: Lisp_Object, from: Lisp_Object, to: Lisp_Object,
                                  size: ptrdiff_t, ifrom: *mut ptrdiff_t,
                                  ito: *mut ptrdiff_t);
        /* src/lisp.h 608; bignum.c 480 defines it (bignum.rs already carries the
           definition, this unit only calls it).  */
        pub fn check_integer_range (x: Lisp_Object, lo: intmax_t, hi: intmax_t) -> intmax_t;
        pub fn string_overflow () -> !;
        pub fn make_uninit_string (length: EMACS_INT) -> Lisp_Object;
        pub fn make_uninit_multibyte_string (nchars: EMACS_INT, nbytes: EMACS_INT)
          -> Lisp_Object;
    }

    // src/lisp.h 4093, 4210, 4652, 4797, 4835: helpers and subrs whose *definitions*
    // live in other units (floatfns.c is the first unit that calls them).  The port
    // declares them here, as lisp.h does, and the defining unit shadows the import
    // (P-39's shape).  `AVOID` is `-> !`.
    // src/lisp.h 690, 4296, 4300: floatfns.c's three non-static definitions.
    unsafe extern "C" {
        pub fn extract_float (num: Lisp_Object) -> c_double;              // floatfns.c
        pub fn double_integer_scale (d: c_double) -> c_int;               // floatfns.c
        pub fn fmod_float (x: Lisp_Object, y: Lisp_Object) -> Lisp_Object;  // floatfns.c
    }

    unsafe extern "C" {
        pub fn double_to_integer (d: c_double) -> Lisp_Object;            // bignum.c
        pub fn expt_integer (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;  // data.c
        pub fn make_float (d: c_double) -> Lisp_Object;                    // alloc.c
        pub fn overflow_error () -> !;                                     // bignum.c
    }

    // src/lisp.h 3919 (`alloc.c`) and 4689 (`chartab.c`); parameter names from those
    // definitions, C's prototypes have none.
    // src/lisp.h 4835-4836: AVOID is `-> !` (PORTING.md 5.2); `error` is C-variadic, so
    // the declaration is variadic too (the port's first variadic *call*).
    unsafe extern "C" {
        pub fn error (format: *const c_char, ...) -> !;
        pub fn verror (format: *const c_char, ap: core::ffi::VaList<'_>) -> !;
    }

    unsafe extern "C" {
        pub fn staticpro (var: *const Lisp_Object);
        pub fn map_char_table (c_function: Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object, Lisp_Object)>,
                               function: Lisp_Object, table: Lisp_Object, arg: Lisp_Object);
    }

    unsafe extern "C" {
        pub fn die (msg: *const c_char, file: *const c_char, line: c_int) -> !;
        pub static mut suppress_checking: bool;
        pub fn strlen (s: *const c_char) -> usize;
        // lisp.h 5302: `extern AVOID emacs_abort (void) NO_INLINE;` (the port keeps `-> !`).
        pub fn emacs_abort () -> !;
    }
    unsafe extern "C" {
        /// Set after Emacs has started up the first time (emacs.c).
        pub static mut initialized: bool;
        pub static mut gflags: gflags;
        }
    unsafe extern "C" {
        pub fn wrong_type_argument (predicate: Lisp_Object, value: Lisp_Object) -> !;
    }
    unsafe extern "C" {
        pub fn slow_eq (x: Lisp_Object, y: Lisp_Object) -> bool;
        pub fn memchr (s: *const c_void, c: c_int, n: usize) -> *mut c_void;
    }
    unsafe extern "C" {
        pub fn char_table_ref (table: Lisp_Object, idx: c_int) -> Lisp_Object;
        pub fn char_table_set (table: Lisp_Object, idx: c_int, val: Lisp_Object);
    }
    unsafe extern "C" {
    }
    unsafe extern "C" {
        pub fn make_misc_ptr (a: *mut c_void) -> Lisp_Object;
        pub fn bignum_to_double (n: Lisp_Object) -> f64;
        pub fn make_bigint (n: i64) -> Lisp_Object;
        pub fn make_biguint (n: u64) -> Lisp_Object;
    }
    // The specpdl is per-thread state in C (`#define specpdl (current_thread->m_specpdl)`,
    // thread.h 131-139): `include/thread.rs` defines all three as the macros C has, so no storage
    // is declared here (the stopgap that did so is retired).
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
        // lisp.h 4537-4538: alloc.c's two bool-vector constructors (data.c's
        // bool-vector drivers call the second).
        pub fn make_clear_bool_vector (size: EMACS_INT, init: bool) -> Lisp_Object;
        pub fn make_uninit_bool_vector (size: EMACS_INT) -> Lisp_Object;
    }
    unsafe extern "C" {
        // lisp.h 4541/4544 spell the length `ptrdiff_t`, not size_t.
        pub fn make_unibyte_string (s: *const c_char, len: ptrdiff_t) -> Lisp_Object;
        // lisp.h 4710 and 4747: print.c's and lread.c's exporters data.c calls.
        pub fn float_to_string (buf: *mut c_char, data: c_double) -> c_int;
        // The third parameter's pointee has no spelling in the probe's type table, so
        // the reference prototype rides along and the probe checks it there (P-55).
        /// C: Lisp_Object string_to_number (char const *, int, ptrdiff_t *)
        pub fn string_to_number (string: *const c_char, base: c_int,
                                 len: *mut ptrdiff_t) -> Lisp_Object;
        // src/lisp.h 4090 and 622: bignum.c's and data.c's own exporters (`number-to-string`
        // calls `bignum_to_string`; print.c and friends call `fixnum_to_string`).  The two
        // cons converters are already declared further down with lisp.h 4187's prototypes.
        pub fn bignum_to_string (num: Lisp_Object, base: c_int) -> Lisp_Object;
        pub fn fixnum_to_string (number: EMACS_INT, buffer: *mut c_char,
                                 end: *mut c_char) -> *mut c_char;
        pub fn make_string (s: *const c_char, len: ptrdiff_t) -> Lisp_Object;
        pub fn allocate_vector (size: isize) -> *mut Lisp_Vector;
        pub fn intern_1 (s: *const c_char, len: ptrdiff_t) -> Lisp_Object;
        pub fn intern_c_string_1 (s: *const c_char, len: ptrdiff_t) -> Lisp_Object;
        pub fn xsignal0 (error_symbol: Lisp_Object) -> !;
        // lisp.h 4793-4795: the signal helpers data.c calls (eval.c defines them).
        pub fn xsignal1 (error_symbol: Lisp_Object, arg: Lisp_Object) -> !;
        pub fn xsignal2 (error_symbol: Lisp_Object, a1: Lisp_Object, a2: Lisp_Object) -> !;
        pub fn xsignal3 (error_symbol: Lisp_Object, a1: Lisp_Object, a2: Lisp_Object,
                         a3: Lisp_Object) -> !;
        pub fn fast_string_match_internal (regexp: Lisp_Object, string: Lisp_Object,
                                           table: Lisp_Object) -> isize;
        pub fn fast_c_string_match_internal (regexp: Lisp_Object, string: *const c_char,
                                             len: ptrdiff_t, table: Lisp_Object) -> isize;
        pub fn check_obarray_slow (obarray: Lisp_Object) -> Lisp_Object;
        pub fn grow_specpdl_allocation ();
        pub fn unbind_to (count: specpdl_ref, value: Lisp_Object) -> Lisp_Object;
        pub fn maybe_garbage_collect ();
        pub fn probably_quit () -> !;
        pub fn xfree (p: *mut c_void);
        pub fn xmalloc (size: usize) -> *mut c_void;                     // src/lisp.h 5535
        pub fn indirect_function (object: Lisp_Object) -> Lisp_Object;   // src/lisp.h 4167 (data.c)
        // lisp.h 4164, 4168, 4197, 4198, 4212: data.c's exported helpers (the port
        // declares a unit's symbols in the same place C declares them).  `lispfwd` is
        // `const struct Lisp_Fwd *` (lisp.h 759), and `KBOARD` lives in keyboard.rs,
        // which this mirror cannot include (P-72 item 4), so that one carries the
        // reference prototype in a `/// C:` line (P-55) and the probe checks it there.
        pub fn wrong_choice (choice: Lisp_Object, wrong: Lisp_Object) -> !;
        pub fn find_symbol_value (symbol: Lisp_Object) -> Lisp_Object;
        /// C: KBOARD *kboard_for_bindings ()
        pub fn kboard_for_bindings () -> *mut c_void;
        pub fn do_symval_forwarding (valcontents: *const Lisp_Fwd) -> Lisp_Object;
        pub fn swap_in_global_binding (symbol: *mut Lisp_Symbol);
        // lisp.h 4748, 4800, 4864: data.c's watcher machinery calls these.
        pub fn map_obarray (obarray: Lisp_Object,
                            fn_: Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object)>,
                            arg: Lisp_Object);
        pub fn funcall_subr (subr: *mut Lisp_Subr, numargs: ptrdiff_t,
                             arg_vector: *mut Lisp_Object) -> Lisp_Object;
        pub fn let_shadows_buffer_binding_p (symbol: *mut Lisp_Symbol) -> bool;
        // lisp.h 4857-4863: eval.c's exports that profiler.c's `record_backtrace` and
        // `add_sample` call (`get_backtrace` is `Lisp_Object *array, ptrdiff_t size`).
        pub fn get_backtrace (array: *mut Lisp_Object, size: ptrdiff_t);
        pub fn backtrace_top_function () -> Lisp_Object;
        // lisp.h 4208-4209: data.c's `set_internal` calls this (it is data.c's too); the
        // fourth parameter is `KBOARD *`, unnameable from this mirror (P-72 item 4).
        /// C: void set_default_internal (Lisp_Object, Lisp_Object, enum Set_Internal_Bind, KBOARD *)
        pub fn set_default_internal (symbol: Lisp_Object, newval: Lisp_Object,
                                     bindflag: Set_Internal_Bind,
                                     buf: *mut c_void);
        // lisp.h 4183: `arithcompare` is data.c's own export (bytecode.c and the
        // comparison subrs below call it); the definition in data.rs shadows this
        // declaration, P-39's shape.
        pub fn arithcompare (num1: Lisp_Object, num2: Lisp_Object) -> cmp_bits_t;
        // lisp.h 4243, 4272, 4276: the list and string helpers data.c calls (fns.c
        // defines them; `string_char_to_byte` is `aref`/`aset`'s).
        pub fn list_length (list: Lisp_Object) -> ptrdiff_t;
        pub fn assq_no_quit (key: Lisp_Object, alist: Lisp_Object) -> Lisp_Object;
        pub fn string_char_to_byte (string: Lisp_Object, char_index: ptrdiff_t) -> ptrdiff_t;
        // lisp.h 4655, 4661: the two object helpers data.c calls (alloc.c defines them).
        pub fn build_symbol_with_pos (symbol: Lisp_Object, position: Lisp_Object) -> Lisp_Object;
        pub fn valid_lisp_object_p (obj: Lisp_Object) -> c_int;
        pub fn egetenv_internal (var: *const c_char, len: ptrdiff_t) -> *mut c_char;
        pub static mut pending_signals: bool;
        pub static mut Vascii_canon_table: Lisp_Object;
        pub static mut consing_until_gc: EMACS_INT;
        pub static mut chartab_size: [c_int; 4];
    }
    unsafe extern "C" {
        /// Mark every object of an array (src/lisp.h 4437, alloc.c).
        pub fn mark_objects (p: *mut Lisp_Object, n: ptrdiff_t);
    }
    unsafe extern "C" { pub static mut finalizers: Lisp_Finalizer; }
    unsafe extern "C" { pub static mut doomed_finalizers: Lisp_Finalizer; }
    unsafe extern "C" { pub static mut memory_signal_data: Lisp_Object; }
    unsafe extern "C" { pub static mut Vascii_downcase_table: Lisp_Object; }
    unsafe extern "C" { pub static mut staticvec: [*const Lisp_Object; NSTATICS as usize]; }
    unsafe extern "C" { pub static mut staticidx: c_int; }
    unsafe extern "C" { pub static mut hashtest_eq: hash_table_test; }
    unsafe extern "C" { pub static mut hashtest_eql: hash_table_test; }
    unsafe extern "C" { pub static mut hashtest_equal: hash_table_test; }
    unsafe extern "C" { pub static mut noninteractive_need_newline: bool; }
    unsafe extern "C" { pub static mut echo_area_buffer: [Lisp_Object; 2]; }
    unsafe extern "C" { pub static mut garbage_collection_inhibited: intptr_t; }
    unsafe extern "C" { pub static mut pending_malloc_warning: *mut c_char; }
    unsafe extern "C" { pub static mut zero_vector: Lisp_Object; }
    unsafe extern "C" { pub static mut number_finalizers_run: c_int; }
    unsafe extern "C" { pub static mut gc_in_progress: bool; }
    unsafe extern "C" { pub static mut __malloc_extra_blocks: usize; }
    unsafe extern "C" { pub static mut Vprin1_to_string_buffer: Lisp_Object; }
    unsafe extern "C" { pub static mut print_level: c_int; }
    unsafe extern "C" { pub static mut Vautoload_queue: Lisp_Object; }
    unsafe extern "C" { pub static mut Vrun_hooks: Lisp_Object; }
    unsafe extern "C" { pub static mut Vsignaling_function: Lisp_Object; }
    unsafe extern "C" { pub static mut Vminibuffer_list: Lisp_Object; }
    unsafe extern "C" { pub static mut last_minibuf_string: Lisp_Object; }
    unsafe extern "C" { pub static mut minibuf_level: EMACS_INT; }
    unsafe extern "C" { pub static mut command_loop_level: EMACS_INT; }
    unsafe extern "C" { pub static mut echo_message_buffer: Lisp_Object; }
    /// C: `extern struct kboard *echo_kboard;` (lisp.h 5081).  `struct kboard`'s definition is
    /// keyboard.h's and this mirror cannot name it (a unit that includes lisp.h need not include
    /// keyboard.h, exactly as in C, where the tag is incomplete here); the pointer is therefore
    /// the untyped one the port uses for incomplete targets, and `keyboard.rs` says where the
    /// definition is (PORTING.md P-72).
    unsafe extern "C" { pub static mut echo_kboard: *mut c_void; }
    unsafe extern "C" { pub static mut input_pending: bool; }
    unsafe extern "C" { pub static mut return_to_command_loop: sigjmp_buf; }
    unsafe extern "C" { pub static mut pending_funcalls: Lisp_Object; }
    unsafe extern "C" { pub static mut initial_argv: *mut *mut c_char; }
    unsafe extern "C" { pub static mut initial_argc: c_int; }
    unsafe extern "C" { pub static mut initial_argv0: *mut c_char; }
    unsafe extern "C" { pub static mut display_arg: bool; }
    unsafe extern "C" { pub static mut empty_unibyte_string: Lisp_Object; }
    unsafe extern "C" { pub static mut empty_multibyte_string: Lisp_Object; }
    unsafe extern "C" { pub static mut Vlibrary_cache: Lisp_Object; }
    unsafe extern "C" { pub static mut noninteractive: bool; }
    unsafe extern "C" { pub static mut no_site_lisp: bool; }
    unsafe extern "C" { pub static mut build_details: bool; }
    unsafe extern "C" { pub static mut daemon_type: c_int; }
    unsafe extern "C" { pub static mut w32_daemon_event: *mut c_void; }
    unsafe extern "C" { pub static mut fatal_error_in_progress: bool; }
    unsafe extern "C" { pub static mut inhibit_window_system: bool; }
    unsafe extern "C" { pub static mut emacs_wd: *const c_char; }
    unsafe extern "C" { pub static mut running_asynch_code: bool; }
    unsafe extern "C" { pub static mut char_ins_del_vector: *mut c_int; }
    unsafe extern "C" { pub static mut Vface_alternative_font_family_alist: Lisp_Object; }
    unsafe extern "C" { pub static mut Vface_alternative_font_registry_alist: Lisp_Object; }
    unsafe extern "C" { pub static mut profiler_memory_running: bool; }
    unsafe extern "C" { pub static mut my_edata: [c_char; 0]; }
    unsafe extern "C" { pub static mut my_endbss: [c_char; 0]; }
    unsafe extern "C" { pub static mut my_endbss_static: *mut c_char; }
    unsafe extern "C" {
        // src/lisp.h 4626-4627: `(int, int, int, enum pvec_type)`, returning
        // `struct Lisp_Vector *` (the port's `*mut Lisp_Vector`).
        pub fn allocate_pseudovector (lispsize: c_int, restsize: c_int, pvecsize: c_int,
                                     tag: pvec_type) -> *mut Lisp_Vector;
    }
    unsafe extern "C" {
        /* PORTING.md P-05/P-117 ⑧: `alloca`'s port and the pair that gives it the scope's
           lifetime.  `USE_SAFE_ALLOCA!` reads the chain's height with `sa_stack_mark`, and
           `SAFE_FREE!`/`SAFE_FREE_UNBIND_TO!` hand it back to `sa_stack_release_to` - the
           release must not ride on `safe_free`'s walk, which only covers what C itself
           pushed (P-117 ⑦).  The names are the port's own; the definitions live in
           `alloc.rs`.  */
        pub fn sa_stack_alloc (size: usize) -> *mut c_void;
        pub fn sa_stack_mark () -> usize;
        pub fn sa_stack_release_to (mark: usize);
        pub fn record_xmalloc (size: usize) -> *mut c_void;
        pub fn xzalloc (size: usize) -> *mut c_void;
        // src/lisp.h 5544: `void *xnmalloc (ptrdiff_t, ptrdiff_t)`, not the
        // `size_t, size_t` of `xcalloc` above it.
        pub fn xnmalloc (n: isize, size: isize) -> *mut c_void;
        // src/lisp.h 5539: `void *xcalloc (size_t, size_t)` (profiler.c's `make_log`).
        pub fn xcalloc (n: usize, size: usize) -> *mut c_void;
        // lisp.h 4390, 4781, 5100, 5548: the four cross-unit functions macros.c calls
        // (`message1` and `run_hook` are xdisp.c's and eval.c's; `command_loop_2` is
        // keyboard.c's own loop entry, declared here so every unit sees one prototype).
        // lisp.h 4654, 4926, 4989: undo.c's three cross-unit calls.  `struct buffer` is
        // incomplete in lisp.h, so `buffer_visited_file_modtime` carries the reference
        // prototype and takes the untyped pointer (P-55/P-72).
        pub fn inhibit_garbage_collection () -> specpdl_ref;
        // lisp.h 5354: category.h's CATEGORY_SET macro calls it; category.c defines it.
        pub fn char_category_set (c: c_int) -> Lisp_Object;
        // lisp.h 4251-4256, 4685-4687: category.c's hash-table and char-table calls.
        // Their pointer parameters have pointees the probe's type table cannot spell, so
        // each carries the reference prototype and the probe checks it there (P-55).
        /// C: Lisp_Object make_hash_table (const struct hash_table_test *, EMACS_INT, hash_table_weakness_t)
        pub fn make_hash_table (test: *const hash_table_test, size: EMACS_INT,
                                weakness: hash_table_weakness_t) -> Lisp_Object;
        /// C: ptrdiff_t hash_find_get_hash (struct Lisp_Hash_Table *, Lisp_Object, hash_hash_t *)
        pub fn hash_find_get_hash (h: *mut Lisp_Hash_Table, key: Lisp_Object,
                                   phash: *mut hash_hash_t) -> ptrdiff_t;
        /// C: Lisp_Object copy_char_table (Lisp_Object)
        pub fn copy_char_table (table: Lisp_Object) -> Lisp_Object;
        /// C: Lisp_Object char_table_ref_and_range (Lisp_Object, int, int *, int *)
        pub fn char_table_ref_and_range (table: Lisp_Object, idx: c_int,
                                         from: *mut c_int, to: *mut c_int) -> Lisp_Object;
        /// C: void char_table_set_range (Lisp_Object, int, int, Lisp_Object)
        pub fn char_table_set_range (table: Lisp_Object, from: c_int, to: c_int,
                                     val: Lisp_Object);

        pub fn make_buffer_string (start: ptrdiff_t, end: ptrdiff_t, nprops: bool)
                                   -> Lisp_Object;
        /// C: Lisp_Object buffer_visited_file_modtime (struct buffer *)
        pub fn buffer_visited_file_modtime (b: *mut c_void) -> Lisp_Object;
        pub fn message1 (s: *const c_char);

        /* src/lisp.h 4390-4391: message1_nolog is message1's sibling, which callint.c's
           `read-char` arm calls.  */
        /// C: void message1_nolog (const char *)
        pub fn message1_nolog (s: *const c_char);
        pub fn run_hook (hook: Lisp_Object);
        pub fn command_loop_2 (arg: Lisp_Object) -> Lisp_Object;
        // The second parameter's pointee has no spelling in the probe's type table, so
        // the reference prototype rides along and the probe checks it there (P-55).
        /// C: void *xpalloc (void *, ptrdiff_t *, ptrdiff_t, ptrdiff_t, ptrdiff_t)
        pub fn xpalloc (block: *mut c_void, n: *mut ptrdiff_t, incr: ptrdiff_t,
                        min_size: ptrdiff_t, size: ptrdiff_t) -> *mut c_void;
        pub fn record_unwind_protect_array (array: *mut Lisp_Object, nelts: isize);
        pub fn record_unwind_protect_ptr (func: Option<unsafe extern "C" fn(*mut c_void)>, arg: *mut c_void);
        pub fn record_unwind_protect_ptr_mark (function: Option<unsafe extern "C" fn(*mut c_void)>,
                                               arg: *mut c_void,
                                               mark: Option<unsafe extern "C" fn(*mut c_void)>);
        pub fn memory_full_up () -> !;
    }
    unsafe extern "C" {
        pub fn cons_to_signed (cons: Lisp_Object, lo: i64, hi: i64) -> i64;
        pub fn cons_to_unsigned (cons: Lisp_Object, hi: u64) -> u64;
    }
    unsafe extern "C" {
        // src/lisp.h 4492: `Lisp_Object listn (ptrdiff_t, Lisp_Object, ...)`; the
        // second parameter is fixed, only the rest are variadic.
        pub fn listn (nargs: isize, first: Lisp_Object, ...) -> Lisp_Object;
    }
    // defsubr / defvar take the subr and fwd records the DEFUN and DEFVAR_* forms
    // build; the records themselves live in the ported units.
    unsafe extern "C" {
        pub fn defsubr (sname: *mut Aligned_Lisp_Subr);
        pub fn defvar_lisp (fwd: *const Lisp_Fwd, lname: *const c_char);
        pub fn defvar_lisp_nopro (fwd: *const Lisp_Fwd, lname: *const c_char);
        pub fn defvar_int (fwd: *const Lisp_Fwd, lname: *const c_char);
        pub fn defvar_bool (fwd: *const Lisp_Fwd, lname: *const c_char);
        pub fn defvar_kboard (fwd: *const Lisp_Fwd, lname: *const c_char);
    }
    unsafe extern "C" {
        #[link_name = "_setjmp"]
        pub fn _setjmp (buf: *mut c_void) -> c_int;
        #[link_name = "_longjmp"]
        pub fn _longjmp (buf: *mut c_void, val: c_int) -> !;
    }
    // src/lisp.h 4818 (the alloc.c group).
    unsafe extern "C" {
      pub fn record_unwind_protect (function: Option<unsafe extern "C" fn (Lisp_Object)>,
                                    arg: Lisp_Object);
      /* src/lisp.h 5856-5857: `extern AVOID circular_list (Lisp_Object);` - the cycle
         handler the FOR_EACH_TAIL macro passes for a list that loops.  */
      /// C: void circular_list (Lisp_Object)
      pub fn circular_list (x: Lisp_Object) -> !;
    }

    // src/lisp.h 4088-4089, 4321-4357, 4561-4817 (the json.c group).
    unsafe extern "C" {
      /* src/lisp.h 4088.  */
      /// C: ptrdiff_t bignum_bufsize (Lisp_Object, int)
      pub fn bignum_bufsize (num: Lisp_Object, base: c_int) -> ptrdiff_t;
      /* src/lisp.h 4089.  */
      /// C: ptrdiff_t bignum_to_c_string (char *, ptrdiff_t, Lisp_Object, int)
      pub fn bignum_to_c_string (buf: *mut c_char, size: ptrdiff_t, num: Lisp_Object,
                                 base: c_int) -> ptrdiff_t;
      /* src/lisp.h 4319.  */
      /// C: void move_gap_both (ptrdiff_t, ptrdiff_t)
      pub fn move_gap_both (charpos: ptrdiff_t, bytepos: ptrdiff_t);
      /* src/lisp.h 4321.  */
      /// C: void make_gap (ptrdiff_t)
      pub fn make_gap (nbytes: ptrdiff_t);
      /* src/lisp.h 4333.  */
      /// C: void insert_from_gap_1 (ptrdiff_t, ptrdiff_t, bool)
      pub fn insert_from_gap_1 (nchars: ptrdiff_t, nbytes: ptrdiff_t,
                                text_at_gap_tail: bool);
      /* src/lisp.h 4353.  */
      /// C: void prepare_to_modify_buffer (ptrdiff_t, ptrdiff_t, ptrdiff_t *)
      pub fn prepare_to_modify_buffer (start: ptrdiff_t, end: ptrdiff_t,
                                       preserve_ptr: *mut ptrdiff_t);
      /* src/lisp.h 4355.  `struct buffer` is opaque here, as in the marker.c group.  */
      /// C: void invalidate_buffer_caches (struct buffer *, ptrdiff_t, ptrdiff_t)
      pub fn invalidate_buffer_caches (buf: *mut c_void, start: ptrdiff_t, end: ptrdiff_t);
      /* src/lisp.h 4356.  */
      /// C: void signal_after_change (ptrdiff_t, ptrdiff_t, ptrdiff_t)
      pub fn signal_after_change (charpos: ptrdiff_t, lendel: ptrdiff_t,
                                  lenins: ptrdiff_t);
      /* src/lisp.h 4357-4358.  */
      /// C: void adjust_after_insert (ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t)
      pub fn adjust_after_insert (from: ptrdiff_t, from_byte: ptrdiff_t, to: ptrdiff_t,
                                  newpos_byte: ptrdiff_t, newlen: ptrdiff_t);
      /* src/lisp.h 4561.  */
      /// C: Lisp_Object make_multibyte_string (const char *, ptrdiff_t, ptrdiff_t)
      pub fn make_multibyte_string (contents: *const c_char, nchars: ptrdiff_t,
                                    nbytes: ptrdiff_t) -> Lisp_Object;
      /* src/lisp.h 4579.  */
      /// C: Lisp_Object make_vector (ptrdiff_t, Lisp_Object)
      pub fn make_vector (size: ptrdiff_t, init: Lisp_Object) -> Lisp_Object;
      /* src/lisp.h 4730-4731.  */
      /// C: Lisp_Object intern_c_multibyte (const char *, ptrdiff_t, ptrdiff_t)
      pub fn intern_c_multibyte (str: *const c_char, nchars: ptrdiff_t,
                                 nbytes: ptrdiff_t) -> Lisp_Object;
      /* src/lisp.h 4796.  AVOID is `_Noreturn void`, which the port writes `-> !`.  */
      /// C: void signal_error (const char *, Lisp_Object)
      pub fn signal_error (message: *const c_char, arg: Lisp_Object) -> !;
      /* src/lisp.h 4798.  */
      /// C: void define_error (Lisp_Object, const char *, Lisp_Object)
      pub fn define_error (name: Lisp_Object, message: *const c_char, parent: Lisp_Object);
      /* src/lisp.h 4817.  */
      /// C: void specbind (Lisp_Object, Lisp_Object)
      pub fn specbind (symbol: Lisp_Object, value: Lisp_Object);
    }
    // src/lisp.h 4952-4956 (the marker.c group).  `struct buffer` is an incomplete type
    // in C here (buffer.h owns the definition) and the port cannot name it in a mirror
    // that units include without buffer.rs, so the pointer stays opaque: C only ever
    // passes `struct buffer *`, and `sizeof` never sees the pointee.  The `/// C:` line
    // carries the reference prototype, which spikes/lisp-abi-probe.sh type-checks
    // against lisp.h (PORTING.md 0.1-2).
    unsafe extern "C" {
      pub fn marker_position (marker: Lisp_Object) -> ptrdiff_t;
      pub fn marker_byte_position (marker: Lisp_Object) -> ptrdiff_t;
      /// C: ptrdiff_t buf_charpos_to_bytepos (struct buffer *, ptrdiff_t)
      pub fn buf_charpos_to_bytepos (b: *mut c_void, charpos: ptrdiff_t) -> ptrdiff_t;
      /// C: ptrdiff_t buf_bytepos_to_charpos (struct buffer *, ptrdiff_t)
      pub fn buf_bytepos_to_charpos (b: *mut c_void, bytepos: ptrdiff_t) -> ptrdiff_t;
      /* src/lisp.h 4958-4963, 4966: the rest of the marker.c group callint.c calls.  */
      /// C: void unchain_marker (struct Lisp_Marker *)
      pub fn unchain_marker (marker: *mut c_void);
      /// C: Lisp_Object set_marker_restricted (Lisp_Object, Lisp_Object, Lisp_Object)
      pub fn set_marker_restricted (marker: Lisp_Object, position: Lisp_Object,
                                    buffer: Lisp_Object) -> Lisp_Object;
      /// C: Lisp_Object set_marker_both (Lisp_Object, Lisp_Object, ptrdiff_t, ptrdiff_t)
      pub fn set_marker_both (marker: Lisp_Object, buffer: Lisp_Object,
                              charpos: ptrdiff_t, bytepos: ptrdiff_t) -> Lisp_Object;
      /// C: Lisp_Object set_marker_restricted_both (Lisp_Object, Lisp_Object, ptrdiff_t, ptrdiff_t)
      pub fn set_marker_restricted_both (marker: Lisp_Object, buffer: Lisp_Object,
                                         charpos: ptrdiff_t, bytepos: ptrdiff_t)
        -> Lisp_Object;
      /* src/lisp.h 4963.  */
      /// C: Lisp_Object build_marker (struct buffer *, ptrdiff_t, ptrdiff_t)
      pub fn build_marker (b: *mut c_void, charpos: ptrdiff_t, bytepos: ptrdiff_t)
        -> Lisp_Object;
      /* src/lisp.h 4966.  */
      /// C: void syms_of_marker ()
      pub fn syms_of_marker ();

      /* src/lisp.h 5551: the string allocator terminfo.c's tparam returns.  */
      /// C: char *xstrdup (char const *)
      pub fn xstrdup (s: *const c_char) -> *mut c_char;

      /* src/lisp.h 4490, 4982-4983, 5311-5314 (the kqueue.c group).  */
      /// C: Lisp_Object list5 (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object)
      pub fn list5 (a: Lisp_Object, b: Lisp_Object, c: Lisp_Object, d: Lisp_Object,
                    e: Lisp_Object) -> Lisp_Object;
      /// C: void report_file_error (const char *, Lisp_Object)
      pub fn report_file_error (string: *const c_char, err: Lisp_Object) -> !;
      /// C: void report_file_notify_error (const char *, Lisp_Object)
      pub fn report_file_notify_error (string: *const c_char, err: Lisp_Object) -> !;
      /// C: int emacs_open (const char *, int, int)
      pub fn emacs_open (file: *const c_char, oflags: c_int, mode: c_int) -> c_int;
      /// C: int emacs_close (int)
      pub fn emacs_close (fd: c_int) -> c_int;
      /* src/lisp.h 5362-5364.  */
      /// C: Lisp_Object directory_files_internal (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, bool, Lisp_Object, Lisp_Object)
      pub fn directory_files_internal (dir: Lisp_Object, full: Lisp_Object,
                                       match_: Lisp_Object, nosort: Lisp_Object,
                                       attr: bool, id: Lisp_Object, regexp: Lisp_Object)
        -> Lisp_Object;
    }

    // src/lisp.h 4227, 4277, 4352, 4364, 4540, 4565, 4696, 4733, 4741, 4823, 4976, 4981 (the
    // casefiddle.c / doc.c group).  `struct buffer` is an incomplete type in lisp.h here
    // too, but none of these takes one.
    unsafe extern "C" {
      /* src/lisp.h 4227: doc.c's get_doc_string counts the characters of a DOC-file
         chunk with it.  */
      /// C: ptrdiff_t multibyte_chars_in_text (const unsigned char *, ptrdiff_t)
      pub fn multibyte_chars_in_text (str_: *const u8, nbytes: ptrdiff_t) -> ptrdiff_t;
      /* src/lisp.h 4277: syntax.h 161's RE_SYNTAX_TABLE_BYTE_TO_CHAR.  */
      /// C: ptrdiff_t string_byte_to_char (Lisp_Object, ptrdiff_t)
      pub fn string_byte_to_char (string: Lisp_Object, bytepos: ptrdiff_t) -> ptrdiff_t;
      /* src/lisp.h 4352.  */
      /// C: void modify_text (ptrdiff_t, ptrdiff_t)
      pub fn modify_text (start: ptrdiff_t, end: ptrdiff_t);
      /* src/lisp.h 4364: the eight parameters of the replacement.  */
      /// C: void replace_range_2 (ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t, const char *, ptrdiff_t, ptrdiff_t, bool)
      pub fn replace_range_2 (from: ptrdiff_t, from_byte: ptrdiff_t, to: ptrdiff_t,
                              to_byte: ptrdiff_t, str_: *const c_char,
                              inschars: ptrdiff_t, insbytes: ptrdiff_t,
                              markers: bool);
      /* src/lisp.h 4540's `string_overflow` is declared above, in the character.c group
         (one definition point per name — PORTING.md 0.1-3).  */
      /* src/lisp.h 4565.  */
      /// C: Lisp_Object make_string_from_bytes (const char *, ptrdiff_t, ptrdiff_t)
      pub fn make_string_from_bytes (contents: *const c_char, nchars: ptrdiff_t,
                                     nbytes: ptrdiff_t) -> Lisp_Object;
      /* src/lisp.h 4696.  */
      /// C: Lisp_Object uniprop_table (Lisp_Object)
      pub fn uniprop_table (prop: Lisp_Object) -> Lisp_Object;
      /* src/lisp.h 4733.  */
      /// C: Lisp_Object oblookup (Lisp_Object, const char *, ptrdiff_t, ptrdiff_t)
      pub fn oblookup (obarray: Lisp_Object, str_: *const c_char, size: ptrdiff_t,
                       size_byte: ptrdiff_t) -> Lisp_Object;
      /* src/lisp.h 4741.  */
      /// C: Lisp_Object save_match_data_load (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object)
      pub fn save_match_data_load (file: Lisp_Object, noerror: Lisp_Object,
                                   nomessage: Lisp_Object, nosuffix: Lisp_Object,
                                   must_suffix: Lisp_Object) -> Lisp_Object;
      /* src/lisp.h 4823: doc.c's get_doc_string/`Snarf-documentation` guard the DOC
         file descriptor with it.  */
      /// C: void record_unwind_protect_int (void (*) (int), int)
      pub fn record_unwind_protect_int (function: Option<unsafe extern "C" fn (c_int)>,
                                        arg: c_int);
      /* src/lisp.h 5251-5252: casefiddle.c's two undo records.  undo.c defines both
         (`#[unsafe(no_mangle)]`), which shadows this declaration the way a DEFUN
         definition shadows its prototype (PORTING.md 0.1-10).  */
      /// C: void record_insert (ptrdiff_t, ptrdiff_t)
      pub fn record_insert (from: ptrdiff_t, nbytes: ptrdiff_t);
      /// C: void record_delete (ptrdiff_t, Lisp_Object, bool)
      pub fn record_delete (from: ptrdiff_t, del: Lisp_Object, record_markers: bool);
      /* lisp.h 5254: `extern void record_first_change (void);` - undo.c defines it, and the
         name is declared at the use site by the units that had not opened this block yet.  */
      /// C: void record_first_change (void)
      pub fn record_first_change ();
      /* src/lisp.h 4269: doc.c's three-piece "Cannot open doc string file" message.  */
      /// C: Lisp_Object concat3 (Lisp_Object, Lisp_Object, Lisp_Object)
      pub fn concat3 (a: Lisp_Object, b: Lisp_Object, c: Lisp_Object) -> Lisp_Object;
      /* src/lisp.h 5332-5333: the interrupted-read pair; doc.c uses the `_quit` one.  */
      /// C: ptrdiff_t emacs_read (int, void *, ptrdiff_t)
      pub fn emacs_read (fd: c_int, buf: *mut c_void, nbyte: ptrdiff_t) -> ptrdiff_t;
      /// C: ptrdiff_t emacs_read_quit (int, void *, ptrdiff_t)
      pub fn emacs_read_quit (fd: c_int, buf: *mut c_void, nbyte: ptrdiff_t) -> ptrdiff_t;
      /* src/lisp.h 4976.  */
      /// C: void close_file_unwind (int)
      pub fn close_file_unwind (fd: c_int);
      /* src/lisp.h 4981.  */
      /// C: void report_file_errno (const char *, Lisp_Object, int)
      pub fn report_file_errno (string: *const c_char, err: Lisp_Object, errnum: c_int) -> !;
    }
}
use other_units::*;

// --- constants the strengthened constant gate found missing (2026-10-02) -----------
// These are lisp.h's own constants; they were absent from this mirror while the gate
// still could not read one-line `enum { … };` forms (P-38), so `--check` reported the
// header "done" without them.  They are grouped here instead of at their C line
// position; docs/ports/lisp.md records that deviation.

// `CHECK_LISP_OBJECT_TYPE` is a *config* macro (lisp.h 204/364 only test it), and
// `include/config.rs` already carries it; redefining it here collided the moment both
// mirrors were in one unit (status 4.633).

// lisp.h 3206-3216 (`enum Lisp_Closure`) and 3219-3235 (`enum char_bits`): the gate could
// not read an enum whose `{` sits on the next line, so these fourteen members of lisp.h were
// never mirrored (P-59).  C's members have type `int`.
pub const CLOSURE_ARGLIST: c_int = 0;
pub const CLOSURE_CODE: c_int = 1;
pub const CLOSURE_CONSTANTS: c_int = 2;
pub const CLOSURE_STACK_DEPTH: c_int = 3;
pub const CLOSURE_DOC_STRING: c_int = 4;
pub const CLOSURE_INTERACTIVE: c_int = 5;

pub const CHAR_ALT: c_int = 0x0400000;
pub const CHAR_SUPER: c_int = 0x0800000;
pub const CHAR_HYPER: c_int = 0x1000000;
pub const CHAR_SHIFT: c_int = 0x2000000;
pub const CHAR_CTL: c_int = 0x4000000;
pub const CHAR_META: c_int = 0x8000000;
pub const CHAR_MODIFIER_MASK: c_int =
    CHAR_ALT | CHAR_SUPER | CHAR_HYPER | CHAR_SHIFT | CHAR_CTL | CHAR_META;
pub const CHARACTERBITS: c_int = 22;

// lisp.h 4174-4182: `enum { Cmp_Bit_EQ, Cmp_Bit_LT, Cmp_Bit_GT };` (C gives the three
// implicit values 0, 1, 2) and `typedef enum { Cmp_EQ = 1 << Cmp_Bit_EQ, … } cmp_bits_t;`.
pub const Cmp_Bit_EQ: c_int = 0;
pub const Cmp_Bit_LT: c_int = 1;
pub const Cmp_Bit_GT: c_int = 2;
pub const Cmp_EQ: c_int = 1 << Cmp_Bit_EQ;
pub const Cmp_LT: c_int = 1 << Cmp_Bit_LT;
pub const Cmp_GT: c_int = 1 << Cmp_Bit_GT;
/// C's `typedef enum { Cmp_EQ = 1 << Cmp_Bit_EQ, … } cmp_bits_t;`: clang's
/// `_Generic ((cmp_bits_t) 0, unsigned int: …)` picks `unsigned int` for the enum type while
/// the enumerators themselves are `int` (which is why `Cmp_*` above are `c_int`).  Review A
/// measured both.
pub type cmp_bits_t = c_uint;          // lisp.h 597 (P-01: the port rejects the true configuration)
pub const DEFAULT_HASH_SIZE: c_int = 0;                  // lisp.h 2766
pub const NSTATICS: c_int = 2048;                        // lisp.h 3921
pub const NEXT_ALMOST_PRIME_LIMIT: c_int = 11;           // lisp.h 4242
// lisp.h 4294: the C compilation itself rejects a target whose radix is neither 2 nor
// 16; the port has no such check unless it mirrors the static_assert (review D's F8).
static_assert! (FLT_RADIX == 2 || FLT_RADIX == 16);
// lisp.h 4149: `static_assert (PTRDIFF_MAX <= ULLONG_MAX);` — the fixnum->bignum path in
// bignum.h relies on it.
static_assert! ((isize::MAX as u64) <= u64::MAX);
// lisp.h 4295: `enum { LOG2_FLT_RADIX = FLT_RADIX == 2 ? 1 : 4 }`, with FLT_RADIX from
// <float.h> (mirrored by lib/include/float.in.rs).
pub const LOG2_FLT_RADIX: c_int = if FLT_RADIX == 2 { 1 } else { 4 };
pub const S2N_IGNORE_TRAILING: c_int = 1;                // lisp.h 4746
pub const defined_GC_CHECK_STRING_BYTES: bool = false;   // lisp.h 5743 (C spells the value through `defined (…)`; the frozen configuration leaves it undefined)
pub const SMALL_LIST_LEN_MAX: c_int = 127;               // lisp.h 5829

// --- FOR_EACH_TAIL_SAFE (src/lisp.h 5836-5912) ------------------------------
/// C's `struct for_each_tail_internal` (lisp.h 5848-5854): Brent's teleporting
/// tortoise-hare state for the two FOR_EACH_TAIL macros.
#[repr(C)]
pub struct for_each_tail_internal
{
  /* The object we're comparing to.  */
  pub tortoise: Lisp_Object,
  /* How many iterations of the algorithm have happened.  */
  pub l: ptrdiff_t,
}

/// C: `#define FOR_EACH_TAIL_THRESHOLD 4096` (lisp.h 5900), whose lisp.h 5902
/// `static_assert (POWER_OF_2 (FOR_EACH_TAIL_THRESHOLD))` the port repeats with the
/// expression inlined, because the port's POWER_OF_2 is a function, not a macro.
pub const FOR_EACH_TAIL_THRESHOLD: ptrdiff_t = 4096;
static_assert! ((FOR_EACH_TAIL_THRESHOLD & (FOR_EACH_TAIL_THRESHOLD - 1)) == 0);

/// FOR_EACH_TAIL_SAFE (lisp.h 5843-5846) is
/// `FOR_EACH_TAIL_INTERNAL (tail, (void) ((tail) = Qnil), false)`, i.e. FOR_EACH_TAIL_BASIC's
/// `for` loop (lisp.h 5873-5876) driven by FOR_EACH_TAIL_STEP_CYCLEP (lisp.h 5909-5912) with
/// CHECK_QUIT false, so the stepper's trailing `(…, false)` keeps it false and the only
/// effect is the periodic tortoise advance; the cycle arm sets TAIL to Qnil and the loop
/// then ends.
///
/// `state` is the caller's loop variable: the macro re-binds it (C's `for` mutates the
/// caller's object) so the body, which is the caller's own tokens, reads the value the
/// stepper produced.  C's macro takes the following statement as its body; Rust has no macro
/// form for that, so the body is the second argument:
///
/// ```ignore
/// FOR_EACH_TAIL_SAFE! (tail, { if NILP (XCDR (tail)) { … } });
/// ```
macro_rules! FOR_EACH_TAIL_SAFE {
    ($state:ident, $body:block) => {
        {
            /* C: `struct for_each_tail_internal li = { tail, 0 };` — the loop variable is the
               *caller's*: C's post-loop `NILP (tail)` reads what the stepper left, and the body's
               own `tail = XCDR (tail)` adds to it.  The port used to shadow it with a local, which
               left the caller's copy stale and made the call sites invent `tail_end` - and then
               check the wrong side of the step (status 3.612, `plist_put`).  */
            let mut li: for_each_tail_internal = for_each_tail_internal {
                tortoise: $state,
                l: 0,
            };
            /* C: `CONSP (tail) || (likely (NILP (tail)), false)`.  The stepper runs in the
               condition, not after the body: a `continue` in a C `for` runs the increment, a
               `continue` in a Rust `while` does not (P-159/P-160).  */
            let mut for_each_tail_first: bool = true;
            while { if !for_each_tail_first
                     {
                       /* Stepper: `(tail) = XCDR (tail)`, then
                          `! likely (! BASE_EQ (tail, li.tortoise))` - "cycle found".  */
                       $state = XCDR ($state);
                       if BASE_EQ ($state, li.tortoise)
                         { $state = Qnil (); }
                       else if ! likely (({ li.l += 1; li.l } & (FOR_EACH_TAIL_THRESHOLD - 1)) != 0)
                         {
                           if false || (POWER_OF_2 (li.l) && { li.tortoise = $state; false })
                             { }
                         }
                     }
                   for_each_tail_first = false;
                   CONSP ($state) || { likely (NILP ($state)); false } }
              {
                $body
              }
        }
    };
}

/// FOR_EACH_TAIL (lisp.h 5839-5840) is the same `for` loop driven by the same stepper as
/// FOR_EACH_TAIL_SAFE, with the two differences the C macro names: the cycle arm evaluates
/// CYCLE, which for this macro is `circular_list (tail)` - it signals, it does not fall out of
/// the loop - and CHECK_QUIT is true, so the periodic branch calls `maybe_quit ()`.
///
/// ```ignore
/// FOR_EACH_TAIL! (tail, { key = XCAR (tail); … });
/// ```
macro_rules! FOR_EACH_TAIL {
    ($state:ident, $body:block) => {
        {
            /* C: `struct for_each_tail_internal li = { tail, 0 };` — the loop variable is the
               *caller's*: C's post-loop `NILP (tail)` reads what the stepper left, and the body's
               own `tail = XCDR (tail)` adds to it.  The port used to shadow it with a local, which
               left the caller's copy stale and made the call sites invent `tail_end` - and then
               check the wrong side of the step (status 3.612, `plist_put`).  */
            let mut li: for_each_tail_internal = for_each_tail_internal {
                tortoise: $state,
                l: 0,
            };
            /* C: `CONSP (tail) || (likely (NILP (tail)), false)`.  As in FOR_EACH_TAIL_SAFE,
               the stepper runs in the condition so that a `continue` in a body reaches it
               (P-159/P-160); CHECK_QUIT is true, so the periodic branch calls `maybe_quit ()`.  */
            let mut for_each_tail_first: bool = true;
            while { if !for_each_tail_first
                     {
                       /* Stepper (lisp.h 5909-5912) with CHECK_QUIT true.  */
                       $state = XCDR ($state);
                       if BASE_EQ ($state, li.tortoise)
                         { circular_list ($state); }
                       else if ! likely (({ li.l += 1; li.l } & (FOR_EACH_TAIL_THRESHOLD - 1)) != 0)
                         {
                           maybe_quit ();
                           if POWER_OF_2 (li.l)
                             { li.tortoise = $state; }
                         }
                     }
                   for_each_tail_first = false;
                   CONSP ($state) || { likely (NILP ($state)); false } }
              {
                $body
              }
        }
    };
}

/// DOHASH (lisp.h 5861-5877): walk the key/value pairs of hash table H, binding K and V and
/// running the body only for entries that are in use.  C's macro pastes its two variables'
/// names into the loop's locals (`dohash_##k##_##v##_kv`), declares K and V in the `for`
/// initializer, assigns them in the condition, and attaches the body to the trailing
/// `if (hash_unused_entry_key_p (k)) ; else`.  Rust's counterpart passes the two names in as
/// metavariables (so the caller's body sees them through call-site hygiene) and takes the body
/// as the last argument; the increment keeps C's eassert that the table was not rehashed.
///
/// ```ignore
/// DOHASH! (h, k, v, { if ! first { … } });
/// ```
macro_rules! DOHASH {
    ($h:expr, $k:ident, $v:ident, $body:block) => {
        {
            let mut dohash_kv: *mut Lisp_Object = (*($h)).key_and_value;
            let dohash_end: *mut Lisp_Object =
                dohash_kv.wrapping_add (2 * HASH_TABLE_SIZE ($h) as usize);
            let dohash_base: *mut Lisp_Object = dohash_kv;
            let mut $k: Lisp_Object;
            let mut $v: Lisp_Object;
            /* The increment runs in the condition, so a `continue` in `$body` reaches it as it
               does in C's `for` (P-159/P-160); it also keeps the eassert that the table was not
               rehashed mid-loop.  */
            let mut dohash_first: bool = true;
            while { if !dohash_first
                     {
                       eassert! (dohash_base == (*($h)).key_and_value
                                 && dohash_end
                                    == dohash_base.wrapping_add (2 * HASH_TABLE_SIZE ($h) as usize));
                       dohash_kv = dohash_kv.wrapping_add (2);
                     }
                   dohash_first = false;
                   dohash_kv < dohash_end }
              {
                $k = *dohash_kv.offset (0);
                $v = *dohash_kv.offset (1); /* maybe unused */
                if ! hash_unused_entry_key_p ($k)
                  { $body }
              }
        }
    };
}
