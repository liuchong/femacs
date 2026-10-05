// femacs: port of src/charset.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1-2): the three enums, the two structs, the 63 macros, the
// two INLINE functions and the `unsafe extern "C"` declarations C spells `extern`.  It
// lands with `femacs/src/charset.rs` (the composite.rs/intervals.rs precedent); until now
// three units declared the names they take from this header at their use sites
// (chartab.rs, ccl.rs and the `coding.h`/`buffer.h` chain), and their own declarations
// keep shadowing the glob imports this mirror adds (PORTING.md 0.1-10).
//
// C's own include graph is reproduced below: charset.h 24 includes <verify.h> (no mirror
// yet, and this header's only use of it is `verify_expr` inside ENCODE_CHAR, recorded in
// docs/ports/macro-body-notes.tsv) and charset.h 26 includes "lisp.h".
//
// The one shape that cannot be copied literally is the six `bool_bf : 1` members of
// `struct charset`: `bool_bf` is `bool` on this target (conf_post.h 56-61), so C stores
// them in one byte.  The port packs them into `bits: u8` with the `CHARSET_*_P` masks
// (PORTING.md 1.3), and the sixteen offsets plus `sizeof` are measured, not guessed: clang
// -std=gnu23 on the frozen target reports sizeof 320 / align 8 with id 0, dimension 4,
// code_space 8, code_space_mask 72, the six bits at 80:0-80:5, iso_final 84, iso_revision
// 88, emacs_mule_id 92, method 96, min_code 100, max_code 104, char_index_offset 108,
// min_char 112, max_char 116, invalid_code 120, fast_map 124 and code_offset 316.  The
// `const _` assertions below pin them, and the mirror's `charset_bits!` names the six
// members so every C read and write keeps its C spelling (the intervals.rs
// `interval_bits!` shape, PORTING.md P-80; the six families are in
// docs/ports/token-aliases.tsv).
//
// `CHARSET_ISO_PLANE` (charset.h 322) reads a field `struct charset` does not have; it is
// mirrored as the header spells it (the same case as intervals.h's `#if false` block) and
// is never expanded.
//
// charset.h 408-421 is C's `#if false` block (`Vchar_charset_set`, `CHAR_CHARSET_SET`,
// `SAME_CHARSET_P`): literal-false text, not a configuration arm, so P-51's feature gate
// does not apply and the three names are not mirrored (the intervals.rs `#if false`
// precedent, recorded in docs/ports/include-charset.md).
//
// The five prototypes this unit defines (`char_charset`, `decode_char`, `encode_char`,
// `string_xstring_p`, `map_charset_chars`) live in `charset_decls` with a glob import
// (PORTING.md 0.1-10), so the unit's own definitions are not a second declaration
// (E0428).  The nineteen file-scope objects live in `charset_objs` the same way
// (PORTING.md P-21), which lets this unit define all nineteen at its crate root.

include!("lisp.rs");

// C's include guard `EMACS_CHARSET_H` has no Rust counterpart; the constant below exists
// only so the mirror's name set matches the reference header's (PORTING.md 0.1, P-24).
pub const EMACS_CHARSET_H: bool = true;

/* Index to arguments of Fdefine_charset_internal.  */

// charset.h 33-52: `enum define_charset_arg_index`.
pub type define_charset_arg_index = c_int;
pub const charset_arg_name: define_charset_arg_index = 0;
pub const charset_arg_dimension: define_charset_arg_index = 1;
pub const charset_arg_code_space: define_charset_arg_index = 2;
pub const charset_arg_min_code: define_charset_arg_index = 3;
pub const charset_arg_max_code: define_charset_arg_index = 4;
pub const charset_arg_iso_final: define_charset_arg_index = 5;
pub const charset_arg_iso_revision: define_charset_arg_index = 6;
pub const charset_arg_emacs_mule_id: define_charset_arg_index = 7;
pub const charset_arg_ascii_compatible_p: define_charset_arg_index = 8;
pub const charset_arg_supplementary_p: define_charset_arg_index = 9;
pub const charset_arg_invalid_code: define_charset_arg_index = 10;
pub const charset_arg_code_offset: define_charset_arg_index = 11;
pub const charset_arg_map: define_charset_arg_index = 12;
pub const charset_arg_subset: define_charset_arg_index = 13;
pub const charset_arg_superset: define_charset_arg_index = 14;
pub const charset_arg_unify_map: define_charset_arg_index = 15;
pub const charset_arg_plist: define_charset_arg_index = 16;
pub const charset_arg_max: define_charset_arg_index = 17;


/* Indices to charset attributes vector.  */

// charset.h 58-121: `enum charset_attr_index`, in C's declaration order.
pub type charset_attr_index = c_int;
pub const charset_id: charset_attr_index = 0;
pub const charset_name: charset_attr_index = 1;
pub const charset_plist: charset_attr_index = 2;
pub const charset_map: charset_attr_index = 3;
pub const charset_decoder: charset_attr_index = 4;
pub const charset_encoder: charset_attr_index = 5;
pub const charset_subset: charset_attr_index = 6;
pub const charset_superset: charset_attr_index = 7;
pub const charset_unify_map: charset_attr_index = 8;
pub const charset_deunifier: charset_attr_index = 9;
pub const charset_attr_max: charset_attr_index = 10;

/* Methods for converting code points and characters of charsets.  */

// charset.h 125-146: `enum charset_method`.
pub type charset_method = c_int;
pub const CHARSET_METHOD_OFFSET: charset_method = 0;
pub const CHARSET_METHOD_MAP: charset_method = 1;
pub const CHARSET_METHOD_SUBSET: charset_method = 2;
pub const CHARSET_METHOD_SUPERSET: charset_method = 3;

/// C: charset.h 148-243: `struct charset`.  The six `bool_bf : 1` members are one
/// storage byte in C; the port reads and writes them with `charset_bits!` and the
/// `CHARSET_*_P` masks (PORTING.md 1.3).  Field order and types are C's.
#[repr(C)]
#[derive(Clone, Copy)]
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
/// names them through the bitfields; Rust reads them by mask (PORTING.md 1.3).
pub const CHARSET_CODE_LINEAR_P: u8 = 1 << 0;
pub const CHARSET_ISO_CHARS_96: u8 = 1 << 1;
pub const CHARSET_ASCII_COMPATIBLE_P: u8 = 1 << 2;
pub const CHARSET_SUPPLEMENTARY_P: u8 = 1 << 3;
pub const CHARSET_COMPACT_CODES_P: u8 = 1 << 4;
pub const CHARSET_UNIFIED_P: u8 = 1 << 5;

/* The clang-measured layout of `struct charset` (see the file comment).  */
const _: () = core::assert! (std::mem::size_of::<charset> () == 320);
const _: () = core::assert! (std::mem::align_of::<charset> () == 8);
const _: () = core::assert! (std::mem::offset_of! (charset, id) == 0);
const _: () = core::assert! (std::mem::offset_of! (charset, dimension) == 4);
const _: () = core::assert! (std::mem::offset_of! (charset, code_space) == 8);
const _: () = core::assert! (std::mem::offset_of! (charset, code_space_mask) == 72);
const _: () = core::assert! (std::mem::offset_of! (charset, bits) == 80);
const _: () = core::assert! (std::mem::offset_of! (charset, iso_final) == 84);
const _: () = core::assert! (std::mem::offset_of! (charset, iso_revision) == 88);
const _: () = core::assert! (std::mem::offset_of! (charset, emacs_mule_id) == 92);
const _: () = core::assert! (std::mem::offset_of! (charset, method) == 96);
const _: () = core::assert! (std::mem::offset_of! (charset, min_code) == 100);
const _: () = core::assert! (std::mem::offset_of! (charset, max_code) == 104);
const _: () = core::assert! (std::mem::offset_of! (charset, char_index_offset) == 108);
const _: () = core::assert! (std::mem::offset_of! (charset, min_char) == 112);
const _: () = core::assert! (std::mem::offset_of! (charset, max_char) == 116);
const _: () = core::assert! (std::mem::offset_of! (charset, invalid_code) == 120);
const _: () = core::assert! (std::mem::offset_of! (charset, fast_map) == 124);
const _: () = core::assert! (std::mem::offset_of! (charset, code_offset) == 316);

/* Hash table of charset symbols vs. the corresponding attribute
   vectors.  */

/* A charset_table is an array of struct charset along with a
   Lisp_Vector of charset attributes.

   The charset_table.start field either points to xmalloced memory or to
   the dump (i.e. pdumper_object_p (charset_table.start) can be true).

   charset_table.attributes_table[id] contains the attribute vector for
   the charset at charset_table.start[id].

   We keep the attributes in a separate vector because that is
   convenient for the GC.  (We probably need to revise this decision, if
   we ever expose struct charset as a Lisp level type.)  */

/// C: charset.h 246-262: `struct charset_table`.
#[repr(C)]
pub struct charset_table
{
  pub start: *mut charset,
  pub size: c_int,
  pub used: c_int,
  pub attributes_table: Lisp_Object,
}

const _: () = core::assert! (std::mem::size_of::<charset_table> () == 24);

/* Macros to access information about charset.  */

/* Return the attribute vector of charset whose symbol is SYMBOL.  */
// charset.h 282: `#define CHARSET_SYMBOL_ATTRIBUTES(symbol)`.
macro_rules! CHARSET_SYMBOL_ATTRIBUTES {
  ($symbol:expr) => { Fgethash ($symbol, Vcharset_hash_table, Qnil ()) };
}

// charset.h 285-294: the ten attribute-vector accessors.
macro_rules! CHARSET_ATTR_ID { ($attrs:expr) => { AREF ($attrs, charset_id as isize) }; }
macro_rules! CHARSET_ATTR_NAME { ($attrs:expr) => { AREF ($attrs, charset_name as isize) }; }
macro_rules! CHARSET_ATTR_PLIST { ($attrs:expr) => { AREF ($attrs, charset_plist as isize) }; }
macro_rules! CHARSET_ATTR_MAP { ($attrs:expr) => { AREF ($attrs, charset_map as isize) }; }
macro_rules! CHARSET_ATTR_DECODER { ($attrs:expr) => { AREF ($attrs, charset_decoder as isize) }; }
macro_rules! CHARSET_ATTR_ENCODER { ($attrs:expr) => { AREF ($attrs, charset_encoder as isize) }; }
macro_rules! CHARSET_ATTR_SUBSET { ($attrs:expr) => { AREF ($attrs, charset_subset as isize) }; }
macro_rules! CHARSET_ATTR_SUPERSET { ($attrs:expr) => { AREF ($attrs, charset_superset as isize) }; }
macro_rules! CHARSET_ATTR_UNIFY_MAP { ($attrs:expr) => { AREF ($attrs, charset_unify_map as isize) }; }
macro_rules! CHARSET_ATTR_DEUNIFIER { ($attrs:expr) => { AREF ($attrs, charset_deunifier as isize) }; }

// charset.h 296: `#define CHARSET_SYMBOL_ID(symbol)`.
macro_rules! CHARSET_SYMBOL_ID {
  ($symbol:expr) => { CHARSET_ATTR_ID! (CHARSET_SYMBOL_ATTRIBUTES! ($symbol)) };
}

/* Return an index to Vcharset_hash_table of the charset whose symbol
   is SYMBOL.  */
// charset.h 301: `#define CHARSET_SYMBOL_HASH_INDEX(symbol)`.
macro_rules! CHARSET_SYMBOL_HASH_INDEX {
  ($symbol:expr) => { hash_find (XHASH_TABLE (Vcharset_hash_table), $symbol) };
}

// charset.h 265-267: `#define CHARSET_FROM_ID(id) (charset_table.start + (id))`.
macro_rules! CHARSET_FROM_ID { ($id:expr) => { charset_table.start.offset (($id) as isize) }; }

/// The one storage byte `struct charset`'s six `bool_bf` members share, read and written
/// by their C names: `charset_bits! (unified_p, cs)` reads and
/// `charset_bits! (unified_p, cs, 0)` writes, so the port text carries the member name
/// the token gate looks for (PORTING.md 1.3/P-80, the intervals.rs `interval_bits!`
/// shape; the six families are in docs/ports/token-aliases.tsv).
#[inline(always)]
pub fn charset_bit_p (charset: *const charset, bit: u8) -> bool
{ unsafe { (*charset).bits & bit != 0 } }

#[inline(always)]
pub fn charset_bit_set (charset: *mut charset, bit: u8, value: bool)
{ unsafe { if value { (*charset).bits |= bit } else { (*charset).bits &= !bit } } }

macro_rules! charset_bits {
  (code_linear_p, $charset:expr) =>
    { charset_bit_p ($charset as *const charset, CHARSET_CODE_LINEAR_P) };
  (code_linear_p, $charset:expr, $value:expr) =>
    { charset_bit_set ($charset as *mut charset, CHARSET_CODE_LINEAR_P, ($value) as c_int != 0) };
  (iso_chars_96, $charset:expr) =>
    { charset_bit_p ($charset as *const charset, CHARSET_ISO_CHARS_96) };
  (iso_chars_96, $charset:expr, $value:expr) =>
    { charset_bit_set ($charset as *mut charset, CHARSET_ISO_CHARS_96, ($value) as c_int != 0) };
  (ascii_compatible_p, $charset:expr) =>
    { charset_bit_p ($charset as *const charset, CHARSET_ASCII_COMPATIBLE_P) };
  (ascii_compatible_p, $charset:expr, $value:expr) =>
    { charset_bit_set ($charset as *mut charset, CHARSET_ASCII_COMPATIBLE_P, ($value) as c_int != 0) };
  (supplementary_p, $charset:expr) =>
    { charset_bit_p ($charset as *const charset, CHARSET_SUPPLEMENTARY_P) };
  (supplementary_p, $charset:expr, $value:expr) =>
    { charset_bit_set ($charset as *mut charset, CHARSET_SUPPLEMENTARY_P, ($value) as c_int != 0) };
  (compact_codes_p, $charset:expr) =>
    { charset_bit_p ($charset as *const charset, CHARSET_COMPACT_CODES_P) };
  (compact_codes_p, $charset:expr, $value:expr) =>
    { charset_bit_set ($charset as *mut charset, CHARSET_COMPACT_CODES_P, ($value) as c_int != 0) };
  (unified_p, $charset:expr) =>
    { charset_bit_p ($charset as *const charset, CHARSET_UNIFIED_P) };
  (unified_p, $charset:expr, $value:expr) =>
    { charset_bit_set ($charset as *mut charset, CHARSET_UNIFIED_P, ($value) as c_int != 0) };
}

/// charset.h 305-313: `INLINE Lisp_Object charset_attributes_getter (struct charset *)`.
pub unsafe fn charset_attributes_getter (charset: *mut charset) -> Lisp_Object
{
  eassert! (ASIZE (charset_table.attributes_table) == charset_table.size as isize);
  let attrs: Lisp_Object = AREF (charset_table.attributes_table, unsafe { (*charset).id } as isize);
  eassert! (XFIXNUM (CHARSET_ATTR_ID! (attrs)) == unsafe { (*charset).id } as isize);
  return attrs;
}

/* Return the attribute vector of CHARSET.  */
// charset.h 314: `#define CHARSET_ATTRIBUTES(charset)`.
macro_rules! CHARSET_ATTRIBUTES {
  ($charset:expr) => { charset_attributes_getter ($charset) };
}

// charset.h 316-334: the per-charset accessors, C's `->` reads with the bitfield names
// replaced by the mirror's masks (PORTING.md 1.3).
macro_rules! CHARSET_ID { ($charset:expr) => { (*$charset).id }; }
macro_rules! CHARSET_DIMENSION { ($charset:expr) => { (*$charset).dimension }; }
macro_rules! CHARSET_CODE_SPACE { ($charset:expr) => { (*$charset).code_space }; }
macro_rules! CHARSET_CODE_LINEAR_P { ($charset:expr) => { (*$charset).bits & CHARSET_CODE_LINEAR_P != 0 }; }
macro_rules! CHARSET_ISO_CHARS_96 { ($charset:expr) => { (*$charset).bits & CHARSET_ISO_CHARS_96 != 0 }; }
macro_rules! CHARSET_ISO_FINAL { ($charset:expr) => { (*$charset).iso_final }; }
macro_rules! CHARSET_ISO_PLANE { ($charset:expr) => { (*$charset).iso_plane }; }
macro_rules! CHARSET_ISO_REVISION { ($charset:expr) => { (*$charset).iso_revision }; }
macro_rules! CHARSET_EMACS_MULE_ID { ($charset:expr) => { (*$charset).emacs_mule_id }; }
macro_rules! CHARSET_ASCII_COMPATIBLE_P { ($charset:expr) => { (*$charset).bits & CHARSET_ASCII_COMPATIBLE_P != 0 }; }
macro_rules! CHARSET_COMPACT_CODES_P { ($charset:expr) => { (*$charset).bits & CHARSET_COMPACT_CODES_P != 0 }; }
macro_rules! CHARSET_METHOD { ($charset:expr) => { (*$charset).method }; }
macro_rules! CHARSET_MIN_CODE { ($charset:expr) => { (*$charset).min_code }; }
macro_rules! CHARSET_MAX_CODE { ($charset:expr) => { (*$charset).max_code }; }
macro_rules! CHARSET_INVALID_CODE { ($charset:expr) => { (*$charset).invalid_code }; }
macro_rules! CHARSET_MIN_CHAR { ($charset:expr) => { (*$charset).min_char }; }
macro_rules! CHARSET_MAX_CHAR { ($charset:expr) => { (*$charset).max_char }; }
macro_rules! CHARSET_CODE_OFFSET { ($charset:expr) => { (*$charset).code_offset }; }
macro_rules! CHARSET_UNIFIED_P { ($charset:expr) => { (*$charset).bits & CHARSET_UNIFIED_P != 0 }; }

// charset.h 336-350: the attribute accessors.
macro_rules! CHARSET_NAME {
  ($charset:expr) => { CHARSET_ATTR_NAME! (CHARSET_ATTRIBUTES! ($charset)) };
}
macro_rules! CHARSET_MAP {
  ($charset:expr) => { CHARSET_ATTR_MAP! (CHARSET_ATTRIBUTES! ($charset)) };
}
macro_rules! CHARSET_DECODER {
  ($charset:expr) => { CHARSET_ATTR_DECODER! (CHARSET_ATTRIBUTES! ($charset)) };
}
macro_rules! CHARSET_ENCODER {
  ($charset:expr) => { CHARSET_ATTR_ENCODER! (CHARSET_ATTRIBUTES! ($charset)) };
}
macro_rules! CHARSET_SUBSET {
  ($charset:expr) => { CHARSET_ATTR_SUBSET! (CHARSET_ATTRIBUTES! ($charset)) };
}
macro_rules! CHARSET_SUPERSET {
  ($charset:expr) => { CHARSET_ATTR_SUPERSET! (CHARSET_ATTRIBUTES! ($charset)) };
}
macro_rules! CHARSET_UNIFY_MAP {
  ($charset:expr) => { CHARSET_ATTR_UNIFY_MAP! (CHARSET_ATTRIBUTES! ($charset)) };
}
macro_rules! CHARSET_DEUNIFIER {
  ($charset:expr) => { CHARSET_ATTR_DEUNIFIER! (CHARSET_ATTRIBUTES! ($charset)) };
}

/// charset.h 352-360: `INLINE void set_charset_attr (struct charset *, enum
/// charset_attr_index, Lisp_Object)`.
pub unsafe fn set_charset_attr (charset: *mut charset, idx: charset_attr_index, val: Lisp_Object)
{
  ASET (CHARSET_ATTRIBUTES! (charset), idx as isize, val);
}


/* Nonzero if OBJ is a valid charset symbol.  */
// charset.h 362: `#define CHARSETP(obj)`.
macro_rules! CHARSETP { ($obj:expr) => { CHARSET_SYMBOL_HASH_INDEX! ($obj) >= 0 }; }

/* Check if X is a valid charset symbol.  If not, signal an error.  */
macro_rules! CHECK_CHARSET {
  ($x:expr) => { {
    if ! SYMBOLP ($x) || CHARSET_SYMBOL_HASH_INDEX! ($x) < 0
      { wrong_type_argument (Qcharsetp (), $x); }
  } };
}


/* Check if X is a valid charset symbol.  If valid, set ID to the id
   number of the charset.  Otherwise, signal an error. */
macro_rules! CHECK_CHARSET_GET_ID {
  ($x:expr, $id:ident) => { {
    let mut idx: ptrdiff_t = 0;

    if ! SYMBOLP ($x) || { idx = CHARSET_SYMBOL_HASH_INDEX! ($x); idx < 0 }
      { wrong_type_argument (Qcharsetp (), $x); }
    $id = XFIXNUM (AREF (HASH_VALUE (XHASH_TABLE (Vcharset_hash_table), idx as hash_idx_t),
		     charset_id as isize)) as _;
  } };
}


/* Check if X is a valid charset symbol.  If valid, set ATTR to the
   attr vector of the charset.  Otherwise, signal an error. */
macro_rules! CHECK_CHARSET_GET_ATTR {
  ($x:expr, $attr:ident) => { {
    if ! SYMBOLP ($x) || NILP ({ $attr = CHARSET_SYMBOL_ATTRIBUTES! ($x); $attr })
      { wrong_type_argument (Qcharsetp (), $x); }
  } };
}


macro_rules! CHECK_CHARSET_GET_CHARSET {
  ($x:expr, $charset:ident) => { {
    let mut csid: c_int = 0;
    CHECK_CHARSET_GET_ID! ($x, csid);
    $charset = CHARSET_FROM_ID! (csid as isize);
  } };
}


/* Lookup Vcharset_ordered_list and return the first charset that
   contains the character C.  */
// charset.h 404-406: `#define CHAR_CHARSET(c)`.
macro_rules! CHAR_CHARSET {
  ($c:expr) => {
    if ($c) < 0x80 { CHARSET_FROM_ID! (charset_ascii) }
    else { char_charset ($c, Qnil (), std::ptr::null_mut ()) }
  };
}

// C's `#if false` block (charset.h 408-421) is recorded in the file comment above.


/* Return a character corresponding to the code-point CODE of CHARSET.
   Try some optimization before calling decode_char.  */

// charset.h 427-446: `DECODE_CHAR`.  The bitfield reads are the mirror's masks
// (docs/ports/token-aliases.tsv's families); the two operands are bound once so each
// `->` has one Rust spelling (C's `->` on an `expr` fragment has none).
macro_rules! DECODE_CHAR {
  ($charset:expr, $code:expr) => {
    {
      let charset: *mut charset = $charset;
      let code: c_uint = $code;
      (if ASCII_CHAR_P (code as isize) && (*charset).bits & CHARSET_ASCII_COMPATIBLE_P != 0
	 { code as c_int }
       else if code < (*charset).min_code || code > (*charset).max_code
	 { -1 }
       else if (*charset).bits & CHARSET_UNIFIED_P != 0
	 { decode_char (charset, code) }
       else if (*charset).method == CHARSET_METHOD_OFFSET
	 { if (*charset).bits & CHARSET_CODE_LINEAR_P != 0
	     { code.wrapping_sub ((*charset).min_code) as c_int + (*charset).code_offset }
	   else { decode_char (charset, code) } }
       else if (*charset).method == CHARSET_METHOD_MAP
	 { if (*charset).bits & CHARSET_CODE_LINEAR_P != 0
	      && VECTORP (CHARSET_DECODER! (charset))
	     { XFIXNUM (AREF (CHARSET_DECODER! (charset),
			      code.wrapping_sub ((*charset).min_code) as isize)) as c_int }
	   else { decode_char (charset, code) } }
       else
	 { decode_char (charset, code) })
    }
  };
}

/* Return a code point of CHAR in CHARSET.
   Try some optimization before calling encode_char.  */

// charset.h 451-477: `ENCODE_CHAR`.  `verify_expr` (charset.h 24, <verify.h>) is a
// compile-time assertion; the port keeps the comparison it asserts (both `sizeof`s are
// the one token family, docs/ports/token-aliases.tsv) and records the `verify_expr` token
// (docs/ports/macro-body-notes.tsv).  C's comma expression assigning `charset_work` is
// the block below; the two operands are bound once for the same reason as DECODE_CHAR.
macro_rules! ENCODE_CHAR {
  ($charset:expr, $c:expr) => {
    {
      let charset: *mut charset = $charset;
      let c: c_int = $c;
      let _verify_expr: bool = std::mem::size_of::<c_int> () <= std::mem::size_of::<c_int> ();
      let _ = _verify_expr;
      (if ASCII_CHAR_P (c as isize) && (*charset).bits & CHARSET_ASCII_COMPATIBLE_P != 0
	 { c as c_uint }
       else if (*charset).bits & CHARSET_UNIFIED_P != 0
	       || (*charset).method == CHARSET_METHOD_SUBSET
	       || (*charset).method == CHARSET_METHOD_SUPERSET
	 { encode_char (charset, c) }
       else if c < (*charset).min_char || c > (*charset).max_char
	 { (*charset).invalid_code }
       else if (*charset).method == CHARSET_METHOD_OFFSET
	 { if (*charset).bits & CHARSET_CODE_LINEAR_P != 0
	     { ((c as c_uint).wrapping_sub ((*charset).code_offset as c_uint))
		 .wrapping_add ((*charset).min_code) }
	   else { encode_char (charset, c) } }
       else if (*charset).method == CHARSET_METHOD_MAP
	 { if (*charset).bits & CHARSET_COMPACT_CODES_P != 0
	      && CHAR_TABLE_P (CHARSET_ENCODER! (charset))
	     { charset_work = CHAR_TABLE_REF (CHARSET_ENCODER! (charset), c as isize);
	       if NILP (charset_work) { (*charset).invalid_code }
	       else { XFIXNAT (charset_work) as c_uint } }
	   else { encode_char (charset, c) } }
       else
	 { encode_char (charset, c) })
    }
  };
}


/* Set to true when a charset map is loaded to warn that a buffer text
   and a string data may be relocated.  */


/* Set CHARSET to the charset highest priority of C, CODE to the
   code-point of C in CHARSET.  */
// charset.h 484: `#define SPLIT_CHAR(c, charset, code)`.
macro_rules! SPLIT_CHAR {
  ($c:expr, $charset:ident, $code:ident) => {
    { $charset = char_charset ($c, Qnil (), &mut $code); }
  };
}


// charset.h 488-490.
pub const ISO_MAX_DIMENSION: c_int = 3;
pub const ISO_MAX_CHARS: c_int = 2;
pub const ISO_MAX_FINAL: c_int = 0x80;	/* only 0x30..0xFF are used */

/* Mapping table from ISO2022's charset (specified by DIMENSION,
   CHARS, and FINAL_CHAR) to Emacs's charset ID.  Should be accessed by
   macro ISO_CHARSET_TABLE (DIMENSION, CHARS, FINAL_CHAR).  */

/* A charset of type iso2022 who has DIMENSION, CHARS_96, and FINAL
   (final character).  */
// charset.h 499: `#define ISO_CHARSET_TABLE(dimension, chars_96, final)`.
macro_rules! ISO_CHARSET_TABLE {
  ($dimension:expr, $chars_96:expr, $final:expr) => {
    iso_charset_table[($dimension) as usize - 1][($chars_96) as usize][($final) as usize]
  };
}

/* Nonzero if the charset who has FAST_MAP may contain C.  */
// charset.h 503-506: `#define CHARSET_FAST_MAP_REF(c, fast_map)`.
macro_rules! CHARSET_FAST_MAP_REF {
  ($c:expr, $fast_map:expr) => {
    (if ($c) < 0x10000
       { *$fast_map.offset ((($c) >> 10) as isize) & (1 << ((($c) >> 7) & 7)) != 0 }
     else
       { *$fast_map.offset (((($c) >> 15) + 62) as isize) & (1 << ((($c) >> 12) & 7)) != 0 })
  };
}

// charset.h 508-514: `#define CHARSET_FAST_MAP_SET(c, fast_map)`.
macro_rules! CHARSET_FAST_MAP_SET {
  ($c:expr, $fast_map:expr) => { {
    if ($c) < 0x10000
      { *$fast_map.offset ((($c) >> 10) as isize) |= 1 << ((($c) >> 7) & 7); }
    else
      { *$fast_map.offset (((($c) >> 15) + 62) as isize) |= 1 << ((($c) >> 12) & 7); }
  } };
}


/* True if CHARSET may contain the character C.  */
// charset.h 519-529: `CHAR_CHARSET_P`.
  macro_rules! CHAR_CHARSET_P {
    ($c:expr, $charset:expr) => {
      {
	let charset: *mut charset = $charset;
	let c: c_int = $c;
	((ASCII_CHAR_P (c as isize) && (*charset).bits & CHARSET_ASCII_COMPATIBLE_P != 0)
	 || (if (*charset).bits & CHARSET_UNIFIED_P != 0
		|| (*charset).method == CHARSET_METHOD_SUBSET
		|| (*charset).method == CHARSET_METHOD_SUPERSET
	       { encode_char (charset, c) != (*charset).invalid_code }
	     else if CHARSET_FAST_MAP_REF! (c, (*charset).fast_map.as_ptr ())
	       { if (*charset).method == CHARSET_METHOD_OFFSET
		   { c >= (*charset).min_char && c <= (*charset).max_char }
		 else if (*charset).method == CHARSET_METHOD_MAP
			 && (*charset).bits & CHARSET_COMPACT_CODES_P != 0
			 && CHAR_TABLE_P (CHARSET_ENCODER! (charset))
		   { ! NILP (CHAR_TABLE_REF (CHARSET_ENCODER! (charset), c as isize)) }
		 else
		   { encode_char (charset, c) != (*charset).invalid_code } }
	     else { false }))
      }
    };
  }

/* Special macros for emacs-mule encoding.  */

/* Leading-code followed by extended leading-code.    DIMENSION/COLUMN */
pub const EMACS_MULE_LEADING_CODE_PRIVATE_11: c_int = 0x9A; /* 1/1 */
pub const EMACS_MULE_LEADING_CODE_PRIVATE_12: c_int = 0x9B; /* 1/2 */
pub const EMACS_MULE_LEADING_CODE_PRIVATE_21: c_int = 0x9C; /* 2/2 */
pub const EMACS_MULE_LEADING_CODE_PRIVATE_22: c_int = 0x9D; /* 2/2 */

/// The file-scope objects the header declares (`extern`).  A unit that defines one
/// shadows this glob import with its own `#[unsafe(no_mangle)] pub static mut`
/// (PORTING.md P-21, 0.1-10).
pub mod charset_objs {
  use super::*;

  unsafe extern "C" {
    /// C: charset.h 244: `extern Lisp_Object Vcharset_hash_table;`
    pub static mut Vcharset_hash_table: Lisp_Object;
    /// C: charset.h 265: `extern struct charset_table charset_table;`
    pub static mut charset_table: charset_table;
    /// C: charset.h 269: `extern Lisp_Object Vcharset_ordered_list;`
    pub static mut Vcharset_ordered_list: Lisp_Object;
    /// C: charset.h 270: `extern Lisp_Object Vcharset_non_preferred_head;`
    pub static mut Vcharset_non_preferred_head: Lisp_Object;
    /// C: charset.h 272: `extern EMACS_UINT charset_ordered_list_tick;`
    pub static mut charset_ordered_list_tick: EMACS_UINT;
    /// C: charset.h 274: `extern Lisp_Object Viso_2022_charset_list;`
    pub static mut Viso_2022_charset_list: Lisp_Object;
    /// C: charset.h 275: `extern Lisp_Object Vemacs_mule_charset_list;`
    pub static mut Vemacs_mule_charset_list: Lisp_Object;
    /// C: charset.h 277: `extern int emacs_mule_charset[256];`
    pub static mut emacs_mule_charset: [c_int; 256];
    /// C: charset.h 446: `extern Lisp_Object charset_work;`
    pub static mut charset_work: Lisp_Object;
    /// C: charset.h 479: `extern bool charset_map_loaded;`
    pub static mut charset_map_loaded: bool;
    /// C: charset.h 495: `extern int
    /// iso_charset_table[ISO_MAX_DIMENSION][ISO_MAX_CHARS][ISO_MAX_FINAL];`
    pub static mut iso_charset_table:
      [[[c_int; ISO_MAX_FINAL as usize]; ISO_MAX_CHARS as usize]; ISO_MAX_DIMENSION as usize];
    /// C: charset.h 545: `extern int charset_ascii, charset_eight_bit;`
    pub static mut charset_ascii: c_int;
    /// C: charset.h 545: `extern int charset_ascii, charset_eight_bit;`
    pub static mut charset_eight_bit: c_int;
    /// C: charset.h 546: `extern int charset_unicode;`
    pub static mut charset_unicode: c_int;
    /// C: charset.h 547: `extern int charset_jisx0201_roman;`
    pub static mut charset_jisx0201_roman: c_int;
    /// C: charset.h 548: `extern int charset_jisx0208_1978;`
    pub static mut charset_jisx0208_1978: c_int;
    /// C: charset.h 549: `extern int charset_jisx0208;`
    pub static mut charset_jisx0208: c_int;
    /// C: charset.h 550: `extern int charset_ksc5601;`
    pub static mut charset_ksc5601: c_int;
    /// C: charset.h 552: `extern int charset_unibyte;`
    pub static mut charset_unibyte: c_int;
  }
}
use charset_objs::*;

/// The prototypes this unit defines (PORTING.md 0.1-10): a glob import so the unit's own
/// `#[unsafe(no_mangle)] pub unsafe extern "C" fn` definitions are not a second
/// declaration of the same name at the crate root (E0428).
pub mod charset_decls {
  use super::*;

  unsafe extern "C" {
    /// C: charset.h 554: `extern struct charset *char_charset (int, Lisp_Object, unsigned *);`
    pub fn char_charset (c: c_int, charset_list: Lisp_Object, code: *mut c_uint)
			 -> *mut charset;
    /// C: charset.h 557: `extern int decode_char (struct charset *, unsigned);`
    pub fn decode_char (charset: *mut charset, code: c_uint) -> c_int;
    /// C: charset.h 558: `extern unsigned encode_char (struct charset *, int);`
    pub fn encode_char (charset: *mut charset, c: c_int) -> c_uint;
    /// C: charset.h 559: `extern int string_xstring_p (Lisp_Object);`
    pub fn string_xstring_p (string: Lisp_Object) -> c_int;
    /// C: charset.h 561-563: `extern void map_charset_chars (void (*) (Lisp_Object,
    /// Lisp_Object), Lisp_Object, Lisp_Object, struct charset *, unsigned, unsigned);`
    pub fn map_charset_chars (c_function: Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object)>,
			      function: Lisp_Object, arg: Lisp_Object,
			      charset: *mut charset, from: c_uint, to: c_uint);
  }
}
use charset_decls::*;

/// C: charset.h 555: `extern Lisp_Object charset_attributes (int);`  The header declares
/// it and the reference tree defines it nowhere, so it stays outside `charset_decls`.
unsafe extern "C" {
  pub fn charset_attributes (id: c_int) -> Lisp_Object;
}
