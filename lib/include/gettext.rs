// femacs: port of lib/gettext.h @ fd7afced403.
#![forbid(unsafe_code)]

use super::*;

pub const _LIBGETTEXT_H: c_int = 1;
pub const GETTEXT_CONTEXT_GLUE: &core::ffi::CStr = c"\x04";
const _: () = core::assert!(GETTEXT_CONTEXT_GLUE.to_bytes()[0] == 0x04);

macro_rules! _LIBGETTEXT_FUNCAST {
    ($ty:ty, $expr:expr) => {
        $expr as $ty
    };
}

pub const _LIBGETTEXT_INLINE: () = ();

#[inline(always)]
pub fn gettext(msgid: *const c_char) -> *mut c_char {
    msgid as *mut c_char
}
#[inline(always)]
pub fn dgettext(_domain: *const c_char, msgid: *const c_char) -> *mut c_char {
    msgid as *mut c_char
}
#[inline(always)]
pub fn dcgettext(_domain: *const c_char, msgid: *const c_char, _category: c_int) -> *mut c_char {
    msgid as *mut c_char
}
#[inline(always)]
pub fn ngettext(
    msgid1: *const c_char,
    msgid2: *const c_char,
    n: core::ffi::c_ulong,
) -> *mut c_char {
    (if n == 1 { msgid1 } else { msgid2 }) as *mut c_char
}
#[inline(always)]
pub fn dngettext(
    _domain: *const c_char,
    msgid1: *const c_char,
    msgid2: *const c_char,
    n: core::ffi::c_ulong,
) -> *mut c_char {
    (if n == 1 { msgid1 } else { msgid2 }) as *mut c_char
}
#[inline(always)]
pub fn dcngettext(
    _domain: *const c_char,
    msgid1: *const c_char,
    msgid2: *const c_char,
    n: core::ffi::c_ulong,
    _category: c_int,
) -> *mut c_char {
    (if n == 1 { msgid1 } else { msgid2 }) as *mut c_char
}
#[inline(always)]
pub fn textdomain(domainname: *const c_char) -> *mut c_char {
    domainname as *mut c_char
}

#[cfg(feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_gettext(msgid: *const c_char) -> *const c_char {
    msgid
}
#[cfg(feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_dgettext(_domain: *const c_char, msgid: *const c_char) -> *const c_char {
    msgid
}
#[cfg(feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_dcgettext(
    _domain: *const c_char,
    msgid: *const c_char,
    _category: c_int,
) -> *const c_char {
    msgid
}
#[cfg(feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_ngettext(
    msgid1: *const c_char,
    msgid2: *const c_char,
    n: core::ffi::c_ulong,
) -> *const c_char {
    if n == 1 { msgid1 } else { msgid2 }
}
#[cfg(feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_dngettext(
    _domain: *const c_char,
    msgid1: *const c_char,
    msgid2: *const c_char,
    n: core::ffi::c_ulong,
) -> *const c_char {
    if n == 1 { msgid1 } else { msgid2 }
}
#[cfg(feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_dcngettext(
    _domain: *const c_char,
    msgid1: *const c_char,
    msgid2: *const c_char,
    n: core::ffi::c_ulong,
    _category: c_int,
) -> *const c_char {
    if n == 1 { msgid1 } else { msgid2 }
}
#[cfg(feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_textdomain(domainname: *const c_char) -> *const c_char {
    domainname
}

#[cfg(feature = "no_gnu_inline")]
macro_rules! gettext {
    ($msgid:expr) => {
        _LIBGETTEXT_FUNCAST!(*const c_char, $msgid)
    };
}

#[cfg(feature = "gnulib_defined_setlocale")]
macro_rules! setlocale {
    () => {
        rpl_setlocale
    };
}

macro_rules! gettext_noop {
    ($string:expr) => {
        $string
    };
}

#[cfg(feature = "no_lc_messages")]
pub const LC_MESSAGES: c_int = 1729;
pub const LC_MESSAGES: c_int = 6;

macro_rules! pgettext {
    ($msgctxt:literal, $msgid:literal) => {{
        const _: () = core::assert!(GETTEXT_CONTEXT_GLUE.to_bytes()[0] == 0x04);
        pgettext_aux(
            core::ptr::null(),
            concat!($msgctxt, "\x04", $msgid, "\0").as_ptr() as *const c_char,
            $msgid,
            LC_MESSAGES,
        )
    }};
}
macro_rules! dpgettext {
    ($domainname:expr, $msgctxt:literal, $msgid:literal) => {{
        const _: () = core::assert!(GETTEXT_CONTEXT_GLUE.to_bytes()[0] == 0x04);
        pgettext_aux(
            $domainname,
            concat!($msgctxt, "\x04", $msgid, "\0").as_ptr() as *const c_char,
            $msgid,
            LC_MESSAGES,
        )
    }};
}
macro_rules! dcpgettext {
    ($domainname:expr, $msgctxt:literal, $msgid:literal, $category:expr) => {{
        const _: () = core::assert!(GETTEXT_CONTEXT_GLUE.to_bytes()[0] == 0x04);
        pgettext_aux(
            $domainname,
            concat!($msgctxt, "\x04", $msgid, "\0").as_ptr() as *const c_char,
            $msgid,
            $category,
        )
    }};
}
macro_rules! npgettext {
    ($msgctxt:literal, $msgid:literal, $msgidplural:literal, $n:expr) => {{
        const _: () = core::assert!(GETTEXT_CONTEXT_GLUE.to_bytes()[0] == 0x04);
        npgettext_aux(
            core::ptr::null(),
            concat!($msgctxt, "\x04", $msgid, "\0").as_ptr() as *const c_char,
            $msgid,
            $msgidplural,
            $n,
            LC_MESSAGES,
        )
    }};
}
macro_rules! dnpgettext {
    ($domainname:expr, $msgctxt:literal, $msgid:literal, $msgidplural:literal, $n:expr) => {{
        const _: () = core::assert!(GETTEXT_CONTEXT_GLUE.to_bytes()[0] == 0x04);
        npgettext_aux(
            $domainname,
            concat!($msgctxt, "\x04", $msgid, "\0").as_ptr() as *const c_char,
            $msgid,
            $msgidplural,
            $n,
            LC_MESSAGES,
        )
    }};
}
macro_rules! dcnpgettext {
    ($domainname:expr, $msgctxt:literal, $msgid:literal, $msgidplural:literal, $n:expr, $category:expr) => {{
        const _: () = core::assert!(GETTEXT_CONTEXT_GLUE.to_bytes()[0] == 0x04);
        npgettext_aux(
            $domainname,
            concat!($msgctxt, "\x04", $msgid, "\0").as_ptr() as *const c_char,
            $msgid,
            $msgidplural,
            $n,
            $category,
        )
    }};
}

macro_rules! bindtextdomain {
    ($domainname:expr, $dirname:expr) => {{
        let _ = _LIBGETTEXT_FUNCAST!(*const c_char, $domainname);
        let _ = _LIBGETTEXT_FUNCAST!(*const c_char, $dirname);
    }};
}
macro_rules! bind_textdomain_codeset {
    ($domainname:expr, $codeset:expr) => {{
        let _ = _LIBGETTEXT_FUNCAST!(*const c_char, $domainname);
        let _ = _LIBGETTEXT_FUNCAST!(*const c_char, $codeset);
    }};
}

#[inline(always)]
pub fn pgettext_aux(
    domain: *const c_char,
    msg_ctxt_id: *const c_char,
    msgid: *const c_char,
    category: c_int,
) -> *const c_char {
    let translation: *const c_char = dcgettext(domain, msg_ctxt_id, category) as *const c_char;
    if core::ptr::eq(translation, msg_ctxt_id) {
        msgid
    } else {
        translation
    }
}

#[inline(always)]
pub fn npgettext_aux(
    domain: *const c_char,
    msg_ctxt_id: *const c_char,
    msgid: *const c_char,
    msgid_plural: *const c_char,
    n: core::ffi::c_ulong,
    category: c_int,
) -> *const c_char {
    let translation: *const c_char =
        dcngettext(domain, msg_ctxt_id, msgid_plural, n, category) as *const c_char;
    if core::ptr::eq(translation, msg_ctxt_id) || core::ptr::eq(translation, msgid_plural) {
        if n == 1 { msgid } else { msgid_plural }
    } else {
        translation
    }
}

#[cfg(feature = "no_variable_size_arrays")]
pub const _LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS: c_int = 0;
pub const _LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS: c_int = 1;

macro_rules! pgettext_expr {
    ($msgctxt:expr, $msgid:expr) => {
        dcpgettext_expr(core::ptr::null(), $msgctxt, $msgid, LC_MESSAGES)
    };
}
macro_rules! dpgettext_expr {
    ($domainname:expr, $msgctxt:expr, $msgid:expr) => {
        dcpgettext_expr($domainname, $msgctxt, $msgid, LC_MESSAGES)
    };
}
macro_rules! npgettext_expr {
    ($msgctxt:expr, $msgid:expr, $msgidplural:expr, $n:expr) => {
        dcnpgettext_expr(
            core::ptr::null(),
            $msgctxt,
            $msgid,
            $msgidplural,
            $n,
            LC_MESSAGES,
        )
    };
}
macro_rules! dnpgettext_expr {
    ($domainname:expr, $msgctxt:expr, $msgid:expr, $msgidplural:expr, $n:expr) => {
        dcnpgettext_expr($domainname, $msgctxt, $msgid, $msgidplural, $n, LC_MESSAGES)
    };
}

#[inline(always)]
pub fn dcpgettext_expr(
    _domain: *const c_char,
    _msgctxt: *const c_char,
    msgid: *const c_char,
    _category: c_int,
) -> *const c_char {
    // With `ENABLE_NLS` disabled, `dcgettext` is an identity stub (`found_translation == 0`).
    msgid
}

#[inline(always)]
pub fn dcnpgettext_expr(
    _domain: *const c_char,
    _msgctxt: *const c_char,
    msgid: *const c_char,
    msgid_plural: *const c_char,
    n: core::ffi::c_ulong,
    _category: c_int,
) -> *const c_char {
    if n == 1 { msgid } else { msgid_plural }
}
