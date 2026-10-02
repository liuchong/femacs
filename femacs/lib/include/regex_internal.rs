// femacs: mirror of lib/regex_internal.h @ fd7afced403 (GNU Emacs 32.0.50; PORTING.md 0.1-2).
//
// Declarations only: types, constants, `macro_rules!` and the inline helpers the
// reference spells `static inline` (they carry no C symbol, so the mirror may define
// them; PORTING.md 1.3/0.1-2, P-12/P-37).  The `#if`s are the frozen configuration's
// (P-08): `_LIBC` is undefined, `_REGEX_AVOID_UCHAR_H` is 1, `_REGEX_LARGE_OFFSETS` is 1,
// `HAVE_ISBLANK`/`HAVE_DECL_ISBLANK` are 1, no `GNULIB_*` lock is defined and `DEBUG` is
// undefined, so the lock macros are the no-op arm, `Idx` is `ssize_t` and `wchar_t` is
// C's.  The dead arms the reference's `#if`s would still *show* keep their text (P-51);
// where a dead arm would change layout or define a second item it is `#[cfg]`-gated.

include!("limits.in.rs");
include!("stdint.in.rs");
include!("stdcountof.rs");
include!("stdlib.rs");
include!("string.rs");
include!("ctype.rs");
include!("wchar.rs");
include!("wctype.rs");
include!("langinfo.rs");
include!("attribute.rs");
include!("verify.rs");
include!("gettext.rs");
include!("regex.rs");


/* regex_internal.h 21: the include guard itself is a `#define` the gate names.  */
pub const _REGEX_INTERNAL_H: c_int = 1;

/* regex_internal.h 36-44: unlike the system `assert`, `DEBUG_ASSERT` is `assume` in the
   non-`DEBUG` build.  Both arms are in the reference text (P-51); the port keeps the
   frozen one.  */
macro_rules! DEBUG_ASSERT {
    ($x:expr) => { assume!($x) };
}

/* regex_internal.h 49-67: the four lock spellings.  The frozen configuration defines no
   `GNULIB_LOCK`/`GNULIB_PTHREAD` and no `GNULIB_REGEX_SINGLE_THREAD`, so C takes the last
   arm: `lock_define` is empty and the others are no-ops that keep `dfa` "used".  */
macro_rules! lock_define {
    ($name:expr) => {};
}
macro_rules! lock_init {
    ($lock:expr) => { 0 };
}
macro_rules! lock_fini {
    ($lock:expr) => { () };
}
macro_rules! lock_lock {
    ($lock:expr) => {{ let dfa = $lock; let _ = dfa; }};
}
macro_rules! lock_unlock {
    ($lock:expr) => { () };
}

/* regex_internal.h 70-72: the `isblank` fallback is dead (HAVE_ISBLANK is 1).  */
macro_rules! isblank {
    ($ch:expr) => { ($ch) == b' ' as c_int || ($ch) == b'\t' as c_int };
}

/* regex_internal.h 87-90 */
macro_rules! isascii {
    ($c:expr) => { (($c) & !0x7f) == 0 };
}

/* regex_internal.h 94-95: `_RE_DEFINE_LOCALE_FUNCTIONS` only exists under `_LIBC`.  */
pub const _RE_DEFINE_LOCALE_FUNCTIONS: c_int = 1;

/* regex_internal.h 111-115 */
pub const ASCII_CHARS: c_int = 0x80;
pub const SBC_MAX: Idx = (UCHAR_MAX + 1) as Idx;
pub const COLL_ELEM_LEN_MAX: c_int = 8;
pub const NEWLINE_CHAR: char = '\n';
pub const WIDE_NEWLINE_CHAR: wchar_t = '\n' as wchar_t;

/* regex_internal.h 126-131: the `_REGEX_AVOID_UCHAR_H` arm is the live one, so `wchar_t`
   and `wctype_t` are C's types; the `#else` arm's two `#define`s are still named, and
   `macro_rules!` may not shadow a type alias's name, so they live here as the names the
   ledger expects and the aliases come from the <wchar.h>/<wctype.h> mirrors.  */
macro_rules! wchar_t {
    () => { char32_t };
}
macro_rules! wctype_t {
    () => { c32_type_test_t };
}

/* regex_internal.h 133-162: the seven `__name` renames (the `_REGEX_AVOID_UCHAR_H` half).
   Both halves are in the reference text, so C's macro table has two bodies for each and
   the body comparison is skipped; the names must be here either way.  */
macro_rules! __regfree {
    ($p:expr) => { regfree ($p) };
}
macro_rules! __wctype {
    ($property:expr) => { wctype ($property) };
}
macro_rules! __iswalnum {
    ($wc:expr) => { iswalnum ($wc) };
}
macro_rules! __iswctype {
    ($wc:expr, $ty:expr) => { iswctype ($wc, $ty) };
}
macro_rules! __towlower {
    ($wc:expr) => { towlower ($wc) };
}
macro_rules! __towupper {
    ($wc:expr) => { towupper ($wc) };
}
macro_rules! __btowc {
    ($c:expr) => { btowc ($c) };
}
macro_rules! __mbrtowc {
    ($pwc:expr, $s:expr, $n:expr, $ps:expr) => { mbrtowc ($pwc, $s, $n, $ps) };
}
macro_rules! __wcrtomb {
    ($s:expr, $wc:expr, $ps:expr) => { wcrtomb ($s, $wc, $ps) };
}

/* regex_internal.h 170-187: `ULONG_WIDTH` comes from <limits.h> on the frozen target, so
   the `REGEX_COB*` chain is dead text; the names are kept (P-51) and the width is
   `limits.in.rs`'s.  */
macro_rules! REGEX_UINTEGER_WIDTH {
    ($max:expr) => { REGEX_COB128 ($max) };
}
macro_rules! REGEX_COB128 {
    ($n:expr) => { REGEX_COB64 (($n) >> 31 >> 31 >> 2) + REGEX_COB64 ($n) };
}
macro_rules! REGEX_COB64 {
    ($n:expr) => { REGEX_COB32 (($n) >> 31 >> 1) + REGEX_COB32 ($n) };
}
macro_rules! REGEX_COB32 {
    ($n:expr) => { REGEX_COB16 (($n) >> 16) + REGEX_COB16 ($n) };
}
macro_rules! REGEX_COB16 {
    ($n:expr) => { REGEX_COB8 (($n) >> 8) + REGEX_COB8 ($n) };
}
macro_rules! REGEX_COB8 {
    ($n:expr) => { REGEX_COB4 (($n) >> 4) + REGEX_COB4 ($n) };
}
macro_rules! REGEX_COB4 {
    ($n:expr) => { (!!(($n) & 8) + !!(($n) & 4) + !!(($n) & 2) + (($n) & 1)) };
}

/* regex_internal.h 193-200: `_REGEX_LARGE_OFFSETS` is 1 and `SSIZE_MAX` is defined, so
   `IDX_MAX` is `SSIZE_MAX`.  */
pub const SSIZE_MAX: isize = isize::MAX;
pub const IDX_MAX: Idx = SSIZE_MAX;

/* regex_internal.h 193-194: the index type.  `_REGEX_LARGE_OFFSETS` is on, so it is
   `regoff_t` (== `ssize_t`).  */
pub type Idx = regoff_t;

/* regex_internal.h 203: the type of object sizes.  */
pub type re_hashval_t = __re_size_t;

/* regex_internal.h 207-217 */
pub type bitset_word_t = c_ulong;
pub const BITSET_WORD_MAX: bitset_word_t = ULONG_MAX as bitset_word_t;
pub const BITSET_WORD_BITS: Idx = ULONG_WIDTH as Idx;
pub const BITSET_WORDS: usize =
    ((SBC_MAX + BITSET_WORD_BITS - 1) / BITSET_WORD_BITS) as usize;

pub type bitset_t = [bitset_word_t; BITSET_WORDS];
pub type re_bitset_ptr_t = *mut bitset_word_t;
pub type re_const_bitset_ptr_t = *const bitset_word_t;

/* regex_internal.h 219-228 */
pub const PREV_WORD_CONSTRAINT: c_int = 0x0001;
pub const PREV_NOTWORD_CONSTRAINT: c_int = 0x0002;
pub const NEXT_WORD_CONSTRAINT: c_int = 0x0004;
pub const NEXT_NOTWORD_CONSTRAINT: c_int = 0x0008;
pub const PREV_NEWLINE_CONSTRAINT: c_int = 0x0010;
pub const NEXT_NEWLINE_CONSTRAINT: c_int = 0x0020;
pub const PREV_BEGBUF_CONSTRAINT: c_int = 0x0040;
pub const NEXT_ENDBUF_CONSTRAINT: c_int = 0x0080;
pub const WORD_DELIM_CONSTRAINT: c_int = 0x0100;
pub const NOT_WORD_DELIM_CONSTRAINT: c_int = 0x0200;

/* regex_internal.h 230-243 */
#[repr(C)]
#[derive(Clone, Copy)]
pub enum re_context_type {
  INSIDE_WORD = (PREV_WORD_CONSTRAINT | NEXT_WORD_CONSTRAINT) as isize,
  WORD_FIRST = (PREV_NOTWORD_CONSTRAINT | NEXT_WORD_CONSTRAINT) as isize,
  WORD_LAST = (PREV_WORD_CONSTRAINT | NEXT_NOTWORD_CONSTRAINT) as isize,
  INSIDE_NOTWORD = (PREV_NOTWORD_CONSTRAINT | NEXT_NOTWORD_CONSTRAINT) as isize,
  LINE_FIRST = PREV_NEWLINE_CONSTRAINT as isize,
  LINE_LAST = NEXT_NEWLINE_CONSTRAINT as isize,
  BUF_FIRST = PREV_BEGBUF_CONSTRAINT as isize,
  BUF_LAST = NEXT_ENDBUF_CONSTRAINT as isize,
  WORD_DELIM = WORD_DELIM_CONSTRAINT as isize,
  NOT_WORD_DELIM = NOT_WORD_DELIM_CONSTRAINT as isize,
}

/* regex_internal.h 245-250 */
#[repr(C)]
pub struct re_node_set {
  pub alloc: Idx,
  pub nelem: Idx,
  pub elems: *mut Idx,
}

/* regex_internal.h 252-297 */
pub const NON_TYPE: c_int = 0;
pub const CHARACTER: c_int = 1;
pub const END_OF_RE: c_int = 2;
pub const SIMPLE_BRACKET: c_int = 3;
pub const OP_BACK_REF: c_int = 4;
pub const OP_PERIOD: c_int = 5;
pub const COMPLEX_BRACKET: c_int = 6;
pub const OP_UTF8_PERIOD: c_int = 7;
pub const EPSILON_BIT: c_int = 8;
pub const ANCHOR: c_int = EPSILON_BIT | 4;
pub const CONCAT: c_int = 16;
pub const SUBEXP: c_int = 17;

macro_rules! IS_EPSILON_NODE {
    ($type:expr) => { ($type) & EPSILON_BIT as c_uint };
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum re_token_type_t {
  CHARACTER = 1,
  END_OF_RE = 2,
  SIMPLE_BRACKET = 3,
  OP_BACK_REF = 4,
  OP_PERIOD = 5,
  COMPLEX_BRACKET = 6,
  OP_UTF8_PERIOD = 7,
  OP_OPEN_SUBEXP = (EPSILON_BIT | 0) as isize,
  OP_CLOSE_SUBEXP = (EPSILON_BIT | 1) as isize,
  OP_ALT = (EPSILON_BIT | 2) as isize,
  OP_DUP_ASTERISK = (EPSILON_BIT | 3) as isize,
  ANCHOR = (EPSILON_BIT | 4) as isize,
  CONCAT = 16,
  SUBEXP = 17,
  OP_DUP_PLUS = 18,
  OP_DUP_QUESTION = 19,
  OP_OPEN_BRACKET = 20,
  OP_CLOSE_BRACKET = 21,
  OP_CHARSET_RANGE = 22,
  OP_OPEN_DUP_NUM = 23,
  OP_CLOSE_DUP_NUM = 24,
  OP_NON_MATCH_LIST = 25,
  OP_OPEN_COLL_ELEM = 26,
  OP_CLOSE_COLL_ELEM = 27,
  OP_OPEN_EQUIV_CLASS = 28,
  OP_CLOSE_EQUIV_CLASS = 29,
  OP_OPEN_CHAR_CLASS = 30,
  OP_CLOSE_CHAR_CLASS = 31,
  OP_WORD = 32,
  OP_NOTWORD = 33,
  OP_SPACE = 34,
  OP_NOTSPACE = 35,
  BACK_SLASH = 36,
}

/* regex_internal.h 299-341: `_LIBC` is undefined, so the `re_charset_t` ranges are
   `wchar_t *` and the three glibc-only pointer members are absent.  */
#[repr(C)]
pub struct re_charset_t {
  pub mbchars: *mut wchar_t,
  pub range_starts: *mut wchar_t,
  pub range_ends: *mut wchar_t,
  pub char_classes: *mut wctype_t,
  /// C: `unsigned int non_match : 1;`
  pub bits: c_uint,
  pub nmbchars: Idx,
  pub ncoll_syms: Idx,
  pub nequiv_classes: Idx,
  pub nranges: Idx,
  pub nchar_classes: Idx,
}

/* regex_internal.h 343-370.  The union `opr` and the five trailing flag bitfields are
   packed the way PORTING.md 1.3 prescribes: one integer for the bitfield run, the union
   spelled as `#[repr(C)] union` with C's member names.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub union re_token_opr_t {
  pub c: u8,
  pub sbcset: re_bitset_ptr_t,
  pub mbcset: *mut re_charset_t,
  pub idx: Idx,
  pub ctx_type: re_context_type,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct re_token_t {
  pub opr: re_token_opr_t,
  /// C: `re_token_type_t type : 8; unsigned int constraint : 10; unsigned int duplicated : 1;
  /// unsigned int opt_subexp : 1; unsigned int accept_mb : 1; unsigned int mb_partial : 1;
  /// unsigned int word_char : 1;`
  pub bits: c_uint,
}

/* regex_internal.h 372-422 */
#[repr(C)]
pub struct re_string_t {
  pub raw_mbs: *const u8,
  pub mbs: *mut u8,
  pub wcs: *mut wint_t,
  pub offsets: *mut Idx,
  pub cur_state: mbstate_t,
  pub raw_mbs_idx: Idx,
  pub valid_len: Idx,
  pub valid_raw_len: Idx,
  pub bufs_len: Idx,
  pub cur_idx: Idx,
  pub raw_len: Idx,
  pub len: Idx,
  pub raw_stop: Idx,
  pub stop: Idx,
  pub tip_context: c_uint,
  pub trans: RE_TRANSLATE_TYPE,
  pub word_char: re_const_bitset_ptr_t,
  pub icase: u8,
  pub is_utf8: u8,
  pub map_notascii: u8,
  pub mbs_allocated: u8,
  pub offsets_needed: u8,
  pub newline_anchor: u8,
  pub word_ops_used: u8,
  pub mb_cur_max: c_int,
}

/* regex_internal.h 424-430: `IS_IN` is `false` outside glibc.  */
macro_rules! IS_IN {
    ($libc:expr) => { false };
}

/* regex_internal.h 432-446: the accessor macros; `->` is not a token, so the bodies are
   the field names and the arithmetic.  */
macro_rules! re_string_peek_byte {
    ($pstr:expr, $offset:expr) => { *(*$pstr).mbs.add (((*$pstr).cur_idx + $offset) as usize) };
}
macro_rules! re_string_fetch_byte {
    ($pstr:expr) => {{ let r = *(*$pstr).mbs.add ((*$pstr).cur_idx as usize); (*$pstr).cur_idx += 1; r }};
}
macro_rules! re_string_first_byte {
    ($pstr:expr, $idx:expr) => { ($idx) == (*$pstr).valid_len || *(*$pstr).wcs.add($idx as usize) != WEOF };
}
macro_rules! re_string_is_single_byte_char {
    ($pstr:expr, $idx:expr) => { *(*$pstr).wcs.add($idx as usize) != WEOF
        && ((*$pstr).valid_len == ($idx) + 1 || *(*$pstr).wcs.add(($idx + 1) as usize) != WEOF) };
}
macro_rules! re_string_eoi {
    ($pstr:expr) => { (*$pstr).stop <= (*$pstr).cur_idx };
}
macro_rules! re_string_cur_idx {
    ($pstr:expr) => { (*$pstr).cur_idx };
}
macro_rules! re_string_get_buffer {
    ($pstr:expr) => { (*$pstr).mbs };
}
macro_rules! re_string_length {
    ($pstr:expr) => { (*$pstr).len };
}
macro_rules! re_string_byte_at {
    ($pstr:expr, $idx:expr) => { *(*$pstr).mbs.add (($idx) as usize) };
}
macro_rules! re_string_skip_bytes {
    ($pstr:expr, $idx:expr) => { (*$pstr).cur_idx += ($idx) };
}
macro_rules! re_string_set_index {
    ($pstr:expr, $idx:expr) => { (*$pstr).cur_idx = ($idx) };
}

/* regex_internal.h 448-453 */
macro_rules! MAX {
    ($a:expr, $b:expr) => { if ($a) < ($b) { ($b) } else { ($a) } };
}
macro_rules! MIN {
    ($a:expr, $b:expr) => { if ($a) < ($b) { ($a) } else { ($b) } };
}

/* regex_internal.h 455-461: `HAVE_MALLOC_0_NONNULL` is 1 on the frozen target.  */
macro_rules! re_malloc {
    ($t:ty, $n:expr) => { malloc (($n) as usize * std::mem::size_of::<$t> ()) as *mut $t };
}
macro_rules! re_realloc {
    ($p:expr, $t:ty, $n:expr) => { realloc ($p as *mut c_void, ($n) as usize * std::mem::size_of::<$t> ()) as *mut $t };
}
macro_rules! re_free {
    ($p:expr) => { free ($p as *mut c_void) };
}

/* regex_internal.h 463-487 */
#[repr(C)]
pub struct bin_tree_t {
  pub parent: *mut bin_tree_t,
  pub left: *mut bin_tree_t,
  pub right: *mut bin_tree_t,
  pub first: *mut bin_tree_t,
  pub next: *mut bin_tree_t,
  pub token: re_token_t,
  pub node_idx: Idx,
}

pub const BIN_TREE_STORAGE_SIZE: usize =
    ((1024 - std::mem::size_of::<*mut c_void> ()) / std::mem::size_of::<bin_tree_t> ());

#[repr(C)]
pub struct bin_tree_storage_t {
  pub next: *mut bin_tree_storage_t,
  pub data: [bin_tree_t; BIN_TREE_STORAGE_SIZE],
}

/* regex_internal.h 489-503 */
pub const CONTEXT_WORD: c_int = 1;
pub const CONTEXT_NEWLINE: c_int = CONTEXT_WORD << 1;
pub const CONTEXT_BEGBUF: c_int = CONTEXT_NEWLINE << 1;
pub const CONTEXT_ENDBUF: c_int = CONTEXT_BEGBUF << 1;

macro_rules! IS_WORD_CONTEXT {
    ($c:expr) => { ($c) & CONTEXT_WORD as c_uint };
}
macro_rules! IS_NEWLINE_CONTEXT {
    ($c:expr) => { ($c) & CONTEXT_NEWLINE as c_uint };
}
macro_rules! IS_BEGBUF_CONTEXT {
    ($c:expr) => { ($c) & CONTEXT_BEGBUF as c_uint };
}
macro_rules! IS_ENDBUF_CONTEXT {
    ($c:expr) => { ($c) & CONTEXT_ENDBUF as c_uint };
}
macro_rules! IS_ORDINARY_CONTEXT {
    ($c:expr) => { ($c) == 0 };
}

macro_rules! IS_WORD_CHAR {
    ($ch:expr) => { isalnum ($ch) != 0 || ($ch) == b'_' as c_int };
}
macro_rules! IS_NEWLINE {
    ($ch:expr) => { ($ch) == NEWLINE_CHAR as c_int };
}
macro_rules! IS_WIDE_WORD_CHAR {
    ($ch:expr) => { __iswalnum! ($ch) != 0 || ($ch) == b'_' as wint_t };
}
macro_rules! IS_WIDE_NEWLINE {
    ($ch:expr) => { ($ch) == WIDE_NEWLINE_CHAR as wint_t };
}

macro_rules! NOT_SATISFY_PREV_CONSTRAINT {
    ($constraint:expr, $context:expr) => {
        ((($constraint) & PREV_WORD_CONSTRAINT as c_uint) != 0 && IS_WORD_CONTEXT! ($context) == 0)
        || ((($constraint) & PREV_NOTWORD_CONSTRAINT as c_uint) != 0 && IS_WORD_CONTEXT! ($context) != 0)
        || ((($constraint) & PREV_NEWLINE_CONSTRAINT as c_uint) != 0 && IS_NEWLINE_CONTEXT! ($context) == 0)
        || ((($constraint) & PREV_BEGBUF_CONSTRAINT as c_uint) != 0 && IS_BEGBUF_CONTEXT! ($context) == 0)
    };
}
macro_rules! NOT_SATISFY_NEXT_CONSTRAINT {
    ($constraint:expr, $context:expr) => {
        ((($constraint) & NEXT_WORD_CONSTRAINT as c_uint) != 0 && IS_WORD_CONTEXT! ($context) == 0)
        || ((($constraint) & NEXT_NOTWORD_CONSTRAINT as c_uint) != 0 && IS_WORD_CONTEXT! ($context) != 0)
        || ((($constraint) & NEXT_NEWLINE_CONSTRAINT as c_uint) != 0 && IS_NEWLINE_CONTEXT! ($context) == 0)
        || ((($constraint) & NEXT_ENDBUF_CONSTRAINT as c_uint) != 0 && IS_ENDBUF_CONTEXT! ($context) == 0)
    };
}

/* regex_internal.h 505-524 */
#[repr(C)]
pub struct re_dfastate_t {
  pub hash: re_hashval_t,
  pub nodes: re_node_set,
  pub non_eps_nodes: re_node_set,
  pub inveclosure: re_node_set,
  pub entrance_nodes: *mut re_node_set,
  pub trtable: *mut *mut re_dfastate_t,
  pub word_trtable: *mut *mut re_dfastate_t,
  /// C: `unsigned int context : 4; unsigned int halt : 1; unsigned int accept_mb : 1;
  /// unsigned int has_backref : 1; unsigned int has_constraint : 1;`
  pub bits: c_uint,
}

/* regex_internal.h 526-531 */
#[repr(C)]
pub struct re_state_table_entry {
  pub num: Idx,
  pub alloc: Idx,
  pub array: *mut *mut re_dfastate_t,
}

/* regex_internal.h 535-541 */
#[repr(C)]
pub struct state_array_t {
  pub next_idx: Idx,
  pub alloc: Idx,
  pub array: *mut *mut re_dfastate_t,
}

/* regex_internal.h 545-552 */
#[repr(C)]
pub struct re_sub_match_last_t {
  pub node: Idx,
  pub str_idx: Idx,
  pub path: state_array_t,
}

/* regex_internal.h 555-567 */
#[repr(C)]
pub struct re_sub_match_top_t {
  pub str_idx: Idx,
  pub node: Idx,
  pub path: *mut state_array_t,
  pub alasts: Idx,
  pub nlasts: Idx,
  pub lasts: *mut *mut re_sub_match_last_t,
}

/* regex_internal.h 569-577 */
#[repr(C)]
pub struct re_backref_cache_entry {
  pub node: Idx,
  pub str_idx: Idx,
  pub subexp_from: Idx,
  pub subexp_to: Idx,
  pub eps_reachable_subexps_map: bitset_word_t,
  pub more: c_char,
}

/* regex_internal.h 579-599 */
#[repr(C)]
pub struct re_match_context_t {
  pub input: re_string_t,
  pub dfa: *const re_dfa_t,
  pub eflags: c_int,
  pub match_last: Idx,
  pub last_node: Idx,
  pub state_log: *mut *mut re_dfastate_t,
  pub state_log_top: Idx,
  pub nbkref_ents: Idx,
  pub abkref_ents: Idx,
  pub bkref_ents: *mut re_backref_cache_entry,
  pub max_mb_elem_len: c_int,
  pub nsub_tops: Idx,
  pub asub_tops: Idx,
  pub sub_tops: *mut *mut re_sub_match_top_t,
}

/* regex_internal.h 601-609 */
#[repr(C)]
pub struct re_sift_context_t {
  pub sifted_states: *mut *mut re_dfastate_t,
  pub limited_states: *mut *mut re_dfastate_t,
  pub last_node: Idx,
  pub last_str_idx: Idx,
  pub limits: re_node_set,
}

/* regex_internal.h 611-616 */
#[repr(C)]
pub struct re_fail_stack_ent_t {
  pub idx: Idx,
  pub node: Idx,
  pub regs: *mut regmatch_t,
  pub eps_via_nodes: re_node_set,
}

/* regex_internal.h 618-623 */
#[repr(C)]
pub struct re_fail_stack_t {
  pub num: Idx,
  pub alloc: Idx,
  pub stack: *mut re_fail_stack_ent_t,
}

/* regex_internal.h 625-680 */
#[repr(C)]
pub struct re_dfa_t {
  pub nodes: *mut re_token_t,
  pub nodes_alloc: usize,
  pub nodes_len: usize,
  pub nexts: *mut Idx,
  pub org_indices: *mut Idx,
  pub edests: *mut re_node_set,
  pub eclosures: *mut re_node_set,
  pub inveclosures: *mut re_node_set,
  pub state_table: *mut re_state_table_entry,
  pub init_state: *mut re_dfastate_t,
  pub init_state_word: *mut re_dfastate_t,
  pub init_state_nl: *mut re_dfastate_t,
  pub init_state_begbuf: *mut re_dfastate_t,
  pub str_tree: *mut bin_tree_t,
  pub str_tree_storage: *mut bin_tree_storage_t,
  pub sb_char: re_bitset_ptr_t,
  pub str_tree_storage_idx: c_int,
  pub state_hash_mask: re_hashval_t,
  pub init_node: Idx,
  pub nbackref: Idx,
  pub used_bkref_map: bitset_word_t,
  pub completed_bkref_map: bitset_word_t,
  /// C: `unsigned int has_plural_match : 1; unsigned int has_mb_node : 1;
  /// unsigned int is_utf8 : 1; unsigned int map_notascii : 1; unsigned int word_ops_used : 1;`
  pub bits: c_uint,
  pub mb_cur_max: c_int,
  pub word_char: bitset_t,
  pub syntax: reg_syntax_t,
  pub subexp_map: *mut Idx,
  /// C: `#ifdef DEBUG char *re_str; #endif` -- dead on the frozen target, kept as text
  /// (P-51) behind a cfg so it cannot change the layout.
  #[cfg(feature = "regex_debug_re_str")]
  pub re_str: *mut c_char,
}

/* regex_internal.h 682-687 */
macro_rules! re_node_set_init_empty {
    ($set:expr) => { memset ($set as *mut c_void, 0, std::mem::size_of::<re_node_set> ()) };
}
macro_rules! re_node_set_remove {
    ($set:expr, $id:expr) => {
        re_node_set_remove_at ($set, re_node_set_contains ($set, $id) - 1)
    };
}
macro_rules! re_node_set_empty {
    ($p:expr) => { (*$p).nelem = 0 };
}
macro_rules! re_node_set_free {
    ($set:expr) => { re_free! ((*$set).elems) };
}

/* regex_internal.h 688-698 */
#[repr(C)]
#[derive(Clone, Copy)]
pub enum bracket_elem_type {
  SB_CHAR = 0,
  MB_CHAR = 1,
  EQUIV_CLASS = 2,
  COLL_SYM = 3,
  CHAR_CLASS = 4,
}

/* regex_internal.h 700-709 */
#[repr(C)]
#[derive(Clone, Copy)]
pub union bracket_elem_opr_t {
  pub ch: u8,
  pub name: *mut u8,
  pub wch: wchar_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct bracket_elem_t {
  pub type_: bracket_elem_type,
  pub opr: bracket_elem_opr_t,
}

/* regex_internal.h 712-780: the bitset inline helpers (no C symbol; the mirror defines
   them the way the reference's `static inline` does).  */
#[inline(always)]
pub unsafe fn bitset_set (set: *mut bitset_word_t, i: Idx)
{
  *set.add((i / BITSET_WORD_BITS) as usize) |= (1 as bitset_word_t) << (i % BITSET_WORD_BITS);
}

#[inline(always)]
pub unsafe fn bitset_clear (set: *mut bitset_word_t, i: Idx)
{
  *set.add((i / BITSET_WORD_BITS) as usize) &=
      !((1 as bitset_word_t) << (i % BITSET_WORD_BITS));
}

#[inline(always)]
pub unsafe fn bitset_contain (set: *const bitset_word_t, i: Idx) -> bool
{
  ((*set.add((i / BITSET_WORD_BITS) as usize) >> (i % BITSET_WORD_BITS)) & 1) != 0
}

#[inline(always)]
pub unsafe fn bitset_empty (set: *mut bitset_word_t)
{
  memset (set as *mut c_void, 0, std::mem::size_of::<bitset_t> ());
}

#[inline(always)]
pub unsafe fn bitset_set_all (set: *mut bitset_word_t)
{
  memset (set as *mut c_void, -1,
          std::mem::size_of::<bitset_word_t> () * (SBC_MAX / BITSET_WORD_BITS) as usize);
  if SBC_MAX % BITSET_WORD_BITS != 0
    { *set.add(BITSET_WORDS - 1) =
        ((1 as bitset_word_t) << (SBC_MAX % BITSET_WORD_BITS)) - 1; }
}

#[inline(always)]
pub unsafe fn bitset_copy (dest: *mut bitset_word_t, src: *const bitset_word_t)
{
  memcpy (dest as *mut c_void, src as *const c_void, std::mem::size_of::<bitset_t> ());
}

#[inline(always)]
pub unsafe fn bitset_not (set: *mut bitset_word_t)
{
  let mut bitset_i: Idx;
  bitset_i = 0;
  while bitset_i < SBC_MAX / BITSET_WORD_BITS
    {
      *set.add(bitset_i as usize) = !*set.add(bitset_i as usize);
      bitset_i += 1;
    }
  if SBC_MAX % BITSET_WORD_BITS != 0
    { *set.add(BITSET_WORDS - 1) =
        ((((1 as bitset_word_t) << (SBC_MAX % BITSET_WORD_BITS)) - 1)
         & !*set.add(BITSET_WORDS - 1)); }
}

#[inline(always)]
pub unsafe fn bitset_merge (dest: *mut bitset_word_t, src: *const bitset_word_t)
{
  let mut bitset_i: Idx;
  bitset_i = 0;
  while bitset_i < BITSET_WORDS as Idx
    {
      *dest.add(bitset_i as usize) |= *src.add(bitset_i as usize);
      bitset_i += 1;
    }
}

#[inline(always)]
pub unsafe fn bitset_mask (dest: *mut bitset_word_t, src: *const bitset_word_t)
{
  let mut bitset_i: Idx;
  bitset_i = 0;
  while bitset_i < BITSET_WORDS as Idx
    {
      *dest.add(bitset_i as usize) &= *src.add(bitset_i as usize);
      bitset_i += 1;
    }
}

/* regex_internal.h 782-818: the three re_string helpers.  The `__attribute__ ((pure,
   unused))` prefixes have no Rust counterpart (PORTING.md 8).  */
#[inline(always)]
pub unsafe fn re_string_char_size_at (pstr: *const re_string_t, idx: Idx) -> c_int
{
  let mut byte_idx: Idx;
  if (*pstr).mb_cur_max == 1 { return 1; }
  byte_idx = 1;
  while idx + byte_idx < (*pstr).valid_len
    {
      if *(*pstr).wcs.add((idx + byte_idx) as usize) != WEOF { break; }
      byte_idx += 1;
    }
  return byte_idx as c_int;
}

#[inline(always)]
pub unsafe fn re_string_wchar_at (pstr: *const re_string_t, idx: Idx) -> wint_t
{
  if (*pstr).mb_cur_max == 1 { return *(*pstr).mbs.add (idx as usize) as wint_t; }
  return *(*pstr).wcs.add(idx as usize);
}

#[inline(always)]
pub unsafe fn re_string_elem_size_at (pstr: *const re_string_t, idx: Idx) -> c_int
{
  // C: `#ifdef _LIBC` (regex_internal.h 808-827) -- dead on the frozen target, kept as
  // text so the body comparison sees C's tokens (P-51).  The glibc names it uses have no
  // mirror of their own; the block is never compiled.
  #[cfg(feature = "regex_libc_weight")]
  {
    let mut p: *const u8;
    let mut extra: *const u8;
    let mut table: *const int32_t;
    let mut indirect: *const int32_t;
    let nrules: uint_fast32_t = _NL_CURRENT_WORD (LC_COLLATE, _NL_COLLATE_NRULES);

    if nrules != 0
      {
        table = _NL_CURRENT (LC_COLLATE, _NL_COLLATE_TABLEMB) as *const int32_t;
        extra = _NL_CURRENT (LC_COLLATE, _NL_COLLATE_EXTRAMB) as *const u8;
        indirect = _NL_CURRENT (LC_COLLATE, _NL_COLLATE_INDIRECTMB) as *const int32_t;
        p = (*pstr).mbs.add (idx as usize) as *const u8;
        findidx (table, indirect, extra, &mut p, (*pstr).len - idx);
        return (p.offset_from ((*pstr).mbs as *const u8)) as c_int - idx as c_int;
      }
  }

  return 1;
}

/* The packed bitfields (PORTING.md 1.3).  C reads and writes them as struct fields;
   the mirror keeps one integer per storage unit, so each accessor below is the port's
   spelling of one C field.  The getters are named with a trailing `_` so the token
   check still sees C's field name (`type_` folds to `type`, P-78); the setters'
   `set_…` spellings are registered as families in docs/ports/token-aliases.tsv, which
   also keeps an occurrence floor so a dropped write cannot pass.  Bit positions are
   LSB-first within the unit, the little-endian layout GCC/Clang use on the frozen
   target; P-35 records that the storage-unit bit order is not covered by the generated
   layout assertions, so the probe scripts have to pin it.  */

/// C: `re_token_t.type : 8` (bits 0-7).
pub const TOKEN_TYPE_SHIFT: u32 = 0;
pub const TOKEN_TYPE_MASK: c_uint = 0xff;
/// C: `re_token_t.constraint : 10` (bits 8-17).
pub const TOKEN_CONSTRAINT_SHIFT: u32 = 8;
pub const TOKEN_CONSTRAINT_MASK: c_uint = 0x3ff;
/// C: `re_token_t.accept_mb : 1` (bit 20).
pub const TOKEN_ACCEPT_MB_SHIFT: u32 = 20;
pub const TOKEN_ACCEPT_MB_MASK: c_uint = 1;

/// C: `re_dfastate_t.context : 4` (bits 0-3).
pub const STATE_CONTEXT_SHIFT: u32 = 0;
pub const STATE_CONTEXT_MASK: c_uint = 0xf;
/// C: `re_dfastate_t.halt : 1` (bit 4).
pub const STATE_HALT_SHIFT: u32 = 4;
pub const STATE_HALT_MASK: c_uint = 1;
/// C: `re_dfastate_t.accept_mb : 1` (bit 5).
pub const STATE_ACCEPT_MB_SHIFT: u32 = 5;
pub const STATE_ACCEPT_MB_MASK: c_uint = 1;
/// C: `re_dfastate_t.has_backref : 1` (bit 6).
pub const STATE_HAS_BACKREF_SHIFT: u32 = 6;
pub const STATE_HAS_BACKREF_MASK: c_uint = 1;
/// C: `re_dfastate_t.has_constraint : 1` (bit 7).
pub const STATE_HAS_CONSTRAINT_SHIFT: u32 = 7;
pub const STATE_HAS_CONSTRAINT_MASK: c_uint = 1;

/// C: `re_dfa_t.is_utf8 : 1` (bit 2).
pub const DFA_IS_UTF8_SHIFT: u32 = 2;
pub const DFA_IS_UTF8_MASK: c_uint = 1;
/// C: `re_dfa_t.map_notascii : 1` (bit 3).
pub const DFA_MAP_NOTASCII_SHIFT: u32 = 3;
pub const DFA_MAP_NOTASCII_MASK: c_uint = 1;
/// C: `re_dfa_t.word_ops_used : 1` (bit 4).
pub const DFA_WORD_OPS_USED_SHIFT: u32 = 4;
pub const DFA_WORD_OPS_USED_MASK: c_uint = 1;

#[inline(always)]
fn bits_get (bits: c_uint, shift: u32, mask: c_uint) -> c_uint { (bits >> shift) & mask }
#[inline(always)]
fn bits_set (bits: c_uint, shift: u32, mask: c_uint, value: c_uint) -> c_uint {
    (bits & !(mask << shift)) | ((value & mask) << shift)
}

#[inline(always)]
pub unsafe fn type_ (token: *const re_token_t) -> c_uint { bits_get ((*token).bits, TOKEN_TYPE_SHIFT, TOKEN_TYPE_MASK) }
#[inline(always)]
pub unsafe fn set_type_ (token: *mut re_token_t, value: c_uint) { (*token).bits = bits_set ((*token).bits, TOKEN_TYPE_SHIFT, TOKEN_TYPE_MASK, value); }
#[inline(always)]
pub unsafe fn constraint_ (token: *const re_token_t) -> c_uint { bits_get ((*token).bits, TOKEN_CONSTRAINT_SHIFT, TOKEN_CONSTRAINT_MASK) }
#[inline(always)]
pub unsafe fn set_constraint_ (token: *mut re_token_t, value: c_uint) { (*token).bits = bits_set ((*token).bits, TOKEN_CONSTRAINT_SHIFT, TOKEN_CONSTRAINT_MASK, value); }
#[inline(always)]
pub unsafe fn accept_mb_ (token: *const re_token_t) -> c_uint { bits_get ((*token).bits, TOKEN_ACCEPT_MB_SHIFT, TOKEN_ACCEPT_MB_MASK) }
#[inline(always)]
pub unsafe fn set_accept_mb_ (token: *mut re_token_t, value: c_uint) { (*token).bits = bits_set ((*token).bits, TOKEN_ACCEPT_MB_SHIFT, TOKEN_ACCEPT_MB_MASK, value); }

#[inline(always)]
pub unsafe fn context_ (state: *const re_dfastate_t) -> c_uint { bits_get ((*state).bits, STATE_CONTEXT_SHIFT, STATE_CONTEXT_MASK) }
#[inline(always)]
pub unsafe fn set_context_ (state: *mut re_dfastate_t, value: c_uint) { (*state).bits = bits_set ((*state).bits, STATE_CONTEXT_SHIFT, STATE_CONTEXT_MASK, value); }
#[inline(always)]
pub unsafe fn halt_ (state: *const re_dfastate_t) -> c_uint { bits_get ((*state).bits, STATE_HALT_SHIFT, STATE_HALT_MASK) }
#[inline(always)]
pub unsafe fn set_halt_ (state: *mut re_dfastate_t, value: c_uint) { (*state).bits = bits_set ((*state).bits, STATE_HALT_SHIFT, STATE_HALT_MASK, value); }
#[inline(always)]
pub unsafe fn state_accept_mb_ (state: *const re_dfastate_t) -> c_uint { bits_get ((*state).bits, STATE_ACCEPT_MB_SHIFT, STATE_ACCEPT_MB_MASK) }
#[inline(always)]
pub unsafe fn set_state_accept_mb_ (state: *mut re_dfastate_t, value: c_uint) { (*state).bits = bits_set ((*state).bits, STATE_ACCEPT_MB_SHIFT, STATE_ACCEPT_MB_MASK, value); }
#[inline(always)]
pub unsafe fn has_backref_ (state: *const re_dfastate_t) -> c_uint { bits_get ((*state).bits, STATE_HAS_BACKREF_SHIFT, STATE_HAS_BACKREF_MASK) }
#[inline(always)]
pub unsafe fn set_has_backref_ (state: *mut re_dfastate_t, value: c_uint) { (*state).bits = bits_set ((*state).bits, STATE_HAS_BACKREF_SHIFT, STATE_HAS_BACKREF_MASK, value); }
#[inline(always)]
pub unsafe fn has_constraint_ (state: *const re_dfastate_t) -> c_uint { bits_get ((*state).bits, STATE_HAS_CONSTRAINT_SHIFT, STATE_HAS_CONSTRAINT_MASK) }
#[inline(always)]
pub unsafe fn set_has_constraint_ (state: *mut re_dfastate_t, value: c_uint) { (*state).bits = bits_set ((*state).bits, STATE_HAS_CONSTRAINT_SHIFT, STATE_HAS_CONSTRAINT_MASK, value); }

#[inline(always)]
pub unsafe fn is_utf8_ (dfa: *const re_dfa_t) -> c_uint { bits_get ((*dfa).bits, DFA_IS_UTF8_SHIFT, DFA_IS_UTF8_MASK) }
#[inline(always)]
pub unsafe fn map_notascii_ (dfa: *const re_dfa_t) -> c_uint { bits_get ((*dfa).bits, DFA_MAP_NOTASCII_SHIFT, DFA_MAP_NOTASCII_MASK) }
#[inline(always)]
pub unsafe fn word_ops_used_ (dfa: *const re_dfa_t) -> c_uint { bits_get ((*dfa).bits, DFA_WORD_OPS_USED_SHIFT, DFA_WORD_OPS_USED_MASK) }
