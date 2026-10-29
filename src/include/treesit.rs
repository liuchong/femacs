// Header mirror: src/treesit.h at fd7afced4036fc1a221567d70705d20a2bd64713.
// P430 restores the gated shared layouts; platform acceptance remains pending.
pub const EMACS_TREESIT_H: bool = true;

#[cfg(feature = "have_tree_sitter")]
include!("tree_sitter_api.rs");

// The six initial Lisp fields are scanned by the pseudovector collector.
// Native handles/input payloads are owned by this exact parser; nodes retain it.
#[cfg(feature = "have_tree_sitter")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_TS_Parser {
    pub header: vectorlike_header,
    pub language_symbol: Lisp_Object,
    pub after_change_functions: Lisp_Object,
    pub tag: Lisp_Object,
    pub last_set_ranges: Lisp_Object,
    pub embed_level: Lisp_Object,
    pub buffer: Lisp_Object,
    pub parser: *mut TSParser,
    pub tree: *mut TSTree,
    pub input: TSInput,
    pub need_reparse: bool,
    pub visible_beg: ptrdiff_t,
    pub visible_end: ptrdiff_t,
    pub visi_beg_linecol: ts_linecol,
    pub visi_end_linecol: ts_linecol,
    pub timestamp: ptrdiff_t,
    pub deleted: bool,
    pub need_to_gc_buffer: bool,
    pub within_reparse: bool,
}

#[cfg(feature = "have_tree_sitter")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_TS_Node {
    pub header: vectorlike_header,
    pub parser: Lisp_Object,
    pub node: TSNode,
    pub timestamp: ptrdiff_t,
}

// Source/language are Lisp roots. Native query/cursor are compiled lazily and
// may be null; the portable dumper stores only source/language, not handles.
#[cfg(feature = "have_tree_sitter")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_TS_Query {
    pub header: vectorlike_header,
    pub language: Lisp_Object,
    pub source: Lisp_Object,
    pub query: *mut TSQuery,
    pub cursor: *mut TSQueryCursor,
}

#[cfg(feature = "have_tree_sitter")]
#[inline(always)]
pub fn TS_PARSERP(x: Lisp_Object) -> bool {
    PSEUDOVECTORP(x, pvec_type::PVEC_TS_PARSER)
}
#[cfg(feature = "have_tree_sitter")]
#[inline(always)]
pub fn XTS_PARSER(a: Lisp_Object) -> *mut Lisp_TS_Parser {
    eassert!(TS_PARSERP(a));
    XUNTAG!(a, Lisp_Type::Lisp_Vectorlike, Lisp_TS_Parser)
}
#[cfg(feature = "have_tree_sitter")]
#[inline(always)]
pub fn TS_NODEP(x: Lisp_Object) -> bool {
    PSEUDOVECTORP(x, pvec_type::PVEC_TS_NODE)
}
#[cfg(feature = "have_tree_sitter")]
#[inline(always)]
pub fn XTS_NODE(a: Lisp_Object) -> *mut Lisp_TS_Node {
    eassert!(TS_NODEP(a));
    XUNTAG!(a, Lisp_Type::Lisp_Vectorlike, Lisp_TS_Node)
}
#[cfg(feature = "have_tree_sitter")]
#[inline(always)]
pub fn TS_COMPILED_QUERY_P(x: Lisp_Object) -> bool {
    PSEUDOVECTORP(x, pvec_type::PVEC_TS_COMPILED_QUERY)
}
#[cfg(feature = "have_tree_sitter")]
#[inline(always)]
pub fn XTS_COMPILED_QUERY(a: Lisp_Object) -> *mut Lisp_TS_Query {
    eassert!(TS_COMPILED_QUERY_P(a));
    XUNTAG!(a, Lisp_Type::Lisp_Vectorlike, Lisp_TS_Query)
}
#[cfg(feature = "have_tree_sitter")]
#[inline(always)]
pub unsafe fn CHECK_TS_PARSER(parser: Lisp_Object) {
    CHECK_TYPE(TS_PARSERP(parser), Qtreesit_parser_p(), parser);
}
#[cfg(feature = "have_tree_sitter")]
#[inline(always)]
pub unsafe fn CHECK_TS_NODE(node: Lisp_Object) {
    CHECK_TYPE(TS_NODEP(node), Qtreesit_node_p(), node);
}
#[cfg(feature = "have_tree_sitter")]
#[inline(always)]
pub unsafe fn CHECK_TS_COMPILED_QUERY(query: Lisp_Object) {
    CHECK_TYPE(TS_COMPILED_QUERY_P(query), Qtreesit_compiled_query_p(), query);
}

// Glob-export declarations let the defining translation unit use the same
// layouts/accessors while its local definitions shadow these foreign names.
pub mod treesit_declarations {
    use super::*;
    unsafe extern "C" {
        pub fn syms_of_treesit();
        #[cfg(feature = "have_tree_sitter")]
        pub static TREESIT_EMPTY_LINECOL: ts_linecol;
        #[cfg(feature = "have_tree_sitter")]
        pub fn linecol_offset(linecol: ts_linecol, offset: ts_linecol) -> ts_linecol;
        #[cfg(feature = "have_tree_sitter")]
        pub fn treesit_linecol_maybe(pos: ptrdiff_t, bytepos: ptrdiff_t, cache: ts_linecol) -> ts_linecol;
        #[cfg(feature = "have_tree_sitter")]
        pub fn treesit_record_change(start: ptrdiff_t, old_end: ptrdiff_t, new_end: ptrdiff_t,
                                    start_linecol: ts_linecol, old_end_linecol: ts_linecol,
                                    new_len: ptrdiff_t);
        #[cfg(feature = "have_tree_sitter")]
        pub fn treesit_node_uptodate_p(node: Lisp_Object) -> bool;
        #[cfg(feature = "have_tree_sitter")]
        pub fn treesit_node_buffer_live_p(node: Lisp_Object) -> bool;
        #[cfg(feature = "have_tree_sitter")]
        pub fn treesit_delete_parser(parser: *mut Lisp_TS_Parser);
        #[cfg(feature = "have_tree_sitter")]
        pub fn treesit_delete_query(query: *mut Lisp_TS_Query);
        #[cfg(feature = "have_tree_sitter")]
        pub fn treesit_named_node_p(node: TSNode) -> bool;
        #[cfg(feature = "have_tree_sitter")]
        pub fn treesit_node_eq(left: Lisp_Object, right: Lisp_Object) -> bool;
    }
}
pub use treesit_declarations::*;
