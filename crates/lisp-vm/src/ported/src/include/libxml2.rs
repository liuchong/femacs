// Public libxml2 2.9 tree ABI used by the owned Darwin target.
// External library memory is owned and released by libxml2; these types are
// never constructed or freed by the Rust allocator. Opaque fields retain their
// pointer widths; all public node/attribute/document fields retain C layout.
// [U1-MODULE-MIRROR]
use super::*;
#[cfg(feature = "have_libxml2")]
pub mod libxml2_ffi {
    use core::ffi::{c_char, c_int, c_void};

    pub const LIBXML_VERSION: c_int = 20913;
    pub const XML_ELEMENT_NODE: c_int = 1;
    pub const XML_TEXT_NODE: c_int = 3;
    pub const XML_CDATA_SECTION_NODE: c_int = 4;
    pub const XML_COMMENT_NODE: c_int = 8;
    pub const XML_PARSE_NOERROR: c_int = 1 << 5;
    pub const XML_PARSE_NOWARNING: c_int = 1 << 6;
    pub const XML_PARSE_NOBLANKS: c_int = 1 << 8;
    pub const XML_PARSE_NONET: c_int = 1 << 11;
    pub const HTML_PARSE_RECOVER: c_int = 1;
    pub const HTML_PARSE_NOERROR: c_int = 1 << 5;
    pub const HTML_PARSE_NOWARNING: c_int = 1 << 6;
    pub const HTML_PARSE_NOBLANKS: c_int = 1 << 8;
    pub const HTML_PARSE_NONET: c_int = 1 << 11;

    #[repr(C)]
    pub struct xmlNode {
        pub _private: *mut c_void,
        pub r#type: c_int,
        pub name: *const u8,
        pub children: *mut xmlNode,
        pub last: *mut xmlNode,
        pub parent: *mut xmlNode,
        pub next: *mut xmlNode,
        pub prev: *mut xmlNode,
        pub doc: *mut xmlDoc,
        pub ns: *mut c_void,
        pub content: *mut u8,
        pub properties: *mut xmlAttr,
        pub nsDef: *mut c_void,
        pub psvi: *mut c_void,
        pub line: u16,
        pub extra: u16,
    }

    #[repr(C)]
    pub struct xmlAttr {
        pub _private: *mut c_void,
        pub r#type: c_int,
        pub name: *const u8,
        pub children: *mut xmlNode,
        pub last: *mut xmlNode,
        pub parent: *mut xmlNode,
        pub next: *mut xmlAttr,
        pub prev: *mut xmlAttr,
        pub doc: *mut xmlDoc,
        pub ns: *mut c_void,
        pub atype: c_int,
        pub psvi: *mut c_void,
    }

    #[repr(C)]
    pub struct xmlDoc {
        pub _private: *mut c_void,
        pub r#type: c_int,
        pub name: *mut c_char,
        pub children: *mut xmlNode,
        pub last: *mut xmlNode,
        pub parent: *mut xmlNode,
        pub next: *mut xmlNode,
        pub prev: *mut xmlNode,
        pub doc: *mut xmlDoc,
        pub compression: c_int,
        pub standalone: c_int,
        pub intSubset: *mut c_void,
        pub extSubset: *mut c_void,
        pub oldNs: *mut c_void,
        pub version: *const u8,
        pub encoding: *const u8,
        pub ids: *mut c_void,
        pub refs: *mut c_void,
        pub URL: *const u8,
        pub charset: c_int,
        pub dict: *mut c_void,
        pub psvi: *mut c_void,
        pub parseFlags: c_int,
        pub properties: c_int,
    }

    unsafe extern "C" {
        pub fn xmlCheckVersion(version: c_int);
        pub fn xmlReadMemory(buffer: *const c_char, size: c_int,
                             url: *const c_char, encoding: *const c_char,
                             options: c_int) -> *mut xmlDoc;
        pub fn htmlReadMemory(buffer: *const c_char, size: c_int,
                              url: *const c_char, encoding: *const c_char,
                              options: c_int) -> *mut xmlDoc;
        pub fn xmlDocGetRootElement(doc: *const xmlDoc) -> *mut xmlNode;
        pub fn xmlFreeDoc(doc: *mut xmlDoc);
        pub fn xmlCleanupParser();
    }
}
