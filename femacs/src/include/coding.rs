// femacs: port of src/coding.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror (PORTING.md P-66): the type layer.  `coding.h` includes `lisp.h` (line 29) and
// `composite.h` (line 284), so `MAX_COMPOSITION_COMPONENTS` comes from composite.rs; the two
// inlines and the 62 `CODING_*` macros in docs/ports/include-coding.md arrive with coding.c
// (batch C).  `tools/check-cond-eval.py` reports zero disagreements for this header, so the
// ledger's branch set is the frozen configuration's.

// coding.h 27: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_CODING_H: bool = true;

/* `struct ccl_spec` is defined in ccl.h 69-71, which coding.h does not include: C names it
   only as a pointer inside `coding_system.spec`, and an undeclared tag is an incomplete type
   there.  Rust has no incomplete types, so the port declares the stand-in here and deletes it
   when ccl.rs lands (PORTING.md P-70; recorded in the ledger's deviations).  Only the pointer
   is ever used, so the stand-in's own size never enters a layout.  */
pub struct ccl_spec;

// coding.h 252-259: `enum coding_result_code`.
pub type coding_result_code = c_int;
pub const CODING_RESULT_SUCCESS: coding_result_code = 0;
pub const CODING_RESULT_INSUFFICIENT_SRC: coding_result_code = 1;
pub const CODING_RESULT_INSUFFICIENT_DST: coding_result_code = 2;
pub const CODING_RESULT_INVALID_SRC: coding_result_code = 3;
pub const CODING_RESULT_INTERRUPT: coding_result_code = 4;

// coding.h 286-294: `enum composition_state`.
pub type composition_state = c_int;
pub const COMPOSING_NO: composition_state = 0;
pub const COMPOSING_CHAR: composition_state = 1;
pub const COMPOSING_RULE: composition_state = 2;
pub const COMPOSING_COMPONENT_CHAR: composition_state = 3;
pub const COMPOSING_COMPONENT_RULE: composition_state = 4;

// coding.h 364-369 / 371-375.
pub type utf_bom_type = c_int;
pub const utf_detect_bom: utf_bom_type = 0;
pub const utf_without_bom: utf_bom_type = 1;
pub const utf_with_bom: utf_bom_type = 2;

pub type utf_16_endian_type = c_int;
pub const utf_16_big_endian: utf_16_endian_type = 0;
pub const utf_16_little_endian: utf_16_endian_type = 1;

// coding.h 296-312.  Twelve bytes of counters and then `carryover`, whose bound C spells as
// `4 + MAX_COMPOSITION_COMPONENTS * 3 - 2 + 2 + MAX_COMPOSITION_COMPONENTS` = 68 (272 bytes
// at offset 24; sizeof 296, align 4 — measured).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct composition_status
{
  pub state: composition_state,                        // coding.h 298
  pub method: composition_method,                      // 299
  pub old_form: bool,                                  // 300
  pub length: c_int,                                   // 301
  pub nchars: c_int,                                   // 302
  pub ncomps: c_int,                                   // 303
  pub carryover: [c_int; (4 + MAX_COMPOSITION_COMPONENTS as usize * 3 - 2 + 2
                           + MAX_COMPOSITION_COMPONENTS as usize)],   // 306-310
}

// coding.h 316-344.  `flags`, the two register arrays and the segment length are four-byte
// members (0/4/12/28); the three `bool_bf : 1` members share a 4-byte unit at 32, and
// `cmp_status` follows at 36 (sizeof 332, align 4 — measured).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct iso_2022_spec
{
  pub flags: c_uint,                                   // coding.h 319
  pub current_invocation: [c_int; 2],                  // 322
  pub current_designation: [c_int; 4],                 // 327
  pub ctext_extended_segment_len: c_int,               // 331
  pub bits: u32,                                       // single_shifting:1 | bol:1 | embedded_utf_8:1
  pub cmp_status: composition_status,                  // 343
}

// coding.h 346-349.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct emacs_mule_spec
{
  pub cmp_status: composition_status,                  // coding.h 348
}

// coding.h 351-361.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct undecided_spec
{
  pub inhibit_nbd: c_int,                              // coding.h 354
  pub inhibit_ied: c_int,                              // 357
  pub prefer_utf_8: bool,                              // 360
}

// coding.h 377-382.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct utf_16_spec
{
  pub bom: utf_bom_type,                               // coding.h 379
  pub endian: utf_16_endian_type,                      // 380
  pub surrogate: c_int,                                // 381
}

// coding.h 444-453: C's anonymous union.  Its size is 336 (the widest member,
// `iso_2022_spec`'s 332, rounded up to the union's 8-byte alignment, which the `ccl` pointer
// sets) and it sits at offset 16 — measured.
#[repr(C)]
pub union coding_system_spec
{
  pub iso_2022: iso_2022_spec,                         // coding.h 446
  pub ccl: *mut ccl_spec,                              // 447 (`struct ccl_spec *`)
  pub utf_16: utf_16_spec,                             // 448
  pub utf_8_bom: utf_bom_type,                         // 449
  pub emacs_mule: emacs_mule_spec,                     // 450
  pub undecided: undecided_spec,                       // 451
}

// coding.h 396-521.  The flag word after `id` packs ten members into 31 bits of one 4-byte
// unit (offset 8): `common_flags:14`, `mode:5`, then six `bool_bf : 1`, `eol_seen:3` and
// `ENUM_BF (coding_result_code) result:3`.  Measured with all six bools set, `mode` 7,
// `common_flags` 5, `eol_seen` 3 and `result` 4:
//
//     bytes 8..11 : 05 c0 f9 47      (= 0x47f9c005)
//
// so the port stores the unit as `bits: u32` and keeps the field reads/writes in the
// accessors (PORTING.md 1.3: the storage unit is what the gate can assert).
#[repr(C)]
pub struct coding_system
{
  pub id: ptrdiff_t,                                   // coding.h 404
  pub bits: u32,                                       // 408-440
  pub max_charset_id: c_int,                           // 442
  pub spec: coding_system_spec,                        // 453
  pub safe_charsets: *mut u8,                          // 455
  pub head_ascii: ptrdiff_t,                           // 462
  pub detected_utf8_bytes: ptrdiff_t,                  // 466
  pub detected_utf8_chars: ptrdiff_t,
  pub produced: ptrdiff_t,                             // 469
  pub produced_char: ptrdiff_t,
  pub consumed: ptrdiff_t,
  pub consumed_char: ptrdiff_t,
  pub src_pos: ptrdiff_t,                              // 471
  pub src_pos_byte: ptrdiff_t,
  pub src_chars: ptrdiff_t,
  pub src_bytes: ptrdiff_t,
  pub src_object: Lisp_Object,                         // 472
  pub source: *const u8,                               // 473 (`const unsigned char *`)
  pub dst_pos: ptrdiff_t,                              // 475
  pub dst_pos_byte: ptrdiff_t,
  pub dst_bytes: ptrdiff_t,
  pub dst_object: Lisp_Object,                         // 476
  pub destination: *mut u8,                            // 477
  pub charbuf: *mut c_int,                             // 490
  pub charbuf_size: c_int,                             // 491
  pub charbuf_used: c_int,
  pub carryover: [u8; 64],                             // 493
  pub carryover_bytes: c_int,                          // 494
  pub default_char: c_int,                             // 496
  pub detector: Option<unsafe extern "C" fn (*mut coding_system,
                                              *mut coding_detection_info) -> bool>,  // 498
  pub decoder: Option<unsafe extern "C" fn (*mut coding_system)>,                    // 499
  pub encoder: Option<unsafe extern "C" fn (*mut coding_system) -> bool>,            // 500
}

// coding.h 384-392: the detection bookkeeping `detector` fills in.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct coding_detection_info
{
  pub checked: c_int,                                  // coding.h 388
  pub found: c_int,                                    // 390
  pub rejected: c_int,                                 // 392
}

// coding.h 100-141: `enum coding_attr_index` — C's members carry no initialiser, so
// their values are the declaration order (the constant gate compares them).
pub type coding_attr_index = c_int;
pub const coding_attr_base_name: coding_attr_index = 0;
pub const coding_attr_docstring: coding_attr_index = 1;
pub const coding_attr_mnemonic: coding_attr_index = 2;
pub const coding_attr_type: coding_attr_index = 3;
pub const coding_attr_charset_list: coding_attr_index = 4;
pub const coding_attr_ascii_compat: coding_attr_index = 5;
pub const coding_attr_decode_tbl: coding_attr_index = 6;
pub const coding_attr_encode_tbl: coding_attr_index = 7;
pub const coding_attr_trans_tbl: coding_attr_index = 8;
pub const coding_attr_post_read: coding_attr_index = 9;
pub const coding_attr_pre_write: coding_attr_index = 10;
pub const coding_attr_default_char: coding_attr_index = 11;
pub const coding_attr_for_unibyte: coding_attr_index = 12;
pub const coding_attr_plist: coding_attr_index = 13;
pub const coding_attr_category: coding_attr_index = 14;
pub const coding_attr_safe_charsets: coding_attr_index = 15;
pub const coding_attr_charset_valids: coding_attr_index = 16;
pub const coding_attr_ccl_decoder: coding_attr_index = 17;
pub const coding_attr_ccl_encoder: coding_attr_index = 18;
pub const coding_attr_ccl_valids: coding_attr_index = 19;
pub const coding_attr_iso_initial: coding_attr_index = 20;
pub const coding_attr_iso_usage: coding_attr_index = 21;
pub const coding_attr_iso_request: coding_attr_index = 22;
pub const coding_attr_iso_flags: coding_attr_index = 23;
pub const coding_attr_utf_bom: coding_attr_index = 24;
pub const coding_attr_utf_16_endian: coding_attr_index = 25;
pub const coding_attr_emacs_mule_full: coding_attr_index = 26;
pub const coding_attr_undecided_inhibit_null_byte_detection: coding_attr_index = 27;
pub const coding_attr_undecided_inhibit_iso_escape_detection: coding_attr_index = 28;
pub const coding_attr_undecided_prefer_utf_8: coding_attr_index = 29;
pub const coding_attr_last_index: coding_attr_index = 30;

// coding.h 267-281: the bits of `mode` (used by CODING_REQUIRE_ENCODING below).
pub const CODING_MODE_LAST_BLOCK: c_int = 0x01;              // coding.h 267
pub const CODING_MODE_SELECTIVE_DISPLAY: c_int = 0x02;       // 271
pub const CODING_MODE_DIRECTION: c_int = 0x04;               // 275
pub const CODING_MODE_FIXED_DESTINATION: c_int = 0x08;       // 277
pub const CODING_MODE_SAFE_ENCODING: c_int = 0x10;           // 281

// coding.h 507-516: the bits of `common_flags` (the low 8 are the annotation kinds).
pub const CODING_ANNOTATION_MASK: c_uint = 0x00FF;           // coding.h 507
pub const CODING_ANNOTATE_COMPOSITION_MASK: c_uint = 0x0001; // 508
pub const CODING_ANNOTATE_DIRECTION_MASK: c_uint = 0x0002;   // 509
pub const CODING_ANNOTATE_CHARSET_MASK: c_uint = 0x0003;     // 510
pub const CODING_FOR_UNIBYTE_MASK: c_uint = 0x0100;          // 511
pub const CODING_REQUIRE_FLUSHING_MASK: c_uint = 0x0200;     // 512
pub const CODING_REQUIRE_DECODING_MASK: c_uint = 0x0400;     // 513
pub const CODING_REQUIRE_ENCODING_MASK: c_uint = 0x0800;     // 514
pub const CODING_REQUIRE_DETECTION_MASK: c_uint = 0x1000;    // 515
pub const CODING_RESET_AT_BOL_MASK: c_uint = 0x2000;         // 516

// `coding_system.bits` (offset 8) holds the ten members C declares between `id` and
// `max_charset_id`.  The measured word for `common_flags` 5, `mode` 7, all six bools set,
// `eol_seen` 3 and `result` 4 is 0x47f9c005 (`05 c0 f9 47` little-endian), which fixes every
// field's window; the port names them the way `glyph_slice`'s four members are named
// (PORTING.md 1.3).  C reads and writes the bitfields directly; the port masks.
pub const CODING_SYSTEM_COMMON_FLAGS_SHIFT: u32 = 0;
pub const CODING_SYSTEM_COMMON_FLAGS_MASK: u32 = 0x3FFF;     // 14 bits
pub const CODING_SYSTEM_MODE_SHIFT: u32 = 14;
pub const CODING_SYSTEM_MODE_MASK: u32 = 0x1F;               // 5 bits
pub const CODING_SYSTEM_SRC_MULTIBYTE_SHIFT: u32 = 19;
pub const CODING_SYSTEM_DST_MULTIBYTE_SHIFT: u32 = 20;
pub const CODING_SYSTEM_CHARS_AT_SOURCE_SHIFT: u32 = 21;
pub const CODING_SYSTEM_RAW_DESTINATION_SHIFT: u32 = 22;
pub const CODING_SYSTEM_ANNOTATED_SHIFT: u32 = 23;
pub const CODING_SYSTEM_INSERT_BEFORE_MARKERS_SHIFT: u32 = 24;
pub const CODING_SYSTEM_EOL_SEEN_SHIFT: u32 = 25;
pub const CODING_SYSTEM_EOL_SEEN_MASK: u32 = 0x07;           // 3 bits
pub const CODING_SYSTEM_RESULT_SHIFT: u32 = 28;
pub const CODING_SYSTEM_RESULT_MASK: u32 = 0x07;             // 3 bits

// C reaches `coding->src_multibyte`, `->dst_multibyte` and `->mode` as bitfields; the port's
// storage is one unit, so these three read it (the writers land with coding.c, where C
// assigns them).
#[inline(always)]
fn src_multibyte_p (coding: *mut coding_system) -> bool
{ (unsafe { (*coding).bits }) >> CODING_SYSTEM_SRC_MULTIBYTE_SHIFT & 1 != 0 }

#[inline(always)]
fn dst_multibyte_p (coding: *mut coding_system) -> bool
{ (unsafe { (*coding).bits }) >> CODING_SYSTEM_DST_MULTIBYTE_SHIFT & 1 != 0 }

#[inline(always)]
fn mode_of (coding: *mut coding_system) -> c_uint
{ (unsafe { (*coding).bits }) >> CODING_SYSTEM_MODE_SHIFT & CODING_SYSTEM_MODE_MASK }

// coding.h 520-558.  C's macros yield an `int` (the masked bits, or a `||` of them), so the
// port returns `c_int` and call sites test it against zero exactly as C does.
#[inline(always)]
pub fn CODING_REQUIRE_ANNOTATION (coding: *mut coding_system) -> c_int
{ (unsafe { (*coding).bits }) as c_int & CODING_ANNOTATION_MASK as c_int }

#[inline(always)]
pub fn CODING_FOR_UNIBYTE (coding: *mut coding_system) -> c_int
{ (unsafe { (*coding).bits }) as c_int & CODING_FOR_UNIBYTE_MASK as c_int }

#[inline(always)]
pub fn CODING_REQUIRE_FLUSHING (coding: *mut coding_system) -> c_int
{ (unsafe { (*coding).bits }) as c_int & CODING_REQUIRE_FLUSHING_MASK as c_int }

#[inline(always)]
pub fn CODING_REQUIRE_DECODING (coding: *mut coding_system) -> c_int
{ (dst_multibyte_p (coding) as c_int)
  | ((unsafe { (*coding).bits }) as c_int & CODING_REQUIRE_DECODING_MASK as c_int) }

#[inline(always)]
pub fn CODING_REQUIRE_ENCODING (coding: *mut coding_system) -> c_int
{ (src_multibyte_p (coding) as c_int)
  | ((unsafe { (*coding).bits }) as c_int & CODING_REQUIRE_ENCODING_MASK as c_int)
  | (mode_of (coding) as c_int & CODING_MODE_SELECTIVE_DISPLAY) }

#[inline(always)]
pub fn CODING_REQUIRE_DETECTION (coding: *mut coding_system) -> c_int
{ (unsafe { (*coding).bits }) as c_int & CODING_REQUIRE_DETECTION_MASK as c_int }

#[inline(always)]
pub fn CODING_MAY_REQUIRE_DECODING (coding: *mut coding_system) -> c_int
{ CODING_REQUIRE_DECODING (coding) | CODING_REQUIRE_DETECTION (coding) }

// coding.h 674-682: C's comparisons yield an `int`; the port yields the same truth value.
#[inline(always)]
pub fn UTF_16_HIGH_SURROGATE_P (val: c_int) -> c_int
{ ((val & 0xFC00) == 0xD800) as c_int }

#[inline(always)]
pub fn UTF_16_LOW_SURROGATE_P (val: c_int) -> c_int
{ ((val & 0xFC00) == 0xDC00) as c_int }

// coding.h 704: the prototype `build_string_from_utf8` calls (coding.c defines it, batch C).
unsafe extern "C" { pub fn make_string_from_utf8 (str: *const c_char, len: ptrdiff_t) -> Lisp_Object; }

/* Return the Unicode code point for the given UTF-16 surrogates.  */
// coding.h 756-767.
#[inline(always)]
pub fn surrogates_to_codepoint (low: c_int, high: c_int) -> c_int
{
  eassert! (0 <= low && low <= 0xFFFF);
  eassert! (0 <= high && high <= 0xFFFF);
  eassert! (UTF_16_LOW_SURROGATE_P (low) != 0);
  eassert! (UTF_16_HIGH_SURROGATE_P (high) != 0);
  0x10000 + (low - 0xDC00) + ((high - 0xD800) * 0x400)
}

/* Like build_string, but always returns a multibyte string, and is
   optimized for speed when STR is a UTF-8 encoded text string.  */
// coding.h 771-778.
#[inline(always)]
pub fn build_string_from_utf8 (str: *const c_char) -> Lisp_Object
{
  unsafe { make_string_from_utf8 (str, strlen (str) as ptrdiff_t) }
}

// coding.h 98/784/786: the file-scope objects (coding.c defines them).
unsafe extern "C" { pub static mut Vcoding_system_hash_table: Lisp_Object; }
unsafe extern "C" { pub static mut safe_terminal_coding: coding_system; }
unsafe extern "C" { pub static mut emacs_mule_bytes: [c_char; 256]; }

// coding.h 35-83: the six `enum define_coding_*_arg_index` sets (the argument indices of
// `define-coding-system` and its per-type extensions).  C gives them no initialisers except
// each block's `= coding_arg_max` alias and the members after it, whose values follow from
// C's enum rule; the port spells the aliases as C does and the implicit members as their
// value.
pub type define_coding_system_arg_index = c_int;

pub const coding_arg_name: c_int = 0;
pub const coding_arg_mnemonic: c_int = 1;
pub const coding_arg_coding_type: c_int = 2;
pub const coding_arg_charset_list: c_int = 3;
pub const coding_arg_ascii_compatible_p: c_int = 4;
pub const coding_arg_decode_translation_table: c_int = 5;
pub const coding_arg_encode_translation_table: c_int = 6;
pub const coding_arg_post_read_conversion: c_int = 7;
pub const coding_arg_pre_write_conversion: c_int = 8;
pub const coding_arg_default_char: c_int = 9;
pub const coding_arg_for_unibyte: c_int = 10;
pub const coding_arg_plist: c_int = 11;
pub const coding_arg_eol_type: c_int = 12;
pub const coding_arg_max: c_int = 13;
pub const coding_arg_iso2022_initial: c_int = coding_arg_max;
pub const coding_arg_iso2022_reg_usage: c_int = 14;
pub const coding_arg_iso2022_request: c_int = 15;
pub const coding_arg_iso2022_flags: c_int = 16;
pub const coding_arg_iso2022_max: c_int = 17;
pub const coding_arg_utf8_bom: c_int = coding_arg_max;
pub const coding_arg_utf8_max: c_int = 14;
pub const coding_arg_utf16_bom: c_int = coding_arg_max;
pub const coding_arg_utf16_endian: c_int = 14;
pub const coding_arg_utf16_max: c_int = 15;
pub const coding_arg_ccl_decoder: c_int = coding_arg_max;
pub const coding_arg_ccl_encoder: c_int = 14;
pub const coding_arg_ccl_valids: c_int = 15;
pub const coding_arg_ccl_max: c_int = 16;
pub const coding_arg_undecided_inhibit_null_byte_detection: c_int = coding_arg_max;
pub const coding_arg_undecided_inhibit_iso_escape_detection: c_int = 14;
pub const coding_arg_undecided_prefer_utf_8: c_int = 15;
pub const coding_arg_undecided_max: c_int = 16;

/* Macros to access an element of an attribute vector.  */
// coding.h 149-164.  C's macros are `AREF (attrs, coding_attr_*)`; the port keeps the names
// and the index enumeration.
#[inline(always)]
pub fn CODING_ATTR_BASE_NAME (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_base_name as isize) }
#[inline(always)]
pub fn CODING_ATTR_TYPE (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_type as isize) }
#[inline(always)]
pub fn CODING_ATTR_CHARSET_LIST (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_charset_list as isize) }
#[inline(always)]
pub fn CODING_ATTR_MNEMONIC (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_mnemonic as isize) }
#[inline(always)]
pub fn CODING_ATTR_DOCSTRING (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_docstring as isize) }
#[inline(always)]
pub fn CODING_ATTR_ASCII_COMPAT (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_ascii_compat as isize) }
#[inline(always)]
pub fn CODING_ATTR_DECODE_TBL (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_decode_tbl as isize) }
#[inline(always)]
pub fn CODING_ATTR_ENCODE_TBL (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_encode_tbl as isize) }
#[inline(always)]
pub fn CODING_ATTR_TRANS_TBL (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_trans_tbl as isize) }
#[inline(always)]
pub fn CODING_ATTR_POST_READ (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_post_read as isize) }
#[inline(always)]
pub fn CODING_ATTR_PRE_WRITE (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_pre_write as isize) }
#[inline(always)]
pub fn CODING_ATTR_DEFAULT_CHAR (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_default_char as isize) }
#[inline(always)]
pub fn CODING_ATTR_FOR_UNIBYTE (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_for_unibyte as isize) }
#[inline(always)]
pub fn CODING_ATTR_PLIST (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_plist as isize) }
#[inline(always)]
pub fn CODING_ATTR_CATEGORY (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_category as isize) }
#[inline(always)]
pub fn CODING_ATTR_SAFE_CHARSETS (attrs: Lisp_Object) -> Lisp_Object
{ AREF (attrs, coding_attr_safe_charsets as isize) }

/* Return the name of a coding system specified by ID.  */
// coding.h 168-186: `id` is a hash index, so these take `hash_idx_t` as C passes it.
#[inline(always)]
pub fn CODING_ID_NAME (id: hash_idx_t) -> Lisp_Object
{ HASH_KEY (unsafe { XHASH_TABLE (Vcoding_system_hash_table) }, id) }
#[inline(always)]
pub fn CODING_ID_ATTRS (id: hash_idx_t) -> Lisp_Object
{ AREF (HASH_VALUE (unsafe { XHASH_TABLE (Vcoding_system_hash_table) }, id), 0) }
#[inline(always)]
pub fn CODING_ID_ALIASES (id: hash_idx_t) -> Lisp_Object
{ AREF (HASH_VALUE (unsafe { XHASH_TABLE (Vcoding_system_hash_table) }, id), 1) }
#[inline(always)]
pub fn CODING_ID_EOL_TYPE (id: hash_idx_t) -> Lisp_Object
{ AREF (HASH_VALUE (unsafe { XHASH_TABLE (Vcoding_system_hash_table) }, id), 2) }

/* Return the spec vector of CODING_SYSTEM_SYMBOL.  */
#[inline(always)]
pub fn CODING_SYSTEM_SPEC (coding_system_symbol: Lisp_Object) -> Lisp_Object
{ unsafe { Fgethash (coding_system_symbol, Vcoding_system_hash_table, lisp_h_Qnil ()) } }

/* Return the ID of CODING_SYSTEM_SYMBOL.  */
#[inline(always)]
pub fn CODING_SYSTEM_ID (coding_system_symbol: Lisp_Object) -> ptrdiff_t
{ unsafe { hash_find (XHASH_TABLE (Vcoding_system_hash_table), coding_system_symbol) } }

/* Return true if CODING_SYSTEM_SYMBOL is a coding system.  */
#[inline(always)]
pub fn CODING_SYSTEM_P (coding_system_symbol: Lisp_Object) -> bool
{ CODING_SYSTEM_ID (coding_system_symbol) >= 0
  || (!NILP (coding_system_symbol)
      && !NILP (unsafe { Fcoding_system_p (coding_system_symbol) })) }

/* Check if X is a coding system or not.  */
// coding.h 207-213.
#[inline(always)]
pub fn CHECK_CODING_SYSTEM (x: Lisp_Object)
{
  if CODING_SYSTEM_ID (x) < 0 && NILP (unsafe { Fcheck_coding_system (x) })
    { unsafe { wrong_type_argument (Qcoding_system_p (), x) }; }
}

/* Check if X is a coding system or not.  If it is, set SPEC to the
   spec vector of the coding system.  */
// coding.h 218-230.  C writes SEPC through an out-parameter; the port returns it (the same
// value the C macro leaves in the caller's variable).
#[inline(always)]
pub fn CHECK_CODING_SYSTEM_GET_SPEC (x: Lisp_Object) -> Lisp_Object
{
  let mut spec = CODING_SYSTEM_SPEC (x);
  if NILP (spec)
    {
      unsafe { Fcheck_coding_system (x) };
      spec = CODING_SYSTEM_SPEC (x);
    }
  if NILP (spec) { unsafe { wrong_type_argument (Qcoding_system_p (), x) }; }
  spec
}

/* Check if X is a coding system or not.  If it is, set ID to the
   ID of the coding system.  */
// coding.h 234-247 (out-parameter `id` becomes the return value).
#[inline(always)]
pub fn CHECK_CODING_SYSTEM_GET_ID (x: Lisp_Object) -> ptrdiff_t
{
  let mut id = CODING_SYSTEM_ID (x);
  if id < 0
    {
      unsafe { Fcheck_coding_system (x) };
      id = CODING_SYSTEM_ID (x);
    }
  if id < 0 { unsafe { wrong_type_argument (Qcoding_system_p (), x) }; }
  id
}

// coding.h 691-713: the prototypes these wrappers call (coding.c defines them, batch C).
unsafe extern "C" { pub fn code_convert_string_norecord (string: Lisp_Object,
                                                         coding_system: Lisp_Object,
                                                         nocopy: bool) -> Lisp_Object; }
unsafe extern "C" { pub fn encode_file_name (name: Lisp_Object) -> Lisp_Object; }   // coding.h 698
unsafe extern "C" { pub fn decode_file_name (name: Lisp_Object) -> Lisp_Object; }   // coding.h 699
unsafe extern "C" { pub fn decode_coding_gap (coding: *mut coding_system, nbytes: ptrdiff_t); }
unsafe extern "C" { pub fn decode_coding_object (coding: *mut coding_system, src_object: Lisp_Object,
                                                 from: ptrdiff_t, from_byte: ptrdiff_t, to: ptrdiff_t,
                                                 to_byte: ptrdiff_t, dst_object: Lisp_Object); }
unsafe extern "C" { pub fn encode_coding_object (coding: *mut coding_system, src_object: Lisp_Object,
                                                 from: ptrdiff_t, from_byte: ptrdiff_t, to: ptrdiff_t,
                                                 to_byte: ptrdiff_t, dst_object: Lisp_Object); }

/* Encode the file name NAME using the specified coding system
   for file names, if any.  May return NAME itself.  */
// coding.h 648-652.
#[inline(always)]
pub fn ENCODE_FILE (name: Lisp_Object) -> Lisp_Object { unsafe { encode_file_name (name) } }
#[inline(always)]
pub fn DECODE_FILE (name: Lisp_Object) -> Lisp_Object { unsafe { decode_file_name (name) } }

/* Encode the string STR using the specified coding system
   for system functions, if any.  */
// coding.h 656-669.  C's conditional expression becomes an `if`: both arms return the same
// value the C macro does.
#[inline(always)]
pub fn ENCODE_SYSTEM (str: Lisp_Object) -> Lisp_Object
{
  if !NILP (unsafe { Vlocale_coding_system })
    { unsafe { code_convert_string_norecord (str, Vlocale_coding_system, true) } }
  else { str }
}

#[inline(always)]
pub fn DECODE_SYSTEM (str: Lisp_Object) -> Lisp_Object
{
  if !NILP (unsafe { Vlocale_coding_system })
    { unsafe { code_convert_string_norecord (str, Vlocale_coding_system, false) } }
  else { str }
}

/* Note that this encodes utf-8, not utf-8-emacs, so it's not a no-op.  */
#[inline(always)]
pub fn ENCODE_UTF_8 (str: Lisp_Object) -> Lisp_Object
{ unsafe { code_convert_string_norecord (str, Qutf_8 (), true) } }

/* Macros for backward compatibility.  */
// coding.h 743-746.  C's conditional expression evaluates the multibyte arm for its effect and
// yields `(coding)->dst_object`.
#[inline(always)]
pub fn encode_coding_string (coding: *mut coding_system, string: Lisp_Object, nocopy: bool)
  -> Lisp_Object
{
  if STRING_MULTIBYTE (string)
    {
      unsafe {
        encode_coding_object (coding, string, 0, 0, SCHARS (string), SBYTES (string), Qt ());
        (*coding).dst_object
      }
    }
  else { string }
}

// coding.h 750-756.  C writes it as `do { … } while (false)`; Rust has no do-while, so the
// body runs once (the macro-body note records the missing `false` token).
#[inline(always)]
pub fn decode_coding_c_string (coding: *mut coding_system, src: *mut u8, bytes: ptrdiff_t,
                               dst_object: Lisp_Object)
{
  unsafe {
    (*coding).source = src;
    (*coding).src_chars = bytes;
    (*coding).src_bytes = bytes;
    decode_coding_object (coding, lisp_h_Qnil (), 0, 0, bytes, bytes, dst_object);
  }
}

/* Macros to convert between SJIS and JIS (the ISO-2022-JP graphic registers).  C writes
   these as `do { … } while (false)` statement macros that rewrite their `code` argument;
   the port returns the transformed code (C's out-parameter) and the macro-body note records
   the `false` token it cannot mention.  */
// coding.h 567-580.
#[inline(always)]
pub fn SJIS_TO_JIS (code: c_int) -> c_int
{
  let s1 = code >> 8;
  let s2 = code & 0xFF;
  let (j1, j2);
  if s2 >= 0x9F
    {
      j1 = s1 * 2 - (if s1 >= 0xE0 { 0x160 } else { 0xE0 });
      j2 = s2 - 0x7E;
    }
  else
    {
      j1 = s1 * 2 - (if s1 >= 0xE0 { 0x161 } else { 0xE1 });
      j2 = s2 - (if s2 >= 0x7F { 0x20 } else { 0x1F });
    }
  (j1 << 8) | j2
}

// coding.h 582-606.
#[inline(always)]
pub fn SJIS_TO_JIS2 (code: c_int) -> c_int
{
  let s1 = code >> 8;
  let s2 = code & 0xFF;
  let (j1, j2);
  if s2 >= 0x9F
    {
      j1 = if s1 == 0xF0 { 0x28 }
           else if s1 == 0xF1 { 0x24 }
           else if s1 == 0xF2 { 0x2C }
           else if s1 == 0xF3 { 0x2E }
           else { 0x6E + (s1 - 0xF4) * 2 };
      j2 = s2 - 0x7E;
    }
  else
    {
      j1 = if s1 <= 0xF2 { 0x21 + (s1 - 0xF0) * 2 }
           else if s1 <= 0xF4 { 0x2D + (s1 - 0xF3) * 2 }
           else { 0x6F + (s1 - 0xF5) * 2 };
      j2 = s2 - (if s2 >= 0x7F { 0x20 } else { 0x1F });
    }
  (j1 << 8) | j2
}

// coding.h 608-621.
#[inline(always)]
pub fn JIS_TO_SJIS (code: c_int) -> c_int
{
  let j1 = code >> 8;
  let j2 = code & 0xFF;
  let (s1, s2);
  if j1 & 1 != 0
    {
      s1 = j1 / 2 + (if j1 < 0x5F { 0x71 } else { 0xB1 });
      s2 = j2 + (if j2 >= 0x60 { 0x20 } else { 0x1F });
    }
  else
    {
      s1 = j1 / 2 + (if j1 < 0x5F { 0x70 } else { 0xB0 });
      s2 = j2 + 0x7E;
    }
  (s1 << 8) | s2
}

// coding.h 622-645.
#[inline(always)]
pub fn JIS_TO_SJIS2 (code: c_int) -> c_int
{
  let j1 = code >> 8;
  let j2 = code & 0xFF;
  let (s1, s2);
  if j1 & 1 != 0
    {
      s1 = if j1 <= 0x25 { 0xF0 + (j1 - 0x21) / 2 }
           else if j1 <= 0x2F { 0xF3 + (j1 - 0x2D) / 2 }
           else { 0xF5 + (j1 - 0x6F) / 2 };
      s2 = j2 + (if j2 >= 0x60 { 0x20 } else { 0x1F });
    }
  else
    {
      s1 = if j1 == 0x28 { 0xF0 }
           else if j1 == 0x24 { 0xF1 }
           else if j1 == 0x2C { 0xF2 }
           else if j1 == 0x2E { 0xF3 }
           else { 0xF4 + (j1 - 0x6E) / 2 };
      s2 = j2 + 0x7E;
    }
  (s1 << 8) | s2
}
