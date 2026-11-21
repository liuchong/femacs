// femacs: port of src/chartab.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  The thirteen
// `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's ten non-`static` definitions
// (`copy_char_table`, `char_table_ref`, `char_table_set`, `char_table_ref_and_range`,
// `char_table_set_range`, `map_char_table`, `map_char_table_for_charset`, `uniprop_table`,
// `get_unicode_property`, `syms_of_chartab`) and its one non-`static` object
// (`chartab_size`) are external symbols, so they carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2); the twenty-one `static` helpers stay plain `unsafe fn`, except
// `uniprop_decode_value_run_length` and the three `uniprop_encode_value_*` callbacks, which
// are `unsafe extern "C" fn` because C puts their addresses in the
// `uniprop_decoder`/`uniprop_encoder` tables (PORTING.md 1.4).  C's three forward
// declarations (C 64-68) have no Rust counterpart and are deleted, the definitions keeping
// their C place (PORTING.md 0.1-12).
//
// The `#include` list is C 21-26's: `config.h` (21), lisp.h (23) - which brings
// `<stdio.h>` (lisp.h 34) and `<stdcountof.h>` (lisp.h 28, `countof`) - `character.h` (24),
// `charset.h` (25) and `coding.h` (26).  `charset.h` has no mirror yet
// (docs/port-map.md still lists `src/include/charset.rs` as unstarted), so the names
// this unit's live text takes from it are declared at their use site below, with C's own
// bodies (PORTING.md 0.1-2, P-72); they move to the mirror when charset.c lands.  The
// `coding.rs` mirror does not carry `coding_inherit_eol_type` yet, so it too is declared at
// its use site.
//
// The one dead region is the `#if 0` arm of `uniprop_table_uncompress` (C 1139-1144, the
// WORD-LIST table C's own comment says it does not need); it is recorded in
// docs/ports/chartab.md rather than transcribed (P-08).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/stdcountof.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/composite.rs imported via common

/* The names C 25's `charset.h` gives this unit, written at their use site because that
   header has no mirror yet (PORTING.md 0.1-2).  `docs/port-map.md` still lists
   `src/include/charset.rs` as unstarted; this block moves there when charset.c
   lands, and the ledger records the decision.  */

/* Methods for converting code points and characters of charsets.  */
// charset.h 130-146: `enum charset_method`.
pub type charset_method = c_int;
pub const CHARSET_METHOD_OFFSET: charset_method = 0;
pub const CHARSET_METHOD_MAP: charset_method = 1;
pub const CHARSET_METHOD_SUBSET: charset_method = 2;
pub const CHARSET_METHOD_SUPERSET: charset_method = 3;

// charset.h 62-75: `enum charset_attr_index`.  Only the two members ENCODE_CHAR's
// `CHARSET_ENCODER` reaches are written here, in C's declaration order.
pub type charset_attr_index = c_int;
pub const charset_id: charset_attr_index = 0;
pub const charset_encoder: charset_attr_index = 5;

/// C: `struct charset` charset.h 148-243.  The six `bool_bf` members are one 1-byte
/// storage unit in C (`bool_bf` is `bool` on this target, `conf_post.h` 56-61), so the port
/// packs them into one byte named `bits` (PORTING.md 1.3); clang's record layout on the
/// frozen target puts `id` at 0, `dimension` at 4, `code_space` at 8, `code_space_mask` at
/// 72, the six bits at 80:0-80:5, `iso_final` at 84, `iso_revision` at 88, `emacs_mule_id`
/// at 92, `method` at 96, `min_code` at 100, `max_code` at 104, `char_index_offset` at 108,
/// `min_char` at 112, `max_char` at 116, `invalid_code` at 120, `fast_map` at 124,
/// `code_offset` at 316 and `sizeof (struct charset)` at 320/align 8 (measured; the ledger
/// carries the command and the numbers).
#[repr(C)]
pub struct charset
{
  /* Index to charset_table.  */
  pub id: c_int,

  /* Dimension of the charset: 1, 2, 3, or 4.  */
  pub dimension: c_int,

  /* Byte code range of each dimension.  <code_space>[4N] is a minimum
     byte code of the (N+1)th dimension, <code_space>[4N+1] is a
     maximum byte code of the (N+1)th dimension, <code_space>[4N+2] is
     (<code_space>[4N+1] - <code_space>[4N] + 1), <code_space>[4N+3]
     is the number of characters contained in the first through (N+1)th
     dimensions, except that there is no <code_space>[15].
     We get `char-index' of a `code-point' from this
     information.  */
  pub code_space: [c_int; 15],

  /* If B is a byte of Nth dimension of a code-point, the (N-1)th bit
     of code_space_mask[B] is set.  This array is used to quickly
     check if a code-point is in a valid range.  */
  pub code_space_mask: *mut u8,

  /* True if there's no gap in code-points.  */
  /// bool_bf code_linear_p : 1;
  /* True if the charset is treated as 96 chars in ISO-2022
     as opposed to 94 chars.  */
  /// bool_bf iso_chars_96 : 1;
  /* True if the charset is compatible with ASCII.  */
  /// bool_bf ascii_compatible_p : 1;
  /* True if the charset is supplementary.  */
  /// bool_bf supplementary_p : 1;
  /* True if all the code points are representable by Lisp_Int.  */
  /// bool_bf compact_codes_p : 1;
  /* True if the charset is unified with Unicode.  */
  /// bool_bf unified_p : 1;
  /// clang's record layout puts the six bits at 80:0-80:5 of this byte.
  pub bits: u8,

  /* ISO final byte of the charset: 48..127.  It may be -1 if the
     charset doesn't conform to ISO-2022.  */
  pub iso_final: c_int,

  /* ISO revision number of the charset.  */
  pub iso_revision: c_int,

  /* If the charset is identical to what supported by Emacs 21 and the
     priors, the identification number of the charset used in those
     version.  Otherwise, -1.  */
  pub emacs_mule_id: c_int,

  /* The method for encoding/decoding characters of the charset.  */
  pub method: charset_method,

  /* Minimum and Maximum code points of the charset.  */
  pub min_code: c_uint,
  pub max_code: c_uint,

  /* Offset value used by macros CODE_POINT_TO_INDEX and
      INDEX_TO_CODE_POINT.  */
  pub char_index_offset: c_uint,

  /* Minimum and Maximum character codes of the charset.  If the
     charset is compatible with ASCII, min_char is a minimum non-ASCII
     character of the charset.  If the method of charset is
     CHARSET_METHOD_OFFSET, even if the charset is unified, min_char
     and max_char doesn't change.  */
  pub min_char: c_int,
  pub max_char: c_int,

  /* The code returned by ENCODE_CHAR if a character is not encodable
     by the charset.  */
  pub invalid_code: c_uint,

  /* If the method of the charset is CHARSET_METHOD_MAP, this is a
     table of bits used to quickly and roughly guess if a character
     belongs to the charset.

     The first 64 elements are 512 bits for characters less than
     0x10000.  Each bit corresponds to 128-character block.  The last
     126 elements are 1008 bits for the greater characters
     (0x10000..0x3FFFFF).  Each bit corresponds to 4096-character
     block.

     If a bit is 1, at least one character in the corresponding block is
     in this charset.  */
  pub fast_map: [u8; 190],

  /* Offset value to calculate a character code from code-point, and
     visa versa.  */
  pub code_offset: c_int,
}

/// The six `bool_bf` members of `struct charset`, in charset.h's declaration order.  C
/// names them through the bitfields; Rust reads them by mask (PORTING.md 1.3).  The names
/// coincide with charset.h's own `CHARSET_*_P(charset)` accessor macros, which live in
/// Rust's macro namespace and so do not collide with these constants.
pub const CHARSET_CODE_LINEAR_P: u8 = 1 << 0;
pub const CHARSET_ISO_CHARS_96: u8 = 1 << 1;
pub const CHARSET_ASCII_COMPATIBLE_P: u8 = 1 << 2;
pub const CHARSET_SUPPLEMENTARY_P: u8 = 1 << 3;
pub const CHARSET_COMPACT_CODES_P: u8 = 1 << 4;
pub const CHARSET_UNIFIED_P: u8 = 1 << 5;

/// C: `struct charset_table` charset.h 246-262.
#[repr(C)]
pub struct charset_table
{
  pub start: *mut charset,
  pub size: c_int,
  pub used: c_int,
  pub attributes_table: Lisp_Object,
}

unsafe extern "C-unwind" {
  /// C: charset.h 265: `extern struct charset_table charset_table;` (charset.c defines it).
  pub static mut charset_table: charset_table;
  /// C: charset.h 446: `extern Lisp_Object charset_work;` (charset.c defines it).
  pub static mut charset_work: Lisp_Object;
  /// C: charset.h 558: `extern unsigned encode_char (struct charset *, int);`
  pub fn encode_char (charset: *mut charset, c: c_int) -> c_uint;

}

// charset.h 282-296 and 313-340: the accessor macros `CHARSET_ATTR_ID`,
// `CHARSET_ATTR_ENCODER`, `CHARSET_ATTRIBUTES` and `CHARSET_ENCODER`, each with C's body.
macro_rules! CHARSET_ATTR_ID { ($attrs:expr) => { AREF ($attrs, charset_id as isize) }; }
macro_rules! CHARSET_ATTR_ENCODER { ($attrs:expr) => { AREF ($attrs, charset_encoder as isize) }; }
macro_rules! CHARSET_ATTRIBUTES { ($charset:expr) => { charset_attributes_getter ($charset) }; }
macro_rules! CHARSET_ENCODER {
  ($charset:expr) => { CHARSET_ATTR_ENCODER! (CHARSET_ATTRIBUTES! ($charset)) }; }

/// charset.h 306-313: `INLINE Lisp_Object charset_attributes_getter (struct charset *)`.
/// It is a header inline, so C gives every translation unit its own copy; the port defines
/// it here until the mirror exists.
unsafe fn charset_attributes_getter (charset: *mut charset) -> Lisp_Object
{
  eassert! (ASIZE (charset_table.attributes_table) == charset_table.size as isize);
  let attrs: Lisp_Object = AREF (charset_table.attributes_table, (*charset).id as isize);
  eassert! (XFIXNUM (CHARSET_ATTR_ID! (attrs)) == (*charset).id as isize);
  return attrs;
}

/// C: charset.h 449-480 (`ENCODE_CHAR`), the macro the live `map_sub_char_table_for_charset`
/// expands.  C wraps the whole expression in `verify_expr (sizeof (c) <= sizeof (int), …)`,
/// a compile-time guard (lib/verify.h 350-351) whose only runtime effect is the expression
/// itself; the port's `$c` is a `c_int` at the unit's single use site, which is exactly what
/// the guard asserts, so only the guard is dropped (recorded in the ledger).  Each use of
/// `$c` and `$charset` keeps C's own evaluation count.
macro_rules! ENCODE_CHAR {
  ($charset:expr, $c:expr) => {
    (if ASCII_CHAR_P ($c as isize) && (*$charset).bits & CHARSET_ASCII_COMPATIBLE_P != 0
       { $c as c_uint }
     else if (*$charset).bits & CHARSET_UNIFIED_P != 0
	  || (*$charset).method == CHARSET_METHOD_SUBSET
	  || (*$charset).method == CHARSET_METHOD_SUPERSET
       { encode_char ($charset, $c) }
     else if ($c) < (*$charset).min_char || ($c) > (*$charset).max_char
       { (*$charset).invalid_code }
     else if (*$charset).method == CHARSET_METHOD_OFFSET
       { if (*$charset).bits & CHARSET_CODE_LINEAR_P != 0
	   { (($c) - (*$charset).code_offset) as c_uint + (*$charset).min_code }
	 else { encode_char ($charset, $c) } }
     else if (*$charset).method == CHARSET_METHOD_MAP
       { if (*$charset).bits & CHARSET_COMPACT_CODES_P != 0
	    && CHAR_TABLE_P (CHARSET_ENCODER! ($charset))
	   { charset_work = CHAR_TABLE_REF (CHARSET_ENCODER! ($charset), $c as isize);
	     if NILP (charset_work) { (*$charset).invalid_code }
	     else { XFIXNAT (charset_work) as c_uint } }
	 else { encode_char ($charset, $c) } }
     else { encode_char ($charset, $c) })
  };
}

/* 64/16/32/128 */

/* Number of elements in Nth level char-table.  */
#[unsafe(no_mangle)]
pub static mut chartab_size: [c_int; 4] =
  [ (1 << CHARTAB_SIZE_BITS_0),
    (1 << CHARTAB_SIZE_BITS_1),
    (1 << CHARTAB_SIZE_BITS_2),
    (1 << CHARTAB_SIZE_BITS_3) ];

/* Number of characters each element of Nth level char-table
   covers.  */
static chartab_chars: [c_int; 4] =
  [ (1 << (CHARTAB_SIZE_BITS_1 + CHARTAB_SIZE_BITS_2 + CHARTAB_SIZE_BITS_3)),
    (1 << (CHARTAB_SIZE_BITS_2 + CHARTAB_SIZE_BITS_3)),
    (1 << CHARTAB_SIZE_BITS_3),
    1 ];

/* Number of characters (in bits) each element of Nth level char-table
   covers.  */
static chartab_bits: [c_int; 4] =
  [ (CHARTAB_SIZE_BITS_1 + CHARTAB_SIZE_BITS_2 + CHARTAB_SIZE_BITS_3) as c_int,
    (CHARTAB_SIZE_BITS_2 + CHARTAB_SIZE_BITS_3) as c_int,
    CHARTAB_SIZE_BITS_3 as c_int,
    0 ];

macro_rules! CHARTAB_IDX {
  ($c:expr, $depth:expr, $min_char:expr) =>
    { ((($c) - ($min_char)) >> chartab_bits[($depth) as usize]) };
}

/* Preamble for uniprop (Unicode character property) tables.  See the
   comment of "Unicode character property tables".  */

/* Types of decoder and encoder functions for uniprop values.  */
pub type uniprop_decoder_t =
  Option<unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object) -> Lisp_Object>;
pub type uniprop_encoder_t =
  Option<unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object) -> Lisp_Object>;

// C's forward declarations (C 64-68) of `uniprop_table_uncompress`,
// `uniprop_get_decoder` and `sub_char_table_ref_and_range` are deleted; the definitions keep
// their C place (PORTING.md 0.1-12, recorded in the ledger).

/* 1 iff TABLE is a uniprop table.  */
macro_rules! UNIPROP_TABLE_P {
  ($TABLE:expr) =>
    { (EQ ((*XCHAR_TABLE ($TABLE)).purpose, Qchar_code_property_table ())
       && CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE ($TABLE)) == 5) };
}

/* Return a decoder for values in the uniprop table TABLE.  */
macro_rules! UNIPROP_GET_DECODER {
  ($TABLE:expr) =>
    { (if UNIPROP_TABLE_P! ($TABLE) { uniprop_get_decoder ($TABLE) } else { None }) };
}

/* Nonzero iff OBJ is a string representing uniprop values of 128
   succeeding characters (the bottom level of a char-table) by a
   compressed format.  We are sure that no property value has a string
   starting with '\001' or '\002'.  */
macro_rules! UNIPROP_COMPRESSED_FORM_P {
  ($OBJ:expr) =>
    { (STRINGP ($OBJ) && SCHARS ($OBJ) > 0
       && ((SREF ($OBJ, 0) == 1 || (SREF ($OBJ, 0) == 2)))) };
}

unsafe fn CHECK_CHAR_TABLE (x: Lisp_Object)
{
  CHECK_TYPE (CHAR_TABLE_P (x), Qchar_table_p (), x);
}

unsafe fn set_char_table_ascii (table: Lisp_Object, val: Lisp_Object)
{
  (*XCHAR_TABLE (table)).ascii = val;
}
unsafe fn set_char_table_parent (table: Lisp_Object, val: Lisp_Object)
{
  (*XCHAR_TABLE (table)).parent = val;
}

DEFUN! ("make-char-table", Fmake_char_table, Smake_char_table, 1, 2, 0,
       doc: "Return a newly created char-table, with purpose PURPOSE.\nEach element is initialized to INIT, which defaults to nil.\n\nPURPOSE should be a symbol.  If it has a `char-table-extra-slots'\nproperty, the property's value should be an integer between 0 and 10\nthat specifies how many extra slots the char-table has.  Otherwise,\nthe char-table has no extra slot.",
       (purpose: Lisp_Object, init: Lisp_Object),
       {
  let mut vector: Lisp_Object;
  let mut n: Lisp_Object;
  let mut n_extras: c_int;
  let mut size: c_int;

  CHECK_SYMBOL (purpose);
  n = Fget (purpose, Qchar_table_extra_slots ());
  if NILP (n)
    { n_extras = 0; }
  else
    {
      CHECK_FIXNAT (n);
      n_extras = XFIXNUM (n) as c_int;
    }

  size = CHAR_TABLE_STANDARD_SLOTS as c_int + n_extras;
  vector = make_vector (size as isize, init);
  XSETPVECTYPE! (XVECTOR (vector), pvec_type::PVEC_CHAR_TABLE);
  set_char_table_parent (vector, Qnil ());
  set_char_table_purpose (vector, purpose);
  XSETCHAR_TABLE! (vector, XCHAR_TABLE (vector));
  return vector;
});

unsafe fn make_sub_char_table (depth: c_int, min_char: c_int, defalt: Lisp_Object)
  -> Lisp_Object
{
  let mut i: c_int;
  let table: Lisp_Object = make_uninit_sub_char_table (depth, min_char);

  i = 0;
  while i < chartab_size[depth as usize]
    {
      FAM_ELEM! ((*XSUB_CHAR_TABLE (table)).contents, i) = defalt;
      i += 1;
    }
  return table;
}

unsafe fn char_table_ascii (table: Lisp_Object) -> Lisp_Object
{
  let mut sub: Lisp_Object;
  let mut val: Lisp_Object;

  sub = FAM_ELEM! ((*XCHAR_TABLE (table)).contents, 0);
  if ! SUB_CHAR_TABLE_P (sub)
    { return sub; }
  sub = FAM_ELEM! ((*XSUB_CHAR_TABLE (sub)).contents, 0);
  if ! SUB_CHAR_TABLE_P (sub)
    { return sub; }
  val = FAM_ELEM! ((*XSUB_CHAR_TABLE (sub)).contents, 0);
  if UNIPROP_TABLE_P! (table) && UNIPROP_COMPRESSED_FORM_P! (val)
    { val = uniprop_table_uncompress (sub, 0); }
  return val;
}

unsafe fn copy_sub_char_table (table: Lisp_Object) -> Lisp_Object
{
  let depth: c_int = (*XSUB_CHAR_TABLE (table)).depth;
  let min_char: c_int = (*XSUB_CHAR_TABLE (table)).min_char;
  let copy: Lisp_Object = make_sub_char_table (depth, min_char, Qnil ());
  let mut i: c_int;

  /* Recursively copy any sub char-tables.  */
  i = 0;
  while i < chartab_size[depth as usize]
    {
      let val: Lisp_Object = FAM_ELEM! ((*XSUB_CHAR_TABLE (table)).contents, i);
      set_sub_char_table_contents
	(copy, i as isize,
	 if SUB_CHAR_TABLE_P (val) { copy_sub_char_table (val) } else { val });
      i += 1;
    }

  return copy;
}


#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn copy_char_table (table: Lisp_Object) -> Lisp_Object
{
  let mut size: c_int = PVSIZE (table) as c_int;
  let mut copy: Lisp_Object = make_nil_vector (size as isize);
  XSETPVECTYPE! (XVECTOR (copy), pvec_type::PVEC_CHAR_TABLE);
  set_char_table_defalt (copy, (*XCHAR_TABLE (table)).defalt);
  set_char_table_parent (copy, (*XCHAR_TABLE (table)).parent);
  set_char_table_purpose (copy, (*XCHAR_TABLE (table)).purpose);
  let mut i: c_int = 0;
  while i < chartab_size[0]
    {
      set_char_table_contents
	(copy, i as isize,
	 (if SUB_CHAR_TABLE_P (FAM_ELEM! ((*XCHAR_TABLE (table)).contents, i))
	    { copy_sub_char_table (FAM_ELEM! ((*XCHAR_TABLE (table)).contents, i)) }
	  else { FAM_ELEM! ((*XCHAR_TABLE (table)).contents, i) }));
      i += 1;
    }
  set_char_table_ascii (copy, char_table_ascii (copy));
  size -= CHAR_TABLE_STANDARD_SLOTS as c_int;
  let mut i: c_int = 0;
  while i < size
    {
      set_char_table_extras
	(copy, i as isize, FAM_ELEM! ((*XCHAR_TABLE (table)).extras, i));
      i += 1;
    }

  XSETCHAR_TABLE! (copy, XCHAR_TABLE (copy));
  return copy;
}

unsafe fn sub_char_table_ref (table: Lisp_Object, c: c_int, is_uniprop: bool) -> Lisp_Object
{
  let tbl: *mut Lisp_Sub_Char_Table = XSUB_CHAR_TABLE (table);
  let mut val: Lisp_Object;
  let idx: c_int = CHARTAB_IDX! (c, (*tbl).depth, (*tbl).min_char);

  val = FAM_ELEM! ((*tbl).contents, idx);
  if is_uniprop && UNIPROP_COMPRESSED_FORM_P! (val)
    { val = uniprop_table_uncompress (table, idx); }
  if SUB_CHAR_TABLE_P (val)
    { val = sub_char_table_ref (val, c, is_uniprop); }
  return val;
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn char_table_ref (table: Lisp_Object, c: c_int) -> Lisp_Object
{
  let tbl: *mut Lisp_Char_Table = XCHAR_TABLE (table);
  let mut val: Lisp_Object;

  if ASCII_CHAR_P (c as isize)
    {
      val = (*tbl).ascii;
      if SUB_CHAR_TABLE_P (val)
	{ val = FAM_ELEM! ((*XSUB_CHAR_TABLE (val)).contents, c); }
    }
  else
    {
      val = FAM_ELEM! ((*tbl).contents, CHARTAB_IDX! (c, 0, 0));
      if SUB_CHAR_TABLE_P (val)
	{ val = sub_char_table_ref (val, c, UNIPROP_TABLE_P! (table)); }
    }
  if NILP (val)
    {
      val = (*tbl).defalt;
      if NILP (val) && CHAR_TABLE_P ((*tbl).parent)
	{ val = char_table_ref ((*tbl).parent, c); }
    }
  return val;
}

unsafe fn char_table_ref_simple (table: Lisp_Object, idx: c_int, c: c_int,
				 from: *mut c_int, to: *mut c_int,
				 defalt: Lisp_Object, is_uniprop: bool,
				 is_subtable: bool) -> Lisp_Object
{
  let mut val: Lisp_Object =
    if is_subtable
      { FAM_ELEM! ((*XSUB_CHAR_TABLE (table)).contents, idx) }
    else
      { FAM_ELEM! ((*XCHAR_TABLE (table)).contents, idx) };
  if is_uniprop && UNIPROP_COMPRESSED_FORM_P! (val)
    { val = uniprop_table_uncompress (table, idx); }
  if SUB_CHAR_TABLE_P (val)
    { val = sub_char_table_ref_and_range (val, c, from, to, defalt, is_uniprop); }
  else if NILP (val)
    { val = defalt; }
  return val;
}

unsafe fn sub_char_table_ref_and_range (table: Lisp_Object, c: c_int, from: *mut c_int,
					to: *mut c_int, defalt: Lisp_Object,
					is_uniprop: bool) -> Lisp_Object
{
  let mut c: c_int = c;
  let tbl: *mut Lisp_Sub_Char_Table = XSUB_CHAR_TABLE (table);
  let depth: c_int = (*tbl).depth;
  let min_char: c_int = (*tbl).min_char;
  let mut chartab_idx: c_int = CHARTAB_IDX! (c, depth, min_char);
  let mut idx: c_int;
  let mut val: Lisp_Object
    = char_table_ref_simple (table, chartab_idx, c, from, to,
			     defalt, is_uniprop, true);

  idx = chartab_idx;
  while idx > 0 && (*from) < min_char + idx * chartab_chars[depth as usize]
    {
      c = min_char + idx * chartab_chars[depth as usize] - 1;
      idx -= 1;
      let this_val: Lisp_Object
	= char_table_ref_simple (table, idx, c, from, to,
				 defalt, is_uniprop, true);

      if ! EQ (this_val, val)
	{
	  (*from) = c + 1;
	  break;
	}
    }
  while { c = (chartab_idx + 1) * chartab_chars[depth as usize];
	  (c < chartab_chars[(depth - 1) as usize])
	    && { c += min_char; c <= (*to) } }
    {
      chartab_idx += 1;
      let this_val: Lisp_Object
	= char_table_ref_simple (table, chartab_idx, c, from, to,
				 defalt, is_uniprop, true);

      if ! EQ (this_val, val)
	{
	  (*to) = c - 1;
	  break;
	}
    }

  return val;
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn char_table_ref_and_range (table: Lisp_Object, c: c_int,
						   from: *mut c_int, to: *mut c_int)
  -> Lisp_Object
{
  let mut c: c_int = c;
  let tbl: *mut Lisp_Char_Table = XCHAR_TABLE (table);
  let mut chartab_idx: c_int = CHARTAB_IDX! (c, 0, 0);
  let is_uniprop: bool = UNIPROP_TABLE_P! (table);

  if (*from) < 0
    { (*from) = 0; }
  if (*to) < 0
    { (*to) = MAX_CHAR; }

  let mut val: Lisp_Object
    = char_table_ref_simple (table, chartab_idx, c, from, to,
			     (*tbl).defalt, is_uniprop, false);

  let mut idx: c_int = chartab_idx;
  while (*from) < idx * chartab_chars[0]
    {
      c = idx * chartab_chars[0] - 1;
      idx -= 1;
      let this_val: Lisp_Object
	= char_table_ref_simple (table, idx, c, from, to,
				 (*tbl).defalt, is_uniprop, false);

      if ! EQ (this_val, val)
	{
	  (*from) = c + 1;
	  break;
	}
    }
  while (*to) >= (chartab_idx + 1) * chartab_chars[0]
    {
      chartab_idx += 1;
      c = chartab_idx * chartab_chars[0];
      let this_val: Lisp_Object
	= char_table_ref_simple (table, chartab_idx, c, from, to,
				 (*tbl).defalt, is_uniprop, false);

      if ! EQ (this_val, val)
	{
	  (*to) = c - 1;
	  break;
	}
    }

  return val;
}


unsafe fn sub_char_table_set (table: Lisp_Object, c: c_int, val: Lisp_Object,
			      is_uniprop: bool)
{
  let tbl: *mut Lisp_Sub_Char_Table = XSUB_CHAR_TABLE (table);
  let depth: c_int = (*tbl).depth;
  let min_char: c_int = (*tbl).min_char;
  let i: c_int = CHARTAB_IDX! (c, depth, min_char);
  let mut sub: Lisp_Object;

  if depth == 3
    { set_sub_char_table_contents (table, i as isize, val); }
  else
    {
      sub = FAM_ELEM! ((*tbl).contents, i);
      if ! SUB_CHAR_TABLE_P (sub)
	{
	  if is_uniprop && UNIPROP_COMPRESSED_FORM_P! (sub)
	    { sub = uniprop_table_uncompress (table, i); }
	  else
	    {
	      sub = make_sub_char_table (depth + 1,
					 min_char + i * chartab_chars[depth as usize],
					 sub);
	      set_sub_char_table_contents (table, i as isize, sub);
	    }
	}
      sub_char_table_set (sub, c, val, is_uniprop);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn char_table_set (table: Lisp_Object, c: c_int, val: Lisp_Object)
{
  let tbl: *mut Lisp_Char_Table = XCHAR_TABLE (table);

  if ASCII_CHAR_P (c as isize)
      && SUB_CHAR_TABLE_P ((*tbl).ascii)
    { set_sub_char_table_contents ((*tbl).ascii, c as isize, val); }
  else
    {
      let i: c_int = CHARTAB_IDX! (c, 0, 0);
      let mut sub: Lisp_Object;

      sub = FAM_ELEM! ((*tbl).contents, i);
      if ! SUB_CHAR_TABLE_P (sub)
	{
	  sub = make_sub_char_table (1, i * chartab_chars[0], sub);
	  set_char_table_contents (table, i as isize, sub);
	}
      sub_char_table_set (sub, c, val, UNIPROP_TABLE_P! (table));
      if ASCII_CHAR_P (c as isize)
	{ set_char_table_ascii (table, char_table_ascii (table)); }
    }
}

unsafe fn sub_char_table_set_range (table: Lisp_Object, from: c_int, to: c_int,
				    val: Lisp_Object, is_uniprop: bool)
{
  let mut from: c_int = from;
  let tbl: *mut Lisp_Sub_Char_Table = XSUB_CHAR_TABLE (table);
  let depth: c_int = (*tbl).depth;
  let min_char: c_int = (*tbl).min_char;
  let chars_in_block: c_int = chartab_chars[depth as usize];
  let mut i: c_int;
  let mut c: c_int;
  let lim: c_int = chartab_size[depth as usize];

  if from < min_char
    { from = min_char; }
  i = CHARTAB_IDX! (from, depth, min_char);
  c = min_char + chars_in_block * i;
  while i < lim
    {
      if c > to
	{ break; }
      if from <= c && c + chars_in_block - 1 <= to
	{ set_sub_char_table_contents (table, i as isize, val); }
      else
	{
	  let mut sub: Lisp_Object = FAM_ELEM! ((*tbl).contents, i);
	  if ! SUB_CHAR_TABLE_P (sub)
	    {
	      if is_uniprop && UNIPROP_COMPRESSED_FORM_P! (sub)
		{ sub = uniprop_table_uncompress (table, i); }
	      else
		{
		  sub = make_sub_char_table (depth + 1, c, sub);
		  set_sub_char_table_contents (table, i as isize, sub);
		}
	    }
	  sub_char_table_set_range (sub, from, to, val, is_uniprop);
	}
      i += 1;
      c += chars_in_block;
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn char_table_set_range (table: Lisp_Object, from: c_int, to: c_int,
					       val: Lisp_Object)
{
  let tbl: *mut Lisp_Char_Table = XCHAR_TABLE (table);

  if from == to
    { char_table_set (table, from, val); }
  else
    {
      let is_uniprop: bool = UNIPROP_TABLE_P! (table);
      let lim: c_int = CHARTAB_IDX! (to, 0, 0);
      let mut i: c_int;
      let mut c: c_int;

      i = CHARTAB_IDX! (from, 0, 0);
      c = i * chartab_chars[0];
      while i <= lim
	{
	  if c > to
	    { break; }
	  if from <= c && c + chartab_chars[0] - 1 <= to
	    { set_char_table_contents (table, i as isize, val); }
	  else
	    {
	      let mut sub: Lisp_Object = FAM_ELEM! ((*tbl).contents, i);
	      if ! SUB_CHAR_TABLE_P (sub)
		{
		  sub = make_sub_char_table (1, i * chartab_chars[0], sub);
		  set_char_table_contents (table, i as isize, sub);
		}
	      sub_char_table_set_range (sub, from, to, val, is_uniprop);
	    }
	  i += 1;
	  c += chartab_chars[0];
	}
      if ASCII_CHAR_P (from as isize)
	{ set_char_table_ascii (table, char_table_ascii (table)); }
    }
}

DEFUN! ("char-table-subtype", Fchar_table_subtype, Schar_table_subtype,
       1, 1, 0,
       doc: "Return the subtype of char-table CHAR-TABLE.  The value is a symbol.",
       (char_table: Lisp_Object),
       {
  CHECK_CHAR_TABLE (char_table);

  return (*XCHAR_TABLE (char_table)).purpose;
});

DEFUN! ("char-table-parent", Fchar_table_parent, Schar_table_parent,
       1, 1, 0,
       doc: "Return the parent char-table of CHAR-TABLE.\nThe value is either nil or another char-table.\nIf CHAR-TABLE holds nil for a given character,\nthen the actual applicable value is inherited from the parent char-table\n(or from its parents, if necessary).",
       (char_table: Lisp_Object),
       {
  CHECK_CHAR_TABLE (char_table);

  return (*XCHAR_TABLE (char_table)).parent;
});

DEFUN! ("set-char-table-parent", Fset_char_table_parent, Sset_char_table_parent,
       2, 2, 0,
       doc: "Set the parent char-table of CHAR-TABLE to PARENT.\nReturn PARENT.  PARENT must be either nil or another char-table.",
       (char_table: Lisp_Object, parent: Lisp_Object),
       {
  let mut temp: Lisp_Object;

  CHECK_CHAR_TABLE (char_table);

  if !NILP (parent)
    {
      CHECK_CHAR_TABLE (parent);

      temp = parent;
      while !NILP (temp)
	{
	  if EQ (temp, char_table)
	    { error (c"Attempt to make a chartable be its own parent".as_ptr ()); }
	  temp = (*XCHAR_TABLE (temp)).parent;
	}
    }

  set_char_table_parent (char_table, parent);

  return parent;
});

DEFUN! ("char-table-extra-slot", Fchar_table_extra_slot, Schar_table_extra_slot,
       2, 2, 0,
       doc: "Return the value of CHAR-TABLE's extra-slot number N.",
       (char_table: Lisp_Object, n: Lisp_Object),
       {
  CHECK_CHAR_TABLE (char_table);
  CHECK_FIXNUM (n);
  if XFIXNUM (n) < 0
      || XFIXNUM (n) >= CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (char_table))
    { args_out_of_range (char_table, n); }

  return FAM_ELEM! ((*XCHAR_TABLE (char_table)).extras, XFIXNUM (n));
});

DEFUN! ("set-char-table-extra-slot", Fset_char_table_extra_slot,
       Sset_char_table_extra_slot,
       3, 3, 0,
       doc: "Set CHAR-TABLE's extra-slot number N to VALUE.",
       (char_table: Lisp_Object, n: Lisp_Object, value: Lisp_Object),
       {
  CHECK_CHAR_TABLE (char_table);
  CHECK_FIXNUM (n);
  if XFIXNUM (n) < 0
      || XFIXNUM (n) >= CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (char_table))
    { args_out_of_range (char_table, n); }

  set_char_table_extras (char_table, XFIXNUM (n), value);
  return value;
});

DEFUN! ("char-table-range", Fchar_table_range, Schar_table_range,
       2, 2, 0,
       doc: "Return the value in CHAR-TABLE for a range of characters RANGE.\nRANGE should be nil (for the default value),\na cons of character codes (for characters in the range), or a character code.\nIf RANGE is a cons (FROM . TO), the function returns the value for FROM.",
       (char_table: Lisp_Object, range: Lisp_Object),
       {
  let mut val: Lisp_Object;
  CHECK_CHAR_TABLE (char_table);

  if NILP (range)
    { val = (*XCHAR_TABLE (char_table)).defalt; }
  else if CHARACTERP (range)
    { val = CHAR_TABLE_REF (char_table, XFIXNAT (range)); }
  else if CONSP (range)
    {
      let mut from: c_int;
      let mut to: c_int;

      CHECK_CHARACTER_CAR (range);
      CHECK_CHARACTER_CDR (range);
      from = XFIXNAT (XCAR (range)) as c_int;
      to = XFIXNAT (XCDR (range)) as c_int;
      val = char_table_ref_and_range (char_table, from, &raw mut from, &raw mut to);
      /* Not yet implemented. */
    }
  else
    { error (c"Invalid RANGE argument to `char-table-range'".as_ptr ()); }
  return val;
});

DEFUN! ("set-char-table-range", Fset_char_table_range, Sset_char_table_range,
       3, 3, 0,
       doc: "Set the value in CHAR-TABLE for a range of characters RANGE to VALUE.\nRANGE should be t (for all characters), nil (for the default value),\na cons of character codes (for characters in the range),\nor a character code.  Return VALUE.",
       (char_table: Lisp_Object, range: Lisp_Object, value: Lisp_Object),
       {
  CHECK_CHAR_TABLE (char_table);
  if EQ (range, Qt ())
    {
      let mut i: c_int;

      set_char_table_ascii (char_table, value);
      i = 0;
      while i < chartab_size[0]
	{
	  set_char_table_contents (char_table, i as isize, value);
	  i += 1;
	}
    }
  else if NILP (range)
    { set_char_table_defalt (char_table, value); }
  else if CHARACTERP (range)
    { char_table_set (char_table, XFIXNUM (range) as c_int, value); }
  else if CONSP (range)
    {
      CHECK_CHARACTER_CAR (range);
      CHECK_CHARACTER_CDR (range);
      char_table_set_range (char_table,
			    XFIXNUM (XCAR (range)) as c_int,
			    XFIXNUM (XCDR (range)) as c_int, value);
    }
  else
    { error (c"Invalid RANGE argument to `set-char-table-range'".as_ptr ()); }

  return value;
});

unsafe fn optimize_sub_char_table (table: Lisp_Object, test: Lisp_Object) -> Lisp_Object
{
  let tbl: *mut Lisp_Sub_Char_Table = XSUB_CHAR_TABLE (table);
  let mut i: c_int;
  let depth: c_int = (*tbl).depth;
  let mut elt: Lisp_Object;
  let mut this: Lisp_Object;
  let mut optimizable: bool;

  elt = FAM_ELEM! ((*XSUB_CHAR_TABLE (table)).contents, 0);
  if SUB_CHAR_TABLE_P (elt)
    {
      elt = optimize_sub_char_table (elt, test);
      set_sub_char_table_contents (table, 0, elt);
    }
  optimizable = if SUB_CHAR_TABLE_P (elt) { false } else { true };
  i = 1;
  while i < chartab_size[depth as usize]
    {
      this = FAM_ELEM! ((*XSUB_CHAR_TABLE (table)).contents, i);
      if SUB_CHAR_TABLE_P (this)
	{
	  this = optimize_sub_char_table (this, test);
	  set_sub_char_table_contents (table, i as isize, this);
	}
      if optimizable
	  && (if NILP (test) { NILP (Fequal (this, elt)) } /* defaults to `equal'. */
	      else if EQ (test, Qeq ()) { !EQ (this, elt) }  /* Optimize `eq' case.  */
	      else { NILP (calln! (test, this, elt)) })
	{ optimizable = false; }
      i += 1;
    }

  return if optimizable { elt } else { table };
}

DEFUN! ("optimize-char-table", Foptimize_char_table, Soptimize_char_table,
       1, 2, 0,
       doc: "Optimize CHAR-TABLE.\nTEST is the comparison function used to decide whether two entries are\nequivalent and can be merged.  It defaults to `equal'.",
       (char_table: Lisp_Object, test: Lisp_Object),
       {
  let mut elt: Lisp_Object;
  let mut i: c_int;

  CHECK_CHAR_TABLE (char_table);

  i = 0;
  while i < chartab_size[0]
    {
      elt = FAM_ELEM! ((*XCHAR_TABLE (char_table)).contents, i);
      if SUB_CHAR_TABLE_P (elt)
	{
	  set_char_table_contents
	    (char_table, i as isize, optimize_sub_char_table (elt, test));
	}
      i += 1;
    }
  /* Reset the `ascii' cache, in case it got optimized away.  */
  set_char_table_ascii (char_table, char_table_ascii (char_table));

  return Qnil ();
});


/* Map C_FUNCTION or FUNCTION over TABLE (top or sub char-table),
   calling it for each character or group of characters that share a
   value.  RANGE is a cons (FROM . TO) specifying the range of target
   characters, VAL is a value of FROM in TABLE, TOP is the top
   char-table.

   ARG is passed to C_FUNCTION when that is called.

   It returns the value of last character covered by TABLE (not the
   value inherited from the parent), and by side-effect, the car part
   of RANGE is updated to the minimum character C where C and all the
   following characters in TABLE have the same value.  */

unsafe fn map_sub_char_table (c_function: Option<unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object, Lisp_Object)>,
			      function: Lisp_Object, table: Lisp_Object, arg: Lisp_Object,
			      mut val: Lisp_Object, range: Lisp_Object, top: Lisp_Object)
  -> Lisp_Object
{
  /* Depth of TABLE.  */
  let mut depth: c_int;
  /* Minimum and maximum characters covered by TABLE. */
  let mut min_char: c_int;
  let mut max_char: c_int;
  /* Number of characters covered by one element of TABLE.  */
  let mut chars_in_block: c_int;
  let mut from: c_int = XFIXNUM (XCAR (range)) as c_int;
  let to: c_int = XFIXNUM (XCDR (range)) as c_int;
  let mut i: c_int;
  let mut c: c_int;
  let is_uniprop: bool = UNIPROP_TABLE_P! (top);
  let decoder: uniprop_decoder_t = UNIPROP_GET_DECODER! (top);

  if SUB_CHAR_TABLE_P (table)
    {
      let tbl: *mut Lisp_Sub_Char_Table = XSUB_CHAR_TABLE (table);

      depth = (*tbl).depth;
      min_char = (*tbl).min_char;
      max_char = min_char + chartab_chars[(depth - 1) as usize] - 1;
    }
  else
    {
      depth = 0;
      min_char = 0;
      max_char = MAX_CHAR;
    }
  chars_in_block = chartab_chars[depth as usize];

  if to < max_char
    { max_char = to; }
  /* Set I to the index of the first element to check.  */
  if from <= min_char
    { i = 0; }
  else
    { i = (from - min_char) / chars_in_block; }
  c = min_char + chars_in_block * i;
  while c <= max_char
    {
      let mut this: Lisp_Object =
	(if SUB_CHAR_TABLE_P (table)
	   { FAM_ELEM! ((*XSUB_CHAR_TABLE (table)).contents, i) }
	 else { FAM_ELEM! ((*XCHAR_TABLE (table)).contents, i) });
      let nextc: c_int = c + chars_in_block;

      if is_uniprop && UNIPROP_COMPRESSED_FORM_P! (this)
	{ this = uniprop_table_uncompress (table, i); }
      if SUB_CHAR_TABLE_P (this)
	{
	  if to >= nextc
	    { XSETCDR (range, make_fixnum ((nextc - 1) as EMACS_INT)); }
	  val = map_sub_char_table (c_function, function, this, arg,
				    val, range, top);
	}
      else
	{
	  if NILP (this)
	    { this = (*XCHAR_TABLE (top)).defalt; }
	  if !EQ (val, this)
	    {
	      let mut different_value: bool = true;

	      if NILP (val)
		{
		  if ! NILP ((*XCHAR_TABLE (top)).parent)
		    {
		      let parent: Lisp_Object = (*XCHAR_TABLE (top)).parent;
		      let temp: Lisp_Object = (*XCHAR_TABLE (parent)).parent;

		      /* This is to get a value of FROM in PARENT
			 without checking the parent of PARENT.  */
		      set_char_table_parent (parent, Qnil ());
		      val = CHAR_TABLE_REF (parent, from as isize);
		      set_char_table_parent (parent, temp);
		      XSETCDR (range, make_fixnum ((c - 1) as EMACS_INT));
		      val = map_sub_char_table (c_function, function,
						parent, arg, val, range,
						parent);
		      if EQ (val, this)
			{ different_value = false; }
		    }
		}
	      if ! NILP (val) && different_value
		{
		  XSETCDR (range, make_fixnum ((c - 1) as EMACS_INT));
		  if EQ (XCAR (range), XCDR (range))
		    {
		      if let Some (c_function) = c_function
			{ c_function (arg, XCAR (range), val); }
		      else
			{
			  if let Some (decoder) = decoder
			    { val = decoder (top, val); }
			  calln! (function, XCAR (range), val);
			}
		    }
		  else
		    {
		      if let Some (c_function) = c_function
			{ c_function (arg, range, val); }
		      else
			{
			  if let Some (decoder) = decoder
			    { val = decoder (top, val); }
			  calln! (function, range, val);
			}
		    }
		}
	      val = this;
	      from = c;
	      XSETCAR (range, make_fixnum (c as EMACS_INT));
	    }
	}
      XSETCDR (range, make_fixnum (to as EMACS_INT));
      i += 1;
      c += chars_in_block;
    }
  return val;
}


/* Map C_FUNCTION or FUNCTION over TABLE, calling it for each
   character or group of characters that share a value.

   ARG is passed to C_FUNCTION when that is called.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn map_char_table (c_function: Option<unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object, Lisp_Object)>,
					 function: Lisp_Object, mut table: Lisp_Object,
					 arg: Lisp_Object)
{
  let mut range: Lisp_Object;
  let mut val: Lisp_Object;
  let mut parent: Lisp_Object;
  let decoder: uniprop_decoder_t = UNIPROP_GET_DECODER! (table);

  range = Fcons (make_fixnum (0), make_fixnum (MAX_CHAR as EMACS_INT));
  parent = (*XCHAR_TABLE (table)).parent;

  val = (*XCHAR_TABLE (table)).ascii;
  if SUB_CHAR_TABLE_P (val)
    { val = FAM_ELEM! ((*XSUB_CHAR_TABLE (val)).contents, 0); }
  val = map_sub_char_table (c_function, function, table, arg, val, range,
			    table);

  /* If VAL is nil and TABLE has a parent, we must consult the parent
     recursively.  */
  while NILP (val) && ! NILP ((*XCHAR_TABLE (table)).parent)
    {
      let mut temp: Lisp_Object;
      let from: c_int = XFIXNUM (XCAR (range)) as c_int;

      parent = (*XCHAR_TABLE (table)).parent;
      temp = (*XCHAR_TABLE (parent)).parent;
      /* This is to get a value of FROM in PARENT without checking the
	 parent of PARENT.  */
      set_char_table_parent (parent, Qnil ());
      val = CHAR_TABLE_REF (parent, from as isize);
      set_char_table_parent (parent, temp);
      val = map_sub_char_table (c_function, function, parent, arg, val, range,
				parent);
      table = parent;
    }

  if ! NILP (val)
    {
      if EQ (XCAR (range), XCDR (range))
	{
	  if let Some (c_function) = c_function
	    { c_function (arg, XCAR (range), val); }
	  else
	    {
	      if let Some (decoder) = decoder
		{ val = decoder (table, val); }
	      calln! (function, XCAR (range), val);
	    }
	}
      else
	{
	  if let Some (c_function) = c_function
	    { c_function (arg, range, val); }
	  else
	    {
	      if let Some (decoder) = decoder
		{ val = decoder (table, val); }
	      calln! (function, range, val);
	    }
	}
    }
}

DEFUN! ("map-char-table", Fmap_char_table, Smap_char_table,
  2, 2, 0,
       doc: "Call FUNCTION for each character in CHAR-TABLE that has non-nil value.\nFUNCTION is called with two arguments, KEY and VALUE.\nKEY is a character code or a cons of character codes specifying a\nrange of characters that have the same value.\nVALUE is what (char-table-range CHAR-TABLE KEY) returns.",
       (function: Lisp_Object, char_table: Lisp_Object),
       {
  CHECK_CHAR_TABLE (char_table);

  map_char_table (None, function, char_table, char_table);
  return Qnil ();
});


unsafe fn map_sub_char_table_for_charset (c_function: Option<unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object)>,
					   function: Lisp_Object, table: Lisp_Object,
					   arg: Lisp_Object, range: Lisp_Object,
					   charset: *mut charset,
					   from: c_uint, to: c_uint)
{
  let tbl: *mut Lisp_Sub_Char_Table = XSUB_CHAR_TABLE (table);
  let mut i: c_int;
  let mut c: c_int = (*tbl).min_char;
  let depth: c_int = (*tbl).depth;

  if depth < 3
    {
      i = 0;
      while i < chartab_size[depth as usize]
	{
	  let mut this: Lisp_Object;

	  this = FAM_ELEM! ((*tbl).contents, i);
	  if SUB_CHAR_TABLE_P (this)
	    { map_sub_char_table_for_charset (c_function, function, this, arg,
					      range, charset, from, to); }
	  else
	    {
	      if ! NILP (XCAR (range))
		{
		  XSETCDR (range, make_fixnum ((c - 1) as EMACS_INT));
		  if let Some (c_function) = c_function
		    { c_function (arg, range); }
		  else
		    { calln! (function, range, arg); }
		}
	      XSETCAR (range, Qnil ());
	    }
	  i += 1;
	  c += chartab_chars[depth as usize];
	}
    }
  else
    {
      i = 0;
      while i < chartab_size[depth as usize]
	{
	  let mut this: Lisp_Object;
	  let mut code: c_uint;

	  this = FAM_ELEM! ((*tbl).contents, i);
	  if NILP (this)
	      || (!charset.is_null ()
		  && { code = ENCODE_CHAR! (charset, c);
		       (code < from || code > to) })
	    {
	      if ! NILP (XCAR (range))
		{
		  XSETCDR (range, make_fixnum ((c - 1) as EMACS_INT));
		  if let Some (c_function) = c_function
		    { c_function (arg, range); }
		  else
		    { calln! (function, range, arg); }
		  XSETCAR (range, Qnil ());
		}
	    }
	  else
	    {
	      if NILP (XCAR (range))
		{ XSETCAR (range, make_fixnum (c as EMACS_INT)); }
	    }
	  i += 1;
	  c += 1;
	}
    }
}


/* Support function for `map-charset-chars'.  Map C_FUNCTION or
   FUNCTION over TABLE, calling it for each character or a group of
   succeeding characters that have non-nil value in TABLE.  TABLE is a
   "mapping table" or a "deunifier table" of a certain charset.

   If CHARSET is not NULL (this is the case that `map-charset-chars'
   is called with non-nil FROM-CODE and TO-CODE), it is a charset that
   owns TABLE, and the function is called only for characters in the
   range FROM and TO.  FROM and TO are not character codes, but code
   points of characters in CHARSET (see 'decode-char').

   This function is called in these two cases:

   (1) A charset has a mapping file name in :map property.

   (2) A charset has an upper code space in :offset property and a
   mapping file name in :unify-map property.  In this case, this
   function is called only for characters in the Unicode code space.
   Characters in upper code space are handled directly in
   map_charset_chars.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn map_char_table_for_charset (c_function: Option<unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object)>,
						     function: Lisp_Object,
						     table: Lisp_Object, arg: Lisp_Object,
						     charset: *mut charset,
						     from: c_uint, to: c_uint)
{
  let mut range: Lisp_Object;
  let mut c: c_int;
  let mut i: c_int;

  range = Fcons (Qnil (), Qnil ());

  i = 0;
  c = 0;
  while i < chartab_size[0]
    {
      let mut this: Lisp_Object;

      this = FAM_ELEM! ((*XCHAR_TABLE (table)).contents, i);
      if SUB_CHAR_TABLE_P (this)
	{ map_sub_char_table_for_charset (c_function, function, this, arg,
					  range, charset, from, to); }
      else
	{
	  if ! NILP (XCAR (range))
	    {
	      XSETCDR (range, make_fixnum ((c - 1) as EMACS_INT));
	      if let Some (c_function) = c_function
		{ c_function (arg, range); }
	      else
		{ calln! (function, range, arg); }
	    }
	  XSETCAR (range, Qnil ());
	}
      i += 1;
      c += chartab_chars[0];
    }
  if ! NILP (XCAR (range))
    {
      XSETCDR (range, make_fixnum ((c - 1) as EMACS_INT));
      if let Some (c_function) = c_function
	{ c_function (arg, range); }
      else
	{ calln! (function, range, arg); }
    }
}


/* Unicode character property tables.

   This section provides a convenient and efficient way to get Unicode
   character properties of characters from C code (from Lisp, you must
   use get-char-code-property).

   The typical usage is to get a char-table object for a specific
   property like this (use of the "bidi-class" property below is just
   an example):

	Lisp_Object bidi_class_table = uniprop_table (intern ("bidi-class"));

   (uniprop_table can return nil if it fails to find data for the
   named property, or if it fails to load the appropriate Lisp support
   file, so the return value should be tested to be non-nil, before it
   is used.)

   To get a property value for character CH use CHAR_TABLE_REF:

	Lisp_Object bidi_class = CHAR_TABLE_REF (bidi_class_table, CH);

   In this case, what you actually get is an index number to the
   vector of property values (symbols nil, L, R, etc).

   The full list of Unicode character properties supported by Emacs is
   documented in the Elisp manual, in the node "Character Properties".

   A table for Unicode character property has these characteristics:

   o The purpose is `char-code-property-table', which implies that the
     table has 5 extra slots.

   o The second extra slot is a Lisp function, an index (integer) to
     the array uniprop_decoder[], or nil.  If it is a Lisp function, we
     can't use such a table from C (at the moment).  If it is nil, it
     means that we don't have to decode values.

   o The third extra slot is a Lisp function, an index (integer) to
     the array uniprop_encoder[], or nil.  If it is a Lisp function, we
     can't use such a table from C (at the moment).  If it is nil, it
     means that we don't have to encode values.  */


/* Uncompress the IDXth element of sub-char-table TABLE.  */

unsafe fn uniprop_table_uncompress (table: Lisp_Object, idx: c_int) -> Lisp_Object
{
  let mut idx: c_int = idx;
  let val: Lisp_Object = FAM_ELEM! ((*XSUB_CHAR_TABLE (table)).contents, idx);
  let min_char: c_int = (*XSUB_CHAR_TABLE (table)).min_char + chartab_chars[2] * idx;
  let sub: Lisp_Object = make_sub_char_table (3, min_char, Qnil ());
  let mut p: *const u8;
  let mut pend: *const u8;

  set_sub_char_table_contents (table, idx as isize, sub);
  p = SDATA (val);
  pend = p.offset (SBYTES (val));
  if *p == 1
    {
      /* SIMPLE TABLE */
      p = p.offset (1);
      idx = string_char_advance (&raw mut p);
      while p < pend && idx < chartab_chars[2]
	{
	  let v: c_int = string_char_advance (&raw mut p);
	  set_sub_char_table_contents
	    (sub, idx as isize, if v > 0 { make_fixnum (v as EMACS_INT) } else { Qnil () });
	  idx += 1;
	}
    }
  else if *p == 2
    {
      /* RUN-LENGTH TABLE */
      p = p.offset (1);
      idx = 0;
      while p < pend
	{
	  let v: c_int = string_char_advance (&raw mut p);
	  let mut count: c_int = 1;

	  if p < pend
	    {
	      let mut len: c_int = 0;
	      count = string_char_and_length (p, &raw mut len);
	      if count < 128
		{ count = 1; }
	      else
		{
		  count -= 128;
		  p = p.offset (len as isize);
		}
	    }
	  while { count -= 1; count >= 0 }
	    {
	      set_sub_char_table_contents (sub, idx as isize, make_fixnum (v as EMACS_INT));
	      idx += 1;
	    }
	}
    }
/* It seems that we don't need this function because C code won't need
   to get a property that is compressed in this form.  */
/* C 1139-1144's `#if 0` arm (the WORD-LIST table) is dead in the frozen configuration and
   is recorded in docs/ports/chartab.md instead of transcribed (P-08).  */
  return sub;
}


/* Decode VALUE as an element of char-table TABLE.  */

unsafe extern "C-unwind" fn uniprop_decode_value_run_length (table: Lisp_Object,
						      mut value: Lisp_Object)
  -> Lisp_Object
{
  if VECTORP (FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 4))
    {
      let valvec: Lisp_Object = FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 4);

      if XFIXNUM (value) >= 0 && XFIXNUM (value) < ASIZE (valvec)
	{ value = AREF (valvec, XFIXNUM (value)); }
    }
  return value;
}

static uniprop_decoder: [uniprop_decoder_t; 1] =
  [ Some (uniprop_decode_value_run_length) ];

static uniprop_decoder_count: c_int = countof! (uniprop_decoder) as c_int;

/* Return the decoder of char-table TABLE or nil if none.  */

unsafe fn uniprop_get_decoder (table: Lisp_Object) -> uniprop_decoder_t
{
  let mut i: EMACS_INT;

  if ! FIXNUMP (FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 1))
    { return None; }
  i = XFIXNUM (FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 1));
  if i < 0 || i >= uniprop_decoder_count as EMACS_INT
    { return None; }
  return uniprop_decoder[i as usize];
}


/* Encode VALUE as an element of char-table TABLE which contains
   characters as elements.  */

unsafe extern "C-unwind" fn uniprop_encode_value_character (table: Lisp_Object, value: Lisp_Object)
  -> Lisp_Object
{
  if ! NILP (value) && ! CHARACTERP (value)
    { wrong_type_argument (Qintegerp (), value); }
  return value;
}


/* Encode VALUE as an element of char-table TABLE which adopts RUN-LENGTH
   compression.  */

unsafe extern "C-unwind" fn uniprop_encode_value_run_length (table: Lisp_Object,
						      mut value: Lisp_Object)
  -> Lisp_Object
{
  let value_table: *mut Lisp_Object =
    fam_base (std::ptr::addr_of_mut! ((*XVECTOR (FAM_ELEM! ((*XCHAR_TABLE (table)).extras,
							     4))).contents));
  let mut i: c_int;
  let size: c_int = ASIZE (FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 4)) as c_int;

  i = 0;
  while i < size
    {
      if EQ (value, *value_table.offset (i as isize))
	{ break; }
      i += 1;
    }
  if i == size
    { wrong_type_argument (build_string (c"Unicode property value".as_ptr ()), value); }
  return make_fixnum (i as EMACS_INT);
}


/* Encode VALUE as an element of char-table TABLE which adopts RUN-LENGTH
   compression and contains numbers as elements.  */

unsafe extern "C-unwind" fn uniprop_encode_value_numeric (table: Lisp_Object,
						   mut value: Lisp_Object)
  -> Lisp_Object
{
  let value_table: *mut Lisp_Object =
    fam_base (std::ptr::addr_of_mut! ((*XVECTOR (FAM_ELEM! ((*XCHAR_TABLE (table)).extras,
							     4))).contents));
  let mut i: c_int;
  let size: c_int = ASIZE (FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 4)) as c_int;

  CHECK_FIXNUM (value);
  i = 0;
  while i < size
    {
      if EQ (value, *value_table.offset (i as isize))
	{ break; }
      i += 1;
    }
  value = make_fixnum (i as EMACS_INT);
  if i == size
    { set_char_table_extras (table, 4,
			     CALLN! (Fvconcat,
				     FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 4),
				     make_vector (1, value))); }
  return make_fixnum (i as EMACS_INT);
}

static uniprop_encoder: [uniprop_encoder_t; 3] =
  [ Some (uniprop_encode_value_character),
    Some (uniprop_encode_value_run_length),
    Some (uniprop_encode_value_numeric) ];

static uniprop_encoder_count: c_int = countof! (uniprop_encoder) as c_int;

/* Return the encoder of char-table TABLE or nil if none.  */

unsafe fn uniprop_get_encoder (table: Lisp_Object) -> uniprop_decoder_t
{
  let mut i: EMACS_INT;

  if ! FIXNUMP (FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 2))
    { return None; }
  i = XFIXNUM (FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 2));
  if i < 0 || i >= uniprop_encoder_count as EMACS_INT
    { return None; }
  return uniprop_encoder[i as usize];
}

/* Return a char-table for Unicode character property PROP.  This
   function may load a Lisp file and thus may cause
   garbage-collection.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn uniprop_table (prop: Lisp_Object) -> Lisp_Object
{
  let mut val: Lisp_Object;
  let mut table: Lisp_Object;
  let mut result: Lisp_Object;

  val = Fassq (prop, Vchar_code_property_alist);
  if ! CONSP (val)
    { return Qnil (); }
  table = XCDR (val);
  if STRINGP (table)
    {
      AUTO_STRING! (intl, c"international/".as_ptr ());
      /* The uni-*.el files _must_ be read using utf-8-emacs-unix, or
         else Emacs might crash.  We bind coding-system-for-read below
         to protect against some Lisp which overrides the coding:
         cookies in the uni-*.el files by, for example, binding
         auto-coding-regexp-alist to some strange value.  */
      let count: specpdl_ref = SPECPDL_INDEX ();
      let coding: Lisp_Object = coding_inherit_eol_type (Qutf_8_emacs (), Qunix ());
      specbind (Qcoding_system_for_read (), coding);
      result = save_match_data_load (concat2 (intl, table), Qt (), Qt (), Qt (), Qt ());
      unbind_to (count, Qnil ());

      if NILP (result)
	{ return Qnil (); }
      table = XCDR (val);
    }
  if ! CHAR_TABLE_P (table)
      || ! UNIPROP_TABLE_P! (table)
    { return Qnil (); }
  val = FAM_ELEM! ((*XCHAR_TABLE (table)).extras, 1);
  if (if FIXNUMP (val)
	{ XFIXNUM (val) < 0 || XFIXNUM (val) >= uniprop_decoder_count as EMACS_INT }
      else { ! NILP (val) })
    { return Qnil (); }
  /* Prepare ASCII values in advance for CHAR_TABLE_REF.  */
  set_char_table_ascii (table, char_table_ascii (table));
  return table;
}

DEFUN! ("unicode-property-table-internal", Funicode_property_table_internal,
       Sunicode_property_table_internal, 1, 1, 0,
       doc: "Return a char-table for Unicode character property PROP.\nUse `get-unicode-property-internal' and\n`put-unicode-property-internal' instead of `aref' and `aset' to get\nand put an element value.",
       (prop: Lisp_Object),
       {
  let table: Lisp_Object = uniprop_table (prop);

  if CHAR_TABLE_P (table)
    { return table; }
  return Fcdr (Fassq (prop, Vchar_code_property_alist));
});

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn get_unicode_property (char_table: Lisp_Object, ch: c_int)
  -> Lisp_Object
{
  let val: Lisp_Object = CHAR_TABLE_REF (char_table, ch as isize);
  let decoder: uniprop_decoder_t = uniprop_get_decoder (char_table);
  return if let Some (decoder) = decoder { decoder (char_table, val) } else { val };
}

DEFUN! ("get-unicode-property-internal", Fget_unicode_property_internal,
       Sget_unicode_property_internal, 2, 2, 0,
       doc: "Return an element of CHAR-TABLE for character CH.\nCHAR-TABLE must be what returned by `unicode-property-table-internal'.",
       (char_table: Lisp_Object, ch: Lisp_Object),
       {
  CHECK_CHAR_TABLE (char_table);
  CHECK_CHARACTER (ch);
  if ! UNIPROP_TABLE_P! (char_table)
    { error (c"Invalid Unicode property table".as_ptr ()); }
  return get_unicode_property (char_table, XFIXNUM (ch) as c_int);
});

DEFUN! ("put-unicode-property-internal", Fput_unicode_property_internal,
       Sput_unicode_property_internal, 3, 3, 0,
       doc: "Set an element of CHAR-TABLE for character CH to VALUE.\nCHAR-TABLE must be what returned by `unicode-property-table-internal'.",
       (char_table: Lisp_Object, ch: Lisp_Object, value: Lisp_Object),
       {
  let mut value: Lisp_Object = value;
  let mut encoder: uniprop_encoder_t;

  CHECK_CHAR_TABLE (char_table);
  CHECK_CHARACTER (ch);
  if ! UNIPROP_TABLE_P! (char_table)
    { error (c"Invalid Unicode property table".as_ptr ()); }
  encoder = uniprop_get_encoder (char_table);
  if let Some (encoder) = encoder
    { value = encoder (char_table, value); }
  CHAR_TABLE_SET (char_table, XFIXNUM (ch), value);
  return Qnil ();
});


// C's `DEFVAR_LISP` storage is `globals.f_Vchar_code_property_alist` in C (lisp.h
// 3495-3530), and `globals.rs` mirrors the name as a declaration, so the unit that owns it
// defines it here and shadows that declaration (P-21, status 3.29).
#[unsafe(no_mangle)]
pub static mut Vchar_code_property_alist: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_chartab ()
{
  /* Purpose of uniprop tables. */
  DEFSYM! (Qchar_code_property_table, c"char-code-property-table");

  defsubr (&raw mut Smake_char_table);
  defsubr (&raw mut Schar_table_parent);
  defsubr (&raw mut Schar_table_subtype);
  defsubr (&raw mut Sset_char_table_parent);
  defsubr (&raw mut Schar_table_extra_slot);
  defsubr (&raw mut Sset_char_table_extra_slot);
  defsubr (&raw mut Schar_table_range);
  defsubr (&raw mut Sset_char_table_range);
  defsubr (&raw mut Soptimize_char_table);
  defsubr (&raw mut Smap_char_table);
  defsubr (&raw mut Sunicode_property_table_internal);
  defsubr (&raw mut Sget_unicode_property_internal);
  defsubr (&raw mut Sput_unicode_property_internal);

  /* Each element has the form (PROP . TABLE).
     PROP is a symbol representing a character property.
     TABLE is a char-table containing the property value for each character.
     TABLE may be a name of file to load to build a char-table.
     This variable should be modified only through
     `define-char-code-property'. */

  DEFVAR_LISP! (c"char-code-property-alist", Vchar_code_property_alist,
		"Alist of character property name vs char-table containing property values.\nInternal use only.");
  Vchar_code_property_alist = Qnil ();
  DEFSYM! (Qcoding_system_for_read, c"coding-system-for-read");
}
