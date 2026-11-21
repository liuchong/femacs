// femacs: port of lib/gettext.h @ fd7afced403.
//
// The header has three arms and the frozen configuration takes the third:
//
//   * C 23's `#if defined ENABLE_NLS && ENABLE_NLS` is FALSE: `ENABLE_NLS` is not in
//     build/emacs-ref/src/config.h (the frozen configure was `--without-all`), so C 26's
//     `<libintl.h>` and C 31-38's DEFAULT_TEXT_DOMAIN arm are dead and the port has no message
//     catalog - every `*gettext` below is the identity C spells.
//   * inside the no-NLS arm, C 109's `#if (defined __GNUC__ || defined __clang__) &&
//     !defined __cplusplus` is TRUE for this target's clang, so C 121-224's seven
//     `extern inline` definitions (gettext, dgettext, dcgettext, ngettext, dngettext,
//     dcngettext, textdomain) are the live text; C 229-291's C++ arm and C 293-322's
//     plain-macro arm are dead and are carried under port-local features (P-51).
//   * C 337's `#ifdef GNULIB_defined_setlocale` is dead (`GNULIB_defined_setlocale` is not in
//     the frozen macro table), so `setlocale` is the system's.
//
// The frozen values, measured with the frozen includes (`clang -dM -E -std=gnu23
// -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib`):
// `LC_MESSAGES` is 6 (C 361's `<locale.h>`), `_LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS` is 1
// (`__STDC_VERSION__` is >= 199901L, and neither `GNULIB_NO_VLA` nor `__STDC_NO_VLA__` is
// defined) and `_LIBGETTEXT_INLINE` is `__always_inline__, __gnu_inline__` (C 111's clang arm).
// `tools/extract-abi.py` reads this header blind (it is not in the verified list), so it takes
// the LAST definition of a name - the *dead* arm for `LC_MESSAGES` and
// `_LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS`.  Those are written first, gated, and their frozen
// values second, exactly as lib/include/fcntl.in.rs writes its `#ifndef` fallbacks: the
// extractor's constant reader takes the first definition, rustc compiles only the ungated one.
//
// The seven live definitions are header-inline in C (`__gnu_inline__` is what keeps C from
// emitting an external definition of them), so they are `#[inline(always)]` Rust functions with
// no symbol (PORTING.md 1.2/P-12), and the four helpers C spells `static` are ordinary
// functions in the mirror for the same reason (they carry no C symbol either).
// [U1-MODULE-MIRROR]
use super::*;
// [U1] string.rs imported via super
// [U1] stdlib.rs imported via super

/* C 17-18: `#ifndef _LIBGETTEXT_H / #define _LIBGETTEXT_H 1` - the include guard.  */
pub const _LIBGETTEXT_H: c_int = 1;

/* C 353-354: `#define GETTEXT_CONTEXT_GLUE "\004"` - the separator between msgctxt and msgid
   in a .mo file.  A string-valued macro keeps its value (PORTING.md 5.1), as a `&CStr`.  */
pub const GETTEXT_CONTEXT_GLUE: &core::ffi::CStr = c"\x04";
const _: () = core::assert! (GETTEXT_CONTEXT_GLUE.to_bytes ()[0] == 0x04);

/* C 60-67's `_LIBGETTEXT_FUNCAST (type, expr)`: `(type) {(expr)}` in the non-C++ arm (C 66)
   and `static_cast <type> (expr)` in the C++ one (C 64) - a cast that only does the conversions
   an ordinary assignment would.  Rust's `as` is that conversion.  */
macro_rules! _LIBGETTEXT_FUNCAST {
  ($ty:ty, $expr:expr) => { $expr as $ty };
}

/* C 109-114's `_LIBGETTEXT_INLINE` attribute: C 111's clang arm spells
   `__always_inline__, __gnu_inline__` and C 113's fallback `__always_inline__`.  Both are the
   C compiler's spelling of the inline definitions below, which Rust spells
   `#[inline(always)]`; the name is a marker for the arms' relationship (the `_GL_INLINE`
   shape, lib/include/gnulib_config.rs).  C defines it twice, so the extractor compares only
   the name.  */
pub const _LIBGETTEXT_INLINE: () = ();

/* C 121-224's live arm: the seven `extern inline` identity definitions, in C's order.  C's
   `__attribute__ ((_LIBGETTEXT_INLINE, __format_arg__ (n)))` is the compiler's business (the
   format-string check has no Rust counterpart) and `__gnu_inline__` is why C emits no external
   definition; the port's functions carry no symbol either.  */
#[inline(always)]
pub fn gettext (msgid: *const c_char) -> *mut c_char { return msgid as *mut c_char; }
#[inline(always)]
pub fn dgettext (domain: *const c_char, msgid: *const c_char) -> *mut c_char
{ let _ = domain; return msgid as *mut c_char; }
#[inline(always)]
pub fn dcgettext (domain: *const c_char, msgid: *const c_char, category: c_int) -> *mut c_char
{ let _ = domain; let _ = category; return msgid as *mut c_char; }
#[inline(always)]
pub fn ngettext (msgid1: *const c_char, msgid2: *const c_char, n: core::ffi::c_ulong)
  -> *mut c_char
{ return if n == 1 { msgid1 } else { msgid2 } as *mut c_char; }
#[inline(always)]
pub fn dngettext (domain: *const c_char, msgid1: *const c_char, msgid2: *const c_char,
                  n: core::ffi::c_ulong) -> *mut c_char
{ let _ = domain; return if n == 1 { msgid1 } else { msgid2 } as *mut c_char; }
#[inline(always)]
pub fn dcngettext (domain: *const c_char, msgid1: *const c_char, msgid2: *const c_char,
                   n: core::ffi::c_ulong, category: c_int) -> *mut c_char
{ let _ = domain; let _ = category; return if n == 1 { msgid1 } else { msgid2 } as *mut c_char; }
#[inline(always)]
pub fn textdomain (domainname: *const c_char) -> *mut c_char
{ return domainname as *mut c_char; }

/* C 229-291's C++ arm: dead (the port is not C++).  Its seven `static inline` identities are
   kept under the feature naming the non-frozen side (P-51); the extractor asks for their
   names.  Its `# undef gettext / # define gettext _libgettext_gettext` renames (C 237-291)
   are the same names the live functions above carry, so they need no item of their own.  */
#[cfg (feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_gettext (msgid: *const c_char) -> *const c_char { return msgid; }
#[cfg (feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_dgettext (domain: *const c_char, msgid: *const c_char) -> *const c_char
{ let _ = domain; return msgid; }
#[cfg (feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_dcgettext (domain: *const c_char, msgid: *const c_char, category: c_int)
  -> *const c_char
{ let _ = domain; let _ = category; return msgid; }
#[cfg (feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_ngettext (msgid1: *const c_char, msgid2: *const c_char,
                             n: core::ffi::c_ulong) -> *const c_char
{ return if n == 1 { msgid1 } else { msgid2 }; }
#[cfg (feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_dngettext (domain: *const c_char, msgid1: *const c_char,
                              msgid2: *const c_char, n: core::ffi::c_ulong) -> *const c_char
{ let _ = domain; return if n == 1 { msgid1 } else { msgid2 }; }
#[cfg (feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_dcngettext (domain: *const c_char, msgid1: *const c_char,
                               msgid2: *const c_char, n: core::ffi::c_ulong, category: c_int)
  -> *const c_char
{ let _ = domain; let _ = category; return if n == 1 { msgid1 } else { msgid2 }; }
#[cfg (feature = "cplusplus")]
#[inline(always)]
pub fn _libgettext_textdomain (domainname: *const c_char) -> *const c_char { return domainname; }

/* C 293-322's plain-macro arm: dead (C 229's arm is live for this target).  It renames each of
   the seven names to its `_LIBGETTEXT_FUNCAST` form, which is what the extractor's last
   definition of each of those macros reads; the names are already carried by the functions
   above, so only the one arm whose text has no item is spelled here - `gettext`, whose C 296
   body is `_LIBGETTEXT_FUNCAST (const char *, Msgid)`.  Under the feature naming the
   non-frozen side (P-51).  */
#[cfg (feature = "no_gnu_inline")]
macro_rules! gettext {
  ($msgid:expr) => { _LIBGETTEXT_FUNCAST! (*const c_char, $msgid) };
}

/* C 336-340's `#ifdef GNULIB_defined_setlocale`: dead (the frozen macro table has no
   `GNULIB_defined_setlocale`), so the port's `setlocale` is the system's.  */
#[cfg (feature = "gnulib_defined_setlocale")]
macro_rules! setlocale { () => { rpl_setlocale } }

/* C 343-350's `gettext_noop`, live in every arm: a marker for the message extractor whose
   expansion is its argument.  */
macro_rules! gettext_noop {
  ($string:expr) => { $string };
}

/* C 364-366's `#if !defined LC_MESSAGES / # define LC_MESSAGES 1729`: dead (C 361's
   `<locale.h>` defines it, measured 6), and its 1729 is what the blind extractor compares -
   hence first, gated (P-51/P-66②).  */
#[cfg (feature = "no_lc_messages")]
pub const LC_MESSAGES: c_int = 1729;
/// The frozen value: `LC_MESSAGES 6` (the system `<locale.h>`).
pub const LC_MESSAGES: c_int = 6;

/* C 368-389's pseudo calls.  C's `Msgctxt GETTEXT_CONTEXT_GLUE Msgid` is *adjacent string
   literals* (the header's comment requires literals, C 356-359), so Rust's `concat!` is the
   same compile-time concatenation; each macro's `const _` pins the const's byte to the escape
   `concat!` spells.  `core::ptr::null ()` is C's `NULL`.  */
macro_rules! pgettext {
  ($msgctxt:literal, $msgid:literal) => {
    unsafe {
      const _: () = core::assert! (GETTEXT_CONTEXT_GLUE.to_bytes ()[0] == 0x04);
      pgettext_aux (core::ptr::null (), concat! ($msgctxt, "\x04", $msgid, "\0").as_ptr ()
                      as *const c_char, $msgid, LC_MESSAGES)
    }
  };
}
macro_rules! dpgettext {
  ($domainname:expr, $msgctxt:literal, $msgid:literal) => {
    unsafe {
      const _: () = core::assert! (GETTEXT_CONTEXT_GLUE.to_bytes ()[0] == 0x04);
      pgettext_aux ($domainname, concat! ($msgctxt, "\x04", $msgid, "\0").as_ptr ()
                      as *const c_char, $msgid, LC_MESSAGES)
    }
  };
}
macro_rules! dcpgettext {
  ($domainname:expr, $msgctxt:literal, $msgid:literal, $category:expr) => {
    unsafe {
      const _: () = core::assert! (GETTEXT_CONTEXT_GLUE.to_bytes ()[0] == 0x04);
      pgettext_aux ($domainname, concat! ($msgctxt, "\x04", $msgid, "\0").as_ptr ()
                      as *const c_char, $msgid, $category)
    }
  };
}
macro_rules! npgettext {
  ($msgctxt:literal, $msgid:literal, $msgidplural:literal, $n:expr) => {
    unsafe {
      const _: () = core::assert! (GETTEXT_CONTEXT_GLUE.to_bytes ()[0] == 0x04);
      npgettext_aux (core::ptr::null (), concat! ($msgctxt, "\x04", $msgid, "\0").as_ptr ()
                       as *const c_char, $msgid, $msgidplural, $n, LC_MESSAGES)
    }
  };
}
macro_rules! dnpgettext {
  ($domainname:expr, $msgctxt:literal, $msgid:literal, $msgidplural:literal, $n:expr) => {
    unsafe {
      const _: () = core::assert! (GETTEXT_CONTEXT_GLUE.to_bytes ()[0] == 0x04);
      npgettext_aux ($domainname, concat! ($msgctxt, "\x04", $msgid, "\0").as_ptr ()
                       as *const c_char, $msgid, $msgidplural, $n, LC_MESSAGES)
    }
  };
}
macro_rules! dcnpgettext {
  ($domainname:expr, $msgctxt:literal, $msgid:literal, $msgidplural:literal, $n:expr,
   $category:expr) => {
    unsafe {
      const _: () = core::assert! (GETTEXT_CONTEXT_GLUE.to_bytes ()[0] == 0x04);
      npgettext_aux ($domainname, concat! ($msgctxt, "\x04", $msgid, "\0").as_ptr ()
                       as *const c_char, $msgid, $msgidplural, $n, $category)
    }
  };
}

/* C 324-331: `bindtextdomain` and `bind_textdomain_codeset` are live in every arm - the
   comma expression's value is discarded (C yields nothing), so the port evaluates both
   arguments and yields `()`.  */
macro_rules! bindtextdomain {
  ($domainname:expr, $dirname:expr) => {{
    let _ = _LIBGETTEXT_FUNCAST! (*const c_char, $domainname);
    let _ = _LIBGETTEXT_FUNCAST! (*const c_char, $dirname);
  }};
}
macro_rules! bind_textdomain_codeset {
  ($domainname:expr, $codeset:expr) => {{
    let _ = _LIBGETTEXT_FUNCAST! (*const c_char, $domainname);
    let _ = _LIBGETTEXT_FUNCAST! (*const c_char, $codeset);
  }};
}

/* C 396-406: the msgctxt lookup, shared by the six macros above (C spells it `__inline static`
   in the live arm; a Rust function carries no symbol either).  */
#[inline(always)]
pub unsafe fn pgettext_aux (domain: *const c_char, msg_ctxt_id: *const c_char,
                            msgid: *const c_char, category: c_int) -> *const c_char
{
  let translation: *const c_char = dcgettext (domain, msg_ctxt_id, category) as *const c_char;
  if translation == msg_ctxt_id
    { return msgid; }
  else
    { return translation; }
}

/* C 408-425: the plural form of the same.  */
#[inline(always)]
pub unsafe fn npgettext_aux (domain: *const c_char, msg_ctxt_id: *const c_char,
                             msgid: *const c_char, msgid_plural: *const c_char,
                             n: core::ffi::c_ulong, category: c_int) -> *const c_char
{
  let translation: *const c_char =
    dcngettext (domain, msg_ctxt_id, msgid_plural, n, category) as *const c_char;
  if translation == msg_ctxt_id || translation == msgid_plural
    { return if n == 1 { msgid } else { msgid_plural }; }
  else
    { return translation; }
}

/* C 440-446's `_LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS`: C's `#if` arm (1) is the frozen value
   and its `#else` arm (0) is the last definition the blind extractor takes, so the dead arm
   comes first, gated (P-51/P-66②).  */
#[cfg (feature = "no_variable_size_arrays")]
pub const _LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS: c_int = 0;
/// The frozen value: 1 (C's VLA is available; the port's two helpers below use the `#else`
/// arm's `buf`/`malloc` pair, since Rust has no variable-length array).
pub const _LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS: c_int = 1;

/* C 452-455 and C 496-499's four non-constant-argument pseudo calls.  */
macro_rules! pgettext_expr {
  ($msgctxt:expr, $msgid:expr) => {
    unsafe { dcpgettext_expr (core::ptr::null (), $msgctxt, $msgid, LC_MESSAGES) }
  };
}
macro_rules! dpgettext_expr {
  ($domainname:expr, $msgctxt:expr, $msgid:expr) => {
    unsafe { dcpgettext_expr ($domainname, $msgctxt, $msgid, LC_MESSAGES) }
  };
}
macro_rules! npgettext_expr {
  ($msgctxt:expr, $msgid:expr, $msgidplural:expr, $n:expr) => {
    unsafe { dcnpgettext_expr (core::ptr::null (), $msgctxt, $msgid, $msgidplural, $n,
                               LC_MESSAGES) }
  };
}
macro_rules! dnpgettext_expr {
  ($domainname:expr, $msgctxt:expr, $msgid:expr, $msgidplural:expr, $n:expr) => {
    unsafe { dcnpgettext_expr ($domainname, $msgctxt, $msgid, $msgidplural, $n, LC_MESSAGES) }
  };
}

/* C 457-494: the same lookup for arguments that are not literals.  C's live arm declares a VLA
   (`char msg_ctxt_id[msgctxt_len + msgid_len]`, C 471) or, in its `#else`, the `buf`/`malloc`
   pair (C 473-477); Rust has no VLA, so the mirror keeps the `#else` arm - whose tokens cover
   the VLA arm's (the VLA arm adds only `msg_ctxt_id` and the two lengths, all here).  */
#[inline(always)]
pub unsafe fn dcpgettext_expr (domain: *const c_char, msgctxt: *const c_char,
                               msgid: *const c_char, category: c_int) -> *const c_char
{
  let msgctxt_len: usize = strlen (msgctxt) + 1;
  let msgid_len: usize = strlen (msgid) + 1;
  let mut buf: [c_char; 1024] = [0; 1024];
  let msg_ctxt_id: *mut c_char =
    if msgctxt_len + msgid_len <= core::mem::size_of_val (&buf)
      { buf.as_mut_ptr () }
    else
      { malloc (msgctxt_len + msgid_len) as *mut c_char };
  let translation: *const c_char;
  if ! msg_ctxt_id.is_null ()
    {
      memcpy (msg_ctxt_id as *mut c_void, msgctxt as *const c_void, msgctxt_len - 1);
      *msg_ctxt_id.add (msgctxt_len - 1) = b'\x04' as c_char;
      memcpy (msg_ctxt_id.add (msgctxt_len) as *mut c_void, msgid as *const c_void, msgid_len);
      translation = dcgettext (domain, msg_ctxt_id, category) as *const c_char;
      let found_translation: c_int =
        (translation != msg_ctxt_id as *const c_char) as c_int;
      if msg_ctxt_id != buf.as_mut_ptr ()
        { free (msg_ctxt_id as *mut c_void); }
      if found_translation != 0
        { return translation; }
    }
  return msgid;
}

/* C 501-539: the plural form of the same.  */
#[inline(always)]
pub unsafe fn dcnpgettext_expr (domain: *const c_char, msgctxt: *const c_char,
                                msgid: *const c_char, msgid_plural: *const c_char,
                                n: core::ffi::c_ulong, category: c_int) -> *const c_char
{
  let msgctxt_len: usize = strlen (msgctxt) + 1;
  let msgid_len: usize = strlen (msgid) + 1;
  let mut buf: [c_char; 1024] = [0; 1024];
  let msg_ctxt_id: *mut c_char =
    if msgctxt_len + msgid_len <= core::mem::size_of_val (&buf)
      { buf.as_mut_ptr () }
    else
      { malloc (msgctxt_len + msgid_len) as *mut c_char };
  let translation: *const c_char;
  if ! msg_ctxt_id.is_null ()
    {
      memcpy (msg_ctxt_id as *mut c_void, msgctxt as *const c_void, msgctxt_len - 1);
      *msg_ctxt_id.add (msgctxt_len - 1) = b'\x04' as c_char;
      memcpy (msg_ctxt_id.add (msgctxt_len) as *mut c_void, msgid as *const c_void, msgid_len);
      translation = dcngettext (domain, msg_ctxt_id, msgid_plural, n, category)
                      as *const c_char;
      let found_translation: c_int =
        ! (translation == msg_ctxt_id as *const c_char
           || translation == msgid_plural as *const c_char) as c_int;
      if msg_ctxt_id != buf.as_mut_ptr ()
        { free (msg_ctxt_id as *mut c_void); }
      if found_translation != 0
        { return translation; }
    }
  return if n == 1 { msgid } else { msgid_plural };
}
