// femacs: port of src/coding.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct/union field, file-scope object, local), same control flow, same error
// model.  The reference file's 32 `DEFUN`s use `DEFUN!` (PORTING.md 5.1); C's non-`static`
// definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2) and the C `static` helpers stay
// plain `unsafe fn`.
//
// **Branch selection.**  `src/coding.c` *is* in
// `docs/baseline/config/cond-eval-verified.txt` (`python3 tools/check-cond-eval.py
// src/coding.c` reports `13 conditions, 0 unknown, 0 disagreements [VERIFIED]`), so the
// frozen configuration's branch selection is clang's (P-08): the file's conditionals are
// three `#if 0` blocks (C 154-189, 205-243, 262-284 — the template bodies of the decode and
// encode general notes), `#ifdef HAVE_WCHAR_H` (C 291, live), `#ifdef DOS_NT` (C 5987, dead),
// `#if 0`/`#else` (C 6338-6361, the `#else` arm live), the
// `WINDOWSNT || CYGWIN || HAVE_ANDROID` block (C 8529-8589 with its nested
// `WINDOWSNT || CYGWIN` at 8547-8569, dead), `WINDOWSNT` at C 10432-10446 and 10458-10472
// (the `#else` arms live), and `ENABLE_UTF_8_CONVERTER_TEST` at C 10292-10415 and
// 11911-11914 (dead).  They are recorded in `docs/ports/coding.md` rather than transcribed
// (`#if 0` regions are dead in every configuration, docs/status.md 3.55's rule).
//
// **`coding.h`'s mirror and this unit's use sites.**  `emacs/src/coding.h`'s mirror
// (`femacs/src/include/coding.rs`) is complete (`extract-abi.py --check emacs/src/coding.h
// femacs/src/include/coding.rs` reports `OK`); the macros the header does *not* carry are
// coding.c's own (the `CODING_ISO_*`, `CODING_UTF_*` and `CODING_CCL_*` accessors of C
// 316-472), which are defined here.  Names C reaches through headers whose mirrors do not
// carry them are declared at their use site with C's own prototype in a `/// C:` line
// (PORTING.md 0.1-2/P-72 rule ③); the mirror gaps this unit reports and does not touch are
// listed in `docs/ports/coding.md`.
//
// **The `goto` labels.**  C's `no_more_source`, `invalid_code`, `label_done`,
// `no_more_source` and friends are the port's labelled blocks (`'label: { … break 'label; }`)
// or labelled loops, so a `goto` out of a loop keeps skipping exactly what C skips
// (PORTING.md 2.2).  The statement macros that both capture the caller's locals and jump to
// one of those labels (`ONE_MORE_BYTE`, `TWO_MORE_BYTES`, …) cannot be file-scope
// `macro_rules!`s — a macro resolves local variables and labels at its *definition* site —
// so, as in `src/ccl.rs` (docs/status.md 3.55), each is defined inside every function that
// uses it, after the locals it captures and inside the label's scope.
//
// **The packed `iso_2022_spec` flags.**  The mirror packs `single_shifting`, `bol` and
// `embedded_utf_8` into one `bits: u32`; C reads and writes them as lvalues
// (`CODING_ISO_SINGLE_SHIFTING (coding) = 1;`), which Rust has no place for, so those three
// macros yield a small `iso_2022_bit` proxy whose `get`/`set` read and write the bit
// (PORTING.md 1.3).  The field names this removes from the macro bodies are recorded in
// `docs/ports/macro-body-notes.tsv`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_assignments,
         unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn, unused_parens,
         clashing_extern_declarations, unused_comparisons, overflowing_literals,
         unreachable_code, unused_labels, while_true, unused_must_use)]

include!("include/config.rs");              // C 289: `#include <config.h>`
include!("../lib/include/wchar.rs");        // C 291-293: `#include <wchar.h>` (`HAVE_WCHAR_H`)
include!("include/lisp.rs");                // C 295: `#include "lisp.h"`
include!("include/character.rs");           // C 296: `#include "character.h"`
include!("include/buffer.rs");              // C 297: `#include "buffer.h"`
include!("include/charset.rs");             // C 298: `#include "charset.h"`
include!("include/itree.rs");               // C 297: `buffer.h` 28 `#include "itree.h"`
include!("include/systime.rs");             // C 302: `termhooks.h` 28 `#include "systime.h"` (`Time`)
include!("include/dispextern.rs");          // C 302: `termhooks.h` 27 `#include "dispextern.h"`
include!("include/font.rs");                // C 302: `dispextern.h` pulls `font.h`
include!("include/ccl.rs");                 // C 299: `#include "ccl.h"`
include!("include/composite.rs");           // C 300: `#include "composite.h"`
include!("include/coding.rs");              // C 301: `#include "coding.h"`
include!("include/termhooks.rs");           // C 302: `#include "termhooks.h"`
include!("include/pdumper.rs");             // C 303: `#include "pdumper.h"`

/* Use-site declarations (PORTING.md P-72 ③): names C declares in headers whose mirrors do
   not carry them.  Each keeps C's own prototype in a `/// C:` line.

   The four `<string.h>` names this block used to carry (`memset`, `memcpy`, `strncmp`,
   `strcpy`) are gone from it: C reaches them through `lisp.h` 30's `#include <string.h>`,
   which the port's chain now carries (`src/include/lisp.rs`, status 3.524), so keeping the
   copies here was `E0428` against the mirror's own declarations.  */

unsafe extern "C" {
  /// C: lisp.h 4322: `extern void make_gap_1 (struct buffer *, ptrdiff_t);`
  fn make_gap_1 (b: *mut buffer, delta: ptrdiff_t);
  /// C: lisp.h 4349-4351: `extern Lisp_Object del_range_2 (ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t, bool);`
  fn del_range_2 (from: ptrdiff_t, from_byte: ptrdiff_t, to: ptrdiff_t, to_byte: ptrdiff_t,
		  prepare: bool) -> Lisp_Object;
  /// C: lisp.h 4331-4333: `extern void insert_1_both (const char *, ptrdiff_t, ptrdiff_t, bool, bool, bool);`
  fn insert_1_both (string: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
		    inherit: bool, prepare: bool, before_markers: bool);
  /// C: lisp.h 4335-4336: `extern void insert_from_string (Lisp_Object, ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t, bool);`
  fn insert_from_string (string: Lisp_Object, pos: ptrdiff_t, pos_byte: ptrdiff_t,
			 length: ptrdiff_t, length_byte: ptrdiff_t, inherit: bool);
  /// C: lisp.h 4337-4338: `extern void insert_from_buffer (struct buffer *, ptrdiff_t, ptrdiff_t, bool);`
  fn insert_from_buffer (buf: *mut buffer, charpos: ptrdiff_t, nchars: ptrdiff_t,
			 inherit: bool);
  /// C: lisp.h 4348: `extern void del_range_both (ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t, bool);`
  fn del_range_both (from: ptrdiff_t, from_byte: ptrdiff_t, to: ptrdiff_t,
		     to_byte: ptrdiff_t, prepare: bool);
  /// C: lisp.h 4206-4207: `extern Lisp_Object nconc2 (Lisp_Object, Lisp_Object);`
  fn nconc2 (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4281-4282: `extern Lisp_Object plist_put (Lisp_Object plist, Lisp_Object prop, Lisp_Object val);`
  fn plist_put (plist: Lisp_Object, prop: Lisp_Object, val: Lisp_Object) -> Lisp_Object;
  /// C: termhooks.h 961: `extern struct terminal *decode_live_terminal (Lisp_Object);`
  fn decode_live_terminal (terminal: Lisp_Object) -> *mut terminal;
  /// C: lisp.h 4412: `extern Lisp_Object safe_eval (Lisp_Object);`
  fn safe_eval (form: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4334: `extern void insert_from_gap (ptrdiff_t, ptrdiff_t, bool, bool);`
  fn insert_from_gap (nchars: ptrdiff_t, nbytes: ptrdiff_t, text_at_gap_tail: bool,
		      before_markers: bool);
}

/* `xrealloc`, `string_overflow`, `make_gap` (lisp.h), `SIZE_MAX` (stdint.in.rs) and the
   buffer accessors are reached through the mirrors above.  */

/* The bitfield helpers C spells as plain member accesses.  `coding_system`'s ten members
   share one 4-byte unit (coding.rs 234-252); C reads and writes them directly, the port
   masks, and this pair is the one spelling of that operation (PORTING.md 1.3).  They are not
   C names, so no ledger compares them.  */
#[inline(always)]
unsafe fn coding_bits_get (coding: *mut coding_system, shift: u32, mask: u32) -> u32
{ ((*coding).bits >> shift) & mask }

#[inline(always)]
unsafe fn coding_bits_set (coding: *mut coding_system, shift: u32, mask: u32, value: u32)
{
  (*coding).bits = ((*coding).bits & !(mask << shift)) | ((value & mask) << shift);
}

/* `coding_system`'s `mode` and `common_flags` bitfields, spelled the way the mirror's
   accessors read them (PORTING.md 1.3).  C writes `coding->mode |= X` / `&= ~X` and tests
   `coding->common_flags & X`; these three helpers are that spelling for the writers.  */
#[inline(always)]
unsafe fn mode_of_int (coding: *mut coding_system) -> c_int
{ mode_of (coding) as c_int }

#[inline(always)]
unsafe fn coding_mode_set (coding: *mut coding_system, bits: c_int)
{ coding_bits_set (coding, CODING_SYSTEM_MODE_SHIFT, CODING_SYSTEM_MODE_MASK,
                   (mode_of (coding) as c_int | bits) as u32) }

#[inline(always)]
unsafe fn coding_mode_clear (coding: *mut coding_system, bits: c_int)
{ coding_bits_set (coding, CODING_SYSTEM_MODE_SHIFT, CODING_SYSTEM_MODE_MASK,
                   (mode_of (coding) as c_int & !bits) as u32) }

#[inline(always)]
unsafe fn common_flags_of (coding: *mut coding_system) -> c_uint
{ coding_bits_get (coding, CODING_SYSTEM_COMMON_FLAGS_SHIFT, CODING_SYSTEM_COMMON_FLAGS_MASK) }


/* C writes `coding->common_flags = X` / `|= X` in `setup_coding_system`; this is that
   spelling for the writer, next to the reader above (PORTING.md 1.3).  */
#[inline(always)]
unsafe fn common_flags_set (coding: *mut coding_system, flags: c_uint)
{ coding_bits_set (coding, CODING_SYSTEM_COMMON_FLAGS_SHIFT, CODING_SYSTEM_COMMON_FLAGS_MASK, flags) }

/* `coding_system`'s three-bit `eol_seen` member: C reads and `|=`s it directly.  */
#[inline(always)]
unsafe fn eol_seen_of (coding: *mut coding_system) -> c_int
{ coding_bits_get (coding, CODING_SYSTEM_EOL_SEEN_SHIFT, CODING_SYSTEM_EOL_SEEN_MASK) as c_int }

#[inline(always)]
unsafe fn coding_eol_seen_set (coding: *mut coding_system, seen: c_int)
{ coding_bits_set (coding, CODING_SYSTEM_EOL_SEEN_SHIFT, CODING_SYSTEM_EOL_SEEN_MASK, seen as u32) }
/* gnulib's `<stdckdint.h>` `ckd_add`: stores the wrapped sum in `$dst` and yields the
   overflow flag (P-15, the `ckd_add` family of docs/ports/token-aliases.tsv).  */
macro_rules! ckd_add {
  ($dst:ident, $a:expr, $b:expr) => {{
    let (ckd_add_value, ckd_add_overflow) = ($a).overflowing_add ($b);
    $dst = ckd_add_value;
    ckd_add_overflow
  }};
}

/* The three file-scope objects coding.h declares (`Vcoding_system_hash_table`,
   `safe_terminal_coding`, `emacs_mule_bytes`) are that header's only declarations the mirror
   keeps at its top level instead of in an `*_objs` module (compare `charset_objs`), so a
   crate-root definition here would be `E0428`.  They are defined in this module and
   glob-imported instead: the crate-root `extern` declaration shadows the import for name
   resolution while the definition still emits C's own symbol (PORTING.md P-21's mechanism).
   Reported as a mirror gap in docs/ports/coding.md; the mirror itself is not touched.  */
pub mod coding_objs
{
  use super::*;

  #[unsafe(no_mangle)]
  pub static mut Vcoding_system_hash_table: Lisp_Object = lisp_h_Qnil ();

  /* Coding system to be used to encode text for terminal display when
     terminal coding system is nil.  */
  #[unsafe(no_mangle)]
  pub static mut safe_terminal_coding: coding_system = unsafe { std::mem::zeroed () };

  #[unsafe(no_mangle)]
  pub static mut emacs_mule_bytes: [c_char; 256] = [0; 256];
}
use coding_objs::*;

/* `preferred_coding_system` is declared in coding.h 780 and defined by this unit at C 8522,
   but `complement_process_encoding_system` (C 6016) calls it first.  The declaration sits in
   a module that is glob-imported, so the definition further down shadows the import for name
   resolution (PORTING.md 0.1-10/P-21, the `coding_objs` mechanism).  */
pub mod coding_fwd
{
  use super::*;

  unsafe extern "C" {
    /// C: coding.h 780: `extern Lisp_Object preferred_coding_system (void);`
    pub fn preferred_coding_system () -> Lisp_Object;
    /// C: coding.h 695-697: `extern Lisp_Object decode_string_utf_8 (Lisp_Object, const char *, ptrdiff_t, Lisp_Object, bool, Lisp_Object, Lisp_Object);`
    pub fn decode_string_utf_8 (arg0: Lisp_Object, text: *const c_char, nbytes: ptrdiff_t,
				arg3: Lisp_Object, arg4: bool, arg5: Lisp_Object,
				arg6: Lisp_Object) -> Lisp_Object;
  }
}
use coding_fwd::*;

/* The `DEFVAR_*` variables this unit owns (docs/status.md §3.29): C stores them as fields
   of `struct emacs_globals`, the owner defines them here and shadows globals.rs's
   declarations.  `Vlast_code_conversion_error` is `record_conversion_result`'s carrier
   (C 11999); the other twenty-eight arrive with `syms_of_coding`, where C defines them.  */
#[unsafe(no_mangle)]
pub static mut Vlast_code_conversion_error: Lisp_Object = lisp_h_Qnil ();

/* The eight `DEFVAR_*` carriers the already-ported half of this unit reads.  C defines them
   at that half's own sites in `syms_of_coding` (C 11738+), which is not ported yet, so their
   storage is written here (the `Vlast_code_conversion_error` precedent above) and
   `syms_of_coding` will only register them; the remaining carriers arrive with it.  This is a
   placement deviation from C's order, recorded in docs/ports/coding.md.  */
#[unsafe(no_mangle)]
pub static mut Venable_character_translation: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vlatin_extra_code_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vstandard_translation_table_for_decode: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vstandard_translation_table_for_encode: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut disable_ascii_optimization: bool = false;
#[unsafe(no_mangle)]
pub static mut inhibit_eol_conversion: bool = false;
#[unsafe(no_mangle)]
pub static mut inhibit_iso_escape_detection: bool = false;
#[unsafe(no_mangle)]
pub static mut inhibit_null_byte_detection: bool = false;

/* The remaining `DEFVAR_*` carriers this unit owns (C defines their storage in
   `syms_of_coding`, C 11738+; the port keeps them here with the eight above, the same
   placement deviation, and `syms_of_coding` only registers them).  */
#[unsafe(no_mangle)]
pub static mut Vcoding_system_list: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vcoding_system_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vcoding_category_list: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vcoding_system_for_read: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vcoding_system_for_write: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vlast_coding_system_used: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut inherit_process_coding_system: bool = false;
#[unsafe(no_mangle)]
pub static mut Vfile_coding_system_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vprocess_coding_system_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vnetwork_coding_system_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vlocale_coding_system: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut eol_mnemonic_unix: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut eol_mnemonic_dos: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut eol_mnemonic_mac: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut eol_mnemonic_undecided: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vcharset_revision_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vdefault_process_coding_system: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vselect_safe_coding_system_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut coding_system_require_warning: bool = false;
#[unsafe(no_mangle)]
pub static mut Vtranslation_table_for_input: Lisp_Object = lisp_h_Qnil ();

/* Coding-systems are handed between Emacs Lisp programs and C internal
   routines by the following three variables.  */

/* Two special coding systems.  */
static mut Vsjis_coding_system: Lisp_Object = lisp_h_Qnil ();
static mut Vbig5_coding_system: Lisp_Object = lisp_h_Qnil ();

/* ISO2022 section */

/* C's `CODING_ISO_INITIAL (coding, reg)`.  */
macro_rules! CODING_ISO_INITIAL {
  ($coding:expr, $reg:expr) => {
    XFIXNUM (AREF (AREF (CODING_ID_ATTRS ((*$coding).id as hash_idx_t),
                         coding_attr_iso_initial as isize), $reg as isize))
  };
}

/* C's `CODING_ISO_REQUEST (coding, charset_id)`.  */
macro_rules! CODING_ISO_REQUEST {
  ($coding:expr, $charset_id:expr) => {
    (if ($charset_id) <= (*$coding).max_charset_id
       { if *(*$coding).safe_charsets.offset ($charset_id as isize) != 255
           { *(*$coding).safe_charsets.offset ($charset_id as isize) as c_int }
           else { -1 } }
       else { -1 })
  };
}

macro_rules! CODING_ISO_FLAGS {
  ($coding:expr) => { (*$coding).spec.iso_2022.flags };
}
macro_rules! CODING_ISO_DESIGNATION {
  ($coding:expr, $reg:expr) => {
    (*$coding).spec.iso_2022.current_designation[$reg as usize]
  };
}
macro_rules! CODING_ISO_INVOCATION {
  ($coding:expr, $plane:expr) => {
    (*$coding).spec.iso_2022.current_invocation[$plane as usize]
  };
}

/* `single_shifting`, `bol` and `embedded_utf_8` are `bool_bf : 1` members that coding.h's
   mirror packs into `iso_2022_spec.bits` (coding.rs 66-75, PORTING.md 1.3); C reads and
   writes them as lvalues, so the three macros yield this proxy and the call sites read
   through `get` and write through `set` (the `/// C:` comment on each macro names C's
   own spelling; the field names are recorded in docs/ports/macro-body-notes.tsv).  */
pub const ISO_2022_SINGLE_SHIFTING_SHIFT: u32 = 0;   /* coding.h 334 */
pub const ISO_2022_BOL_SHIFT: u32 = 1;               /* coding.h 337 */
pub const ISO_2022_EMBEDDED_UTF_8_SHIFT: u32 = 2;    /* coding.h 340 */

#[derive(Clone, Copy)]
pub struct iso_2022_bit
{
  word: *mut u32,
  shift: u32,
}

impl iso_2022_bit
{
  #[inline(always)]
  pub fn get (self) -> c_int
  { ((unsafe { *self.word }) >> self.shift) as c_int & 1 }

  #[inline(always)]
  pub fn set (self, value: c_int)
  {
    if value != 0
      { unsafe { *self.word |= 1 << self.shift; } }
    else
      { unsafe { *self.word &= !(1 << self.shift); } }
  }
}

macro_rules! CODING_ISO_SINGLE_SHIFTING {
  ($coding:expr) => {
    /* C: `((coding)->spec.iso_2022.single_shifting)`  */
    iso_2022_bit { word: unsafe { std::ptr::addr_of_mut! ((*$coding).spec.iso_2022.bits) },
                   shift: ISO_2022_SINGLE_SHIFTING_SHIFT }
  };
}
macro_rules! CODING_ISO_BOL {
  ($coding:expr) => {
    /* C: `((coding)->spec.iso_2022.bol)`  */
    iso_2022_bit { word: unsafe { std::ptr::addr_of_mut! ((*$coding).spec.iso_2022.bits) },
                   shift: ISO_2022_BOL_SHIFT }
  };
}
macro_rules! CODING_ISO_EMBEDDED_UTF_8 {
  ($coding:expr) => {
    /* C: `((coding)->spec.iso_2022.embedded_utf_8)`  */
    iso_2022_bit { word: unsafe { std::ptr::addr_of_mut! ((*$coding).spec.iso_2022.bits) },
                   shift: ISO_2022_EMBEDDED_UTF_8_SHIFT }
  };
}

macro_rules! CODING_ISO_INVOKED_CHARSET {
  ($coding:expr, $plane:expr) => {
    (if CODING_ISO_INVOCATION! ($coding, $plane) < 0 { -1 }
       else { CODING_ISO_DESIGNATION! ($coding, CODING_ISO_INVOCATION! ($coding, $plane)) })
  };
}
macro_rules! CODING_ISO_CMP_STATUS {
  ($coding:expr) => {
    unsafe { std::ptr::addr_of_mut! ((*$coding).spec.iso_2022.cmp_status) }
  };
}
macro_rules! CODING_ISO_EXTSEGMENT_LEN {
  ($coding:expr) => { (*$coding).spec.iso_2022.ctext_extended_segment_len };
}

/* Control characters of ISO2022.  */
			/* code */	/* function */
pub const ISO_CODE_SO: c_int = 0x0E;		/* shift-out */
pub const ISO_CODE_SI: c_int = 0x0F;		/* shift-in */
pub const ISO_CODE_SS2_7: c_int = 0x19;		/* single-shift-2 for 7-bit code */
pub const ISO_CODE_ESC: c_int = 0x1B;		/* escape */
pub const ISO_CODE_SS2: c_int = 0x8E;		/* single-shift-2 */
pub const ISO_CODE_SS3: c_int = 0x8F;		/* single-shift-3 */
pub const ISO_CODE_CSI: c_int = 0x9B;		/* control-sequence-introducer */

/* All code (1-byte) of ISO2022 is classified into one of the
   followings.  */
pub type iso_code_class_type = c_int;
pub const ISO_control_0: iso_code_class_type = 0;		/* Control codes in the range
				   0x00..0x1F and 0x7F, except for the
				   following 5 codes.  */
pub const ISO_shift_out: iso_code_class_type = 1;		/* ISO_CODE_SO (0x0E) */
pub const ISO_shift_in: iso_code_class_type = 2;		/* ISO_CODE_SI (0x0F) */
pub const ISO_single_shift_2_7: iso_code_class_type = 3;	/* ISO_CODE_SS2_7 (0x19) */
pub const ISO_escape: iso_code_class_type = 4;			/* ISO_CODE_ESC (0x1B) */
pub const ISO_control_1: iso_code_class_type = 5;		/* Control codes in the range
				   0x80..0x9F, except for the
				   following 3 codes.  */
pub const ISO_single_shift_2: iso_code_class_type = 6;		/* ISO_CODE_SS2 (0x8E) */
pub const ISO_single_shift_3: iso_code_class_type = 7;		/* ISO_CODE_SS3 (0x8F) */
pub const ISO_control_sequence_introducer: iso_code_class_type = 8; /* ISO_CODE_CSI (0x9B) */
pub const ISO_0x20_or_0x7F: iso_code_class_type = 9;		/* Codes of the values 0x20 or 0x7F.  */
pub const ISO_graphic_plane_0: iso_code_class_type = 10;	/* Graphic codes in the range 0x21..0x7E.  */
pub const ISO_0xA0_or_0xFF: iso_code_class_type = 11;		/* Codes of the values 0xA0 or 0xFF.  */
pub const ISO_graphic_plane_1: iso_code_class_type = 12;	/* Graphic codes in the range 0xA1..0xFE.  */

/** The macros CODING_ISO_FLAG_XXX as c_uint defines a flag bit of the
    `iso-flags' attribute of an iso2022 coding system.  */

/* If set, produce long-form designation sequence (e.g. ESC $ ( A)
   instead of the correct short-form sequence (e.g. ESC $ A).  */
pub const CODING_ISO_FLAG_LONG_FORM: c_int = 0x0001;

/* If set, reset graphic planes and registers at end-of-line to the
   initial state.  */
pub const CODING_ISO_FLAG_RESET_AT_EOL: c_int = 0x0002;

/* If set, reset graphic planes and registers before any control
   characters to the initial state.  */
pub const CODING_ISO_FLAG_RESET_AT_CNTL: c_int = 0x0004;

/* If set, encode by 7-bit environment.  */
pub const CODING_ISO_FLAG_SEVEN_BITS: c_int = 0x0008;

/* If set, use locking-shift function.  */
pub const CODING_ISO_FLAG_LOCKING_SHIFT: c_int = 0x0010;

/* If set, use single-shift function.  Overwrite
   CODING_ISO_FLAG_LOCKING_SHIFT as c_uint.  */
pub const CODING_ISO_FLAG_SINGLE_SHIFT: c_int = 0x0020;

/* If set, use designation escape sequence.  */
pub const CODING_ISO_FLAG_DESIGNATION: c_int = 0x0040;

/* If set, produce revision number sequence.  */
pub const CODING_ISO_FLAG_REVISION: c_int = 0x0080;

/* If set, produce ISO6429's direction specifying sequence.  */
pub const CODING_ISO_FLAG_DIRECTION: c_int = 0x0100;

/* If set, assume designation states are reset at beginning of line on
   output.  */
pub const CODING_ISO_FLAG_INIT_AT_BOL: c_int = 0x0200;

/* If set, designation sequence should be placed at beginning of line
   on output.  */
pub const CODING_ISO_FLAG_DESIGNATE_AT_BOL: c_int = 0x0400;

/* If set, do not encode unsafe characters on output.  */
pub const CODING_ISO_FLAG_SAFE: c_int = 0x0800;

/* If set, extra latin codes (128..159) are accepted as a valid code
   on input.  */
pub const CODING_ISO_FLAG_LATIN_EXTRA: c_int = 0x1000;

pub const CODING_ISO_FLAG_COMPOSITION: c_int = 0x2000;

/* #define CODING_ISO_FLAG_EUC_TW_SHIFT as c_uint	0x4000 */

pub const CODING_ISO_FLAG_USE_ROMAN: c_int = 0x8000;

pub const CODING_ISO_FLAG_USE_OLDJIS: c_int = 0x10000;

pub const CODING_ISO_FLAG_LEVEL_4: c_int = 0x20000;

pub const CODING_ISO_FLAG_FULL_SUPPORT: c_int = 0x100000;

/* A character to be produced on output if encoding of the original
   character is prohibited by CODING_ISO_FLAG_SAFE as c_uint.  */
/* C spells the macro `'?'`; a `char` const keeps that spelling and the two uses
   (`decode_coding_sjis`/`encode_coding_charset`) cast it back to `c_int`.  */
pub const CODING_INHIBIT_CHARACTER_SUBSTITUTION: char = '?';

/* UTF-8 section */
macro_rules! CODING_UTF_8_BOM {
  ($coding:expr) => { (*$coding).spec.utf_8_bom };
}

/* UTF-16 section */
macro_rules! CODING_UTF_16_BOM {
  ($coding:expr) => { (*$coding).spec.utf_16.bom };
}

macro_rules! CODING_UTF_16_ENDIAN {
  ($coding:expr) => { (*$coding).spec.utf_16.endian };
}

macro_rules! CODING_UTF_16_SURROGATE {
  ($coding:expr) => { (*$coding).spec.utf_16.surrogate };
}


/* CCL section */
macro_rules! CODING_CCL_DECODER {
  ($coding:expr) => {
    AREF (CODING_ID_ATTRS ((*$coding).id as hash_idx_t), coding_attr_ccl_decoder as isize)
  };
}
macro_rules! CODING_CCL_ENCODER {
  ($coding:expr) => {
    AREF (CODING_ID_ATTRS ((*$coding).id as hash_idx_t), coding_attr_ccl_encoder as isize)
  };
}
macro_rules! CODING_CCL_VALIDS {
  ($coding:expr) => {
    SDATA (AREF (CODING_ID_ATTRS ((*$coding).id as hash_idx_t), coding_attr_ccl_valids as isize))
  };
}

/* Index for each coding category in `coding_categories' */

pub type coding_category = c_int;
pub const coding_category_iso_7: coding_category = 0;
pub const coding_category_iso_7_tight: coding_category = 1;
pub const coding_category_iso_8_1: coding_category = 2;
pub const coding_category_iso_8_2: coding_category = 3;
pub const coding_category_iso_7_else: coding_category = 4;
pub const coding_category_iso_8_else: coding_category = 5;
pub const coding_category_utf_8_auto: coding_category = 6;
pub const coding_category_utf_8_nosig: coding_category = 7;
pub const coding_category_utf_8_sig: coding_category = 8;
pub const coding_category_utf_16_auto: coding_category = 9;
pub const coding_category_utf_16_be: coding_category = 10;
pub const coding_category_utf_16_le: coding_category = 11;
pub const coding_category_utf_16_be_nosig: coding_category = 12;
pub const coding_category_utf_16_le_nosig: coding_category = 13;
pub const coding_category_charset: coding_category = 14;
pub const coding_category_sjis: coding_category = 15;
pub const coding_category_big5: coding_category = 16;
pub const coding_category_ccl: coding_category = 17;
pub const coding_category_emacs_mule: coding_category = 18;
    /* All above are targets of code detection.  */
pub const coding_category_raw_text: coding_category = 19;
pub const coding_category_undecided: coding_category = 20;
pub const coding_category_max: coding_category = 21;

/* Definitions of flag bits used in detect_coding_XXXX.  */
pub const CATEGORY_MASK_ISO_7: c_int = 1 << coding_category_iso_7;
pub const CATEGORY_MASK_ISO_7_TIGHT: c_int = 1 << coding_category_iso_7_tight;
pub const CATEGORY_MASK_ISO_8_1: c_int = 1 << coding_category_iso_8_1;
pub const CATEGORY_MASK_ISO_8_2: c_int = 1 << coding_category_iso_8_2;
pub const CATEGORY_MASK_ISO_7_ELSE: c_int = 1 << coding_category_iso_7_else;
pub const CATEGORY_MASK_ISO_8_ELSE: c_int = 1 << coding_category_iso_8_else;
pub const CATEGORY_MASK_UTF_8_AUTO: c_int = 1 << coding_category_utf_8_auto;
pub const CATEGORY_MASK_UTF_8_NOSIG: c_int = 1 << coding_category_utf_8_nosig;
pub const CATEGORY_MASK_UTF_8_SIG: c_int = 1 << coding_category_utf_8_sig;
pub const CATEGORY_MASK_UTF_16_AUTO: c_int = 1 << coding_category_utf_16_auto;
pub const CATEGORY_MASK_UTF_16_BE: c_int = 1 << coding_category_utf_16_be;
pub const CATEGORY_MASK_UTF_16_LE: c_int = 1 << coding_category_utf_16_le;
pub const CATEGORY_MASK_UTF_16_BE_NOSIG: c_int = 1 << coding_category_utf_16_be_nosig;
pub const CATEGORY_MASK_UTF_16_LE_NOSIG: c_int = 1 << coding_category_utf_16_le_nosig;
pub const CATEGORY_MASK_CHARSET: c_int = 1 << coding_category_charset;
pub const CATEGORY_MASK_SJIS: c_int = 1 << coding_category_sjis;
pub const CATEGORY_MASK_BIG5: c_int = 1 << coding_category_big5;
pub const CATEGORY_MASK_CCL: c_int = 1 << coding_category_ccl;
pub const CATEGORY_MASK_EMACS_MULE: c_int = 1 << coding_category_emacs_mule;
pub const CATEGORY_MASK_RAW_TEXT: c_int = 1 << coding_category_raw_text;

/* This value is returned if detect_coding_mask () find nothing other
   than ASCII characters.  */
pub const CATEGORY_MASK_ANY: c_int =
  (CATEGORY_MASK_ISO_7
   | CATEGORY_MASK_ISO_7_TIGHT
   | CATEGORY_MASK_ISO_8_1
   | CATEGORY_MASK_ISO_8_2
   | CATEGORY_MASK_ISO_7_ELSE
   | CATEGORY_MASK_ISO_8_ELSE
   | CATEGORY_MASK_UTF_8_AUTO
   | CATEGORY_MASK_UTF_8_NOSIG
   | CATEGORY_MASK_UTF_8_SIG
   | CATEGORY_MASK_UTF_16_AUTO
   | CATEGORY_MASK_UTF_16_BE
   | CATEGORY_MASK_UTF_16_LE
   | CATEGORY_MASK_UTF_16_BE_NOSIG
   | CATEGORY_MASK_UTF_16_LE_NOSIG
   | CATEGORY_MASK_CHARSET
   | CATEGORY_MASK_SJIS
   | CATEGORY_MASK_BIG5
   | CATEGORY_MASK_CCL
   | CATEGORY_MASK_EMACS_MULE);


pub const CATEGORY_MASK_ISO_7BIT: c_int =
  (CATEGORY_MASK_ISO_7 | CATEGORY_MASK_ISO_7_TIGHT);

pub const CATEGORY_MASK_ISO_8BIT: c_int =
  (CATEGORY_MASK_ISO_8_1 | CATEGORY_MASK_ISO_8_2);

pub const CATEGORY_MASK_ISO_ELSE: c_int =
  (CATEGORY_MASK_ISO_7_ELSE | CATEGORY_MASK_ISO_8_ELSE);

pub const CATEGORY_MASK_ISO_ESCAPE: c_int =
  (CATEGORY_MASK_ISO_7
   | CATEGORY_MASK_ISO_7_TIGHT
   | CATEGORY_MASK_ISO_7_ELSE
   | CATEGORY_MASK_ISO_8_ELSE);

pub const CATEGORY_MASK_ISO: c_int =
  (  CATEGORY_MASK_ISO_7BIT
     | CATEGORY_MASK_ISO_8BIT
     | CATEGORY_MASK_ISO_ELSE);

pub const CATEGORY_MASK_UTF_16: c_int =
  (CATEGORY_MASK_UTF_16_AUTO
   | CATEGORY_MASK_UTF_16_BE
   | CATEGORY_MASK_UTF_16_LE
   | CATEGORY_MASK_UTF_16_BE_NOSIG
   | CATEGORY_MASK_UTF_16_LE_NOSIG);

pub const CATEGORY_MASK_UTF_8: c_int =
  (CATEGORY_MASK_UTF_8_AUTO
   | CATEGORY_MASK_UTF_8_NOSIG
   | CATEGORY_MASK_UTF_8_SIG);

/* Table of coding categories (Lisp symbols).  This variable is for
   internal use only.  */
static mut Vcoding_category_table: Lisp_Object = lisp_h_Qnil ();

/* Table of coding-categories ordered by priority.  */
static mut coding_priorities: [coding_category; coding_category_max as usize]
  = [0; coding_category_max as usize];

/* Nth element is a coding context for the coding system bound to the
   Nth coding category.  */
static mut coding_categories: [coding_system; coding_category_max as usize]
  = unsafe { std::mem::zeroed () };

/* Encode a flag that can be nil, something else, or t as -1, 0, 1.  */

unsafe fn encode_inhibit_flag (flag: Lisp_Object) -> c_int
{
  return if NILP (flag) { -1 } else { EQ (flag, Qt ()) as c_int };
}

/* True if the value of ENCODED_FLAG says a flag should be treated as set.
   1 means yes, -1 means no, 0 means ask the user variable VAR.  */

unsafe fn inhibit_flag (encoded_flag: c_int, var: bool) -> bool
{
  return 0 < encoded_flag + var as c_int;
}

macro_rules! CODING_GET_INFO {
  ($coding:expr, $attrs:expr, $charset_list:expr) => {
    {
      ($attrs) = CODING_ID_ATTRS ((*$coding).id as hash_idx_t);
      ($charset_list) = CODING_ATTR_CHARSET_LIST ($attrs);
    }
  };
}

/* True if CODING's destination can be grown.  */

unsafe fn growable_destination (coding: *mut coding_system) -> bool
{
  return (STRINGP ((*coding).dst_object)
	  || BUFFERP ((*coding).dst_object)
	  || NILP ((*coding).dst_object));
}

unsafe fn record_conversion_result (coding: *mut coding_system,
                                           result: coding_result_code)
{
  coding_bits_set (coding, CODING_SYSTEM_RESULT_SHIFT, CODING_SYSTEM_RESULT_MASK, result as u32);
  match result
    {
    CODING_RESULT_INSUFFICIENT_SRC =>
      Vlast_code_conversion_error = Qinsufficient_source (),
    CODING_RESULT_INVALID_SRC =>
      Vlast_code_conversion_error = Qinvalid_source (),
    CODING_RESULT_INTERRUPT =>
      Vlast_code_conversion_error = Qinterrupted (),
    CODING_RESULT_INSUFFICIENT_DST =>
      /* Don't record this error in Vlast_code_conversion_error
	 because it happens just temporarily and is resolved when the
	 whole conversion is finished.  */
      (),
    CODING_RESULT_SUCCESS =>
      (),
    _ =>
      Vlast_code_conversion_error = QUnknown_error (),
    }
}

/* These wrapper macros are used to preserve validity of pointers into
   buffer text across calls to decode_char, encode_char, etc, which
   could cause relocation of buffers if it loads a charset map,
   because loading a charset map allocates large structures.  */

/* Store multibyte form of the character C in P, and advance P to the
   end of the multibyte form.  This used to be like adding CHAR_STRING
   without ever calling MAYBE_UNIFY_CHAR, but nowadays we don't call
   MAYBE_UNIFY_CHAR in CHAR_STRING.  */

macro_rules! CHAR_STRING_ADVANCE_NO_UNIFY {
  ($c:expr, $p:expr) => { ($p) = ($p).offset (CHAR_STRING ($c, $p) as isize) };
}

/* Return the character code of character whose multibyte form is at
   P, and advance P to the end of the multibyte form.  This used to be
   like string_char_advance without ever calling MAYBE_UNIFY_CHAR, but
   nowadays string_char_advance doesn't call MAYBE_UNIFY_CHAR.  */

macro_rules! STRING_CHAR_ADVANCE_NO_UNIFY {
  ($p:expr) => { string_char_advance (&raw mut $p) };
}

/* Set coding->source from coding->src_object.  */

unsafe fn coding_set_source (coding: *mut coding_system)
{
  if BUFFERP ((*coding).src_object)
    {
      let buf: *mut buffer = XBUFFER ((*coding).src_object);

      if (*coding).src_pos < 0
	{ (*coding).source = BUF_GAP_END_ADDR (buf).offset ((*coding).src_pos_byte); }
      else
	{ (*coding).source = BUF_BYTE_ADDRESS (buf, (*coding).src_pos_byte); }
    }
  else if STRINGP ((*coding).src_object)
    {
      (*coding).source = SDATA ((*coding).src_object).offset ((*coding).src_pos_byte);
    }
  else
    {
      /* Otherwise, the source is C string and is never relocated
	 automatically.  Thus we don't have to update anything.  */
    }
}


/* Set coding->source from coding->src_object, and return how many
   bytes coding->source was changed.  */

unsafe fn coding_change_source (coding: *mut coding_system) -> ptrdiff_t
{
  let orig: *const c_uchar = (*coding).source;
  coding_set_source (coding);
  return (*coding).source.offset_from (orig);
}


/* Set coding->destination from coding->dst_object.  */

unsafe fn coding_set_destination (coding: *mut coding_system)
{
  if BUFFERP ((*coding).dst_object)
    {
      if BUFFERP ((*coding).src_object) && (*coding).src_pos < 0
	{
	  (*coding).destination = BEG_ADDR! ().offset ((*coding).dst_pos_byte - BEG_BYTE as isize);
	  (*coding).dst_bytes = (GAP_END_ADDR! ()
			       .offset (- ((*coding).src_bytes - (*coding).consumed))
			       .offset_from ((*coding).destination));
	}
      else
	{
	  /* We are sure that coding->dst_pos_byte is before the gap
	     of the buffer. */
	  (*coding).destination = (BUF_BEG_ADDR! (XBUFFER ((*coding).dst_object))
				   .offset ((*coding).dst_pos_byte - BEG_BYTE as isize));
	  (*coding).dst_bytes = (BUF_GAP_END_ADDR (XBUFFER ((*coding).dst_object))
			       .offset_from ((*coding).destination));
	}
    }
  else
    {
      /* Otherwise, the destination is C string and is never relocated
	 automatically.  Thus we don't have to update anything.  */
    }
}


/* Set coding->destination from coding->dst_object, and return how
   many bytes coding->destination was changed.  */

unsafe fn coding_change_destination (coding: *mut coding_system) -> ptrdiff_t
{
  let orig: *const c_uchar = (*coding).destination;
  coding_set_destination (coding);
  return (*coding).destination.offset_from (orig as *mut c_uchar);
}


unsafe fn coding_alloc_by_realloc (coding: *mut coding_system, bytes: ptrdiff_t)
{
  let mut newbytes: ptrdiff_t = 0;
  if ckd_add! (newbytes, (*coding).dst_bytes, bytes)
    || SIZE_MAX < newbytes as usize
    { string_overflow (); }
  (*coding).destination = xrealloc ((*coding).destination as *mut c_void, newbytes as usize) as *mut u8;
  (*coding).dst_bytes = newbytes;
}

unsafe fn coding_alloc_by_making_gap (coding: *mut coding_system,
                                             gap_head_used: ptrdiff_t, bytes: ptrdiff_t)
{
  if EQ ((*coding).src_object, (*coding).dst_object)
    {
      /* The gap may contain the produced data at the head and not-yet
	 consumed data at the tail.  To preserve those data, we at
	 first make the gap size to zero, then increase the gap
	 size.  */
      let add: ptrdiff_t = GAP_SIZE! ();

      GPT! () += gap_head_used; GPT_BYTE! () += gap_head_used;
      GAP_SIZE! () = 0; ZV! () += add; Z! () += add; ZV_BYTE! () += add; Z_BYTE! () += add;
      make_gap (bytes);
      GAP_SIZE! () += add; ZV! () -= add; Z! () -= add; ZV_BYTE! () -= add; Z_BYTE! () -= add;
      GPT! () -= gap_head_used; GPT_BYTE! () -= gap_head_used;
    }
  else
    { make_gap_1 (XBUFFER ((*coding).dst_object), bytes); }
}


unsafe fn alloc_destination (coding: *mut coding_system, nbytes: ptrdiff_t,
                                    dst: *mut c_uchar) -> *mut c_uchar
{
  let mut offset: ptrdiff_t = dst.offset_from ((*coding).destination);

  if BUFFERP ((*coding).dst_object)
    {
      let buf: *mut buffer = XBUFFER ((*coding).dst_object);

      coding_alloc_by_making_gap (coding, dst.offset_from (BUF_GPT_ADDR (buf)), nbytes);
    }
  else
    { coding_alloc_by_realloc (coding, nbytes); }
  coding_set_destination (coding);
  let mut dst: *mut c_uchar = (*coding).destination.offset (offset);
  return dst;
}


/*** 2. Emacs's internal format (emacs-utf-8) ***/



/*** 3. UTF-8 ***/

/* See the above "GENERAL NOTES on `detect_coding_XXX ()' functions".
   Return true if a text is encoded in UTF-8.  */

macro_rules! UTF_8_1_OCTET_P {
  ($c:expr) => { (($c) < 0x80) };
}
macro_rules! UTF_8_EXTRA_OCTET_P {
  ($c:expr) => { ((($c) & 0xC0) == 0x80) };
}
macro_rules! UTF_8_2_OCTET_LEADING_P {
  ($c:expr) => { ((($c) & 0xE0) == 0xC0) };
}
macro_rules! UTF_8_3_OCTET_LEADING_P {
  ($c:expr) => { ((($c) & 0xF0) == 0xE0) };
}
macro_rules! UTF_8_4_OCTET_LEADING_P {
  ($c:expr) => { ((($c) & 0xF8) == 0xF0) };
}
macro_rules! UTF_8_5_OCTET_LEADING_P {
  ($c:expr) => { ((($c) & 0xFC) == 0xF8) };
}

pub const UTF_8_BOM_1: c_int = 0xEF;
pub const UTF_8_BOM_2: c_int = 0xBB;
pub const UTF_8_BOM_3: c_int = 0xBF;

/* Unlike the other detect_coding_XXX, this function counts the number
   of characters and checks the EOL format.  */

unsafe extern "C" fn detect_coding_utf_8 (coding: *mut coding_system,
                               detect_info: *mut coding_detection_info) -> bool
{
  let mut src: *const c_uchar = (*coding).source;
  let mut src_base: *const c_uchar = std::ptr::null ();
  let src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let multibytep = src_multibyte_p (coding);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut bom_found: bool = false;
  let mut nchars: ptrdiff_t = (*coding).head_ascii;

  (*detect_info).checked |= CATEGORY_MASK_UTF_8;
  /* A coding system of this category is always ASCII compatible.  */
  src = src.offset (nchars);

  if src == (*coding).source	/* BOM should be at the head.  */
      && src.offset (3) < src_end	/* BOM is 3-byte long.  */
      && *src.offset (0) as c_int == UTF_8_BOM_1
      && *src.offset (1) as c_int == UTF_8_BOM_2
      && *src.offset (2) as c_int == UTF_8_BOM_3
    {
      bom_found = true;
      src = src.offset (3);
      nchars += 1;
    }
  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }

    while true
      {
	let mut c: c_int;
	let mut c1: c_int; let mut c2: c_int; let mut c3: c_int; let mut c4: c_int;

	src_base = src;
	ONE_MORE_BYTE! (c);
	if c < 0 || UTF_8_1_OCTET_P! (c)
	  {
	    nchars += 1;
	    if c == '\r' as c_int
	      {
		if src < src_end && *src as c_int == '\n' as c_int
		  {
		    src = src.offset (1);
		    nchars += 1;
		  }
	      }
	    continue;
	  }
	ONE_MORE_BYTE! (c1);
	if c1 < 0 || ! UTF_8_EXTRA_OCTET_P! (c1)
	  { break; }
	if UTF_8_2_OCTET_LEADING_P! (c)
	  {
	    nchars += 1;
	    continue;
	  }
	ONE_MORE_BYTE! (c2);
	if c2 < 0 || ! UTF_8_EXTRA_OCTET_P! (c2)
	  { break; }
	if UTF_8_3_OCTET_LEADING_P! (c)
	  {
	    nchars += 1;
	    continue;
	  }
	ONE_MORE_BYTE! (c3);
	if c3 < 0 || ! UTF_8_EXTRA_OCTET_P! (c3)
	  { break; }
	if UTF_8_4_OCTET_LEADING_P! (c)
	  {
	    nchars += 1;
	    continue;
	  }
	ONE_MORE_BYTE! (c4);
	if c4 < 0 || ! UTF_8_EXTRA_OCTET_P! (c4)
	  { break; }
	if UTF_8_5_OCTET_LEADING_P! (c)
	    /* If we ever need to increase MAX_CHAR, the below may need
	       to be reviewed.  */
	    && c < MAX_MULTIBYTE_LEADING_CODE
	  {
	    nchars += 1;
	    continue;
	  }
	break;
      }
    (*detect_info).rejected |= CATEGORY_MASK_UTF_8;
    return false;
  }

  if src_base < src && mode_of (coding) as c_int & CODING_MODE_LAST_BLOCK != 0
    {
      (*detect_info).rejected |= CATEGORY_MASK_UTF_8;
      return false;
    }
  if bom_found
    {
      /* The first character 0xFFFE doesn't necessarily mean a BOM.  */
      (*detect_info).found |= CATEGORY_MASK_UTF_8_AUTO | CATEGORY_MASK_UTF_8_SIG | CATEGORY_MASK_UTF_8_NOSIG;
    }
  else
    {
      (*detect_info).rejected |= CATEGORY_MASK_UTF_8_SIG;
      if nchars < src_end.offset_from ((*coding).source)
	/* The found characters are less than source bytes, which
	   means that we found a valid non-ASCII characters.  */
	{ (*detect_info).found |= CATEGORY_MASK_UTF_8_AUTO | CATEGORY_MASK_UTF_8_NOSIG; }
    }
  (*coding).detected_utf8_bytes = src_base.offset_from ((*coding).source);
  (*coding).detected_utf8_chars = nchars;
  return true;
}


unsafe extern "C" fn decode_coding_utf_8 (coding: *mut coding_system)
{
  let mut src: *const c_uchar = (*coding).source.offset ((*coding).consumed);
  let src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let mut src_base: *const c_uchar = std::ptr::null ();
  let mut charbuf: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_used as isize);
  let charbuf_end: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_size as isize);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut consumed_chars_base: ptrdiff_t = 0;
  let multibytep = src_multibyte_p (coding);
  let bom: utf_bom_type = CODING_UTF_8_BOM! (coding);
  let eol_dos
    = !inhibit_eol_conversion && EQ (CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t), Qdos ());
  let mut byte_after_cr: c_int = -1;

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }

    if bom != utf_without_bom
      {
	let mut c1: c_int; let mut c2: c_int; let mut c3: c_int;

	src_base = src;
	ONE_MORE_BYTE! (c1);
	if ! UTF_8_3_OCTET_LEADING_P! (c1)
	  { src = src_base; }
	else
	  {
	    ONE_MORE_BYTE! (c2);
	    if ! UTF_8_EXTRA_OCTET_P! (c2)
	      { src = src_base; }
	    else
	      {
		ONE_MORE_BYTE! (c3);
		if ! UTF_8_EXTRA_OCTET_P! (c3)
		  { src = src_base; }
		else
		  {
		    if (c1 != UTF_8_BOM_1)
			|| (c2 != UTF_8_BOM_2) || (c3 != UTF_8_BOM_3)
		      { src = src_base; }
		    else
		      { CODING_UTF_8_BOM! (coding) = utf_without_bom; }
		  }
	      }
	  }
      }
    CODING_UTF_8_BOM! (coding) = utf_without_bom;

    'decode_loop: loop
      {
	let mut c: c_int;
	let mut c1: c_int; let mut c2: c_int; let mut c3: c_int; let mut c4: c_int;
	let mut c5: c_int;

	src_base = src;
	consumed_chars_base = consumed_chars;

	if charbuf >= charbuf_end
	  {
	    if byte_after_cr >= 0
	      { src_base = src_base.offset (-1); }
	    break;
	  }

	/* In the simple case, rapidly handle ordinary characters */
	if multibytep && ! eol_dos
	    && charbuf < charbuf_end.offset (-6) && src < src_end.offset (-6)
	  {
	    while charbuf < charbuf_end.offset (-6) && src < src_end.offset (-6)
	      {
		c1 = *src as c_int;
		if c1 & 0x80 != 0
		  { break; }
		src = src.offset (1);
		consumed_chars += 1;
		*charbuf = c1; charbuf = charbuf.offset (1);

		c1 = *src as c_int;
		if c1 & 0x80 != 0
		  { break; }
		src = src.offset (1);
		consumed_chars += 1;
		*charbuf = c1; charbuf = charbuf.offset (1);

		c1 = *src as c_int;
		if c1 & 0x80 != 0
		  { break; }
		src = src.offset (1);
		consumed_chars += 1;
		*charbuf = c1; charbuf = charbuf.offset (1);

		c1 = *src as c_int;
		if c1 & 0x80 != 0
		  { break; }
		src = src.offset (1);
		consumed_chars += 1;
		*charbuf = c1; charbuf = charbuf.offset (1);
	      }
	    /* If we handled at least one character, restart the main loop.  */
	    if src != src_base
	      { continue; }
	  }

	'invalid_code: {
	  if byte_after_cr >= 0
	    { c1 = byte_after_cr; byte_after_cr = -1; }
	  else
	    { ONE_MORE_BYTE! (c1); }
	  if c1 < 0
	    {
	      c = - c1;
	    }
	  else if UTF_8_1_OCTET_P! (c1)
	    {
	      if eol_dos && c1 == '\r' as c_int
		{ ONE_MORE_BYTE! (byte_after_cr); }
	      c = c1;
	    }
	  else
	    {
	      ONE_MORE_BYTE! (c2);
	      if c2 < 0 || ! UTF_8_EXTRA_OCTET_P! (c2)
		{ break 'invalid_code; }
	      if UTF_8_2_OCTET_LEADING_P! (c1)
		{
		  c = ((c1 & 0x1F) << 6) | (c2 & 0x3F);
		  /* Reject overlong sequences here and below.  Encoders
		     producing them are incorrect, they can be misleading,
		     and they mess up read/write invariance.  */
		  if c < 128
		    { break 'invalid_code; }
		}
	      else
		{
		  ONE_MORE_BYTE! (c3);
		  if c3 < 0 || ! UTF_8_EXTRA_OCTET_P! (c3)
		    { break 'invalid_code; }
		  if UTF_8_3_OCTET_LEADING_P! (c1)
		    {
		      c = (((c1 & 0xF) << 12)
			   | ((c2 & 0x3F) << 6) | (c3 & 0x3F));
		      if c < 0x800
			  || (c >= 0xd800 && c < 0xe000) /* surrogates (invalid) */
			{ break 'invalid_code; }
		    }
		  else
		    {
		      ONE_MORE_BYTE! (c4);
		      if c4 < 0 || ! UTF_8_EXTRA_OCTET_P! (c4)
			{ break 'invalid_code; }
		      if UTF_8_4_OCTET_LEADING_P! (c1)
			{
			c = (((c1 & 0x7) << 18) | ((c2 & 0x3F) << 12)
			     | ((c3 & 0x3F) << 6) | (c4 & 0x3F));
			if c < 0x10000
			  { break 'invalid_code; }
			}
		      else
			{
			  ONE_MORE_BYTE! (c5);
			  if c5 < 0 || ! UTF_8_EXTRA_OCTET_P! (c5)
			    { break 'invalid_code; }
			  if UTF_8_5_OCTET_LEADING_P! (c1)
			    {
			      c = (((c1 & 0x3) << 24) | ((c2 & 0x3F) << 18)
				   | ((c3 & 0x3F) << 12) | ((c4 & 0x3F) << 6)
				   | (c5 & 0x3F));
			      if (c > MAX_CHAR) || (c < 0x200000)
				{ break 'invalid_code; }
			    }
			  else
			    { break 'invalid_code; }
			}
		    }
		}
	    }

	  *charbuf = c; charbuf = charbuf.offset (1);
	  continue 'decode_loop;
	}

	/* invalid_code: */
	src = src_base;
	consumed_chars = consumed_chars_base;
	ONE_MORE_BYTE! (c);
	*charbuf = if ASCII_CHAR_P (c as isize) { c } else { BYTE8_TO_CHAR (c) }; charbuf = charbuf.offset (1);
      }
  }

  (*coding).consumed_char += consumed_chars_base;
  (*coding).consumed = src_base.offset_from ((*coding).source);
  (*coding).charbuf_used = charbuf.offset_from ((*coding).charbuf) as c_int;
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn encode_coding_utf_8 (coding: *mut coding_system) -> bool
{
  let multibytep = dst_multibyte_p (coding);
  let mut charbuf: *mut c_int = (*coding).charbuf;
  let charbuf_end: *mut c_int = charbuf.offset ((*coding).charbuf_used as isize);
  let mut dst: *mut c_uchar = (*coding).destination.offset ((*coding).produced);
  let mut dst_end: *mut c_uchar = (*coding).destination.offset ((*coding).dst_bytes);
  let mut produced_chars: ptrdiff_t = 0;
  let mut c: c_int;

  macro_rules! ASSURE_DESTINATION {
    ($bytes:expr) => {
      {
	if dst.offset (($bytes) as isize) >= dst_end
	  {
	    let more_bytes: ptrdiff_t = charbuf_end.offset_from (charbuf) + ($bytes) as isize;

	    dst = alloc_destination (coding, more_bytes, dst);
	    dst_end = (*coding).destination.offset ((*coding).dst_bytes);
	  }
      }
    };
  }
  macro_rules! EMIT_ONE_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	if multibytep
	  {
	    let mut ch: c_uint = ($c) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  { *dst = ($c) as c_uchar; dst = dst.offset (1); }
      }
    };
  }
  macro_rules! EMIT_TWO_BYTES {
    ($c1:expr, $c2:expr) => {
      {
	produced_chars += 2;
	if multibytep
	  {
	    let mut ch: c_uint;

	    ch = ($c1) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	    ch = ($c2) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  {
	    *dst = ($c1) as c_uchar; dst = dst.offset (1);
	    *dst = ($c2) as c_uchar; dst = dst.offset (1);
	  }
      }
    };
  }
  macro_rules! EMIT_THREE_BYTES {
    ($c1:expr, $c2:expr, $c3:expr) => {
      {
	EMIT_ONE_BYTE! ($c1);
	EMIT_TWO_BYTES! ($c2, $c3);
      }
    };
  }
  macro_rules! EMIT_FOUR_BYTES {
    ($c1:expr, $c2:expr, $c3:expr, $c4:expr) => {
      {
	EMIT_TWO_BYTES! ($c1, $c2);
	EMIT_TWO_BYTES! ($c3, $c4);
      }
    };
  }

  if CODING_UTF_8_BOM! (coding) != utf_without_bom
    {
      ASSURE_DESTINATION! (3);
      EMIT_THREE_BYTES! (UTF_8_BOM_1, UTF_8_BOM_2, UTF_8_BOM_3);
      CODING_UTF_8_BOM! (coding) = utf_without_bom;
    }

  if multibytep
    {
      let safe_room: c_int = MAX_MULTIBYTE_LENGTH * 2;

      while charbuf < charbuf_end
	{
	  let mut str: [c_uchar; MAX_MULTIBYTE_LENGTH as usize]
	    = [0; MAX_MULTIBYTE_LENGTH as usize];
	  let mut pend: *mut c_uchar = str.as_mut_ptr ();
	  let mut p: *mut c_uchar;

	  ASSURE_DESTINATION! (safe_room);
	  c = *charbuf; charbuf = charbuf.offset (1);
	  if CHAR_BYTE8_P (c)
	    {
	      c = CHAR_TO_BYTE8 (c);
	      EMIT_ONE_BYTE! (c);
	    }
	  else
	    {
	      CHAR_STRING_ADVANCE_NO_UNIFY! (c, pend);
	      p = str.as_mut_ptr ();
	      while p < pend
		{
		  EMIT_ONE_BYTE! (*p as c_int);
		  p = p.offset (1);
		}
	    }
	}
    }
  else
    {
      let safe_room: c_int = MAX_MULTIBYTE_LENGTH;

      while charbuf < charbuf_end
	{
	  ASSURE_DESTINATION! (safe_room);
	  c = *charbuf; charbuf = charbuf.offset (1);
	  if CHAR_BYTE8_P (c)
	    { *dst = CHAR_TO_BYTE8 (c) as c_uchar; dst = dst.offset (1); }
	  else
	    { CHAR_STRING_ADVANCE_NO_UNIFY! (c, dst); }
	}
      produced_chars = dst.offset_from ((*coding).destination) - (*coding).produced;
    }
  record_conversion_result (coding, CODING_RESULT_SUCCESS);
  (*coding).produced_char += produced_chars;
  (*coding).produced = dst.offset_from ((*coding).destination);
  return false;
}


/* See the above "GENERAL NOTES on `detect_coding_XXX ()' functions".
   Return true if a text is encoded in one of UTF-16 based coding systems.  */

unsafe extern "C" fn detect_coding_utf_16 (coding: *mut coding_system,
                                detect_info: *mut coding_detection_info) -> bool
{
  let mut src: *const c_uchar = (*coding).source;
  let src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let multibytep = src_multibyte_p (coding);
  let mut c1: c_int;
  let mut c2: c_int;

  (*detect_info).checked |= CATEGORY_MASK_UTF_16;

  'no_more_source: {
    macro_rules! TWO_MORE_BYTES {
      ($c1:ident, $c2:ident) => {
	{
	  loop
	    {
	      if src == src_end
		{ break 'no_more_source; }
	      $c1 = *src as c_int;
	      src = src.offset (1);
	      if multibytep && ($c1 & 0x80) != 0
		{
		  if ($c1 & 0xFE) == 0xC0
		    { $c1 = (($c1 & 1) << 6) | *src as c_int; src = src.offset (1); }
		  else
		    {
		      src = src.offset ((BYTES_BY_CHAR_HEAD ($c1) - 1) as isize);
		      $c1 = -1;
		    }
		}
	      if ! ($c1 < 0)
		{ break; }
	    }
	  if src == src_end
	    { break 'no_more_source; }
	  $c2 = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c2 & 0x80) != 0
	    {
	      if ($c2 & 0xFE) == 0xC0
		{ $c2 = (($c2 & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{ $c2 = -1; }
	    }
	}
      };
    }

    if mode_of (coding) as c_int & CODING_MODE_LAST_BLOCK != 0
	&& ((*coding).src_chars & 1) != 0
      {
	(*detect_info).rejected |= CATEGORY_MASK_UTF_16;
	return false;
      }

    TWO_MORE_BYTES! (c1, c2);
    if (c1 == 0xFF) && (c2 == 0xFE)
      {
	(*detect_info).found |= (CATEGORY_MASK_UTF_16_LE
			       | CATEGORY_MASK_UTF_16_AUTO);
	(*detect_info).rejected |= (CATEGORY_MASK_UTF_16_BE
				  | CATEGORY_MASK_UTF_16_BE_NOSIG
				  | CATEGORY_MASK_UTF_16_LE_NOSIG);
      }
    else if (c1 == 0xFE) && (c2 == 0xFF)
      {
	(*detect_info).found |= (CATEGORY_MASK_UTF_16_BE
			       | CATEGORY_MASK_UTF_16_AUTO);
	(*detect_info).rejected |= (CATEGORY_MASK_UTF_16_LE
				  | CATEGORY_MASK_UTF_16_BE_NOSIG
				  | CATEGORY_MASK_UTF_16_LE_NOSIG);
      }
    else if c2 < 0
      {
	(*detect_info).rejected |= CATEGORY_MASK_UTF_16;
	return false;
      }
    else
      {
	/* We check the dispersion of Eth and Oth bytes where E is even and
	   O is odd.  If both are high, we assume binary data.*/
	let mut e: [c_uchar; 256] = [0; 256];
	let mut o: [c_uchar; 256] = [0; 256];
	let mut e_num: c_uint = 1;
	let mut o_num: c_uint = 1;

	e[c1 as usize] = 1;
	o[c2 as usize] = 1;

	(*detect_info).rejected |= (CATEGORY_MASK_UTF_16_AUTO
				  | CATEGORY_MASK_UTF_16_BE
				  | CATEGORY_MASK_UTF_16_LE);

	while ((*detect_info).rejected & CATEGORY_MASK_UTF_16)
	       != CATEGORY_MASK_UTF_16
	  {
	    TWO_MORE_BYTES! (c1, c2);
	    if c2 < 0
	      { break; }
	    if e[c1 as usize] == 0
	      {
		e[c1 as usize] = 1;
		e_num += 1;
		if e_num >= 128
		  { (*detect_info).rejected |= CATEGORY_MASK_UTF_16_BE_NOSIG; }
	      }
	    if o[c2 as usize] == 0
	      {
		o[c2 as usize] = 1;
		o_num += 1;
		if o_num >= 128
		  { (*detect_info).rejected |= CATEGORY_MASK_UTF_16_LE_NOSIG; }
	      }
	  }
	return false;
      }
    return false;
  }

  /* no_more_source: */
  return true;
}


unsafe extern "C" fn decode_coding_utf_16 (coding: *mut coding_system)
{
  let mut src: *const c_uchar = (*coding).source.offset ((*coding).consumed);
  let src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let mut src_base: *const c_uchar = std::ptr::null ();
  let mut charbuf: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_used as isize);
  /* We may produces at most 3 chars in one loop.  */
  let charbuf_end: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_size as isize - 2);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut consumed_chars_base: ptrdiff_t = 0;
  let multibytep = src_multibyte_p (coding);
  let bom: utf_bom_type = CODING_UTF_16_BOM! (coding);
  let endian: utf_16_endian_type = CODING_UTF_16_ENDIAN! (coding);
  let mut surrogate: c_int = CODING_UTF_16_SURROGATE! (coding);
  let eol_dos
    = !inhibit_eol_conversion && EQ (CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t), Qdos ());
  let mut byte_after_cr1: c_int = -1;
  let mut byte_after_cr2: c_int = -1;

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }

    if bom == utf_with_bom
      {
	let mut c: c_int;
	let mut c1: c_int; let mut c2: c_int;

	src_base = src;
	ONE_MORE_BYTE! (c1);
	ONE_MORE_BYTE! (c2);
	c = (c1 << 8) | c2;

	if if endian == utf_16_big_endian { c != 0xFEFF } else { c != 0xFFFE }
	  {
	    /* The first two bytes are not BOM.  Treat them as bytes
	       for a normal character.  */
	    src = src_base;
	  }
	CODING_UTF_16_BOM! (coding) = utf_without_bom;
      }
    else if bom == utf_detect_bom
      {
	/* We have already tried to detect BOM and failed in
	   detect_coding.  */
	CODING_UTF_16_BOM! (coding) = utf_without_bom;
      }

    loop
      {
	let mut c: c_int;
	let mut c1: c_int; let mut c2: c_int;

	src_base = src;
	consumed_chars_base = consumed_chars;

	if charbuf >= charbuf_end
	  {
	    if byte_after_cr1 >= 0
	      { src_base = src_base.offset (-2); }
	    break;
	  }

	if byte_after_cr1 >= 0
	  { c1 = byte_after_cr1; byte_after_cr1 = -1; }
	else
	  { ONE_MORE_BYTE! (c1); }
	if c1 < 0
	  {
	    *charbuf = -c1; charbuf = charbuf.offset (1);
	    continue;
	  }
	if byte_after_cr2 >= 0
	  { c2 = byte_after_cr2; byte_after_cr2 = -1; }
	else
	  { ONE_MORE_BYTE! (c2); }
	if c2 < 0
	  {
	    *charbuf = if ASCII_CHAR_P (c1 as isize) { c1 } else { BYTE8_TO_CHAR (c1) };
	    charbuf = charbuf.offset (1);
	    *charbuf = -c2; charbuf = charbuf.offset (1);
	    continue;
	  }
	c = if endian == utf_16_big_endian { (c1 << 8) | c2 } else { (c2 << 8) | c1 };

	if surrogate != 0
	  {
	    if UTF_16_LOW_SURROGATE_P (c) == 0
	      {
		if endian == utf_16_big_endian
		  { c1 = surrogate >> 8; c2 = surrogate & 0xFF; }
		else
		  { c1 = surrogate & 0xFF; c2 = surrogate >> 8; }
		*charbuf = c1; charbuf = charbuf.offset (1);
		*charbuf = c2; charbuf = charbuf.offset (1);
		if UTF_16_HIGH_SURROGATE_P (c) != 0
		  { CODING_UTF_16_SURROGATE! (coding) = c; surrogate = c; }
		else
		  { *charbuf = c; charbuf = charbuf.offset (1); }
	      }
	    else
	      {
		c = ((surrogate - 0xD800) << 10) | (c - 0xDC00);
		CODING_UTF_16_SURROGATE! (coding) = 0; surrogate = 0;
		*charbuf = 0x10000 + c; charbuf = charbuf.offset (1);
	      }
	  }
	else
	  {
	    if UTF_16_HIGH_SURROGATE_P (c) != 0
	      { CODING_UTF_16_SURROGATE! (coding) = c; surrogate = c; }
	    else
	      {
		if eol_dos && c == '\r' as c_int
		  {
		    ONE_MORE_BYTE! (byte_after_cr1);
		    ONE_MORE_BYTE! (byte_after_cr2);
		  }
		*charbuf = c; charbuf = charbuf.offset (1);
	      }
	  }
      }
  }

  (*coding).consumed_char += consumed_chars_base;
  (*coding).consumed = src_base.offset_from ((*coding).source);
  (*coding).charbuf_used = charbuf.offset_from ((*coding).charbuf) as c_int;
}

unsafe extern "C" fn encode_coding_utf_16 (coding: *mut coding_system) -> bool
{
  let multibytep = dst_multibyte_p (coding);
  let mut charbuf: *mut c_int = (*coding).charbuf;
  let charbuf_end: *mut c_int = charbuf.offset ((*coding).charbuf_used as isize);
  let mut dst: *mut c_uchar = (*coding).destination.offset ((*coding).produced);
  let mut dst_end: *mut c_uchar = (*coding).destination.offset ((*coding).dst_bytes);
  let safe_room: c_int = 8;
  let bom: utf_bom_type = CODING_UTF_16_BOM! (coding);
  let big_endian: bool = CODING_UTF_16_ENDIAN! (coding) == utf_16_big_endian;
  let mut produced_chars: ptrdiff_t = 0;
  let mut c: c_int;

  macro_rules! ASSURE_DESTINATION {
    ($bytes:expr) => {
      {
	if dst.offset (($bytes) as isize) >= dst_end
	  {
	    let more_bytes: ptrdiff_t = charbuf_end.offset_from (charbuf) + ($bytes) as isize;

	    dst = alloc_destination (coding, more_bytes, dst);
	    dst_end = (*coding).destination.offset ((*coding).dst_bytes);
	  }
      }
    };
  }
  macro_rules! EMIT_TWO_BYTES {
    ($c1:expr, $c2:expr) => {
      {
	produced_chars += 2;
	if multibytep
	  {
	    let mut ch: c_uint;

	    ch = ($c1) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	    ch = ($c2) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  {
	    *dst = ($c1) as c_uchar; dst = dst.offset (1);
	    *dst = ($c2) as c_uchar; dst = dst.offset (1);
	  }
      }
    };
  }
  macro_rules! EMIT_FOUR_BYTES {
    ($c1:expr, $c2:expr, $c3:expr, $c4:expr) => {
      {
	EMIT_TWO_BYTES! ($c1, $c2);
	EMIT_TWO_BYTES! ($c3, $c4);
      }
    };
  }

  if bom != utf_without_bom
    {
      ASSURE_DESTINATION! (safe_room);
      if big_endian
	{ EMIT_TWO_BYTES! (0xFE, 0xFF); }
      else
	{ EMIT_TWO_BYTES! (0xFF, 0xFE); }
      CODING_UTF_16_BOM! (coding) = utf_without_bom;
    }

  while charbuf < charbuf_end
    {
      ASSURE_DESTINATION! (safe_room);
      c = *charbuf; charbuf = charbuf.offset (1);
      if c > MAX_UNICODE_CHAR
	{ c = (*coding).default_char; }

      if c < 0x10000
	{
	  if big_endian
	    { EMIT_TWO_BYTES! (c >> 8, c & 0xFF); }
	  else
	    { EMIT_TWO_BYTES! (c & 0xFF, c >> 8); }
	}
      else
	{
	  let mut c1: c_int; let mut c2: c_int;

	  c -= 0x10000;
	  c1 = (c >> 10) + 0xD800;
	  c2 = (c & 0x3FF) + 0xDC00;
	  if big_endian
	    { EMIT_FOUR_BYTES! (c1 >> 8, c1 & 0xFF, c2 >> 8, c2 & 0xFF); }
	  else
	    { EMIT_FOUR_BYTES! (c1 & 0xFF, c1 >> 8, c2 & 0xFF, c2 >> 8); }
	}
    }
  record_conversion_result (coding, CODING_RESULT_SUCCESS);
  (*coding).produced = dst.offset_from ((*coding).destination);
  (*coding).produced_char += produced_chars;
  return false;
}

/*** 6. Old Emacs's internal format (emacs-mule) ***/

/* Emacs's internal format for representation of multiple character
   sets is a kind of multi-byte encoding, i.e. characters are
   represented by variable-length sequences of one-byte codes.

   ASCII characters and control characters (e.g. `tab', `newline') are
   represented by one-byte sequences which are their ASCII codes, in
   the range 0x00 through 0x7F.

   8-bit characters of the range 0x80..0x9F are represented by
   two-byte sequences of LEADING_CODE_8_BIT_CONTROL and (their 8-bit
   code + 0x20).

   8-bit characters of the range 0xA0..0xFF are represented by
   one-byte sequences which are their 8-bit code.

   The other characters are represented by a sequence of `base
   leading-code', optional `extended leading-code', and one or two
   `position-code's.  The length of the sequence is determined by the
   base leading-code.  Leading-code takes the range 0x81 through 0x9D,
   whereas extended leading-code and position-code take the range 0xA0
   through 0xFF.  See `charset.h' for more details about leading-code
   and position-code.

   --- CODE RANGE of Emacs's internal format ---
   character set	range
   -------------	-----
   ascii		0x00..0x7F
   eight-bit-control	LEADING_CODE_8_BIT_CONTROL + 0xA0..0xBF
   eight-bit-graphic	0xA0..0xBF
   ELSE			0x81..0x9D + [0xA0..0xFF]+
   ---------------------------------------------

   As this is the internal character representation, the format is
   usually not used externally (i.e. in a file or in a data sent to a
   process).  But, it is possible to have a text externally in this
   format (i.e. by encoding by the coding system `emacs-mule').

   In that case, a sequence of one-byte codes has a slightly different
   form.

   At first, all characters in eight-bit-control are represented by
   one-byte sequences which are their 8-bit code.

   Next, character composition data are represented by the byte
   sequence of the form: 0x80 METHOD BYTES CHARS COMPONENT ...,
   where,
	METHOD is 0xF2 plus one of composition method (enum
	composition_method),

	BYTES is 0xA0 plus a byte length of this composition data,

	CHARS is 0xA0 plus a number of characters composed by this
	data,

	COMPONENTs are characters of multibyte form or composition
	rules encoded by two-byte of ASCII codes.

   In addition, for backward compatibility, the following formats are
   also recognized as composition data on decoding.

   0x80 MSEQ ...
   0x80 0xFF MSEQ RULE MSEQ RULE ... MSEQ

   Here,
	MSEQ is a multibyte form but in these special format:
	  ASCII: 0xA0 ASCII_CODE+0x80,
	  other: LEADING_CODE+0x20 FOLLOWING-BYTE ...,
	RULE is a one byte code of the range 0xA0..0xF0 that
	represents a composition rule.
  */

/* See the above "GENERAL NOTES on `detect_coding_XXX ()' functions".
   Return true if a text is encoded in 'emacs-mule'.  */

unsafe extern "C" fn detect_coding_emacs_mule (coding: *mut coding_system,
                                    detect_info: *mut coding_detection_info) -> bool
{
  let mut src: *const c_uchar = (*coding).source;
  let mut src_base: *const c_uchar = std::ptr::null ();
  let src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let multibytep = src_multibyte_p (coding);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut c: c_int;
  let mut found: c_int = 0;

  (*detect_info).checked |= CATEGORY_MASK_EMACS_MULE;
  /* A coding system of this category is always ASCII compatible.  */
  src = src.offset ((*coding).head_ascii);

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }

    'detect_loop: while true
      {
	src_base = src;
	ONE_MORE_BYTE! (c);
	if c < 0
	  { continue 'detect_loop; }
	if c == 0x80
	  {
	    /* Perhaps the start of composite character.  We simply skip
	       it because analyzing it is too heavy for detecting.  But,
	       at least, we check that the composite character
	       constitutes of more than 4 bytes.  */
	    let mut src_start: *const c_uchar;

	    'repeat: loop
	      {
		src_start = src;
		loop
		  {
		    ONE_MORE_BYTE! (c);
		    if ! (c >= 0xA0)
		      { break; }
		  }

		if src.offset_from (src_start) <= 4
		  { break 'detect_loop; }
		found = CATEGORY_MASK_EMACS_MULE;
		if c == 0x80
		  { continue 'repeat; }
		break 'repeat;
	      }
	  }

	if c < 0x80
	  {
	    if c < 0x20
		&& (c == ISO_CODE_ESC || c == ISO_CODE_SI || c == ISO_CODE_SO)
	      { break 'detect_loop; }
	  }
	else
	  {
	    let mut more_bytes: c_int = emacs_mule_bytes[c as usize] as c_int - 1;

	    while more_bytes > 0
	      {
		ONE_MORE_BYTE! (c);
		if c < 0xA0
		  {
		    src = src.offset (-1);	/* Unread the last byte.  */
		    break;
		  }
		more_bytes -= 1;
	      }
	    if more_bytes != 0
	      { break 'detect_loop; }
	    found = CATEGORY_MASK_EMACS_MULE;
	  }
      }
    (*detect_info).rejected |= CATEGORY_MASK_EMACS_MULE;
    return false;
  }

  if src_base < src && mode_of (coding) as c_int & CODING_MODE_LAST_BLOCK != 0
    {
      (*detect_info).rejected |= CATEGORY_MASK_EMACS_MULE;
      return false;
    }
  (*detect_info).found |= found;
  return true;
}


/* Parse emacs-mule multibyte sequence at SRC and return the decoded
   character.  If CMP_STATUS indicates that we must expect MSEQ or
   RULE described above, decode it and return the negative value of
   the decoded character or rule.  If an invalid byte is found, return
   -1.  If SRC is too short, return -2.  */

unsafe fn emacs_mule_char (coding: *mut coding_system, src: *const c_uchar,
			   nbytes: *mut c_int, nchars: *mut c_int, id: *mut c_int,
			   cmp_status: *mut composition_status) -> c_int
{
  let mut src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let mut src_base: *const c_uchar = src;
  let multibytep = src_multibyte_p (coding);
  let mut charset_ID: c_int = 0;
  let mut code: c_uint = 0;
  let mut c: c_int;
  let mut consumed_chars: ptrdiff_t = 0;
  let mut mseq_found: bool = false;
  let mut src: *const c_uchar = src;

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }
    macro_rules! CODING_DECODE_CHAR {
      ($coding:expr, $src:expr, $src_base:expr, $src_end:expr, $charset:expr, $code:expr, $c:expr) => {
	{
	  let offset: ptrdiff_t;

	  charset_map_loaded = false;
	  $c = DECODE_CHAR! ($charset, $code);
	  if charset_map_loaded
	      && { offset = coding_change_source ($coding); offset != 0 }
	    {
	      $src = $src.offset (offset);
	      $src_base = $src_base.offset (offset);
	      $src_end = $src_end.offset (offset);
	    }
	}
      };
    }

    'invalid_code: {
      ONE_MORE_BYTE! (c);
      if c < 0
	{
	  c = -c;
	  charset_ID = emacs_mule_charset[0];
	}
      else
	{
	  if c >= 0xA0
	    {
	      if (*cmp_status).state != COMPOSING_NO
		  && (*cmp_status).old_form
		{
		  if (*cmp_status).state == COMPOSING_CHAR
		    {
		      if c == 0xA0
			{
			  ONE_MORE_BYTE! (c);
			  c -= 0x80;
			  if c < 0
			    { break 'invalid_code; }
			}
		      else
			{ c -= 0x20; }
		      mseq_found = true;
		    }
		  else
		    {
		      *nbytes = src.offset_from (src_base) as c_int;
		      *nchars = consumed_chars as c_int;
		      return -c;
		    }
		}
	      else
		{ break 'invalid_code; }
	    }

	  match emacs_mule_bytes[c as usize] as c_int
	    {
	    2 =>
	      {
		charset_ID = emacs_mule_charset[c as usize];
		if charset_ID < 0
		  { break 'invalid_code; }
		ONE_MORE_BYTE! (c);
		if c < 0xA0
		  { break 'invalid_code; }
		code = (c & 0x7F) as c_uint;
	      }
	    3 =>
	      {
		if c == EMACS_MULE_LEADING_CODE_PRIVATE_11
		    || c == EMACS_MULE_LEADING_CODE_PRIVATE_12
		  {
		    ONE_MORE_BYTE! (c);
		    charset_ID = emacs_mule_charset[c as usize];
		    if c < 0xA0 || charset_ID < 0
		      { break 'invalid_code; }
		    ONE_MORE_BYTE! (c);
		    if c < 0xA0
		      { break 'invalid_code; }
		    code = (c & 0x7F) as c_uint;
		  }
		else
		  {
		    charset_ID = emacs_mule_charset[c as usize];
		    if charset_ID < 0
		      { break 'invalid_code; }
		    ONE_MORE_BYTE! (c);
		    if c < 0xA0
		      { break 'invalid_code; }
		    code = ((c & 0x7F) << 8) as c_uint;
		    ONE_MORE_BYTE! (c);
		    if c < 0xA0
		      { break 'invalid_code; }
		    code |= (c & 0x7F) as c_uint;
		  }
	      }
	    4 =>
	      {
		ONE_MORE_BYTE! (c);
		charset_ID = emacs_mule_charset[c as usize];
		if c < 0 || charset_ID < 0
		  { break 'invalid_code; }
		ONE_MORE_BYTE! (c);
		if c < 0xA0
		  { break 'invalid_code; }
		code = ((c & 0x7F) << 8) as c_uint;
		ONE_MORE_BYTE! (c);
		if c < 0xA0
		  { break 'invalid_code; }
		code |= (c & 0x7F) as c_uint;
	      }
	    1 =>
	      {
		code = c as c_uint;
		charset_ID = if ASCII_CHAR_P (code as isize) { charset_ascii } else { charset_eight_bit };
	      }
	    _ =>
	      { emacs_abort (); }
	    }
	  CODING_DECODE_CHAR! (coding, src, src_base, src_end,
			       CHARSET_FROM_ID! (charset_ID), code, c);
	  if c < 0
	    { break 'invalid_code; }
	}
      *nbytes = src.offset_from (src_base) as c_int;
      *nchars = consumed_chars as c_int;
      if ! id.is_null ()
	{ *id = charset_ID; }
      return if mseq_found { -c } else { c };
    }

    /* no_more_source: */
    return -2;
  }

  /* invalid_code: */
  return -1;
}


/* Maximum length of the header of annotation data.  */
pub const MAX_ANNOTATION_LENGTH: c_int = 5;

/* Bitmasks for coding->eol_seen.  */

pub const EOL_SEEN_NONE: c_int = 0;
pub const EOL_SEEN_LF: c_int = 1;
pub const EOL_SEEN_CR: c_int = 2;
pub const EOL_SEEN_CRLF: c_int = 4;

unsafe fn emacs_mule_finish_composition (charbuf: *mut c_int,
                                         cmp_status: *mut composition_status) -> c_int
{
  let mut idx: c_int = - (*cmp_status).length;
  let mut new_chars: c_int;

  if (*cmp_status).old_form && (*cmp_status).nchars > 0
    {
      *charbuf.offset ((idx + 2) as isize) = (*cmp_status).nchars;
      new_chars = 0;
      if (*cmp_status).method == COMPOSITION_WITH_RULE
	  && (*cmp_status).state == COMPOSING_CHAR
	{
	  /* The last rule was invalid.  */
	  let rule: c_int = *charbuf.offset (-1) + 0xA0;

	  *charbuf.offset (-2) = BYTE8_TO_CHAR (rule);
	  *charbuf.offset (-1) = -1;
	  new_chars = 1;
	}
    }
  else
    {
      *charbuf.offset (idx as isize) = BYTE8_TO_CHAR (0x80); idx += 1;

      if (*cmp_status).method == COMPOSITION_WITH_RULE
	{
	  *charbuf.offset (idx as isize) = BYTE8_TO_CHAR (0xFF); idx += 1;
	  *charbuf.offset (idx as isize) = -3; idx += 1;
	  *charbuf.offset (idx as isize) = 0; idx += 1;
	  new_chars = 1;
	}
      else
	{
	  let nchars: c_int = *charbuf.offset ((idx + 1) as isize) + 0xA0;
	  let nbytes: c_int = *charbuf.offset ((idx + 2) as isize) + 0xA0;

	  *charbuf.offset (idx as isize) = BYTE8_TO_CHAR (0xF2 + (*cmp_status).method); idx += 1;
	  *charbuf.offset (idx as isize) = BYTE8_TO_CHAR (nbytes); idx += 1;
	  *charbuf.offset (idx as isize) = BYTE8_TO_CHAR (nchars); idx += 1;
	  *charbuf.offset (idx as isize) = -1; idx += 1;
	  new_chars = 4;
	}
    }
  (*cmp_status).state = COMPOSING_NO;
  return new_chars;
}

unsafe extern "C" fn decode_coding_emacs_mule (coding: *mut coding_system)
{
  let mut src: *const c_uchar = (*coding).source.offset ((*coding).consumed);
  let mut src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let mut src_base: *const c_uchar = std::ptr::null ();
  let mut charbuf: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_used as isize);
  /* We may produce two annotations (charset and composition) in one
     loop and one more charset annotation at the end.  */
  let charbuf_end: *mut c_int
    = (*coding).charbuf.offset (((*coding).charbuf_size - (MAX_ANNOTATION_LENGTH * 3)
				 /* We can produce up to 2 characters in a loop.  */
				 - 1) as isize);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut consumed_chars_base: ptrdiff_t = 0;
  let multibytep = src_multibyte_p (coding);
  let mut char_offset: ptrdiff_t = (*coding).produced_char;
  let mut last_offset: ptrdiff_t = char_offset;
  let mut last_id: c_int = charset_ascii;
  let eol_dos
    = !inhibit_eol_conversion && EQ (CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t), Qdos ());
  let mut byte_after_cr: c_int = -1;
  let cmp_status: *mut composition_status
    = &raw mut (*coding).spec.emacs_mule.cmp_status;


    macro_rules! ADD_ANNOTATION_DATA {
      ($buf:expr, $len:expr, $mask:expr, $nchars:expr) => {
	{
	  *$buf = -($len); $buf = $buf.offset (1);
	  *$buf = ($mask); $buf = $buf.offset (1);
	  *$buf = ($nchars) as c_int; $buf = $buf.offset (1);
	  coding_bits_set (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1, 1);
	}
      };
    }
    macro_rules! ADD_COMPOSITION_DATA {
      ($buf:expr, $nchars:expr, $nbytes:expr, $method:expr) => {
	{
	  ADD_ANNOTATION_DATA! ($buf, 5, CODING_ANNOTATE_COMPOSITION_MASK as c_int, $nchars);
	  *$buf = $nbytes; $buf = $buf.offset (1);
	  *$buf = $method; $buf = $buf.offset (1);
	}
      };
    }
    macro_rules! ADD_CHARSET_DATA {
      ($buf:expr, $nchars:expr, $id:expr) => {
	{
	  ADD_ANNOTATION_DATA! ($buf, 4, CODING_ANNOTATE_CHARSET_MASK as c_int, $nchars);
	  *$buf = $id; $buf = $buf.offset (1);
	}
      };
    }
    macro_rules! EMACS_MULE_MAYBE_FINISH_COMPOSITION {
      () => {
	{
	  if (*cmp_status).state != COMPOSING_NO
	    { char_offset += emacs_mule_finish_composition (charbuf, cmp_status) as ptrdiff_t; }
	}
      };
    }
  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }

    if (*cmp_status).state != COMPOSING_NO
      {
	let mut i: c_int;

	if charbuf_end.offset_from (charbuf) < (*cmp_status).length as isize
	  { emacs_abort (); }
	i = 0;
	while i < (*cmp_status).length
	  {
	    *charbuf = (*cmp_status).carryover[i as usize] as c_int; charbuf = charbuf.offset (1);
	    i += 1;
	  }
	coding_bits_set (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1, 1);
      }

    'decode_loop: loop
      {
	let mut c: c_int;
	let mut id: c_int = 0;

	src_base = src;
	consumed_chars_base = consumed_chars;

	if charbuf >= charbuf_end
	  {
	    if byte_after_cr >= 0
	      { src_base = src_base.offset (-1); }
	    break;
	  }

	'invalid_code: {

    macro_rules! DECODE_EMACS_MULE_21_COMPOSITION {
      () => {
	{
	  let method: composition_method = c - 0xF2;
	  let mut nbytes: c_int;
	  let mut nchars: c_int;

	  ONE_MORE_BYTE! (c);
	  if c < 0
	    { break 'invalid_code; }
	  nbytes = c - 0xA0;
	  if nbytes < 3 || (method == COMPOSITION_RELATIVE && nbytes != 4)
	    { break 'invalid_code; }
	  ONE_MORE_BYTE! (c);
	  nchars = c - 0xA0;
	  if nchars <= 0 || nchars >= MAX_COMPOSITION_COMPONENTS
	    { break 'invalid_code; }
	  (*cmp_status).old_form = false;
	  (*cmp_status).method = method;
	  if method == COMPOSITION_RELATIVE
	    { (*cmp_status).state = COMPOSING_CHAR; }
	  else
	    { (*cmp_status).state = COMPOSING_COMPONENT_CHAR; }
	  (*cmp_status).length = MAX_ANNOTATION_LENGTH;
	  (*cmp_status).nchars = nchars;
	  (*cmp_status).ncomps = nbytes - 4;
	  ADD_COMPOSITION_DATA! (charbuf, nchars, nbytes, method);
	}
      };
    }

    macro_rules! DECODE_EMACS_MULE_20_RELATIVE_COMPOSITION {
      () => {
	{
	  (*cmp_status).old_form = true;
	  (*cmp_status).method = COMPOSITION_RELATIVE;
	  (*cmp_status).state = COMPOSING_CHAR;
	  (*cmp_status).length = MAX_ANNOTATION_LENGTH;
	  (*cmp_status).nchars = 0; (*cmp_status).ncomps = 0;
	  ADD_COMPOSITION_DATA! (charbuf, 0, 0, (*cmp_status).method);
	}
      };
    }

    macro_rules! DECODE_EMACS_MULE_20_RULEBASE_COMPOSITION {
      () => {
	{
	  (*cmp_status).old_form = true;
	  (*cmp_status).method = COMPOSITION_WITH_RULE;
	  (*cmp_status).state = COMPOSING_CHAR;
	  (*cmp_status).length = MAX_ANNOTATION_LENGTH;
	  (*cmp_status).nchars = 0; (*cmp_status).ncomps = 0;
	  ADD_COMPOSITION_DATA! (charbuf, 0, 0, (*cmp_status).method);
	}
      };
    }

    macro_rules! DECODE_EMACS_MULE_COMPOSITION_START {
      () => {
	{
	  let current_src: *const c_uchar = src;

	  ONE_MORE_BYTE! (c);
	  if c < 0
	    { break 'invalid_code; }
	  if c - 0xF2 >= COMPOSITION_RELATIVE
	      && c - 0xF2 <= COMPOSITION_WITH_RULE_ALTCHARS
	    { DECODE_EMACS_MULE_21_COMPOSITION! (); }
	  else if c < 0xA0
	    { break 'invalid_code; }
	  else if c < 0xC0
	    {
	      DECODE_EMACS_MULE_20_RELATIVE_COMPOSITION! ();
	      /* Re-read C as a composition component.  */
	      src = current_src;
	    }
	  else if c == 0xFF
	    { DECODE_EMACS_MULE_20_RULEBASE_COMPOSITION! (); }
	  else
	    { break 'invalid_code; }
	}
      };
    }

    macro_rules! DECODE_EMACS_MULE_COMPOSITION_RULE_20 {
      ($c:expr, $rule:ident) => {
	{
	  let mut gref: c_int;
	  let mut nref: c_int;

	  $c -= 0xA0;
	  if $c < 0 || $c >= 81
	    { break 'invalid_code; }
	  gref = $c / 9; nref = $c % 9;
	  if gref == 4 { gref = 10; }
	  if nref == 4 { nref = 10; }
	  $rule = COMPOSITION_ENCODE_RULE (gref, nref);
	}
      };
    }

    macro_rules! DECODE_EMACS_MULE_COMPOSITION_RULE_21 {
      ($c:ident, $rule:ident) => {
	{
	  let mut gref: c_int;
	  let mut nref: c_int;

	  gref = $c - 0x20;
	  if gref < 0 || gref >= 81
	    { break 'invalid_code; }
	  ONE_MORE_BYTE! ($c);
	  nref = $c - 0x20;
	  if nref < 0 || nref >= 81
	    { break 'invalid_code; }
	  $rule = COMPOSITION_ENCODE_RULE (gref, nref);
	}
      };
    }

    macro_rules! EMACS_MULE_COMPOSITION_END {
      () => {
	{
	  let idx: c_int = - (*cmp_status).length;

	  if (*cmp_status).old_form
	    { *charbuf.offset ((idx + 2) as isize) = (*cmp_status).nchars; }
	  else if (*cmp_status).method > COMPOSITION_RELATIVE
	    { *charbuf.offset (idx as isize) = *charbuf.offset ((idx + 2) as isize) - (*cmp_status).length; }
	  (*cmp_status).state = COMPOSING_NO;
	}
      };
    }

	  if byte_after_cr >= 0
	    { c = byte_after_cr; byte_after_cr = -1; }
	  else
	    { ONE_MORE_BYTE! (c); }

	  if c < 0 || c == 0x80
	    {
	      EMACS_MULE_MAYBE_FINISH_COMPOSITION! ();
	      if c < 0
		{
		  *charbuf = -c; charbuf = charbuf.offset (1);
		  char_offset += 1;
		}
	      else
		{ DECODE_EMACS_MULE_COMPOSITION_START! (); }
	      continue 'decode_loop;
	    }

	  if c < 0x80
	    {
	      if eol_dos && c == '\r' as c_int
		{ ONE_MORE_BYTE! (byte_after_cr); }
	      id = charset_ascii;
	      if (*cmp_status).state != COMPOSING_NO
		{
		  if (*cmp_status).old_form
		    { EMACS_MULE_MAYBE_FINISH_COMPOSITION! (); }
		  else if (*cmp_status).state >= COMPOSING_COMPONENT_CHAR
		    { (*cmp_status).ncomps -= 1; }
		}
	    }
	  else
	    {
	      let mut nchars: c_int = 0;
	      let mut nbytes: c_int = 0;
	      /* emacs_mule_char can load a charset map from a file, which
		 allocates a large structure and might cause buffer text
		 to be relocated as result.  Thus, we need to remember the
		 original pointer to buffer text, and fix up all related
		 pointers after the call.  */
	      let orig: *const c_uchar = (*coding).source;
	      let offset: ptrdiff_t;

	      c = emacs_mule_char (coding, src_base, &raw mut nbytes, &raw mut nchars, &raw mut id,
				   cmp_status);
	      offset = (*coding).source.offset_from (orig);
	      if offset != 0
		{
		  src = src.offset (offset);
		  src_base = src_base.offset (offset);
		  src_end = src_end.offset (offset);
		}
	      if c < 0
		{
		  if c == -1
		    { break 'invalid_code; }
		  if c == -2
		    { break 'decode_loop; }
		}
	      src = src_base.offset (nbytes as isize);
	      consumed_chars = consumed_chars_base + nchars as ptrdiff_t;
	      if (*cmp_status).state >= COMPOSING_COMPONENT_CHAR
		{ (*cmp_status).ncomps -= nchars; }
	    }

	  /* Now if C >= 0, we found a normally encoded character, if C <
	     0, we found an old-style composition component character or
	     rule.  */

	  if (*cmp_status).state == COMPOSING_NO
	    {
	      if last_id != id
		{
		  if last_id != charset_ascii
		    { ADD_CHARSET_DATA! (charbuf, char_offset - last_offset, last_id); }
		  last_id = id;
		  last_offset = char_offset;
		}
	      *charbuf = c; charbuf = charbuf.offset (1);
	      char_offset += 1;
	    }
	  else if (*cmp_status).state == COMPOSING_CHAR
	    {
	      if (*cmp_status).old_form
		{
		  if c >= 0
		    {
		      EMACS_MULE_MAYBE_FINISH_COMPOSITION! ();
		      *charbuf = c; charbuf = charbuf.offset (1);
		      char_offset += 1;
		    }
		  else
		    {
		      *charbuf = -c; charbuf = charbuf.offset (1);
		      (*cmp_status).nchars += 1;
		      (*cmp_status).length += 1;
		      if (*cmp_status).nchars == MAX_COMPOSITION_COMPONENTS
			{ EMACS_MULE_COMPOSITION_END! (); }
		      else if (*cmp_status).method == COMPOSITION_WITH_RULE
			{ (*cmp_status).state = COMPOSING_RULE; }
		    }
		}
	      else
		{
		  *charbuf = c; charbuf = charbuf.offset (1);
		  (*cmp_status).length += 1;
		  (*cmp_status).nchars -= 1;
		  if (*cmp_status).nchars == 0
		    { EMACS_MULE_COMPOSITION_END! (); }
		}
	    }
	  else if (*cmp_status).state == COMPOSING_RULE
	    {
	      let mut rule: c_int;

	      if c >= 0
		{
		  EMACS_MULE_COMPOSITION_END! ();
		  *charbuf = c; charbuf = charbuf.offset (1);
		  char_offset += 1;
		}
	      else
		{
		  c = -c;
		  DECODE_EMACS_MULE_COMPOSITION_RULE_20! (c, rule);
		  if rule < 0
		    { break 'invalid_code; }
		  *charbuf = -2; charbuf = charbuf.offset (1);
		  *charbuf = rule; charbuf = charbuf.offset (1);
		  (*cmp_status).length += 2;
		  (*cmp_status).state = COMPOSING_CHAR;
		}
	    }
	  else if (*cmp_status).state == COMPOSING_COMPONENT_CHAR
	    {
	      *charbuf = c; charbuf = charbuf.offset (1);
	      (*cmp_status).length += 1;
	      if (*cmp_status).ncomps == 0
		{ (*cmp_status).state = COMPOSING_CHAR; }
	      else if (*cmp_status).ncomps > 0
		{
		  if (*cmp_status).method == COMPOSITION_WITH_RULE_ALTCHARS
		    { (*cmp_status).state = COMPOSING_COMPONENT_RULE; }
		}
	      else
		{ EMACS_MULE_MAYBE_FINISH_COMPOSITION! (); }
	    }
	  else			/* COMPOSING_COMPONENT_RULE */
	    {
	      let mut rule: c_int;

	      DECODE_EMACS_MULE_COMPOSITION_RULE_21! (c, rule);
	      if rule < 0
		{ break 'invalid_code; }
	      *charbuf = -2; charbuf = charbuf.offset (1);
	      *charbuf = rule; charbuf = charbuf.offset (1);
	      (*cmp_status).length += 2;
	      (*cmp_status).ncomps -= 1;
	      if (*cmp_status).ncomps > 0
		{ (*cmp_status).state = COMPOSING_COMPONENT_CHAR; }
	      else
		{ EMACS_MULE_MAYBE_FINISH_COMPOSITION! (); }
	    }
	  continue 'decode_loop;
	}

	/* invalid_code: */
	EMACS_MULE_MAYBE_FINISH_COMPOSITION! ();
	src = src_base;
	consumed_chars = consumed_chars_base;
	ONE_MORE_BYTE! (c);
	*charbuf = if ASCII_CHAR_P (c as isize) { c } else { BYTE8_TO_CHAR (c) }; charbuf = charbuf.offset (1);
	char_offset += 1;
      }
  }

  if (*cmp_status).state != COMPOSING_NO
    {
      if mode_of (coding) as c_int & CODING_MODE_LAST_BLOCK != 0
	{ EMACS_MULE_MAYBE_FINISH_COMPOSITION! (); }
      else
	{
	  let mut i: c_int;

	  charbuf = charbuf.offset (- ((*cmp_status).length as isize));
	  i = 0;
	  while i < (*cmp_status).length
	    {
	      (*cmp_status).carryover[i as usize] = *charbuf;
	      i += 1;
	    }
	}
    }
  if last_id != charset_ascii
    { ADD_CHARSET_DATA! (charbuf, char_offset - last_offset, last_id); }
  (*coding).consumed_char += consumed_chars_base;
  (*coding).consumed = src_base.offset_from ((*coding).source);
  (*coding).charbuf_used = charbuf.offset_from ((*coding).charbuf) as c_int;
}


unsafe extern "C" fn encode_coding_emacs_mule (coding: *mut coding_system) -> bool
{
  let multibytep = dst_multibyte_p (coding);
  let mut charbuf: *mut c_int = (*coding).charbuf;
  let charbuf_end: *mut c_int = charbuf.offset ((*coding).charbuf_used as isize);
  let mut dst: *mut c_uchar = (*coding).destination.offset ((*coding).produced);
  let mut dst_end: *mut c_uchar = (*coding).destination.offset ((*coding).dst_bytes);
  let safe_room: c_int = 8;
  let mut produced_chars: ptrdiff_t = 0;
  let mut attrs: Lisp_Object = lisp_h_Qnil ();
  let mut charset_list: Lisp_Object = lisp_h_Qnil ();
  let mut c: c_int;
  let mut preferred_charset_id: c_int = -1;

  macro_rules! ASSURE_DESTINATION {
    ($bytes:expr) => {
      {
	if dst.offset (($bytes) as isize) >= dst_end
	  {
	    let more_bytes: ptrdiff_t = charbuf_end.offset_from (charbuf) + ($bytes) as isize;

	    dst = alloc_destination (coding, more_bytes, dst);
	    dst_end = (*coding).destination.offset ((*coding).dst_bytes);
	  }
      }
    };
  }
  macro_rules! EMIT_ONE_ASCII_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	*dst = ($c) as c_uchar; dst = dst.offset (1);
      }
    };
  }
  macro_rules! EMIT_ONE_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	if multibytep
	  {
	    let mut ch: c_uint = ($c) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  { *dst = ($c) as c_uchar; dst = dst.offset (1); }
      }
    };
  }
  macro_rules! CODING_CHAR_CHARSET {
    ($coding:expr, $dst:expr, $dst_end:expr, $c:expr, $charset_list:expr, $code_return:expr, $charset:expr) => {
      {
	let offset: ptrdiff_t;

	charset_map_loaded = false;
	$charset = char_charset ($c, $charset_list, $code_return);
	if charset_map_loaded
	    && { offset = coding_change_destination ($coding); offset != 0 }
	  {
	    $dst = $dst.offset (offset);
	    $dst_end = $dst_end.offset (offset);
	  }
      }
    };
  }
  /* `CHAR_CHARSET_P` (charset.h 519-529) is the mirror's now: its body compared the boolean
     `CHARSET_FAST_MAP_REF!` against `0` (unexpandable, `expected bool, found integer`) until
     status 3.333 ported it from this unit's copy; the two use-site workarounds that stood in
     for it here are gone, and the wrapper below calls the mirror's macro.  */

  macro_rules! CODING_CHAR_CHARSET_P {
    ($coding:expr, $dst:expr, $dst_end:expr, $c:expr, $charset:expr, $result:expr) => {
      {
	let offset: ptrdiff_t;

	charset_map_loaded = false;
	$result = CHAR_CHARSET_P! ($c, $charset);
	if charset_map_loaded
	    && { offset = coding_change_destination ($coding); offset != 0 }
	  {
	    $dst = $dst.offset (offset);
	    $dst_end = $dst_end.offset (offset);
	  }
      }
    };
  }
  macro_rules! EMACS_MULE_LEADING_CODES {
    ($id:expr, $codes:expr) => {
      {
	if $id < 0xA0
	  { $codes[0] = $id as c_uchar; $codes[1] = 0; }
	else if $id < 0xE0
	  { $codes[0] = 0x9A; $codes[1] = $id as c_uchar; }
	else if $id < 0xF0
	  { $codes[0] = 0x9B; $codes[1] = $id as c_uchar; }
	else if $id < 0xF5
	  { $codes[0] = 0x9C; $codes[1] = $id as c_uchar; }
	else
	  { $codes[0] = 0x9D; $codes[1] = $id as c_uchar; }
      }
    };
  }

  CODING_GET_INFO! (coding, attrs, charset_list);
  if ! EQ (charset_list, Vemacs_mule_charset_list)
    {
      charset_list = Vemacs_mule_charset_list;
      ASET (attrs, coding_attr_charset_list as isize, charset_list);
    }

  while charbuf < charbuf_end
    {
      ASSURE_DESTINATION! (safe_room);
      c = *charbuf; charbuf = charbuf.offset (1);

      if c < 0
	{
	  /* Handle an annotation.  */
	  if *charbuf == CODING_ANNOTATE_COMPOSITION_MASK as c_int
	    {
	      /* Not yet implemented.  */
	      ()
	    }
	  else if *charbuf == CODING_ANNOTATE_CHARSET_MASK as c_int
	    {
	      preferred_charset_id = *charbuf.offset (3);
	      if preferred_charset_id >= 0
		  && NILP (Fmemq (make_fixnum (preferred_charset_id as isize),
				  charset_list))
		{ preferred_charset_id = -1; }
	    }
	  else
	    { emacs_abort (); }
	  charbuf = charbuf.offset ((-c - 1) as isize);
	  continue;
	}

      if ASCII_CHAR_P (c as isize)
	{ EMIT_ONE_ASCII_BYTE! (c); }
      else if CHAR_BYTE8_P (c)
	{
	  c = CHAR_TO_BYTE8 (c);
	  EMIT_ONE_BYTE! (c);
	}
      else
	{
	  let mut charset: *mut charset = std::ptr::null_mut ();
	  let mut code: c_uint = 0;
	  let mut dimension: c_int;
	  let mut emacs_mule_id: c_int;
	  let mut leading_codes: [c_uchar; 2] = [0; 2];

	  if preferred_charset_id >= 0
	    {
	      let mut result: bool;

	      charset = CHARSET_FROM_ID! (preferred_charset_id);
	      CODING_CHAR_CHARSET_P! (coding, dst, dst_end, c, charset, result);
	      if result
		{ code = ENCODE_CHAR! (charset, c); }
	      else
		{ CODING_CHAR_CHARSET! (coding, dst, dst_end, c, charset_list,
					&raw mut code, charset); }
	    }
	  else
	    { CODING_CHAR_CHARSET! (coding, dst, dst_end, c, charset_list,
				    &raw mut code, charset); }
	  if charset.is_null ()
	    {
	      c = (*coding).default_char;
	      if ASCII_CHAR_P (c as isize)
		{
		  EMIT_ONE_ASCII_BYTE! (c);
		  continue;
		}
	      CODING_CHAR_CHARSET! (coding, dst, dst_end, c, charset_list,
				    &raw mut code, charset);
	    }
	  dimension = CHARSET_DIMENSION! (charset);
	  emacs_mule_id = CHARSET_EMACS_MULE_ID! (charset);
	  EMACS_MULE_LEADING_CODES! (emacs_mule_id, leading_codes);
	  EMIT_ONE_BYTE! (leading_codes[0] as c_int);
	  if leading_codes[1] != 0
	    { EMIT_ONE_BYTE! (leading_codes[1] as c_int); }
	  if dimension == 1
	    { EMIT_ONE_BYTE! ((code | 0x80) as c_int); }
	  else
	    {
	      code |= 0x8080;
	      EMIT_ONE_BYTE! ((code >> 8) as c_int);
	      EMIT_ONE_BYTE! ((code & 0xFF) as c_int);
	    }
	}
    }
  record_conversion_result (coding, CODING_RESULT_SUCCESS);
  (*coding).produced_char += produced_chars;
  (*coding).produced = dst.offset_from ((*coding).destination);
  return false;
}


/*** 7. ISO2022 handlers ***/

static mut iso_code_class: [iso_code_class_type; 256] = [0; 256];

macro_rules! SAFE_CHARSET_P {
  ($coding:expr, $id:expr) => {
    (($id) <= (*$coding).max_charset_id
     && *(*$coding).safe_charsets.offset (($id) as isize) != 255)
  };
}

unsafe fn setup_iso_safe_charsets (attrs: Lisp_Object)
{
  let mut charset_list: Lisp_Object;
  let mut safe_charsets: Lisp_Object;
  let mut request: Lisp_Object;
  let mut reg_usage: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut reg94: EMACS_INT;
  let mut reg96: EMACS_INT;
  let flags: c_int = XFIXNUM (AREF (attrs, coding_attr_iso_flags as isize)) as c_int;
  let mut max_charset_id: c_int;

  charset_list = CODING_ATTR_CHARSET_LIST (attrs);
  if (flags as c_uint & CODING_ISO_FLAG_FULL_SUPPORT as c_uint) != 0
      && ! EQ (charset_list, Viso_2022_charset_list)
    {
      charset_list = Viso_2022_charset_list;
      ASET (attrs, coding_attr_charset_list as isize, charset_list);
      ASET (attrs, coding_attr_safe_charsets as isize, Qnil ());
    }

  if STRINGP (AREF (attrs, coding_attr_safe_charsets as isize))
    { return; }

  max_charset_id = 0;
  tail = charset_list;
  while CONSP (tail)
    {
      let id: c_int = XFIXNUM (XCAR (tail)) as c_int;
      if max_charset_id < id
	{ max_charset_id = id; }
      tail = XCDR (tail);
    }

  safe_charsets = make_uninit_string ((max_charset_id + 1) as EMACS_INT);
  memset (SDATA (safe_charsets) as *mut c_void, 255, (max_charset_id + 1) as usize);
  request = AREF (attrs, coding_attr_iso_request as isize);
  reg_usage = AREF (attrs, coding_attr_iso_usage as isize);
  reg94 = XFIXNUM (XCAR (reg_usage));
  reg96 = XFIXNUM (XCDR (reg_usage));

  tail = charset_list;
  while CONSP (tail)
    {
      let id: Lisp_Object;
      let reg: Lisp_Object;
      let charset: *mut charset;

      id = XCAR (tail);
      charset = CHARSET_FROM_ID! (XFIXNUM (id));
      reg = Fcdr (Fassq (id, request));
      if ! NILP (reg)
	{ SSET (safe_charsets, XFIXNUM (id) as usize, XFIXNUM (reg) as u8); }
      else if CHARSET_ISO_CHARS_96! (charset)
	{
	  if reg96 < 4
	    { SSET (safe_charsets, XFIXNUM (id) as usize, reg96 as u8); }
	}
      else
	{
	  if reg94 < 4
	    { SSET (safe_charsets, XFIXNUM (id) as usize, reg94 as u8); }
	}
      tail = XCDR (tail);
    }
  ASET (attrs, coding_attr_safe_charsets as isize, safe_charsets);
}


/* See the above "GENERAL NOTES on `detect_coding_XXX ()' functions".
   Return true if a text is encoded in one of ISO-2022 based coding
   systems.  */

unsafe extern "C" fn detect_coding_iso_2022 (coding: *mut coding_system,
                                  detect_info: *mut coding_detection_info) -> bool
{
  let mut src: *const c_uchar = (*coding).source;
  let mut src_base: *const c_uchar = src;
  let src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let multibytep = src_multibyte_p (coding);
  let mut single_shifting: bool = false;
  let mut id: c_int = 0;
  let mut c: c_int;
  let mut c1: c_int;
  let mut consumed_chars: ptrdiff_t = 0;
  let mut i: c_int;
  let mut rejected: c_int = 0;
  let mut found: c_int = 0;
  let mut composition_count: c_int = -1;

  (*detect_info).checked |= CATEGORY_MASK_ISO;

  i = coding_category_iso_7;
  while i <= coding_category_iso_8_else
    {
      let this: *mut coding_system = &raw mut coding_categories[i as usize];
      let mut attrs: Lisp_Object;
      let mut val: Lisp_Object;

      if (*this).id < 0
	{ i += 1; continue; }
      attrs = CODING_ID_ATTRS ((*this).id as hash_idx_t);
      if (CODING_ISO_FLAGS! (this) & CODING_ISO_FLAG_FULL_SUPPORT as c_uint) != 0
	  && ! EQ (CODING_ATTR_CHARSET_LIST (attrs), Viso_2022_charset_list)
	{ setup_iso_safe_charsets (attrs); }
      val = CODING_ATTR_SAFE_CHARSETS (attrs);
      (*this).max_charset_id = SCHARS (val) as c_int - 1;
      (*this).safe_charsets = SDATA (val);
      i += 1;
    }

  /* A coding system of this category is always ASCII compatible.  */
  src = src.offset ((*coding).head_ascii);

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }

    'detect_loop: while rejected != CATEGORY_MASK_ISO
      {
	src_base = src;
	ONE_MORE_BYTE! (c);
	'check_extra_latin: {
	  match c
	    {
	    ISO_CODE_ESC =>
	      {
		if inhibit_iso_escape_detection
		  { break 'check_extra_latin; }
		single_shifting = false;
		ONE_MORE_BYTE! (c);
		if c == 'N' as c_int || c == 'O' as c_int
		  {
		    /* ESC <Fe> for SS2 or SS3.  */
		    single_shifting = true;
		    rejected |= CATEGORY_MASK_ISO_7BIT | CATEGORY_MASK_ISO_8BIT;
		  }
		else if c == '1' as c_int
		  {
		    /* End of composition.  */
		    if composition_count < 0
			|| composition_count > MAX_COMPOSITION_COMPONENTS
		      /* Invalid */
		      { break 'check_extra_latin; }
		    composition_count = -1;
		    found |= CATEGORY_MASK_ISO;
		  }
		else if c >= '0' as c_int && c <= '4' as c_int
		  {
		    /* ESC <Fp> for start/end composition.  */
		    composition_count = 0;
		  }
		else
		  {
		    if c >= '(' as c_int && c <= '/' as c_int
		      {
			/* Designation sequence for a charset of dimension 1.  */
			ONE_MORE_BYTE! (c1);
			id = iso_charset_table[0][(c >= ',' as c_int) as usize][c1 as usize];
			if c1 < ' ' as c_int || c1 >= 0x80
			    || id < 0
			  {
			    /* Invalid designation sequence.  Just ignore.  */
			    if c1 >= 0x80
			      { rejected |= (CATEGORY_MASK_ISO_7BIT
					     | CATEGORY_MASK_ISO_7_ELSE); }
			    break 'check_extra_latin;
			  }
		      }
		    else if c == '$' as c_int
		      {
			/* Designation sequence for a charset of dimension 2.  */
			ONE_MORE_BYTE! (c);
			if c >= '@' as c_int && c <= 'B' as c_int
			  /* Designation for JISX0208.1978, GB2312, or JISX0208.  */
			  { id = iso_charset_table[1][0][c as usize]; }
			else if c >= '(' as c_int && c <= '/' as c_int
			  {
			    ONE_MORE_BYTE! (c1);
			    id = iso_charset_table[1][(c >= ',' as c_int) as usize][c1 as usize];
			    if c1 < ' ' as c_int || c1 >= 0x80
				|| id < 0
			      {
				/* Invalid designation sequence.  Just ignore.  */
				if c1 >= 0x80
				  { rejected |= (CATEGORY_MASK_ISO_7BIT
						 | CATEGORY_MASK_ISO_7_ELSE); }
				break 'check_extra_latin;
			      }
			  }
			else
			  {
			    /* Invalid designation sequence.  Just ignore it.  */
			    if c >= 0x80
			      { rejected |= (CATEGORY_MASK_ISO_7BIT
					     | CATEGORY_MASK_ISO_7_ELSE); }
			    break 'check_extra_latin;
			  }
		      }
		    else
		      {
			/* Invalid escape sequence.  Just ignore it.  */
			if c >= 0x80
			  { rejected |= (CATEGORY_MASK_ISO_7BIT
					 | CATEGORY_MASK_ISO_7_ELSE); }
			break 'check_extra_latin;
		      }

		    /* We found a valid designation sequence for CHARSET.  */
		    rejected |= CATEGORY_MASK_ISO_8BIT;
		    if SAFE_CHARSET_P! (&raw mut coding_categories[coding_category_iso_7 as usize],
					id)
		      { found |= CATEGORY_MASK_ISO_7; }
		    else
		      { rejected |= CATEGORY_MASK_ISO_7; }
		    if SAFE_CHARSET_P! (&raw mut coding_categories[coding_category_iso_7_tight as usize],
					id)
		      { found |= CATEGORY_MASK_ISO_7_TIGHT; }
		    else
		      { rejected |= CATEGORY_MASK_ISO_7_TIGHT; }
		    if SAFE_CHARSET_P! (&raw mut coding_categories[coding_category_iso_7_else as usize],
					id)
		      { found |= CATEGORY_MASK_ISO_7_ELSE; }
		    else
		      { rejected |= CATEGORY_MASK_ISO_7_ELSE; }
		    if SAFE_CHARSET_P! (&raw mut coding_categories[coding_category_iso_8_else as usize],
					id)
		      { found |= CATEGORY_MASK_ISO_8_ELSE; }
		    else
		      { rejected |= CATEGORY_MASK_ISO_8_ELSE; }
		  }
		break 'check_extra_latin;
	      }

	    ISO_CODE_SO | ISO_CODE_SI =>
	      {
		/* Locking shift out/in.  */
		if inhibit_iso_escape_detection
		  { break 'check_extra_latin; }
		single_shifting = false;
		rejected |= CATEGORY_MASK_ISO_7BIT | CATEGORY_MASK_ISO_8BIT;
		break 'check_extra_latin;
	      }

	    ISO_CODE_CSI =>
	      {
		/* Control sequence introducer.  */
		single_shifting = false;
		rejected |= CATEGORY_MASK_ISO_7BIT | CATEGORY_MASK_ISO_7_ELSE;
		found |= CATEGORY_MASK_ISO_8_ELSE;
		break 'check_extra_latin;
	      }

	    ISO_CODE_SS2 | ISO_CODE_SS3 =>
	      {
		/* Single shift.   */
		if inhibit_iso_escape_detection
		  { break 'check_extra_latin; }
		single_shifting = false;
		rejected |= CATEGORY_MASK_ISO_7BIT | CATEGORY_MASK_ISO_7_ELSE;
		if (CODING_ISO_FLAGS! (&raw mut coding_categories[coding_category_iso_8_1 as usize])
		    & CODING_ISO_FLAG_SINGLE_SHIFT as c_uint) != 0
		  {
		    found |= CATEGORY_MASK_ISO_8_1;
		    single_shifting = true;
		  }
		if (CODING_ISO_FLAGS! (&raw mut coding_categories[coding_category_iso_8_2 as usize])
		    & CODING_ISO_FLAG_SINGLE_SHIFT as c_uint) != 0
		  {
		    found |= CATEGORY_MASK_ISO_8_2;
		    single_shifting = true;
		  }
		if single_shifting
		  { break 'check_extra_latin; }
		break 'check_extra_latin;
	      }

	    _ =>
	      {
		if c < 0
		  { continue 'detect_loop; }
		if c < 0x80
		  {
		    if composition_count >= 0
		      { composition_count += 1; }
		    single_shifting = false;
		    break 'check_extra_latin;
		  }
		rejected |= CATEGORY_MASK_ISO_7BIT | CATEGORY_MASK_ISO_7_ELSE;
		if c >= 0xA0
		  {
		    found |= CATEGORY_MASK_ISO_8_1;
		    /* Check the length of succeeding codes of the range
		       0xA0..0FF.  If the byte length is even, we include
		       CATEGORY_MASK_ISO_8_2 in `found'.  We can check this
		       only when we are not single shifting.  */
		    if ! single_shifting
			&& (rejected & CATEGORY_MASK_ISO_8_2) == 0
		      {
			let mut len: ptrdiff_t = 1;
			while src < src_end
			  {
			    src_base = src;
			    ONE_MORE_BYTE! (c);
			    if c < 0xA0
			      {
				src = src_base;
				break;
			      }
			    len += 1;
			  }

			if len & 1 != 0 && src < src_end
			  {
			    rejected |= CATEGORY_MASK_ISO_8_2;
			    if composition_count >= 0
			      { composition_count += len as c_int; }
			  }
			else
			  {
			    found |= CATEGORY_MASK_ISO_8_2;
			    if composition_count >= 0
			      { composition_count += (len / 2) as c_int; }
			  }
		      }
		    break 'check_extra_latin;
		  }
		/* check_extra_latin: */
		if ! VECTORP (Vlatin_extra_code_table)
		    || NILP (AREF (Vlatin_extra_code_table, c as isize))
		  {
		    rejected = CATEGORY_MASK_ISO;
		    break 'check_extra_latin;
		  }
		if (CODING_ISO_FLAGS! (&raw mut coding_categories[coding_category_iso_8_1 as usize])
		    & CODING_ISO_FLAG_LATIN_EXTRA as c_uint) != 0
		  { found |= CATEGORY_MASK_ISO_8_1; }
		else
		  { rejected |= CATEGORY_MASK_ISO_8_1; }
		rejected |= CATEGORY_MASK_ISO_8_2;
		break 'check_extra_latin;
	      }
	    }
	}
      }
    (*detect_info).rejected |= CATEGORY_MASK_ISO;
    return false;
  }

  (*detect_info).rejected |= rejected;
  (*detect_info).found |= (found & !rejected);
  return true;
}

/* Finish the current composition as invalid.  */

unsafe fn finish_composition (charbuf: *mut c_int, cmp_status: *mut composition_status) -> c_int
{
  let mut idx: c_int = - (*cmp_status).length;
  let mut new_chars: c_int;

  macro_rules! ENCODE_COMPOSITION_RULE {
    ($rule:expr) => {
      {
	let gref: c_int = ($rule % 0x100) / 12;
	let nref: c_int = ($rule % 0x100) % 12;

	if ($rule) < 0x100		/* old format */
	  {
	    let mut gref: c_int = gref;
	    let mut nref: c_int = nref;
	    if gref == 10 { gref = 4; }
	    if nref == 10 { nref = 4; }
	    *charbuf.offset (idx as isize) = 32 + gref * 9 + nref;
	    *charbuf.offset ((idx + 1) as isize) = -1;
	    new_chars += 1;
	  }
	else				/* new format */
	  {
	    *charbuf.offset (idx as isize) = 32 + 81 + gref;
	    *charbuf.offset ((idx + 1) as isize) = 32 + nref;
	    new_chars += 2;
	  }
      }
    };
  }

  /* Recover the original ESC sequence */
  *charbuf.offset (idx as isize) = ISO_CODE_ESC; idx += 1;
  *charbuf.offset (idx as isize) = (if (*cmp_status).method == COMPOSITION_RELATIVE { '0' as c_int }
				   else if (*cmp_status).method == COMPOSITION_WITH_RULE { '2' as c_int }
				   else if (*cmp_status).method == COMPOSITION_WITH_ALTCHARS { '3' as c_int }
				   /* cmp_status->method == COMPOSITION_WITH_RULE_ALTCHARS */
				   else { '4' as c_int }); idx += 1;
  *charbuf.offset (idx as isize) = -2; idx += 1;
  *charbuf.offset (idx as isize) = 0; idx += 1;
  *charbuf.offset (idx as isize) = -1; idx += 1;
  new_chars = (*cmp_status).nchars;
  if (*cmp_status).method >= COMPOSITION_WITH_RULE
    {
      while idx < 0
	{
	  let elt: c_int = *charbuf.offset (idx as isize);

	  if elt == -2
	    {
	      ENCODE_COMPOSITION_RULE! (*charbuf.offset ((idx + 1) as isize));
	      idx += 1;
	    }
	  else if elt == -1
	    {
	      *charbuf.offset (idx as isize) = ISO_CODE_ESC; idx += 1;
	      *charbuf.offset (idx as isize) = '0' as c_int;
	      new_chars += 2;
	    }
	  idx += 1;
	}
    }
  (*cmp_status).state = COMPOSING_NO;
  return new_chars;
}

unsafe extern "C" fn decode_coding_iso_2022 (coding: *mut coding_system)
{
  let mut src: *const c_uchar = (*coding).source.offset ((*coding).consumed);
  let mut src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let mut src_base: *const c_uchar = std::ptr::null ();
  let mut charbuf: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_used as isize);
  /* We may produce two annotations (charset and composition) in one
     loop and one more charset annotation at the end.  */
  let charbuf_end: *mut c_int
    = (*coding).charbuf.offset (((*coding).charbuf_size - (MAX_ANNOTATION_LENGTH * 3)) as isize);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut consumed_chars_base: ptrdiff_t = 0;
  let multibytep = src_multibyte_p (coding);
  /* Charsets invoked to graphic plane 0 and 1 respectively.  */
  let mut charset_id_0: c_int = CODING_ISO_INVOKED_CHARSET! (coding, 0);
  let mut charset_id_1: c_int = CODING_ISO_INVOKED_CHARSET! (coding, 1);
  let mut charset_id_2: c_int = 0;
  let mut charset_id_3: c_int = 0;
  let mut charset: *mut charset = std::ptr::null_mut ();
  let mut c: c_int;
  let cmp_status: *mut composition_status = CODING_ISO_CMP_STATUS! (coding);
  let attrs: Lisp_Object = CODING_ID_ATTRS ((*coding).id as hash_idx_t);
  let mut char_offset: ptrdiff_t = (*coding).produced_char;
  let mut last_offset: ptrdiff_t = char_offset;
  let mut last_id: c_int = charset_ascii;
  let eol_dos
    = !inhibit_eol_conversion && EQ (CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t), Qdos ());
  let mut byte_after_cr: c_int = -1;
  let mut i: c_int;

  setup_iso_safe_charsets (attrs);
  (*coding).safe_charsets = SDATA (CODING_ATTR_SAFE_CHARSETS (attrs));

  if (*cmp_status).state != COMPOSING_NO
    {
      if charbuf_end.offset_from (charbuf) < (*cmp_status).length as isize
	{ emacs_abort (); }
      i = 0;
      while i < (*cmp_status).length
	{
	  *charbuf = (*cmp_status).carryover[i as usize] as c_int; charbuf = charbuf.offset (1);
	  i += 1;
	}
      coding_bits_set (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1, 1);
    }

    macro_rules! ADD_ANNOTATION_DATA {
      ($buf:expr, $len:expr, $mask:expr, $nchars:expr) => {
	{
	  *$buf = -($len); $buf = $buf.offset (1);
	  *$buf = ($mask); $buf = $buf.offset (1);
	  *$buf = ($nchars) as c_int; $buf = $buf.offset (1);
	  coding_bits_set (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1, 1);
	}
      };
    }
    macro_rules! ADD_COMPOSITION_DATA {
      ($buf:expr, $nchars:expr, $nbytes:expr, $method:expr) => {
	{
	  ADD_ANNOTATION_DATA! ($buf, 5, CODING_ANNOTATE_COMPOSITION_MASK as c_int, $nchars);
	  *$buf = $nbytes; $buf = $buf.offset (1);
	  *$buf = $method; $buf = $buf.offset (1);
	}
      };
    }
    macro_rules! ADD_CHARSET_DATA {
      ($buf:expr, $nchars:expr, $id:expr) => {
	{
	  ADD_ANNOTATION_DATA! ($buf, 4, CODING_ANNOTATE_CHARSET_MASK as c_int, $nchars);
	  *$buf = $id; $buf = $buf.offset (1);
	}
      };
    }
	  macro_rules! MAYBE_FINISH_COMPOSITION {
	    () => {
	      {
		if (*cmp_status).state != COMPOSING_NO
		  { char_offset += finish_composition (charbuf, cmp_status) as ptrdiff_t; }
	      }
	    };
	  }
  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }
    macro_rules! CODING_DECODE_CHAR {
      ($coding:expr, $src:expr, $src_base:expr, $src_end:expr, $charset:expr, $code:expr, $c:expr) => {
	{
	  let offset: ptrdiff_t;

	  charset_map_loaded = false;
	  $c = DECODE_CHAR! ($charset, $code);
	  if charset_map_loaded
	      && { offset = coding_change_source ($coding); offset != 0 }
	    {
	      $src = $src.offset (offset);
	      $src_base = $src_base.offset (offset);
	      $src_end = $src_end.offset (offset);
	    }
	}
      };
    }

    'decode_loop: loop
      {
	let mut c1: c_int;
	let mut c2: c_int;
	let mut c3: c_int;
	/* Set by the arms C jumps out of with `goto label_escape_sequence`.  */
	let mut escape_sequence_p: bool = false;

	'invalid_code: {

	  macro_rules! DECODE_DESIGNATION {
	    ($reg:expr, $dim:expr, $chars_96:ident, $final:expr) => {
	      {
		let mut id: c_int;
		let mut prev: c_int;

		id = ISO_CHARSET_TABLE! ($dim, $chars_96, $final);
		if $final < '0' as c_int || $final >= 128
		    || id < 0
		    || ! SAFE_CHARSET_P! (coding, id)
		  {
		    CODING_ISO_DESIGNATION! (coding, $reg) = -2;
		    $chars_96 = -1;
		    break 'invalid_code;
		  }
		prev = CODING_ISO_DESIGNATION! (coding, $reg);
		if id == charset_jisx0201_roman
		  {
		    if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_USE_ROMAN as c_uint) != 0
		      { id = charset_ascii; }
		  }
		else if id == charset_jisx0208_1978
		  {
		    if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_USE_OLDJIS as c_uint) != 0
		      { id = charset_jisx0208; }
		  }
		CODING_ISO_DESIGNATION! (coding, $reg) = id;
		/* If there was an invalid designation to REG previously, and this
		   designation is ASCII to REG, we should keep this designation
		   sequence.  */
		if prev == -2 && id == charset_ascii
		  { $chars_96 = -1; }
	      }
	    };
	  }
	  macro_rules! DECODE_COMPOSITION_RULE {
	    ($rule:ident) => {
	      {
		$rule = c1 - 32;
		if $rule < 0
		  { break 'invalid_code; }
		if $rule < 81		/* old format (before ver.21) */
		  {
		    let mut gref: c_int = ($rule) / 9;
		    let mut nref: c_int = ($rule) % 9;
		    if gref == 4 { gref = 10; }
		    if nref == 4 { nref = 10; }
		    $rule = COMPOSITION_ENCODE_RULE (gref, nref);
		  }
		else			/* new format (after ver.21) */
		  {
		    let mut b: c_int;

		    ONE_MORE_BYTE! (b);
		    if ! COMPOSITION_ENCODE_RULE_VALID ($rule - 81, b - 32)
		      { break 'invalid_code; }
		    $rule = COMPOSITION_ENCODE_RULE ($rule - 81, b - 32);
		    $rule += 0x100;   /* Distinguish it from the old format.  */
		  }
	      }
	    };
	  }
	  macro_rules! DECODE_COMPOSITION_START {
	    ($c1:expr) => {
	      {
		if $c1 == '0' as c_int
		    && (((*cmp_status).state == COMPOSING_COMPONENT_CHAR
			 && (*cmp_status).method == COMPOSITION_WITH_ALTCHARS)
			|| ((*cmp_status).state == COMPOSING_COMPONENT_RULE
			    && (*cmp_status).method == COMPOSITION_WITH_RULE_ALTCHARS))
		  {
		    *charbuf = -1; charbuf = charbuf.offset (1);
		    *charbuf = -1; charbuf = charbuf.offset (1);
		    (*cmp_status).state = COMPOSING_CHAR;
		    (*cmp_status).length += 2;
		  }
		else
		  {
		    MAYBE_FINISH_COMPOSITION! ();
		    (*cmp_status).method = (if $c1 == '0' as c_int { COMPOSITION_RELATIVE }
					    else if $c1 == '2' as c_int { COMPOSITION_WITH_RULE }
					    else if $c1 == '3' as c_int { COMPOSITION_WITH_ALTCHARS }
					    else { COMPOSITION_WITH_RULE_ALTCHARS });
		    (*cmp_status).state
		      = (if $c1 <= '2' as c_int { COMPOSING_CHAR } else { COMPOSING_COMPONENT_CHAR });
		    ADD_COMPOSITION_DATA! (charbuf, 0, 0, (*cmp_status).method);
		    (*cmp_status).length = MAX_ANNOTATION_LENGTH;
		    (*cmp_status).nchars = 0; (*cmp_status).ncomps = 0;
		    coding_bits_set (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1, 1);
		  }
	      }
	    };
	  }
	  macro_rules! DECODE_COMPOSITION_END {
	    () => {
	      {
		if (*cmp_status).nchars == 0
		    || (((*cmp_status).state == COMPOSING_CHAR)
			== ((*cmp_status).method == COMPOSITION_WITH_RULE))
		  {
		    MAYBE_FINISH_COMPOSITION! ();
		    break 'invalid_code;
		  }
		if (*cmp_status).method == COMPOSITION_WITH_ALTCHARS
		  { *charbuf.offset (- ((*cmp_status).length as isize)) -= (*cmp_status).ncomps + 2; }
		else if (*cmp_status).method == COMPOSITION_WITH_RULE_ALTCHARS
		  { *charbuf.offset (- ((*cmp_status).length as isize)) -= (*cmp_status).ncomps * 3; }
		*charbuf.offset (- ((*cmp_status).length as isize) + 2) = (*cmp_status).nchars;
		char_offset += (*cmp_status).nchars as ptrdiff_t;
		(*cmp_status).state = COMPOSING_NO;
	      }
	    };
	  }
	  macro_rules! STORE_COMPOSITION_RULE {
	    ($rule:expr) => {
	      {
		*charbuf = -2; charbuf = charbuf.offset (1);
		*charbuf = $rule; charbuf = charbuf.offset (1);
		(*cmp_status).length += 2;
		(*cmp_status).state -= 1;
	      }
	    };
	  }
	  macro_rules! STORE_COMPOSITION_CHAR {
	    ($c:expr) => {
	      {
		*charbuf = ($c); charbuf = charbuf.offset (1);
		(*cmp_status).length += 1;
		if (*cmp_status).state == COMPOSING_CHAR
		  { (*cmp_status).nchars += 1; }
		else
		  { (*cmp_status).ncomps += 1; }
		if (*cmp_status).method == COMPOSITION_WITH_RULE
		    || ((*cmp_status).method == COMPOSITION_WITH_RULE_ALTCHARS
			&& (*cmp_status).state == COMPOSING_COMPONENT_CHAR)
		  { (*cmp_status).state += 1; }
	      }
	    };
	  }

	  src_base = src;
	  consumed_chars_base = consumed_chars;

	  if charbuf >= charbuf_end
	    {
	      if byte_after_cr >= 0
		{ src_base = src_base.offset (-1); }
	      break 'decode_loop;
	    }

	  if byte_after_cr >= 0
	    { c1 = byte_after_cr; byte_after_cr = -1; }
	  else
	    { ONE_MORE_BYTE! (c1); }
	  if c1 < 0
	    { break 'invalid_code; }

	  if CODING_ISO_EXTSEGMENT_LEN! (coding) > 0
	    {
	      *charbuf = if ASCII_CHAR_P (c1 as isize) { c1 } else { BYTE8_TO_CHAR (c1) };
	      charbuf = charbuf.offset (1);
	      char_offset += 1;
	      CODING_ISO_EXTSEGMENT_LEN! (coding) -= 1;
	      continue 'decode_loop;
	    }

	  if CODING_ISO_EMBEDDED_UTF_8! (coding).get () > 0
	    {
	      if c1 == ISO_CODE_ESC
		{
		  if src.offset (1) >= src_end
		    {
		      record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC);
		      break 'no_more_source;
		    }
		  *charbuf = ISO_CODE_ESC; charbuf = charbuf.offset (1);
		  char_offset += 1;
		  if *src.offset (0) as c_int == '%' as c_int
		      && *src.offset (1) as c_int == '@' as c_int
		    {
		      src = src.offset (2);
		      consumed_chars += 2;
		      char_offset += 2;
		      /* We are sure charbuf can contain two more chars. */
		      *charbuf = '%' as c_int; charbuf = charbuf.offset (1);
		      *charbuf = '@' as c_int; charbuf = charbuf.offset (1);
		      CODING_ISO_EMBEDDED_UTF_8! (coding).set (0);
		    }
		}
	      else
		{
		  *charbuf = if ASCII_CHAR_P (c1 as isize) { c1 } else { BYTE8_TO_CHAR (c1) };
		  charbuf = charbuf.offset (1);
		  char_offset += 1;
		}
	      continue 'decode_loop;
	    }

	  if ((*cmp_status).state == COMPOSING_RULE
	      || (*cmp_status).state == COMPOSING_COMPONENT_RULE)
	      && c1 != ISO_CODE_ESC
	    {
	      let mut rule: c_int;

	      DECODE_COMPOSITION_RULE! (rule);
	      STORE_COMPOSITION_RULE! (rule);
	      continue 'decode_loop;
	    }

	  /* We produce at most one character.  */
	  match iso_code_class[c1 as usize]
	    {
	    ISO_0x20_or_0x7F =>
	      {
		if charset_id_0 < 0
		    || ! CHARSET_ISO_CHARS_96! (CHARSET_FROM_ID! (charset_id_0))
		  /* This is SPACE or DEL.  */
		  { charset = CHARSET_FROM_ID! (charset_ascii); }
		else
		  { charset = CHARSET_FROM_ID! (charset_id_0); }
	      }

	    ISO_graphic_plane_0 =>
	      {
		if charset_id_0 < 0
		  { charset = CHARSET_FROM_ID! (charset_ascii); }
		else
		  { charset = CHARSET_FROM_ID! (charset_id_0); }
	      }

	    ISO_0xA0_or_0xFF | ISO_graphic_plane_1 =>
	      {
		if iso_code_class[c1 as usize] == ISO_0xA0_or_0xFF
		  {
		    if charset_id_1 < 0
			|| ! CHARSET_ISO_CHARS_96! (CHARSET_FROM_ID! (charset_id_1))
			|| (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SEVEN_BITS as c_uint) != 0
		      { break 'invalid_code; }
		    /* This is a graphic character, we fall down ... */
		  }
		if charset_id_1 < 0
		  { break 'invalid_code; }
		charset = CHARSET_FROM_ID! (charset_id_1);
	      }

	    ISO_control_0 =>
	      {
		if eol_dos && c1 == '\r' as c_int
		  { ONE_MORE_BYTE! (byte_after_cr); }
		MAYBE_FINISH_COMPOSITION! ();
		charset = CHARSET_FROM_ID! (charset_ascii);
	      }

	    ISO_control_1 =>
	      { break 'invalid_code; }

	    ISO_shift_out =>
	      {
		if ((CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_LOCKING_SHIFT as c_uint) == 0)
		    || CODING_ISO_DESIGNATION! (coding, 1) < 0
		  { break 'invalid_code; }
		CODING_ISO_INVOCATION! (coding, 0) = 1;
		charset_id_0 = CODING_ISO_INVOKED_CHARSET! (coding, 0);
		continue 'decode_loop;
	      }

	    ISO_shift_in =>
	      {
		if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_LOCKING_SHIFT as c_uint) == 0
		  { break 'invalid_code; }
		CODING_ISO_INVOCATION! (coding, 0) = 0;
		charset_id_0 = CODING_ISO_INVOKED_CHARSET! (coding, 0);
		continue 'decode_loop;
	      }

	    ISO_single_shift_2_7 | ISO_single_shift_2 =>
	      {
		if iso_code_class[c1 as usize] == ISO_single_shift_2_7
		  {
		    if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SEVEN_BITS as c_uint) == 0
		      { break 'invalid_code; }
		  }
		if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SINGLE_SHIFT as c_uint) == 0
		  { break 'invalid_code; }
		/* SS2 is handled as an escape sequence of ESC 'N' */
		c1 = 'N' as c_int;
		escape_sequence_p = true;
	      }

	    ISO_single_shift_3 =>
	      {
		if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SINGLE_SHIFT as c_uint) == 0
		  { break 'invalid_code; }
		/* SS2 is handled as an escape sequence of ESC 'O' */
		c1 = 'O' as c_int;
		escape_sequence_p = true;
	      }

	    ISO_control_sequence_introducer =>
	      {
		/* CSI is handled as an escape sequence of ESC '[' ...  */
		c1 = '[' as c_int;
		escape_sequence_p = true;
	      }

	    ISO_escape =>
	      {
		ONE_MORE_BYTE! (c1);
		escape_sequence_p = true;
	      }

	    _ =>
	      { emacs_abort (); }
	    }

	  if escape_sequence_p
	    {
	      /* label_escape_sequence: */
	      'label_escape_sequence: loop
		{
		  /* Escape sequences handled here are invocation,
		     designation, direction specification, and character
		     composition specification.  */
		  match c1
		    {
		    _ if c1 == '&' as c_int =>	/* revision of following character set */
		      {
			ONE_MORE_BYTE! (c1);
			if ! (c1 >= '@' as c_int && c1 <= '~' as c_int)
			  { break 'invalid_code; }
			ONE_MORE_BYTE! (c1);
			if c1 != ISO_CODE_ESC
			  { break 'invalid_code; }
			ONE_MORE_BYTE! (c1);
			continue 'label_escape_sequence;
		      }

		    _ if c1 == '$' as c_int =>	/* designation of 2-byte character set */
		      {
			if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_DESIGNATION as c_uint) == 0
			  { break 'invalid_code; }
			{
			  let mut reg: c_int = 0;
			  let mut chars96: c_int = 0;

			  ONE_MORE_BYTE! (c1);
			  if c1 >= '@' as c_int && c1 <= 'B' as c_int
			    {	/* designation of JISX0208.1978, GB2312.1980,
				   or JISX0208.1980 */
			      reg = 0; chars96 = 0;
			    }
			  else if c1 >= 0x28 && c1 <= 0x2B
			    { /* designation of DIMENSION2_CHARS94 character set */
			      reg = c1 - 0x28; chars96 = 0;
			      ONE_MORE_BYTE! (c1);
			    }
			  else if c1 >= 0x2C && c1 <= 0x2F
			    { /* designation of DIMENSION2_CHARS96 character set */
			      reg = c1 - 0x2C; chars96 = 1;
			      ONE_MORE_BYTE! (c1);
			    }
			  else
			    { break 'invalid_code; }
			  DECODE_DESIGNATION! (reg, 2, chars96, c1);
			  /* We must update these variables now.  */
			  if reg == 0
			    { charset_id_0 = CODING_ISO_INVOKED_CHARSET! (coding, 0); }
			  else if reg == 1
			    { charset_id_1 = CODING_ISO_INVOKED_CHARSET! (coding, 1); }
			  if chars96 < 0
			    { break 'invalid_code; }
			}
			continue 'decode_loop;
		      }

		    _ if c1 == 'n' as c_int =>	/* invocation of locking-shift-2 */
		      {
			if ((CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_LOCKING_SHIFT as c_uint) == 0)
			    || CODING_ISO_DESIGNATION! (coding, 2) < 0
			  { break 'invalid_code; }
			CODING_ISO_INVOCATION! (coding, 0) = 2;
			charset_id_0 = CODING_ISO_INVOKED_CHARSET! (coding, 0);
			continue 'decode_loop;
		      }

		    _ if c1 == 'o' as c_int =>	/* invocation of locking-shift-3 */
		      {
			if ((CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_LOCKING_SHIFT as c_uint) == 0)
			    || CODING_ISO_DESIGNATION! (coding, 3) < 0
			  { break 'invalid_code; }
			CODING_ISO_INVOCATION! (coding, 0) = 3;
			charset_id_0 = CODING_ISO_INVOKED_CHARSET! (coding, 0);
			continue 'decode_loop;
		      }

		    _ if c1 == 'N' as c_int =>	/* invocation of single-shift-2 */
		      {
			if ((CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SINGLE_SHIFT as c_uint) == 0)
			    || CODING_ISO_DESIGNATION! (coding, 2) < 0
			  { break 'invalid_code; }
			charset_id_2 = CODING_ISO_DESIGNATION! (coding, 2);
			if charset_id_2 < 0
			  { charset = CHARSET_FROM_ID! (charset_ascii); }
			else
			  { charset = CHARSET_FROM_ID! (charset_id_2); }
			ONE_MORE_BYTE! (c1);
			if c1 < 0x20 || (c1 >= 0x80 && c1 < 0xA0)
			    || ((CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SEVEN_BITS as c_uint) == 0
				&& (if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_LEVEL_4 as c_uint) != 0
				      { c1 >= 0x80 } else { c1 < 0x80 }))
			  { break 'invalid_code; }
			break;
		      }

		    _ if c1 == 'O' as c_int =>	/* invocation of single-shift-3 */
		      {
			if ((CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SINGLE_SHIFT as c_uint) == 0)
			    || CODING_ISO_DESIGNATION! (coding, 3) < 0
			  { break 'invalid_code; }
			charset_id_3 = CODING_ISO_DESIGNATION! (coding, 3);
			if charset_id_3 < 0
			  { charset = CHARSET_FROM_ID! (charset_ascii); }
			else
			  { charset = CHARSET_FROM_ID! (charset_id_3); }
			ONE_MORE_BYTE! (c1);
			if c1 < 0x20 || (c1 >= 0x80 && c1 < 0xA0)
			    || ((CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SEVEN_BITS as c_uint) == 0
				&& (if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_LEVEL_4 as c_uint) != 0
				      { c1 >= 0x80 } else { c1 < 0x80 }))
			  { break 'invalid_code; }
			break;
		      }

		    _ if c1 == '0' as c_int || c1 == '2' as c_int
			  || c1 == '3' as c_int || c1 == '4' as c_int => /* start composition */
		      {
			if (common_flags_of (coding) & CODING_ANNOTATE_COMPOSITION_MASK) == 0
			  { break 'invalid_code; }
			if last_id != charset_ascii
			  {
			    ADD_CHARSET_DATA! (charbuf, char_offset - last_offset, last_id);
			    last_id = charset_ascii;
			    last_offset = char_offset;
			  }
			DECODE_COMPOSITION_START! (c1);
			continue 'decode_loop;
		      }

		    _ if c1 == '1' as c_int =>	/* end composition */
		      {
			if (*cmp_status).state == COMPOSING_NO
			  { break 'invalid_code; }
			DECODE_COMPOSITION_END! ();
			continue 'decode_loop;
		      }

		    _ if c1 == '[' as c_int =>	/* specification of direction */
		      {
			if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_DIRECTION as c_uint) == 0
			  { break 'invalid_code; }
			/* For the moment, nested direction is not supported.
			   So, `coding->mode & CODING_MODE_DIRECTION' zero means
			   left-to-right, and nonzero means right-to-left.  */
			ONE_MORE_BYTE! (c1);
			'escape_direction: {
			  if c1 == ']' as c_int	/* end of the current direction */
			    { coding_mode_clear (coding, CODING_MODE_DIRECTION); }
			  else if c1 == '0' as c_int	/* end of the current direction */
				  || c1 == '1' as c_int	/* start of left-to-right direction */
			    {
			      ONE_MORE_BYTE! (c1);
			      if c1 == ']' as c_int
				{ coding_mode_clear (coding, CODING_MODE_DIRECTION); }
			      else
				{ break 'invalid_code; }
			    }
			  else if c1 == '2' as c_int	/* start of right-to-left direction */
			    {
			      ONE_MORE_BYTE! (c1);
			      if c1 == ']' as c_int
				{ coding_mode_set (coding, CODING_MODE_DIRECTION); }
			      else
				{ break 'invalid_code; }
			    }
			  else
			    { break 'invalid_code; }
			}
			continue 'decode_loop;
		      }

		    _ if c1 == '%' as c_int =>
		      {
			ONE_MORE_BYTE! (c1);
			if c1 == '/' as c_int
			  {
			    /* CTEXT extended segment:
			       ESC % / [0-4] M L --ENCODING-NAME-- \002 --BYTES--
			       We keep these bytes as is for the moment.
			       They may be decoded by post-read-conversion.  */
			    let mut dim: c_int;
			    let mut M: c_int;
			    let mut L: c_int;
			    let mut size: c_int;

			    ONE_MORE_BYTE! (dim);
			    if dim < '0' as c_int || dim > '4' as c_int
			      { break 'invalid_code; }
			    ONE_MORE_BYTE! (M);
			    if M < 128
			      { break 'invalid_code; }
			    ONE_MORE_BYTE! (L);
			    if L < 128
			      { break 'invalid_code; }
			    size = ((M - 128) * 128) + (L - 128);
			    if charbuf.offset (6) > charbuf_end
			      { break 'decode_loop; }
			    *charbuf = ISO_CODE_ESC; charbuf = charbuf.offset (1);
			    *charbuf = '%' as c_int; charbuf = charbuf.offset (1);
			    *charbuf = '/' as c_int; charbuf = charbuf.offset (1);
			    *charbuf = dim; charbuf = charbuf.offset (1);
			    *charbuf = BYTE8_TO_CHAR (M); charbuf = charbuf.offset (1);
			    *charbuf = BYTE8_TO_CHAR (L); charbuf = charbuf.offset (1);
			    CODING_ISO_EXTSEGMENT_LEN! (coding) = size;
			  }
			else if c1 == 'G' as c_int
			  {
			    /* XFree86 extension for embedding UTF-8 in CTEXT:
			       ESC % G --UTF-8-BYTES-- ESC % @
			       We keep these bytes as is for the moment.
			       They may be decoded by post-read-conversion.  */
			    if charbuf.offset (3) > charbuf_end
			      { break 'decode_loop; }
			    *charbuf = ISO_CODE_ESC; charbuf = charbuf.offset (1);
			    *charbuf = '%' as c_int; charbuf = charbuf.offset (1);
			    *charbuf = 'G' as c_int; charbuf = charbuf.offset (1);
			    CODING_ISO_EMBEDDED_UTF_8! (coding).set (1);
			  }
			else
			  { break 'invalid_code; }
			continue 'decode_loop;
		      }

		    _ =>
		      {
			if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_DESIGNATION as c_uint) == 0
			  { break 'invalid_code; }
			{
			  let mut reg: c_int = 0;
			  let mut chars96: c_int = 0;

			  if c1 >= 0x28 && c1 <= 0x2B
			    { /* designation of DIMENSION1_CHARS94 character set */
			      reg = c1 - 0x28; chars96 = 0;
			      ONE_MORE_BYTE! (c1);
			    }
			  else if c1 >= 0x2C && c1 <= 0x2F
			    { /* designation of DIMENSION1_CHARS96 character set */
			      reg = c1 - 0x2C; chars96 = 1;
			      ONE_MORE_BYTE! (c1);
			    }
			  else
			    { break 'invalid_code; }
			  DECODE_DESIGNATION! (reg, 1, chars96, c1);
			  /* We must update these variables now.  */
			  if reg == 0
			    { charset_id_0 = CODING_ISO_INVOKED_CHARSET! (coding, 0); }
			  else if reg == 1
			    { charset_id_1 = CODING_ISO_INVOKED_CHARSET! (coding, 1); }
			  if chars96 < 0
			    { break 'invalid_code; }
			}
			continue 'decode_loop;
		      }
		    }
		  break 'label_escape_sequence;
		}
	    }

	  if (*cmp_status).state == COMPOSING_NO
	      && (*charset).id != charset_ascii
	      && last_id != (*charset).id
	    {
	      if last_id != charset_ascii
		{ ADD_CHARSET_DATA! (charbuf, char_offset - last_offset, last_id); }
	      last_id = (*charset).id;
	      last_offset = char_offset;
	    }

	  /* Now we know CHARSET and 1st position code C1 of a character.
	     Produce a decoded character while getting 2nd and 3rd
	     position codes C2, C3 if necessary.  */
	  if CHARSET_DIMENSION! (charset) > 1
	    {
	      ONE_MORE_BYTE! (c2);
	      if c2 < 0x20 || (c2 >= 0x80 && c2 < 0xA0)
		  || ((c1 & 0x80) != (c2 & 0x80))
		/* C2 is not in a valid range.  */
		{ break 'invalid_code; }
	      if CHARSET_DIMENSION! (charset) == 2
		{ c1 = (c1 << 8) | c2; }
	      else
		{
		  ONE_MORE_BYTE! (c3);
		  if c3 < 0x20 || (c3 >= 0x80 && c3 < 0xA0)
		      || ((c1 & 0x80) != (c3 & 0x80))
		    /* C3 is not in a valid range.  */
		    { break 'invalid_code; }
		  c1 = (c1 << 16) | (c2 << 8) | c2;
		}
	    }
	  c1 &= 0x7F7F7F;
	  CODING_DECODE_CHAR! (coding, src, src_base, src_end, charset, c1 as c_uint, c);
	  if c < 0
	    {
	      MAYBE_FINISH_COMPOSITION! ();
	      while src_base < src
		{
		  if ASCII_CHAR_P (*src_base as isize)
		    { *charbuf = *src_base as c_int; }
		  else
		    { *charbuf = BYTE8_TO_CHAR (*src_base as c_int); }
		  charbuf = charbuf.offset (1);
		  src_base = src_base.offset (1);
		  char_offset += 1;
		}
	    }
	  else if (*cmp_status).state == COMPOSING_NO
	    {
	      *charbuf = c; charbuf = charbuf.offset (1);
	      char_offset += 1;
	    }
	  else if ((if (*cmp_status).state == COMPOSING_CHAR
		      { (*cmp_status).nchars } else { (*cmp_status).ncomps })
		   >= MAX_COMPOSITION_COMPONENTS)
	    {
	      /* Too long composition.  */
	      MAYBE_FINISH_COMPOSITION! ();
	      *charbuf = c; charbuf = charbuf.offset (1);
	      char_offset += 1;
	    }
	  else
	    { STORE_COMPOSITION_CHAR! (c); }
	  continue 'decode_loop;
	}

	/* invalid_code: */
	MAYBE_FINISH_COMPOSITION! ();
	src = src_base;
	consumed_chars = consumed_chars_base;
	ONE_MORE_BYTE! (c);
	*charbuf = if c < 0 { -c } else if ASCII_CHAR_P (c as isize) { c } else { BYTE8_TO_CHAR (c) };
	charbuf = charbuf.offset (1);
	char_offset += 1;
	/* Reset the invocation and designation status to the safest
	   one; i.e. designate ASCII to the graphic register 0, and
	   invoke that register to the graphic plane 0.  This typically
	   helps the case that a designation sequence for ASCII "ESC (
	   B" is somehow broken (e.g. broken by a newline).  */
	CODING_ISO_INVOCATION! (coding, 0) = 0;
	CODING_ISO_DESIGNATION! (coding, 0) = charset_ascii;
	charset_id_0 = charset_ascii;
	continue 'decode_loop;
      }
  }

  if (*cmp_status).state != COMPOSING_NO
    {
      if mode_of (coding) as c_int & CODING_MODE_LAST_BLOCK != 0
	{ MAYBE_FINISH_COMPOSITION! (); }
      else
	{
	  charbuf = charbuf.offset (- ((*cmp_status).length as isize));
	  i = 0;
	  while i < (*cmp_status).length
	    {
	      (*cmp_status).carryover[i as usize] = *charbuf;
	      i += 1;
	    }
	}
    }
  else if last_id != charset_ascii
    { ADD_CHARSET_DATA! (charbuf, char_offset - last_offset, last_id); }
  (*coding).consumed_char += consumed_chars_base;
  (*coding).consumed = src_base.offset_from ((*coding).source);
  (*coding).charbuf_used = charbuf.offset_from ((*coding).charbuf) as c_int;
}

unsafe fn encode_invocation_designation (charset: *mut charset,
                                         coding: *mut coding_system,
                                         dst: *mut c_uchar, p_nchars: *mut ptrdiff_t) -> *mut c_uchar
{
  let multibytep = dst_multibyte_p (coding);
  let mut produced_chars: ptrdiff_t = *p_nchars;
  let mut dst: *mut c_uchar = dst;
  let mut reg: c_int;			/* graphic register number */
  let id: c_int = CHARSET_ID! (charset);

  macro_rules! EMIT_ONE_ASCII_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	*dst = ($c) as c_uchar; dst = dst.offset (1);
      }
    };
  }
  macro_rules! EMIT_TWO_ASCII_BYTES {
    ($c1:expr, $c2:expr) => {
      {
	produced_chars += 2;
	*dst = ($c1) as c_uchar; dst = dst.offset (1);
	*dst = ($c2) as c_uchar; dst = dst.offset (1);
      }
    };
  }
  macro_rules! EMIT_ONE_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	if multibytep
	  {
	    let mut ch: c_uint = ($c) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  { *dst = ($c) as c_uchar; dst = dst.offset (1); }
      }
    };
  }
  macro_rules! ENCODE_DESIGNATION {
    ($charset:expr, $reg:expr, $coding:expr) => {
      {
	let final_char: c_uchar = CHARSET_ISO_FINAL! ($charset) as c_uchar;
	let intermediate_char_94: &[u8] = b"()*+";
	let intermediate_char_96: &[u8] = b",-./";
	let mut revision: c_int = -1;

	if (CODING_ISO_FLAGS! ($coding) & CODING_ISO_FLAG_REVISION as c_uint) != 0
	  { revision = CHARSET_ISO_REVISION! ($charset); }

	if revision >= 0
	  {
	    EMIT_TWO_ASCII_BYTES! (ISO_CODE_ESC, '&' as c_int);
	    EMIT_ONE_BYTE! ('@' as c_int + revision);
	  }
	EMIT_ONE_ASCII_BYTE! (ISO_CODE_ESC);
	if CHARSET_DIMENSION! ($charset) == 1
	  {
	    let mut b: c_int;
	    if ! CHARSET_ISO_CHARS_96! ($charset)
	      { b = intermediate_char_94[$reg as usize] as c_int; }
	    else
	      { b = intermediate_char_96[$reg as usize] as c_int; }
	    EMIT_ONE_ASCII_BYTE! (b);
	  }
	else
	  {
	    EMIT_ONE_ASCII_BYTE! ('$' as c_int);
	    if ! CHARSET_ISO_CHARS_96! ($charset)
	      {
		if (CODING_ISO_FLAGS! ($coding) & CODING_ISO_FLAG_LONG_FORM as c_uint) != 0
		    || $reg != 0
		    || final_char < '@' as c_uchar || final_char > 'B' as c_uchar
		  { EMIT_ONE_ASCII_BYTE! (intermediate_char_94[$reg as usize] as c_int); }
	      }
	    else
	      { EMIT_ONE_ASCII_BYTE! (intermediate_char_96[$reg as usize] as c_int); }
	  }
	EMIT_ONE_ASCII_BYTE! (final_char as c_int);

	CODING_ISO_DESIGNATION! ($coding, $reg) = CHARSET_ID! ($charset);
      }
    };
  }
  macro_rules! ENCODE_SINGLE_SHIFT_2 {
    () => {
      {
	if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SEVEN_BITS as c_uint) != 0
	  { EMIT_TWO_ASCII_BYTES! (ISO_CODE_ESC, 'N' as c_int); }
	else
	  { EMIT_ONE_BYTE! (ISO_CODE_SS2); }
	CODING_ISO_SINGLE_SHIFTING! (coding).set (1);
      }
    };
  }
  macro_rules! ENCODE_SINGLE_SHIFT_3 {
    () => {
      {
	if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SEVEN_BITS as c_uint) != 0
	  { EMIT_TWO_ASCII_BYTES! (ISO_CODE_ESC, 'O' as c_int); }
	else
	  { EMIT_ONE_BYTE! (ISO_CODE_SS3); }
	CODING_ISO_SINGLE_SHIFTING! (coding).set (1);
      }
    };
  }
  macro_rules! ENCODE_SHIFT_IN {
    () => {
      {
	EMIT_ONE_ASCII_BYTE! (ISO_CODE_SI);
	CODING_ISO_INVOCATION! (coding, 0) = 0;
      }
    };
  }
  macro_rules! ENCODE_SHIFT_OUT {
    () => {
      {
	EMIT_ONE_ASCII_BYTE! (ISO_CODE_SO);
	CODING_ISO_INVOCATION! (coding, 0) = 1;
      }
    };
  }
  macro_rules! ENCODE_LOCKING_SHIFT_2 {
    () => {
      {
	EMIT_TWO_ASCII_BYTES! (ISO_CODE_ESC, 'n' as c_int);
	CODING_ISO_INVOCATION! (coding, 0) = 2;
      }
    };
  }
  macro_rules! ENCODE_LOCKING_SHIFT_3 {
    () => {
      {
	EMIT_TWO_ASCII_BYTES! (ISO_CODE_ESC, 'n' as c_int);
	CODING_ISO_INVOCATION! (coding, 0) = 3;
      }
    };
  }

  /* At first, check designations.  */
  reg = 0;
  while reg < 4
    {
      if id == CODING_ISO_DESIGNATION! (coding, reg)
	{ break; }
      reg += 1;
    }

  if reg >= 4
    {
      /* CHARSET is not yet designated to any graphic registers.  */
      /* At first check the requested designation.  */
      reg = CODING_ISO_REQUEST! (coding, id);
      if reg < 0
	/* Since CHARSET requests no special designation, designate it
	   to graphic register 0.  */
	{ reg = 0; }

      ENCODE_DESIGNATION! (charset, reg, coding);
    }

  if CODING_ISO_INVOCATION! (coding, 0) != reg
      && CODING_ISO_INVOCATION! (coding, 1) != reg
    {
      /* Since the graphic register REG is not invoked to any graphic
	 planes, invoke it to graphic plane 0.  */
      match reg
	{
	0 =>			/* graphic register 0 */
	  {
	    ENCODE_SHIFT_IN! ();
	  }

	1 =>			/* graphic register 1 */
	  {
	    ENCODE_SHIFT_OUT! ();
	  }

	2 =>			/* graphic register 2 */
	  {
	    if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SINGLE_SHIFT as c_uint) != 0
	      { ENCODE_SINGLE_SHIFT_2! (); }
	    else
	      { ENCODE_LOCKING_SHIFT_2! (); }
	  }

	3 =>			/* graphic register 3 */
	  {
	    if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SINGLE_SHIFT as c_uint) != 0
	      { ENCODE_SINGLE_SHIFT_3! (); }
	    else
	      { ENCODE_LOCKING_SHIFT_3! (); }
	  }

	_ =>
	  {
	  }
	}
    }

  *p_nchars = produced_chars;
  return dst;
}


unsafe fn encode_designation_at_bol (coding: *mut coding_system,
                                    charbuf_in: *mut c_int, charbuf_end: *mut c_int,
                                    dst: *mut c_uchar) -> ptrdiff_t
{
  let orig: *mut c_uchar = dst;
  let mut charset: *mut charset;
  /* Table of charsets to be designated to each graphic register.  */
  let mut r: [c_int; 4] = [0; 4];
  let mut c: c_int;
  let mut found: c_int = 0;
  let mut reg: c_int;
  let mut produced_chars: ptrdiff_t = 0;
  let multibytep = dst_multibyte_p (coding);
  let attrs: Lisp_Object;
  let mut charset_list: Lisp_Object;
  let mut charbuf: *mut c_int = charbuf_in;
  let mut dst: *mut c_uchar = dst;

  macro_rules! EMIT_ONE_ASCII_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	*dst = ($c) as c_uchar; dst = dst.offset (1);
      }
    };
  }
  macro_rules! EMIT_TWO_ASCII_BYTES {
    ($c1:expr, $c2:expr) => {
      {
	produced_chars += 2;
	*dst = ($c1) as c_uchar; dst = dst.offset (1);
	*dst = ($c2) as c_uchar; dst = dst.offset (1);
      }
    };
  }
  macro_rules! EMIT_ONE_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	if multibytep
	  {
	    let mut ch: c_uint = ($c) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  { *dst = ($c) as c_uchar; dst = dst.offset (1); }
      }
    };
  }
  macro_rules! ENCODE_DESIGNATION {
    ($charset:expr, $reg:expr, $coding:expr) => {
      {
	let final_char: c_uchar = CHARSET_ISO_FINAL! ($charset) as c_uchar;
	let intermediate_char_94: &[u8] = b"()*+";
	let intermediate_char_96: &[u8] = b",-./";
	let mut revision: c_int = -1;

	if (CODING_ISO_FLAGS! ($coding) & CODING_ISO_FLAG_REVISION as c_uint) != 0
	  { revision = CHARSET_ISO_REVISION! ($charset); }

	if revision >= 0
	  {
	    EMIT_TWO_ASCII_BYTES! (ISO_CODE_ESC, '&' as c_int);
	    EMIT_ONE_BYTE! ('@' as c_int + revision);
	  }
	EMIT_ONE_ASCII_BYTE! (ISO_CODE_ESC);
	if CHARSET_DIMENSION! ($charset) == 1
	  {
	    let mut b: c_int;
	    if ! CHARSET_ISO_CHARS_96! ($charset)
	      { b = intermediate_char_94[$reg as usize] as c_int; }
	    else
	      { b = intermediate_char_96[$reg as usize] as c_int; }
	    EMIT_ONE_ASCII_BYTE! (b);
	  }
	else
	  {
	    EMIT_ONE_ASCII_BYTE! ('$' as c_int);
	    if ! CHARSET_ISO_CHARS_96! ($charset)
	      {
		if (CODING_ISO_FLAGS! ($coding) & CODING_ISO_FLAG_LONG_FORM as c_uint) != 0
		    || $reg != 0
		    || final_char < '@' as c_uchar || final_char > 'B' as c_uchar
		  { EMIT_ONE_ASCII_BYTE! (intermediate_char_94[$reg as usize] as c_int); }
	      }
	    else
	      { EMIT_ONE_ASCII_BYTE! (intermediate_char_96[$reg as usize] as c_int); }
	  }
	EMIT_ONE_ASCII_BYTE! (final_char as c_int);

	CODING_ISO_DESIGNATION! ($coding, $reg) = CHARSET_ID! ($charset);
      }
    };
  }

  attrs = CODING_ID_ATTRS ((*coding).id as hash_idx_t);
  charset_list = CODING_ATTR_CHARSET_LIST (attrs);
  if EQ (charset_list, Qiso_2022 ())
    { charset_list = Viso_2022_charset_list; }

  reg = 0;
  while reg < 4
    {
      r[reg as usize] = -1;
      reg += 1;
    }

  while charbuf < charbuf_end && found < 4
    {
      let id: c_int;

      c = *charbuf; charbuf = charbuf.offset (1);
      if c == '\n' as c_int
	{ break; }
      charset = char_charset (c, charset_list, std::ptr::null_mut ());
      id = CHARSET_ID! (charset);
      reg = CODING_ISO_REQUEST! (coding, id);
      if reg >= 0 && r[reg as usize] < 0
	{
	  found += 1;
	  r[reg as usize] = id;
	}
    }

  if found != 0
    {
      reg = 0;
      while reg < 4
	{
	  if r[reg as usize] >= 0
	      && CODING_ISO_DESIGNATION! (coding, reg) != r[reg as usize]
	    { ENCODE_DESIGNATION! (CHARSET_FROM_ID! (r[reg as usize]), reg, coding); }
	  reg += 1;
	}
    }

  return dst.offset_from (orig);
}

unsafe extern "C" fn encode_coding_iso_2022 (coding: *mut coding_system) -> bool
{
  let multibytep = dst_multibyte_p (coding);
  let mut charbuf: *mut c_int = (*coding).charbuf;
  let charbuf_end: *mut c_int = charbuf.offset ((*coding).charbuf_used as isize);
  let mut dst: *mut c_uchar = (*coding).destination.offset ((*coding).produced);
  let mut dst_end: *mut c_uchar = (*coding).destination.offset ((*coding).dst_bytes);
  let safe_room: c_int = 16;
  let mut bol_designation: bool
    = (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_DESIGNATE_AT_BOL as c_uint) != 0
       && CODING_ISO_BOL! (coding).get () != 0;
  let mut produced_chars: ptrdiff_t = 0;
  let mut attrs: Lisp_Object = lisp_h_Qnil ();
  let mut eol_type: Lisp_Object = lisp_h_Qnil ();
  let mut charset_list: Lisp_Object = lisp_h_Qnil ();
  let ascii_compatible: bool;
  let mut c: c_int;
  let mut preferred_charset_id: c_int = -1;

  macro_rules! ASSURE_DESTINATION {
    ($bytes:expr) => {
      {
	if dst.offset (($bytes) as isize) >= dst_end
	  {
	    let more_bytes: ptrdiff_t = charbuf_end.offset_from (charbuf) + ($bytes) as isize;

	    dst = alloc_destination (coding, more_bytes, dst);
	    dst_end = (*coding).destination.offset ((*coding).dst_bytes);
	  }
      }
    };
  }
  macro_rules! EMIT_ONE_ASCII_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	*dst = ($c) as c_uchar; dst = dst.offset (1);
      }
    };
  }
  macro_rules! EMIT_TWO_ASCII_BYTES {
    ($c1:expr, $c2:expr) => {
      {
	produced_chars += 2;
	*dst = ($c1) as c_uchar; dst = dst.offset (1);
	*dst = ($c2) as c_uchar; dst = dst.offset (1);
      }
    };
  }
  macro_rules! EMIT_ONE_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	if multibytep
	  {
	    let mut ch: c_uint = ($c) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  { *dst = ($c) as c_uchar; dst = dst.offset (1); }
      }
    };
  }
  macro_rules! EMIT_TWO_BYTES {
    ($c1:expr, $c2:expr) => {
      {
	produced_chars += 2;
	if multibytep
	  {
	    let mut ch: c_uint;

	    ch = ($c1) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	    ch = ($c2) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  {
	    *dst = ($c1) as c_uchar; dst = dst.offset (1);
	    *dst = ($c2) as c_uchar; dst = dst.offset (1);
	  }
      }
    };
  }
  macro_rules! CODING_ENCODE_CHAR {
    ($coding:expr, $dst:expr, $dst_end:expr, $charset:expr, $c:expr, $code:expr) => {
      {
	let offset: ptrdiff_t;

	charset_map_loaded = false;
	$code = ENCODE_CHAR! ($charset, $c);
	if charset_map_loaded
	    && { offset = coding_change_destination ($coding); offset != 0 }
	  {
	    $dst = $dst.offset (offset);
	    $dst_end = $dst_end.offset (offset);
	  }
      }
    };
  }
  /* The charset.rs mirror's `CHAR_CHARSET_P` does not compile (see encode_coding_emacs_mule);
     the faithful transcription shadows it here too.  */
  macro_rules! CODING_CHAR_CHARSET {
    ($coding:expr, $dst:expr, $dst_end:expr, $c:expr, $charset_list:expr, $code_return:expr, $charset:expr) => {
      {
	let offset: ptrdiff_t;

	charset_map_loaded = false;
	$charset = char_charset ($c, $charset_list, $code_return);
	if charset_map_loaded
	    && { offset = coding_change_destination ($coding); offset != 0 }
	  {
	    $dst = $dst.offset (offset);
	    $dst_end = $dst_end.offset (offset);
	  }
      }
    };
  }
  macro_rules! CODING_CHAR_CHARSET_P {
    ($coding:expr, $dst:expr, $dst_end:expr, $c:expr, $charset:expr, $result:expr) => {
      {
	let offset: ptrdiff_t;

	charset_map_loaded = false;
	$result = CHAR_CHARSET_P! ($c, $charset);
	if charset_map_loaded
	    && { offset = coding_change_destination ($coding); offset != 0 }
	  {
	    $dst = $dst.offset (offset);
	    $dst_end = $dst_end.offset (offset);
	  }
      }
    };
  }
  macro_rules! ENCODE_DESIGNATION {
    ($charset:expr, $reg:expr, $coding:expr) => {
      {
	let final_char: c_uchar = CHARSET_ISO_FINAL! ($charset) as c_uchar;
	let intermediate_char_94: &[u8] = b"()*+";
	let intermediate_char_96: &[u8] = b",-./";
	let mut revision: c_int = -1;

	if (CODING_ISO_FLAGS! ($coding) & CODING_ISO_FLAG_REVISION as c_uint) != 0
	  { revision = CHARSET_ISO_REVISION! ($charset); }

	if revision >= 0
	  {
	    EMIT_TWO_ASCII_BYTES! (ISO_CODE_ESC, '&' as c_int);
	    EMIT_ONE_BYTE! ('@' as c_int + revision);
	  }
	EMIT_ONE_ASCII_BYTE! (ISO_CODE_ESC);
	if CHARSET_DIMENSION! ($charset) == 1
	  {
	    let mut b: c_int;
	    if ! CHARSET_ISO_CHARS_96! ($charset)
	      { b = intermediate_char_94[$reg as usize] as c_int; }
	    else
	      { b = intermediate_char_96[$reg as usize] as c_int; }
	    EMIT_ONE_ASCII_BYTE! (b);
	  }
	else
	  {
	    EMIT_ONE_ASCII_BYTE! ('$' as c_int);
	    if ! CHARSET_ISO_CHARS_96! ($charset)
	      {
		if (CODING_ISO_FLAGS! ($coding) & CODING_ISO_FLAG_LONG_FORM as c_uint) != 0
		    || $reg != 0
		    || final_char < '@' as c_uchar || final_char > 'B' as c_uchar
		  { EMIT_ONE_ASCII_BYTE! (intermediate_char_94[$reg as usize] as c_int); }
	      }
	    else
	      { EMIT_ONE_ASCII_BYTE! (intermediate_char_96[$reg as usize] as c_int); }
	  }
	EMIT_ONE_ASCII_BYTE! (final_char as c_int);

	CODING_ISO_DESIGNATION! ($coding, $reg) = CHARSET_ID! ($charset);
      }
    };
  }
  macro_rules! ENCODE_SINGLE_SHIFT_2 {
    () => {
      {
	if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SEVEN_BITS as c_uint) != 0
	  { EMIT_TWO_ASCII_BYTES! (ISO_CODE_ESC, 'N' as c_int); }
	else
	  { EMIT_ONE_BYTE! (ISO_CODE_SS2); }
	CODING_ISO_SINGLE_SHIFTING! (coding).set (1);
      }
    };
  }
  macro_rules! ENCODE_SINGLE_SHIFT_3 {
    () => {
      {
	if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SEVEN_BITS as c_uint) != 0
	  { EMIT_TWO_ASCII_BYTES! (ISO_CODE_ESC, 'O' as c_int); }
	else
	  { EMIT_ONE_BYTE! (ISO_CODE_SS3); }
	CODING_ISO_SINGLE_SHIFTING! (coding).set (1);
      }
    };
  }
  macro_rules! ENCODE_SHIFT_IN {
    () => {
      {
	EMIT_ONE_ASCII_BYTE! (ISO_CODE_SI);
	CODING_ISO_INVOCATION! (coding, 0) = 0;
      }
    };
  }
  macro_rules! ENCODE_SHIFT_OUT {
    () => {
      {
	EMIT_ONE_ASCII_BYTE! (ISO_CODE_SO);
	CODING_ISO_INVOCATION! (coding, 0) = 1;
      }
    };
  }
  macro_rules! ENCODE_LOCKING_SHIFT_2 {
    () => {
      {
	EMIT_TWO_ASCII_BYTES! (ISO_CODE_ESC, 'n' as c_int);
	CODING_ISO_INVOCATION! (coding, 0) = 2;
      }
    };
  }
  macro_rules! ENCODE_LOCKING_SHIFT_3 {
    () => {
      {
	EMIT_TWO_ASCII_BYTES! (ISO_CODE_ESC, 'n' as c_int);
	CODING_ISO_INVOCATION! (coding, 0) = 3;
      }
    };
  }
  macro_rules! ENCODE_RESET_PLANE_AND_REGISTER {
    () => {
      {
	let mut reg: c_int;
	let mut charset: *mut charset;

	if CODING_ISO_INVOCATION! (coding, 0) != 0
	  { ENCODE_SHIFT_IN! (); }
	reg = 0;
	while reg < 4
	  {
	    if CODING_ISO_INITIAL! (coding, reg) as c_int >= 0
		&& (CODING_ISO_DESIGNATION! (coding, reg)
		    != CODING_ISO_INITIAL! (coding, reg) as c_int)
	      {
		charset = CHARSET_FROM_ID! (CODING_ISO_INITIAL! (coding, reg) as c_int);
		ENCODE_DESIGNATION! (charset, reg, coding);
	      }
	    reg += 1;
	  }
      }
    };
  }
  macro_rules! ENCODE_ISO_CHARACTER_DIMENSION1 {
    ($charset:expr, $c1:expr) => {
      {
	let mut id: c_int = CHARSET_ID! ($charset);

	if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_USE_ROMAN as c_uint) != 0
	    && id == charset_ascii
	  {
	    id = charset_jisx0201_roman;
	    $charset = CHARSET_FROM_ID! (id);
	  }

	loop
	  {
	    if CODING_ISO_SINGLE_SHIFTING! (coding).get () != 0
	      {
		if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SEVEN_BITS as c_uint) != 0
		  { EMIT_ONE_ASCII_BYTE! ($c1 & 0x7F); }
		else
		  { EMIT_ONE_BYTE! ($c1 | 0x80); }
		CODING_ISO_SINGLE_SHIFTING! (coding).set (0);
		break;
	      }
	    else if id == CODING_ISO_INVOKED_CHARSET! (coding, 0)
	      {
		EMIT_ONE_ASCII_BYTE! ($c1 & 0x7F);
		break;
	      }
	    else if id == CODING_ISO_INVOKED_CHARSET! (coding, 1)
	      {
		EMIT_ONE_BYTE! ($c1 | 0x80);
		break;
	      }
	    else
	      /* Since CHARSET is not yet invoked to any graphic planes, we
		 must invoke it, or, at first, designate it to some graphic
		 register.  Then repeat the loop to actually produce the
		 character.  */
	      { dst = encode_invocation_designation ($charset, coding, dst,
						     &raw mut produced_chars); }
	  }
      }
    };
  }
  macro_rules! ENCODE_ISO_CHARACTER_DIMENSION2 {
    ($charset:expr, $c1:expr, $c2:expr) => {
      {
	let mut id: c_int = CHARSET_ID! ($charset);

	if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_USE_OLDJIS as c_uint) != 0
	    && id == charset_jisx0208
	  {
	    id = charset_jisx0208_1978;
	    $charset = CHARSET_FROM_ID! (id);
	  }

	loop
	  {
	    if CODING_ISO_SINGLE_SHIFTING! (coding).get () != 0
	      {
		if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_SEVEN_BITS as c_uint) != 0
		  { EMIT_TWO_ASCII_BYTES! ($c1 & 0x7F, $c2 & 0x7F); }
		else
		  { EMIT_TWO_BYTES! ($c1 | 0x80, $c2 | 0x80); }
		CODING_ISO_SINGLE_SHIFTING! (coding).set (0);
		break;
	      }
	    else if id == CODING_ISO_INVOKED_CHARSET! (coding, 0)
	      {
		EMIT_TWO_ASCII_BYTES! ($c1 & 0x7F, $c2 & 0x7F);
		break;
	      }
	    else if id == CODING_ISO_INVOKED_CHARSET! (coding, 1)
	      {
		EMIT_TWO_BYTES! ($c1 | 0x80, $c2 | 0x80);
		break;
	      }
	    else
	      /* Since CHARSET is not yet invoked to any graphic planes, we
		 must invoke it, or, at first, designate it to some graphic
		 register.  Then repeat the loop to actually produce the
		 character.  */
	      { dst = encode_invocation_designation ($charset, coding, dst,
						     &raw mut produced_chars); }
	  }
      }
    };
  }
  macro_rules! ENCODE_ISO_CHARACTER {
    ($charset:expr, $c:expr) => {
      {
	let mut code: c_uint = 0;
	CODING_ENCODE_CHAR! (coding, dst, dst_end, $charset, $c, code);

	if CHARSET_DIMENSION! ($charset) == 1
	  { ENCODE_ISO_CHARACTER_DIMENSION1! ($charset, code); }
	else
	  { ENCODE_ISO_CHARACTER_DIMENSION2! ($charset, code >> 8, code & 0xFF); }
      }
    };
  }

  CODING_GET_INFO! (coding, attrs, charset_list);
  eol_type = if inhibit_eol_conversion { Qunix () } else { CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t) };
  if VECTORP (eol_type)
    { eol_type = Qunix (); }

  setup_iso_safe_charsets (attrs);
  /* Charset list may have been changed.  */
  charset_list = CODING_ATTR_CHARSET_LIST (attrs);
  (*coding).safe_charsets = SDATA (CODING_ATTR_SAFE_CHARSETS (attrs));

  ascii_compatible
    = (! NILP (CODING_ATTR_ASCII_COMPAT (attrs))
       && (CODING_ISO_FLAGS! (coding) & (CODING_ISO_FLAG_DESIGNATION
					 | CODING_ISO_FLAG_LOCKING_SHIFT) as c_uint) == 0);

  while charbuf < charbuf_end
    {
      ASSURE_DESTINATION! (safe_room);

      if bol_designation
	{
	  /* We have to produce designation sequences if any now.  */
	  let mut desig_buf: [c_uchar; 16] = [0; 16];
	  let mut nbytes: ptrdiff_t;
	  let mut offset: ptrdiff_t;

	  charset_map_loaded = false;
	  nbytes = encode_designation_at_bol (coding, charbuf, charbuf_end,
					      desig_buf.as_mut_ptr ());
	  if charset_map_loaded
	      && { offset = coding_change_destination (coding); offset != 0 }
	    {
	      dst = dst.offset (offset);
	      dst_end = dst_end.offset (offset);
	    }
	  memcpy (dst as *mut c_void, desig_buf.as_ptr () as *const c_void, nbytes as usize);
	  dst = dst.offset (nbytes);
	  /* We are sure that designation sequences are all ASCII bytes.  */
	  produced_chars += nbytes;
	  bol_designation = false;
	  ASSURE_DESTINATION! (safe_room);
	}

      c = *charbuf; charbuf = charbuf.offset (1);

      if c < 0
	{
	  /* Handle an annotation.  */
	  if *charbuf == CODING_ANNOTATE_COMPOSITION_MASK as c_int
	    {
	      /* Not yet implemented.  */
	      ()
	    }
	  else if *charbuf == CODING_ANNOTATE_CHARSET_MASK as c_int
	    {
	      preferred_charset_id = *charbuf.offset (2);
	      if preferred_charset_id >= 0
		  && NILP (Fmemq (make_fixnum (preferred_charset_id as isize),
				  charset_list))
		{ preferred_charset_id = -1; }
	    }
	  else
	    { emacs_abort (); }
	  charbuf = charbuf.offset ((-c - 1) as isize);
	  continue;
	}

      /* Now encode the character C.  */
      if c < 0x20 || c == 0x7F
	{
	  if c == '\n' as c_int
	      || (c == '\r' as c_int && EQ (eol_type, Qmac ()))
	    {
	      if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_RESET_AT_EOL as c_uint) != 0
		{ ENCODE_RESET_PLANE_AND_REGISTER! (); }
	      if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_INIT_AT_BOL as c_uint) != 0
		{
		  let mut i: c_int;

		  i = 0;
		  while i < 4
		    {
		      CODING_ISO_DESIGNATION! (coding, i)
			= CODING_ISO_INITIAL! (coding, i) as c_int;
		      i += 1;
		    }
		}
	      bol_designation = ((CODING_ISO_FLAGS! (coding)
				  & CODING_ISO_FLAG_DESIGNATE_AT_BOL as c_uint)
				 != 0);
	    }
	  else if (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_RESET_AT_CNTL as c_uint) != 0
	    { ENCODE_RESET_PLANE_AND_REGISTER! (); }
	  EMIT_ONE_ASCII_BYTE! (c);
	}
      else if ASCII_CHAR_P (c as isize)
	{
	  if ascii_compatible
	    { EMIT_ONE_ASCII_BYTE! (c); }
	  else
	    {
	      let mut charset: *mut charset = CHARSET_FROM_ID! (charset_ascii);
	      ENCODE_ISO_CHARACTER! (charset, c);
	    }
	}
      else if CHAR_BYTE8_P (c)
	{
	  c = CHAR_TO_BYTE8 (c);
	  EMIT_ONE_BYTE! (c);
	}
      else
	{
	  let mut charset: *mut charset = std::ptr::null_mut ();

	  if preferred_charset_id >= 0
	    {
	      let mut result: bool;

	      charset = CHARSET_FROM_ID! (preferred_charset_id);
	      CODING_CHAR_CHARSET_P! (coding, dst, dst_end, c, charset, result);
	      if ! result
		{ CODING_CHAR_CHARSET! (coding, dst, dst_end, c, charset_list,
					std::ptr::null_mut (), charset); }
	    }
	  else
	    { CODING_CHAR_CHARSET! (coding, dst, dst_end, c, charset_list,
				    std::ptr::null_mut (), charset); }
	  if charset.is_null ()
	    {
	      if mode_of (coding) as c_int & CODING_MODE_SAFE_ENCODING != 0
		{
		  c = CODING_INHIBIT_CHARACTER_SUBSTITUTION as c_int;
		  charset = CHARSET_FROM_ID! (charset_ascii);
		}
	      else
		{
		  c = (*coding).default_char;
		  CODING_CHAR_CHARSET! (coding, dst, dst_end, c,
					charset_list, std::ptr::null_mut (), charset);
		}
	    }
	  ENCODE_ISO_CHARACTER! (charset, c);
	}
    }

  if mode_of (coding) as c_int & CODING_MODE_LAST_BLOCK != 0
      && (CODING_ISO_FLAGS! (coding) & CODING_ISO_FLAG_RESET_AT_EOL as c_uint) != 0
    {
      ASSURE_DESTINATION! (safe_room);
      ENCODE_RESET_PLANE_AND_REGISTER! ();
    }
  record_conversion_result (coding, CODING_RESULT_SUCCESS);
  CODING_ISO_BOL! (coding).set (bol_designation as c_int);
  (*coding).produced_char += produced_chars;
  (*coding).produced = dst.offset_from ((*coding).destination);
  return false;
}
/* See the above "GENERAL NOTES on `detect_coding_XXX ()' functions".
   Return true if a text is encoded in SJIS.  */

unsafe extern "C" fn detect_coding_sjis (coding: *mut coding_system,
			      detect_info: *mut coding_detection_info) -> bool
{
  let mut src: *const c_uchar = (*coding).source;
  let mut src_base: *const c_uchar = std::ptr::null ();
  let src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let multibytep = src_multibyte_p (coding);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut found: c_int = 0;
  let mut c: c_int;
  let mut attrs: Lisp_Object;
  let mut charset_list: Lisp_Object;
  let mut max_first_byte_of_2_byte_code: c_int;

  CODING_GET_INFO! (coding, attrs, charset_list);
  max_first_byte_of_2_byte_code = if list_length (charset_list) <= 3 { 0xEF } else { 0xFC };

  (*detect_info).checked |= CATEGORY_MASK_SJIS;
  /* A coding system of this category is always ASCII compatible.  */
  src = src.offset ((*coding).head_ascii);

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }

    loop
      {
	src_base = src;
	ONE_MORE_BYTE! (c);
	if c < 0x80
	  { continue; }
	if (c >= 0x81 && c <= 0x9F)
	    || (c >= 0xE0 && c <= max_first_byte_of_2_byte_code)
	  {
	    ONE_MORE_BYTE! (c);
	    if c < 0x40 || c == 0x7F || c > 0xFC
	      { break; }
	    found = CATEGORY_MASK_SJIS;
	  }
	else if c >= 0xA0 && c < 0xE0
	  { found = CATEGORY_MASK_SJIS; }
	else
	  { break; }
      }
    (*detect_info).rejected |= CATEGORY_MASK_SJIS;
    return false;
  }

  if src_base < src && mode_of (coding) as c_int & CODING_MODE_LAST_BLOCK != 0
    {
      (*detect_info).rejected |= CATEGORY_MASK_SJIS;
      return false;
    }
  (*detect_info).found |= found;
  return true;
}

/* See the above "GENERAL NOTES on `detect_coding_XXX ()' functions".
   Return true if a text is encoded in BIG5.  */

unsafe extern "C" fn detect_coding_big5 (coding: *mut coding_system,
			      detect_info: *mut coding_detection_info) -> bool
{
  let mut src: *const c_uchar = (*coding).source;
  let mut src_base: *const c_uchar = std::ptr::null ();
  let src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let multibytep = src_multibyte_p (coding);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut found: c_int = 0;
  let mut c: c_int;

  (*detect_info).checked |= CATEGORY_MASK_BIG5;
  /* A coding system of this category is always ASCII compatible.  */
  src = src.offset ((*coding).head_ascii);

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }

    loop
      {
	src_base = src;
	ONE_MORE_BYTE! (c);
	if c < 0x80
	  { continue; }
	if c >= 0xA1
	  {
	    ONE_MORE_BYTE! (c);
	    if c < 0x40 || (c >= 0x7F && c <= 0xA0)
	      { return false; }
	    found = CATEGORY_MASK_BIG5;
	  }
	else
	  { break; }
      }
    (*detect_info).rejected |= CATEGORY_MASK_BIG5;
    return false;
  }

  if src_base < src && mode_of (coding) as c_int & CODING_MODE_LAST_BLOCK != 0
    {
      (*detect_info).rejected |= CATEGORY_MASK_BIG5;
      return false;
    }
  (*detect_info).found |= found;
  return true;
}

/* See the above "GENERAL NOTES on `decode_coding_XXX ()' functions".  */

unsafe extern "C" fn decode_coding_sjis (coding: *mut coding_system)
{
  let mut src: *const c_uchar = (*coding).source.offset ((*coding).consumed);
  let mut src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let mut src_base: *const c_uchar = std::ptr::null ();
  let mut charbuf: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_used as isize);
  /* We may produce one charset annotation in one loop and one more at
     the end.  */
  let charbuf_end: *mut c_int
    = (*coding).charbuf.offset (((*coding).charbuf_size - (MAX_ANNOTATION_LENGTH * 2)) as isize);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut consumed_chars_base: ptrdiff_t = 0;
  let multibytep = src_multibyte_p (coding);
  let mut charset_roman: *mut charset;
  let mut charset_kanji: *mut charset;
  let mut charset_kana: *mut charset;
  let mut charset_kanji2: *mut charset;
  let mut attrs: Lisp_Object;
  let mut charset_list: Lisp_Object;
  let mut val: Lisp_Object;
  let mut char_offset: ptrdiff_t = (*coding).produced_char;
  let mut last_offset: ptrdiff_t = char_offset;
  let mut last_id: c_int = charset_ascii;
  let eol_dos
    = !inhibit_eol_conversion && EQ (CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t), Qdos ());
  let mut byte_after_cr: c_int = -1;

  /* `ADD_CHARSET_DATA` is used after `no_more_source` as well as inside the loop, so the
     two annotation macros sit at function scope rather than inside the labelled block
     (`macro_rules!` is resolved where it is defined).  */
  macro_rules! ADD_ANNOTATION_DATA {
    ($buf:expr, $len:expr, $mask:expr, $nchars:expr) => {
      {
	*$buf = -($len); $buf = $buf.offset (1);
	*$buf = ($mask); $buf = $buf.offset (1);
	*$buf = ($nchars) as c_int; $buf = $buf.offset (1);
	coding_bits_set (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1, 1);
      }
    };
  }
  macro_rules! ADD_CHARSET_DATA {
    ($buf:expr, $nchars:expr, $id:expr) => {
      {
	ADD_ANNOTATION_DATA! ($buf, 4, CODING_ANNOTATE_CHARSET_MASK as c_int, $nchars);
	*$buf = $id; $buf = $buf.offset (1);
      }
    };
  }

  CODING_GET_INFO! (coding, attrs, charset_list);

  val = charset_list;
  charset_roman = CHARSET_FROM_ID! (XFIXNUM (XCAR (val))); val = XCDR (val);
  charset_kana = CHARSET_FROM_ID! (XFIXNUM (XCAR (val))); val = XCDR (val);
  charset_kanji = CHARSET_FROM_ID! (XFIXNUM (XCAR (val))); val = XCDR (val);
  charset_kanji2 = if NILP (val) { std::ptr::null_mut () } else { CHARSET_FROM_ID! (XFIXNUM (XCAR (val))) };

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }
    macro_rules! CODING_DECODE_CHAR {
      ($coding:expr, $src:expr, $src_base:expr, $src_end:expr, $charset:expr, $code:expr, $c:expr) => {
	{
	  let offset: ptrdiff_t;

	  charset_map_loaded = false;
	  $c = DECODE_CHAR! ($charset, $code);
	  if charset_map_loaded
	      && { offset = coding_change_source ($coding); offset != 0 }
	    {
	      $src = $src.offset (offset);
	      $src_base = $src_base.offset (offset);
	      $src_end = $src_end.offset (offset);
	    }
	}
      };
    }

    'decode_loop: loop
      {
	let mut c: c_int; let mut c1: c_int;
	let mut charset: *mut charset;

	src_base = src;
	consumed_chars_base = consumed_chars;

	if charbuf >= charbuf_end
	  {
	    if byte_after_cr >= 0
	      { src_base = src_base.offset (-1); }
	    break;
	  }

	'invalid_code: {
	  if byte_after_cr >= 0
	    { c = byte_after_cr; byte_after_cr = -1; }
	  else
	    { ONE_MORE_BYTE! (c); }
	  if c < 0
	    { break 'invalid_code; }
	  if c < 0x80
	    {
	      if eol_dos && c == '\r' as c_int
		{ ONE_MORE_BYTE! (byte_after_cr); }
	      charset = charset_roman;
	    }
	  else if c == 0x80 || c == 0xA0
	    { break 'invalid_code; }
	  else if c >= 0xA1 && c <= 0xDF
	    {
	      /* SJIS -> JISX0201-Kana */
	      c &= 0x7F;
	      charset = charset_kana;
	    }
	  else if c <= 0xEF
	    {
	      /* SJIS -> JISX0208 */
	      ONE_MORE_BYTE! (c1);
	      if c1 < 0x40 || c1 == 0x7F || c1 > 0xFC
		{ break 'invalid_code; }
	      c = (c << 8) | c1;
	      c = SJIS_TO_JIS (c);
	      charset = charset_kanji;
	    }
	  else if c <= 0xFC && ! charset_kanji2.is_null ()
	    {
	      /* SJIS -> JISX0213-2 */
	      ONE_MORE_BYTE! (c1);
	      if c1 < 0x40 || c1 == 0x7F || c1 > 0xFC
		{ break 'invalid_code; }
	      c = (c << 8) | c1;
	      c = SJIS_TO_JIS2 (c);
	      charset = charset_kanji2;
	    }
	  else
	    { break 'invalid_code; }
	  if (*charset).id != charset_ascii
	      && last_id != (*charset).id
	    {
	      if last_id != charset_ascii
		{ ADD_CHARSET_DATA! (charbuf, char_offset - last_offset, last_id); }
	      last_id = (*charset).id;
	      last_offset = char_offset;
	    }
	  CODING_DECODE_CHAR! (coding, src, src_base, src_end, charset, c as c_uint, c);
	  *charbuf = c; charbuf = charbuf.offset (1);
	  char_offset += 1;
	  continue 'decode_loop;
	}

	/* invalid_code: */
	src = src_base;
	consumed_chars = consumed_chars_base;
	ONE_MORE_BYTE! (c);
	*charbuf = if c < 0 { -c } else { BYTE8_TO_CHAR (c) }; charbuf = charbuf.offset (1);
	char_offset += 1;
      }
  }

  if last_id != charset_ascii
    { ADD_CHARSET_DATA! (charbuf, char_offset - last_offset, last_id); }
  (*coding).consumed_char += consumed_chars_base;
  (*coding).consumed = src_base.offset_from ((*coding).source);
  (*coding).charbuf_used = charbuf.offset_from ((*coding).charbuf) as c_int;
}

unsafe extern "C" fn decode_coding_big5 (coding: *mut coding_system)
{
  let mut src: *const c_uchar = (*coding).source.offset ((*coding).consumed);
  let mut src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let mut src_base: *const c_uchar = std::ptr::null ();
  let mut charbuf: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_used as isize);
  /* We may produce one charset annotation in one loop and one more at
     the end.  */
  let charbuf_end: *mut c_int
    = (*coding).charbuf.offset (((*coding).charbuf_size - (MAX_ANNOTATION_LENGTH * 2)) as isize);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut consumed_chars_base: ptrdiff_t = 0;
  let multibytep = src_multibyte_p (coding);
  let mut charset_roman: *mut charset;
  let mut charset_big5: *mut charset;
  let mut attrs: Lisp_Object;
  let mut charset_list: Lisp_Object;
  let mut val: Lisp_Object;
  let mut char_offset: ptrdiff_t = (*coding).produced_char;
  let mut last_offset: ptrdiff_t = char_offset;
  let mut last_id: c_int = charset_ascii;
  let eol_dos
    = !inhibit_eol_conversion && EQ (CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t), Qdos ());
  let mut byte_after_cr: c_int = -1;

  macro_rules! ADD_ANNOTATION_DATA {
    ($buf:expr, $len:expr, $mask:expr, $nchars:expr) => {
      {
	*$buf = -($len); $buf = $buf.offset (1);
	*$buf = ($mask); $buf = $buf.offset (1);
	*$buf = ($nchars) as c_int; $buf = $buf.offset (1);
	coding_bits_set (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1, 1);
      }
    };
  }
  macro_rules! ADD_CHARSET_DATA {
    ($buf:expr, $nchars:expr, $id:expr) => {
      {
	ADD_ANNOTATION_DATA! ($buf, 4, CODING_ANNOTATE_CHARSET_MASK as c_int, $nchars);
	*$buf = $id; $buf = $buf.offset (1);
      }
    };
  }

  CODING_GET_INFO! (coding, attrs, charset_list);
  val = charset_list;
  charset_roman = CHARSET_FROM_ID! (XFIXNUM (XCAR (val))); val = XCDR (val);
  charset_big5 = CHARSET_FROM_ID! (XFIXNUM (XCAR (val)));

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }
    macro_rules! CODING_DECODE_CHAR {
      ($coding:expr, $src:expr, $src_base:expr, $src_end:expr, $charset:expr, $code:expr, $c:expr) => {
	{
	  let offset: ptrdiff_t;

	  charset_map_loaded = false;
	  $c = DECODE_CHAR! ($charset, $code);
	  if charset_map_loaded
	      && { offset = coding_change_source ($coding); offset != 0 }
	    {
	      $src = $src.offset (offset);
	      $src_base = $src_base.offset (offset);
	      $src_end = $src_end.offset (offset);
	    }
	}
      };
    }

    'decode_loop: loop
      {
	let mut c: c_int; let mut c1: c_int;
	let mut charset: *mut charset;

	src_base = src;
	consumed_chars_base = consumed_chars;

	if charbuf >= charbuf_end
	  {
	    if byte_after_cr >= 0
	      { src_base = src_base.offset (-1); }
	    break;
	  }

	'invalid_code: {
	  if byte_after_cr >= 0
	    { c = byte_after_cr; byte_after_cr = -1; }
	  else
	    { ONE_MORE_BYTE! (c); }

	  if c < 0
	    { break 'invalid_code; }
	  if c < 0x80
	    {
	      if eol_dos && c == '\r' as c_int
		{ ONE_MORE_BYTE! (byte_after_cr); }
	      charset = charset_roman;
	    }
	  else
	    {
	      /* BIG5 -> Big5 */
	      if c < 0xA1 || c > 0xFE
		{ break 'invalid_code; }
	      ONE_MORE_BYTE! (c1);
	      if c1 < 0x40 || (c1 > 0x7E && c1 < 0xA1) || c1 > 0xFE
		{ break 'invalid_code; }
	      c = c << 8 | c1;
	      charset = charset_big5;
	    }
	  if (*charset).id != charset_ascii
	      && last_id != (*charset).id
	    {
	      if last_id != charset_ascii
		{ ADD_CHARSET_DATA! (charbuf, char_offset - last_offset, last_id); }
	      last_id = (*charset).id;
	      last_offset = char_offset;
	    }
	  CODING_DECODE_CHAR! (coding, src, src_base, src_end, charset, c as c_uint, c);
	  *charbuf = c; charbuf = charbuf.offset (1);
	  char_offset += 1;
	  continue 'decode_loop;
	}

	/* invalid_code: */
	src = src_base;
	consumed_chars = consumed_chars_base;
	ONE_MORE_BYTE! (c);
	*charbuf = if c < 0 { -c } else { BYTE8_TO_CHAR (c) }; charbuf = charbuf.offset (1);
	char_offset += 1;
      }
  }

  if last_id != charset_ascii
    { ADD_CHARSET_DATA! (charbuf, char_offset - last_offset, last_id); }
  (*coding).consumed_char += consumed_chars_base;
  (*coding).consumed = src_base.offset_from ((*coding).source);
  (*coding).charbuf_used = charbuf.offset_from ((*coding).charbuf) as c_int;
}
/* See the above "GENERAL NOTES on `encode_coding_XXX ()' functions".
   This function can encode charsets `ascii', `katakana-jisx0201',
   `japanese-jisx0208', `chinese-big5-1', and `chinese-big5-2'.  We
   are sure that all these charsets are registered as official charset
   (i.e. do not have extended leading-codes).  Characters of other
   charsets are produced without any encoding.  */

unsafe extern "C" fn encode_coding_sjis (coding: *mut coding_system) -> bool
{
  let multibytep = dst_multibyte_p (coding);
  let mut charbuf: *mut c_int = (*coding).charbuf;
  let charbuf_end: *mut c_int = charbuf.offset ((*coding).charbuf_used as isize);
  let mut dst: *mut c_uchar = (*coding).destination.offset ((*coding).produced);
  let mut dst_end: *mut c_uchar = (*coding).destination.offset ((*coding).dst_bytes);
  let safe_room: c_int = 4;
  let mut produced_chars: ptrdiff_t = 0;
  let mut attrs: Lisp_Object;
  let mut charset_list: Lisp_Object;
  let mut val: Lisp_Object;
  let mut ascii_compatible: bool;
  let mut charset_kanji: *mut charset;
  let mut charset_kana: *mut charset;
  let mut charset_kanji2: *mut charset;
  let mut c: c_int;

  macro_rules! ASSURE_DESTINATION {
    ($bytes:expr) => {
      {
	if dst.offset (($bytes) as isize) >= dst_end
	  {
	    let more_bytes: ptrdiff_t = charbuf_end.offset_from (charbuf) + ($bytes) as isize;

	    dst = alloc_destination (coding, more_bytes, dst);
	    dst_end = (*coding).destination.offset ((*coding).dst_bytes);
	  }
      }
    };
  }
  macro_rules! EMIT_ONE_ASCII_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	*dst = ($c) as c_uchar; dst = dst.offset (1);
      }
    };
  }
  macro_rules! EMIT_ONE_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	if multibytep
	  {
	    let mut ch: c_uint = ($c) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  { *dst = ($c) as c_uchar; dst = dst.offset (1); }
      }
    };
  }
  macro_rules! EMIT_TWO_BYTES {
    ($c1:expr, $c2:expr) => {
      {
	produced_chars += 2;
	if multibytep
	  {
	    let mut ch: c_uint;

	    ch = ($c1) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	    ch = ($c2) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  {
	    *dst = ($c1) as c_uchar; dst = dst.offset (1);
	    *dst = ($c2) as c_uchar; dst = dst.offset (1);
	  }
      }
    };
  }
  macro_rules! CODING_CHAR_CHARSET {
    ($coding:expr, $dst:expr, $dst_end:expr, $c:expr, $charset_list:expr, $code_return:expr, $charset:expr) => {
      {
	let offset: ptrdiff_t;

	charset_map_loaded = false;
	$charset = char_charset ($c, $charset_list, $code_return);
	if charset_map_loaded
	    && { offset = coding_change_destination ($coding); offset != 0 }
	  {
	    $dst = $dst.offset (offset);
	    $dst_end = $dst_end.offset (offset);
	  }
      }
    };
  }

  CODING_GET_INFO! (coding, attrs, charset_list);
  val = XCDR (charset_list);
  charset_kana = CHARSET_FROM_ID! (XFIXNUM (XCAR (val))); val = XCDR (val);
  charset_kanji = CHARSET_FROM_ID! (XFIXNUM (XCAR (val))); val = XCDR (val);
  charset_kanji2 = if NILP (val) { std::ptr::null_mut () } else { CHARSET_FROM_ID! (XFIXNUM (XCAR (val))) };

  ascii_compatible = ! NILP (CODING_ATTR_ASCII_COMPAT (attrs));

  while charbuf < charbuf_end
    {
      ASSURE_DESTINATION! (safe_room);
      c = *charbuf; charbuf = charbuf.offset (1);
      /* Now encode the character C.  */
      if ASCII_CHAR_P (c as isize) && ascii_compatible
	{ EMIT_ONE_ASCII_BYTE! (c); }
      else if CHAR_BYTE8_P (c)
	{
	  c = CHAR_TO_BYTE8 (c);
	  EMIT_ONE_BYTE! (c);
	}
      else
	{
	  let mut code: c_uint = 0;
	  let mut charset: *mut charset;
	  CODING_CHAR_CHARSET! (coding, dst, dst_end, c, charset_list,
				&raw mut code, charset);

	  if charset.is_null ()
	    {
	      if mode_of (coding) as c_int & CODING_MODE_SAFE_ENCODING != 0
		{
		  code = CODING_INHIBIT_CHARACTER_SUBSTITUTION as c_uint;
		  charset = CHARSET_FROM_ID! (charset_ascii);
		}
	      else
		{
		  c = (*coding).default_char;
		  CODING_CHAR_CHARSET! (coding, dst, dst_end, c,
					charset_list, &raw mut code, charset);
		}
	    }
	  if code == CHARSET_INVALID_CODE! (charset)
	    { emacs_abort (); }
	  if charset == charset_kanji
	    {
	      let mut c1: c_int; let mut c2: c_int;
	      code = JIS_TO_SJIS (code as c_int) as c_uint;
	      c1 = (code >> 8) as c_int; c2 = (code & 0xFF) as c_int;
	      EMIT_TWO_BYTES! (c1, c2);
	    }
	  else if charset == charset_kana
	    { EMIT_ONE_BYTE! (code | 0x80); }
	  else if ! charset_kanji2.is_null () && charset == charset_kanji2
	    {
	      let mut c1: c_int; let mut c2: c_int;

	      c1 = (code >> 8) as c_int;
	      if c1 == 0x21 || (c1 >= 0x23 && c1 <= 0x25)
		  || c1 == 0x28
		  || (c1 >= 0x2C && c1 <= 0x2F) || c1 >= 0x6E
		{
		  code = JIS_TO_SJIS2 (code as c_int) as c_uint;
		  c1 = (code >> 8) as c_int; c2 = (code & 0xFF) as c_int;
		  EMIT_TWO_BYTES! (c1, c2);
		}
	      else
		{ EMIT_ONE_ASCII_BYTE! (code & 0x7F); }
	    }
	  else
	    { EMIT_ONE_ASCII_BYTE! (code & 0x7F); }
	}
    }
  record_conversion_result (coding, CODING_RESULT_SUCCESS);
  (*coding).produced_char += produced_chars;
  (*coding).produced = dst.offset_from ((*coding).destination);
  return false;
}

unsafe extern "C" fn encode_coding_big5 (coding: *mut coding_system) -> bool
{
  let multibytep = dst_multibyte_p (coding);
  let mut charbuf: *mut c_int = (*coding).charbuf;
  let charbuf_end: *mut c_int = charbuf.offset ((*coding).charbuf_used as isize);
  let mut dst: *mut c_uchar = (*coding).destination.offset ((*coding).produced);
  let mut dst_end: *mut c_uchar = (*coding).destination.offset ((*coding).dst_bytes);
  let safe_room: c_int = 4;
  let mut produced_chars: ptrdiff_t = 0;
  let mut attrs: Lisp_Object;
  let mut charset_list: Lisp_Object;
  let mut val: Lisp_Object;
  let mut ascii_compatible: bool;
  let mut charset_big5: *mut charset;
  let mut c: c_int;

  macro_rules! ASSURE_DESTINATION {
    ($bytes:expr) => {
      {
	if dst.offset (($bytes) as isize) >= dst_end
	  {
	    let more_bytes: ptrdiff_t = charbuf_end.offset_from (charbuf) + ($bytes) as isize;

	    dst = alloc_destination (coding, more_bytes, dst);
	    dst_end = (*coding).destination.offset ((*coding).dst_bytes);
	  }
      }
    };
  }
  macro_rules! EMIT_ONE_ASCII_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	*dst = ($c) as c_uchar; dst = dst.offset (1);
      }
    };
  }
  macro_rules! EMIT_ONE_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	if multibytep
	  {
	    let mut ch: c_uint = ($c) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  { *dst = ($c) as c_uchar; dst = dst.offset (1); }
      }
    };
  }
  macro_rules! EMIT_TWO_BYTES {
    ($c1:expr, $c2:expr) => {
      {
	produced_chars += 2;
	if multibytep
	  {
	    let mut ch: c_uint;

	    ch = ($c1) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	    ch = ($c2) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  {
	    *dst = ($c1) as c_uchar; dst = dst.offset (1);
	    *dst = ($c2) as c_uchar; dst = dst.offset (1);
	  }
      }
    };
  }
  macro_rules! CODING_CHAR_CHARSET {
    ($coding:expr, $dst:expr, $dst_end:expr, $c:expr, $charset_list:expr, $code_return:expr, $charset:expr) => {
      {
	let offset: ptrdiff_t;

	charset_map_loaded = false;
	$charset = char_charset ($c, $charset_list, $code_return);
	if charset_map_loaded
	    && { offset = coding_change_destination ($coding); offset != 0 }
	  {
	    $dst = $dst.offset (offset);
	    $dst_end = $dst_end.offset (offset);
	  }
      }
    };
  }

  CODING_GET_INFO! (coding, attrs, charset_list);
  val = XCDR (charset_list);
  charset_big5 = CHARSET_FROM_ID! (XFIXNUM (XCAR (val)));
  ascii_compatible = ! NILP (CODING_ATTR_ASCII_COMPAT (attrs));

  while charbuf < charbuf_end
    {
      ASSURE_DESTINATION! (safe_room);
      c = *charbuf; charbuf = charbuf.offset (1);
      /* Now encode the character C.  */
      if ASCII_CHAR_P (c as isize) && ascii_compatible
	{ EMIT_ONE_ASCII_BYTE! (c); }
      else if CHAR_BYTE8_P (c)
	{
	  c = CHAR_TO_BYTE8 (c);
	  EMIT_ONE_BYTE! (c);
	}
      else
	{
	  let mut code: c_uint = 0;
	  let mut charset: *mut charset;
	  CODING_CHAR_CHARSET! (coding, dst, dst_end, c, charset_list,
				&raw mut code, charset);

	  if charset.is_null ()
	    {
	      if mode_of (coding) as c_int & CODING_MODE_SAFE_ENCODING != 0
		{
		  code = CODING_INHIBIT_CHARACTER_SUBSTITUTION as c_uint;
		  charset = CHARSET_FROM_ID! (charset_ascii);
		}
	      else
		{
		  c = (*coding).default_char;
		  CODING_CHAR_CHARSET! (coding, dst, dst_end, c,
					charset_list, &raw mut code, charset);
		}
	    }
	  if code == CHARSET_INVALID_CODE! (charset)
	    { emacs_abort (); }
	  if charset == charset_big5
	    {
	      let mut c1: c_int; let mut c2: c_int;

	      c1 = (code >> 8) as c_int; c2 = (code & 0xFF) as c_int;
	      EMIT_TWO_BYTES! (c1, c2);
	    }
	  else
	    { EMIT_ONE_ASCII_BYTE! (code & 0x7F); }
	}
    }
  record_conversion_result (coding, CODING_RESULT_SUCCESS);
  (*coding).produced_char += produced_chars;
  (*coding).produced = dst.offset_from ((*coding).destination);
  return false;
}


/* See the above "GENERAL NOTES on `detect_coding_XXX ()' functions".
   Return true if a text is encoded in a coding system of which
   encoder/decoder are written in CCL program.  */

unsafe extern "C" fn detect_coding_ccl (mut coding: *mut coding_system,
			     detect_info: *mut coding_detection_info) -> bool
{
  let mut src: *const c_uchar = (*coding).source;
  let mut src_base: *const c_uchar = std::ptr::null ();
  let src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let multibytep = src_multibyte_p (coding);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut found: c_int = 0;
  let mut valids: *mut c_uchar;
  let head_ascii: ptrdiff_t = (*coding).head_ascii;
  let mut attrs: Lisp_Object;
  let mut c: c_int;

  (*detect_info).checked |= CATEGORY_MASK_CCL;

  coding = &raw mut coding_categories[coding_category_ccl as usize];
  valids = CODING_CCL_VALIDS! (coding);
  attrs = CODING_ID_ATTRS ((*coding).id as hash_idx_t);
  if ! NILP (CODING_ATTR_ASCII_COMPAT (attrs))
    { src = src.offset (head_ascii); }

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }

    loop
      {
	src_base = src;
	ONE_MORE_BYTE! (c);
	if c < 0 || *valids.offset (c as isize) == 0
	  { break; }
	if *valids.offset (c as isize) > 1
	  { found = CATEGORY_MASK_CCL; }
      }
    (*detect_info).rejected |= CATEGORY_MASK_CCL;
    return false;
  }

  (*detect_info).found |= found;
  return true;
}

unsafe extern "C" fn decode_coding_ccl (coding: *mut coding_system)
{
  let mut src: *const c_uchar = (*coding).source.offset ((*coding).consumed);
  let mut src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let mut charbuf: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_used as isize);
  let charbuf_end: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_size as isize);
  let mut consumed_chars: ptrdiff_t = 0;
  let multibytep = src_multibyte_p (coding);
  /* coding.h 447's `struct ccl_spec *` is the mirror's ABI-equal `*mut c_void` (P-72 (3),
     because coding.h does not include ccl.h), so the type is recovered here; ccl.h 73's
     CODING_SPEC_CCL_PROGRAM cannot compile against that degraded member (reported in
     docs/ports/coding.md).  */
  let ccl: *mut ccl_program = &raw mut (*((*coding).spec.ccl as *mut ccl_spec)).ccl;
  let mut source_charbuf: [c_int; 1024] = [0; 1024];
  let mut source_byteidx: [c_int; 1025] = [0; 1025];
  let mut attrs: Lisp_Object;
  let mut charset_list: Lisp_Object;

  CODING_GET_INFO! (coding, attrs, charset_list);

  loop
    {
      let mut p: *const c_uchar = src;
      let mut offset: ptrdiff_t;
      let mut i: c_int = 0;

      if multibytep
	{
	  while i < 1024 && p < src_end
	    {
	      source_byteidx[i as usize] = p.offset_from (src) as c_int;
	      source_charbuf[i as usize] = string_char_advance (&raw mut p);
	      i += 1;
	    }
	  source_byteidx[i as usize] = p.offset_from (src) as c_int;
	}
      else
	{
	  while i < 1024 && p < src_end
	    {
	      source_charbuf[i as usize] = *p as c_int;
	      p = p.offset (1);
	      i += 1;
	    }
	}

      if p == src_end && mode_of (coding) as c_int & CODING_MODE_LAST_BLOCK != 0
	{ (*ccl).bits = (*ccl).bits | CCL_PROGRAM_LAST_BLOCK; }
      /* As ccl_driver calls DECODE_CHAR, buffer may be relocated.  */
      charset_map_loaded = false;
      ccl_driver (ccl, source_charbuf.as_mut_ptr (), charbuf, i,
		  charbuf_end.offset_from (charbuf) as c_int,
		  charset_list);
      if charset_map_loaded
	  && { offset = coding_change_source (coding); offset != 0 }
	{
	  p = p.offset (offset);
	  src = src.offset (offset);
	  src_end = src_end.offset (offset);
	}
      charbuf = charbuf.offset ((*ccl).produced as isize);
      if multibytep
	{ src = src.offset (source_byteidx[(*ccl).consumed as usize] as isize); }
      else
	{ src = src.offset ((*ccl).consumed as isize); }
      consumed_chars += (*ccl).consumed as isize;
      if p == src_end || (*ccl).status != CCL_STAT_SUSPEND_BY_SRC
	{ break; }
    }

  match (*ccl).status
    {
      CCL_STAT_SUSPEND_BY_SRC =>
	{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
      CCL_STAT_SUSPEND_BY_DST =>
	{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_DST); }
      CCL_STAT_QUIT | CCL_STAT_INVALID_CMD =>
	{ record_conversion_result (coding, CODING_RESULT_INTERRUPT); }
      _ =>
	{ record_conversion_result (coding, CODING_RESULT_SUCCESS); }
    }
  (*coding).consumed_char += consumed_chars;
  (*coding).consumed = src.offset_from ((*coding).source);
  (*coding).charbuf_used = charbuf.offset_from ((*coding).charbuf) as c_int;
}

unsafe extern "C" fn encode_coding_ccl (coding: *mut coding_system) -> bool
{
  let ccl: *mut ccl_program = &raw mut (*((*coding).spec.ccl as *mut ccl_spec)).ccl;
  let multibytep = dst_multibyte_p (coding);
  let mut charbuf: *mut c_int = (*coding).charbuf;
  let charbuf_end: *mut c_int = charbuf.offset ((*coding).charbuf_used as isize);
  let mut dst: *mut c_uchar = (*coding).destination.offset ((*coding).produced);
  let mut dst_end: *mut c_uchar = (*coding).destination.offset ((*coding).dst_bytes);
  let mut destination_charbuf: [c_int; 1024] = [0; 1024];
  let mut produced_chars: ptrdiff_t = 0;
  let mut i: c_int;
  let mut attrs: Lisp_Object;
  let mut charset_list: Lisp_Object;

  macro_rules! ASSURE_DESTINATION {
    ($bytes:expr) => {
      {
	if dst.offset (($bytes) as isize) >= dst_end
	  {
	    let more_bytes: ptrdiff_t = charbuf_end.offset_from (charbuf) + ($bytes) as isize;

	    dst = alloc_destination (coding, more_bytes, dst);
	    dst_end = (*coding).destination.offset ((*coding).dst_bytes);
	  }
      }
    };
  }
  macro_rules! EMIT_ONE_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	if multibytep
	  {
	    let mut ch: c_uint = ($c) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  { *dst = ($c) as c_uchar; dst = dst.offset (1); }
      }
    };
  }
  CODING_GET_INFO! (coding, attrs, charset_list);
  if (*coding).consumed_char == (*coding).src_chars
      && mode_of (coding) as c_int & CODING_MODE_LAST_BLOCK != 0
    { (*ccl).bits = (*ccl).bits | CCL_PROGRAM_LAST_BLOCK; }

  loop
    {
      let mut offset: ptrdiff_t;

      /* As ccl_driver calls DECODE_CHAR, buffer may be relocated.  */
      charset_map_loaded = false;
      ccl_driver (ccl, charbuf, destination_charbuf.as_mut_ptr (),
		  charbuf_end.offset_from (charbuf) as c_int, 1024, charset_list);
      if charset_map_loaded
	  && { offset = coding_change_destination (coding); offset != 0 }
	{ dst = dst.offset (offset); }
      if multibytep
	{
	  ASSURE_DESTINATION! ((*ccl).produced as c_int * 2);
	  i = 0;
	  while i < (*ccl).produced
	    {
	      EMIT_ONE_BYTE! (destination_charbuf[i as usize] & 0xFF);
	      i += 1;
	    }
	}
      else
	{
	  ASSURE_DESTINATION! ((*ccl).produced as c_int);
	  i = 0;
	  while i < (*ccl).produced
	    {
	      *dst = (destination_charbuf[i as usize] & 0xFF) as c_uchar; dst = dst.offset (1);
	      i += 1;
	    }
	  produced_chars += (*ccl).produced as isize;
	}
      charbuf = charbuf.offset ((*ccl).consumed as isize);
      if (*ccl).status == CCL_STAT_QUIT
	  || (*ccl).status == CCL_STAT_INVALID_CMD
	{ break; }
      if ! (charbuf < charbuf_end)
	{ break; }
    }

  match (*ccl).status
    {
      CCL_STAT_SUSPEND_BY_SRC =>
	{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
      CCL_STAT_SUSPEND_BY_DST =>
	{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_DST); }
      CCL_STAT_QUIT | CCL_STAT_INVALID_CMD =>
	{ record_conversion_result (coding, CODING_RESULT_INTERRUPT); }
      _ =>
	{ record_conversion_result (coding, CODING_RESULT_SUCCESS); }
    }

  (*coding).produced_char += produced_chars;
  (*coding).produced = dst.offset_from ((*coding).destination);
  return false;
}


/*** 10, 11. no-conversion handlers ***/

/* See the above "GENERAL NOTES on `decode_coding_XXX ()' functions".  */

unsafe extern "C" fn decode_coding_raw_text (coding: *mut coding_system)
{
  let eol_dos
    = !inhibit_eol_conversion && EQ (CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t), Qdos ());

  coding_bits_set (coding, CODING_SYSTEM_CHARS_AT_SOURCE_SHIFT, 1, 1);
  (*coding).consumed_char = (*coding).src_chars;
  (*coding).consumed = (*coding).src_bytes;
  if eol_dos
      && (*coding).src_bytes > 0	/* empty source text? */
      && *(*coding).source.offset ((*coding).src_bytes - 1) == '\r' as c_uchar
    {
      (*coding).consumed_char -= 1;
      (*coding).consumed -= 1;
      record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC);
    }
  else
    { record_conversion_result (coding, CODING_RESULT_SUCCESS); }
}


unsafe extern "C" fn encode_coding_raw_text (coding: *mut coding_system) -> bool
{
  let multibytep = dst_multibyte_p (coding);
  let mut charbuf: *mut c_int = (*coding).charbuf;
  let charbuf_end: *mut c_int = charbuf.offset ((*coding).charbuf_used as isize);
  let mut dst: *mut c_uchar = (*coding).destination.offset ((*coding).produced);
  let mut dst_end: *mut c_uchar = (*coding).destination.offset ((*coding).dst_bytes);
  let mut produced_chars: ptrdiff_t = 0;
  let mut c: c_int;

  macro_rules! ASSURE_DESTINATION {
    ($bytes:expr) => {
      {
	if dst.offset (($bytes) as isize) >= dst_end
	  {
	    let more_bytes: ptrdiff_t = charbuf_end.offset_from (charbuf) + ($bytes) as isize;

	    dst = alloc_destination (coding, more_bytes, dst);
	    dst_end = (*coding).destination.offset ((*coding).dst_bytes);
	  }
      }
    };
  }
  macro_rules! EMIT_ONE_ASCII_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	*dst = ($c) as c_uchar; dst = dst.offset (1);
      }
    };
  }
  macro_rules! EMIT_ONE_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	if multibytep
	  {
	    let mut ch: c_uint = ($c) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  { *dst = ($c) as c_uchar; dst = dst.offset (1); }
      }
    };
  }

  if multibytep
    {
      let safe_room: c_int = MAX_MULTIBYTE_LENGTH * 2;

      if src_multibyte_p (coding)
	{
	  while charbuf < charbuf_end
	    {
	      ASSURE_DESTINATION! (safe_room);
	      c = *charbuf; charbuf = charbuf.offset (1);
	      if ASCII_CHAR_P (c as isize)
		{ EMIT_ONE_ASCII_BYTE! (c); }
	      else if CHAR_BYTE8_P (c)
		{
		  c = CHAR_TO_BYTE8 (c);
		  EMIT_ONE_BYTE! (c);
		}
	      else
		{
		  let mut str: [c_uchar; MAX_MULTIBYTE_LENGTH as usize] = [0; MAX_MULTIBYTE_LENGTH as usize];
		  let len: c_int = CHAR_STRING (c, str.as_mut_ptr ());
		  let mut i: c_int;

		  i = 0;
		  while i < len
		    {
		      EMIT_ONE_BYTE! (str[i as usize]);
		      i += 1;
		    }
		}
	    }
	}
      else
	{
	  while charbuf < charbuf_end
	    {
	      ASSURE_DESTINATION! (safe_room);
	      c = *charbuf; charbuf = charbuf.offset (1);
	      EMIT_ONE_BYTE! (c);
	    }
	}
    }
  else
    {
      if src_multibyte_p (coding)
	{
	  let safe_room: c_int = MAX_MULTIBYTE_LENGTH;

	  while charbuf < charbuf_end
	    {
	      ASSURE_DESTINATION! (safe_room);
	      c = *charbuf; charbuf = charbuf.offset (1);
	      if ASCII_CHAR_P (c as isize)
		{ *dst = c as c_uchar; dst = dst.offset (1); }
	      else if CHAR_BYTE8_P (c)
		{ *dst = CHAR_TO_BYTE8 (c) as c_uchar; dst = dst.offset (1); }
	      else
		{ dst = dst.offset (CHAR_STRING (c, dst) as isize); }
	    }
	}
      else
	{
	  ASSURE_DESTINATION! (charbuf_end.offset_from (charbuf) as c_int);
	  while charbuf < charbuf_end && dst < dst_end
	    {
	      *dst = *charbuf as c_uchar; dst = dst.offset (1); charbuf = charbuf.offset (1);
	    }
	}
      produced_chars = dst.offset_from ((*coding).destination.offset ((*coding).produced));
    }
  record_conversion_result (coding, CODING_RESULT_SUCCESS);
  (*coding).produced_char += produced_chars;
  (*coding).produced = dst.offset_from ((*coding).destination);
  return false;
}

/* See the above "GENERAL NOTES on `detect_coding_XXX ()' functions".
   Return true if a text is encoded in a charset-based coding system.  */

unsafe extern "C" fn detect_coding_charset (mut coding: *mut coding_system,
				 detect_info: *mut coding_detection_info) -> bool
{
  let mut src: *const c_uchar = (*coding).source;
  let mut src_base: *const c_uchar = std::ptr::null ();
  let src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let multibytep = src_multibyte_p (coding);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut attrs: Lisp_Object;
  let mut valids: Lisp_Object;
  let mut name: Lisp_Object;
  let mut found: c_int = 0;
  let head_ascii: ptrdiff_t = (*coding).head_ascii;
  let mut check_latin_extra: bool = false;

  (*detect_info).checked |= CATEGORY_MASK_CHARSET;

  coding = &raw mut coding_categories[coding_category_charset as usize];
  attrs = CODING_ID_ATTRS ((*coding).id as hash_idx_t);
  valids = AREF (attrs, coding_attr_charset_valids as isize);
  name = CODING_ID_NAME ((*coding).id as hash_idx_t);
  /* C spells both counts as `sizeof ("…") - 1`; the port's `size_of_val` of the same
     literal is that count (the `sizeof` family in docs/ports/token-aliases.tsv).  */
  if strncmp (SSDATA (SYMBOL_NAME (name)), c"iso-8859-".as_ptr (),
	      (std::mem::size_of_val (b"iso-8859-") - 1)) == 0
      || strncmp (SSDATA (SYMBOL_NAME (name)), c"iso-latin-".as_ptr (),
		  (std::mem::size_of_val (b"iso-latin-") - 1)) == 0
    { check_latin_extra = true; }

  if ! NILP (CODING_ATTR_ASCII_COMPAT (attrs))
    { src = src.offset (head_ascii); }

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }

    'too_short: {
      'detect_loop: loop
	{
	  let mut c: c_int;
	  let mut val: Lisp_Object;
	  let mut charset: *mut charset;
	  let mut dim: c_int;
	  let mut idx: c_int;

	  src_base = src;
	  ONE_MORE_BYTE! (c);
	  if c < 0
	    { continue 'detect_loop; }
	  val = AREF (valids, c as isize);
	  if NILP (val)
	    { break; }
	  if c >= 0x80
	    {
	      if c < 0xA0
		  && check_latin_extra
		  && (! VECTORP (Vlatin_extra_code_table)
		      || NILP (AREF (Vlatin_extra_code_table, c as isize)))
		{ break; }
	      found = CATEGORY_MASK_CHARSET;
	    }
	  if FIXNUMP (val)
	    {
	      charset = CHARSET_FROM_ID! (XFIXNAT (val));
	      dim = CHARSET_DIMENSION! (charset);
	      idx = 1;
	      while idx < dim
		{
		  if src == src_end
		    { break 'too_short; }
		  ONE_MORE_BYTE! (c);
		  if c < (*charset).code_space[((dim - 1 - idx) * 4) as usize]
		      || c > (*charset).code_space[((dim - 1 - idx) * 4 + 1) as usize]
		    { break; }
		  idx += 1;
		}
	      if idx < dim
		{ break; }
	    }
	  else
	    {
	      idx = 1;
	      while CONSP (val)
		{
		  charset = CHARSET_FROM_ID! (XFIXNAT (XCAR (val)));
		  dim = CHARSET_DIMENSION! (charset);
		  while idx < dim
		    {
		      if src == src_end
			{ break 'too_short; }
		      ONE_MORE_BYTE! (c);
		      if c < (*charset).code_space[((dim - 1 - idx) * 4) as usize]
			  || c > (*charset).code_space[((dim - 1 - idx) * 4 + 1) as usize]
			{ break; }
		      idx += 1;
		    }
		  if idx == dim
		    {
		      val = Qnil ();
		      break;
		    }
		  val = XCDR (val);
		}
	      if CONSP (val)
		{ break; }
	    }
	}
    }
    (*detect_info).rejected |= CATEGORY_MASK_CHARSET;
    return false;
  }

  (*detect_info).found |= found;
  return true;
}

unsafe extern "C" fn decode_coding_charset (coding: *mut coding_system)
{
  let mut src: *const c_uchar = (*coding).source.offset ((*coding).consumed);
  let mut src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let mut src_base: *const c_uchar = std::ptr::null ();
  let mut charbuf: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_used as isize);
  /* We may produce one charset annotation in one loop and one more at
     the end.  */
  let charbuf_end: *mut c_int
    = (*coding).charbuf.offset (((*coding).charbuf_size - (MAX_ANNOTATION_LENGTH * 2)) as isize);
  let mut consumed_chars: ptrdiff_t = 0;
  let mut consumed_chars_base: ptrdiff_t = 0;
  let multibytep = src_multibyte_p (coding);
  let attrs: Lisp_Object = CODING_ID_ATTRS ((*coding).id as hash_idx_t);
  let mut valids: Lisp_Object;
  let mut char_offset: ptrdiff_t = (*coding).produced_char;
  let mut last_offset: ptrdiff_t = char_offset;
  let mut last_id: c_int = charset_ascii;
  let eol_dos
    = !inhibit_eol_conversion && EQ (CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t), Qdos ());
  let mut byte_after_cr: c_int = -1;

  valids = AREF (attrs, coding_attr_charset_valids as isize);

  macro_rules! ADD_ANNOTATION_DATA {
    ($buf:expr, $len:expr, $mask:expr, $nchars:expr) => {
      {
	*$buf = -($len); $buf = $buf.offset (1);
	*$buf = ($mask); $buf = $buf.offset (1);
	*$buf = ($nchars) as c_int; $buf = $buf.offset (1);
	coding_bits_set (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1, 1);
      }
    };
  }
  macro_rules! ADD_CHARSET_DATA {
    ($buf:expr, $nchars:expr, $id:expr) => {
      {
	ADD_ANNOTATION_DATA! ($buf, 4, CODING_ANNOTATE_CHARSET_MASK as c_int, $nchars);
	*$buf = $id; $buf = $buf.offset (1);
      }
    };
  }

  'no_more_source: {
    macro_rules! ONE_MORE_BYTE {
      ($c:ident) => {
	{
	  if src == src_end
	    {
	      if src_base < src
		{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
	      break 'no_more_source;
	    }
	  $c = *src as c_int;
	  src = src.offset (1);
	  if multibytep && ($c & 0x80) != 0
	    {
	      if ($c & 0xFE) == 0xC0
		{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
	      else
		{
		  src = src.offset (-1);
		  $c = - string_char_advance (&raw mut src);
		  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
		}
	    }
	  consumed_chars += 1;
	}
      };
    }
    macro_rules! CODING_DECODE_CHAR {
      ($coding:expr, $src:expr, $src_base:expr, $src_end:expr, $charset:expr, $code:expr, $c:expr) => {
	{
	  let offset: ptrdiff_t;

	  charset_map_loaded = false;
	  $c = DECODE_CHAR! ($charset, $code);
	  if charset_map_loaded
	      && { offset = coding_change_source ($coding); offset != 0 }
	    {
	      $src = $src.offset (offset);
	      $src_base = $src_base.offset (offset);
	      $src_end = $src_end.offset (offset);
	    }
	}
      };
    }

    'decode_loop: loop
      {
	let mut c: c_int;
	let mut val: Lisp_Object;
	/* C: `struct charset *charset UNINIT;` — `UNINIT` is conf_post.h's zero marker in the
	   frozen configuration (PORTING.md 1.5).  */
	let mut charset: *mut charset = std::ptr::null_mut ();
	let mut dim: c_int;
	let mut len: c_int = 1;
	let mut code: c_uint = 0;

	src_base = src;
	consumed_chars_base = consumed_chars;

	if charbuf >= charbuf_end
	  {
	    if byte_after_cr >= 0
	      { src_base = src_base.offset (-1); }
	    break;
	  }

	'invalid_code: {
	  if byte_after_cr >= 0
	    {
	      c = byte_after_cr;
	      byte_after_cr = -1;
	    }
	  else
	    {
	      ONE_MORE_BYTE! (c);
	      if eol_dos && c == '\r' as c_int
		{ ONE_MORE_BYTE! (byte_after_cr); }
	    }
	  if c < 0
	    { break 'invalid_code; }
	  code = c as c_uint;

	  val = AREF (valids, c as isize);
	  if ! FIXNUMP (val) && ! CONSP (val)
	    { break 'invalid_code; }
	  if FIXNUMP (val)
	    {
	      charset = CHARSET_FROM_ID! (XFIXNAT (val));
	      dim = CHARSET_DIMENSION! (charset);
	      while len < dim
		{
		  ONE_MORE_BYTE! (c);
		  code = (code << 8) | c as c_uint;
		  len += 1;
		}
	      CODING_DECODE_CHAR! (coding, src, src_base, src_end,
				   charset, code, c);
	    }
	  else
	    {
	      /* VAL is a list of charset IDs.  It is assured that the
		 list is sorted by charset dimensions (smaller one
		 comes first).  */
	      while CONSP (val)
		{
		  charset = CHARSET_FROM_ID! (XFIXNAT (XCAR (val)));
		  dim = CHARSET_DIMENSION! (charset);
		  while len < dim
		    {
		      ONE_MORE_BYTE! (c);
		      code = (code << 8) | c as c_uint;
		      len += 1;
		    }
		  CODING_DECODE_CHAR! (coding, src, src_base,
				       src_end, charset, code, c);
		  if c >= 0
		    { break; }
		  val = XCDR (val);
		}
	    }
	  if c < 0
	    { break 'invalid_code; }
	  if (*charset).id != charset_ascii
	      && last_id != (*charset).id
	    {
	      if last_id != charset_ascii
		{ ADD_CHARSET_DATA! (charbuf, char_offset - last_offset, last_id); }
	      last_id = (*charset).id;
	      last_offset = char_offset;
	    }

	  *charbuf = c; charbuf = charbuf.offset (1);
	  char_offset += 1;
	  continue 'decode_loop;
	}

	/* invalid_code: */
	src = src_base;
	consumed_chars = consumed_chars_base;
	ONE_MORE_BYTE! (c);
	*charbuf = if c < 0 { -c } else if ASCII_CHAR_P (c as isize) { c } else { BYTE8_TO_CHAR (c) };
	charbuf = charbuf.offset (1);
	char_offset += 1;
      }
  }

  if last_id != charset_ascii
    { ADD_CHARSET_DATA! (charbuf, char_offset - last_offset, last_id); }
  (*coding).consumed_char += consumed_chars_base;
  (*coding).consumed = src_base.offset_from ((*coding).source);
  (*coding).charbuf_used = charbuf.offset_from ((*coding).charbuf) as c_int;
}

unsafe extern "C" fn encode_coding_charset (coding: *mut coding_system) -> bool
{
  let multibytep = dst_multibyte_p (coding);
  let mut charbuf: *mut c_int = (*coding).charbuf;
  let charbuf_end: *mut c_int = charbuf.offset ((*coding).charbuf_used as isize);
  let mut dst: *mut c_uchar = (*coding).destination.offset ((*coding).produced);
  let mut dst_end: *mut c_uchar = (*coding).destination.offset ((*coding).dst_bytes);
  let safe_room: c_int = MAX_MULTIBYTE_LENGTH;
  let mut produced_chars: ptrdiff_t = 0;
  let mut attrs: Lisp_Object;
  let mut charset_list: Lisp_Object;
  let mut ascii_compatible: bool;
  let mut c: c_int;

  macro_rules! ASSURE_DESTINATION {
    ($bytes:expr) => {
      {
	if dst.offset (($bytes) as isize) >= dst_end
	  {
	    let more_bytes: ptrdiff_t = charbuf_end.offset_from (charbuf) + ($bytes) as isize;

	    dst = alloc_destination (coding, more_bytes, dst);
	    dst_end = (*coding).destination.offset ((*coding).dst_bytes);
	  }
      }
    };
  }
  macro_rules! EMIT_ONE_ASCII_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	*dst = ($c) as c_uchar; dst = dst.offset (1);
      }
    };
  }
  macro_rules! EMIT_ONE_BYTE {
    ($c:expr) => {
      {
	produced_chars += 1;
	if multibytep
	  {
	    let mut ch: c_uint = ($c) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  { *dst = ($c) as c_uchar; dst = dst.offset (1); }
      }
    };
  }
  macro_rules! EMIT_TWO_BYTES {
    ($c1:expr, $c2:expr) => {
      {
	produced_chars += 2;
	if multibytep
	  {
	    let mut ch: c_uint;

	    ch = ($c1) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	    ch = ($c2) as c_uint;
	    if ch >= 0x80
	      { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
	    dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
	  }
	else
	  {
	    *dst = ($c1) as c_uchar; dst = dst.offset (1);
	    *dst = ($c2) as c_uchar; dst = dst.offset (1);
	  }
      }
    };
  }
  macro_rules! EMIT_THREE_BYTES {
    ($c1:expr, $c2:expr, $c3:expr) => {
      {
	EMIT_ONE_BYTE! ($c1);
	EMIT_TWO_BYTES! ($c2, $c3);
      }
    };
  }
  macro_rules! EMIT_FOUR_BYTES {
    ($c1:expr, $c2:expr, $c3:expr, $c4:expr) => {
      {
	EMIT_TWO_BYTES! ($c1, $c2);
	EMIT_TWO_BYTES! ($c3, $c4);
      }
    };
  }
  macro_rules! CODING_CHAR_CHARSET {
    ($coding:expr, $dst:expr, $dst_end:expr, $c:expr, $charset_list:expr, $code_return:expr, $charset:expr) => {
      {
	let offset: ptrdiff_t;

	charset_map_loaded = false;
	$charset = char_charset ($c, $charset_list, $code_return);
	if charset_map_loaded
	    && { offset = coding_change_destination ($coding); offset != 0 }
	  {
	    $dst = $dst.offset (offset);
	    $dst_end = $dst_end.offset (offset);
	  }
      }
    };
  }

  CODING_GET_INFO! (coding, attrs, charset_list);
  ascii_compatible = ! NILP (CODING_ATTR_ASCII_COMPAT (attrs));

  while charbuf < charbuf_end
    {
      let mut charset: *mut charset;
      let mut code: c_uint = 0;

      ASSURE_DESTINATION! (safe_room);
      c = *charbuf; charbuf = charbuf.offset (1);
      if ascii_compatible && ASCII_CHAR_P (c as isize)
	{ EMIT_ONE_ASCII_BYTE! (c); }
      else if CHAR_BYTE8_P (c)
	{
	  c = CHAR_TO_BYTE8 (c);
	  EMIT_ONE_BYTE! (c);
	}
      else
	{
	  CODING_CHAR_CHARSET! (coding, dst, dst_end, c, charset_list,
				&raw mut code, charset);

	  if ! charset.is_null ()
	    {
	      if CHARSET_DIMENSION! (charset) == 1
		{ EMIT_ONE_BYTE! (code); }
	      else if CHARSET_DIMENSION! (charset) == 2
		{ EMIT_TWO_BYTES! (code >> 8, code & 0xFF); }
	      else if CHARSET_DIMENSION! (charset) == 3
		{ EMIT_THREE_BYTES! (code >> 16, (code >> 8) & 0xFF, code & 0xFF); }
	      else
		{ EMIT_FOUR_BYTES! (code >> 24, (code >> 16) & 0xFF,
				    (code >> 8) & 0xFF, code & 0xFF); }
	    }
	  else
	    {
	      if mode_of (coding) as c_int & CODING_MODE_SAFE_ENCODING != 0
		{ c = CODING_INHIBIT_CHARACTER_SUBSTITUTION as c_int; }
	      else
		{ c = (*coding).default_char; }
	      EMIT_ONE_BYTE! (c);
	    }
	}
    }

  record_conversion_result (coding, CODING_RESULT_SUCCESS);
  (*coding).produced_char += produced_chars;
  (*coding).produced = dst.offset_from ((*coding).destination);
  return false;
}

/* Setup coding context CODING from information about CODING_SYSTEM.
   If CODING_SYSTEM is nil, `no-conversion' is assumed.  If
   CODING_SYSTEM is invalid, signal an error.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_coding_system (coding_system: Lisp_Object,
					      coding: *mut coding_system)
{
  let mut coding_system: Lisp_Object = coding_system;
  let mut attrs: Lisp_Object;
  let mut eol_type: Lisp_Object;
  let mut coding_type: Lisp_Object;
  let mut val: Lisp_Object;

  if NILP (coding_system)
    { coding_system = Qundecided (); }

  (*coding).id = CHECK_CODING_SYSTEM_GET_ID (coding_system);

  attrs = CODING_ID_ATTRS ((*coding).id as hash_idx_t);
  eol_type = if inhibit_eol_conversion { Qunix () } else { CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t) };

  coding_bits_set (coding, CODING_SYSTEM_MODE_SHIFT, CODING_SYSTEM_MODE_MASK, 0);
  if VECTORP (eol_type)
    { common_flags_set (coding, CODING_REQUIRE_DECODING_MASK
			| CODING_REQUIRE_DETECTION_MASK); }
  else if ! EQ (eol_type, Qunix ())
    { common_flags_set (coding, CODING_REQUIRE_DECODING_MASK
			| CODING_REQUIRE_ENCODING_MASK); }
  else
    { common_flags_set (coding, 0); }
  if ! NILP (CODING_ATTR_POST_READ (attrs))
    { common_flags_set (coding, common_flags_of (coding) | CODING_REQUIRE_DECODING_MASK); }
  if ! NILP (CODING_ATTR_PRE_WRITE (attrs))
    { common_flags_set (coding, common_flags_of (coding) | CODING_REQUIRE_ENCODING_MASK); }
  if ! NILP (CODING_ATTR_FOR_UNIBYTE (attrs))
    { common_flags_set (coding, common_flags_of (coding) | CODING_FOR_UNIBYTE_MASK); }

  val = CODING_ATTR_SAFE_CHARSETS (attrs);
  (*coding).max_charset_id = SCHARS (val) as c_int - 1;
  (*coding).safe_charsets = SDATA (val);
  (*coding).default_char = XFIXNUM (CODING_ATTR_DEFAULT_CHAR (attrs)) as c_int;
  (*coding).carryover_bytes = 0;
  coding_bits_set (coding, CODING_SYSTEM_RAW_DESTINATION_SHIFT, 1, 0);
  coding_bits_set (coding, CODING_SYSTEM_INSERT_BEFORE_MARKERS_SHIFT, 1, 0);

  coding_type = CODING_ATTR_TYPE (attrs);
  if EQ (coding_type, Qundecided ())
    {
      (*coding).detector = None;
      (*coding).decoder = Some (decode_coding_raw_text);
      (*coding).encoder = Some (encode_coding_raw_text);
      common_flags_set (coding, common_flags_of (coding) | CODING_REQUIRE_DETECTION_MASK);
      (*coding).spec.undecided.inhibit_nbd
	= encode_inhibit_flag
	    (AREF (attrs, coding_attr_undecided_inhibit_null_byte_detection as isize));
      (*coding).spec.undecided.inhibit_ied
	= encode_inhibit_flag
	    (AREF (attrs, coding_attr_undecided_inhibit_iso_escape_detection as isize));
      (*coding).spec.undecided.prefer_utf_8
	= ! NILP (AREF (attrs, coding_attr_undecided_prefer_utf_8 as isize));
    }
  else if EQ (coding_type, Qiso_2022 ())
    {
      let mut i: c_int;
      let flags: c_int = XFIXNUM (AREF (attrs, coding_attr_iso_flags as isize)) as c_int;

      /* Invoke graphic register 0 to plane 0.  */
      CODING_ISO_INVOCATION! (coding, 0) = 0;
      /* Invoke graphic register 1 to plane 1 if we can use 8-bit.  */
      CODING_ISO_INVOCATION! (coding, 1)
	= if (flags & CODING_ISO_FLAG_SEVEN_BITS) != 0 { -1 } else { 1 };
      /* Setup the initial status of designation.  */
      i = 0;
      while i < 4
	{
	  CODING_ISO_DESIGNATION! (coding, i) = CODING_ISO_INITIAL! (coding, i) as c_int;
	  i += 1;
	}
      /* Not single shifting initially.  */
      CODING_ISO_SINGLE_SHIFTING! (coding).set (0);
      /* Beginning of buffer should also be regarded as bol. */
      CODING_ISO_BOL! (coding).set (1);
      (*coding).detector = Some (detect_coding_iso_2022);
      (*coding).decoder = Some (decode_coding_iso_2022);
      (*coding).encoder = Some (encode_coding_iso_2022);
      if (flags & CODING_ISO_FLAG_SAFE) != 0
	{ coding_mode_set (coding, CODING_MODE_SAFE_ENCODING); }
      common_flags_set (coding, common_flags_of (coding)
			| (CODING_REQUIRE_DECODING_MASK | CODING_REQUIRE_ENCODING_MASK
			   | CODING_REQUIRE_FLUSHING_MASK));
      if (flags & CODING_ISO_FLAG_COMPOSITION) != 0
	{ common_flags_set (coding, common_flags_of (coding) | CODING_ANNOTATE_COMPOSITION_MASK); }
      if (flags & CODING_ISO_FLAG_DESIGNATION) != 0
	{ common_flags_set (coding, common_flags_of (coding) | CODING_ANNOTATE_CHARSET_MASK); }
      if (flags & CODING_ISO_FLAG_FULL_SUPPORT) != 0
	{
	  setup_iso_safe_charsets (attrs);
	  val = CODING_ATTR_SAFE_CHARSETS (attrs);
	  (*coding).max_charset_id = SCHARS (val) as c_int - 1;
	  (*coding).safe_charsets = SDATA (val);
	}
      CODING_ISO_FLAGS! (coding) = flags as c_uint;
      (*CODING_ISO_CMP_STATUS! (coding)).state = COMPOSING_NO;
      (*CODING_ISO_CMP_STATUS! (coding)).method = COMPOSITION_NO;
      CODING_ISO_EXTSEGMENT_LEN! (coding) = 0;
      CODING_ISO_EMBEDDED_UTF_8! (coding).set (0);
    }
  else if EQ (coding_type, Qcharset ())
    {
      (*coding).detector = Some (detect_coding_charset);
      (*coding).decoder = Some (decode_coding_charset);
      (*coding).encoder = Some (encode_coding_charset);
      common_flags_set (coding, common_flags_of (coding)
			| (CODING_REQUIRE_DECODING_MASK | CODING_REQUIRE_ENCODING_MASK));
    }
  else if EQ (coding_type, Qutf_8 ())
    {
      val = AREF (attrs, coding_attr_utf_bom as isize);
      CODING_UTF_8_BOM! (coding) = if CONSP (val) { utf_detect_bom }
				   else if EQ (val, Qt ()) { utf_with_bom }
				   else { utf_without_bom };
      (*coding).detector = Some (detect_coding_utf_8);
      (*coding).decoder = Some (decode_coding_utf_8);
      (*coding).encoder = Some (encode_coding_utf_8);
      common_flags_set (coding, common_flags_of (coding)
			| (CODING_REQUIRE_DECODING_MASK | CODING_REQUIRE_ENCODING_MASK));
      if CODING_UTF_8_BOM! (coding) == utf_detect_bom
	{ common_flags_set (coding, common_flags_of (coding) | CODING_REQUIRE_DETECTION_MASK); }
    }
  else if EQ (coding_type, Qutf_16 ())
    {
      val = AREF (attrs, coding_attr_utf_bom as isize);
      CODING_UTF_16_BOM! (coding) = if CONSP (val) { utf_detect_bom }
				    else if EQ (val, Qt ()) { utf_with_bom }
				    else { utf_without_bom };
      val = AREF (attrs, coding_attr_utf_16_endian as isize);
      CODING_UTF_16_ENDIAN! (coding) = if EQ (val, Qbig ()) { utf_16_big_endian }
				       else { utf_16_little_endian };
      CODING_UTF_16_SURROGATE! (coding) = 0;
      (*coding).detector = Some (detect_coding_utf_16);
      (*coding).decoder = Some (decode_coding_utf_16);
      (*coding).encoder = Some (encode_coding_utf_16);
      common_flags_set (coding, common_flags_of (coding)
			| (CODING_REQUIRE_DECODING_MASK | CODING_REQUIRE_ENCODING_MASK));
      if CODING_UTF_16_BOM! (coding) == utf_detect_bom
	{ common_flags_set (coding, common_flags_of (coding) | CODING_REQUIRE_DETECTION_MASK); }
    }
  else if EQ (coding_type, Qccl ())
    {
      (*coding).detector = Some (detect_coding_ccl);
      (*coding).decoder = Some (decode_coding_ccl);
      (*coding).encoder = Some (encode_coding_ccl);
      common_flags_set (coding, common_flags_of (coding)
			| (CODING_REQUIRE_DECODING_MASK | CODING_REQUIRE_ENCODING_MASK
			   | CODING_REQUIRE_FLUSHING_MASK));
    }
  else if EQ (coding_type, Qemacs_mule ())
    {
      (*coding).detector = Some (detect_coding_emacs_mule);
      (*coding).decoder = Some (decode_coding_emacs_mule);
      (*coding).encoder = Some (encode_coding_emacs_mule);
      common_flags_set (coding, common_flags_of (coding)
			| (CODING_REQUIRE_DECODING_MASK | CODING_REQUIRE_ENCODING_MASK));
      if ! NILP (AREF (attrs, coding_attr_emacs_mule_full as isize))
	  && ! EQ (CODING_ATTR_CHARSET_LIST (attrs), Vemacs_mule_charset_list)
	{
	  let mut tail: Lisp_Object;
	  let mut safe_charsets: Lisp_Object;
	  let mut max_charset_id: c_int = 0;

	  tail = Vemacs_mule_charset_list;
	  while CONSP (tail)
	    {
	      if max_charset_id < XFIXNAT (XCAR (tail)) as c_int
		{ max_charset_id = XFIXNAT (XCAR (tail)) as c_int; }
	      tail = XCDR (tail);
	    }
	  safe_charsets = make_uninit_string ((max_charset_id + 1) as EMACS_INT);
	  memset (SDATA (safe_charsets) as *mut c_void, 255, (max_charset_id + 1) as usize);
	  tail = Vemacs_mule_charset_list;
	  while CONSP (tail)
	    {
	      SSET (safe_charsets, XFIXNAT (XCAR (tail)) as usize, 0);
	      tail = XCDR (tail);
	    }
	  (*coding).max_charset_id = max_charset_id;
	  (*coding).safe_charsets = SDATA (safe_charsets);
	}
      (*coding).spec.emacs_mule.cmp_status.state = COMPOSING_NO;
      (*coding).spec.emacs_mule.cmp_status.method = COMPOSITION_NO;
    }
  else if EQ (coding_type, Qshift_jis ())
    {
      (*coding).detector = Some (detect_coding_sjis);
      (*coding).decoder = Some (decode_coding_sjis);
      (*coding).encoder = Some (encode_coding_sjis);
      common_flags_set (coding, common_flags_of (coding)
			| (CODING_REQUIRE_DECODING_MASK | CODING_REQUIRE_ENCODING_MASK));
    }
  else if EQ (coding_type, Qbig5 ())
    {
      (*coding).detector = Some (detect_coding_big5);
      (*coding).decoder = Some (decode_coding_big5);
      (*coding).encoder = Some (encode_coding_big5);
      common_flags_set (coding, common_flags_of (coding)
			| (CODING_REQUIRE_DECODING_MASK | CODING_REQUIRE_ENCODING_MASK));
    }
  else				/* EQ (coding_type, Qraw_text) */
    {
      (*coding).detector = None;
      (*coding).decoder = Some (decode_coding_raw_text);
      (*coding).encoder = Some (encode_coding_raw_text);
      if ! EQ (eol_type, Qunix ())
	{
	  common_flags_set (coding, common_flags_of (coding) | CODING_REQUIRE_DECODING_MASK);
	  if ! VECTORP (eol_type)
	    { common_flags_set (coding, common_flags_of (coding) | CODING_REQUIRE_ENCODING_MASK); }
	}

    }

  return;
}

/* Return a list of charsets supported by CODING.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn coding_charset_list (coding: *mut coding_system) -> Lisp_Object
{
  let mut attrs: Lisp_Object;
  let mut charset_list: Lisp_Object;

  CODING_GET_INFO! (coding, attrs, charset_list);
  if EQ (CODING_ATTR_TYPE (attrs), Qiso_2022 ())
    {
      let flags: c_int = XFIXNUM (AREF (attrs, coding_attr_iso_flags as isize)) as c_int;

      if (flags & CODING_ISO_FLAG_FULL_SUPPORT) != 0
	{ charset_list = Viso_2022_charset_list; }
    }
  else if EQ (CODING_ATTR_TYPE (attrs), Qemacs_mule ())
    {
      charset_list = Vemacs_mule_charset_list;
    }
  return charset_list;
}


/* Return a list of charsets supported by CODING-SYSTEM.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn coding_system_charset_list (coding_system: Lisp_Object) -> Lisp_Object
{
  let mut id: ptrdiff_t;
  let mut attrs: Lisp_Object;
  let mut charset_list: Lisp_Object;

  id = CHECK_CODING_SYSTEM_GET_ID (coding_system);
  attrs = CODING_ID_ATTRS (id as hash_idx_t);

  if EQ (CODING_ATTR_TYPE (attrs), Qiso_2022 ())
    {
      let flags: c_int = XFIXNUM (AREF (attrs, coding_attr_iso_flags as isize)) as c_int;

      if (flags & CODING_ISO_FLAG_FULL_SUPPORT) != 0
	{ charset_list = Viso_2022_charset_list; }
      else
	{ charset_list = CODING_ATTR_CHARSET_LIST (attrs); }
    }
  else if EQ (CODING_ATTR_TYPE (attrs), Qemacs_mule ())
    {
      charset_list = Vemacs_mule_charset_list;
    }
  else
    {
      charset_list = CODING_ATTR_CHARSET_LIST (attrs);
    }
  return charset_list;
}


/* Return raw-text or one of its subsidiaries that has the same
   eol_type as CODING-SYSTEM.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn raw_text_coding_system (coding_system: Lisp_Object) -> Lisp_Object
{
  let mut spec: Lisp_Object;
  let mut attrs: Lisp_Object;
  let eol_type: Lisp_Object;
  let raw_text_eol_type: Lisp_Object;

  if NILP (coding_system)
    { return Qraw_text (); }
  spec = CODING_SYSTEM_SPEC (coding_system);
  attrs = AREF (spec, 0);

  if EQ (CODING_ATTR_TYPE (attrs), Qraw_text ())
    { return coding_system; }

  eol_type = AREF (spec, 2);
  if VECTORP (eol_type)
    { return Qraw_text (); }
  spec = CODING_SYSTEM_SPEC (Qraw_text ());
  raw_text_eol_type = AREF (spec, 2);
  return if EQ (eol_type, Qunix ()) { AREF (raw_text_eol_type, 0) }
	 else if EQ (eol_type, Qdos ()) { AREF (raw_text_eol_type, 1) }
	 else { AREF (raw_text_eol_type, 2) };
}

/* Return true if CODING corresponds to raw-text coding-system.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn raw_text_coding_system_p (coding: *mut coding_system) -> bool
{
  return if (*coding).decoder == Some (decode_coding_raw_text)
	   && (*coding).encoder == Some (encode_coding_raw_text) { true } else { false };
}


/* If CODING_SYSTEM doesn't specify end-of-line format, return one of
   the subsidiary that has the same eol-spec as PARENT (if it is not
   nil and specifies end-of-line format) or the system's setting.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn coding_inherit_eol_type (coding_system: Lisp_Object,
						  parent: Lisp_Object) -> Lisp_Object
{
  let mut coding_system: Lisp_Object = coding_system;
  let spec: Lisp_Object;
  let eol_type: Lisp_Object;

  if NILP (coding_system)
    { coding_system = Qraw_text (); }
  else
    { CHECK_CODING_SYSTEM (coding_system); }
  spec = CODING_SYSTEM_SPEC (coding_system);
  eol_type = AREF (spec, 2);
  if VECTORP (eol_type)
    {
      /* Format of end-of-line decided by system.
	 This is Qunix on Unix and Mac, Qdos on DOS/Windows.
	 This has an effect only for external encoding (i.e., for output to
	 file and process), not for in-buffer or Lisp string encoding.  */
      let system_eol_type: Lisp_Object = Qunix ();

      let mut parent_eol_type: Lisp_Object = system_eol_type;
      if ! NILP (parent)
	{
	  CHECK_CODING_SYSTEM (parent);
	  let parent_spec: Lisp_Object = CODING_SYSTEM_SPEC (parent);
	  let pspec_type: Lisp_Object = AREF (parent_spec, 2);
	  if ! VECTORP (pspec_type)
	    { parent_eol_type = pspec_type; }
	}
      if EQ (parent_eol_type, Qunix ())
	{ coding_system = AREF (eol_type, 0); }
      else if EQ (parent_eol_type, Qdos ())
	{ coding_system = AREF (eol_type, 1); }
      else if EQ (parent_eol_type, Qmac ())
	{ coding_system = AREF (eol_type, 2); }
    }
  return coding_system;
}


/* Check if text-conversion and eol-conversion of CODING_SYSTEM are
   decided for writing to a process.  If not, complement them, and
   return a new coding system.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn complement_process_encoding_system (coding_system: Lisp_Object) -> Lisp_Object
{
  let mut coding_system: Lisp_Object = coding_system;
  let mut coding_base: Lisp_Object = Qnil ();
  let mut eol_base: Lisp_Object = Qnil ();
  let mut spec: Lisp_Object;
  let mut attrs: Lisp_Object;
  let mut i: c_int;

  i = 0;
  while i < 3
    {
      if i == 1
	{ coding_system = CDR_SAFE (Vdefault_process_coding_system); }
      else if i == 2
	{ coding_system = preferred_coding_system (); }
      spec = CODING_SYSTEM_SPEC (coding_system);
      if NILP (spec)
	{ i += 1; continue; }
      attrs = AREF (spec, 0);
      if NILP (coding_base) && ! EQ (CODING_ATTR_TYPE (attrs), Qundecided ())
	{ coding_base = CODING_ATTR_BASE_NAME (attrs); }
      if NILP (eol_base) && ! VECTORP (AREF (spec, 2))
	{ eol_base = coding_system; }
      if ! NILP (coding_base) && ! NILP (eol_base)
	{ break; }
      i += 1;
    }

  if i > 0
    /* The original CODING_SYSTEM didn't specify text-conversion or
       eol-conversion.  Be sure that we return a fully complemented
       coding system.  */
    { coding_system = coding_inherit_eol_type (coding_base, eol_base); }
  return coding_system;
}


/* Return the number of ASCII characters at the head of the source.
   By side effects, set coding->head_ascii and update
   coding->eol_seen.  The value of coding->eol_seen is "logical or" of
   EOL_SEEN_LF, EOL_SEEN_CR, and EOL_SEEN_CRLF, but the value is
   reliable only when all the source bytes are ASCII.  */

unsafe fn check_ascii (coding: *mut coding_system) -> ptrdiff_t
{
  let mut src: *const c_uchar;
  let mut end: *const c_uchar;
  let eol_type: Lisp_Object = CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t);
  let mut eol_seen: c_int = eol_seen_of (coding);

  coding_set_source (coding);
  src = (*coding).source;
  end = src.offset ((*coding).src_bytes);

  if inhibit_eol_conversion
      || SYMBOLP (eol_type)
    {
      /* We don't have to check EOL format.  */
      while src < end && (*src & 0x80) == 0
	{
	  if *src == '\n' as c_uchar
	    { eol_seen |= EOL_SEEN_LF; }
	  src = src.offset (1);
	}
    }
  else
    {
      end = end.offset (-1);	/* We look ahead one byte for "CR LF".  */
      while src < end
	{
	  let c: c_int = *src as c_int;

	  if c & 0x80 != 0
	    { break; }
	  src = src.offset (1);
	  if c == '\r' as c_int
	    {
	      if *src == '\n' as c_uchar
		{
		  eol_seen |= EOL_SEEN_CRLF;
		  src = src.offset (1);
		}
	      else
		{ eol_seen |= EOL_SEEN_CR; }
	    }
	  else if c == '\n' as c_int
	    { eol_seen |= EOL_SEEN_LF; }
	}
      if src == end
	{
	  let c: c_int = *src as c_int;

	  /* All bytes but the last one C are ASCII.  */
	  if c & 0x80 == 0
	    {
	      if c == '\r' as c_int
		{ eol_seen |= EOL_SEEN_CR; }
	      else if c == '\n' as c_int
		{ eol_seen |= EOL_SEEN_LF; }
	      src = src.offset (1);
	    }
	}
    }
  (*coding).head_ascii = src.offset_from ((*coding).source);
  coding_eol_seen_set (coding, eol_seen);
  return (*coding).head_ascii;
}


/* Return the number of characters at the source if all the bytes are
   valid UTF-8 (of Unicode range).  Otherwise, return -1.  By side
   effects, update coding->eol_seen.  The value of coding->eol_seen is
   "logical or" of EOL_SEEN_LF, EOL_SEEN_CR, and EOL_SEEN_CRLF, but
   the value is reliable only when all the source bytes are valid
   UTF-8.  */

unsafe fn check_utf_8 (coding: *mut coding_system) -> ptrdiff_t
{
  let mut src: *const c_uchar;
  let mut end: *const c_uchar;
  let mut eol_seen: c_int;
  let mut nchars: ptrdiff_t = (*coding).head_ascii;

  if (*coding).head_ascii < 0
    { check_ascii (coding); }
  else
    { coding_set_source (coding); }
  src = (*coding).source.offset ((*coding).head_ascii);
  /* We look ahead one byte for CR LF.  */
  end = (*coding).source.offset ((*coding).src_bytes - 1);
  eol_seen = eol_seen_of (coding);
  while src < end
    {
      let mut c: c_int = *src as c_int;

      if UTF_8_1_OCTET_P! (*src)
	{
	  src = src.offset (1);
	  if c < 0x20
	    {
	      if c == '\r' as c_int
		{
		  if *src == '\n' as c_uchar
		    {
		      eol_seen |= EOL_SEEN_CRLF;
		      src = src.offset (1);
		      nchars += 1;
		    }
		  else
		    { eol_seen |= EOL_SEEN_CR; }
		}
	      else if c == '\n' as c_int
		{ eol_seen |= EOL_SEEN_LF; }
	    }
	}
      else if UTF_8_2_OCTET_LEADING_P! (c)
	{
	  if c < 0xC2		/* overlong sequence */
	      || src.offset (1) >= end
	      || ! UTF_8_EXTRA_OCTET_P! (*src.offset (1))
	    { return -1; }
	  src = src.offset (2);
	}
      else if UTF_8_3_OCTET_LEADING_P! (c)
	{
	  if src.offset (2) >= end
	      || ! (UTF_8_EXTRA_OCTET_P! (*src.offset (1))
		    && UTF_8_EXTRA_OCTET_P! (*src.offset (2)))
	    { return -1; }
	  c = (((c & 0xF) << 12)
	       | ((*src.offset (1) as c_int & 0x3F) << 6) | (*src.offset (2) as c_int & 0x3F));
	  if c < 0x800			      /* overlong sequence */
	      || (c >= 0xd800 && c < 0xe000)  /* surrogates (invalid) */
	    { return -1; }
	  src = src.offset (3);
	}
      else if UTF_8_4_OCTET_LEADING_P! (c)
	{
	  if src.offset (3) >= end
	      || ! (UTF_8_EXTRA_OCTET_P! (*src.offset (1))
		    && UTF_8_EXTRA_OCTET_P! (*src.offset (2))
		    && UTF_8_EXTRA_OCTET_P! (*src.offset (3)))
	    { return -1; }
	  c = (((c & 0x7) << 18) | ((*src.offset (1) as c_int & 0x3F) << 12)
	       | ((*src.offset (2) as c_int & 0x3F) << 6) | (*src.offset (3) as c_int & 0x3F));
	  if c < 0x10000	/* overlong sequence */
	      || c >= 0x110000	/* non-Unicode character  */
	    { return -1; }
	  src = src.offset (4);
	}
      else
	{ return -1; }
      nchars += 1;
    }

  if src == end
    {
      if ! UTF_8_1_OCTET_P! (*src)
	{ return -1; }
      nchars += 1;
      if *src == '\r' as c_uchar
	{ eol_seen |= EOL_SEEN_CR; }
      else if *src == '\n' as c_uchar
	{ eol_seen |= EOL_SEEN_LF; }
    }
  coding_eol_seen_set (coding, eol_seen);
  return nchars;
}

/* Like make_string, but always returns a multibyte Lisp string, and
   avoids decoding if TEXT is encoded in UTF-8.  */

/* `femacs/src/include/coding.rs` declares `make_string_from_utf8` at its crate root
   (coding.h 704), so the definition lives in a glob-imported module: a second crate-root item
   of the same name would be `E0428` (the `space_glyph_def` device, dispnew.rs).  Reported as a
   mirror gap in docs/ports/coding.md.  */
pub mod make_string_from_utf8_def {
  use super::*;

  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn make_string_from_utf8 (text: *const c_char, nbytes: ptrdiff_t) -> Lisp_Object
  {
    /* C 6338-6361 is `#if 0`/`#else`: the slow parse_str_as_multibyte implementation is dead
       in every configuration and the `#else` arm is live (docs/ports/coding.md).  */
    return decode_string_utf_8 (Qnil (), text, nbytes, Qnil (), false, Qt (), Qt ());
  }
}
use make_string_from_utf8_def::*;

/* Detect how end-of-line of a text of length SRC_BYTES pointed by
   SOURCE is encoded.  If CATEGORY is one of
   coding_category_utf_16_XXXX, assume that CR and LF are encoded by
   two-byte, else they are encoded by one-byte.

   Return one of EOL_SEEN_XXX.  */

pub const MAX_EOL_CHECK_COUNT: c_int = 3;

unsafe fn detect_eol (source: *const c_uchar, src_bytes: ptrdiff_t,
		      category: coding_category) -> c_int
{
  let mut src: *const c_uchar = source;
  let src_end: *const c_uchar = src.offset (src_bytes);
  let mut c: c_uchar = 0;
  let mut total: c_int = 0;
  let mut eol_seen: c_int = EOL_SEEN_NONE;

  if ((1 << category) & CATEGORY_MASK_UTF_16) != 0
    {
      let msb: bool = category == (coding_category_utf_16_le
				   | coding_category_utf_16_le_nosig);
      let lsb: bool = ! msb;

      while src.offset (1) < src_end
	{
	  c = *src.offset (lsb as isize);
	  if *src.offset (msb as isize) == 0 && (c == '\n' as c_uchar || c == '\r' as c_uchar)
	    {
	      let mut this_eol: c_int;

	      if c == '\n' as c_uchar
		{ this_eol = EOL_SEEN_LF; }
	      else if src.offset (3) >= src_end
		       || *src.offset (msb as isize + 2) != 0
		       || *src.offset (lsb as isize + 2) != '\n' as c_uchar
		{ this_eol = EOL_SEEN_CR; }
	      else
		{
		  this_eol = EOL_SEEN_CRLF;
		  src = src.offset (2);
		}

	      if eol_seen == EOL_SEEN_NONE
		/* This is the first end-of-line.  */
		{ eol_seen = this_eol; }
	      else if eol_seen != this_eol
		{
		  /* The found type is different from what found before.
		     Allow for stray ^M characters in DOS EOL files.  */
		  if (eol_seen == EOL_SEEN_CR && this_eol == EOL_SEEN_CRLF)
		      || (eol_seen == EOL_SEEN_CRLF
			  && this_eol == EOL_SEEN_CR)
		    { eol_seen = EOL_SEEN_CRLF; }
		  else
		    {
		      eol_seen = EOL_SEEN_LF;
		      break;
		    }
		}
	      total += 1;
	      if total == MAX_EOL_CHECK_COUNT
		{ break; }
	    }
	  src = src.offset (2);
	}
    }
  else
    {
      while src < src_end
	{
	  c = *src as c_uchar; src = src.offset (1);
	  if c == '\n' as c_uchar || c == '\r' as c_uchar
	    {
	      let mut this_eol: c_int;

	      if c == '\n' as c_uchar
		{ this_eol = EOL_SEEN_LF; }
	      else if src >= src_end || *src != '\n' as c_uchar
		{ this_eol = EOL_SEEN_CR; }
	      else
		{
		  this_eol = EOL_SEEN_CRLF; src = src.offset (1);
		}

	      if eol_seen == EOL_SEEN_NONE
		/* This is the first end-of-line.  */
		{ eol_seen = this_eol; }
	      else if eol_seen != this_eol
		{
		  /* The found type is different from what found before.
		     Allow for stray ^M characters in DOS EOL files.  */
		  if (eol_seen == EOL_SEEN_CR && this_eol == EOL_SEEN_CRLF)
		      || (eol_seen == EOL_SEEN_CRLF && this_eol == EOL_SEEN_CR)
		    { eol_seen = EOL_SEEN_CRLF; }
		  else
		    {
		      eol_seen = EOL_SEEN_LF;
		      break;
		    }
		}
	      total += 1;
	      if total == MAX_EOL_CHECK_COUNT
		{ break; }
	    }
	}
    }
  return eol_seen;
}


unsafe fn adjust_coding_eol_type (coding: *mut coding_system, eol_seen: c_int) -> Lisp_Object
{
  let mut eol_type: Lisp_Object;

  eol_type = CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t);
  if ! VECTORP (eol_type)
    /* Already adjusted.  */
    { return eol_type; }
  if (eol_seen & EOL_SEEN_LF) != 0
    {
      (*coding).id = CODING_SYSTEM_ID (AREF (eol_type, 0));
      eol_type = Qunix ();
    }
  else if (eol_seen & EOL_SEEN_CRLF) != 0
    {
      (*coding).id = CODING_SYSTEM_ID (AREF (eol_type, 1));
      eol_type = Qdos ();
    }
  else if (eol_seen & EOL_SEEN_CR) != 0
    {
      (*coding).id = CODING_SYSTEM_ID (AREF (eol_type, 2));
      eol_type = Qmac ();
    }
  return eol_type;
}

/* Detect how a text specified in CODING is encoded.  If a coding
   system is detected, update fields of CODING by the detected coding
   system.  */

unsafe fn detect_coding (coding: *mut coding_system)
{
  let mut src: *const c_uchar;
  let src_end: *const c_uchar;
  let saved_mode: c_uint = mode_of (coding);
  let mut found: Lisp_Object = Qnil ();
  let eol_type: Lisp_Object = CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t);

  (*coding).consumed = 0; (*coding).consumed_char = 0;
  (*coding).produced = 0; (*coding).produced_char = 0;
  coding_set_source (coding);

  src_end = (*coding).source.offset ((*coding).src_bytes);

  coding_eol_seen_set (coding, EOL_SEEN_NONE);
  /* If we have not yet decided the text encoding type, detect it
     now.  */
  if EQ (CODING_ATTR_TYPE (CODING_ID_ATTRS ((*coding).id as hash_idx_t)), Qundecided ())
    {
      let mut c: c_int;
      let mut i: c_int;
      let mut detect_info: coding_detection_info = std::mem::zeroed ();
      let mut null_byte_found: bool = false;
      let mut eight_bit_found: bool = false;
      let inhibit_nbd: bool = inhibit_flag ((*coding).spec.undecided.inhibit_nbd,
					    inhibit_null_byte_detection);
      let inhibit_ied: bool = inhibit_flag ((*coding).spec.undecided.inhibit_ied,
					    inhibit_iso_escape_detection);
      let prefer_utf_8: bool = (*coding).spec.undecided.prefer_utf_8;

      (*coding).head_ascii = 0;
      src = (*coding).source;
      while src < src_end
	{
	  c = *src as c_int;
	  if c & 0x80 != 0
	    {
	      eight_bit_found = true;
	      if null_byte_found
		{ break; }
	    }
	  else if c < 0x20
	    {
	      if (c == ISO_CODE_ESC || c == ISO_CODE_SI || c == ISO_CODE_SO)
		  && ! inhibit_ied
		  && detect_info.checked == 0
		{
		  if detect_coding_iso_2022 (coding, &raw mut detect_info)
		    {
		      /* We have scanned the whole data.  */
		      if (detect_info.rejected & CATEGORY_MASK_ISO_7_ELSE) == 0
			{
			  /* We didn't find an 8-bit code.  We may
			     have found a null-byte, but it's very
			     rare that a binary file conforms to
			     ISO-2022.  */
			  src = src_end;
			  (*coding).head_ascii = src.offset_from ((*coding).source);
			}
		      detect_info.rejected |= ! CATEGORY_MASK_ISO_ESCAPE;
		      break;
		    }
		}
	      else if c == 0 && ! inhibit_nbd
		{
		  null_byte_found = true;
		  if eight_bit_found
		    { break; }
		}
	      else if ! disable_ascii_optimization
		       && ! inhibit_eol_conversion
		{
		  if c == '\r' as c_int
		    {
		      if src < src_end && *src.offset (1) == '\n' as c_uchar
			{
			  coding_eol_seen_set (coding, eol_seen_of (coding) | EOL_SEEN_CRLF);
			  src = src.offset (1);
			  if ! eight_bit_found
			    { (*coding).head_ascii += 1; }
			}
		      else
			{ coding_eol_seen_set (coding, eol_seen_of (coding) | EOL_SEEN_CR); }
		    }
		  else if c == '\n' as c_int
		    {
		      coding_eol_seen_set (coding, eol_seen_of (coding) | EOL_SEEN_LF);
		    }
		}

	      if ! eight_bit_found
		{ (*coding).head_ascii += 1; }
	    }
	  else if ! eight_bit_found
	    { (*coding).head_ascii += 1; }

	  src = src.offset (1);
	}

      if null_byte_found || eight_bit_found
	  || (*coding).head_ascii < (*coding).src_bytes
	  || detect_info.found != 0
	{
	  let mut category: coding_category = 0;
	  let mut this: *mut coding_system = std::ptr::null_mut ();

	  if (*coding).head_ascii == (*coding).src_bytes
	    /* As all bytes are 7-bit, we can ignore non-ISO-2022 codings.  */
	    {
	      i = 0;
	      while i < coding_category_raw_text
		{
		  category = coding_priorities[i as usize];
		  this = &raw mut coding_categories[category as usize];
		  if (detect_info.found & (1 << category)) != 0
		    { break; }
		  i += 1;
		}
	    }
	  else
	    {
	      if null_byte_found
		{
		  detect_info.checked |= ! CATEGORY_MASK_UTF_16;
		  detect_info.rejected |= ! CATEGORY_MASK_UTF_16;
		}
	      else if prefer_utf_8
		       && detect_coding_utf_8 (coding, &raw mut detect_info)
		{
		  detect_info.checked |= ! CATEGORY_MASK_UTF_8;
		  detect_info.rejected |= ! CATEGORY_MASK_UTF_8;
		}
	      i = 0;
	      while i < coding_category_raw_text
		{
		  category = coding_priorities[i as usize];
		  this = &raw mut coding_categories[category as usize];
		  /* Some of this->detector (e.g. detect_coding_sjis)
		     require this information.  */
		  (*coding).id = (*this).id;
		  if (*this).id < 0
		    {
		      /* No coding system of this category is defined.  */
		      detect_info.rejected |= (1 << category);
		    }
		  else if category >= coding_category_raw_text
		    { i += 1; continue; }
		  else if (detect_info.checked & (1 << category)) != 0
		    {
		      if (detect_info.found & (1 << category)) != 0
			{ break; }
		    }
		  else if ((*this).detector.unwrap ()) (coding, &raw mut detect_info)
		      && (detect_info.found & (1 << category)) != 0
		    { break; }
		  i += 1;
		}
	    }

	  if i < coding_category_raw_text
	    {
	      if category == coding_category_utf_8_auto
		{
		  let mut coding_systems: Lisp_Object;

		  coding_systems = AREF (CODING_ID_ATTRS ((*this).id as hash_idx_t),
					 coding_attr_utf_bom as isize);
		  if CONSP (coding_systems)
		    {
		      if (detect_info.found & CATEGORY_MASK_UTF_8_SIG) != 0
			{ found = XCAR (coding_systems); }
		      else
			{ found = XCDR (coding_systems); }
		    }
		  else
		    { found = CODING_ID_NAME ((*this).id as hash_idx_t); }
		}
	      else if category == coding_category_utf_16_auto
		{
		  let mut coding_systems: Lisp_Object;

		  coding_systems = AREF (CODING_ID_ATTRS ((*this).id as hash_idx_t),
					 coding_attr_utf_bom as isize);
		  if CONSP (coding_systems)
		    {
		      if (detect_info.found & CATEGORY_MASK_UTF_16_LE) != 0
			{ found = XCAR (coding_systems); }
		      else if (detect_info.found & CATEGORY_MASK_UTF_16_BE) != 0
			{ found = XCDR (coding_systems); }
		    }
		  else
		    { found = CODING_ID_NAME ((*this).id as hash_idx_t); }
		}
	      else
		{ found = CODING_ID_NAME ((*this).id as hash_idx_t); }
	    }
	  else if null_byte_found
	    { found = Qno_conversion (); }
	  else if (detect_info.rejected & CATEGORY_MASK_ANY) == CATEGORY_MASK_ANY
	    { found = Qraw_text (); }
	  else if detect_info.rejected != 0
	    {
	      i = 0;
	      while i < coding_category_raw_text
		{
		  if (detect_info.rejected & (1 << coding_priorities[i as usize])) == 0
		    {
		      this = &raw mut coding_categories[coding_priorities[i as usize] as usize];
		      found = CODING_ID_NAME ((*this).id as hash_idx_t);
		      break;
		    }
		  i += 1;
		}
	    }
	}
    }
  else if XFIXNUM (CODING_ATTR_CATEGORY (CODING_ID_ATTRS ((*coding).id as hash_idx_t))) as c_int
	   == coding_category_utf_8_auto
    {
      let coding_systems: Lisp_Object
	= AREF (CODING_ID_ATTRS ((*coding).id as hash_idx_t), coding_attr_utf_bom as isize);
      if check_ascii (coding) == (*coding).src_bytes
	{
	  if CONSP (coding_systems)
	    { found = XCDR (coding_systems); }
	}
      else
	{
	  let mut detect_info: coding_detection_info = std::mem::zeroed ();
	  if CONSP (coding_systems)
	      && detect_coding_utf_8 (coding, &raw mut detect_info)
	    {
	      if (detect_info.found & CATEGORY_MASK_UTF_8_SIG) != 0
		{ found = XCAR (coding_systems); }
	      else
		{ found = XCDR (coding_systems); }
	    }
	}
    }
  else if XFIXNUM (CODING_ATTR_CATEGORY (CODING_ID_ATTRS ((*coding).id as hash_idx_t))) as c_int
	   == coding_category_utf_16_auto
    {
      let coding_systems: Lisp_Object
	= AREF (CODING_ID_ATTRS ((*coding).id as hash_idx_t), coding_attr_utf_bom as isize);
      (*coding).head_ascii = 0;
      if CONSP (coding_systems)
	{
	  let mut detect_info: coding_detection_info = std::mem::zeroed ();
	  if detect_coding_utf_16 (coding, &raw mut detect_info)
	    {
	      if (detect_info.found & CATEGORY_MASK_UTF_16_LE) != 0
		{ found = XCAR (coding_systems); }
	      else if (detect_info.found & CATEGORY_MASK_UTF_16_BE) != 0
		{ found = XCDR (coding_systems); }
	    }
	}
    }

  if ! NILP (found)
    {
      let specified_eol: c_int = if VECTORP (eol_type) { EOL_SEEN_NONE }
			   else if EQ (eol_type, Qdos ()) { EOL_SEEN_CRLF }
			   else if EQ (eol_type, Qmac ()) { EOL_SEEN_CR }
			   else { EOL_SEEN_LF };

      setup_coding_system (found, coding);
      if specified_eol != EOL_SEEN_NONE
	{ adjust_coding_eol_type (coding, specified_eol); }
    }

  coding_bits_set (coding, CODING_SYSTEM_MODE_SHIFT, CODING_SYSTEM_MODE_MASK, saved_mode);
}


unsafe fn decode_eol (coding: *mut coding_system)
{
  let mut eol_type: Lisp_Object;
  let mut p: *mut c_uchar;
  let mut pbeg: *mut c_uchar;
  let mut pend: *mut c_uchar;

  eol_type = CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t);
  if EQ (eol_type, Qunix ()) || inhibit_eol_conversion
    { return; }

  if NILP ((*coding).dst_object)
    { pbeg = (*coding).destination; }
  else
    { pbeg = BYTE_POS_ADDR ((*coding).dst_pos_byte); }
  pend = pbeg.offset ((*coding).produced);

  if VECTORP (eol_type)
    {
      let mut eol_seen: c_int = EOL_SEEN_NONE;

      p = pbeg;
      while p < pend
	{
	  if *p == '\n' as c_uchar
	    { eol_seen |= EOL_SEEN_LF; }
	  else if *p == '\r' as c_uchar
	    {
	      if p.offset (1) < pend && *p.offset (1) == '\n' as c_uchar
		{
		  eol_seen |= EOL_SEEN_CRLF;
		  p = p.offset (1);
		}
	      else
		{ eol_seen |= EOL_SEEN_CR; }
	    }
	  p = p.offset (1);
	}
      /* Handle DOS-style EOLs in a file with stray ^M characters.  */
      if (eol_seen & EOL_SEEN_CRLF) != 0
	  && (eol_seen & EOL_SEEN_CR) != 0
	  && (eol_seen & EOL_SEEN_LF) == 0
	{ eol_seen = EOL_SEEN_CRLF; }
      else if eol_seen != EOL_SEEN_NONE
	  && eol_seen != EOL_SEEN_LF
	  && eol_seen != EOL_SEEN_CRLF
	  && eol_seen != EOL_SEEN_CR
	{ eol_seen = EOL_SEEN_LF; }
      if eol_seen != EOL_SEEN_NONE
	{ eol_type = adjust_coding_eol_type (coding, eol_seen); }
    }

  if EQ (eol_type, Qmac ())
    {
      p = pbeg;
      while p < pend
	{
	  if *p == '\r' as c_uchar
	    { *p = '\n' as c_uchar; }
	  p = p.offset (1);
	}
    }
  else if EQ (eol_type, Qdos ())
    {
      let mut n: ptrdiff_t = 0;
      let pos: ptrdiff_t = (*coding).dst_pos;
      let mut pos_byte: ptrdiff_t = (*coding).dst_pos_byte;
      let mut pos_end: ptrdiff_t = pos_byte + (*coding).produced - 1;

      /* This assertion is here instead of code, now deleted, that
	 handled the NILP case, which no longer happens with the
	 current codebase.  */
      eassert! (! NILP ((*coding).dst_object));

      while pos_byte < pos_end
	{
	  let mut incr: c_int;

	  p = BYTE_POS_ADDR (pos_byte);
	  if dst_multibyte_p (coding)
	    { incr = BYTES_BY_CHAR_HEAD (*p as c_int); }
	  else
	    { incr = 1; }

	  if *p == '\r' as c_uchar && *p.offset (1) == '\n' as c_uchar
	    {
	      del_range_2 (pos, pos_byte, pos + 1, pos_byte + 1, false);
	      n += 1;
	      pos_end -= 1;
	    }
	  pos_byte += 1;
	  pos_byte += incr as isize;
	}
      (*coding).produced -= n;
      (*coding).produced_char -= n;
    }
}


/* MAX_LOOKUP's maximum value.  MAX_LOOKUP is an int and so cannot
   exceed INT_MAX.  Also, MAX_LOOKUP is multiplied by sizeof (int) for
   alloca, so it cannot exceed MAX_ALLOCA / sizeof (int).  */
/* C's `min (INT_MAX, MAX_ALLOCA / sizeof (int))`; `min` is not a `const fn` and
   `sizeof (int)` is 4 on the frozen target, so the fold is written out in the same operand
   order (docs/ports/macro-body-notes.tsv records the three tokens it stands for).  */
pub const MAX_LOOKUP_MAX: c_int
  = if (INT_MAX as isize) < MAX_ALLOCA / 4 { INT_MAX } else { (MAX_ALLOCA / 4) as c_int };

/* Return a translation table (or list of them) from coding system
   attribute vector ATTRS for encoding (if ENCODEP) or decoding (if
   not ENCODEP). */

unsafe fn get_translation_table (attrs: Lisp_Object, encodep: bool,
				 max_lookup: *mut c_int) -> Lisp_Object
{
  let mut standard: Lisp_Object;
  let mut translation_table: Lisp_Object;
  let mut val: Lisp_Object;

  if NILP (Venable_character_translation)
    {
      if ! max_lookup.is_null ()
	{ *max_lookup = 0; }
      return Qnil ();
    }
  if encodep
    {
      translation_table = CODING_ATTR_ENCODE_TBL (attrs);
      standard = Vstandard_translation_table_for_encode;
    }
  else
    {
      translation_table = CODING_ATTR_DECODE_TBL (attrs);
      standard = Vstandard_translation_table_for_decode;
    }
  if NILP (translation_table)
    { translation_table = standard; }
  else
    {
      if SYMBOLP (translation_table)
	{ translation_table = Fget (translation_table, Qtranslation_table ()); }
      else if CONSP (translation_table)
	{
	  translation_table = Fcopy_sequence (translation_table);
	  val = translation_table;
	  while CONSP (val)
	    {
	      if SYMBOLP (XCAR (val))
		{ XSETCAR (val, Fget (XCAR (val), Qtranslation_table ())); }
	      val = XCDR (val);
	    }
	}
      if CHAR_TABLE_P (standard)
	{
	  if CONSP (translation_table)
	    { translation_table = nconc2 (translation_table, list1 (standard)); }
	  else
	    { translation_table = list2 (translation_table, standard); }
	}
    }

  if ! max_lookup.is_null ()
    {
      *max_lookup = 1;
      if CHAR_TABLE_P (translation_table)
	  && CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (translation_table)) > 1
	{
	  val = FAM_ELEM! ((*XCHAR_TABLE (translation_table)).extras, 1);
	  if FIXNATP (val) && (*max_lookup as isize) < XFIXNAT (val)
	    { *max_lookup = min (XFIXNAT (val), MAX_LOOKUP_MAX as isize) as c_int; }
	}
      else if CONSP (translation_table)
	{
	  let mut tail: Lisp_Object;

	  tail = translation_table;
	  while CONSP (tail)
	    {
	      if CHAR_TABLE_P (XCAR (tail))
		  && CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (XCAR (tail))) > 1
		{
		  let tailval: Lisp_Object = FAM_ELEM! ((*XCHAR_TABLE (XCAR (tail))).extras, 1);
		  if FIXNATP (tailval) && (*max_lookup as isize) < XFIXNAT (tailval)
		    { *max_lookup = min (XFIXNAT (tailval), MAX_LOOKUP_MAX as isize) as c_int; }
		}
	      tail = XCDR (tail);
	    }
	}
    }
  return translation_table;
}

/* Return a translation of character(s) at BUF according to TRANS.
   TRANS is TO-CHAR, [TO-CHAR ...], or ((FROM .  TO) ...) where FROM =
   [FROM-CHAR ...], TO is TO-CHAR or [TO-CHAR ...].  The return value
   is TO-CHAR or [TO-CHAR ...] if a translation is found, Qnil if not
   found, or Qt if BUF is too short to lookup characters in FROM.  As
   a side effect, if a translation is found, *NCHARS is set to the
   number of characters being translated.  */

unsafe fn get_translation (mut trans: Lisp_Object, buf: *mut c_int, buf_end: *mut c_int,
			   nchars: *mut ptrdiff_t) -> Lisp_Object
{
  if FIXNUMP (trans) || VECTORP (trans)
    {
      *nchars = 1;
      return trans;
    }
  while CONSP (trans)
    {
      let val: Lisp_Object = XCAR (trans);
      let from: Lisp_Object = XCAR (val);
      let len: ptrdiff_t = ASIZE (from);
      let mut i: ptrdiff_t;

      i = 0;
      while i < len
	{
	  if buf.offset (i) == buf_end
	    { return Qt (); }
	  if XFIXNUM (AREF (from, i)) != *buf.offset (i) as isize
	    { break; }
	  i += 1;
	}
      if i == len
	{
	  *nchars = len;
	  return XCDR (val);
	}
      trans = XCDR (trans);
    }
  return Qnil ();
}


unsafe fn produce_chars (coding: *mut coding_system, translation_table: Lisp_Object,
			 last_block: bool) -> c_int
{
  let mut dst: *mut c_uchar = (*coding).destination.offset ((*coding).produced);
  let mut dst_end: *mut c_uchar = (*coding).destination.offset ((*coding).dst_bytes);
  let produced: ptrdiff_t;
  let mut produced_chars: ptrdiff_t = 0;
  let mut carryover: c_int = 0;

  if coding_bits_get (coding, CODING_SYSTEM_CHARS_AT_SOURCE_SHIFT, 1) == 0
    {
      /* Source characters are in coding->charbuf.  */
      let mut buf: *mut c_int = (*coding).charbuf;
      let buf_end: *mut c_int = buf.offset ((*coding).charbuf_used as isize);

      if EQ ((*coding).src_object, (*coding).dst_object)
	  && ! NILP ((*coding).dst_object)
	{
	  eassert! (growable_destination (coding));
	  coding_set_source (coding);
	  dst_end = ((*coding).source as *mut c_uchar).offset ((*coding).consumed);
	}

      while buf < buf_end
	{
	  let mut c: c_int = *buf;
	  let mut i: ptrdiff_t;

	  if c >= 0
	    {
	      let mut from_nchars: ptrdiff_t = 1;
	      let mut to_nchars: ptrdiff_t = 1;
	      let mut trans: Lisp_Object = Qnil ();

	      macro_rules! LOOKUP_TRANSLATION_TABLE {
		($table:expr, $c:ident, $trans:ident) => {
		  {
		    $trans = Qnil ();
		    if CHAR_TABLE_P ($table)
		      {
			$trans = CHAR_TABLE_REF ($table, $c as isize);
			if CHARACTERP ($trans)
			  { $c = XFIXNAT ($trans) as c_int; $trans = Qnil (); }
		      }
		    else if CONSP ($table)
		      {
			let mut tail: Lisp_Object;

			tail = $table;
			while CONSP (tail)
			  {
			    if CHAR_TABLE_P (XCAR (tail))
			      {
				$trans = CHAR_TABLE_REF (XCAR (tail), $c as isize);
				if CHARACTERP ($trans)
				  { $c = XFIXNAT ($trans) as c_int; $trans = Qnil (); }
				else if ! NILP ($trans)
				  { break; }
			      }
			    tail = XCDR (tail);
			  }
		      }
		  }
		};
	      }

	      LOOKUP_TRANSLATION_TABLE! (translation_table, c, trans);
	      if ! NILP (trans)
		{
		  trans = get_translation (trans, buf, buf_end, &raw mut from_nchars);
		  if FIXNUMP (trans)
		    { c = XFIXNUM (trans) as c_int; }
		  else if VECTORP (trans)
		    {
		      to_nchars = ASIZE (trans);
		      c = XFIXNUM (AREF (trans, 0)) as c_int;
		    }
		  else if EQ (trans, Qt ()) && ! last_block
		    { break; }
		}

	      if dst_end.offset_from (dst) / (MAX_MULTIBYTE_LENGTH as isize) < to_nchars
		{
		  let mut dst_size: ptrdiff_t;

		  eassert! (growable_destination (coding));
		  /* C's `ckd_mul (&dst_size, to_nchars, MAX_MULTIBYTE_LENGTH) || ckd_add
		     (&dst_size, dst_size, buf_end - buf)`: the two stores happen only when
		     their operation did not overflow (P-15, the ckd_mul/ckd_add family of
		     docs/ports/token-aliases.tsv).  */
		  let (dst_size_product, dst_size_overflowed)
		    = to_nchars.overflowing_mul (MAX_MULTIBYTE_LENGTH as isize);
		  dst_size = dst_size_product;
		  if dst_size_overflowed
		      || ckd_add! (dst_size, dst_size, buf_end.offset_from (buf))
		    { memory_full_up (); }
		  dst = alloc_destination (coding, dst_size, dst);
		  if EQ ((*coding).src_object, (*coding).dst_object)
		      /* Input and output are not C buffers, which are safe to
			 assume to be different.  */
		      && ! NILP ((*coding).src_object)
		    {
		      coding_set_source (coding);
		      dst_end = ((*coding).source as *mut c_uchar)
			  .offset ((*coding).consumed);
		    }
		  else
		    { dst_end = (*coding).destination.offset ((*coding).dst_bytes); }
		}

	      i = 0;
	      while i < to_nchars
		{
		  if i > 0
		    { c = XFIXNUM (AREF (trans, i)) as c_int; }
		  if dst_multibyte_p (coding)
		      || ! CHAR_BYTE8_P (c)
		    { CHAR_STRING_ADVANCE_NO_UNIFY! (c, dst); }
		  else
		    { *dst = CHAR_TO_BYTE8 (c) as c_uchar; dst = dst.offset (1); }
		  i += 1;
		}
	      produced_chars += to_nchars;
	      buf = buf.offset (from_nchars);
	    }
	  else
	    /* This is an annotation datum.  (-C) is the length.  */
	    { buf = buf.offset (-c as isize); }
	}
      carryover = buf_end.offset_from (buf) as c_int;
    }
  else
    {
      /* Source characters are at coding->source.  */
      let mut src: *const c_uchar = (*coding).source;
      let mut src_end: *const c_uchar = src.offset ((*coding).consumed);

      if EQ ((*coding).dst_object, (*coding).src_object)
	  /* Input and output are not C buffers, which are safe to
	     assume to be different.  */
	  && ! NILP ((*coding).src_object)
	{
	  eassert! (growable_destination (coding));
	  dst_end = src as *mut c_uchar;
	}
      if src_multibyte_p (coding) != dst_multibyte_p (coding)
	{
	  if src_multibyte_p (coding)
	    {
	      let multibytep: bool = true;
	      let mut consumed_chars: ptrdiff_t = 0;

	      'no_more_source: {
		loop
		  {
		    let mut src_base: *const c_uchar = src;
		    let mut c: c_int;

		    /* C declares `src_base` inside this loop; the macro that reads it is
		       therefore defined here too (a `macro_rules!` resolves locals at its
		       definition site, PORTING.md P-57).  */
		    macro_rules! ONE_MORE_BYTE {
		      ($c:ident) => {
			{
			  if src == src_end
			    {
			      if src_base < src
				{ record_conversion_result (coding, CODING_RESULT_INSUFFICIENT_SRC); }
			      break 'no_more_source;
			    }
			  $c = *src as c_int;
			  src = src.offset (1);
			  if multibytep && ($c & 0x80) != 0
			    {
			      if ($c & 0xFE) == 0xC0
				{ $c = (($c & 1) << 6) | *src as c_int; src = src.offset (1); }
			      else
				{
				  src = src.offset (-1);
				  $c = - string_char_advance (&raw mut src);
				  record_conversion_result (coding, CODING_RESULT_INVALID_SRC);
				}
			    }
			  consumed_chars += 1;
			}
		      };
		    }

		    ONE_MORE_BYTE! (c);
		    if dst == dst_end
		      {
			eassert! (growable_destination (coding));
			if EQ ((*coding).src_object, (*coding).dst_object)
			    && ! NILP ((*coding).src_object)
			  { dst_end = src as *mut c_uchar; }
			if dst == dst_end
			  {
			    let offset: ptrdiff_t = src.offset_from ((*coding).source);

			    dst = alloc_destination (coding,
						     src_end.offset_from (src) + 1, dst);
			    dst_end = (*coding).destination.offset ((*coding).dst_bytes);
			    coding_set_source (coding);
			    src = (*coding).source.offset (offset);
			    src_end = (*coding).source.offset ((*coding).consumed);
			    if EQ ((*coding).src_object, (*coding).dst_object)
				&& ! NILP ((*coding).src_object)
			      { dst_end = src as *mut c_uchar; }
			  }
		      }
		    *dst = c as c_uchar; dst = dst.offset (1);
		    produced_chars += 1;
		  }
	      }
	      /* no_more_source: */
	    }
	  else
	    {
	      let multibytep: bool = true;

	      macro_rules! EMIT_ONE_BYTE {
		($c:expr) => {
		  {
		    produced_chars += 1;
		    if multibytep
		      {
			let mut ch: c_uint = ($c) as c_uint;
			if ch >= 0x80
			  { ch = BYTE8_TO_CHAR (ch as c_int) as c_uint; }
			dst = dst.offset (CHAR_STRING (ch as c_int, dst) as isize);
		      }
		    else
		      { *dst = ($c) as c_uchar; dst = dst.offset (1); }
		  }
		};
	      }

	      while src < src_end
		{
		  let c: c_int = *src as c_int; src = src.offset (1);

		  if dst >= dst_end.offset (-1)
		    {
		      eassert! (growable_destination (coding));
		      if EQ ((*coding).src_object, (*coding).dst_object)
			  && ! NILP ((*coding).src_object)
			{ dst_end = src as *mut c_uchar; }
		      if dst >= dst_end.offset (-1)
			{
			  let offset: ptrdiff_t = src.offset_from ((*coding).source);
			  let more_bytes: ptrdiff_t;

			  if EQ ((*coding).src_object, (*coding).dst_object)
			      && ! NILP ((*coding).src_object)
			    { more_bytes = src_end.offset_from (src) / 2 + 2; }
			  else
			    { more_bytes = src_end.offset_from (src) + 2; }
			  dst = alloc_destination (coding, more_bytes, dst);
			  dst_end = (*coding).destination.offset ((*coding).dst_bytes);
			  coding_set_source (coding);
			  src = (*coding).source.offset (offset);
			  src_end = (*coding).source.offset ((*coding).consumed);
			  if EQ ((*coding).src_object, (*coding).dst_object)
			      && ! NILP ((*coding).src_object)
			    { dst_end = src as *mut c_uchar; }
			}
		    }
		  EMIT_ONE_BYTE! (c);
		}
	    }
	}
      else
	{
	  if ! (EQ ((*coding).src_object, (*coding).dst_object)
		&& ! NILP ((*coding).src_object))
	    {
	      let require: ptrdiff_t = (*coding).src_bytes - (*coding).dst_bytes;

	      if require > 0
		{
		  let offset: ptrdiff_t = src.offset_from ((*coding).source);

		  dst = alloc_destination (coding, require, dst);
		  coding_set_source (coding);
		  src = (*coding).source.offset (offset);
		  src_end = (*coding).source.offset ((*coding).consumed);
		}
	    }
	  produced_chars = (*coding).consumed_char;
	  while src < src_end
	    {
	      *dst = *src; dst = dst.offset (1); src = src.offset (1);
	    }
	}
    }

  produced = dst.offset_from ((*coding).destination.offset ((*coding).produced));
  if BUFFERP ((*coding).dst_object) && produced_chars > 0
    {
      insert_from_gap (produced_chars, produced, false,
		       coding_bits_get (coding, CODING_SYSTEM_INSERT_BEFORE_MARKERS_SHIFT, 1) != 0);
    }
  (*coding).produced += produced;
  (*coding).produced_char += produced_chars;
  return carryover;
}

/* Compose text in CODING->object according to the annotation data at
   CHARBUF.  CHARBUF is an array:
     [ -LENGTH ANNOTATION_MASK NCHARS NBYTES METHOD [ COMPONENTS... ] ]
 */

unsafe fn produce_composition (coding: *mut coding_system, mut charbuf: *mut c_int,
			       pos: ptrdiff_t)
{
  let mut len: c_int;
  let to: ptrdiff_t;
  let method: composition_method;
  let mut components: Lisp_Object;

  len = -*charbuf - MAX_ANNOTATION_LENGTH;
  to = pos + *charbuf.offset (2) as isize;
  method = *charbuf.offset (4) as composition_method;

  if method == COMPOSITION_RELATIVE
    { components = Qnil (); }
  else
    {
      let mut args: [Lisp_Object; MAX_COMPOSITION_COMPONENTS as usize * 2 - 1]
	= [Qnil (); MAX_COMPOSITION_COMPONENTS as usize * 2 - 1];
      let mut i: c_int;
      let mut j: c_int;

      if method == COMPOSITION_WITH_RULE
	{ len = *charbuf.offset (2) * 3 - 2; }
      charbuf = charbuf.offset (MAX_ANNOTATION_LENGTH as isize);
      /* charbuf = [ CHRA ... CHAR] or [ CHAR -2 RULE ... CHAR ] */
      i = 0; j = 0;
      while i < len && *charbuf.offset (i as isize) != -1
	{
	  if *charbuf.offset (i as isize) >= 0
	    { args[j as usize] = make_fixnum (*charbuf.offset (i as isize) as EMACS_INT); }
	  else
	    {
	      i += 1;
	      args[j as usize] = make_fixnum ((*charbuf.offset (i as isize) as EMACS_INT) % 0x100);
	    }
	  i += 1; j += 1;
	}
      components = if i == j { Fstring (j as ptrdiff_t, args.as_mut_ptr ()) }
		   else { Fvector (j as ptrdiff_t, args.as_mut_ptr ()) };
    }
  compose_text (pos, to, components, Qnil (), (*coding).dst_object);
}


/* Put `charset' property on text in CODING->object according to
   the annotation data at CHARBUF.  CHARBUF is an array:
     [ -LENGTH ANNOTATION_MASK NCHARS CHARSET-ID ]
 */

unsafe fn produce_charset (coding: *mut coding_system, charbuf: *mut c_int, pos: ptrdiff_t)
{
  let from: ptrdiff_t = pos - *charbuf.offset (2) as isize;
  let charset: *mut charset = CHARSET_FROM_ID! (*charbuf.offset (3));

  Fput_text_property (make_fixnum (from as EMACS_INT), make_fixnum (pos as EMACS_INT),
		      Qcharset (), CHARSET_NAME! (charset),
		      (*coding).dst_object);
}

pub const MAX_CHARBUF_SIZE: c_int = 0x4000;
/* How many units decoding functions expect in coding->charbuf at
   most.  Currently, decode_coding_emacs_mule expects the following
   size, and that is the largest value.  */
pub const MAX_CHARBUF_EXTRA_SIZE: c_int = ((MAX_ANNOTATION_LENGTH * 3) + 1);

unsafe fn produce_annotation (coding: *mut coding_system, pos: ptrdiff_t)
{
  let mut charbuf: *mut c_int = (*coding).charbuf;
  let charbuf_end: *mut c_int = charbuf.offset ((*coding).charbuf_used as isize);
  let mut pos: ptrdiff_t = pos;

  if NILP ((*coding).dst_object)
    { return; }

  while charbuf < charbuf_end
    {
      if *charbuf >= 0
	{ pos += 1; charbuf = charbuf.offset (1); }
      else
	{
	  let len: c_int = -*charbuf;

	  if len > 2
	    {
	      match *charbuf.offset (1) as c_uint
		{
		  CODING_ANNOTATE_COMPOSITION_MASK =>
		    { produce_composition (coding, charbuf, pos); }
		  CODING_ANNOTATE_CHARSET_MASK =>
		    { produce_charset (coding, charbuf, pos); }
		  _ =>
		    { }
		}
	    }
	  charbuf = charbuf.offset (len as isize);
	}
    }
}


/* Decode the data at CODING->src_object into CODING->dst_object.
   CODING->src_object is a buffer, a string, or nil.
   CODING->dst_object is a buffer.

   If CODING->src_object is a buffer, it must be the current buffer.
   In this case, if CODING->src_pos is positive, it is a position of
   the source text in the buffer, otherwise, the source text is in the
   gap area of the buffer, and CODING->src_pos specifies the offset of
   the text from the end of the gap (and GPT must be equal to PT).

   When the text is taken from the gap, it can't be at the beginning
   of the gap because the new decoded text is progressively accumulated
   at the beginning of the gap before it gets inserted at PT (this way,
   as the output grows, the input shrinks, so we only need to allocate
   enough space for `max(IN, OUT)` instead of `IN + OUT`).

   If CODING->src_object is a string, CODING->src_pos is an index to
   that string.

   If CODING->src_object is nil, CODING->source must already point to
   the non-relocatable memory area.  In this case, CODING->src_pos is
   an offset from CODING->source.

   The decoded data is inserted at the current point of the buffer
   CODING->dst_object.
*/

unsafe fn decode_coding (coding: *mut coding_system)
{
  let mut attrs: Lisp_Object;
  let mut undo_list: Lisp_Object;
  let translation_table: Lisp_Object;
  let mut cclspec: ccl_spec = std::mem::zeroed ();
  let mut carryover: c_int;
  let mut i: c_int;
  #[allow(unused_mut)]
  let mut count: specpdl_ref = SPECPDL_INDEX ();

  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  if BUFFERP ((*coding).src_object)
      && (*coding).src_pos > 0
      && (*coding).src_pos < GPT! ()
      && (*coding).src_pos + (*coding).src_chars > GPT! ()
    { move_gap_both ((*coding).src_pos, (*coding).src_pos_byte); }

  undo_list = Qt ();
  if BUFFERP ((*coding).dst_object)
    {
      set_buffer_internal (XBUFFER ((*coding).dst_object));
      if GPT! () != PT! ()
	{ move_gap_both (PT! (), PT_BYTE! ()); }

      /* We must disable undo_list in order to record the whole insert
	 transaction via record_insert at the end.  But doing so also
	 disables the recording of the first change to the undo_list.
	 Therefore we check for first change here and record it via
	 record_first_change if needed.  */
      if MODIFF! () <= SAVE_MODIFF! ()
	{ record_first_change (); }

      undo_list = BVAR! ((current_buffer! ()), undo_list_);
      bset_undo_list ((current_buffer! ()), Qt ());
      /* Avoid running nested *-change-functions via 'produce_annotation'.
	 Our callers run *-change-functions over the whole region anyway.  */
      specbind (Qinhibit_modification_hooks (), Qt ());
    }

  (*coding).consumed = 0; (*coding).consumed_char = 0;
  (*coding).produced = 0; (*coding).produced_char = 0;
  coding_bits_set (coding, CODING_SYSTEM_CHARS_AT_SOURCE_SHIFT, 1, 0);
  record_conversion_result (coding, CODING_RESULT_SUCCESS);

  /* C's `ALLOC_CONVERSION_WORK_AREA (coding, size)`: `SAFE_ALLOCA` reads the caller's
     `sa_avail`, so the macro is defined in each using function (PORTING.md P-57).  */
  macro_rules! ALLOC_CONVERSION_WORK_AREA {
    ($coding:expr, $size:expr) => {
      {
	let units: ptrdiff_t
	  = min (($size) as isize + MAX_CHARBUF_EXTRA_SIZE as isize, MAX_CHARBUF_SIZE as isize);
	(*$coding).charbuf = SAFE_ALLOCA! (sa_avail, units * 4) as *mut c_int;
	(*$coding).charbuf_size = units as c_int;
      }
    };
  }

  ALLOC_CONVERSION_WORK_AREA! (coding, (*coding).src_bytes);

  attrs = CODING_ID_ATTRS ((*coding).id as hash_idx_t);
  translation_table = get_translation_table (attrs, false, std::ptr::null_mut ());

  carryover = 0;
  if (*coding).decoder == Some (decode_coding_ccl)
    {
      (*coding).spec.ccl = &raw mut cclspec as *mut c_void;
      setup_ccl_program (&raw mut cclspec.ccl, CODING_CCL_DECODER! (coding));
    }
  loop
    {
      let pos: ptrdiff_t = (*coding).dst_pos + (*coding).produced_char;

      coding_set_source (coding);
      coding_bits_set (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1, 0);
      (*coding).charbuf_used = carryover;
      ((*coding).decoder.unwrap ()) (coding);
      coding_set_destination (coding);
      carryover = produce_chars (coding, translation_table, false);
      if coding_bits_get (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1) != 0
	{ produce_annotation (coding, pos); }
      i = 0;
      while i < carryover
	{
	  *(*coding).charbuf.offset (i as isize)
	    = *(*coding).charbuf.offset (((*coding).charbuf_used - carryover + i) as isize);
	  i += 1;
	}
      if ! (coding_bits_get (coding, CODING_SYSTEM_RESULT_SHIFT, CODING_SYSTEM_RESULT_MASK)
	      == CODING_RESULT_INSUFFICIENT_DST as u32
	    || ((*coding).consumed < (*coding).src_bytes
		&& (coding_bits_get (coding, CODING_SYSTEM_RESULT_SHIFT,
				     CODING_SYSTEM_RESULT_MASK)
		      == CODING_RESULT_SUCCESS as u32
		    || coding_bits_get (coding, CODING_SYSTEM_RESULT_SHIFT,
					CODING_SYSTEM_RESULT_MASK)
		      == CODING_RESULT_INVALID_SRC as u32)))
	{ break; }
    }

  if carryover > 0
    {
      coding_set_destination (coding);
      (*coding).charbuf_used = carryover;
      produce_chars (coding, translation_table, true);
    }

  (*coding).carryover_bytes = 0;
  if (*coding).consumed < (*coding).src_bytes
    {
      let mut nbytes: ptrdiff_t = (*coding).src_bytes - (*coding).consumed;
      let mut src: *const c_uchar;

      coding_set_source (coding);
      coding_set_destination (coding);
      src = (*coding).source.offset ((*coding).consumed);

      if mode_of (coding) as c_int & CODING_MODE_LAST_BLOCK != 0
	{
	  /* Flush out unprocessed data as binary chars.  We are sure
	     that the number of data is less than the size of
	     coding->charbuf.  */
	  (*coding).charbuf_used = 0;
	  coding_bits_set (coding, CODING_SYSTEM_CHARS_AT_SOURCE_SHIFT, 1, 0);

	  while nbytes > 0
	    {
	      let mut c: c_int;

	      nbytes -= 1;
	      /* Copy raw bytes in their 2-byte forms from multibyte
		 text as single characters.  */
	      if src_multibyte_p (coding)
		  && CHAR_BYTE8_HEAD_P (*src as c_int) && nbytes > 0
		{
		  c = string_char_advance (&raw mut src);
		  nbytes -= 1;
		}
	      else
		{
		  c = *src as c_int; src = src.offset (1);

		  if c & 0x80 != 0
		    { c = BYTE8_TO_CHAR (c); }
		}
	      *(*coding).charbuf.offset ((*coding).charbuf_used as isize) = c;
	      (*coding).charbuf_used += 1;
	    }
	  produce_chars (coding, Qnil (), true);
	}
      else
	{
	  /* Record unprocessed bytes in coding->carryover.  We are
	     sure that the number of data is less than the size of
	     coding->carryover.  */
	  let mut p: *mut c_uchar = (*coding).carryover.as_mut_ptr ();

	  if nbytes > std::mem::size_of_val (&(*coding).carryover) as ptrdiff_t
	    { nbytes = std::mem::size_of_val (&(*coding).carryover) as ptrdiff_t; }
	  (*coding).carryover_bytes = nbytes as c_int;
	  while nbytes > 0
	    {
	      nbytes -= 1;
	      *p = *src; p = p.offset (1); src = src.offset (1);
	    }
	}
      (*coding).consumed = (*coding).src_bytes;
    }

  if ! EQ (CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t), Qunix ())
      && !inhibit_eol_conversion
    { decode_eol (coding); }
  if BUFFERP ((*coding).dst_object)
    {
      bset_undo_list ((current_buffer! ()), undo_list);
      record_insert ((*coding).dst_pos, (*coding).produced_char);
    }

  SAFE_FREE_UNBIND_TO! (sa_count, count, Qnil ());
}


/* Extract an annotation datum from a composition starting at POS and
   ending before LIMIT of CODING->src_object (buffer or string), store
   the data in BUF, set *STOP to a starting position of the next
   composition (if any) or to LIMIT, and return the address of the
   next element of BUF.

   If such an annotation is not found, set *STOP to a starting
   position of a composition after POS (if any) or to LIMIT, and
   return BUF.  */

unsafe fn handle_composition_annotation (pos: ptrdiff_t, limit: ptrdiff_t,
					 coding: *mut coding_system, mut buf: *mut c_int,
					 stop: *mut ptrdiff_t) -> *mut c_int
{
  let mut start: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;
  let mut prop: Lisp_Object = Qnil ();

  if ! find_composition (pos, limit, &raw mut start, &raw mut end, &raw mut prop,
			 (*coding).src_object)
      || end > limit
    { *stop = limit; }
  else if start > pos
    { *stop = start; }
  else
    {
      macro_rules! ADD_ANNOTATION_DATA {
	($buf:expr, $len:expr, $mask:expr, $nchars:expr) => {
	  {
	    *$buf = -($len); $buf = $buf.offset (1);
	    *$buf = ($mask) as c_int; $buf = $buf.offset (1);
	    *$buf = ($nchars) as c_int; $buf = $buf.offset (1);
	    coding_bits_set (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1, 1);
	  }
	};
      }
      macro_rules! ADD_COMPOSITION_DATA {
	($buf:expr, $nchars:expr, $nbytes:expr, $method:expr) => {
	  {
	    ADD_ANNOTATION_DATA! ($buf, 5, CODING_ANNOTATE_COMPOSITION_MASK as c_int, $nchars);
	    *$buf = $nbytes; $buf = $buf.offset (1);
	    *$buf = $method; $buf = $buf.offset (1);
	  }
	};
      }

      if start == pos
	{
	  /* We found a composition.  Store the corresponding
	     annotation data in BUF.  */
	  let head: *mut c_int = buf;
	  let method: composition_method = composition_method (prop);
	  let nchars: c_int = COMPOSITION_LENGTH (prop) as c_int;

	  ADD_COMPOSITION_DATA! (buf, nchars, 0, method);
	  if method != COMPOSITION_RELATIVE
	    {
	      let mut components: Lisp_Object;
	      let mut i: ptrdiff_t;
	      let mut len: ptrdiff_t;
	      let mut i_byte: ptrdiff_t;

	      components = COMPOSITION_COMPONENTS (prop);
	      if VECTORP (components)
		{
		  len = ASIZE (components);
		  i = 0;
		  while i < len
		    {
		      *buf = XFIXNUM (AREF (components, i)) as c_int; buf = buf.offset (1);
		      i += 1;
		    }
		}
	      else if STRINGP (components)
		{
		  len = SCHARS (components);
		  i = 0; i_byte = 0;
		  while i < len
		    {
		      *buf = fetch_string_char_advance (components,
							&raw mut i, &raw mut i_byte);
		      buf = buf.offset (1);
		    }
		}
	      else if FIXNUMP (components)
		{
		  len = 1;
		  *buf = XFIXNUM (components) as c_int; buf = buf.offset (1);
		}
	      else if CONSP (components)
		{
		  len = 0;
		  while CONSP (components)
		    {
		      *buf = XFIXNUM (XCAR (components)) as c_int; buf = buf.offset (1);
		      len += 1; components = XCDR (components);
		    }
		}
	      else
		{ emacs_abort (); }
	      *head -= len as c_int;
	    }
	}

      if find_composition (end, limit, &raw mut start, &raw mut end, &raw mut prop,
			   (*coding).src_object)
	  && end <= limit
	{ *stop = start; }
      else
	{ *stop = limit; }
    }
  return buf;
}


/* Extract an annotation datum from a text property `charset' at POS of
   CODING->src_object (buffer of string), store the data in BUF, set
   *STOP to the position where the value of `charset' property changes
   (limiting by LIMIT), and return the address of the next element of
   BUF.

   If the property value is nil, set *STOP to the position where the
   property value is non-nil (limiting by LIMIT), and return BUF.  */

unsafe fn handle_charset_annotation (pos: ptrdiff_t, limit: ptrdiff_t,
				     coding: *mut coding_system, mut buf: *mut c_int,
				     stop: *mut ptrdiff_t) -> *mut c_int
{
  let val: Lisp_Object;
  let next: Lisp_Object;
  let mut id: c_int;

  macro_rules! ADD_ANNOTATION_DATA {
    ($buf:expr, $len:expr, $mask:expr, $nchars:expr) => {
      {
	*$buf = -($len); $buf = $buf.offset (1);
	*$buf = ($mask) as c_int; $buf = $buf.offset (1);
	*$buf = ($nchars) as c_int; $buf = $buf.offset (1);
	coding_bits_set (coding, CODING_SYSTEM_ANNOTATED_SHIFT, 1, 1);
      }
    };
  }
  macro_rules! ADD_CHARSET_DATA {
    ($buf:expr, $nchars:expr, $id:expr) => {
      {
	ADD_ANNOTATION_DATA! ($buf, 4, CODING_ANNOTATE_CHARSET_MASK as c_int, $nchars);
	*$buf = $id; $buf = $buf.offset (1);
      }
    };
  }

  val = Fget_text_property (make_fixnum (pos as EMACS_INT), Qcharset (), (*coding).src_object);
  if ! NILP (val) && CHARSETP! (val)
    { id = XFIXNUM (CHARSET_SYMBOL_ID! (val)) as c_int; }
  else
    { id = -1; }
  ADD_CHARSET_DATA! (buf, 0, id);
  next = Fnext_single_property_change (make_fixnum (pos as EMACS_INT), Qcharset (),
				       (*coding).src_object,
				       make_fixnum (limit as EMACS_INT));
  *stop = XFIXNUM (next) as ptrdiff_t;
  return buf;
}

/* Encode the text at CODING->src_object into CODING->dst_object.
   ... (see C) */

unsafe fn consume_chars (coding: *mut coding_system, translation_table: Lisp_Object,
			 max_lookup: c_int)
{
  let mut buf: *mut c_int = (*coding).charbuf;
  let mut buf_end: *mut c_int = (*coding).charbuf.offset ((*coding).charbuf_size as isize);
  let mut src: *const c_uchar = (*coding).source.offset ((*coding).consumed);
  let mut src_end: *const c_uchar = (*coding).source.offset ((*coding).src_bytes);
  let mut pos: ptrdiff_t = (*coding).src_pos + (*coding).consumed_char;
  let end_pos: ptrdiff_t = (*coding).src_pos + (*coding).src_chars;
  let multibytep = src_multibyte_p (coding);
  let mut eol_type: Lisp_Object;
  let mut c: c_int;
  let mut stop: ptrdiff_t;
  let mut stop_composition: ptrdiff_t;
  let mut stop_charset: ptrdiff_t;
  /* C's `lookup_buf = alloca (sizeof (int) * max_lookup)`: the port has no `alloca`
     (PORTING.md 3.2/P-05 is open), and `MAX_LOOKUP_MAX` is *defined* as
     `MAX_ALLOCA / sizeof (int)` exactly because C allocates this buffer with `alloca`
     (C 6850-6853), so the port takes the same bounded extent from its own frame; only the
     first `max_lookup` entries are ever touched, as in C.  Recorded in
     docs/ports/coding.md.  */
  let mut lookup_buf_store: [c_int; MAX_LOOKUP_MAX as usize] = [0; MAX_LOOKUP_MAX as usize];
  let mut lookup_buf: *mut c_int = std::ptr::null_mut ();

  if ! NILP (translation_table)
    { lookup_buf = lookup_buf_store.as_mut_ptr (); }

  eol_type = if inhibit_eol_conversion { Qunix () } else { CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t) };
  if VECTORP (eol_type)
    { eol_type = Qunix (); }

  /* Note: composition handling is not yet implemented.  */
  common_flags_set (coding, common_flags_of (coding) & ! (CODING_ANNOTATE_COMPOSITION_MASK as c_uint));

  if NILP ((*coding).src_object)
    { stop = end_pos; stop_composition = end_pos; stop_charset = end_pos; }
  else
    {
      if (common_flags_of (coding) & CODING_ANNOTATE_COMPOSITION_MASK) != 0
	{ stop = pos; stop_composition = pos; }
      else
	{ stop = end_pos; stop_composition = end_pos; }
      if (common_flags_of (coding) & CODING_ANNOTATE_CHARSET_MASK) != 0
	{ stop = pos; stop_charset = pos; }
      else
	{ stop_charset = end_pos; }
    }

  /* Compensate for CRLF and conversion.  */
  buf_end = buf_end.offset (-(1 + MAX_ANNOTATION_LENGTH) as isize);
  while buf < buf_end
    {
      let mut trans: Lisp_Object;

      if pos == stop
	{
	  if pos == end_pos
	    { break; }
	  if pos == stop_composition
	    { buf = handle_composition_annotation (pos, end_pos, coding,
						  buf, &raw mut stop_composition); }
	  if pos == stop_charset
	    { buf = handle_charset_annotation (pos, end_pos, coding,
					       buf, &raw mut stop_charset); }
	  stop = min (stop_composition, stop_charset);
	}

      if ! multibytep
	{
	  if (*coding).encoder == Some (encode_coding_raw_text)
	      || (*coding).encoder == Some (encode_coding_ccl)
	    { c = *src as c_int; src = src.offset (1); pos += 1; }
	  else
	    {
	      let bytes: c_int = multibyte_length (src, src_end, true, true);
	      if 0 < bytes
		{ c = STRING_CHAR_ADVANCE_NO_UNIFY! (src); pos += bytes as ptrdiff_t; }
	      else
		{ c = BYTE8_TO_CHAR (*src as c_int); src = src.offset (1); pos += 1; }
	    }
	}
      else
	{ c = STRING_CHAR_ADVANCE_NO_UNIFY! (src); pos += 1; }
      if c == '\r' as c_int
	  && mode_of (coding) as c_int & CODING_MODE_SELECTIVE_DISPLAY != 0
	{ c = '\n' as c_int; }
      if ! EQ (eol_type, Qunix ())
	{
	  if c == '\n' as c_int
	    {
	      if EQ (eol_type, Qdos ())
		{ *buf = '\r' as c_int; buf = buf.offset (1); }
	      else
		{ c = '\r' as c_int; }
	    }
	}

      trans = Qnil ();
      {
	macro_rules! LOOKUP_TRANSLATION_TABLE {
	  ($table:expr, $c:ident, $trans:ident) => {
	    {
	      $trans = Qnil ();
	      if CHAR_TABLE_P ($table)
		{
		  $trans = CHAR_TABLE_REF ($table, $c as isize);
		  if CHARACTERP ($trans)
		    { $c = XFIXNAT ($trans) as c_int; $trans = Qnil (); }
		}
	      else if CONSP ($table)
		{
		  let mut tail: Lisp_Object;

		  tail = $table;
		  while CONSP (tail)
		    {
		      if CHAR_TABLE_P (XCAR (tail))
			{
			  $trans = CHAR_TABLE_REF (XCAR (tail), $c as isize);
			  if CHARACTERP ($trans)
			    { $c = XFIXNAT ($trans) as c_int; $trans = Qnil (); }
			  else if ! NILP ($trans)
			    { break; }
			}
		      tail = XCDR (tail);
		    }
		}
	    }
	  };
	}

	LOOKUP_TRANSLATION_TABLE! (translation_table, c, trans);
      }
      if NILP (trans)
	{ *buf = c; buf = buf.offset (1); }
      else
	{
	  let mut from_nchars: ptrdiff_t = 1;
	  let mut to_nchars: ptrdiff_t = 1;
	  let lookup_buf_end: *mut c_int;
	  let mut p: *const c_uchar = src;
	  /* C's three `i` loops are one `int`; Rust needs the loop variable's type to match
	     both `ASIZE`/`to_nchars` (ptrdiff_t) and `lookup_buf`'s index.  */
	  let mut i: ptrdiff_t;

	  *lookup_buf = c;
	  i = 1;
	  while i < max_lookup as isize && p < src_end
	    {
	      *lookup_buf.offset (i) = string_char_advance (&raw mut p);
	      i += 1;
	    }
	  lookup_buf_end = lookup_buf.offset (i);
	  trans = get_translation (trans, lookup_buf, lookup_buf_end,
				   &raw mut from_nchars);
	  if FIXNUMP (trans)
	    { c = XFIXNUM (trans) as c_int; }
	  else if VECTORP (trans)
	    {
	      to_nchars = ASIZE (trans);
	      if buf_end.offset_from (buf) < to_nchars
		{ break; }
	      c = XFIXNUM (AREF (trans, 0)) as c_int;
	    }
	  else
	    { break; }
	  *buf = c; buf = buf.offset (1);
	  i = 1;
	  while i < to_nchars
	    {
	      *buf = XFIXNUM (AREF (trans, i)) as c_int; buf = buf.offset (1);
	      i += 1;
	    }
	  i = 1;
	  while i < from_nchars
	    {
	      src = src.offset (multibyte_length (src, std::ptr::null (), false, true) as isize);
	      pos += 1;
	      i += 1;
	    }
	}
    }

  (*coding).consumed = src.offset_from ((*coding).source);
  (*coding).consumed_char = pos - (*coding).src_pos;
  (*coding).charbuf_used = buf.offset_from ((*coding).charbuf) as c_int;
  coding_bits_set (coding, CODING_SYSTEM_CHARS_AT_SOURCE_SHIFT, 1, 0);
}


/* Encode the text at CODING->src_object into CODING->dst_object.
   CODING->src_object is a buffer or a string.
   CODING->dst_object is a buffer or nil.

   If CODING->src_object is a buffer, it must be the current buffer.
   In this case, if CODING->src_pos is positive, it is a position of
   the source text in the buffer, otherwise. the source text is in the
   gap area of the buffer, and coding->src_pos specifies the offset of
   the text from GPT (which must be the same as PT).  If this is the
   same buffer as CODING->dst_object, CODING->src_pos must be
   negative and CODING should not have `pre-write-conversion'.

   If CODING->src_object is a string, CODING should not have
   `pre-write-conversion'.

   If CODING->dst_object is a buffer, the encoded data is inserted at
   the current point of that buffer.

   If CODING->dst_object is nil, the encoded data is placed at the
   memory area specified by CODING->destination.  */

unsafe fn encode_coding (coding: *mut coding_system)
{
  let attrs: Lisp_Object;
  let translation_table: Lisp_Object;
  let mut max_lookup: c_int = 0;
  let mut cclspec: ccl_spec = std::mem::zeroed ();

  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  attrs = CODING_ID_ATTRS ((*coding).id as hash_idx_t);
  if (*coding).encoder == Some (encode_coding_raw_text)
    { translation_table = Qnil (); max_lookup = 0; }
  else
    { translation_table = get_translation_table (attrs, true, &raw mut max_lookup); }

  if BUFFERP ((*coding).dst_object)
    {
      set_buffer_internal (XBUFFER ((*coding).dst_object));
      coding_bits_set (coding, CODING_SYSTEM_DST_MULTIBYTE_SHIFT, 1,
		       if ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)) { 1 } else { 0 });
    }

  (*coding).consumed = 0; (*coding).consumed_char = 0;
  (*coding).produced = 0; (*coding).produced_char = 0;
  record_conversion_result (coding, CODING_RESULT_SUCCESS);

  /* C's `ALLOC_CONVERSION_WORK_AREA (coding, size)`: `SAFE_ALLOCA` reads the caller's
     `sa_avail`, so the macro is defined in each using function (PORTING.md P-57).  */
  macro_rules! ALLOC_CONVERSION_WORK_AREA {
    ($coding:expr, $size:expr) => {
      {
	let units: ptrdiff_t
	  = min (($size) as isize + MAX_CHARBUF_EXTRA_SIZE as isize, MAX_CHARBUF_SIZE as isize);
	(*$coding).charbuf = SAFE_ALLOCA! (sa_avail, units * 4) as *mut c_int;
	(*$coding).charbuf_size = units as c_int;
      }
    };
  }

  ALLOC_CONVERSION_WORK_AREA! (coding, (*coding).src_chars);

  if (*coding).encoder == Some (encode_coding_ccl)
    {
      (*coding).spec.ccl = &raw mut cclspec as *mut c_void;
      setup_ccl_program (&raw mut cclspec.ccl, CODING_CCL_ENCODER! (coding));
    }
  loop
    {
      coding_set_source (coding);
      consume_chars (coding, translation_table, max_lookup);
      coding_set_destination (coding);
      /* The CODING_MODE_LAST_BLOCK flag should be set only for the last
	 iteration of the encoding.  */
      let saved_mode: c_uint = mode_of (coding);
      if (*coding).consumed_char < (*coding).src_chars
	{ coding_mode_clear (coding, CODING_MODE_LAST_BLOCK); }
      ((*coding).encoder.unwrap ()) (coding);
      coding_bits_set (coding, CODING_SYSTEM_MODE_SHIFT, CODING_SYSTEM_MODE_MASK, saved_mode);
      if ! ((*coding).consumed_char < (*coding).src_chars)
	{ break; }
    }

  if BUFFERP ((*coding).dst_object) && (*coding).produced_char > 0
    {
      insert_from_gap ((*coding).produced_char, (*coding).produced, false,
		       coding_bits_get (coding, CODING_SYSTEM_INSERT_BEFORE_MARKERS_SHIFT, 1) != 0);
    }

  SAFE_FREE! (sa_count);
}

/* Code-conversion operations use internal buffers.  There's a single
   reusable buffer, which is created the first time it is needed, and
   then never killed.  When this reusable buffer is being used, the
   reused_workbuf_in_use flag is set.  If we need another conversion
   buffer while the reusable one is in use (e.g., if code-conversion
   is reentered when another code-conversion is in progress), we
   create temporary buffers using the name of the reusable buffer as
   the base name, see code_conversion_save below.  These temporary
   buffers are killed when the code-conversion operations that use
   them return, see code_conversion_restore below.  */

/* A string that serves as name of the reusable work buffer, and as base
   name of temporary work buffers used for code-conversion operations.  */
static mut Vcode_conversion_workbuf_name: Lisp_Object = lisp_h_Qnil ();

/* The reusable working buffer, created once and never killed.  */
static mut Vcode_conversion_reused_workbuf: Lisp_Object = lisp_h_Qnil ();

/* True iff Vcode_conversion_reused_workbuf is already in use.  */
static mut reused_workbuf_in_use: bool = false;

unsafe extern "C" fn code_conversion_restore (arg: Lisp_Object)
{
  let current: Lisp_Object;
  let workbuf: Lisp_Object;

  current = XCAR (arg);
  workbuf = XCDR (arg);
  if ! NILP (workbuf)
    {
      if EQ (workbuf, Vcode_conversion_reused_workbuf)
	{ reused_workbuf_in_use = false; }
      else
	{ Fkill_buffer (workbuf); }
    }
  set_buffer_internal (XBUFFER (current));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn code_conversion_save (with_work_buf: bool, multibyte: bool) -> Lisp_Object
{
  let mut workbuf: Lisp_Object = Qnil ();

  if with_work_buf
    {
      if reused_workbuf_in_use
	{
	  let name: Lisp_Object
	    = Fgenerate_new_buffer_name (Vcode_conversion_workbuf_name, Qnil ());
	  workbuf = Fget_buffer_create (name, Qt ());
	}
      else
	{
	  if NILP (Fbuffer_live_p (Vcode_conversion_reused_workbuf))
	    {
	      Vcode_conversion_reused_workbuf
		= Fget_buffer_create (Vcode_conversion_workbuf_name, Qt ());
	    }
	  workbuf = Vcode_conversion_reused_workbuf;
	}
    }
  record_unwind_protect (Some (code_conversion_restore),
			 Fcons (Fcurrent_buffer (), workbuf));
  if ! NILP (workbuf)
    {
      let current: *mut buffer = (current_buffer! ());
      set_buffer_internal (XBUFFER (workbuf));
      /* We can't allow modification hooks to run in the work buffer.  For
	 instance, directory_files_internal assumes that file decoding
	 doesn't compile new regexps.  */
      Fset (Fmake_local_variable (Qinhibit_modification_hooks ()), Qt ());
      Ferase_buffer ();
      bset_undo_list ((current_buffer! ()), Qt ());
      bset_enable_multibyte_characters ((current_buffer! ()), if multibyte { Qt () } else { Qnil () });
      if EQ (workbuf, Vcode_conversion_reused_workbuf)
	{ reused_workbuf_in_use = true; }
      /* FIXME: Maybe we should stay in the new workbuf, because we often
	 switch right back to it anyway in order to initialize it further.  */
      set_buffer_internal (current);
    }

  return workbuf;
}

unsafe extern "C" fn coding_restore_undo_list (arg: Lisp_Object)
{
  let undo_list: Lisp_Object = XCAR (arg);
  let buf: *mut buffer = XBUFFER (XCDR (arg));

  bset_undo_list (buf, undo_list);
}

/* Decode the *last* BYTES of the gap and insert them at point.  */
/* `femacs/src/include/coding.rs` declares `decode_coding_gap` at its crate root
   (coding.h 706), so the definition lives in a glob-imported module (the `space_glyph_def`
   device; reported as a mirror gap in docs/ports/coding.md).  */
pub mod decode_coding_gap_def {
  use super::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_coding_gap (coding: *mut coding_system, mut bytes: ptrdiff_t)
{
  let mut count: specpdl_ref = SPECPDL_INDEX ();
  let mut attrs: Lisp_Object;

  eassert! (GPT_BYTE! () == PT_BYTE! ());

  (*coding).src_object = Fcurrent_buffer ();
  (*coding).src_chars = bytes;
  (*coding).src_bytes = bytes;
  (*coding).src_pos = -bytes;
  (*coding).src_pos_byte = -bytes;
  coding_bits_set (coding, CODING_SYSTEM_SRC_MULTIBYTE_SHIFT, 1, 0);
  (*coding).dst_object = (*coding).src_object;
  (*coding).dst_pos = PT! ();
  (*coding).dst_pos_byte = PT_BYTE! ();
  eassert! ((coding_bits_get (coding, CODING_SYSTEM_DST_MULTIBYTE_SHIFT, 1) != 0)
	    == ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)));

  (*coding).head_ascii = -1;
  (*coding).detected_utf8_bytes = -1; (*coding).detected_utf8_chars = -1;
  coding_eol_seen_set (coding, EOL_SEEN_NONE);
  if CODING_REQUIRE_DETECTION (coding) != 0
    { detect_coding (coding); }
  attrs = CODING_ID_ATTRS ((*coding).id as hash_idx_t);
  if ! disable_ascii_optimization
      && ! src_multibyte_p (coding)
      && ! NILP (CODING_ATTR_ASCII_COMPAT (attrs))
      && NILP (CODING_ATTR_POST_READ (attrs))
      && NILP (get_translation_table (attrs, false, std::ptr::null_mut ()))
    {
      let mut chars: ptrdiff_t = (*coding).head_ascii;
      if chars < 0
	{ chars = check_ascii (coding); }
      if chars != bytes
	{
	  /* There exists a non-ASCII byte.  */
	  if EQ (CODING_ATTR_TYPE (attrs), Qutf_8 ())
	      && (*coding).detected_utf8_bytes == (*coding).src_bytes
	    {
	      if (*coding).detected_utf8_chars >= 0
		{ chars = (*coding).detected_utf8_chars; }
	      else
		{ chars = check_utf_8 (coding); }
	      if CODING_UTF_8_BOM! (coding) != utf_without_bom
		  && (*coding).head_ascii == 0
		  && *(*coding).source.offset (0) as c_int == UTF_8_BOM_1
		  && *(*coding).source.offset (1) as c_int == UTF_8_BOM_2
		  && *(*coding).source.offset (2) as c_int == UTF_8_BOM_3
		{
		  chars -= 1;
		  bytes -= 3;
		  (*coding).src_bytes -= 3;
		}
	    }
	  else
	    { chars = -1; }
	}
      if chars >= 0
	{
	  let mut eol_type: Lisp_Object;

	  eol_type = CODING_ID_EOL_TYPE ((*coding).id as hash_idx_t);
	  if VECTORP (eol_type)
	    {
	      if eol_seen_of (coding) != EOL_SEEN_NONE
		{ eol_type = adjust_coding_eol_type (coding, eol_seen_of (coding)); }
	    }
	  if EQ (eol_type, Qmac ())
	    {
	      let src_end: *mut c_uchar = GAP_END_ADDR! ();
	      let mut src: *mut c_uchar = src_end.offset (-(*coding).src_bytes);

	      while src < src_end
		{
		  if *src == '\r' as c_uchar
		    { *src = '\n' as c_uchar; }
		  src = src.offset (1);
		}
	    }
	  else if EQ (eol_type, Qdos ())
	    {
	      let src: *mut c_uchar = GAP_END_ADDR! ();
	      let src_beg: *mut c_uchar = src.offset (-(*coding).src_bytes);
	      let mut dst: *mut c_uchar = src;
	      let mut src_pos: *mut c_uchar = src;
	      let diff: ptrdiff_t;

	      while src_beg < src_pos
		{
		  dst = dst.offset (-1); src_pos = src_pos.offset (-1);
		  *dst = *src_pos;
		  if *src_pos == '\n' as c_uchar && src_pos > src_beg
		      && *src_pos.offset (-1) == '\r' as c_uchar
		    { src_pos = src_pos.offset (-1); }
		}
	      diff = dst.offset_from (src_pos);
	      bytes -= diff;
	      chars -= diff;
	    }
	  (*coding).produced = bytes;
	  (*coding).produced_char = chars;
	  insert_from_gap (chars, bytes, true,
			   coding_bits_get (coding, CODING_SYSTEM_INSERT_BEFORE_MARKERS_SHIFT, 1) != 0);
	  return;
	}
    }
  code_conversion_save (false, false);

  coding_mode_set (coding, CODING_MODE_LAST_BLOCK);
  (*(*(current_buffer! ())).text).bits |= BUFFER_TEXT_INHIBIT_SHRINKING;
  decode_coding (coding);
  (*(*(current_buffer! ())).text).bits &= ! BUFFER_TEXT_INHIBIT_SHRINKING;

  if ! NILP (CODING_ATTR_POST_READ (attrs))
    {
      let prev_Z: ptrdiff_t = Z! (); let prev_Z_BYTE: ptrdiff_t = Z_BYTE! ();
      let val: Lisp_Object;
      let undo_list: Lisp_Object = BVAR! ((current_buffer! ()), undo_list_);

      record_unwind_protect (Some (coding_restore_undo_list),
			     Fcons (undo_list, Fcurrent_buffer ()));
      bset_undo_list ((current_buffer! ()), Qt ());
      TEMP_SET_PT_BOTH ((*coding).dst_pos, (*coding).dst_pos_byte);
      val = calln! (CODING_ATTR_POST_READ (attrs),
		    make_fixnum ((*coding).produced_char as EMACS_INT));
      CHECK_FIXNAT (val);
      (*coding).produced_char += Z! () - prev_Z;
      (*coding).produced += Z_BYTE! () - prev_Z_BYTE;
    }

  unbind_to (count, Qnil ());
}
}
use decode_coding_gap_def::*;


/* Decode the text in the range FROM/FROM_BYTE and TO/TO_BYTE in
   SRC_OBJECT into DST_OBJECT by coding context CODING.

   SRC_OBJECT is a buffer, a string, or Qnil.

   If it is a buffer, the text is at point of the buffer.  FROM and TO
   are positions in the buffer.

   If it is a string, the text is at the beginning of the string.
   FROM and TO are indices to the string.

   If it is nil, the text is at coding->source.  FROM and TO are
   indices to coding->source.

   DST_OBJECT is a buffer, Qt, or Qnil.

   If it is a buffer, the decoded text is inserted at point of the
   buffer.  If the buffer is the same as SRC_OBJECT, the source text
   is deleted.

   If it is Qt, a string is made from the decoded text, and
   set in CODING->dst_object.

   If it is Qnil, the decoded text is stored at CODING->destination.
   The caller must allocate CODING->dst_bytes bytes at
   CODING->destination by xmalloc.  If the decoded text is longer than
   CODING->dst_bytes, CODING->destination is relocated by xrealloc.
 */
/* `femacs/src/include/coding.rs` declares `decode_coding_object` at its crate root
   (coding.h 691-713), so the definition lives in a glob-imported module (the
   `decode_coding_gap_def` device; reported as a mirror gap in docs/ports/coding.md).  */
pub mod decode_coding_object_def {
  use super::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_coding_object (coding: *mut coding_system,
					       src_object: Lisp_Object,
					       from: ptrdiff_t, from_byte: ptrdiff_t,
					       to: ptrdiff_t, to_byte: ptrdiff_t,
					       dst_object: Lisp_Object)
{
  let mut count: specpdl_ref = SPECPDL_INDEX ();
  let mut destination: *mut u8 = std::ptr::null_mut ();
  let mut dst_bytes: ptrdiff_t = 0;
  let mut chars: ptrdiff_t = to - from;
  let mut bytes: ptrdiff_t = to_byte - from_byte;
  let mut attrs: Lisp_Object;
  let mut saved_pt: ptrdiff_t = -1; let mut saved_pt_byte: ptrdiff_t = 0;
  let mut need_marker_adjustment: bool = false;
  let mut old_deactivate_mark: Lisp_Object;

  old_deactivate_mark = Vdeactivate_mark;

  if NILP (dst_object)
    {
      destination = (*coding).destination;
      dst_bytes = (*coding).dst_bytes;
    }

  (*coding).src_object = src_object;
  (*coding).src_chars = chars;
  (*coding).src_bytes = bytes;
  coding_bits_set (coding, CODING_SYSTEM_SRC_MULTIBYTE_SHIFT, 1, (chars < bytes) as u32);

  if STRINGP (src_object)
    {
      (*coding).src_pos = from;
      (*coding).src_pos_byte = from_byte;
    }
  else if BUFFERP (src_object)
    {
      set_buffer_internal (XBUFFER (src_object));
      if from != GPT! ()
	{ move_gap_both (from, from_byte); }
      if BASE_EQ (src_object, dst_object)
	{
	  let mut tail: *mut Lisp_Marker;

	  tail = BUF_MARKERS! ((current_buffer! ()));
	  while ! tail.is_null ()
	    {
	      (*tail).bits
		= ((*tail).bits & ! MARKER_NEED_ADJUSTMENT)
		  | (if (*tail).charpos
			 == (if MARKER_INSERTION_TYPE_P (tail) { from } else { to })
		     { MARKER_NEED_ADJUSTMENT } else { 0 });
	      need_marker_adjustment |= (*tail).bits & MARKER_NEED_ADJUSTMENT != 0;
	      tail = (*tail).next;
	    }
	  saved_pt = PT! (); saved_pt_byte = PT_BYTE! ();
	  TEMP_SET_PT_BOTH (from, from_byte);
	  (*(*(current_buffer! ())).text).bits |= BUFFER_TEXT_INHIBIT_SHRINKING;
	  prepare_to_modify_buffer (from, to, std::ptr::null_mut ());
	  del_range_2 (from, from_byte, to, to_byte, false);
	  (*coding).src_pos = -chars;
	  (*coding).src_pos_byte = -bytes;
	}
      else
	{
	  (*coding).src_pos = from;
	  (*coding).src_pos_byte = from_byte;
	}
    }

  if CODING_REQUIRE_DETECTION (coding) != 0
    { detect_coding (coding); }
  attrs = CODING_ID_ATTRS ((*coding).id as hash_idx_t);

  if EQ (dst_object, Qt ())
      || (! NILP (CODING_ATTR_POST_READ (attrs))
	  && NILP (dst_object))
    {
      coding_bits_set (coding, CODING_SYSTEM_DST_MULTIBYTE_SHIFT, 1,
		       (CODING_FOR_UNIBYTE (coding) == 0) as u32);
      (*coding).dst_object = code_conversion_save (true, dst_multibyte_p (coding));
      (*coding).dst_pos = BEG as ptrdiff_t;
      (*coding).dst_pos_byte = BEG_BYTE as ptrdiff_t;
    }
  else if BUFFERP (dst_object)
    {
      if ! BASE_EQ (src_object, dst_object)
	{
	  let current: *mut buffer = (current_buffer! ());
	  set_buffer_internal (XBUFFER (dst_object));
	  prepare_to_modify_buffer (PT! (), PT! (), std::ptr::null_mut ());
	  set_buffer_internal (current);
	}
      code_conversion_save (false, false);
      (*coding).dst_object = dst_object;
      (*coding).dst_pos = BUF_PT (XBUFFER (dst_object));
      (*coding).dst_pos_byte = BUF_PT_BYTE (XBUFFER (dst_object));
      coding_bits_set (coding, CODING_SYSTEM_DST_MULTIBYTE_SHIFT, 1,
		       (! NILP (BVAR! (XBUFFER (dst_object), enable_multibyte_characters_)))
		       as u32);
    }
  else
    {
      code_conversion_save (false, false);
      (*coding).dst_object = Qnil ();
      /* Most callers presume this will return a multibyte result, and they
	 won't use `binary' or `raw-text' anyway, so let's not worry about
	 CODING_FOR_UNIBYTE.  */
      coding_bits_set (coding, CODING_SYSTEM_DST_MULTIBYTE_SHIFT, 1, 1);
    }

  decode_coding (coding);

  if BUFFERP ((*coding).dst_object)
    {
      set_buffer_internal (XBUFFER ((*coding).dst_object));
      signal_after_change ((*coding).dst_pos,
			   if BASE_EQ (src_object, dst_object) { to - from } else { 0 },
			   (*coding).produced_char);
      update_compositions ((*coding).dst_pos,
			   (*coding).dst_pos + (*coding).produced_char, CHECK_ALL);
    }

  if ! NILP (CODING_ATTR_POST_READ (attrs))
    {
      let prev_Z: ptrdiff_t = Z! (); let prev_Z_BYTE: ptrdiff_t = Z_BYTE! ();
      let val: Lisp_Object;
      let undo_list: Lisp_Object = BVAR! ((current_buffer! ()), undo_list_);
      let mut count1: specpdl_ref = SPECPDL_INDEX ();

      record_unwind_protect (Some (coding_restore_undo_list),
			     Fcons (undo_list, Fcurrent_buffer ()));
      bset_undo_list ((current_buffer! ()), Qt ());
      TEMP_SET_PT_BOTH ((*coding).dst_pos, (*coding).dst_pos_byte);
      val = safe_calln! (CODING_ATTR_POST_READ (attrs),
			 make_fixnum ((*coding).produced_char as EMACS_INT));
      CHECK_FIXNAT (val);
      (*coding).produced_char += Z! () - prev_Z;
      (*coding).produced += Z_BYTE! () - prev_Z_BYTE;
      unbind_to (count1, Qnil ());
    }

  if EQ (dst_object, Qt ())
    {
      (*coding).dst_object = Fbuffer_string ();
    }
  else if NILP (dst_object) && BUFFERP ((*coding).dst_object)
    {
      set_buffer_internal (XBUFFER ((*coding).dst_object));
      if dst_bytes < (*coding).produced
	{
	  eassert! ((*coding).produced > 0);
	  destination = xrealloc (destination as *mut c_void,
				  (*coding).produced as usize) as *mut u8;
	  if BEGV! () < GPT! () && GPT! () < BEGV! () + (*coding).produced_char
	    { move_gap_both (BEGV! (), BEGV_BYTE! ()); }
	  memcpy (destination as *mut c_void, BEGV_ADDR! () as *const c_void,
		  (*coding).produced as size_t);
	  (*coding).destination = destination;
	}
    }

  if saved_pt >= 0
    {
      /* This is the case of:
	 (BUFFERP (src_object) && BASE_EQ (src_object, dst_object))
	 As we have moved PT while replacing the original buffer
	 contents, we must recover it now.  */
      set_buffer_internal (XBUFFER (src_object));
      (*(*(current_buffer! ())).text).bits &= ! BUFFER_TEXT_INHIBIT_SHRINKING;
      if saved_pt < from
	{ TEMP_SET_PT_BOTH (saved_pt, saved_pt_byte); }
      else if saved_pt < from + chars
	{ TEMP_SET_PT_BOTH (from, from_byte); }
      else if ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	{
	  TEMP_SET_PT_BOTH (saved_pt + ((*coding).produced_char - chars),
			    saved_pt_byte + ((*coding).produced - bytes));
	}
      else
	{
	  TEMP_SET_PT_BOTH (saved_pt + ((*coding).produced - bytes),
			    saved_pt_byte + ((*coding).produced - bytes));
	}

      if need_marker_adjustment
	{
	  let mut tail: *mut Lisp_Marker;

	  tail = BUF_MARKERS! ((current_buffer! ()));
	  while ! tail.is_null ()
	    {
	      if (*tail).bits & MARKER_NEED_ADJUSTMENT != 0
		{
		  (*tail).bits &= ! MARKER_NEED_ADJUSTMENT;
		  if MARKER_INSERTION_TYPE_P (tail)
		    {
		      (*tail).bytepos = from_byte;
		      (*tail).charpos = from;
		    }
		  else
		    {
		      (*tail).bytepos = from_byte + (*coding).produced;
		      (*tail).charpos
			= (if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
			     { (*tail).bytepos } else { from + (*coding).produced_char });
		    }
		}
	      tail = (*tail).next;
	    }
	}
    }

  Vdeactivate_mark = old_deactivate_mark;
  unbind_to (count, (*coding).dst_object);
}
}
use decode_coding_object_def::*;

/* Encode the text in the range FROM/FROM_BYTE and TO/TO_BYTE in
   SRC_OBJECT into DST_OBJECT by coding context CODING.

   SRC_OBJECT is a buffer, a string, or Qnil.

   If it is a buffer, the text is at point of the buffer.  FROM and TO
   are positions in the buffer.

   If it is a string, the text is at the beginning of the string.
   FROM and TO are indices into the string.

   If it is nil, the text is at coding->source.  FROM and TO are
   indices into coding->source.

   DST_OBJECT is a buffer, Qt, or Qnil.

   If it is a buffer, the encoded text is inserted at point of the
   buffer.  If the buffer is the same as SRC_OBJECT, the source text
   is replaced with the encoded text.

   If it is Qt, a string is made from the encoded text, and set in
   CODING->dst_object.  However, if CODING->raw_destination is non-zero,
   the encoded text is instead returned in CODING->destination as a C string,
   and the caller is responsible for freeing CODING->destination.  This
   feature is meant to be used when the caller doesn't need the result as
   a Lisp string, and wants to avoid unnecessary consing of large strings.

   If it is Qnil, the encoded text is stored at CODING->destination.
   The caller must allocate CODING->dst_bytes bytes at
   CODING->destination by xmalloc.  If the encoded text is longer than
   CODING->dst_bytes, CODING->destination is reallocated by xrealloc
   (and CODING->dst_bytes is enlarged accordingly).  */
/* `femacs/src/include/coding.rs` declares `encode_coding_object` at its crate root
   (coding.h 691-713), so the definition lives in a glob-imported module (the
   `decode_coding_gap_def` device; reported as a mirror gap in docs/ports/coding.md).  */
pub mod encode_coding_object_def {
  use super::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn encode_coding_object (coding: *mut coding_system,
					       src_object: Lisp_Object,
					       mut from: ptrdiff_t, mut from_byte: ptrdiff_t,
					       to: ptrdiff_t, to_byte: ptrdiff_t,
					       dst_object: Lisp_Object)
{
  let mut count: specpdl_ref = SPECPDL_INDEX ();
  let mut chars: ptrdiff_t = to - from;
  let mut bytes: ptrdiff_t = to_byte - from_byte;
  let mut attrs: Lisp_Object;
  let mut saved_pt: ptrdiff_t = -1; let mut saved_pt_byte: ptrdiff_t = 0;
  let mut need_marker_adjustment: bool = false;
  let mut kill_src_buffer: bool = false;
  let mut old_deactivate_mark: Lisp_Object;

  old_deactivate_mark = Vdeactivate_mark;

  (*coding).src_object = src_object;
  (*coding).src_chars = chars;
  (*coding).src_bytes = bytes;
  coding_bits_set (coding, CODING_SYSTEM_SRC_MULTIBYTE_SHIFT, 1, (chars < bytes) as u32);

  attrs = CODING_ID_ATTRS ((*coding).id as hash_idx_t);

  let mut same_buffer: bool = false;
  if BASE_EQ (src_object, dst_object) && BUFFERP (src_object)
    {
      let mut tail: *mut Lisp_Marker;

      same_buffer = true;

      tail = BUF_MARKERS! (XBUFFER (src_object));
      while ! tail.is_null ()
	{
	  (*tail).bits
	    = ((*tail).bits & ! MARKER_NEED_ADJUSTMENT)
	      | (if (*tail).charpos
		     == (if MARKER_INSERTION_TYPE_P (tail) { from } else { to })
		 { MARKER_NEED_ADJUSTMENT } else { 0 });
	  need_marker_adjustment |= (*tail).bits & MARKER_NEED_ADJUSTMENT != 0;
	  tail = (*tail).next;
	}
    }

  if ! NILP (CODING_ATTR_PRE_WRITE (attrs))
    {
      (*coding).src_object = code_conversion_save (true, src_multibyte_p (coding));
      set_buffer_internal (XBUFFER ((*coding).src_object));
      if STRINGP (src_object)
	{ insert_from_string (src_object, from, from_byte, chars, bytes, false); }
      else if BUFFERP (src_object)
	{ insert_from_buffer (XBUFFER (src_object), from, chars, false); }
      else
	{ insert_1_both (((*coding).source as *const c_char).offset (from), chars, bytes,
			 false, false, false); }

      if same_buffer
	{
	  set_buffer_internal (XBUFFER (src_object));
	  saved_pt = PT! (); saved_pt_byte = PT_BYTE! ();
	  del_range_both (from, from_byte, to, to_byte, true);
	  set_buffer_internal (XBUFFER ((*coding).src_object));
	}

      safe_calln! (CODING_ATTR_PRE_WRITE (attrs),
		   make_fixnum (BEG as EMACS_INT), make_fixnum (Z! () as EMACS_INT));
      if XBUFFER ((*coding).src_object) != (current_buffer! ())
	{ kill_src_buffer = true; }
      (*coding).src_object = Fcurrent_buffer ();
      if (BEG as ptrdiff_t) != GPT! ()
	{ move_gap_both (BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t); }
      (*coding).src_chars = Z! () - BEG as ptrdiff_t;
      (*coding).src_bytes = Z_BYTE! () - BEG_BYTE as ptrdiff_t;
      (*coding).src_pos = BEG as ptrdiff_t;
      (*coding).src_pos_byte = BEG_BYTE as ptrdiff_t;
      coding_bits_set (coding, CODING_SYSTEM_SRC_MULTIBYTE_SHIFT, 1,
		       (Z! () < Z_BYTE! ()) as u32);
    }
  else if STRINGP (src_object)
    {
      code_conversion_save (false, false);
      (*coding).src_pos = from;
      (*coding).src_pos_byte = from_byte;
    }
  else if BUFFERP (src_object)
    {
      code_conversion_save (false, false);
      set_buffer_internal (XBUFFER (src_object));
      if same_buffer
	{
	  saved_pt = PT! (); saved_pt_byte = PT_BYTE! ();
	  /* Run 'prepare_to_modify_buffer' by hand because we don't want
	     to run the after-change hooks yet.  */
	  prepare_to_modify_buffer (from, to, &raw mut from);
	  (*coding).src_object = del_range_2 (from, CHAR_TO_BYTE (from),
					      to, CHAR_TO_BYTE (to),
					      true);
	  (*coding).src_pos = 0;
	  (*coding).src_pos_byte = 0;
	}
      else
	{
	  if from < GPT! () && to >= GPT! ()
	    { move_gap_both (from, from_byte); }
	  (*coding).src_pos = from;
	  (*coding).src_pos_byte = from_byte;
	}
    }
  else
    {
      code_conversion_save (false, false);
      (*coding).src_pos = from;
      (*coding).src_pos_byte = from_byte;
    }

  if BUFFERP (dst_object)
    {
      (*coding).dst_object = dst_object;
      if BASE_EQ (src_object, dst_object)
	{
	  (*coding).dst_pos = from;
	  (*coding).dst_pos_byte = from_byte;
	}
      else
	{
	  let current: *mut buffer = (current_buffer! ());

	  set_buffer_internal (XBUFFER (dst_object));
	  prepare_to_modify_buffer (PT! (), PT! (), std::ptr::null_mut ());
	  (*coding).dst_pos = PT! ();
	  (*coding).dst_pos_byte = PT_BYTE! ();
	  move_gap_both ((*coding).dst_pos, (*coding).dst_pos_byte);
	  set_buffer_internal (current);
	}
      coding_bits_set (coding, CODING_SYSTEM_DST_MULTIBYTE_SHIFT, 1,
		       (! NILP (BVAR! (XBUFFER (dst_object), enable_multibyte_characters_)))
		       as u32);
    }
  else if EQ (dst_object, Qt ())
    {
      let dst_bytes: ptrdiff_t = max (1, (*coding).src_chars);
      (*coding).dst_object = Qnil ();
      (*coding).destination = xmalloc (dst_bytes as usize) as *mut u8;
      (*coding).dst_bytes = dst_bytes;
      coding_bits_set (coding, CODING_SYSTEM_DST_MULTIBYTE_SHIFT, 1, 0);
    }
  else
    {
      (*coding).dst_object = Qnil ();
      coding_bits_set (coding, CODING_SYSTEM_DST_MULTIBYTE_SHIFT, 1, 0);
    }

  encode_coding (coding);

  if EQ (dst_object, Qt ())
    {
      if BUFFERP ((*coding).dst_object)
	{ (*coding).dst_object = Fbuffer_string (); }
      else if coding_bits_get (coding, CODING_SYSTEM_RAW_DESTINATION_SHIFT, 1) != 0
	/* This is used to avoid creating huge Lisp string.
	   NOTE: caller who sets `raw_destination' is also
	   responsible for freeing `destination' buffer.  */
	{ (*coding).dst_object = Qnil (); }
      else
	{
	  (*coding).dst_object
	    = make_unibyte_string ((*coding).destination as *const c_char,
				   (*coding).produced);
	  xfree ((*coding).destination as *mut c_void);
	}
    }
  else if BUFFERP ((*coding).dst_object)
    {
      let current: *mut buffer = (current_buffer! ());
      set_buffer_internal (XBUFFER (dst_object));
      signal_after_change ((*coding).dst_pos,
			   if same_buffer { to - from } else { 0 },
			   (*coding).produced_char);
      update_compositions ((*coding).dst_pos,
			   (*coding).dst_pos + (*coding).produced_char, CHECK_ALL);
      set_buffer_internal (current);
    }

  if saved_pt >= 0
    {
      /* This is the case of:
	 (BUFFERP (src_object) && BASE_EQ (src_object, dst_object))
	 As we have moved PT while replacing the original buffer
	 contents, we must recover it now.  */
      set_buffer_internal (XBUFFER (src_object));
      if saved_pt < from
	{ TEMP_SET_PT_BOTH (saved_pt, saved_pt_byte); }
      else if saved_pt < from + chars
	{ TEMP_SET_PT_BOTH (from, from_byte); }
      else if ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	{
	  TEMP_SET_PT_BOTH (saved_pt + ((*coding).produced_char - chars),
			    saved_pt_byte + ((*coding).produced - bytes));
	}
      else
	{
	  TEMP_SET_PT_BOTH (saved_pt + ((*coding).produced - bytes),
			    saved_pt_byte + ((*coding).produced - bytes));
	}

      if need_marker_adjustment
	{
	  let mut tail: *mut Lisp_Marker;

	  tail = BUF_MARKERS! ((current_buffer! ()));
	  while ! tail.is_null ()
	    {
	      if (*tail).bits & MARKER_NEED_ADJUSTMENT != 0
		{
		  (*tail).bits &= ! MARKER_NEED_ADJUSTMENT;
		  if MARKER_INSERTION_TYPE_P (tail)
		    {
		      (*tail).bytepos = from_byte;
		      (*tail).charpos = from;
		    }
		  else
		    {
		      (*tail).bytepos = from_byte + (*coding).produced;
		      (*tail).charpos
			= (if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
			     { (*tail).bytepos } else { from + (*coding).produced_char });
		    }
		}
	      tail = (*tail).next;
	    }
	}
    }

  if kill_src_buffer
    { Fkill_buffer ((*coding).src_object); }

  Vdeactivate_mark = old_deactivate_mark;
  unbind_to (count, Qnil ());
}
}
use encode_coding_object_def::*;

/* `femacs/src/coding.rs`'s `coding_fwd` module declares `preferred_coding_system` (coding.h
   780), and the definition shadows that glob import (PORTING.md 0.1-10).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preferred_coding_system () -> Lisp_Object
{
  let id: c_int = coding_categories[coding_priorities[0] as usize].id as c_int;

  return CODING_ID_NAME (id as hash_idx_t);
}

/*** 8. Emacs Lisp library functions ***/

DEFUN! ("coding-system-p", Fcoding_system_p, Scoding_system_p, 1, 1, 0,
       doc: "Return t if OBJECT is nil or a coding-system.\nSee the documentation of `define-coding-system' for information\nabout coding-system objects.",
       (object: Lisp_Object),
       {
	 if NILP (object)
	     || CODING_SYSTEM_ID (object) >= 0
	   { return Qt (); }
	 if ! SYMBOLP (object)
	     || NILP (Fget (object, Qcoding_system_define_form ()))
	   { return Qnil (); }
	 return Qt ();
       });

DEFUN! ("read-non-nil-coding-system", Fread_non_nil_coding_system,
       Sread_non_nil_coding_system, 1, 1, 0,
       doc: "Read a coding system from the minibuffer, prompting with string PROMPT.\nReturn the symbol of the coding-system.",
       (prompt: Lisp_Object),
       {
	 let mut val: Lisp_Object;
	 /* C's `do { … } while (…)` runs the body once and re-tests; the port's `loop`
	    with the same test as its only exit is the same loop (PORTING.md 2.1).  */
	 loop
	   {
	     val = Fcompleting_read (prompt, Vcoding_system_alist, Qnil (),
				     Qt (), Qnil (), Qcoding_system_history (), Qnil (), Qnil ());
	     if ! (SCHARS (val) == 0) { break; }
	   }
	 return Fintern (val, Qnil ());
       });

DEFUN! ("read-coding-system", Fread_coding_system, Sread_coding_system, 1, 2, 0,
       doc: "Read a coding system from the minibuffer, prompting with string PROMPT.\nIf the user enters null input, return second argument DEFAULT-CODING-SYSTEM.\nReturn the coding-system's symbol, or nil if both the user input and\nDEFAULT-CODING-SYSTEM are empty or null.\nIgnores case when completing coding systems (all Emacs coding systems\nare lower-case).",
       (prompt: Lisp_Object, default_coding_system: Lisp_Object),
       {
	 let mut val: Lisp_Object;
	 let mut count: specpdl_ref = SPECPDL_INDEX ();
	 let mut default_coding_system = default_coding_system;   /* C reassigns the argument.  */

	 if SYMBOLP (default_coding_system)
	   { default_coding_system = SYMBOL_NAME (default_coding_system); }
	 specbind (Qcompletion_ignore_case (), Qt ());
	 val = Fcompleting_read (prompt, Vcoding_system_alist, Qnil (),
				 Qt (), Qnil (), Qcoding_system_history (),
				 default_coding_system, Qnil ());
	 val = unbind_to (count, val);
	 return if SCHARS (val) == 0 { Qnil () } else { Fintern (val, Qnil ()) };
       });

DEFUN! ("check-coding-system", Fcheck_coding_system, Scheck_coding_system,
       1, 1, 0,
       doc: "Check validity of CODING-SYSTEM.\nIf valid, return CODING-SYSTEM, else signal a `coding-system-error' error.\nIt is valid if it is nil or a symbol defined as a coding system by the\nfunction `define-coding-system'.",
       (coding_system: Lisp_Object),
       {
	 let mut define_form: Lisp_Object;

	 define_form = Fget (coding_system, Qcoding_system_define_form ());
	 if ! NILP (define_form)
	   {
	     Fput (coding_system, Qcoding_system_define_form (), Qnil ());
	     safe_eval (define_form);
	   }
	 if ! NILP (Fcoding_system_p (coding_system))
	   { return coding_system; }
	 xsignal1 (Qcoding_system_error (), coding_system)
       });

/* Detect how the bytes at SRC of length SRC_BYTES are encoded.  If
   HIGHEST, return the coding system of the highest
   priority among the detected coding systems.  Otherwise return a
   list of detected coding systems sorted by their priorities.  If
   MULTIBYTEP, it is assumed that the bytes are in correct
   multibyte form but contains only ASCII and eight-bit chars.
   Otherwise, the bytes are raw bytes.

   CODING-SYSTEM controls the detection as below:

   If it is nil, detect both text-format and eol-format.  If the
   text-format part of CODING-SYSTEM is already specified
   (e.g. `iso-latin-1'), detect only eol-format.  If the eol-format
   part of CODING-SYSTEM is already specified (e.g. `undecided-unix'),
   detect only text-format.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_coding_system (mut src: *const c_uchar,
					       src_chars: ptrdiff_t, src_bytes: ptrdiff_t,
					       highest: bool, multibytep: bool,
					       mut coding_system: Lisp_Object) -> Lisp_Object
{
  let src_end: *const c_uchar = src.offset (src_bytes);
  let mut attrs: Lisp_Object; let mut eol_type: Lisp_Object;
  let mut val: Lisp_Object = Qnil ();
  let mut coding: coding_system = std::mem::zeroed ();
  let mut id: ptrdiff_t;
  let mut detect_info: coding_detection_info = std::mem::zeroed ();
  let mut base_category: coding_category;
  let mut null_byte_found: bool = false; let mut eight_bit_found: bool = false;

  if NILP (coding_system)
    { coding_system = Qundecided (); }
  setup_coding_system (coding_system, &raw mut coding);
  attrs = CODING_ID_ATTRS (coding.id as hash_idx_t);
  eol_type = CODING_ID_EOL_TYPE (coding.id as hash_idx_t);
  coding_system = CODING_ATTR_BASE_NAME (attrs);

  coding.source = src;
  coding.src_chars = src_chars;
  coding.src_bytes = src_bytes;
  coding_bits_set (&raw mut coding, CODING_SYSTEM_SRC_MULTIBYTE_SHIFT, 1, multibytep as u32);
  coding.consumed = 0;
  coding_mode_set (&raw mut coding, CODING_MODE_LAST_BLOCK);
  coding.head_ascii = 0;

  /* At first, detect text-format if necessary.  */
  base_category = XFIXNUM (CODING_ATTR_CATEGORY (attrs)) as coding_category;
  if base_category == coding_category_undecided
    {
      let mut category: coding_category = 0;
      let mut this: *mut coding_system = std::ptr::null_mut ();
      let mut c: c_int = 0; let mut i: c_int = 0;
      let inhibit_nbd: bool = inhibit_flag (coding.spec.undecided.inhibit_nbd,
					    inhibit_null_byte_detection);
      let inhibit_ied: bool = inhibit_flag (coding.spec.undecided.inhibit_ied,
					    inhibit_iso_escape_detection);
      let prefer_utf_8: bool = coding.spec.undecided.prefer_utf_8;

      /* Skip all ASCII bytes except for a few ISO2022 controls.  */
      while src < src_end
	{
	  c = *src as c_int;
	  if c & 0x80 != 0
	    {
	      eight_bit_found = true;
	      if null_byte_found
		{ break; }
	    }
	  else if c < 0x20
	    {
	      if (c == ISO_CODE_ESC || c == ISO_CODE_SI || c == ISO_CODE_SO)
		  && ! inhibit_ied
		  && detect_info.checked == 0
		{
		  if detect_coding_iso_2022 (&raw mut coding, &raw mut detect_info)
		    {
		      /* We have scanned the whole data.  */
		      if (detect_info.rejected & CATEGORY_MASK_ISO_7_ELSE) == 0
			{
			  /* We didn't find an 8-bit code.  We may
			     have found a null-byte, but it's very
			     rare that a binary file confirm to
			     ISO-2022.  */
			  src = src_end;
			  coding.head_ascii = src.offset_from (coding.source);
			}
		      detect_info.rejected |= ! CATEGORY_MASK_ISO_ESCAPE;
		      break;
		    }
		}
	      else if c == 0 && ! inhibit_nbd
		{
		  null_byte_found = true;
		  if eight_bit_found
		    { break; }
		}
	      if ! eight_bit_found
		{ coding.head_ascii += 1; }
	    }
	  else if ! eight_bit_found
	    { coding.head_ascii += 1; }

	  src = src.offset (1);
	}

      if null_byte_found || eight_bit_found
	  || coding.head_ascii < coding.src_bytes
	  || detect_info.found != 0
	{
	  if coding.head_ascii == coding.src_bytes
	    /* As all bytes are 7-bit, we can ignore non-ISO-2022 codings.  */
	    {
	      i = 0;
	      while i < coding_category_raw_text
		{
		  category = coding_priorities[i as usize];
		  this = &raw mut coding_categories[category as usize];
		  if (detect_info.found & (1 << category)) != 0
		    { break; }
		  i += 1;
		}
	    }
	  else
	    {
	      if null_byte_found
		{
		  detect_info.checked |= ! CATEGORY_MASK_UTF_16;
		  detect_info.rejected |= ! CATEGORY_MASK_UTF_16;
		}
	      else if prefer_utf_8
		       && detect_coding_utf_8 (&raw mut coding, &raw mut detect_info)
		{
		  detect_info.checked |= ! CATEGORY_MASK_UTF_8;
		  detect_info.rejected |= ! CATEGORY_MASK_UTF_8;
		}
	      i = 0;
	      while i < coding_category_raw_text
		{
		  category = coding_priorities[i as usize];
		  this = &raw mut coding_categories[category as usize];

		  if (*this).id < 0
		    {
		      /* No coding system of this category is defined.  */
		      detect_info.rejected |= 1 << category;
		    }
		  else if category >= coding_category_raw_text
		    { i += 1; continue; }
		  else if (detect_info.checked & (1 << category)) != 0
		    {
		      if highest
			  && (detect_info.found & (1 << category)) != 0
			{ break; }
		    }
		  else if ((*this).detector.unwrap ()) (&raw mut coding, &raw mut detect_info)
			   && highest
			   && (detect_info.found & (1 << category)) != 0
		    {
		      if category == coding_category_utf_16_auto
			{
			  if (detect_info.found & CATEGORY_MASK_UTF_16_LE) != 0
			    { category = coding_category_utf_16_le; }
			  else
			    { category = coding_category_utf_16_be; }
			}
		      break;
		    }
		  i += 1;
		}
	    }
	}

      if (detect_info.rejected & CATEGORY_MASK_ANY) == CATEGORY_MASK_ANY
	  || null_byte_found
	{
	  detect_info.found = CATEGORY_MASK_RAW_TEXT;
	  id = CODING_SYSTEM_ID (Qno_conversion ());
	  val = list1i (id as EMACS_INT);
	}
      else if detect_info.rejected == 0 && detect_info.found == 0
	{
	  detect_info.found = CATEGORY_MASK_ANY;
	  id = coding_categories[coding_category_undecided as usize].id;
	  val = list1i (id as EMACS_INT);
	}
      else if highest
	{
	  if detect_info.found != 0
	    {
	      detect_info.found = 1 << category;
	      val = list1i ((*this).id as EMACS_INT);
	    }
	  else
	    {
	      i = 0;
	      while i < coding_category_raw_text
		{
		  if (detect_info.rejected & (1 << coding_priorities[i as usize])) == 0
		    {
		      detect_info.found = 1 << coding_priorities[i as usize];
		      id = coding_categories[coding_priorities[i as usize] as usize].id;
		      val = list1i (id as EMACS_INT);
		      break;
		    }
		  i += 1;
		}
	    }
	}
      else
	{
	  let mask: c_int = detect_info.rejected | detect_info.found;
	  let mut found: c_int = 0;

	  i = coding_category_raw_text - 1;
	  while i >= 0
	    {
	      category = coding_priorities[i as usize];
	      if (mask & (1 << category)) == 0
		{
		  found |= 1 << category;
		  id = coding_categories[category as usize].id;
		  if id >= 0
		    { val = list1i (id as EMACS_INT); }
		}
	      i -= 1;
	    }
	  i = coding_category_raw_text - 1;
	  while i >= 0
	    {
	      category = coding_priorities[i as usize];
	      if (detect_info.found & (1 << category)) != 0
		{
		  id = coding_categories[category as usize].id;
		  val = Fcons (make_fixnum (id as EMACS_INT), val);
		}
	      i -= 1;
	    }
	  detect_info.found |= found;
	}
    }
  else if base_category == coding_category_utf_8_auto
    {
      if detect_coding_utf_8 (&raw mut coding, &raw mut detect_info)
	{
	  let mut this: *mut coding_system;

	  if (detect_info.found & CATEGORY_MASK_UTF_8_SIG) != 0
	    { this = &raw mut coding_categories[coding_category_utf_8_sig as usize]; }
	  else
	    { this = &raw mut coding_categories[coding_category_utf_8_nosig as usize]; }
	  val = list1i ((*this).id as EMACS_INT);
	}
    }
  else if base_category == coding_category_utf_16_auto
    {
      if detect_coding_utf_16 (&raw mut coding, &raw mut detect_info)
	{
	  let mut this: *mut coding_system;

	  if (detect_info.found & CATEGORY_MASK_UTF_16_LE) != 0
	    { this = &raw mut coding_categories[coding_category_utf_16_le as usize]; }
	  else if (detect_info.found & CATEGORY_MASK_UTF_16_BE) != 0
	    { this = &raw mut coding_categories[coding_category_utf_16_be as usize]; }
	  else if (detect_info.rejected & CATEGORY_MASK_UTF_16_LE_NOSIG) != 0
	    { this = &raw mut coding_categories[coding_category_utf_16_be_nosig as usize]; }
	  else
	    { this = &raw mut coding_categories[coding_category_utf_16_le_nosig as usize]; }
	  val = list1i ((*this).id as EMACS_INT);
	}
    }
  else
    {
      detect_info.found = 1 << XFIXNUM (CODING_ATTR_CATEGORY (attrs));
      val = list1i (coding.id as EMACS_INT);
    }

  /* Then, detect eol-format if necessary.  */
  {
    let mut normal_eol: c_int = -1; let mut utf_16_be_eol: c_int = -1;
    let mut utf_16_le_eol: c_int = -1;
    let mut tail: Lisp_Object;

    if VECTORP (eol_type)
      {
	if (detect_info.found & ! CATEGORY_MASK_UTF_16) != 0
	  {
	    if null_byte_found
	      { normal_eol = EOL_SEEN_LF; }
	    else
	      { normal_eol = detect_eol (coding.source, src_bytes,
					 coding_category_raw_text); }
	  }
	if (detect_info.found & (CATEGORY_MASK_UTF_16_BE
				 | CATEGORY_MASK_UTF_16_BE_NOSIG)) != 0
	  { utf_16_be_eol = detect_eol (coding.source, src_bytes,
					coding_category_utf_16_be); }
	if (detect_info.found & (CATEGORY_MASK_UTF_16_LE
				 | CATEGORY_MASK_UTF_16_LE_NOSIG)) != 0
	  { utf_16_le_eol = detect_eol (coding.source, src_bytes,
					coding_category_utf_16_le); }
      }
    else
      {
	if EQ (eol_type, Qunix ())
	  { normal_eol = EOL_SEEN_LF; utf_16_be_eol = EOL_SEEN_LF; utf_16_le_eol = EOL_SEEN_LF; }
	else if EQ (eol_type, Qdos ())
	  { normal_eol = EOL_SEEN_CRLF; utf_16_be_eol = EOL_SEEN_CRLF;
	    utf_16_le_eol = EOL_SEEN_CRLF; }
	else
	  { normal_eol = EOL_SEEN_CR; utf_16_be_eol = EOL_SEEN_CR; utf_16_le_eol = EOL_SEEN_CR; }
      }

    tail = val;
    while CONSP (tail)
      {
	let mut category: coding_category;
	let mut this_eol: c_int;

	id = XFIXNUM (XCAR (tail));
	attrs = CODING_ID_ATTRS (id as hash_idx_t);
	category = XFIXNUM (CODING_ATTR_CATEGORY (attrs)) as coding_category;
	eol_type = CODING_ID_EOL_TYPE (id as hash_idx_t);
	if VECTORP (eol_type)
	  {
	    if category == coding_category_utf_16_be
		|| category == coding_category_utf_16_be_nosig
	      { this_eol = utf_16_be_eol; }
	    else if category == coding_category_utf_16_le
		     || category == coding_category_utf_16_le_nosig
	      { this_eol = utf_16_le_eol; }
	    else
	      { this_eol = normal_eol; }

	    if this_eol == EOL_SEEN_LF
	      { XSETCAR (tail, AREF (eol_type, 0)); }
	    else if this_eol == EOL_SEEN_CRLF
	      { XSETCAR (tail, AREF (eol_type, 1)); }
	    else if this_eol == EOL_SEEN_CR
	      { XSETCAR (tail, AREF (eol_type, 2)); }
	    else
	      { XSETCAR (tail, CODING_ID_NAME (id as hash_idx_t)); }
	  }
	else
	  { XSETCAR (tail, CODING_ID_NAME (id as hash_idx_t)); }
	tail = XCDR (tail);
      }
  }

  return if highest { if CONSP (val) { XCAR (val) } else { Qnil () } } else { val };
}

DEFUN! ("detect-coding-region", Fdetect_coding_region, Sdetect_coding_region,
       2, 3, 0,
       doc: "Detect coding system of the text in the region between START and END.\nReturn a list of possible coding systems ordered by priority.\nThe coding systems to try and their priorities follows what\nthe function `coding-system-priority-list' (which see) returns.\n\nIf only ASCII characters are found (except for such ISO-2022 control\ncharacters as ESC), it returns a list of single element `undecided'\nor its subsidiary coding system according to a detected end-of-line\nformat.\n\nIf optional argument HIGHEST is non-nil, return the coding system of\nhighest priority.",
       (start: Lisp_Object, end: Lisp_Object, highest: Lisp_Object),
       {
	 let mut start = start; let mut end = end;   /* C passes their addresses.  */
	 let mut from: ptrdiff_t; let mut to: ptrdiff_t;
	 let from_byte: ptrdiff_t; let to_byte: ptrdiff_t;

	 validate_region (&raw mut start, &raw mut end);
	 from = XFIXNUM (start); to = XFIXNUM (end);
	 from_byte = CHAR_TO_BYTE (from);
	 to_byte = CHAR_TO_BYTE (to);

	 if from < GPT! () && to >= GPT! ()
	   { move_gap_both (to, to_byte); }

	 return detect_coding_system (BYTE_POS_ADDR (from_byte) as *const c_uchar,
				      to - from, to_byte - from_byte,
				      ! NILP (highest),
				      ! NILP (BVAR! ((current_buffer! ())
						     , enable_multibyte_characters_)),
				      Qnil ());
       });

DEFUN! ("detect-coding-string", Fdetect_coding_string, Sdetect_coding_string,
       1, 2, 0,
       doc: "Detect coding system of the text in STRING.\nReturn a list of possible coding systems ordered by priority.\nThe coding systems to try and their priorities follows what\nthe function `coding-system-priority-list' (which see) returns.\n\nIf only ASCII characters are found (except for such ISO-2022 control\ncharacters as ESC), it returns a list of single element `undecided'\nor its subsidiary coding system according to a detected end-of-line\nformat.\n\nIf optional argument HIGHEST is non-nil, return the coding system of\nhighest priority.",
       (string: Lisp_Object, highest: Lisp_Object),
       {
	 CHECK_STRING (string);

	 return detect_coding_system (SDATA (string) as *const c_uchar,
				      SCHARS (string), SBYTES (string),
				      ! NILP (highest), STRING_MULTIBYTE (string),
				      Qnil ());
       });


unsafe fn char_encodable_p (mut c: c_int, attrs: Lisp_Object) -> bool
{
  let mut tail: Lisp_Object;
  let mut charset: *mut charset;
  let mut translation_table: Lisp_Object;

  /* `CHAR_CHARSET_P` (charset.h 519-529): the charset.rs mirror's copy compares the
     boolean `CHARSET_FAST_MAP_REF!` against `0`, which does not compile (`expected bool,
     found integer`); this function is the reference's second user, so the faithful
     transcription is defined here and shadows the mirror inside it (a use-site
     workaround -- the mirror is not edited, and the defect is reported in
     docs/ports/coding.md).  */

  translation_table = CODING_ATTR_TRANS_TBL (attrs);
  if ! NILP (translation_table)
    { c = translate_char (translation_table, c); }
  tail = CODING_ATTR_CHARSET_LIST (attrs);
  while CONSP (tail)
    {
      charset = CHARSET_FROM_ID! (XFIXNUM (XCAR (tail)));
      if CHAR_CHARSET_P! (c, charset)
	{ break; }
      tail = XCDR (tail);
    }
  return ! NILP (tail);
}


/* Return a list of coding systems that safely encode the text between
   START and END.  If EXCLUDE is non-nil, it is a list of coding
   systems not to check.  The returned list doesn't contain any such
   coding systems.  In any case, if the text contains only ASCII or is
   unibyte, return t.  */

DEFUN! ("find-coding-systems-region-internal",
       Ffind_coding_systems_region_internal,
       Sfind_coding_systems_region_internal, 2, 3, 0,
       doc: "Internal use only.",
       (start: Lisp_Object, end: Lisp_Object, exclude: Lisp_Object),
       {
	 let mut coding_attrs_list: Lisp_Object; let mut safe_codings: Lisp_Object;
	 let mut start_byte: ptrdiff_t; let mut end_byte: ptrdiff_t;
	 let mut p: *const c_uchar; let mut pbeg: *const c_uchar;
	 let mut pend: *const c_uchar;
	 let mut c: c_int;
	 let mut tail: Lisp_Object; let mut elt: Lisp_Object; let mut work_table: Lisp_Object;

	 if STRINGP (start)
	   {
	     if ! STRING_MULTIBYTE (start)
		 || SCHARS (start) == SBYTES (start)
	       { return Qt (); }
	     start_byte = 0;
	     end_byte = SBYTES (start);
	   }
	 else
	   {
	     let s: EMACS_INT = fix_position (start);
	     let e: EMACS_INT = fix_position (end);
	     if ! ((BEG as ptrdiff_t) <= s && s <= e && e <= Z! ())
	       { args_out_of_range (start, end); }
	     if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	       { return Qt (); }
	     start_byte = CHAR_TO_BYTE (s);
	     end_byte = CHAR_TO_BYTE (e);
	     if e - s == end_byte - start_byte
	       { return Qt (); }

	     if s < GPT! () && GPT! () < e
	       {
		 if GPT! () - s < e - GPT! ()
		   { move_gap_both (s, start_byte); }
		 else
		   { move_gap_both (e, end_byte); }
	       }
	   }

	 coding_attrs_list = Qnil ();
	 tail = Vcoding_system_list;
	 while CONSP (tail)
	   {
	     if NILP (exclude)
		 || NILP (Fmemq (XCAR (tail), exclude))
	       {
		 let attrs: Lisp_Object;

		 attrs = AREF (CODING_SYSTEM_SPEC (XCAR (tail)), 0);
		 if EQ (XCAR (tail), CODING_ATTR_BASE_NAME (attrs))
		   {
		     ASET (attrs, coding_attr_trans_tbl as isize,
			   get_translation_table (attrs, true, std::ptr::null_mut ()));
		     coding_attrs_list = Fcons (attrs, coding_attrs_list);
		   }
	       }
	     tail = XCDR (tail);
	   }

	 if STRINGP (start)
	   { p = SDATA (start) as *const c_uchar; pbeg = p; }
	 else
	   { p = BYTE_POS_ADDR (start_byte) as *const c_uchar; pbeg = p; }
	 pend = p.offset (end_byte - start_byte);

	 while p < pend && ASCII_CHAR_P (*p as isize) { p = p.offset (1); }
	 while p < pend && ASCII_CHAR_P (*(pend.offset (-1)) as isize) { pend = pend.offset (-1); }

	 work_table = Fmake_char_table (Qnil (), Qnil ());
	 while p < pend
	   {
	     if ASCII_CHAR_P (*p as isize)
	       { p = p.offset (1); }
	     else
	       {
		 c = string_char_advance (&raw mut p);
		 if ! NILP (char_table_ref (work_table, c))
		   /* This character was already checked.  Ignore it.  */
		   { continue; }

		 charset_map_loaded = false;
		 tail = coding_attrs_list;
		 while CONSP (tail)
		   {
		     elt = XCAR (tail);
		     if NILP (elt)
		       { tail = XCDR (tail); }
		     else if char_encodable_p (c, elt)
		       { tail = XCDR (tail); }
		     else if CONSP (XCDR (tail))
		       {
			 XSETCAR (tail, XCAR (XCDR (tail)));
			 XSETCDR (tail, XCDR (XCDR (tail)));
		       }
		     else
		       {
			 XSETCAR (tail, Qnil ());
			 tail = XCDR (tail);
		       }
		   }
		 if charset_map_loaded
		   {
		     let p_offset: ptrdiff_t = p.offset_from (pbeg);
		     let pend_offset: ptrdiff_t = pend.offset_from (pbeg);

		     if STRINGP (start)
		       { pbeg = SDATA (start) as *const c_uchar; }
		     else
		       { pbeg = BYTE_POS_ADDR (start_byte) as *const c_uchar; }
		     p = pbeg.offset (p_offset);
		     pend = pbeg.offset (pend_offset);
		   }
		 char_table_set (work_table, c, Qt ());
	       }
	   }

	 safe_codings = list2 (Qraw_text (), Qno_conversion ());
	 tail = coding_attrs_list;
	 while CONSP (tail)
	   {
	     if ! NILP (XCAR (tail))
	       { safe_codings = Fcons (CODING_ATTR_BASE_NAME (XCAR (tail)), safe_codings); }
	     tail = XCDR (tail);
	   }

	 return safe_codings;
       });

DEFUN! ("unencodable-char-position", Funencodable_char_position,
       Sunencodable_char_position, 3, 5, 0,
       doc: "Return position of first un-encodable character in a region.\nSTART and END specify the region and CODING-SYSTEM specifies the\nencoding to check.  Return nil if CODING-SYSTEM does encode the region.\n\nIf optional 4th argument COUNT is non-nil, it specifies at most how\nmany un-encodable characters to search.  In this case, the value is a\nlist of positions.\n\nIf optional 5th argument STRING is non-nil, it is a string to search\nfor un-encodable characters.  In that case, START and END are indexes\nto the string and treated as in `substring'.",
       (start: Lisp_Object, end: Lisp_Object, coding_system: Lisp_Object,
	count: Lisp_Object, string: Lisp_Object),
       {
	 let mut n: EMACS_INT;
	 let mut start = start; let mut end = end;   /* C passes their addresses.  */
	 let mut coding: coding_system = std::mem::zeroed ();
	 let mut attrs: Lisp_Object; let mut charset_list: Lisp_Object;
	 let mut translation_table: Lisp_Object;
	 let mut positions: Lisp_Object;
	 let mut from: ptrdiff_t = 0; let mut to: ptrdiff_t = 0;   /* C's `validate_subarray` writes them.  */
	 let mut p: *const c_uchar; let mut stop: *const c_uchar;
	 let mut pend: *const c_uchar;
	 let mut ascii_compatible: bool;

	 setup_coding_system (Fcheck_coding_system (coding_system), &raw mut coding);
	 attrs = CODING_ID_ATTRS (coding.id as hash_idx_t);
	 if EQ (CODING_ATTR_TYPE (attrs), Qraw_text ())
	   { return Qnil (); }
	 ascii_compatible = ! NILP (CODING_ATTR_ASCII_COMPAT (attrs));
	 charset_list = CODING_ATTR_CHARSET_LIST (attrs);
	 translation_table = get_translation_table (attrs, true, std::ptr::null_mut ());

	 if NILP (string)
	   {
	     validate_region (&raw mut start, &raw mut end);
	     from = XFIXNUM (start);
	     to = XFIXNUM (end);
	     if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
		 || (ascii_compatible
		     && (to - from) == (CHAR_TO_BYTE (to) - (CHAR_TO_BYTE (from))))
	       { return Qnil (); }
	     p = CHAR_POS_ADDR (from) as *const c_uchar;
	     pend = CHAR_POS_ADDR (to) as *const c_uchar;
	     if from < GPT! () && to >= GPT! ()
	       { stop = GPT_ADDR! () as *const c_uchar; }
	     else
	       { stop = pend; }
	   }
	 else
	   {
	     CHECK_STRING (string);
	     validate_subarray (string, start, end, SCHARS (string), &raw mut from, &raw mut to);
	     if ! STRING_MULTIBYTE (string)
	       { return Qnil (); }
	     p = (SDATA (string) as *const c_uchar).offset (string_char_to_byte (string, from));
	     pend = (SDATA (string) as *const c_uchar).offset (string_char_to_byte (string, to));
	     stop = pend;
	     if ascii_compatible && (to - from) == pend.offset_from (p)
	       { return Qnil (); }
	   }

	 if NILP (count)
	   { n = 1; }
	 else
	   {
	     CHECK_FIXNAT (count);
	     n = XFIXNUM (count);
	   }

	 positions = Qnil ();
	 charset_map_loaded = false;
	 loop
	   {
	     let mut c: c_int;

	     if ascii_compatible
	       {
		 while p < stop && ASCII_CHAR_P (*p as isize)
		   { p = p.offset (1); from += 1; }
	       }
	     if p >= stop
	       {
		 if p >= pend
		   { break; }
		 stop = pend;
		 p = GAP_END_ADDR! () as *const c_uchar;
	       }

	     c = string_char_advance (&raw mut p);
	     if ! (ASCII_CHAR_P (c as isize) && ascii_compatible)
		 && char_charset (translate_char (translation_table, c),
				  charset_list, std::ptr::null_mut ()).is_null ()
	       {
		 positions = Fcons (make_fixnum (from as EMACS_INT), positions);
		 n -= 1;
		 if n == 0
		   { break; }
	       }

	     from += 1;
	     if charset_map_loaded && NILP (string)
	       {
		 p = CHAR_POS_ADDR (from) as *const c_uchar;
		 pend = CHAR_POS_ADDR (to) as *const c_uchar;
		 if from < GPT! () && to >= GPT! ()
		   { stop = GPT_ADDR! () as *const c_uchar; }
		 else
		   { stop = pend; }
		 charset_map_loaded = false;
	       }
	   }

	 return if NILP (count) { Fcar (positions) } else { Fnreverse (positions) };
       });


DEFUN! ("check-coding-systems-region", Fcheck_coding_systems_region,
       Scheck_coding_systems_region, 3, 3, 0,
       doc: "Check if text between START and END is encodable by CODING-SYSTEM-LIST.\n\nSTART and END are buffer positions specifying the region.\nCODING-SYSTEM-LIST is a list of coding systems to check.\n\nIf all coding systems in CODING-SYSTEM-LIST can encode the region, the\nfunction returns nil.\n\nIf some of the coding systems cannot encode the whole region, value is\nan alist, each element of which has the form (CODING-SYSTEM POS1 POS2 ...),\nwhich means that CODING-SYSTEM cannot encode the text at buffer positions\nPOS1, POS2, ...\n\nSTART may be a string.  In that case, check if the string is\nencodable, and the value contains character indices into the string\ninstead of buffer positions.  END is ignored in this case.\n\nIf the current buffer (or START if it is a string) is unibyte, the value\nis nil.",
       (start: Lisp_Object, end: Lisp_Object, coding_system_list: Lisp_Object),
       {
	 let mut list: Lisp_Object;
	 let mut start_byte: ptrdiff_t; let mut end_byte: ptrdiff_t;
	 let mut pos: ptrdiff_t;
	 let mut p: *const c_uchar; let mut pbeg: *const c_uchar;
	 let mut pend: *const c_uchar;
	 let mut c: c_int;
	 let mut tail: Lisp_Object; let mut elt: Lisp_Object; let mut attrs: Lisp_Object;

	 if STRINGP (start)
	   {
	     if ! STRING_MULTIBYTE (start)
		 || SCHARS (start) == SBYTES (start)
	       { return Qnil (); }
	     start_byte = 0;
	     end_byte = SBYTES (start);
	     pos = 0;
	   }
	 else
	   {
	     let s: EMACS_INT = fix_position (start);
	     let e: EMACS_INT = fix_position (end);
	     if ! ((BEG as ptrdiff_t) <= s && s <= e && e <= Z! ())
	       { args_out_of_range (start, end); }
	     if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	       { return Qnil (); }
	     start_byte = CHAR_TO_BYTE (s);
	     end_byte = CHAR_TO_BYTE (e);
	     if e - s == end_byte - start_byte
	       { return Qnil (); }

	     if s < GPT! () && GPT! () < e
	       {
		 if GPT! () - s < e - GPT! ()
		   { move_gap_both (s, start_byte); }
		 else
		   { move_gap_both (e, end_byte); }
	       }
	     pos = s;
	   }

	 list = Qnil ();
	 tail = coding_system_list;
	 while CONSP (tail)
	   {
	     elt = XCAR (tail);
	     {
	       let spec: Lisp_Object = CODING_SYSTEM_SPEC (elt);
	       if ! VECTORP (spec)
		 { xsignal1 (Qcoding_system_error (), elt); }
	       attrs = AREF (spec, 0);
	       ASET (attrs, coding_attr_trans_tbl as isize,
		     get_translation_table (attrs, true, std::ptr::null_mut ()));
	       list = Fcons (list2 (elt, attrs), list);
	     }
	     tail = XCDR (tail);
	   }

	 if STRINGP (start)
	   { p = SDATA (start) as *const c_uchar; pbeg = p; }
	 else
	   { p = BYTE_POS_ADDR (start_byte) as *const c_uchar; pbeg = p; }
	 pend = p.offset (end_byte - start_byte);

	 while p < pend && ASCII_CHAR_P (*p as isize) { p = p.offset (1); pos += 1; }
	 while p < pend && ASCII_CHAR_P (*(pend.offset (-1)) as isize) { pend = pend.offset (-1); }

	 while p < pend
	   {
	     if ASCII_CHAR_P (*p as isize)
	       { p = p.offset (1); }
	     else
	       {
		 c = string_char_advance (&raw mut p);

		 charset_map_loaded = false;
		 tail = list;
		 while CONSP (tail)
		   {
		     elt = XCDR (XCAR (tail));
		     if ! char_encodable_p (c, XCAR (elt))
		       { XSETCDR (elt, Fcons (make_fixnum (pos as EMACS_INT), XCDR (elt))); }
		     tail = XCDR (tail);
		   }
		 if charset_map_loaded
		   {
		     let p_offset: ptrdiff_t = p.offset_from (pbeg);
		     let pend_offset: ptrdiff_t = pend.offset_from (pbeg);

		     if STRINGP (start)
		       { pbeg = SDATA (start) as *const c_uchar; }
		     else
		       { pbeg = BYTE_POS_ADDR (start_byte) as *const c_uchar; }
		     p = pbeg.offset (p_offset);
		     pend = pbeg.offset (pend_offset);
		   }
	       }
	     pos += 1;
	   }

	 tail = list;
	 list = Qnil ();
	 while CONSP (tail)
	   {
	     elt = XCAR (tail);
	     if CONSP (XCDR (XCDR (elt)))
	       { list = Fcons (Fcons (XCAR (elt), Fnreverse (XCDR (XCDR (elt)))),
			       list); }
	     tail = XCDR (tail);
	   }

	 return list;
       });


unsafe fn code_convert_region (mut start: Lisp_Object, mut end: Lisp_Object,
			       mut coding_system: Lisp_Object, mut dst_object: Lisp_Object,
			       encodep: bool, norecord: bool) -> Lisp_Object
{
  let mut coding: coding_system = std::mem::zeroed ();
  let mut from: ptrdiff_t; let mut from_byte: ptrdiff_t;
  let mut to: ptrdiff_t; let mut to_byte: ptrdiff_t;
  let mut src_object: Lisp_Object;

  if NILP (coding_system)
    { coding_system = Qno_conversion (); }
  else
    { CHECK_CODING_SYSTEM (coding_system); }
  src_object = Fcurrent_buffer ();
  if NILP (dst_object)
    { dst_object = src_object; }
  else if ! EQ (dst_object, Qt ())
    { CHECK_BUFFER (dst_object); }

  validate_region (&raw mut start, &raw mut end);
  from = XFIXNAT (start);
  from_byte = CHAR_TO_BYTE (from);
  to = XFIXNAT (end);
  to_byte = CHAR_TO_BYTE (to);

  setup_coding_system (coding_system, &raw mut coding);
  coding_mode_set (&raw mut coding, CODING_MODE_LAST_BLOCK);

  if BUFFERP (dst_object) && ! BASE_EQ (dst_object, src_object)
    {
      let buf: *mut buffer = XBUFFER (dst_object);
      let buf_pt: ptrdiff_t = BUF_PT (buf);

      invalidate_buffer_caches (buf as *mut c_void, buf_pt, buf_pt);
    }

  if encodep
    { encode_coding_object (&raw mut coding, src_object, from, from_byte, to, to_byte,
			    dst_object); }
  else
    { decode_coding_object (&raw mut coding, src_object, from, from_byte, to, to_byte,
			    dst_object); }
  if ! norecord
    { Vlast_coding_system_used = CODING_ID_NAME (coding.id as hash_idx_t); }

  return if BUFFERP (dst_object)
	   { make_fixnum (coding.produced_char as EMACS_INT) }
	 else { coding.dst_object };
}


DEFUN! ("decode-coding-region", Fdecode_coding_region, Sdecode_coding_region,
       3, 4, "r\nzCoding system: ",
       doc: "Decode the current region using the specified coding system.\nInteractively, prompt for the coding system to decode the region, and\nreplace the region with the decoded text.\n\n\"Decoding\" means transforming bytes into readable text (characters).\nIf, for instance, you have a region that contains data that represents\nthe two bytes #xc2 #xa9, after calling this function with the utf-8\ncoding system, the region will contain the single\ncharacter ?\\N{COPYRIGHT SIGN}.\n\nWhen called from a program, takes four arguments:\n\tSTART, END, CODING-SYSTEM, and DESTINATION.\nSTART and END are buffer positions.\n\nOptional 4th arguments DESTINATION specifies where the decoded text goes.\nIf nil, the region between START and END is replaced by the decoded text.\nIf buffer, the decoded text is inserted in that buffer after point (point\ndoes not move).  If that buffer is unibyte, it receives the individual\nbytes of the internal representation of the decoded text.\nIn those cases, the length of the decoded text is returned.\nIf DESTINATION is t, the decoded text is returned.\n\nThis function sets `last-coding-system-used' to the precise coding system\nused (which may be different from CODING-SYSTEM if CODING-SYSTEM is\nnot fully specified.)",
       (start: Lisp_Object, end: Lisp_Object, coding_system: Lisp_Object,
	destination: Lisp_Object),
       {
	 return code_convert_region (start, end, coding_system, destination, false, false);
       });

DEFUN! ("encode-coding-region", Fencode_coding_region, Sencode_coding_region,
       3, 4, "r\nzCoding system: ",
       doc: "Encode the current region using the specified coding system.\nInteractively, prompt for the coding system to encode the region, and\nreplace the region with the bytes that are the result of the encoding.\n\nWhat's meant by \"encoding\" is transforming textual data (characters)\ninto bytes.  If, for instance, you have a region that contains the\nsingle character ?\\N{COPYRIGHT SIGN}, after calling this function with\nthe utf-8 coding system, the data in the region will represent the two\nbytes #xc2 #xa9.\n\nWhen called from a program, takes four arguments:\n        START, END, CODING-SYSTEM and DESTINATION.\nSTART and END are buffer positions.\n\nOptional 4th argument DESTINATION specifies where the encoded text goes.\nIf nil, the region between START and END is replaced by the encoded text.\nIf buffer, the encoded text is inserted in that buffer after point (point\ndoes not move).\nIn those cases, the length of the encoded text is returned.\nIf DESTINATION is t, the encoded text is returned.\n\nThis function sets `last-coding-system-used' to the precise coding system\nused (which may be different from CODING-SYSTEM if CODING-SYSTEM is\nnot fully specified.)",
       (start: Lisp_Object, end: Lisp_Object, coding_system: Lisp_Object,
	destination: Lisp_Object),
       {
	 return code_convert_region (start, end, coding_system, destination, true, false);
       });

/* Whether STRING only contains chars in the 0..127 range.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn string_ascii_p (string: Lisp_Object) -> bool
{
  let nbytes: ptrdiff_t = SBYTES (string);
  let mut i: ptrdiff_t = 0;
  while i < nbytes
    {
      if SREF (string, i as usize) > 127
	{ return false; }
      i += 1;
    }
  return true;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn code_convert_string (string: Lisp_Object,
					      mut coding_system: Lisp_Object,
					      mut dst_object: Lisp_Object,
					      encodep: bool, nocopy: bool,
					      norecord: bool) -> Lisp_Object
{
  let mut coding: coding_system = std::mem::zeroed ();
  let mut chars: ptrdiff_t; let mut bytes: ptrdiff_t;

  CHECK_STRING (string);
  if NILP (coding_system)
    {
      if ! norecord
	{ Vlast_coding_system_used = Qno_conversion (); }
      if NILP (dst_object)
	{ return if nocopy { string } else { Fcopy_sequence (string) }; }
    }

  if NILP (coding_system)
    { coding_system = Qno_conversion (); }
  else
    { CHECK_CODING_SYSTEM (coding_system); }
  if NILP (dst_object)
    { dst_object = Qt (); }
  else if ! EQ (dst_object, Qt ())
    { CHECK_BUFFER (dst_object); }

  setup_coding_system (coding_system, &raw mut coding);
  coding_mode_set (&raw mut coding, CODING_MODE_LAST_BLOCK);
  chars = SCHARS (string);
  bytes = SBYTES (string);

  if EQ (dst_object, Qt ())
    {
      /* Fast path for ASCII-only input and an ASCII-compatible coding:
         act as identity if no EOL conversion is needed.  */
      let attrs: Lisp_Object = CODING_ID_ATTRS (coding.id as hash_idx_t);
      if ! NILP (CODING_ATTR_ASCII_COMPAT (attrs))
	  && (if STRING_MULTIBYTE (string)
		{ chars == bytes } else { string_ascii_p (string) })
	  && (EQ (CODING_ID_EOL_TYPE (coding.id as hash_idx_t), Qunix ())
	      || inhibit_eol_conversion
	      || memchr (SDATA (string) as *const c_void,
			 if encodep { '\n' as c_int } else { '\r' as c_int },
			 bytes as usize).is_null ())
	{
	  if ! norecord
	    { Vlast_coding_system_used = coding_system; }
	  return if nocopy
		   { string }
		 else if encodep
		   { make_unibyte_string (SSDATA (string), bytes) }
		 else { make_multibyte_string (SSDATA (string), bytes, bytes) };
	}
    }
  else if BUFFERP (dst_object)
    {
      let buf: *mut buffer = XBUFFER (dst_object);
      let buf_pt: ptrdiff_t = BUF_PT (buf);

      invalidate_buffer_caches (buf as *mut c_void, buf_pt, buf_pt);
    }

  if encodep
    { encode_coding_object (&raw mut coding, string, 0, 0, chars, bytes, dst_object); }
  else
    { decode_coding_object (&raw mut coding, string, 0, 0, chars, bytes, dst_object); }
  if ! norecord
    { Vlast_coding_system_used = CODING_ID_NAME (coding.id as hash_idx_t); }

  return if BUFFERP (dst_object)
	   { make_fixnum (coding.produced_char as EMACS_INT) }
	 else { coding.dst_object };
}


/* Encode or decode STRING according to CODING_SYSTEM.
   Do not set Vlast_coding_system_used.  */

/* `femacs/src/include/coding.rs` declares `code_convert_string_norecord` at its crate root
   (coding.h 691-713), so the definition lives in a glob-imported module (the
   `decode_coding_gap_def` device; reported as a mirror gap in docs/ports/coding.md).  */
pub mod code_convert_string_norecord_def {
  use super::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn code_convert_string_norecord (string: Lisp_Object,
						      coding_system: Lisp_Object,
						      encodep: bool) -> Lisp_Object
{
  return code_convert_string (string, coding_system, Qt (), encodep, false, true);
}
}
use code_convert_string_norecord_def::*;


/* Return the gap address of BUFFER.  If the gap size is less than
   NBYTES, enlarge the gap in advance.  */

unsafe fn get_buffer_gap_address (buffer: Lisp_Object, nbytes: ptrdiff_t) -> *mut c_uchar
{
  let buf: *mut buffer = XBUFFER (buffer);

  if BUF_GPT! (buf) != BUF_PT (buf)
    {
      let oldb: *mut buffer = (current_buffer! ());

      set_current_buffer! (buf);
      move_gap_both (PT! (), PT_BYTE! ());
      set_current_buffer! (oldb);
    }
  if BUF_GAP_SIZE! (buf) < nbytes
    { make_gap_1 (buf, nbytes); }
  return BUF_GPT_ADDR (buf) as *mut c_uchar;
}

/* Return a pointer to the byte sequence for C, and its byte length in
   LEN.  This function is used to get a byte sequence for HANDLE_8_BIT
   and HANDLE_OVER_UNI arguments of encode_string_utf_8 and
   decode_string_utf_8 when those arguments are given by
   characters.  */

/* C's function-local statics of `get_char_bytes` (`chars`, `bytes`, `nbytes`,
   `last_index`).  The port has no function statics and a crate-root item of these names
   would collide with the mirrors' own parameters and locals (E0530: a `let` binding or a
   parameter may not shadow a `static`), so they live in this private module and are
   glob-imported into the function (PORTING.md 0.2: `static` keeps C's name, no symbol).  */
mod get_char_bytes_caches {
  use super::*;

  pub static mut chars: [c_int; 2] = [0; 2];
  pub static mut bytes: [[c_uchar; 6]; 2] = [[0; 6]; 2];
  pub static mut nbytes: [c_int; 2] = [0; 2];
  pub static mut last_index: c_int = 0;
}

unsafe fn get_char_bytes (c: c_int, len: *mut c_int) -> *mut c_uchar
{
  use get_char_bytes_caches::*;

  /* Use two caches, since encode/decode_string_utf_8 are called
     repeatedly with the same values for HANDLE_8_BIT and
     HANDLE_OVER_UNI arguments.  */
  if chars[last_index as usize] == c
    {
      *len = nbytes[last_index as usize];
      return bytes[last_index as usize].as_mut_ptr ();
    }
  if chars[(1 - last_index) as usize] == c
    {
      *len = nbytes[(1 - last_index) as usize];
      return bytes[(1 - last_index) as usize].as_mut_ptr ();
    }
  last_index = 1 - last_index;
  chars[last_index as usize] = c;
  let n: c_int = CHAR_STRING (c, bytes[last_index as usize].as_mut_ptr ());
  nbytes[last_index as usize] = n;
  *len = n;
  return bytes[last_index as usize].as_mut_ptr ();
}


unsafe fn convert_string_nocopy (string: Lisp_Object, coding_system: Lisp_Object,
				 encodep: bool) -> Lisp_Object
{
  return code_convert_string (string, coding_system, Qt (), encodep, true, true);
}

/* Encode or decode a file name, to or from a unibyte string suitable
   for passing to C library functions.  */
/* `femacs/src/include/coding.rs` declares `decode_file_name` at its crate root
   (coding.h 699), so the definition lives in a glob-imported module (the
   `decode_coding_gap_def` device; reported as a mirror gap in docs/ports/coding.md).  */
pub mod decode_file_name_def {
  use super::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_file_name (fname: Lisp_Object) -> Lisp_Object
{
  /* C 10432-10438 is `#ifdef WINDOWSNT`, dead in the frozen configuration; the `#else`
     arm below is live (docs/ports/coding.md).  */
  if ! NILP (Vfile_name_coding_system)
    { return convert_string_nocopy (fname, Vfile_name_coding_system, false); }
  else if ! NILP (Vdefault_file_name_coding_system)
    { return convert_string_nocopy (fname, Vdefault_file_name_coding_system, false); }
  else
    { return fname; }
}
}
use decode_file_name_def::*;

unsafe fn encode_file_name_1 (fname: Lisp_Object) -> Lisp_Object
{
  /* This is especially important during bootstrap and dumping, when
     file-name encoding is not yet known, and therefore any non-ASCII
     file names are unibyte strings, and could only be thrashed if we
     try to encode them.  */
  if ! STRING_MULTIBYTE (fname)
    { return fname; }
  /* C 10458-10464 is `#ifdef WINDOWSNT`, dead in the frozen configuration; the `#else`
     arm below is live (docs/ports/coding.md).  */
  if ! NILP (Vfile_name_coding_system)
    { return convert_string_nocopy (fname, Vfile_name_coding_system, true); }
  else if ! NILP (Vdefault_file_name_coding_system)
    { return convert_string_nocopy (fname, Vdefault_file_name_coding_system, true); }
  else
    { return fname; }
}

/* `femacs/src/include/coding.rs` declares `encode_file_name` at its crate root
   (coding.h 698), so the definition lives in a glob-imported module (the
   `decode_coding_gap_def` device; reported as a mirror gap in docs/ports/coding.md).  */
pub mod encode_file_name_def {
  use super::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn encode_file_name (fname: Lisp_Object) -> Lisp_Object
{
  let encoded: Lisp_Object = encode_file_name_1 (fname);
  /* No system accepts NUL bytes in filenames.  Allowing them can
     cause subtle bugs because the system would silently use a
     different filename than expected.  Perform this check after
     encoding to not miss NUL bytes introduced through encoding.  */
  CHECK_STRING_NULL_BYTES (encoded);
  return encoded;
}
}
use encode_file_name_def::*;

DEFUN! ("decode-coding-string", Fdecode_coding_string, Sdecode_coding_string,
       2, 4, 0,
       doc: "Decode STRING which is encoded in CODING-SYSTEM, and return the result.\n\nOptional third arg NOCOPY non-nil means it is OK to return STRING itself\nif the decoding operation is trivial.\n\nOptional fourth arg BUFFER non-nil means that the decoded text is\ninserted in that buffer after point (point does not move).  In this\ncase, the return value is the length of the decoded text.  If that\nbuffer is unibyte, it receives the individual bytes of the internal\nrepresentation of the decoded text.\n\nThis function sets `last-coding-system-used' to the precise coding system\nused (which may be different from CODING-SYSTEM if CODING-SYSTEM is\nnot fully specified.)  The function does not change the match data.",
       (string: Lisp_Object, coding_system: Lisp_Object, nocopy: Lisp_Object,
	buffer: Lisp_Object),
       {
	 return code_convert_string (string, coding_system, buffer,
				     false, ! NILP (nocopy), false);
       });

DEFUN! ("encode-coding-string", Fencode_coding_string, Sencode_coding_string,
       2, 4, 0,
       doc: "Encode STRING to CODING-SYSTEM, and return the result.\n\nOptional third arg NOCOPY non-nil means it is OK to return STRING\nitself if the encoding operation is trivial.\n\nOptional fourth arg BUFFER non-nil means that the encoded text is\ninserted in that buffer after point (point does not move).  In this\ncase, the return value is the length of the encoded text.\n\nThis function sets `last-coding-system-used' to the precise coding system\nused (which may be different from CODING-SYSTEM if CODING-SYSTEM is\nnot fully specified.)  The function does not change the match data.",
       (string: Lisp_Object, coding_system: Lisp_Object, nocopy: Lisp_Object,
	buffer: Lisp_Object),
       {
	 return code_convert_string (string, coding_system, buffer,
				     true, ! NILP (nocopy), false);
       });


DEFUN! ("decode-sjis-char", Fdecode_sjis_char, Sdecode_sjis_char, 1, 1, 0,
       doc: "Decode a Japanese character which has CODE in shift_jis encoding.\nReturn the corresponding character.",
       (code: Lisp_Object),
       {
	 let mut spec: Lisp_Object; let mut attrs: Lisp_Object; let mut val: Lisp_Object;
	 let charset_roman: *mut charset; let charset_kanji: *mut charset;
	 let charset_kana: *mut charset; let mut charset: *mut charset;
	 let ch: EMACS_INT;
	 let mut c: c_int;

	 CHECK_FIXNAT (code);
	 ch = XFIXNAT (code);
	 spec = CHECK_CODING_SYSTEM_GET_SPEC (Vsjis_coding_system);
	 attrs = AREF (spec, 0);

	 if ASCII_CHAR_P (ch) 
	     && ! NILP (CODING_ATTR_ASCII_COMPAT (attrs))
	   { return code; }

	 val = CODING_ATTR_CHARSET_LIST (attrs);
	 charset_roman = CHARSET_FROM_ID! (XFIXNUM (XCAR (val))); val = XCDR (val);
	 charset_kana = CHARSET_FROM_ID! (XFIXNUM (XCAR (val))); val = XCDR (val);
	 charset_kanji = CHARSET_FROM_ID! (XFIXNUM (XCAR (val)));

	 if ch <= 0x7F
	   {
	     c = ch as c_int;
	     charset = charset_roman;
	   }
	 else if ch >= 0xA0 && ch < 0xDF
	   {
	     c = (ch - 0x80) as c_int;
	     charset = charset_kana;
	   }
	 else
	   {
	     let c1: EMACS_INT = ch >> 8;
	     let c2: c_int = (ch & 0xFF) as c_int;

	     if c1 < 0x81 || (c1 > 0x9F && c1 < 0xE0) || c1 > 0xEF
		 || c2 < 0x40 || c2 == 0x7F || c2 > 0xFC
	       { error (c"Invalid code: %ld".as_ptr (), ch); }
	     c = ch as c_int;
	     c = SJIS_TO_JIS (c);
	     charset = charset_kanji;
	   }
	 c = DECODE_CHAR! (charset, c as c_uint);
	 if c < 0
	   { error (c"Invalid code: %ld".as_ptr (), ch); }
	 return make_fixnum (c as EMACS_INT);
       });


DEFUN! ("encode-sjis-char", Fencode_sjis_char, Sencode_sjis_char, 1, 1, 0,
       doc: "Encode a Japanese character CH to shift_jis encoding.\nReturn the corresponding code in SJIS.",
       (ch: Lisp_Object),
       {
	 let mut spec: Lisp_Object; let mut attrs: Lisp_Object;
	 let mut charset_list: Lisp_Object;
	 let c: c_int;
	 let mut charset: *mut charset;
	 let mut code: c_uint = 0;

	 CHECK_CHARACTER (ch);
	 c = XFIXNAT (ch) as c_int;
	 spec = CHECK_CODING_SYSTEM_GET_SPEC (Vsjis_coding_system);
	 attrs = AREF (spec, 0);

	 if ASCII_CHAR_P (c as isize)
	     && ! NILP (CODING_ATTR_ASCII_COMPAT (attrs))
	   { return ch; }

	 charset_list = CODING_ATTR_CHARSET_LIST (attrs);
	 charset = char_charset (c, charset_list, &raw mut code);
	 if code == CHARSET_INVALID_CODE! (charset)
	   { error (c"Can't encode by shift_jis encoding: %c".as_ptr (), c); }
	 code = JIS_TO_SJIS (code as c_int) as c_uint;

	 return make_fixnum (code as EMACS_INT);
       });

DEFUN! ("decode-big5-char", Fdecode_big5_char, Sdecode_big5_char, 1, 1, 0,
       doc: "Decode a Big5 character which has CODE in BIG5 coding system.\nReturn the corresponding character.",
       (code: Lisp_Object),
       {
	 let mut spec: Lisp_Object; let mut attrs: Lisp_Object; let mut val: Lisp_Object;
	 let charset_roman: *mut charset; let charset_big5: *mut charset;
	 let mut charset: *mut charset;
	 let ch: EMACS_INT;
	 let mut c: c_int;

	 CHECK_FIXNAT (code);
	 ch = XFIXNAT (code);
	 spec = CHECK_CODING_SYSTEM_GET_SPEC (Vbig5_coding_system);
	 attrs = AREF (spec, 0);

	 if ASCII_CHAR_P (ch)
	     && ! NILP (CODING_ATTR_ASCII_COMPAT (attrs))
	   { return code; }

	 val = CODING_ATTR_CHARSET_LIST (attrs);
	 charset_roman = CHARSET_FROM_ID! (XFIXNUM (XCAR (val))); val = XCDR (val);
	 charset_big5 = CHARSET_FROM_ID! (XFIXNUM (XCAR (val)));

	 if ch <= 0x7F
	   {
	     c = ch as c_int;
	     charset = charset_roman;
	   }
	 else
	   {
	     let b1: EMACS_INT = ch >> 8;
	     let b2: c_int = (ch & 0x7F) as c_int;
	     if b1 < 0xA1 || b1 > 0xFE
		 || b2 < 0x40 || (b2 > 0x7E && b2 < 0xA1) || b2 > 0xFE
	       { error (c"Invalid code: %ld".as_ptr (), ch); }
	     c = ch as c_int;
	     charset = charset_big5;
	   }
	 c = DECODE_CHAR! (charset, c as c_uint);
	 if c < 0
	   { error (c"Invalid code: %ld".as_ptr (), ch); }
	 return make_fixnum (c as EMACS_INT);
       });

DEFUN! ("encode-big5-char", Fencode_big5_char, Sencode_big5_char, 1, 1, 0,
       doc: "Encode the Big5 character CH to BIG5 coding system.\nReturn the corresponding character code in Big5.",
       (ch: Lisp_Object),
       {
	 let mut spec: Lisp_Object; let mut attrs: Lisp_Object;
	 let mut charset_list: Lisp_Object;
	 let mut charset: *mut charset;
	 let c: c_int;
	 let mut code: c_uint = 0;

	 CHECK_CHARACTER (ch);
	 c = XFIXNAT (ch) as c_int;
	 spec = CHECK_CODING_SYSTEM_GET_SPEC (Vbig5_coding_system);
	 attrs = AREF (spec, 0);
	 if ASCII_CHAR_P (c as isize)
	     && ! NILP (CODING_ATTR_ASCII_COMPAT (attrs))
	   { return ch; }

	 charset_list = CODING_ATTR_CHARSET_LIST (attrs);
	 charset = char_charset (c, charset_list, &raw mut code);
	 if code == CHARSET_INVALID_CODE! (charset)
	   { error (c"Can't encode by Big5 encoding: %c".as_ptr (), c); }

	 return make_fixnum (code as EMACS_INT);
       });


DEFUN! ("set-terminal-coding-system-internal", Fset_terminal_coding_system_internal,
       Sset_terminal_coding_system_internal, 1, 2, 0,
       doc: "Internal use only.",
       (coding_system: Lisp_Object, terminal: Lisp_Object),
       {
	 let term: *mut terminal = decode_live_terminal (terminal);
	 let terminal_coding: *mut coding_system = TERMINAL_TERMINAL_CODING (term) as *mut coding_system;
	 CHECK_SYMBOL (coding_system);
	 setup_coding_system (Fcheck_coding_system (coding_system), terminal_coding);
	 /* We had better not send unsafe characters to terminal.  */
	 coding_mode_set (terminal_coding, CODING_MODE_SAFE_ENCODING);
	 /* Character composition should be disabled.  */
	 coding_bits_set (terminal_coding, CODING_SYSTEM_COMMON_FLAGS_SHIFT,
			  CODING_SYSTEM_COMMON_FLAGS_MASK,
			  (coding_bits_get (terminal_coding, CODING_SYSTEM_COMMON_FLAGS_SHIFT,
					    CODING_SYSTEM_COMMON_FLAGS_MASK)
			   & ! CODING_ANNOTATE_COMPOSITION_MASK));
	 coding_bits_set (terminal_coding, CODING_SYSTEM_SRC_MULTIBYTE_SHIFT, 1, 1);
	 coding_bits_set (terminal_coding, CODING_SYSTEM_DST_MULTIBYTE_SHIFT, 1, 0);
	 tset_charset_list
	   (term, if (coding_bits_get (terminal_coding, CODING_SYSTEM_COMMON_FLAGS_SHIFT,
				       CODING_SYSTEM_COMMON_FLAGS_MASK)
		      & CODING_REQUIRE_ENCODING_MASK) != 0
		  { coding_charset_list (terminal_coding) }
		  else { list1i (charset_ascii as EMACS_INT) });
	 return Qnil ();
       });

DEFUN! ("set-safe-terminal-coding-system-internal",
       Fset_safe_terminal_coding_system_internal,
       Sset_safe_terminal_coding_system_internal, 1, 1, 0,
       doc: "Internal use only.",
       (coding_system: Lisp_Object),
       {
	 CHECK_SYMBOL (coding_system);
	 setup_coding_system (Fcheck_coding_system (coding_system),
			      &raw mut safe_terminal_coding);
	 /* Character composition should be disabled.  */
	 coding_bits_set (&raw mut safe_terminal_coding, CODING_SYSTEM_COMMON_FLAGS_SHIFT,
			  CODING_SYSTEM_COMMON_FLAGS_MASK,
			  (coding_bits_get (&raw mut safe_terminal_coding,
					    CODING_SYSTEM_COMMON_FLAGS_SHIFT,
					    CODING_SYSTEM_COMMON_FLAGS_MASK)
			   & ! CODING_ANNOTATE_COMPOSITION_MASK));
	 coding_bits_set (&raw mut safe_terminal_coding, CODING_SYSTEM_SRC_MULTIBYTE_SHIFT,
			  1, 1);
	 coding_bits_set (&raw mut safe_terminal_coding, CODING_SYSTEM_DST_MULTIBYTE_SHIFT,
			  1, 0);
	 return Qnil ();
       });

DEFUN! ("terminal-coding-system", Fterminal_coding_system,
       Sterminal_coding_system, 0, 1, 0,
       doc: "Return coding system specified for terminal output on the given terminal.\nTERMINAL may be a terminal object, a frame, or nil for the selected\nframe's terminal device.",
       (terminal: Lisp_Object),
       {
	 let terminal_coding: *mut coding_system
	   = TERMINAL_TERMINAL_CODING (decode_live_terminal (terminal)) as *mut coding_system;
	 let coding_system: Lisp_Object = CODING_ID_NAME ((*terminal_coding).id as hash_idx_t);

	 /* For backward compatibility, return nil if it is `undecided'.  */
	 return if ! EQ (coding_system, Qundecided ()) { coding_system } else { Qnil () };
       });

DEFUN! ("set-keyboard-coding-system-internal", Fset_keyboard_coding_system_internal,
       Sset_keyboard_coding_system_internal, 1, 2, 0,
       doc: "Internal use only.",
       (coding_system: Lisp_Object, terminal: Lisp_Object),
       {
	 let t: *mut terminal = decode_live_terminal (terminal);
	 let mut coding_system = coding_system;   /* C reassigns the argument.  */
	 CHECK_SYMBOL (coding_system);
	 if NILP (coding_system)
	   { coding_system = Qno_conversion (); }
	 else
	   { Fcheck_coding_system (coding_system); }
	 setup_coding_system (coding_system, TERMINAL_KEYBOARD_CODING (t) as *mut coding_system);
	 /* Character composition should be disabled.  */
	 coding_bits_set (TERMINAL_KEYBOARD_CODING (t) as *mut coding_system, CODING_SYSTEM_COMMON_FLAGS_SHIFT,
			  CODING_SYSTEM_COMMON_FLAGS_MASK,
			  (coding_bits_get (TERMINAL_KEYBOARD_CODING (t) as *mut coding_system,
					    CODING_SYSTEM_COMMON_FLAGS_SHIFT,
					    CODING_SYSTEM_COMMON_FLAGS_MASK)
			   & ! CODING_ANNOTATE_COMPOSITION_MASK));
	 return Qnil ();
       });

DEFUN! ("keyboard-coding-system",
       Fkeyboard_coding_system, Skeyboard_coding_system, 0, 1, 0,
       doc: "Return coding system specified for decoding keyboard input.",
       (terminal: Lisp_Object),
       {
	 return CODING_ID_NAME ((*(TERMINAL_KEYBOARD_CODING
				    (decode_live_terminal (terminal)) as *mut coding_system)).id as hash_idx_t);
       });


DEFUN! ("find-operation-coding-system", Ffind_operation_coding_system,
       Sfind_operation_coding_system,  1, MANY, 0,
       doc: "Choose a coding system for an operation based on the target name.\nThe value names a pair of coding systems: (DECODING-SYSTEM . ENCODING-SYSTEM).\nDECODING-SYSTEM is the coding system to use for decoding\n(in case OPERATION does decoding), and ENCODING-SYSTEM is the coding system\nfor encoding (in case OPERATION does encoding).\n\nThe first argument OPERATION specifies an I/O primitive:\n  For file I/O, `insert-file-contents' or `write-region'.\n  For process I/O, `call-process', `call-process-region', or `start-process'.\n  For network I/O, `open-network-stream'.\n\nThe remaining arguments should be the same arguments that were passed\nto the primitive.  Depending on which primitive, one of those arguments\nis selected as the TARGET.  For example, if OPERATION does file I/O,\nwhichever argument specifies the file name is TARGET.\n\nTARGET has a meaning which depends on OPERATION:\n  For file I/O, TARGET is a file name (except for the special case below).\n  For process I/O, TARGET is a process name.\n  For network I/O, TARGET is a service name or a port number.\n\nThis function looks up what is specified for TARGET in\n`file-coding-system-alist', `process-coding-system-alist',\nor `network-coding-system-alist' depending on OPERATION.\nThey may specify a coding system, a cons of coding systems,\nor a function symbol to call.\nIn the last case, we call the function with one argument,\nwhich is a list of all the arguments given to this function.\nIf the function can't decide a coding system, it can return\n`undecided' so that the normal code-detection is performed.\n\nIf OPERATION is `insert-file-contents', the argument corresponding to\nTARGET may be a cons (FILENAME . BUFFER).  In that case, FILENAME is a\nfile name to look up, and BUFFER is a buffer that contains the file's\ncontents (not yet decoded).  If `file-coding-system-alist' specifies a\nfunction to call for FILENAME, that function should examine the\ncontents of BUFFER instead of reading the file.\n\nusage: (find-operation-coding-system OPERATION ARGUMENTS...)",
       (nargs: isize, args: *mut Lisp_Object),
       {
	 let mut operation: Lisp_Object; let mut target_idx: Lisp_Object;
	 let mut target: Lisp_Object; let mut val: Lisp_Object;
	 let mut chain: Lisp_Object;

	 if nargs < 2
	   { error (c"Too few arguments".as_ptr ()); }
	 operation = *args.offset (0);
	 if ! SYMBOLP (operation)
	     || { target_idx = Fget (operation, Qtarget_idx ());
		  ! FIXNATP (target_idx) }
	   { error (c"Invalid first argument".as_ptr ()); }
	 if nargs <= 1 + XFIXNAT (target_idx)
	   { error (c"Too few arguments for operation `%s'".as_ptr (),
		    SDATA (SYMBOL_NAME (operation))); }
	 target = *args.offset (XFIXNAT (target_idx) + 1);
	 if ! (STRINGP (target)
	       || (EQ (operation, Qinsert_file_contents ()) && CONSP (target)
		   && STRINGP (XCAR (target)) && BUFFERP (XCDR (target)))
	       || (EQ (operation, Qopen_network_stream ())
		   && (FIXNUMP (target) || EQ (target, Qt ()))))
	   { error (c"Invalid argument %ld of operation `%s'".as_ptr (),
		    XFIXNAT (target_idx) + 1, SDATA (SYMBOL_NAME (operation))); }
	 if CONSP (target)
	   { target = XCAR (target); }

	 chain = if EQ (operation, Qinsert_file_contents ())
		    || EQ (operation, Qwrite_region ())
		   { Vfile_coding_system_alist }
		 else if EQ (operation, Qopen_network_stream ())
		   { Vnetwork_coding_system_alist }
		 else { Vprocess_coding_system_alist };
	 if NILP (chain)
	   { return Qnil (); }

	 while CONSP (chain)
	   {
	     let mut elt: Lisp_Object;

	     elt = XCAR (chain);
	     if CONSP (elt)
		 && ((STRINGP (target)
		      && STRINGP (XCAR (elt))
		      && fast_string_match (XCAR (elt), target) >= 0)
		     || (FIXNUMP (target) && BASE_EQ (target, XCAR (elt))))
	       {
		 val = XCDR (elt);
		 /* Here, if VAL is both a valid coding system and a valid
                    function symbol, we return VAL as a coding system.  */
		 if CONSP (val)
		   { return val; }
		 if ! SYMBOLP (val)
		   { return Qnil (); }
		 if ! NILP (Fcoding_system_p (val))
		   { return Fcons (val, val); }
		 if ! NILP (Ffboundp (val))
		   {
		     /* We use calln rather than safe_calln
			so as to get bug reports about functions called here
			which don't handle the current interface.  */
		     val = calln! (val, Flist (nargs, args));
		     if CONSP (val)
		       { return val; }
		     if SYMBOLP (val) && ! NILP (Fcoding_system_p (val))
		       { return Fcons (val, val); }
		   }
		 return Qnil ();
	       }
	     chain = XCDR (chain);
	   }
	 return Qnil ();
       });

DEFUN! ("set-coding-system-priority", Fset_coding_system_priority,
       Sset_coding_system_priority, 0, MANY, 0,
       doc: "Assign higher priority to the coding systems given as arguments.\nIf multiple coding systems belong to the same category,\nall but the first one are ignored.\n\nusage: (set-coding-system-priority &rest coding-systems)",
       (nargs: isize, args: *mut Lisp_Object),
       {
	 let mut i: ptrdiff_t; let mut j: ptrdiff_t;
	 let mut changed: [bool; coding_category_max as usize]
	   = [false; coding_category_max as usize];
	 let mut priorities: [coding_category; coding_category_max as usize]
	   = [0; coding_category_max as usize];

	 i = 0; j = 0;
	 while i < nargs
	   {
	     let mut category: coding_category;
	     let mut spec: Lisp_Object; let mut attrs: Lisp_Object;

	     spec = CHECK_CODING_SYSTEM_GET_SPEC (*args.offset (i));
	     attrs = AREF (spec, 0);
	     category = XFIXNUM (CODING_ATTR_CATEGORY (attrs)) as coding_category;
	     if changed[category as usize]
	       /* Ignore this coding system because a coding system of the
		  same category already had a higher priority.  */
	       { i += 1; continue; }
	     changed[category as usize] = true;
	     priorities[j as usize] = category; j += 1;
	     if (*&raw const coding_categories[category as usize]).id >= 0
		 && ! EQ (*args.offset (i),
			  CODING_ID_NAME ((*&raw const coding_categories
					   [category as usize]).id as hash_idx_t))
	       { setup_coding_system (*args.offset (i),
				      &raw mut coding_categories[category as usize]); }
	     Fset (AREF (Vcoding_category_table, category as isize), *args.offset (i));
	     i += 1;
	   }

	 /* Now we have decided top J priorities.  Reflect the order of the
	    original priorities to the remaining priorities.  */

	 i = j; j = 0;
	 while i < coding_category_max as ptrdiff_t
	   {
	     while j < coding_category_max as ptrdiff_t
		   && changed[coding_priorities[j as usize] as usize]
	       { j += 1; }
	     if j == coding_category_max as ptrdiff_t
	       { emacs_abort (); }
	     priorities[i as usize] = coding_priorities[j as usize];
	     i += 1; j += 1;
	   }

	 memcpy (coding_priorities.as_mut_ptr () as *mut c_void,
		 priorities.as_ptr () as *const c_void,
		 std::mem::size_of_val (&priorities));

	 /* Update `coding-category-list'.  */
	 Vcoding_category_list = Qnil ();
	 i = coding_category_max as ptrdiff_t;
	 while i > 0
	   {
	     i -= 1;
	     Vcoding_category_list
	       = Fcons (AREF (Vcoding_category_table, priorities[i as usize] as isize),
			Vcoding_category_list);
	   }

	 return Qnil ();
       });

DEFUN! ("coding-system-priority-list", Fcoding_system_priority_list,
       Scoding_system_priority_list, 0, 1, 0,
       doc: "Return a list of coding systems ordered by their priorities.\nThe list contains a subset of coding systems; i.e. coding systems\nassigned to each coding category (see `coding-category-list').\n\nHIGHESTP non-nil means just return the highest priority one.",
       (highestp: Lisp_Object),
       {
	 let mut i: c_int;
	 let mut val: Lisp_Object;

	 i = 0; val = Qnil ();
	 while i < coding_category_max
	   {
	     let category: coding_category = coding_priorities[i as usize];
	     let id: c_int = coding_categories[category as usize].id as c_int;
	     let mut attrs: Lisp_Object;

	     if id < 0
	       { i += 1; continue; }
	     attrs = CODING_ID_ATTRS (id as hash_idx_t);
	     if ! NILP (highestp)
	       { return CODING_ATTR_BASE_NAME (attrs); }
	     val = Fcons (CODING_ATTR_BASE_NAME (attrs), val);
	     i += 1;
	   }
	 return Fnreverse (val);
       });

/* C's function-local `static char const suffixes[][8]`; the port has no function statics,
   so it is a file-scope `static` with C's own name and C's bytes (PORTING.md 0.2).  */
static suffixes: [[u8; 8]; 3] = [*b"-unix\0\0\0", *b"-dos\0\0\0\0", *b"-mac\0\0\0\0"];

unsafe fn make_subsidiaries (base: Lisp_Object) -> Lisp_Object
{
  let base_name_len: ptrdiff_t = SBYTES (SYMBOL_NAME (base));
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let buf: *mut c_char = SAFE_ALLOCA! (sa_avail, base_name_len + 6) as *mut c_char;

  memcpy (buf as *mut c_void, SDATA (SYMBOL_NAME (base)) as *const c_void,
	  base_name_len as size_t);
  let subsidiaries: Lisp_Object = make_nil_vector (3);
  let mut i: c_int = 0;
  while i < 3
    {
      strcpy (buf.offset (base_name_len), suffixes[i as usize].as_ptr () as *const c_char);
      ASET (subsidiaries, i as isize, intern (buf));
      i += 1;
    }
  SAFE_FREE! (sa_count);
  return subsidiaries;
}


/* Encode STRING by the coding system utf-8-unix.

   This function is optimized for speed when the input string is
   already a valid sequence of Unicode codepoints in the internal
   representation, i.e. there are neither 8-bit raw bytes nor
   characters beyond the Unicode range in the string's contents.

   Ignore any :pre-write-conversion and :encode-translation-table
   properties.

   Assume that arguments have values as described below.
   The validity must be enforced and ensured by the caller.

   STRING is a multibyte string or an ASCII-only unibyte string.

   BUFFER is a unibyte buffer or Qnil.

   If BUFFER is a unibyte buffer, insert the encoded result
   after point of the buffer, and return the number of
   inserted characters.  The caller should have made BUFFER ready for
   modifying in advance (e.g., by calling invalidate_buffer_caches).

   If BUFFER is nil, return a unibyte string from the encoded result.

   If NOCOPY is non-zero, and if STRING contains only Unicode
   characters (i.e., the encoding does not change the byte sequence),
   return STRING even if it is multibyte.  WARNING: This will return a
   _multibyte_ string, something that callers might not expect, especially
   if STRING is not pure-ASCII; only use NOCOPY non-zero if the caller
   will only use the byte sequence of the encoded result accessed by
   SDATA or SSDATA, and the original STRING will _not_ be modified after
   the encoding.  When in doubt, always pass NOCOPY as zero.  You _have_
   been warned!

   HANDLE-8-BIT and HANDLE-OVER-UNI specify how to handle a non-Unicode
   character in STRING.  The former is for an eight-bit character (represented
   by a 2-byte overlong sequence in a multibyte STRING).  The latter is
   for a codepoint beyond the end of the Unicode range (a character whose
   code is greater than the maximum Unicode character 0x10FFFF, represented
   by a 4 or 5-byte sequence in a multibyte STRING).

   If these two arguments are unibyte strings (typically
   "\357\277\275", the UTF-8 sequence for the Unicode REPLACEMENT
   CHARACTER #xFFFD), encode a non-Unicode character into that
   unibyte sequence.

   If the two arguments are characters, encode a non-Unicode
   character as the respective argument characters.

   If they are Qignored, skip a non-Unicode character.

   If HANDLE-8-BIT is Qt, encode eight-bit characters into single bytes
   of the same value, like the usual Emacs encoding does.

   If HANDLE-OVER-UNI is Qt, encode characters beyond the Unicode
   range into the same 4 or 5-byte sequence as used by Emacs
   internally, like the usual Emacs encoding does.

   If the two arguments are Qnil, return Qnil if STRING has a
   non-Unicode character.  This allows the caller to signal an error
   if such input strings are not allowed.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn encode_string_utf_8 (string: Lisp_Object, buffer: Lisp_Object,
					       nocopy: bool, handle_8_bit: Lisp_Object,
					       handle_over_uni: Lisp_Object) -> Lisp_Object
{
  let mut nchars: ptrdiff_t = SCHARS (string); let nbytes: ptrdiff_t = SBYTES (string);
  if NILP (buffer) && nchars == nbytes && nocopy
    /* STRING contains only ASCII characters.  */
    { return string; }

  let mut num_8_bit: ptrdiff_t = 0;   /* number of eight-bit chars in STRING */
  /* The following two vars are counted only if handle_over_uni is not Qt.  */
  let mut num_over_4: ptrdiff_t = 0; /* number of 4-byte non-Unicode chars in STRING */
  let mut num_over_5: ptrdiff_t = 0; /* number of 5-byte non-Unicode chars in STRING */
  let mut outbytes: ptrdiff_t = 0;	     /* number of bytes of decoding result (C leaves it to the first scan; PORTING.md 1.5) */
  let mut p: *mut c_uchar = SDATA (string) as *mut c_uchar;
  let pend: *mut c_uchar = p.offset (nbytes);
  let mut src: *mut c_uchar = std::ptr::null_mut ();
  let mut dst: *mut c_uchar = std::ptr::null_mut ();
  let mut replace_8_bit: *mut c_uchar = std::ptr::null_mut ();
  let mut replace_over_uni: *mut c_uchar = std::ptr::null_mut ();
  let mut replace_8_bit_len: c_int = 0; let mut replace_over_uni_len: c_int = 0;
  let mut val: Lisp_Object = Qnil ();		/* the return value (PORTING.md 1.5) */

  /* Scan bytes in STRING twice.  The first scan is to count non-Unicode
     characters, and the second scan is to encode STRING.  If the
     encoding is trivial (no need of changing the byte sequence),
     the second scan is avoided.  */
  let mut scan_count: c_int = 0;
  while scan_count < 2
    {
      while p < pend
	{
	  if nchars == p.offset_from (pend)
	    /* There is no multibyte character remaining.  */
	    { break; }

	  let c: c_int = *p as c_int;
	  let mut len: c_int = BYTES_BY_CHAR_HEAD (c);

	  nchars -= 1;
	  if len == 1
	      || len == 3
	      || (if len == 2 { ! CHAR_BYTE8_HEAD_P (c) }
		  else { EQ (handle_over_uni, Qt ())
			 || (len == 4
			     && STRING_CHAR (p) <= MAX_UNICODE_CHAR) })
	    {
	      p = p.offset (len as isize);
	      continue;
	    }

	  /* A character to change the byte sequence on encoding was
	     found.  A rare case.  */
	  if len == 2
	    {
	      /* Handle an eight-bit character by handle_8_bit.  */
	      if scan_count == 0
		{
		  if NILP (handle_8_bit)
		    { return Qnil (); }
		  num_8_bit += 1;
		}
	      else
		{
		  if src < p
		    {
		      memcpy (dst as *mut c_void, src as *const c_void,
			      p.offset_from (src) as size_t);
		      dst = dst.offset (p.offset_from (src));
		    }
		  if replace_8_bit_len > 0
		    {
		      memcpy (dst as *mut c_void, replace_8_bit as *const c_void,
			      replace_8_bit_len as size_t);
		      dst = dst.offset (replace_8_bit_len as isize);
		    }
		  else if EQ (handle_8_bit, Qt ())
		    {
		      let char8: c_int = STRING_CHAR (p);
		      *dst = CHAR_TO_BYTE8 (char8) as c_uchar; dst = dst.offset (1);
		    }
		}
	    }
	  else				/* len == 4 or 5 */
	    {
	      /* Handle an over-unicode character by handle_over_uni.  */
	      if scan_count == 0
		{
		  if NILP (handle_over_uni)
		    { return Qnil (); }
		  if len == 4
		    { num_over_4 += 1; }
		  else
		    { num_over_5 += 1; }
		}
	      else
		{
		  if src < p
		    {
		      memcpy (dst as *mut c_void, src as *const c_void,
			      p.offset_from (src) as size_t);
		      dst = dst.offset (p.offset_from (src));
		    }
		  if replace_over_uni_len > 0
		    {
		      memcpy (dst as *mut c_void, replace_over_uni as *const c_void,
			      replace_over_uni_len as size_t);
		      dst = dst.offset (replace_over_uni_len as isize);
		    }
		}
	    }
	  p = p.offset (len as isize);
	  src = p;
	}

      if scan_count == 0
	{
	  /* End of the first scan.  */
	  outbytes = nbytes;
	  if num_8_bit == 0
	      && (num_over_4 + num_over_5 == 0 || EQ (handle_over_uni, Qt ()))
	    {
	      /* We can break the loop because there is no need of
		 changing the byte sequence.  This is the typical
		 case.  */
	      scan_count = 1;
	    }
	  else
	    {
	      /* Prepare for handling non-Unicode characters during
		 the next scan.  */
	      if num_8_bit > 0
		{
		  if CHARACTERP (handle_8_bit)
		    { replace_8_bit = get_char_bytes (XFIXNUM (handle_8_bit) as c_int,
						      &raw mut replace_8_bit_len); }
		  else if STRINGP (handle_8_bit)
		    {
		      replace_8_bit = SDATA (handle_8_bit) as *mut c_uchar;
		      replace_8_bit_len = SBYTES (handle_8_bit) as c_int;
		    }
		  if ! replace_8_bit.is_null ()
		    { outbytes += (replace_8_bit_len - 2) as ptrdiff_t * num_8_bit; }
		  else if EQ (handle_8_bit, Qignored ())
		    { outbytes -= 2 * num_8_bit; }
		  else if EQ (handle_8_bit, Qt ())
		    { outbytes -= num_8_bit; }
		  else
		    { return Qnil (); }
		}
	      if num_over_4 + num_over_5 > 0
		{
		  if CHARACTERP (handle_over_uni)
		    { replace_over_uni = get_char_bytes (XFIXNUM (handle_over_uni) as c_int,
							 &raw mut replace_over_uni_len); }
		  else if STRINGP (handle_over_uni)
		    {
		      replace_over_uni = SDATA (handle_over_uni) as *mut c_uchar;
		      replace_over_uni_len = SBYTES (handle_over_uni) as c_int;
		    }
		  if num_over_4 > 0
		    {
		      if ! replace_over_uni.is_null ()
			{ outbytes += (replace_over_uni_len - 4) as ptrdiff_t * num_over_4; }
		      else if EQ (handle_over_uni, Qignored ())
			{ outbytes -= 4 * num_over_4; }
		      else if ! EQ (handle_over_uni, Qt ())
			{ return Qnil (); }
		    }
		  if num_over_5 > 0
		    {
		      if ! replace_over_uni.is_null ()
			{ outbytes += (replace_over_uni_len - 5) as ptrdiff_t * num_over_5; }
		      else if EQ (handle_over_uni, Qignored ())
			{ outbytes -= 5 * num_over_5; }
		      else if ! EQ (handle_over_uni, Qt ())
			{ return Qnil (); }
		    }
		}
	    }

	  /* Prepare return value and space to store the encoded bytes.  */
	  if BUFFERP (buffer)
	    {
	      val = make_fixnum (outbytes as EMACS_INT);
	      dst = get_buffer_gap_address (buffer, nbytes);
	    }
	  else
	    {
	      if nocopy && (num_8_bit + num_over_4 + num_over_5) == 0
		{ return string; }
	      val = make_uninit_string (outbytes as EMACS_INT);
	      dst = SDATA (val) as *mut c_uchar;
	    }
	  p = SDATA (string) as *mut c_uchar; src = p;
	}
      scan_count += 1;
    }

  if src < pend
    { memcpy (dst as *mut c_void, src as *const c_void, pend.offset_from (src) as size_t); }
  if BUFFERP (buffer)
    {
      let oldb: *mut buffer = (current_buffer! ());

      set_current_buffer! (XBUFFER (buffer));
      insert_from_gap (outbytes, outbytes, false, false);
      set_current_buffer! (oldb);
    }
  return val;
}


/* Decode input string by the coding system utf-8-unix.

   This function is optimized for speed when the input string is
   already a valid UTF-8 sequence, i.e. there are neither 8-bit raw
   bytes nor any UTF-8 sequences longer than 4 bytes in the string's
   contents.

   Ignore any :post-read-conversion and :decode-translation-table
   properties.

   Assume that arguments have values as described below.
   The validity must be enforced and ensured by the caller.

   STRING is a unibyte string, an ASCII-only multibyte string, or Qnil.
   If STRING is Qnil, the input is a C string pointed by STR whose
   length in bytes is in STR_LEN.

   BUFFER is a multibyte buffer or Qnil.
   If BUFFER is a multibyte buffer, insert the decoding result of
   Unicode characters after point of the buffer, and return the number
   of inserted characters.  The caller should have made BUFFER ready
   for modifying in advance (e.g., by calling invalidate_buffer_caches).

   If BUFFER is Qnil, return a multibyte string from the decoded result.

   NOCOPY non-zero means it is OK to return the input STRING if it
   contains only ASCII characters or only valid UTF-8 sequences of 2
   to 4 bytes.  WARNING: This will return a _unibyte_ string, something
   that callers might not expect, especially if STRING is not
   pure-ASCII; only use NOCOPY non-zero if the caller will only use
   the byte sequence of the decoded result accessed via SDATA or
   SSDATA, and if the original STRING will _not_ be modified after the
   decoding.  When in doubt, always pass NOCOPY as zero.  You _have_
   been warned!

   If STRING is Qnil, and the original string is passed via STR, NOCOPY
   is ignored.

   HANDLE-8-BIT and HANDLE-OVER-UNI specify how to handle a invalid
   byte sequence.  The former is for a 1-byte invalid sequence that
   violates the fundamental UTF-8 encoding rules.  The latter is for a
   4 or 5-byte overlong sequences that Emacs internally uses to
   represent characters beyond the Unicode range (characters whose
   codepoints are greater than #x10FFFF).  Note that this function does
   not in general treat such overlong UTF-8 sequences as invalid.

   If these two arguments are strings (typically a 1-char string of
   the Unicode REPLACEMENT CHARACTER #xFFFD), decode an invalid byte
   sequence into that string.  They must be multibyte strings if they
   contain a non-ASCII character.

   If the two arguments are characters, decode an invalid byte
   sequence into the corresponding multibyte representation of the
   respective character.

   If they are Qignored, skip an invalid byte sequence without
   producing anything in the decoded string.

   If HANDLE-8-BIT is Qt, decode a 1-byte invalid sequence into the
   corresponding eight-bit multibyte representation, like the usual
   Emacs decoding does.

   If HANDLE-OVER-UNI is Qt, decode a 4 or 5-byte overlong sequence
   that follows Emacs's internal representation for a character beyond
   Unicode range into the corresponding character, like the usual
   Emacs decoding does.

   If the two arguments are Qnil, return Qnil if the input string has
   raw bytes or overlong sequences.  This allows the caller to signal
   an error if such inputs are not allowed.  */

/* `femacs/src/coding.rs`'s `coding_fwd` module declares `decode_string_utf_8` (coding.h
   695-697), and the definition shadows that glob import (PORTING.md 0.1-10).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_string_utf_8 (string: Lisp_Object, str: *const c_char,
					      str_len: ptrdiff_t, buffer: Lisp_Object,
					      nocopy: bool, handle_8_bit: Lisp_Object,
					      handle_over_uni: Lisp_Object) -> Lisp_Object
{
  /* This is like BYTES_BY_CHAR_HEAD, but it is assured that C >= 0x80
     and it returns 0 for an invalid sequence.  */
  /* C's function-scope `#define UTF_8_SEQUENCE_LENGTH(c)`; the port defines the same
     macro in the same position (PORTING.md 2.1).  */
  macro_rules! UTF_8_SEQUENCE_LENGTH {
    ($c:expr) => {
      (if $c < 0xC2 { 0 }
       else if $c < 0xE0 { 2 }
       else if $c < 0xF0 { 3 }
       else if $c < 0xF8 { 4 }
       else if $c == 0xF8 { 5 }
       else { 0 })
    };
  }

  let nbytes: ptrdiff_t = if STRINGP (string) { SBYTES (string) } else { str_len };
  let mut p: *mut c_uchar
    = if STRINGP (string) { SDATA (string) as *mut c_uchar } else { str as *mut c_uchar };
  let str_orig: *mut c_uchar = p;
  let pend: *mut c_uchar = p.offset (nbytes);
  let mut num_8_bit: ptrdiff_t = 0;   /* number of invalid 1-byte sequences */
  let mut num_over_4: ptrdiff_t = 0;  /* number of invalid 4-byte sequences */
  let mut num_over_5: ptrdiff_t = 0;  /* number of invalid 5-byte sequences */
  let mut outbytes: ptrdiff_t = nbytes;	/* number of decoded bytes */
  let mut outchars: ptrdiff_t = 0;    /* number of decoded characters */
  let mut src: *mut c_uchar = std::ptr::null_mut ();
  let mut dst: *mut c_uchar = std::ptr::null_mut ();
  let mut change_byte_sequence: bool = false;

  /* Scan input bytes twice.  The first scan is to count invalid
     sequences, and the second scan is to decode input.  If the
     decoding is trivial (no need of changing the byte sequence),
     the second scan is avoided.  */
  while p < pend
    {
      src = p;
      /* Try short cut for an ASCII-only case.  */
      while p < pend && *p < 0x80 { p = p.offset (1); }
      outchars += p.offset_from (src);
      if p == pend
	{ break; }
      let c: c_int = *p as c_int;
      outchars += 1;
      let mut len: c_int = UTF_8_SEQUENCE_LENGTH! (c);
      /* len == 0, 2, 3, 4, 5.  */
      if UTF_8_EXTRA_OCTET_P! (*p.offset (1))
	  && (len == 2
	      || (UTF_8_EXTRA_OCTET_P! (*p.offset (2))
		  && (len == 3
		      || (UTF_8_EXTRA_OCTET_P! (*p.offset (3))
			  && len == 4
			  && STRING_CHAR (p) <= MAX_UNICODE_CHAR))))
	{
	  p = p.offset (len as isize);
	  continue;
	}

      /* A sequence to change on decoding was found.  A rare case.  */
      if len == 0
	{
	  if NILP (handle_8_bit)
	    { return Qnil (); }
	  num_8_bit += 1;
	  len = 1;
	}
      else			/* len == 4 or 5 */
	{
	  if NILP (handle_over_uni)
	    { return Qnil (); }
	  if len == 4
	    { num_over_4 += 1; }
	  else
	    { num_over_5 += 1; }
	}
      change_byte_sequence = true;
      p = p.offset (len as isize);
    }

  let mut val: Lisp_Object = Qnil ();	     /* the return value (PORTING.md 1.5) */

  if ! change_byte_sequence
      && NILP (buffer)
    {
      if nocopy && STRINGP (string)
	{ return string; }
      val = make_uninit_multibyte_string (outchars as EMACS_INT, outbytes as EMACS_INT);
      memcpy (SDATA (val) as *mut c_void, str_orig as *const c_void,
	      pend.offset_from (str_orig) as size_t);
      return val;
    }

  /* Count the number of resulting chars and bytes.  */
  let mut replace_8_bit: *mut c_uchar = std::ptr::null_mut ();
  let mut replace_over_uni: *mut c_uchar = std::ptr::null_mut ();
  let mut replace_8_bit_len: c_int = 0; let mut replace_over_uni_len: c_int = 0;

  if change_byte_sequence
    {
      if num_8_bit > 0
	{
	  if CHARACTERP (handle_8_bit)
	    { replace_8_bit = get_char_bytes (XFIXNUM (handle_8_bit) as c_int,
					      &raw mut replace_8_bit_len); }
	  else if STRINGP (handle_8_bit)
	    {
	      replace_8_bit = SDATA (handle_8_bit) as *mut c_uchar;
	      replace_8_bit_len = SBYTES (handle_8_bit) as c_int;
	    }
	  if ! replace_8_bit.is_null ()
	    { outbytes += (replace_8_bit_len - 1) as ptrdiff_t * num_8_bit; }
	  else if EQ (handle_8_bit, Qignored ())
	    {
	      outbytes -= num_8_bit;
	      outchars -= num_8_bit;
	    }
	  else /* EQ (handle_8_bit, Qt)) */
	    { outbytes += num_8_bit; }
	}
      else if num_over_4 + num_over_5 > 0
	{
	  if CHARACTERP (handle_over_uni)
	    { replace_over_uni = get_char_bytes (XFIXNUM (handle_over_uni) as c_int,
						 &raw mut replace_over_uni_len); }
	  else if STRINGP (handle_over_uni)
	    {
	      replace_over_uni = SDATA (handle_over_uni) as *mut c_uchar;
	      replace_over_uni_len = SBYTES (handle_over_uni) as c_int;
	    }
	  if num_over_4 > 0
	    {
	      if ! replace_over_uni.is_null ()
		{ outbytes += (replace_over_uni_len - 4) as ptrdiff_t * num_over_4; }
	      else if EQ (handle_over_uni, Qignored ())
		{
		  outbytes -= 4 * num_over_4;
		  outchars -= num_over_4;
		}
	    }
	  if num_over_5 > 0
	    {
	      if ! replace_over_uni.is_null ()
		{ outbytes += (replace_over_uni_len - 5) as ptrdiff_t * num_over_5; }
	      else if EQ (handle_over_uni, Qignored ())
		{
		  outbytes -= 5 * num_over_5;
		  outchars -= num_over_5;
		}
	    }
	}
    }

  /* Prepare return value and  space to store the decoded bytes.  */
  if BUFFERP (buffer)
    {
      val = make_fixnum (outchars as EMACS_INT);
      dst = get_buffer_gap_address (buffer, outbytes);
    }
  else
    {
      if nocopy && (num_8_bit + num_over_4 + num_over_5) == 0
	  && STRINGP (string)
	{ return string; }
      val = make_uninit_multibyte_string (outchars as EMACS_INT, outbytes as EMACS_INT);
      dst = SDATA (val) as *mut c_uchar;
    }

  src = str_orig;
  if change_byte_sequence
    {
      p = src;
      while p < pend
	{
	  /* Try short cut for an ASCII-only case.  */
	  /* while (p < pend && *p < 0x80) p++; */
	  /* if (p == pend) */
	  /*   break; */
	  let c: c_int = *p as c_int;
	  if c < 0x80
	    {
	      p = p.offset (1);
	      continue;
	    }
	  let mut len: c_int = UTF_8_SEQUENCE_LENGTH! (c);
	  if len > 1
	    {
	      let mut mlen: c_int;
	      mlen = 1;
	      while mlen < len && UTF_8_EXTRA_OCTET_P! (*p.offset (mlen as isize))
		{ mlen += 1; }
	      if mlen == len
		  && (len <= 3
		      || (len == 4 && STRING_CHAR (p) <= MAX_UNICODE_CHAR)
		      || EQ (handle_over_uni, Qt ()))
		{
		  p = p.offset (len as isize);
		  continue;
		}
	    }

	  if src < p
	    {
	      memcpy (dst as *mut c_void, src as *const c_void,
		      p.offset_from (src) as size_t);
	      dst = dst.offset (p.offset_from (src));
	    }
	  if len == 0
	    {
	      if ! replace_8_bit.is_null ()
		{
		  memcpy (dst as *mut c_void, replace_8_bit as *const c_void,
			  replace_8_bit_len as size_t);
		  dst = dst.offset (replace_8_bit_len as isize);
		}
	      else if EQ (handle_8_bit, Qt ())
		{
		  dst = dst.offset (BYTE8_STRING (c, dst) as isize);
		}
	      len = 1;
	    }
	  else			/* len == 4 or 5 */
	    {
	      /* Handle p[0]... by handle_over_uni.  */
	      if ! replace_over_uni.is_null ()
		{
		  memcpy (dst as *mut c_void, replace_over_uni as *const c_void,
			  replace_over_uni_len as size_t);
		  dst = dst.offset (replace_over_uni_len as isize);
		}
	    }
	  p = p.offset (len as isize);
	  src = p;
	}
    }

  if src < pend
    { memcpy (dst as *mut c_void, src as *const c_void, pend.offset_from (src) as size_t); }
  if BUFFERP (buffer)
    {
      let oldb: *mut buffer = (current_buffer! ());

      set_current_buffer! (XBUFFER (buffer));
      insert_from_gap (outchars, outbytes, false, false);
      set_current_buffer! (oldb);
    }
  return val;
}


DEFUN! ("define-coding-system-internal", Fdefine_coding_system_internal,
       Sdefine_coding_system_internal, coding_arg_max, MANY, 0,
       doc: "For internal use only.\nusage: (define-coding-system-internal ...)",
       (nargs: isize, args: *mut Lisp_Object),
       {
	 let mut category: coding_category = 0;
	 let mut max_charset_id: c_int = 0;

	 /* C's `goto short_args` (PORTING.md 2.2): the forward jump to the tail of the
	    function is the labelled block, and its code follows the block.  */
	 'short_args: {
	 if nargs < coding_arg_max as isize
	   { break 'short_args; }

	 let attrs: Lisp_Object = make_nil_vector (coding_attr_last_index as isize);

	 let name: Lisp_Object = *args.offset (coding_arg_name as isize);
	 CHECK_SYMBOL (name);
	 ASET (attrs, coding_attr_base_name as isize, name);

	 let mut val: Lisp_Object = *args.offset (coding_arg_mnemonic as isize);
	 /* decode_mode_spec_coding assumes the mnemonic is a single character.  */
	 if STRINGP (val)
	   { val = make_fixnum (STRING_CHAR (SDATA (val)) as EMACS_INT); }
	 else
	   { CHECK_CHARACTER (val); }
	 ASET (attrs, coding_attr_mnemonic as isize, val);

	 let coding_type: Lisp_Object = *args.offset (coding_arg_coding_type as isize);
	 CHECK_SYMBOL (coding_type);
	 ASET (attrs, coding_attr_type as isize, coding_type);

	 let mut charset_list: Lisp_Object = *args.offset (coding_arg_charset_list as isize);
	 if SYMBOLP (charset_list)
	   {
	     if EQ (charset_list, Qiso_2022 ())
	       {
		 if ! EQ (coding_type, Qiso_2022 ())
		   { error (c"Invalid charset-list".as_ptr ()); }
		 charset_list = Viso_2022_charset_list;
	       }
	     else if EQ (charset_list, Qemacs_mule ())
	       {
		 if ! EQ (coding_type, Qemacs_mule ())
		   { error (c"Invalid charset-list".as_ptr ()); }
		 charset_list = Vemacs_mule_charset_list;
	       }
	     {
	       let mut tail: Lisp_Object = charset_list;
	       while CONSP (tail)
		 {
		   if ! RANGED_FIXNUMP (0, XCAR (tail), INT_MAX as EMACS_INT - 1)
		     { error (c"Invalid charset-list".as_ptr ()); }
		   if (max_charset_id as ptrdiff_t) < XFIXNAT (XCAR (tail))
		     { max_charset_id = XFIXNAT (XCAR (tail)) as c_int; }
		   tail = XCDR (tail);
		 }
	     }
	   }
	 else
	   {
	     charset_list = Fcopy_sequence (charset_list);
	     {
	       let mut tail: Lisp_Object = charset_list;
	       while CONSP (tail)
		 {
		   let mut charset: *mut charset = std::ptr::null_mut ();

		   val = XCAR (tail);
		   CHECK_CHARSET_GET_CHARSET! (val, charset);
		   if (if EQ (coding_type, Qiso_2022 ())
			 { CHARSET_ISO_FINAL! (charset) < 0 }
		       else if EQ (coding_type, Qemacs_mule ())
			 { CHARSET_EMACS_MULE_ID! (charset) < 0 }
		       else { false })
		     { error (c"Can't handle charset `%s'".as_ptr (),
			      SDATA (SYMBOL_NAME (CHARSET_NAME! (charset)))); }

		   XSETCAR (tail, make_fixnum ((*charset).id as EMACS_INT));
		   if max_charset_id < (*charset).id
		     { max_charset_id = (*charset).id; }
		   tail = XCDR (tail);
		 }
	     }
	   }
	 ASET (attrs, coding_attr_charset_list as isize, charset_list);

	 let safe_charsets: Lisp_Object = make_uninit_string ((max_charset_id + 1) as EMACS_INT);
	 memset (SDATA (safe_charsets) as *mut c_void, 255, (max_charset_id + 1) as size_t);
	 {
	   let mut tail: Lisp_Object = charset_list;
	   while CONSP (tail)
	     {
	       SSET (safe_charsets, XFIXNAT (XCAR (tail)) as usize, 0);
	       tail = XCDR (tail);
	     }
	 }
	 ASET (attrs, coding_attr_safe_charsets as isize, safe_charsets);

	 ASET (attrs, coding_attr_ascii_compat as isize,
	       *args.offset (coding_arg_ascii_compatible_p as isize));

	 val = *args.offset (coding_arg_decode_translation_table as isize);
	 if ! CHAR_TABLE_P (val) && ! CONSP (val)
	   { CHECK_SYMBOL (val); }
	 ASET (attrs, coding_attr_decode_tbl as isize, val);

	 val = *args.offset (coding_arg_encode_translation_table as isize);
	 if ! CHAR_TABLE_P (val) && ! CONSP (val)
	   { CHECK_SYMBOL (val); }
	 ASET (attrs, coding_attr_encode_tbl as isize, val);

	 val = *args.offset (coding_arg_post_read_conversion as isize);
	 CHECK_SYMBOL (val);
	 ASET (attrs, coding_attr_post_read as isize, val);

	 val = *args.offset (coding_arg_pre_write_conversion as isize);
	 CHECK_SYMBOL (val);
	 ASET (attrs, coding_attr_pre_write as isize, val);

	 val = *args.offset (coding_arg_default_char as isize);
	 if NILP (val)
	   { ASET (attrs, coding_attr_default_char as isize, make_fixnum (' ' as EMACS_INT)); }
	 else
	   {
	     CHECK_CHARACTER (val);
	     ASET (attrs, coding_attr_default_char as isize, val);
	   }

	 val = *args.offset (coding_arg_for_unibyte as isize);
	 ASET (attrs, coding_attr_for_unibyte as isize,
	       if NILP (val) { Qnil () } else { Qt () });

	 val = *args.offset (coding_arg_plist as isize);
	 CHECK_LIST (val);
	 ASET (attrs, coding_attr_plist as isize, val);

	 if EQ (coding_type, Qcharset ())
	   {
	     /* Generate a lisp vector of 256 elements.  Each element is nil,
		integer, or a list of charset IDs.

		If Nth element is nil, the byte code N is invalid in this
		coding system.

		If Nth element is a number NUM, N is the first byte of a
		charset whose ID is NUM.

		If Nth element is a list of charset IDs, N is the first byte
		of one of them.  The list is sorted by dimensions of the
		charsets.  A charset of smaller dimension comes first. */
	     val = make_nil_vector (256);

	     {
	       let mut tail: Lisp_Object = charset_list;
	       while CONSP (tail)
		 {
		   let charset: *mut charset = CHARSET_FROM_ID! (XFIXNAT (XCAR (tail)));
		   let dim: c_int = CHARSET_DIMENSION! (charset);
		   let idx: c_int = (dim - 1) * 4;

		   if charset_bits! (ascii_compatible_p, charset)
		     { ASET (attrs, coding_attr_ascii_compat as isize, Qt ()); }

		   let mut i: c_int = (*charset).code_space[idx as usize];
		   while i <= (*charset).code_space[(idx + 1) as usize]
		     {
		       let mut tmp: Lisp_Object; let mut tmp2: Lisp_Object;
		       let mut dim2: c_int;

		       tmp = AREF (val, i as isize);
		       if NILP (tmp)
			 { tmp = XCAR (tail); }
		       else if FIXNATP (tmp)
			 {
			   dim2 = CHARSET_DIMENSION! (CHARSET_FROM_ID! (XFIXNAT (tmp)));
			   if dim < dim2
			     { tmp = list2 (XCAR (tail), tmp); }
			   else
			     { tmp = list2 (tmp, XCAR (tail)); }
			 }
		       else
			 {
			   tmp2 = tmp;
			   while CONSP (tmp2)
			     {
			       dim2 = CHARSET_DIMENSION! (CHARSET_FROM_ID! (XFIXNAT (XCAR (tmp2))));
			       if dim < dim2
				 { break; }
			       tmp2 = XCDR (tmp2);
			     }
			   if NILP (tmp2)
			     { tmp = nconc2 (tmp, list1 (XCAR (tail))); }
			   else
			     {
			       XSETCDR (tmp2, Fcons (XCAR (tmp2), XCDR (tmp2)));
			       XSETCAR (tmp2, XCAR (tail));
			     }
			 }
		       ASET (val, i as isize, tmp);
		       i += 1;
		     }
		   tail = XCDR (tail);
		 }
	     }
	     ASET (attrs, coding_attr_charset_valids as isize, val);
	     category = coding_category_charset;
	   }
	 else if EQ (coding_type, Qccl ())
	   {
	     let mut valids: Lisp_Object;

	     if nargs < coding_arg_ccl_max as isize
	       { break 'short_args; }

	     val = *args.offset (coding_arg_ccl_decoder as isize);
	     CHECK_CCL_PROGRAM! (val);
	     if VECTORP (val)
	       { val = Fcopy_sequence (val); }
	     ASET (attrs, coding_attr_ccl_decoder as isize, val);

	     val = *args.offset (coding_arg_ccl_encoder as isize);
	     CHECK_CCL_PROGRAM! (val);
	     if VECTORP (val)
	       { val = Fcopy_sequence (val); }
	     ASET (attrs, coding_attr_ccl_encoder as isize, val);

	     val = *args.offset (coding_arg_ccl_valids as isize);
	     valids = Fmake_string (make_fixnum (256), make_fixnum (0), Qnil ());
	     {
	       let mut tail: Lisp_Object = val;
	       while CONSP (tail)
		 {
		   let mut from: c_int; let mut to: c_int;

		   val = XCAR (tail);
		   if FIXNUMP (val)
		     {
		       if ! (0 <= XFIXNUM (val) && XFIXNUM (val) <= 255)
			 { args_out_of_range_3 (val, make_fixnum (0), make_fixnum (255)); }
		       from = XFIXNUM (val) as c_int; to = from;
		     }
		   else
		     {
		       CHECK_CONS (val);
		       from = check_integer_range (XCAR (val), 0, 255) as c_int;
		       to = check_integer_range (XCDR (val), from as intmax_t, 255) as c_int;
		     }
		   let mut i: c_int = from;
		   while i <= to
		     {
		       SSET (valids, i as usize, 1);
		       i += 1;
		     }
		   tail = XCDR (tail);
		 }
	     }
	     ASET (attrs, coding_attr_ccl_valids as isize, valids);

	     category = coding_category_ccl;
	   }
	 else if EQ (coding_type, Qutf_16 ())
	   {
	     let bom: Lisp_Object; let mut endian: Lisp_Object;

	     ASET (attrs, coding_attr_ascii_compat as isize, Qnil ());

	     if nargs < coding_arg_utf16_max as isize
	       { break 'short_args; }

	     bom = *args.offset (coding_arg_utf16_bom as isize);
	     if ! NILP (bom) && ! EQ (bom, Qt ())
	       {
		 CHECK_CONS (bom);
		 val = XCAR (bom);
		 CHECK_CODING_SYSTEM (val);
		 val = XCDR (bom);
		 CHECK_CODING_SYSTEM (val);
	       }
	     ASET (attrs, coding_attr_utf_bom as isize, bom);

	     endian = *args.offset (coding_arg_utf16_endian as isize);
	     CHECK_SYMBOL (endian);
	     if NILP (endian)
	       { endian = Qbig (); }
	     else if ! EQ (endian, Qbig ()) && ! EQ (endian, Qlittle ())
	       { error (c"Invalid endian: %s".as_ptr (), SDATA (SYMBOL_NAME (endian))); }
	     ASET (attrs, coding_attr_utf_16_endian as isize, endian);

	     category = if CONSP (bom)
			  { coding_category_utf_16_auto }
			else if NILP (bom)
			  {
			    if EQ (endian, Qbig ())
			      { coding_category_utf_16_be_nosig }
			    else { coding_category_utf_16_le_nosig }
			  }
			else
			  {
			    if EQ (endian, Qbig ())
			      { coding_category_utf_16_be }
			    else { coding_category_utf_16_le }
			  };
	   }
	 else if EQ (coding_type, Qiso_2022 ())
	   {
	     let initial: Lisp_Object; let reg_usage: Lisp_Object;
	     let request: Lisp_Object; let mut flags: Lisp_Object;

	     if nargs < coding_arg_iso2022_max as isize
	       { break 'short_args; }

	     initial = Fcopy_sequence (*args.offset (coding_arg_iso2022_initial as isize));
	     CHECK_VECTOR (initial);
	     {
	       let mut i: c_int = 0;
	       while i < 4
		 {
		   val = AREF (initial, i as isize);
		   if ! NILP (val)
		     {
		       let mut charset: *mut charset = std::ptr::null_mut ();

		       CHECK_CHARSET_GET_CHARSET! (val, charset);
		       ASET (initial, i as isize, make_fixnum (CHARSET_ID! (charset) as EMACS_INT));
		       if i == 0 && charset_bits! (ascii_compatible_p, charset)
			 { ASET (attrs, coding_attr_ascii_compat as isize, Qt ()); }
		     }
		   else
		     { ASET (initial, i as isize, make_fixnum (-1)); }
		   i += 1;
		 }
	     }

	     reg_usage = *args.offset (coding_arg_iso2022_reg_usage as isize);
	     CHECK_CONS (reg_usage);
	     CHECK_FIXNUM (XCAR (reg_usage));
	     CHECK_FIXNUM (XCDR (reg_usage));

	     request = Fcopy_sequence (*args.offset (coding_arg_iso2022_request as isize));
	     {
	       let mut tail: Lisp_Object = request;
	       while CONSP (tail)
		 {
		   let mut id: c_int = 0;

		   val = XCAR (tail);
		   CHECK_CONS (val);
		   CHECK_CHARSET_GET_ID! (XCAR (val), id);
		   check_integer_range (XCDR (val), 0, 3);
		   XSETCAR (val, make_fixnum (id as EMACS_INT));
		   tail = XCDR (tail);
		 }
	     }

	     flags = *args.offset (coding_arg_iso2022_flags as isize);
	     CHECK_FIXNAT (flags);
	     {
	       let mut i: c_int = (XFIXNUM (flags) & INT_MAX as isize) as c_int;
	       if EQ (*args.offset (coding_arg_charset_list as isize), Qiso_2022 ())
		 { i |= CODING_ISO_FLAG_FULL_SUPPORT; }
	       flags = make_fixnum (i as EMACS_INT);

	       ASET (attrs, coding_attr_iso_initial as isize, initial);
	       ASET (attrs, coding_attr_iso_usage as isize, reg_usage);
	       ASET (attrs, coding_attr_iso_request as isize, request);
	       ASET (attrs, coding_attr_iso_flags as isize, flags);
	       setup_iso_safe_charsets (attrs);

	       if i & CODING_ISO_FLAG_SEVEN_BITS != 0
		 {
		   category = if i & (CODING_ISO_FLAG_LOCKING_SHIFT
				      | CODING_ISO_FLAG_SINGLE_SHIFT) != 0
				{ coding_category_iso_7_else }
			      else if EQ (*args.offset (coding_arg_charset_list as isize),
					  Qiso_2022 ())
				{ coding_category_iso_7 }
			      else { coding_category_iso_7_tight };
		 }
	       else
		 {
		   let id: c_int = XFIXNUM (AREF (initial, 1)) as c_int;

		   category = if (i & CODING_ISO_FLAG_LOCKING_SHIFT) != 0
				 || EQ (*args.offset (coding_arg_charset_list as isize),
					Qiso_2022 ())
				 || id < 0
				{ coding_category_iso_8_else }
			      else if CHARSET_DIMENSION! (CHARSET_FROM_ID! (id as isize)) == 1
				{ coding_category_iso_8_1 }
			      else { coding_category_iso_8_2 };
		 }
	       if category != coding_category_iso_8_1
		   && category != coding_category_iso_8_2
		 { ASET (attrs, coding_attr_ascii_compat as isize, Qnil ()); }
	     }
	   }
	 else if EQ (coding_type, Qemacs_mule ())
	   {
	     if EQ (*args.offset (coding_arg_charset_list as isize), Qemacs_mule ())
	       { ASET (attrs, coding_attr_emacs_mule_full as isize, Qt ()); }
	     ASET (attrs, coding_attr_ascii_compat as isize, Qt ());
	     category = coding_category_emacs_mule;
	   }
	 else if EQ (coding_type, Qshift_jis ())
	   {
	     let charset_list_len: ptrdiff_t = list_length (charset_list);
	     if charset_list_len != 3 && charset_list_len != 4
	       { error (c"There should be three or four charsets".as_ptr ()); }

	     let mut charset: *mut charset = CHARSET_FROM_ID! (XFIXNUM (XCAR (charset_list)));
	     if CHARSET_DIMENSION! (charset) != 1
	       { error (c"Dimension of charset %s is not one".as_ptr (),
			SDATA (SYMBOL_NAME (CHARSET_NAME! (charset)))); }
	     if charset_bits! (ascii_compatible_p, charset)
	       { ASET (attrs, coding_attr_ascii_compat as isize, Qt ()); }

	     charset_list = XCDR (charset_list);
	     charset = CHARSET_FROM_ID! (XFIXNUM (XCAR (charset_list)));
	     if CHARSET_DIMENSION! (charset) != 1
	       { error (c"Dimension of charset %s is not one".as_ptr (),
			SDATA (SYMBOL_NAME (CHARSET_NAME! (charset)))); }

	     charset_list = XCDR (charset_list);
	     charset = CHARSET_FROM_ID! (XFIXNUM (XCAR (charset_list)));
	     if CHARSET_DIMENSION! (charset) != 2
	       { error (c"Dimension of charset %s is not two".as_ptr (),
			SDATA (SYMBOL_NAME (CHARSET_NAME! (charset)))); }

	     charset_list = XCDR (charset_list);
	     if ! NILP (charset_list)
	       {
		 charset = CHARSET_FROM_ID! (XFIXNUM (XCAR (charset_list)));
		 if CHARSET_DIMENSION! (charset) != 2
		   { error (c"Dimension of charset %s is not two".as_ptr (),
			    SDATA (SYMBOL_NAME (CHARSET_NAME! (charset)))); }
	       }

	     category = coding_category_sjis;
	     Vsjis_coding_system = name;
	   }
	 else if EQ (coding_type, Qbig5 ())
	   {
	     let mut charset: *mut charset;

	     if list_length (charset_list) != 2
	       { error (c"There should be just two charsets".as_ptr ()); }

	     charset = CHARSET_FROM_ID! (XFIXNUM (XCAR (charset_list)));
	     if CHARSET_DIMENSION! (charset) != 1
	       { error (c"Dimension of charset %s is not one".as_ptr (),
			SDATA (SYMBOL_NAME (CHARSET_NAME! (charset)))); }
	     if charset_bits! (ascii_compatible_p, charset)
	       { ASET (attrs, coding_attr_ascii_compat as isize, Qt ()); }

	     charset_list = XCDR (charset_list);
	     charset = CHARSET_FROM_ID! (XFIXNUM (XCAR (charset_list)));
	     if CHARSET_DIMENSION! (charset) != 2
	       { error (c"Dimension of charset %s is not two".as_ptr (),
			SDATA (SYMBOL_NAME (CHARSET_NAME! (charset)))); }

	     category = coding_category_big5;
	     Vbig5_coding_system = name;
	   }
	 else if EQ (coding_type, Qraw_text ())
	   {
	     category = coding_category_raw_text;
	     ASET (attrs, coding_attr_ascii_compat as isize, Qt ());
	   }
	 else if EQ (coding_type, Qutf_8 ())
	   {
	     let bom: Lisp_Object;

	     if nargs < coding_arg_utf8_max as isize
	       { break 'short_args; }

	     bom = *args.offset (coding_arg_utf8_bom as isize);
	     if ! NILP (bom) && ! EQ (bom, Qt ())
	       {
		 CHECK_CONS (bom);
		 val = XCAR (bom);
		 CHECK_CODING_SYSTEM (val);
		 val = XCDR (bom);
		 CHECK_CODING_SYSTEM (val);
	       }
	     ASET (attrs, coding_attr_utf_bom as isize, bom);
	     if NILP (bom)
	       { ASET (attrs, coding_attr_ascii_compat as isize, Qt ()); }

	     category = if CONSP (bom) { coding_category_utf_8_auto }
			else if NILP (bom) { coding_category_utf_8_nosig }
			else { coding_category_utf_8_sig };
	   }
	 else if EQ (coding_type, Qundecided ())
	   {
	     if nargs < coding_arg_undecided_max as isize
	       { break 'short_args; }
	     ASET (attrs, coding_attr_undecided_inhibit_null_byte_detection as isize,
		   *args.offset (coding_arg_undecided_inhibit_null_byte_detection as isize));
	     ASET (attrs, coding_attr_undecided_inhibit_iso_escape_detection as isize,
		   *args.offset (coding_arg_undecided_inhibit_iso_escape_detection as isize));
	     ASET (attrs, coding_attr_undecided_prefer_utf_8 as isize,
		   *args.offset (coding_arg_undecided_prefer_utf_8 as isize));
	     category = coding_category_undecided;
	   }
	 else
	   { error (c"Invalid coding system type: %s".as_ptr (),
		    SDATA (SYMBOL_NAME (coding_type))); }

	 ASET (attrs, coding_attr_category as isize, make_fixnum (category as EMACS_INT));
	 ASET (attrs, coding_attr_plist as isize,
	       Fcons (QCcategory (),
		      Fcons (AREF (Vcoding_category_table, category as isize),
			     CODING_ATTR_PLIST (attrs))));
	 ASET (attrs, coding_attr_plist as isize,
	       Fcons (QCascii_compatible_p (),
		      Fcons (CODING_ATTR_ASCII_COMPAT (attrs),
			     CODING_ATTR_PLIST (attrs))));

	 let mut eol_type: Lisp_Object = *args.offset (coding_arg_eol_type as isize);
	 if ! NILP (eol_type)
	     && ! EQ (eol_type, Qunix ())
	     && ! EQ (eol_type, Qdos ())
	     && ! EQ (eol_type, Qmac ())
	   { error (c"Invalid eol-type".as_ptr ()); }

	 let aliases: Lisp_Object = list1 (name);

	 if NILP (eol_type)
	   {
	     eol_type = make_subsidiaries (name);
	     {
	       let mut i: c_int = 0;
	       while i < 3
		 {
		   let this_name: Lisp_Object;
		   let this_aliases: Lisp_Object;
		   let this_eol_type: Lisp_Object;
		   let this_spec: Lisp_Object;

		   this_name = AREF (eol_type, i as isize);
		   this_aliases = list1 (this_name);
		   this_eol_type = if i == 0 { Qunix () } else if i == 1 { Qdos () } else { Qmac () };
		   this_spec = make_uninit_vector (3);
		   ASET (this_spec, 0, attrs);
		   ASET (this_spec, 1, this_aliases);
		   ASET (this_spec, 2, this_eol_type);
		   Fputhash (this_name, this_spec, Vcoding_system_hash_table);
		   Vcoding_system_list = Fcons (this_name, Vcoding_system_list);
		   val = Fassoc (Fsymbol_name (this_name), Vcoding_system_alist, Qnil ());
		   if NILP (val)
		     {
		       Vcoding_system_alist
			 = Fcons (Fcons (Fsymbol_name (this_name), Qnil ()),
				  Vcoding_system_alist);
		     }
		   i += 1;
		 }
	     }
	   }

	 let spec_vec: Lisp_Object = make_uninit_vector (3);
	 ASET (spec_vec, 0, attrs);
	 ASET (spec_vec, 1, aliases);
	 ASET (spec_vec, 2, eol_type);

	 Fputhash (name, spec_vec, Vcoding_system_hash_table);
	 Vcoding_system_list = Fcons (name, Vcoding_system_list);
	 val = Fassoc (Fsymbol_name (name), Vcoding_system_alist, Qnil ());
	 if NILP (val)
	   {
	     Vcoding_system_alist = Fcons (Fcons (Fsymbol_name (name), Qnil ()),
					   Vcoding_system_alist);
	   }

	 let id: c_int = coding_categories[category as usize].id as c_int;
	 if id < 0 || EQ (name, CODING_ID_NAME (id as hash_idx_t))
	   { setup_coding_system (name, &raw mut coding_categories[category as usize]); }

	 return Qnil ();

	 }

	 Fsignal (Qwrong_number_of_arguments (),
		  Fcons (Qdefine_coding_system_internal (),
			 make_fixnum (nargs as EMACS_INT)))
       });


DEFUN! ("coding-system-put", Fcoding_system_put, Scoding_system_put,
       3, 3, 0,
       doc: "Change value of CODING-SYSTEM's property PROP to VAL.\n\nThe following properties, if set by this function, override the values\nof the corresponding attributes set by `define-coding-system':\n\n  `:mnemonic', `:default-char', `:ascii-compatible-p'\n  `:decode-translation-table', `:encode-translation-table',\n  `:post-read-conversion', `:pre-write-conversion'\n\nSee `define-coding-system' for the description of these properties.\nSee `coding-system-get' and `coding-system-plist' for accessing the\nproperty list of a coding-system.",
       (coding_system: Lisp_Object, prop: Lisp_Object, val: Lisp_Object),
       {
	 let mut spec: Lisp_Object; let mut attrs: Lisp_Object;
	 let mut val = val;   /* C reassigns the argument.  */

	 spec = CHECK_CODING_SYSTEM_GET_SPEC (coding_system);
	 attrs = AREF (spec, 0);
	 if EQ (prop, QCmnemonic ())
	   {
	     /* decode_mode_spec_coding assumes the mnemonic is a single character.  */
	     if STRINGP (val)
	       { val = make_fixnum (STRING_CHAR (SDATA (val)) as EMACS_INT); }
	     else
	       { CHECK_CHARACTER (val); }
	     ASET (attrs, coding_attr_mnemonic as isize, val);
	   }
	 else if EQ (prop, QCdefault_char ())
	   {
	     if NILP (val)
	       { val = make_fixnum (' ' as EMACS_INT); }
	     else
	       { CHECK_CHARACTER (val); }
	     ASET (attrs, coding_attr_default_char as isize, val);
	   }
	 else if EQ (prop, QCdecode_translation_table ())
	   {
	     if ! CHAR_TABLE_P (val) && ! CONSP (val)
	       { CHECK_SYMBOL (val); }
	     ASET (attrs, coding_attr_decode_tbl as isize, val);
	   }
	 else if EQ (prop, QCencode_translation_table ())
	   {
	     if ! CHAR_TABLE_P (val) && ! CONSP (val)
	       { CHECK_SYMBOL (val); }
	     ASET (attrs, coding_attr_encode_tbl as isize, val);
	   }
	 else if EQ (prop, QCpost_read_conversion ())
	   {
	     CHECK_SYMBOL (val);
	     ASET (attrs, coding_attr_post_read as isize, val);
	   }
	 else if EQ (prop, QCpre_write_conversion ())
	   {
	     CHECK_SYMBOL (val);
	     ASET (attrs, coding_attr_pre_write as isize, val);
	   }
	 else if EQ (prop, QCascii_compatible_p ())
	   {
	     ASET (attrs, coding_attr_ascii_compat as isize, val);
	   }

	 ASET (attrs, coding_attr_plist as isize,
	       plist_put (CODING_ATTR_PLIST (attrs), prop, val));
	 return val;
       });


DEFUN! ("define-coding-system-alias", Fdefine_coding_system_alias,
       Sdefine_coding_system_alias, 2, 2, 0,
       doc: "Define ALIAS as an alias for CODING-SYSTEM.",
       (alias: Lisp_Object, coding_system: Lisp_Object),
       {
	 let mut spec: Lisp_Object; let mut aliases: Lisp_Object;
	 let eol_type: Lisp_Object; let mut val: Lisp_Object;

	 CHECK_SYMBOL (alias);
	 spec = CHECK_CODING_SYSTEM_GET_SPEC (coding_system);
	 aliases = AREF (spec, 1);
	 /* ALIASES should be a list of length more than zero, and the first
	    element is a base coding system.  Append ALIAS at the tail of the
	    list.  */
	 while ! NILP (XCDR (aliases))
	   { aliases = XCDR (aliases); }
	 XSETCDR (aliases, list1 (alias));

	 eol_type = AREF (spec, 2);
	 if VECTORP (eol_type)
	   {
	     let subsidiaries: Lisp_Object;
	     let mut i: c_int;

	     subsidiaries = make_subsidiaries (alias);
	     i = 0;
	     while i < 3
	       {
		 Fdefine_coding_system_alias (AREF (subsidiaries, i as isize),
					      AREF (eol_type, i as isize));
		 i += 1;
	       }
	   }

	 Fputhash (alias, spec, Vcoding_system_hash_table);
	 Vcoding_system_list = Fcons (alias, Vcoding_system_list);
	 val = Fassoc (Fsymbol_name (alias), Vcoding_system_alist, Qnil ());
	 if NILP (val)
	   {
	     Vcoding_system_alist = Fcons (Fcons (Fsymbol_name (alias), Qnil ()),
					   Vcoding_system_alist);
	   }

	 return Qnil ();
       });

DEFUN! ("coding-system-base", Fcoding_system_base, Scoding_system_base,
       1, 1, 0,
       doc: "Return the base of CODING-SYSTEM.\nAny alias or subsidiary coding system is not a base coding system.",
       (coding_system: Lisp_Object),
       {
	 let spec: Lisp_Object; let mut attrs: Lisp_Object;

	 if NILP (coding_system)
	   { return Qno_conversion (); }
	 spec = CHECK_CODING_SYSTEM_GET_SPEC (coding_system);
	 attrs = AREF (spec, 0);
	 return CODING_ATTR_BASE_NAME (attrs);
       });

DEFUN! ("coding-system-plist", Fcoding_system_plist, Scoding_system_plist,
       1, 1, 0,
       doc: "Return the property list of CODING-SYSTEM.",
       (coding_system: Lisp_Object),
       {
	 let mut spec: Lisp_Object; let mut attrs: Lisp_Object;

	 let mut coding_system = coding_system;   /* C reassigns the argument.  */
	 if NILP (coding_system)
	   { coding_system = Qno_conversion (); }
	 spec = CHECK_CODING_SYSTEM_GET_SPEC (coding_system);
	 attrs = AREF (spec, 0);
	 return CODING_ATTR_PLIST (attrs);
       });


DEFUN! ("coding-system-aliases", Fcoding_system_aliases, Scoding_system_aliases,
       1, 1, 0,
       doc: "Return the list of aliases of CODING-SYSTEM.",
       (coding_system: Lisp_Object),
       {
	 let spec: Lisp_Object;

	 let mut coding_system = coding_system;   /* C reassigns the argument.  */
	 if NILP (coding_system)
	   { coding_system = Qno_conversion (); }
	 spec = CHECK_CODING_SYSTEM_GET_SPEC (coding_system);
	 return AREF (spec, 1);
       });

DEFUN! ("coding-system-eol-type", Fcoding_system_eol_type,
       Scoding_system_eol_type, 1, 1, 0,
       doc: "Return eol-type of CODING-SYSTEM.\nAn eol-type is an integer 0, 1, 2, or a vector of coding systems.\n\nInteger values 0, 1, and 2 indicate a format of end-of-line; LF, CRLF,\nand CR respectively.\n\nA vector value indicates that a format of end-of-line should be\ndetected automatically.  Nth element of the vector is the subsidiary\ncoding system whose eol-type is N.",
       (coding_system: Lisp_Object),
       {
	 let spec: Lisp_Object; let mut eol_type: Lisp_Object;
	 let mut n: c_int;

	 let mut coding_system = coding_system;   /* C reassigns the argument.  */
	 if NILP (coding_system)
	   { coding_system = Qno_conversion (); }
	 if ! CODING_SYSTEM_P (coding_system)
	   { return Qnil (); }
	 spec = CODING_SYSTEM_SPEC (coding_system);
	 eol_type = AREF (spec, 2);
	 if VECTORP (eol_type)
	   { return Fcopy_sequence (eol_type); }
	 n = if EQ (eol_type, Qunix ()) { 0 } else if EQ (eol_type, Qdos ()) { 1 } else { 2 };
	 return make_fixnum (n as EMACS_INT);
       });


/*** 9. Post-amble ***/

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_coding_once ()
{
  let mut i: c_int;

  i = 0;
  while i < coding_category_max
    {
      coding_categories[i as usize].id = -1;
      coding_priorities[i as usize] = i;
      i += 1;
    }

  PDUMPER_REMEMBER_SCALAR! (coding_categories);
  PDUMPER_REMEMBER_SCALAR! (coding_priorities);

  /* ISO2022 specific initialize routine.  */
  i = 0;
  while i < 0x20
    {
      iso_code_class[i as usize] = ISO_control_0;
      i += 1;
    }
  i = 0x21;
  while i < 0x7F
    {
      iso_code_class[i as usize] = ISO_graphic_plane_0;
      i += 1;
    }
  i = 0x80;
  while i < 0xA0
    {
      iso_code_class[i as usize] = ISO_control_1;
      i += 1;
    }
  i = 0xA1;
  while i < 0xFF
    {
      iso_code_class[i as usize] = ISO_graphic_plane_1;
      i += 1;
    }
  iso_code_class[0x20] = ISO_0x20_or_0x7F; iso_code_class[0x7F] = ISO_0x20_or_0x7F;
  iso_code_class[0xA0] = ISO_0xA0_or_0xFF; iso_code_class[0xFF] = ISO_0xA0_or_0xFF;
  iso_code_class[ISO_CODE_SO as usize] = ISO_shift_out;
  iso_code_class[ISO_CODE_SI as usize] = ISO_shift_in;
  iso_code_class[ISO_CODE_SS2_7 as usize] = ISO_single_shift_2_7;
  iso_code_class[ISO_CODE_ESC as usize] = ISO_escape;
  iso_code_class[ISO_CODE_SS2 as usize] = ISO_single_shift_2;
  iso_code_class[ISO_CODE_SS3 as usize] = ISO_single_shift_3;
  iso_code_class[ISO_CODE_CSI as usize] = ISO_control_sequence_introducer;

  PDUMPER_REMEMBER_SCALAR! (iso_code_class);

  i = 0;
  while i < 256
    {
      emacs_mule_bytes[i as usize] = 1;
      i += 1;
    }
  emacs_mule_bytes[EMACS_MULE_LEADING_CODE_PRIVATE_11 as usize] = 3;
  emacs_mule_bytes[EMACS_MULE_LEADING_CODE_PRIVATE_12 as usize] = 3;
  emacs_mule_bytes[EMACS_MULE_LEADING_CODE_PRIVATE_21 as usize] = 4;
  emacs_mule_bytes[EMACS_MULE_LEADING_CODE_PRIVATE_22 as usize] = 4;

  PDUMPER_REMEMBER_SCALAR! (emacs_mule_bytes);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_coding ()
{
  staticpro (&raw const Vcoding_system_hash_table);
  Vcoding_system_hash_table = CALLN! (Fmake_hash_table, QCtest (), Qeq ());

  staticpro (&raw const Vsjis_coding_system);
  Vsjis_coding_system = Qnil ();

  staticpro (&raw const Vbig5_coding_system);
  Vbig5_coding_system = Qnil ();

  staticpro (&raw const Vcode_conversion_reused_workbuf);
  Vcode_conversion_reused_workbuf = Qnil ();

  staticpro (&raw const Vcode_conversion_workbuf_name);
  Vcode_conversion_workbuf_name = build_string (c" *code-conversion-work*".as_ptr ());

  reused_workbuf_in_use = false;
  PDUMPER_REMEMBER_SCALAR! (reused_workbuf_in_use);

  DEFSYM! (Qcharset, c"charset");
  DEFSYM! (Qtarget_idx, c"target-idx");
  DEFSYM! (Qcoding_system_history, c"coding-system-history");
  Fset (Qcoding_system_history (), Qnil ());

  /* Target FILENAME is the first argument.  */
  Fput (Qinsert_file_contents (), Qtarget_idx (), make_fixnum (0));
  /* Target FILENAME is the third argument.  */
  Fput (Qwrite_region (), Qtarget_idx (), make_fixnum (2));

  DEFSYM! (Qcall_process, c"call-process");
  /* Target PROGRAM is the first argument.  */
  Fput (Qcall_process (), Qtarget_idx (), make_fixnum (0));

  DEFSYM! (Qcall_process_region, c"call-process-region");
  /* Target PROGRAM is the third argument.  */
  Fput (Qcall_process_region (), Qtarget_idx (), make_fixnum (2));

  DEFSYM! (Qstart_process, c"start-process");
  /* Target PROGRAM is the third argument.  */
  Fput (Qstart_process (), Qtarget_idx (), make_fixnum (2));

  DEFSYM! (Qopen_network_stream, c"open-network-stream");
  /* Target SERVICE is the fourth argument.  */
  Fput (Qopen_network_stream (), Qtarget_idx (), make_fixnum (3));

  DEFSYM! (Qunix, c"unix");
  DEFSYM! (Qdos, c"dos");
  DEFSYM! (Qmac, c"mac");

  DEFSYM! (Qbuffer_file_coding_system, c"buffer-file-coding-system");
  DEFSYM! (Qundecided, c"undecided");
  DEFSYM! (Qno_conversion, c"no-conversion");
  DEFSYM! (Qraw_text, c"raw-text");
  DEFSYM! (Qus_ascii, c"us-ascii");

  DEFSYM! (Qiso_2022, c"iso-2022");

  DEFSYM! (Qutf_8, c"utf-8");
  DEFSYM! (Qutf_8_unix, c"utf-8-unix");
  DEFSYM! (Qutf_8_emacs, c"utf-8-emacs");

  /* C 11800-11803 is `#if defined (WINDOWSNT) || defined (CYGWIN) || defined
     HAVE_ANDROID`, dead in the frozen configuration (docs/ports/coding.md).  */

  DEFSYM! (Qutf_16, c"utf-16");
  DEFSYM! (Qbig, c"big");
  DEFSYM! (Qlittle, c"little");

  DEFSYM! (Qshift_jis, c"shift-jis");
  DEFSYM! (Qbig5, c"big5");

  DEFSYM! (Qcoding_system_p, c"coding-system-p");

  /* Error signaled when there's a problem with detecting a coding system.  */
  DEFSYM! (Qcoding_system_error, c"coding-system-error");
  Fput (Qcoding_system_error (), Qerror_conditions (),
	list! (Qcoding_system_error (), Qerror ()));
  Fput (Qcoding_system_error (), Qerror_message (),
	build_string (c"Invalid coding system".as_ptr ()));

  DEFSYM! (Qtranslation_table, c"translation-table");
  Fput (Qtranslation_table (), Qchar_table_extra_slots (), make_fixnum (2));
  DEFSYM! (Qtranslation_table_id, c"translation-table-id");

  /* Coding system emacs-mule and raw-text are for converting only
     end-of-line format.  */
  DEFSYM! (Qemacs_mule, c"emacs-mule");

  DEFSYM! (QCcategory, c":category");
  DEFSYM! (QCmnemonic, c":mnemonic");
  DEFSYM! (QCdefault_char, c":default-char");
  DEFSYM! (QCdecode_translation_table, c":decode-translation-table");
  DEFSYM! (QCencode_translation_table, c":encode-translation-table");
  DEFSYM! (QCpost_read_conversion, c":post-read-conversion");
  DEFSYM! (QCpre_write_conversion, c":pre-write-conversion");
  DEFSYM! (QCascii_compatible_p, c":ascii-compatible-p");

  Vcoding_category_table = make_nil_vector (coding_category_max as isize);
  staticpro (&raw const Vcoding_category_table);
  /* Followings are target of code detection.  */
  ASET (Vcoding_category_table, coding_category_iso_7 as isize,
	intern_c_string (c"coding-category-iso-7".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_iso_7_tight as isize,
	intern_c_string (c"coding-category-iso-7-tight".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_iso_8_1 as isize,
	intern_c_string (c"coding-category-iso-8-1".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_iso_8_2 as isize,
	intern_c_string (c"coding-category-iso-8-2".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_iso_7_else as isize,
	intern_c_string (c"coding-category-iso-7-else".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_iso_8_else as isize,
	intern_c_string (c"coding-category-iso-8-else".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_utf_8_auto as isize,
	intern_c_string (c"coding-category-utf-8-auto".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_utf_8_nosig as isize,
	intern_c_string (c"coding-category-utf-8".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_utf_8_sig as isize,
	intern_c_string (c"coding-category-utf-8-sig".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_utf_16_be as isize,
	intern_c_string (c"coding-category-utf-16-be".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_utf_16_auto as isize,
	intern_c_string (c"coding-category-utf-16-auto".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_utf_16_le as isize,
	intern_c_string (c"coding-category-utf-16-le".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_utf_16_be_nosig as isize,
	intern_c_string (c"coding-category-utf-16-be-nosig".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_utf_16_le_nosig as isize,
	intern_c_string (c"coding-category-utf-16-le-nosig".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_charset as isize,
	intern_c_string (c"coding-category-charset".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_sjis as isize,
	intern_c_string (c"coding-category-sjis".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_big5 as isize,
	intern_c_string (c"coding-category-big5".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_ccl as isize,
	intern_c_string (c"coding-category-ccl".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_emacs_mule as isize,
	intern_c_string (c"coding-category-emacs-mule".as_ptr ()));
  /* Followings are NOT target of code detection.  */
  ASET (Vcoding_category_table, coding_category_raw_text as isize,
	intern_c_string (c"coding-category-raw-text".as_ptr ()));
  ASET (Vcoding_category_table, coding_category_undecided as isize,
	intern_c_string (c"coding-category-undecided".as_ptr ()));

  DEFSYM! (Qinsufficient_source, c"insufficient-source");
  DEFSYM! (Qinvalid_source, c"invalid-source");
  DEFSYM! (Qinterrupted, c"interrupted");

  /* If a symbol has this property, evaluate the value to define the
     symbol as a coding system.  */
  DEFSYM! (Qcoding_system_define_form, c"coding-system-define-form");

  DEFSYM! (Qignored, c"ignored");

  DEFSYM! (Qutf_8_string_p, c"utf-8-string-p");
  DEFSYM! (Qfilenamep, c"filenamep");

  defsubr (&raw mut Scoding_system_p);
  defsubr (&raw mut Sread_coding_system);
  defsubr (&raw mut Sread_non_nil_coding_system);
  defsubr (&raw mut Scheck_coding_system);
  defsubr (&raw mut Sdetect_coding_region);
  defsubr (&raw mut Sdetect_coding_string);
  defsubr (&raw mut Sfind_coding_systems_region_internal);
  defsubr (&raw mut Sunencodable_char_position);
  defsubr (&raw mut Scheck_coding_systems_region);
  defsubr (&raw mut Sdecode_coding_region);
  defsubr (&raw mut Sencode_coding_region);
  defsubr (&raw mut Sdecode_coding_string);
  defsubr (&raw mut Sencode_coding_string);
  /* C 11911-11914 registers the two ENABLE_UTF_8_CONVERTER_TEST subrs, dead in the
     frozen configuration (docs/ports/coding.md).  */
  defsubr (&raw mut Sdecode_sjis_char);
  defsubr (&raw mut Sencode_sjis_char);
  defsubr (&raw mut Sdecode_big5_char);
  defsubr (&raw mut Sencode_big5_char);
  defsubr (&raw mut Sset_terminal_coding_system_internal);
  defsubr (&raw mut Sset_safe_terminal_coding_system_internal);
  defsubr (&raw mut Sterminal_coding_system);
  defsubr (&raw mut Sset_keyboard_coding_system_internal);
  defsubr (&raw mut Skeyboard_coding_system);
  defsubr (&raw mut Sfind_operation_coding_system);
  defsubr (&raw mut Sset_coding_system_priority);
  defsubr (&raw mut Sdefine_coding_system_internal);
  defsubr (&raw mut Sdefine_coding_system_alias);
  defsubr (&raw mut Scoding_system_put);
  defsubr (&raw mut Scoding_system_base);
  defsubr (&raw mut Scoding_system_plist);
  defsubr (&raw mut Scoding_system_aliases);
  defsubr (&raw mut Scoding_system_eol_type);
  defsubr (&raw mut Scoding_system_priority_list);

  DEFVAR_LISP! (c"coding-system-list", Vcoding_system_list,
	       " List of coding systems.\n\nDo not alter the value of this variable manually.  This variable should be\nupdated by the functions `define-coding-system' and\n`define-coding-system-alias'.  ");
  Vcoding_system_list = Qnil ();

  DEFVAR_LISP! (c"coding-system-alist", Vcoding_system_alist,
	       " Alist of coding system names.\nEach element is one element list of coding system name.\nThis variable is given to `completing-read' as COLLECTION argument.\n\nDo not alter the value of this variable manually.  This variable should be\nupdated by `define-coding-system-alias'.  ");
  Vcoding_system_alist = Qnil ();

  DEFVAR_LISP! (c"coding-category-list", Vcoding_category_list,
	       " List of coding-categories (symbols) ordered by priority.\n\nOn detecting a coding system, Emacs tries code detection algorithms\nassociated with each coding-category one by one in this order.  When\none algorithm agrees with a byte sequence of source text, the coding\nsystem bound to the corresponding coding-category is selected.\n\nDon't modify this variable directly, but use `set-coding-system-priority'.  ");
  {
    let mut i: c_int;

    Vcoding_category_list = Qnil ();
    i = coding_category_max - 1;
    while i >= 0
      {
	Vcoding_category_list
	  = Fcons (AREF (Vcoding_category_table, i as isize),
		   Vcoding_category_list);
	i -= 1;
      }
  }

  DEFVAR_LISP! (c"coding-system-for-read", Vcoding_system_for_read,
	       " Specify the coding system for read operations.\nIt is useful to bind this variable with `let', but do not set it globally.\nIf the value is a coding system, it is used for decoding on read operation.\nIf not, an appropriate element is used from one of the coding system alists.\nThere are three such tables: `file-coding-system-alist',\n`process-coding-system-alist', and `network-coding-system-alist'.  ");
  Vcoding_system_for_read = Qnil ();

  DEFVAR_LISP! (c"coding-system-for-write", Vcoding_system_for_write,
	       " Specify the coding system for write operations.\nPrograms bind this variable with `let', but you should not set it globally.\nIf the value is a coding system, it is used for encoding of output,\nwhen writing it to a file and when sending it to a file or subprocess.\n\nIf this does not specify a coding system, an appropriate element\nis used from one of the coding system alists.\nThere are three such tables: `file-coding-system-alist',\n`process-coding-system-alist', and `network-coding-system-alist'.\nFor output to files, if the above procedure does not specify a coding system,\nthe value of `buffer-file-coding-system' is used.  ");
  Vcoding_system_for_write = Qnil ();

  DEFVAR_LISP! (c"last-coding-system-used", Vlast_coding_system_used,
	       "\nCoding system used in the latest file or process I/O.  ");
  Vlast_coding_system_used = Qnil ();

  DEFVAR_LISP! (c"last-code-conversion-error", Vlast_code_conversion_error,
	       "\nError status of the last code conversion.\n\nWhen an error was detected in the last code conversion, this variable\nis set to one of the following symbols.\n  `insufficient-source'\n  `inconsistent-eol'\n  `invalid-source'\n  `interrupted'\n  `insufficient-memory'\nWhen no error was detected, the value doesn't change.  So, to check\nthe error status of a code conversion by this variable, you must\nexplicitly set this variable to nil before performing code\nconversion.  ");
  Vlast_code_conversion_error = Qnil ();

  DEFVAR_BOOL! (c"inhibit-eol-conversion", inhibit_eol_conversion,
	       "\nNon-nil means always inhibit code conversion of end-of-line format.\nSee info node `Coding Systems' and info node `Text and Binary' concerning\nsuch conversion.  ");
  inhibit_eol_conversion = false;

  DEFVAR_BOOL! (c"inherit-process-coding-system", inherit_process_coding_system,
	       "\nNon-nil means process buffer inherits coding system of process output.\nBind it to t if the process output is to be treated as if it were a file\nread from some filesystem.  ");
  inherit_process_coding_system = false;

  DEFVAR_LISP! (c"file-coding-system-alist", Vfile_coding_system_alist,
	       "\nAlist to decide a coding system to use for a file I/O operation.\nThe format is ((PATTERN . VAL) ...),\nwhere PATTERN is a regular expression matching a file name,\nVAL is a coding system, a cons of coding systems, or a function symbol.\nIf VAL is a coding system, it is used for both decoding and encoding\nthe file contents.\nIf VAL is a cons of coding systems, the car part is used for decoding,\nand the cdr part is used for encoding.\nIf VAL is a function symbol, the function must return a coding system\nor a cons of coding systems which are used as above.  The function is\ncalled with an argument that is a list of the arguments with which\n`find-operation-coding-system' was called.  If the function can't decide\na coding system, it can return `undecided' so that the normal\ncode-detection is performed.\n\nSee also the function `find-operation-coding-system'\nand the variable `auto-coding-alist'.  ");
  Vfile_coding_system_alist = Qnil ();

  DEFVAR_LISP! (c"process-coding-system-alist", Vprocess_coding_system_alist,
	       "\nAlist to decide a coding system to use for a process I/O operation.\nThe format is ((PATTERN . VAL) ...),\nwhere PATTERN is a regular expression matching a program name,\nVAL is a coding system, a cons of coding systems, or a function symbol.\nIf VAL is a coding system, it is used for both decoding what received\nfrom the program and encoding what sent to the program.\nIf VAL is a cons of coding systems, the car part is used for decoding,\nand the cdr part is used for encoding.\nIf VAL is a function symbol, the function must return a coding system\nor a cons of coding systems which are used as above.\n\nSee also the function `find-operation-coding-system'.  ");
  Vprocess_coding_system_alist = Qnil ();

  DEFVAR_LISP! (c"network-coding-system-alist", Vnetwork_coding_system_alist,
	       "\nAlist to decide a coding system to use for a network I/O operation.\nThe format is ((PATTERN . VAL) ...),\nwhere PATTERN is a regular expression matching a network service name\nor is a port number to connect to,\nVAL is a coding system, a cons of coding systems, or a function symbol.\nIf VAL is a coding system, it is used for both decoding what received\nfrom the network stream and encoding what sent to the network stream.\nIf VAL is a cons of coding systems, the car part is used for decoding,\nand the cdr part is used for encoding.\nIf VAL is a function symbol, the function must return a coding system\nor a cons of coding systems which are used as above.\n\nSee also the function `find-operation-coding-system'.  ");
  Vnetwork_coding_system_alist = Qnil ();

  DEFVAR_LISP! (c"locale-coding-system", Vlocale_coding_system,
	       " Coding system to use with system messages.\nPotentially also used for decoding keyboard input on X Windows, and is\nused for encoding standard output and error streams.  ");
  Vlocale_coding_system = Qnil ();

  /* The eol mnemonics are reset in startup.el system-dependently.  */
  DEFVAR_LISP! (c"eol-mnemonic-unix", eol_mnemonic_unix,
	       "\nString displayed in mode line for UNIX-like (LF) end-of-line format.  ");
  eol_mnemonic_unix = build_string (c":".as_ptr ());

  DEFVAR_LISP! (c"eol-mnemonic-dos", eol_mnemonic_dos,
	       "\nString displayed in mode line for DOS-like (CRLF) end-of-line format.  ");
  eol_mnemonic_dos = build_string (c"\\".as_ptr ());

  DEFVAR_LISP! (c"eol-mnemonic-mac", eol_mnemonic_mac,
	       "\nString displayed in mode line for MAC-like (CR) end-of-line format.  ");
  eol_mnemonic_mac = build_string (c"/".as_ptr ());

  DEFVAR_LISP! (c"eol-mnemonic-undecided", eol_mnemonic_undecided,
	       "\nString displayed in mode line when end-of-line format is not yet determined.  ");
  eol_mnemonic_undecided = build_string (c":".as_ptr ());

  DEFVAR_LISP! (c"enable-character-translation", Venable_character_translation,
	       "\nNon-nil enables character translation while encoding and decoding.  ");
  Venable_character_translation = Qt ();

  DEFVAR_LISP! (c"standard-translation-table-for-decode", Vstandard_translation_table_for_decode,
	       " Table for translating characters while decoding.  ");
  Vstandard_translation_table_for_decode = Qnil ();

  DEFVAR_LISP! (c"standard-translation-table-for-encode", Vstandard_translation_table_for_encode,
	       " Table for translating characters while encoding.  ");
  Vstandard_translation_table_for_encode = Qnil ();

  DEFVAR_LISP! (c"charset-revision-table", Vcharset_revision_table,
	       " Alist of charsets vs revision numbers.\nWhile encoding, if a charset (car part of an element) is found,\ndesignate it with the escape sequence identifying revision (cdr part\nof the element).  ");
  Vcharset_revision_table = Qnil ();

  DEFVAR_LISP! (c"default-process-coding-system", Vdefault_process_coding_system,
	       " Cons of coding systems used for process I/O by default.\nThe car part is used for decoding a process output,\nthe cdr part is used for encoding a text to be sent to a process.  ");
  Vdefault_process_coding_system = Qnil ();

  DEFVAR_LISP! (c"latin-extra-code-table", Vlatin_extra_code_table,
	       "\nTable of extra Latin codes in the range 128..159 (inclusive).\nThis is a vector of length 256.\nIf Nth element is non-nil, the existence of code N in a file\n\\(or output of subprocess) doesn't prevent it to be detected as\na coding system of ISO 2022 variant which has a flag\n`accept-latin-extra-code' t (e.g. iso-latin-1) on reading a file\nor reading output of a subprocess.\nOnly 128th through 159th elements have a meaning.  ");
  Vlatin_extra_code_table = make_nil_vector (256);

  DEFVAR_LISP! (c"select-safe-coding-system-function", Vselect_safe_coding_system_function,
	       "\nFunction to call to select safe coding system for encoding a text.\n\nIf set, this function is called to force a user to select a proper\ncoding system which can encode the text in the case that a default\ncoding system used in each operation can't encode the text.  The\nfunction should take care that the buffer is not modified while\nthe coding system is being selected.\n\nThe default value is `select-safe-coding-system' (which see).  ");
  Vselect_safe_coding_system_function = Qnil ();

  DEFVAR_BOOL! (c"coding-system-require-warning", coding_system_require_warning,
	       " Internal use only.\nIf non-nil, on writing a file, `select-safe-coding-system-function' is\ncalled even if `coding-system-for-write' is non-nil.  The command\n`universal-coding-system-argument' binds this variable to t temporarily.  ");
  coding_system_require_warning = false;

  DEFVAR_BOOL! (c"inhibit-iso-escape-detection", inhibit_iso_escape_detection,
	       "\nIf non-nil, Emacs ignores ISO-2022 escape sequences during code detection.\n\nWhen Emacs reads text, it tries to detect how the text is encoded.\nThis code detection is sensitive to escape sequences.  If Emacs sees\na valid ISO-2022 escape sequence, it assumes the text is encoded in one\nof the ISO2022 encodings, and decodes text by the corresponding coding\nsystem (e.g. `iso-2022-7bit').\n\nHowever, there may be a case that you want to read escape sequences in\na file as is.  In such a case, you can set this variable to non-nil.\nThen the code detection will ignore any escape sequences, and no text is\ndetected as encoded in some ISO-2022 encoding.  The result is that all\nescape sequences become visible in a buffer.\n\nThe default value is nil, and it is strongly recommended not to change\nit.  That is because many Emacs Lisp source files that contain\nnon-ASCII characters are encoded by the coding system `iso-2022-7bit'\nin Emacs's distribution, and they won't be decoded correctly on\nreading if you suppress escape sequence detection.\n\nThe other way to read escape sequences in a file without decoding is\nto explicitly specify some coding system that doesn't use ISO-2022\nescape sequence (e.g., `latin-1') on reading by \\\\[universal-coding-system-argument].  ");
  inhibit_iso_escape_detection = false;

  DEFVAR_BOOL! (c"inhibit-null-byte-detection", inhibit_null_byte_detection,
	       " If non-nil, Emacs ignores null bytes on code detection.\nBy default, Emacs treats it as binary data, and does not attempt to\ndecode it.  The effect is as if you specified `no-conversion' for\nreading that text.\n\nSet this to non-nil when a regular text happens to include null bytes.\nExamples are Index nodes of Info files and null-byte delimited output\nfrom GNU Find and GNU Grep.  Emacs will then ignore the null bytes and\ndecode text as usual.  ");
  inhibit_null_byte_detection = false;

  DEFVAR_BOOL! (c"disable-ascii-optimization", disable_ascii_optimization,
	       " If non-nil, Emacs does not optimize code decoder for ASCII files.\nInternal use only.  Remove after the experimental optimizer becomes stable.  ");
  disable_ascii_optimization = false;

  DEFVAR_LISP! (c"translation-table-for-input", Vtranslation_table_for_input,
	       " Char table for translating self-inserting characters.\nThis is applied to the result of input methods, not their input.\nSee also `keyboard-translate-table'.\n\nUse of this variable for character code unification was rendered\nobsolete in Emacs 23.1 and later, since Unicode is now the basis of\ninternal character representation.  ");
  Vtranslation_table_for_input = Qnil ();

  let mut args: [Lisp_Object; coding_arg_undecided_max as usize]
    = [lisp_h_Qnil (); coding_arg_undecided_max as usize];
  memclear (args.as_mut_ptr () as *mut c_void, std::mem::size_of_val (&args) as isize);

  /* C's `Lisp_Object plist[] = { … }`, whose entries use the assignment-as-expression
     idiom `args[i] = X` that Rust has no form for; each such entry binds the value first
     and yields it (the only deviation in this list).  */
  let mut plist: [Lisp_Object; 16] =
    [
      QCname (),
      { let v = Qno_conversion (); args[coding_arg_name as usize] = v; v },
      QCmnemonic (),
      { let v = make_fixnum ('=' as EMACS_INT); args[coding_arg_mnemonic as usize] = v; v },
      intern_c_string (c":coding-type".as_ptr ()),
      { let v = Qraw_text (); args[coding_arg_coding_type as usize] = v; v },
      QCascii_compatible_p (),
      { let v = Qt (); args[coding_arg_ascii_compatible_p as usize] = v; v },
      QCdefault_char (),
      { let v = make_fixnum (0); args[coding_arg_default_char as usize] = v; v },
      intern_c_string (c":for-unibyte".as_ptr ()),
      { let v = Qt (); args[coding_arg_for_unibyte as usize] = v; v },
      intern_c_string (c":docstring".as_ptr ()),
      (build_string
       (c"Do no conversion.\n\nWhen you visit a file with this coding, the file is read into a\nunibyte buffer as is, thus each byte of a file is treated as a\ncharacter.".as_ptr ())),
      intern_c_string (c":eol-type".as_ptr ()),
      { let v = Qunix (); args[coding_arg_eol_type as usize] = v; v },
    ];
  args[coding_arg_plist as usize] = CALLMANY! (Flist, plist);
  Fdefine_coding_system_internal (coding_arg_max as isize, args.as_mut_ptr ());

  plist[1] = Qundecided (); args[coding_arg_name as usize] = plist[1];
  plist[3] = make_fixnum ('-' as EMACS_INT); args[coding_arg_mnemonic as usize] = plist[3];
  plist[5] = Qundecided (); args[coding_arg_coding_type as usize] = plist[5];
  /* This is already set.
     plist[7] = args[coding_arg_ascii_compatible_p] = Qt; */
  plist[8] = intern_c_string (c":charset-list".as_ptr ());
  plist[9] = list1 (Qascii ()); args[coding_arg_charset_list as usize] = plist[9];
  plist[11] = Qnil (); args[coding_arg_for_unibyte as usize] = plist[11];
  plist[13] = build_string (c"No conversion on encoding, automatic conversion on decoding.".as_ptr ());
  plist[15] = Qnil (); args[coding_arg_eol_type as usize] = plist[15];
  args[coding_arg_plist as usize] = CALLMANY! (Flist, plist);
  args[coding_arg_undecided_inhibit_null_byte_detection as usize] = make_fixnum (0);
  args[coding_arg_undecided_inhibit_iso_escape_detection as usize] = make_fixnum (0);
  Fdefine_coding_system_internal (coding_arg_undecided_max as isize, args.as_mut_ptr ());

  setup_coding_system (Qno_conversion (), &raw mut safe_terminal_coding);

  {
    let mut i: c_int = 0;
    while i < coding_category_max
      {
	Fset (AREF (Vcoding_category_table, i as isize), Qno_conversion ());
	i += 1;
      }
  }

  pdumper_do_now_and_after_load (Some (reset_coding_after_pdumper_load));

  DEFSYM! (QUnknown_error, c"Unknown error");
  DEFSYM! (Qdefine_coding_system_internal, c"define-coding-system-internal");
}

unsafe extern "C" fn reset_coding_after_pdumper_load ()
{
  if ! dumped_with_pdumper_p ()
    { return; }
  let mut this: *mut coding_system = coding_categories.as_mut_ptr ();
  while this < coding_categories.as_mut_ptr ().offset (coding_category_max as isize)
    {
      let id: c_int = (*this).id as c_int;
      if id >= 0
	{
	  /* Need to rebuild the coding system object because we
	     persisted it as a scalar and it's full of gunk that's now
	     invalid.  */
	  memset (this as *mut c_void, 0, std::mem::size_of::<coding_system> ());
	  setup_coding_system (CODING_ID_NAME (id as hash_idx_t), this);
	}
      this = this.offset (1);
    }
  /* In temacs the below is done by mule-conf.el, because we need to
     define us-ascii first.  But in dumped Emacs us-ascii is restored
     by the above loop, and mule-conf.el will not be loaded, so we set
     it up now; otherwise safe_terminal_coding will remain zeroed.  */
  Fset_safe_terminal_coding_system_internal (Qus_ascii ());
}
