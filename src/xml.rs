// System XML/HTML parser connection. Reference xml.c is identical at stable
// a360712c9d272d950d8d8255ef74570f7e90b7d9 and pinned fd7afced403.
// Parser options, DOM construction and error/nil behavior follow that source.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;


const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/buffer.rs imported via common

// [U1] include/libxml2.rs imported via common
#[cfg(feature = "have_libxml2")]
use libxml2_ffi::*;

// No Windows DLL loader has been connected yet. A new target must implement
// its actual loader before claiming this capability; Darwin uses the system ABI.
#[cfg(all(feature = "have_libxml2", feature = "windowsnt"))]
compile_error!("libxml2 delayed loading is not connected for this target");

// The non-Windows reference branch is directly linked and returns true;
// the actual parser entry points below are resolved from the system library.
#[cfg(feature = "have_libxml2")]
unsafe fn init_libxml2_functions() -> bool { true }

#[cfg(feature = "have_libxml2")]
unsafe fn make_dom(node: *mut xmlNode) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    if (*node).r#type == XML_ELEMENT_NODE {
        let mut result = list1(intern((*node).name as *const c_char));
        let mut plist = Qnil();
        let mut property = (*node).properties;
        while !property.is_null() {
            let child = (*property).children;
            if !child.is_null() && !(*child).content.is_null() {
                plist = Fcons(Fcons(intern((*property).name as *const c_char),
                                    build_string((*child).content as *const c_char)), plist);
            }
            property = (*property).next;
        }
        result = Fcons(Fnreverse(plist), result);
        let mut child = (*node).children;
        while !child.is_null() {
            result = Fcons(make_dom(child), result);
            child = (*child).next;
        }
        Fnreverse(result)
    } else if (*node).r#type == XML_TEXT_NODE || (*node).r#type == XML_CDATA_SECTION_NODE {
        if !(*node).content.is_null() {
            build_string((*node).content as *const c_char)
        } else { Qnil() }
    } else if (*node).r#type == XML_COMMENT_NODE {
        if !(*node).content.is_null() {
            list3(Qcomment(), Qnil(), build_string((*node).content as *const c_char))
        } else { Qnil() }
    } else { Qnil() }

  }
}

#[cfg(all(feature = "have_libxml2", feature = "rel_alloc"))]
unsafe extern "C" {
    fn r_alloc_inhibit_buffer_relocation(inhibit: c_int);
}

#[cfg(feature = "have_libxml2")]
unsafe fn parse_region(start: Lisp_Object, end: Lisp_Object, base_url: Lisp_Object,
                       discard_comments: Lisp_Object, htmlp: bool) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    let mut start = start;
    let mut end = end;
    let mut result = Qnil();
    let mut burl = c"".as_ptr();
    xmlCheckVersion(LIBXML_VERSION);
    if NILP(start) { start = Fpoint_min(); }
    if NILP(end) { end = Fpoint_max(); }
    validate_region(&raw mut start, &raw mut end);
    let istart = XFIXNUM(start) as ptrdiff_t;
    let iend = XFIXNUM(end) as ptrdiff_t;
    let istart_byte = CHAR_TO_BYTE(istart);
    let iend_byte = CHAR_TO_BYTE(iend);
    if istart < GPT!() && GPT!() < iend { move_gap_both(iend, iend_byte); }
    if !NILP(base_url) {
        CHECK_STRING(base_url);
        burl = SSDATA(base_url);
    }
    let buftext = BYTE_POS_ADDR(istart_byte);
    #[cfg(feature = "rel_alloc")]
    r_alloc_inhibit_buffer_relocation(1);
    let doc = if htmlp {
        htmlReadMemory(buftext as *const c_char, (iend_byte - istart_byte) as c_int,
                       burl, c"utf-8".as_ptr(), HTML_PARSE_RECOVER | HTML_PARSE_NONET |
                       HTML_PARSE_NOWARNING | HTML_PARSE_NOERROR | HTML_PARSE_NOBLANKS)
    } else {
        xmlReadMemory(buftext as *const c_char, (iend_byte - istart_byte) as c_int,
                      burl, c"utf-8".as_ptr(), XML_PARSE_NONET | XML_PARSE_NOWARNING |
                      XML_PARSE_NOBLANKS | XML_PARSE_NOERROR)
    };
    #[cfg(feature = "rel_alloc")]
    r_alloc_inhibit_buffer_relocation(0);
    eassert!(buftext == BYTE_POS_ADDR(istart_byte));
    if !doc.is_null() {
        let mut r = Qnil();
        if NILP(discard_comments) {
            let mut n = (*doc).children;
            while !n.is_null() {
                if !NILP(r) { result = Fcons(r, result); }
                r = make_dom(n);
                n = (*n).next;
            }
        }
        if NILP(result) {
            let node = xmlDocGetRootElement(doc);
            if !node.is_null() { result = make_dom(node); }
        } else {
            result = Fcons(Qtop(), Fcons(Qnil(), Fnreverse(Fcons(r, result))));
        }
        xmlFreeDoc(doc);
    }
    result

  }
}

#[cfg(feature = "have_libxml2")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn xml_cleanup_parser() {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe { xmlCleanupParser(); 
  }
}

#[cfg(feature = "have_libxml2")]
DEFUN! ("libxml-parse-html-region", Flibxml_parse_html_region, Slibxml_parse_html_region,
       0, 4, 0,
       doc: "Parse the region as an HTML document and return the parse tree.\nIf START is nil, it defaults to `point-min'.  If END is nil, it\ndefaults to `point-max'.\n\nIf BASE-URL is non-nil, it is used if and when reporting errors and\nwarnings from the underlying libxml2 library.  Currently, errors and\nwarnings from the library are suppressed, so this argument is largely\nignored.\n\nIf you want comments to be stripped, use the `xml-remove-comments'\nfunction to strip comments before calling this function.",
       (start: Lisp_Object, end: Lisp_Object, base_url: Lisp_Object, discard_comments: Lisp_Object), {
    if init_libxml2_functions() { parse_region(start, end, base_url, discard_comments, true) }
    else { Qnil() }
});

#[cfg(feature = "have_libxml2")]
DEFUN! ("libxml-parse-xml-region", Flibxml_parse_xml_region, Slibxml_parse_xml_region,
       0, 4, 0,
       doc: "Parse the region as an XML document and return the parse tree.\nIf START is nil, it defaults to `point-min'.  If END is nil, it\ndefaults to `point-max'.\n\nIf BASE-URL is non-nil, it is used if and when reporting errors and\nwarnings from the underlying libxml2 library.  Currently, errors and\nwarnings from the library are suppressed, so this argument is largely\nignored.\n\nIf you want comments to be stripped, use the `xml-remove-comments'\nfunction to strip comments before calling this function.",
       (start: Lisp_Object, end: Lisp_Object, base_url: Lisp_Object, discard_comments: Lisp_Object), {
    if init_libxml2_functions() { parse_region(start, end, base_url, discard_comments, false) }
    else { Qnil() }
});

DEFUN! ("libxml-available-p", Flibxml_available_p, Slibxml_available_p, 0, 0, 0,
       doc: "Return t if libxml2 support is available in this instance of Emacs.", (), {
    #[cfg(feature = "have_libxml2")]
    { return Qt(); }
    #[cfg(not(feature = "have_libxml2"))]
    { Qnil() }
});

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_xml() {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    #[cfg(feature = "have_libxml2")]
    {
        defsubr(Slibxml_parse_html_region.as_mut_ptr ());
        defsubr(Slibxml_parse_xml_region.as_mut_ptr ());
    }
    defsubr(Slibxml_available_p.as_mut_ptr ());
    DEFSYM! (Qcomment, "comment");

  }
}
