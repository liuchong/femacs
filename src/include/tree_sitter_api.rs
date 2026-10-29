// Used ABI declarations from Tree-sitter 0.27.1 api.h (MIT).
// SDK header SHA256: f8480415ae20cb5e0fd519ba44c84268c5acb4c918106238fc1d328d91431783.
// Opaque C objects are never allocated or dereferenced by this mirror.
pub const TREE_SITTER_LANGUAGE_VERSION: core::ffi::c_int = 15;
pub const TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION: core::ffi::c_int = 13;
pub type TSStateId = u16;
pub type TSSymbol = u16;
pub type TSFieldId = u16;
#[repr(C)]
pub struct TSLanguage { _opaque: [u8; 0] }
#[repr(C)]
pub struct TSParser { _opaque: [u8; 0] }
#[repr(C)]
pub struct TSTree { _opaque: [u8; 0] }
#[repr(C)]
pub struct TSQuery { _opaque: [u8; 0] }
#[repr(C)]
pub struct TSQueryCursor { _opaque: [u8; 0] }
pub type TSInputEncoding = core::ffi::c_uint;
pub const TSInputEncodingUTF8: TSInputEncoding = 0;
pub const TSInputEncodingUTF16LE: TSInputEncoding = 1;
pub const TSInputEncodingUTF16BE: TSInputEncoding = 2;
pub const TSInputEncodingCustom: TSInputEncoding = 3;
pub type TSLogType = core::ffi::c_uint;
pub const TSLogTypeParse: TSLogType = 0;
pub const TSLogTypeLex: TSLogType = 1;
pub type TSQueryError = core::ffi::c_uint;
pub const TSQueryErrorNone: TSQueryError = 0;
pub const TSQueryErrorSyntax: TSQueryError = 1;
pub const TSQueryErrorNodeType: TSQueryError = 2;
pub const TSQueryErrorField: TSQueryError = 3;
pub const TSQueryErrorCapture: TSQueryError = 4;
pub const TSQueryErrorStructure: TSQueryError = 5;
pub const TSQueryErrorLanguage: TSQueryError = 6;
pub type TSQueryPredicateStepType = core::ffi::c_uint;
pub const TSQueryPredicateStepTypeDone: TSQueryPredicateStepType = 0;
pub const TSQueryPredicateStepTypeCapture: TSQueryPredicateStepType = 1;
pub const TSQueryPredicateStepTypeString: TSQueryPredicateStepType = 2;

pub type TSDecodeFunction = Option<unsafe extern "C" fn (*const u8, u32, *mut i32) -> u32>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TSPoint { pub row: u32, pub column: u32 }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TSRange { pub start_point: TSPoint, pub end_point: TSPoint, pub start_byte: u32, pub end_byte: u32 }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TSInput {
 pub payload: *mut core::ffi::c_void,
 pub read: Option<unsafe extern "C" fn (*mut core::ffi::c_void, u32, TSPoint, *mut u32) -> *const core::ffi::c_char>,
 pub encoding: TSInputEncoding,
 pub decode: TSDecodeFunction,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TSLogger { pub payload: *mut core::ffi::c_void, pub log: Option<unsafe extern "C" fn (*mut core::ffi::c_void, TSLogType, *const core::ffi::c_char)> }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TSInputEdit { pub start_byte: u32, pub old_end_byte: u32, pub new_end_byte: u32, pub start_point: TSPoint, pub old_end_point: TSPoint, pub new_end_point: TSPoint }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TSNode { pub context: [u32; 4], pub id: *const core::ffi::c_void, pub tree: *const TSTree }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TSTreeCursor { pub tree: *const core::ffi::c_void, pub id: *const core::ffi::c_void, pub context: [u32; 3] }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TSQueryCapture { pub node: TSNode, pub index: u32 }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TSQueryMatch { pub id: u32, pub pattern_index: u16, pub capture_count: u16, pub captures: *const TSQueryCapture }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TSQueryPredicateStep { pub r#type: TSQueryPredicateStepType, pub value_id: u32 }
unsafe extern "C" {
  pub fn ts_set_allocator (
    new_malloc: Option<unsafe extern "C" fn (usize) -> *mut core::ffi::c_void>,
    new_calloc: Option<unsafe extern "C" fn (usize, usize) -> *mut core::ffi::c_void>,
    new_realloc: Option<unsafe extern "C" fn (*mut core::ffi::c_void, usize) -> *mut core::ffi::c_void>,
    new_free: Option<unsafe extern "C" fn (*mut core::ffi::c_void)>);
  pub fn ts_parser_new () -> *mut TSParser;
  pub fn ts_parser_delete (self_: *mut TSParser);
  pub fn ts_parser_language (self_: *const TSParser) -> *const TSLanguage;
  pub fn ts_parser_set_language (self_: *mut TSParser, language: *const TSLanguage) -> bool;
  pub fn ts_parser_set_included_ranges (self_: *mut TSParser, ranges: *const TSRange, count: u32) -> bool;
  pub fn ts_parser_included_ranges (self_: *const TSParser, count: *mut u32) -> *const TSRange;
  pub fn ts_parser_parse (self_: *mut TSParser, old_tree: *const TSTree, input: TSInput) -> *mut TSTree;
  pub fn ts_tree_delete (self_: *mut TSTree);
  pub fn ts_tree_root_node (self_: *const TSTree) -> TSNode;
  pub fn ts_tree_edit (self_: *mut TSTree, edit: *const TSInputEdit);
  pub fn ts_tree_get_changed_ranges (old_tree: *const TSTree, new_tree: *const TSTree, length: *mut u32) -> *mut TSRange;
  pub fn ts_node_type (self_: TSNode) -> *const core::ffi::c_char;
  pub fn ts_node_start_byte (self_: TSNode) -> u32;
  pub fn ts_node_end_byte (self_: TSNode) -> u32;
  pub fn ts_node_string (self_: TSNode) -> *mut core::ffi::c_char;
  pub fn ts_node_is_null (self_: TSNode) -> bool;
  pub fn ts_node_is_named (self_: TSNode) -> bool;
  pub fn ts_node_is_missing (self_: TSNode) -> bool;
  pub fn ts_node_is_extra (self_: TSNode) -> bool;
  pub fn ts_node_has_error (self_: TSNode) -> bool;
  pub fn ts_node_child (self_: TSNode, child_index: u32) -> TSNode;
  pub fn ts_node_field_name_for_child (self_: TSNode, child_index: u32) -> *const core::ffi::c_char;
  pub fn ts_node_child_count (self_: TSNode) -> u32;
  pub fn ts_node_named_child (self_: TSNode, child_index: u32) -> TSNode;
  pub fn ts_node_named_child_count (self_: TSNode) -> u32;
  pub fn ts_node_child_by_field_name (self_: TSNode, name: *const core::ffi::c_char, name_length: u32) -> TSNode;
  pub fn ts_node_next_sibling (self_: TSNode) -> TSNode;
  pub fn ts_node_prev_sibling (self_: TSNode) -> TSNode;
  pub fn ts_node_next_named_sibling (self_: TSNode) -> TSNode;
  pub fn ts_node_prev_named_sibling (self_: TSNode) -> TSNode;
  pub fn ts_node_descendant_for_byte_range (self_: TSNode, start: u32, end: u32) -> TSNode;
  pub fn ts_node_named_descendant_for_byte_range (self_: TSNode, start: u32, end: u32) -> TSNode;
  pub fn ts_node_eq (self_: TSNode, other: TSNode) -> bool;
  pub fn ts_tree_cursor_new (node: TSNode) -> TSTreeCursor;
  pub fn ts_tree_cursor_delete (self_: *mut TSTreeCursor);
  pub fn ts_tree_cursor_current_node (self_: *const TSTreeCursor) -> TSNode;
  pub fn ts_tree_cursor_goto_parent (self_: *mut TSTreeCursor) -> bool;
  pub fn ts_tree_cursor_goto_next_sibling (self_: *mut TSTreeCursor) -> bool;
  pub fn ts_tree_cursor_goto_previous_sibling (self_: *mut TSTreeCursor) -> bool;
  pub fn ts_tree_cursor_goto_first_child (self_: *mut TSTreeCursor) -> bool;
  pub fn ts_tree_cursor_goto_first_child_for_byte (self_: *mut TSTreeCursor, goal_byte: u32) -> i64;
  pub fn ts_tree_cursor_copy (cursor: *const TSTreeCursor) -> TSTreeCursor;
  pub fn ts_query_new (language: *const TSLanguage, source: *const core::ffi::c_char, source_len: u32, error_offset: *mut u32, error_type: *mut TSQueryError) -> *mut TSQuery;
  pub fn ts_query_delete (self_: *mut TSQuery);
  pub fn ts_query_pattern_count (self_: *const TSQuery) -> u32;
  pub fn ts_query_predicates_for_pattern (self_: *const TSQuery, pattern_index: u32, step_count: *mut u32) -> *const TSQueryPredicateStep;
  pub fn ts_query_capture_name_for_id (self_: *const TSQuery, index: u32, length: *mut u32) -> *const core::ffi::c_char;
  pub fn ts_query_string_value_for_id (self_: *const TSQuery, index: u32, length: *mut u32) -> *const core::ffi::c_char;
  pub fn ts_query_cursor_new () -> *mut TSQueryCursor;
  pub fn ts_query_cursor_delete (self_: *mut TSQueryCursor);
  pub fn ts_query_cursor_exec (self_: *mut TSQueryCursor, query: *const TSQuery, node: TSNode);
  pub fn ts_query_cursor_set_byte_range (self_: *mut TSQueryCursor, start_byte: u32, end_byte: u32) -> bool;
  pub fn ts_query_cursor_next_match (self_: *mut TSQueryCursor, r#match: *mut TSQueryMatch) -> bool;
  pub fn ts_language_abi_version (self_: *const TSLanguage) -> u32;
}
