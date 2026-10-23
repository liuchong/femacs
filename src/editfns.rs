// femacs: port of src/editfns.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, file-scope object, local), same control flow, same error
// model.  The 80 `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  The non-`static` definitions C
// exports carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static` helpers stay plain
// `unsafe fn`.
//
// `src/editfns.c` *is* in `docs/baseline/config/cond-eval-verified.txt`
// (`python3 tools/check-cond-eval.py src/editfns.c` = `17 conditions, 0 unknown,
// 0 disagreements [VERIFIED]`), so per P-08 the arms the frozen configuration does not
// compile are *recorded in `docs/ports/editfns.md`, not transcribed*.  The dead arms:
//
//   * C 93-100, `init_editfns`'s `#ifdef MSDOS` (MSDOS undef, macros.tsv 4291).
//   * C 106-110, `init_editfns`'s `#ifdef WINDOWSNT` (WINDOWSNT undef).
//   * C 136-146, `init_editfns`'s `#elif defined WINDOWSNT`/`#else` (HAVE_SYS_UTSNAME_H is
//     1 on the frozen target, macros.tsv 1440), so the port keeps the first arm.
//   * C 36-38, `android.h` (HAVE_ANDROID undef); C 59-61, `w32common.h` (WINDOWSNT undef);
//     C 63-65, `treesit.h` (HAVE_TREE_SITTER undef) - three `#include`s the frozen
//     configuration never takes.
//   * C 2316-2327, 2428-2431, 2617-2625 and 2632-2640, the four `HAVE_TREE_SITTER` arms of
//     `Fsubst_char_in_region`/`Ftranslate_region_internal`/`Fdelete_region`
//     (HAVE_TREE_SITTER undef).
//   * C 4049-4054, `styled_format`'s `__STDC_VERSION_STDIO_H__` arm (the frozen target's
//     `<stdio.h>` does not define it), so the port keeps the `#else` text.
//
// **Use-site declarations.**  Several headers this file includes have no mirror; each entry
// below carries C's own prototype in a `/// C:` line (PORTING.md 0.1-2/P-72 rule 3, the
// sysdep.rs/indent.rs shape) and each record's layout is measured with the frozen clang and
// pinned by `const _` assertions (PORTING.md 1.2/P-73).  See `docs/ports/editfns.md` for the
// full list of mirror gaps this unit reports and does not touch.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 21: `#include <config.h>`
include!("../lib/include/sys_types.in.rs");    // C 22: `#include <sys/types.h>`
include!("../lib/include/stdio.rs");           // C 23: `#include <stdio.h>`
include!("../lib/include/unistd.in.rs");       // C 30: `#include <unistd.h>`
include!("include/lisp.rs");                   // C 40: `#include "lisp.h"`
include!("../lib/include/float.in.rs");        // C 42: `#include <float.h>`
include!("../lib/include/limits.in.rs");       // C 43: `#include <limits.h>`
include!("../lib/include/math.rs");            // C 44: `#include <math.h>`
include!("../lib/include/c-ctype.rs");         // C 46: `#include <c-ctype.h>`
include!("../lib/include/stdlib.rs");          // C 48: `#include <stdlib.h>`
include!("include/composite.rs");              // C 50: `#include "composite.h"`
include!("include/intervals.rs");              // C 51: `#include "intervals.h"`
include!("include/systime.rs");                // C 52: `#include "systime.h"`
include!("../lib/include/pwd.rs");           // C: `#include <pwd.h>`
include!("../lib/include/grp.rs");           // C: `#include <grp.h>`
include!("../lib/include/sys_utsname.rs");   // C: `#include <sys/utsname.h>`
include!("include/character.rs");              // C 53: `#include "character.h"`
include!("include/itree.rs");                  // buffer.h 27: `#include "itree.h"`
include!("include/buffer.rs");                 // C 54: `#include "buffer.h"`
include!("include/window.rs");                 // C 55: `#include "window.h"`
include!("include/dispextern.rs");            // window.h 23: `#include "dispextern.h"`,
                                               // for window.rs 159 `text_cursor_kinds`
include!("include/blockinput.rs");             // C 56: `#include "blockinput.h"`
include!("include/coding.rs");                 // C 57: `#include "coding.h"`
include!("../lib/include/string.rs");          // lisp.h's `<string.h>` (memcpy/strlen)
include!("../lib/include/stdcountof.rs");      // lisp.h 30's <stdcountof.h>: `countof`

/* `<intprops.h>` has no mirror (PORTING.md P-26): C's `TYPE_MAXIMUM (t)` is the type's own
   largest value, the syntax.rs/dtotimespec.rs shape; `INT_STRLEN_BOUND` is the digit bound
   `styled_format` needs.  */
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }
macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }

/* `USER_FULL_NAME` is a configure-time macro (`emacs/src/config.in` 2395 has it `#undef`;
   `configure.ac` 2667-2673 defines it `pw->pw_gecos` when `<pwd.h>` exists and the target is
   not Android, which is this frozen configuration).  C's spelling is kept as a macro so the
   body's token is the same.  */
macro_rules! USER_FULL_NAME { ($pw:expr) => { (*$pw).pw_gecos }; }

/* C spells `enum itree_order`'s members bare (`ASCENDING` in the `ITREE_FOREACH`
   expansion); the port names the enum's variants `ITREE_*` (itree.rs 99-105) and imports
   them here the way `ITREE_FOREACH`'s expansion sites do (the textprop.rs shape).  */
use itree_order::*;

/// `sysdep.c`, `eval.c` and `search.c` define these; lisp.h declares them (the last two
/// are declared *inside* lisp.h's block for search.c, which the mirror does not carry).
unsafe extern "C" {
  /// C: lisp.h 5591: `extern void init_system_name (void);` (sysdep.c).
  pub fn init_system_name ();
  /// C: lisp.h 4826: `extern void record_unwind_protect_excursion (void);` (eval.c).
  pub fn record_unwind_protect_excursion ();
}

/// search.c's two newline scanners C's bodies here call; lisp.h 5035-5043 declares both,
/// and the mirror carries only `scan_newline_from_point`.
unsafe extern "C" {


}

/* `intervals.h` 307's `text_property_stickiness` (defined in textprop.c, `src/textprop.rs`)
   and `get_char_property_and_overlay` are both in the `intervals.rs` mirror's `intervals_decls`
   now, so this unit names them through that header exactly as C does and the copy that stood
   here went away with the mirror entry (PORTING.md 0.1-2/0.1-10; two same-named declarations in
   one unit are an `E0428`).  */

/// insdel.c's six insertion entries and its two signaling helpers: lisp.h 4320-4342 and
/// 4938 declare them and the mirror carries none of them, so each is declared here with
/// C's own prototype above it (PORTING.md P-72 rule 3, the insdel.rs shape).
unsafe extern "C" {


  /// C: lisp.h 4340: `extern void insert_before_markers (const char *, ptrdiff_t);`
  pub fn insert_before_markers (str_: *const c_char, nbytes: ptrdiff_t);
  /// C: lisp.h 4341: `extern void insert_before_markers_and_inherit (const char *,
  /// ptrdiff_t);`
  pub fn insert_before_markers_and_inherit (str_: *const c_char, nbytes: ptrdiff_t);
  /// C: lisp.h 4337: `extern void insert_from_buffer (struct buffer *, ptrdiff_t,
  /// ptrdiff_t, bool);`
  pub fn insert_from_buffer (buf: *mut buffer, charpos: ptrdiff_t, nchars: ptrdiff_t,
                             inherit: bool);
  /// C: lisp.h 4320: `extern AVOID buffer_overflow (void);`
  pub fn buffer_overflow () -> !;
  /// C: lisp.h 4938: `extern AVOID nsberror (Lisp_Object);`
  pub fn nsberror (buffer: Lisp_Object) -> !;
}

/// eval.c, coding.c and profiler.c define these; lisp.h 4799 and 5468 and coding.h 684
/// declare them, and the mirror carries none of them.
unsafe extern "C" {
  /// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);` (eval.c).
  pub fn FUNCTIONP (object: Lisp_Object) -> bool;

  /// C: lisp.h 5468: `extern void malloc_probe (size_t);` (profiler.c).
  pub fn malloc_probe (size: usize);
}

/// insdel.c's two change recorders: lisp.h 4324 and 4330-4331 declare them and the mirror
/// carries neither.
unsafe extern "C" {


}

/// alloc.c, fns.c and indent.c define these; lisp.h 4271, 4656, 4957 and 5112 declare them
/// and the mirror carries none of them.
unsafe extern "C" {





}

/// xdisp.c defines these; lisp.h 4387 and 4392 declare them and the mirror carries neither.
unsafe extern "C" {


}

/// `<math.h>`: C 4013's `fabs`.
unsafe extern "C" {
  /// C: `<math.h>`: `double fabs (double);`
  pub fn fabs (x: c_double) -> c_double;
}

/// `<inttypes.h>`: `#define PRIdMAX "jd"` on the frozen target (`intmax_t` is `long long`),
/// the timefns.rs/print.rs/doprnt.rs spelling.
const PRIdMAX: &str = "jd";

/// C's `long double`.  clang measures the frozen target's `long double` as the same 64-bit
/// binary64 as `double` (`sizeof (long double) == 8`), and `<float.h>`'s `LDBL_MANT_DIG` is
/// 53 there — the mirror (`lib/include/float.in.rs`) carries the `DBL_*` family but not the
/// `LDBL_*` one, so both are declared at this use site (PORTING.md P-26's system-header
/// rule; reported in docs/ports/editfns.md).
pub type c_longdouble = c_double;
/// C: `<float.h>`'s `LDBL_MANT_DIG`, measured with clang against the frozen target: 53.
const LDBL_MANT_DIG: c_int = 53;

/// insdel.c's `copy_text` and eval.c's `set_unwind_protect_ptr`: lisp.h 4323 and 4832
/// declare them and the mirror carries neither.
unsafe extern "C" {
  /// C: lisp.h 4323-4324: `extern ptrdiff_t copy_text (const unsigned char *,
  /// unsigned char *, ptrdiff_t, bool, bool);`
  pub fn copy_text (from_addr: *const u8, to_addr: *mut u8, nbytes: ptrdiff_t,
                    from_multibyte: bool, to_multibyte: bool) -> ptrdiff_t;
  /// C: lisp.h 4832: `extern void set_unwind_protect_ptr (specpdl_ref, void (*) (void *),
  /// void *);`
  pub fn set_unwind_protect_ptr (count: specpdl_ref,
                                 function: Option<unsafe extern "C" fn (*mut c_void)>,
                                 arg: *mut c_void);
  /// C: lisp.h 4566-4567: `extern Lisp_Object make_specified_string (const char *chars,
  /// ptrdiff_t nchars, ptrdiff_t nbytes, bool multibyte);` (alloc.c defines it; the mirror
  /// does not carry it, and `search.rs` declares its own use site - this unit does *not*
  /// define it, so no E0428 is possible).
  pub fn make_specified_string (chars: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
                                multibyte: bool) -> Lisp_Object;
}

/// insdel.c's `adjust_markers_bytepos`: lisp.h 4361-4362 declares it and the mirror does not
/// carry it.
unsafe extern "C" {

}

/// `<pwd.h>`: C 1239's `getpwnam`.
unsafe extern "C" {
}

/* ==========================================================================================
   C 68-146: the file-scope declarations and entries.  */

/* C 68-70's two forward declarations (`update_buffer_properties`, `styled_format`) are
   dropped (PORTING.md 0.1-12); their definitions follow.  */

/* The cached value of Vsystem_name.  This is used only to compare it
   to Vsystem_name, so it need not be visible to the GC.  */
static mut cached_system_name: Lisp_Object = lisp_h_Qnil ();

unsafe fn init_and_cache_system_name ()
{
  init_system_name ();
  cached_system_name = Vsystem_name;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_editfns ()
{
  let mut user_name: *const c_char = std::ptr::null ();
  let mut p: *mut c_char = std::ptr::null_mut ();
  let mut pw: *mut passwd = std::ptr::null_mut ();
  let mut tem: Lisp_Object = Qnil ();

  /* Set up system_name even when dumping.  */
  init_and_cache_system_name ();

  pw = getpwuid (getuid ());
  /* #ifdef MSDOS: dead in the frozen configuration (docs/ports/editfns.md).  */
  Vuser_real_login_name = build_string (if pw.is_null () { c"unknown".as_ptr () }
                                        else { (*pw).pw_name });

  /* Get the effective user name, by consulting environment variables,
     or the effective uid if those are unset.  */
  user_name = getenv (c"LOGNAME".as_ptr ());
  if user_name.is_null ()
    /* #ifdef WINDOWSNT: dead (docs/ports/editfns.md).  */
    { user_name = getenv (c"USER".as_ptr ()); }
  if user_name.is_null ()
    {
      pw = getpwuid (geteuid ());
      user_name = if pw.is_null () { c"unknown".as_ptr () } else { (*pw).pw_name };
    }
  Vuser_login_name = build_string (user_name);

  /* If the user name claimed in the environment vars differs from
     the real uid, use the claimed name to find the full name.  */
  tem = Fstring_equal (Vuser_login_name, Vuser_real_login_name);
  if !NILP (tem)
    { tem = Vuser_login_name; }
  else
    {
      let euid: uid_t = geteuid ();
      tem = INT_TO_INTEGER (euid as EMACS_INT);
    }
  Vuser_full_name = Fuser_full_name (tem);

  p = getenv (c"NAME".as_ptr ());
  if !p.is_null ()
    { Vuser_full_name = build_string (p); }
  else if NILP (Vuser_full_name)
    { Vuser_full_name = build_string (c"unknown".as_ptr ()); }

  {
    let mut uts: utsname = std::mem::zeroed ();
    uname (&mut uts);
    Voperating_system_release = build_string (uts.release.as_ptr ());
  }
}

DEFUN! ("char-to-string", Fchar_to_string, Schar_to_string, 1, 1, 0,
       doc: "Convert arg CHAR to a string containing that character.\nusage: (char-to-string CHAR)",
       (character: Lisp_Object),
       {
  let mut c: c_int = 0;
  let mut len: c_int = 0;
  let mut str_: [u8; MAX_MULTIBYTE_LENGTH as usize] = std::mem::zeroed ();

  CHECK_CHARACTER (character);
  c = XFIXNAT (character) as c_int;

  len = CHAR_STRING (c, str_.as_mut_ptr ());
  return make_string_from_bytes (str_.as_ptr () as *const c_char, 1, len as ptrdiff_t);
});

DEFUN! ("byte-to-string", Fbyte_to_string, Sbyte_to_string, 1, 1, 0,
       doc: "Convert arg BYTE to a unibyte string containing that byte.",
       (byte: Lisp_Object),
       {
  let mut b: u8 = 0;
  CHECK_FIXNUM (byte);
  if XFIXNUM (byte) < 0 || XFIXNUM (byte) > 255
    { error (c"Invalid byte".as_ptr ()); }
  b = XFIXNUM (byte) as u8;
  return make_unibyte_string (&raw const b as *const c_char, 1);
});

DEFUN! ("string-to-char", Fstring_to_char, Sstring_to_char, 1, 1, 0,
       doc: "Return the first character in STRING.",
       (string: Lisp_Object),
       {
  CHECK_STRING (string);

  /* This returns zero if STRING is empty.  */
  let ch: c_int = if STRING_MULTIBYTE (string)
    { STRING_CHAR (SDATA (string)) }
    else { SREF (string, 0) as c_int };
  return make_fixnum (ch as EMACS_INT);
});

DEFUN! ("point", Fpoint, Spoint, 0, 0, 0,
       doc: "Return value of point, as an integer.\nBeginning of buffer is position (point-min).",
       (),
       {
  let mut temp: Lisp_Object = Qnil ();
  XSETFASTINT! (temp, PT! ());
  return temp;
});

DEFUN! ("point-marker", Fpoint_marker, Spoint_marker, 0, 0, 0,
       doc: "Return value of point, as a marker object.",
       (),
       {
  return build_marker ((current_buffer! ()) as *mut c_void, PT! (), PT_BYTE! ());
});

DEFUN! ("goto-char", Fgoto_char, Sgoto_char, 1, 1,
       "(goto-char--read-natnum-interactive \"Go to char: \")",
       doc: "Set point to POSITION, a number or marker.\nBeginning of buffer is position (point-min), end is (point-max).\n\nThe return value is POSITION.\n\nIf called interactively, a numeric prefix argument specifies\nPOSITION; without a numeric prefix argument, read POSITION from the\nminibuffer.  The default value is the number at point (if any).",
       (position: Lisp_Object),
       {
  if MARKERP (position)
    { set_point_from_marker (position); }
  else if FIXNUMP (position)
    { SET_PT (clip_to_bounds (BEGV! (), XFIXNUM (position), ZV! ())); }
  else
    { wrong_type_argument (Qinteger_or_marker_p (), position); }
  return position;
});

/* Return the start or end position of the region.
   BEGINNINGP means return the start.
   If there is no region active, signal an error.  */

unsafe fn region_limit (beginningp: bool) -> Lisp_Object
{
  let mut m: Lisp_Object = Qnil ();

  if !NILP (Vtransient_mark_mode)
      && NILP (Vmark_even_if_inactive)
      && NILP (BVAR! ((current_buffer! ()), mark_active_))
    { xsignal0 (Qmark_inactive ()); }

  m = Fmarker_position (BVAR! ((current_buffer! ()), mark_));
  if NILP (m)
    { error (c"The mark is not set now, so there is no region".as_ptr ()); }

  /* Clip to the current narrowing (bug#11770).  */
  return make_fixnum (if (PT! () < XFIXNAT (m)) == beginningp
                      { PT! () as EMACS_INT }
                      else { clip_to_bounds (BEGV! (), XFIXNAT (m), ZV! ()) });
}

DEFUN! ("region-beginning", Fregion_beginning, Sregion_beginning, 0, 0, 0,
       doc: "Return the integer value of point or mark, whichever is smaller.",
       (),
       {
  return region_limit (true);
});

DEFUN! ("region-end", Fregion_end, Sregion_end, 0, 0, 0,
       doc: "Return the integer value of point or mark, whichever is larger.",
       (),
       {
  return region_limit (false);
});

DEFUN! ("mark-marker", Fmark_marker, Smark_marker, 0, 0, 0,
       doc: "Return this buffer's mark, as a marker object.\nWatch out!  Moving this marker changes the mark position.\nIf you set the marker not to point anywhere, the buffer will have no mark.",
       (),
       {
  return BVAR! ((current_buffer! ()), mark_);
});


DEFUN! ("get-pos-property", Fget_pos_property, Sget_pos_property, 2, 3, 0,
       doc: "Return the value of POSITION's property PROP, in OBJECT.\nAlmost identical to `get-char-property' except for the following difference:\nWhereas `get-char-property' returns the property of the char at (i.e. right\nafter) POSITION, this pays attention to properties's stickiness and overlays's\nadvancement settings, in order to find the property of POSITION itself,\ni.e. the property that a char would inherit if it were inserted\nat POSITION.",
       (position: Lisp_Object, prop: Lisp_Object, object: Lisp_Object),
       {
  let mut position: Lisp_Object = position;
  let mut object: Lisp_Object = object;

  CHECK_FIXNUM_COERCE_MARKER! (position);

  if NILP (object)
    { XSETBUFFER! (object, (current_buffer! ())); }
  else if WINDOWP (object)
    { object = (*XWINDOW (object)).contents; }

  if !BUFFERP (object)
    /* pos-property only makes sense in buffers right now, since strings
       have no overlays and no notion of insertion for which stickiness
       could be obeyed.  */
    { return Fget_text_property (position, prop, object); }
  else
    {
      let posn: EMACS_INT = XFIXNUM (position);
      let mut tem: Lisp_Object = Qnil ();
      let obuf: *mut buffer = (current_buffer! ());
      let mut node: *mut itree_node = std::ptr::null_mut ();
      let mut items: [sortvec; 2] = [std::mem::zeroed (); 2];
      let b: *mut buffer = XBUFFER (object);
      let mut result: *mut sortvec = std::ptr::null_mut ();
      let mut res: Lisp_Object = Qnil ();

      set_buffer_temp (b);

      /* itree.h 166-176's `ITREE_FOREACH (node, b->overlays, posn - 1, posn + 1,
	 ASCENDING)` is expanded here (P-33, status 3.7), the textprop.rs shape.  */
      if (*b).overlays.is_null ()
        { }
      else
        {
          let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
          let itree_iter_: *mut itree_iterator
            = itree_iterator_start (&raw mut itree_local_iter_,
                                    (*b).overlays, posn - 1, posn + 1, ITREE_ASCENDING);
          while { node = itree_iterator_next (itree_iter_); !node.is_null () }
            {
              let ol: Lisp_Object = (*node).data;
              tem = Foverlay_get (ol, prop);
              if NILP (tem)
                  /* Check the overlay is indeed active at point.  */
                  || (((*node).begin == posn
                       && OVERLAY_FRONT_ADVANCE_P (ol))
                      || ((*node).end == posn
                          && !OVERLAY_REAR_ADVANCE_P (ol))
                      || (*node).begin > posn
                      || (*node).end < posn)
                /* The overlay will not cover a char inserted at point.  */
                { continue; }

              let this: *mut sortvec = if std::ptr::eq (result, &raw mut items[0])
                { &raw mut items[1] } else { &raw mut items[0] };
              if NILP (res)
                  || { make_sortvec_item (this, (*node).data);
                       compare_overlays (result as *const c_void, this as *const c_void) < 0 }
                {
                  result = this;
                  res = tem;
                }
            }
        }
      set_buffer_temp (obuf);

      if !NILP (res)
        { return res; }

      { /* Now check the text properties.  */
        let stickiness: c_int = text_property_stickiness (prop, position, object);
        if stickiness > 0
          { return Fget_text_property (position, prop, object); }
        else if stickiness < 0
                 && XFIXNUM (position) > BUF_BEGV (XBUFFER (object))
          { return Fget_text_property (make_fixnum (XFIXNUM (position) - 1),
                                       prop, object); }
        else
          { return Qnil (); }
      }
    }
});

/* Find the field surrounding POS in *BEG and *END.  If POS is nil,
   the value of point is used instead.  If BEG or END is null,
   means don't store the beginning or end of the field.

   BEG_LIMIT and END_LIMIT serve to limit the ranged of the returned
   results; they do not effect boundary behavior.

   If MERGE_AT_BOUNDARY is non-nil, then if POS is at the very first
   position of a field, then the beginning of the previous field is
   returned instead of the beginning of POS's field (since the end of a
   field is actually also the beginning of the next input field, this
   behavior is sometimes useful).  Additionally in the MERGE_AT_BOUNDARY
   non-nil case, if two fields are separated by a field with the special
   value `boundary', and POS lies within it, then the two separated
   fields are considered to be adjacent, and POS between them, when
   finding the beginning and ending of the "merged" field.

   Either BEG or END may be 0, in which case the corresponding value
   is not stored.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_field (pos: Lisp_Object, merge_at_boundary: Lisp_Object,
                                     beg_limit: Lisp_Object, beg: *mut ptrdiff_t,
                                     end_limit: Lisp_Object, end: *mut ptrdiff_t)
{
  /* Fields right before and after the point.  */
  let mut before_field: Lisp_Object = Qnil ();
  let mut after_field: Lisp_Object = Qnil ();
  /* True if POS counts as the start of a field.  */
  let mut at_field_start: bool = false;
  /* True if POS counts as the end of a field.  */
  let mut at_field_end: bool = false;

  let mut pos: Lisp_Object = pos;

  if NILP (pos)
    { XSETFASTINT! (pos, PT! ()); }
  else
    { CHECK_FIXNUM_COERCE_MARKER! (pos); }

  after_field
    = get_char_property_and_overlay (pos, Qfield (), Qnil (), std::ptr::null_mut ());
  before_field
    = if XFIXNAT (pos) > BEGV! ()
      { get_char_property_and_overlay (make_fixnum (XFIXNUM (pos) - 1),
                                       Qfield (), Qnil (), std::ptr::null_mut ()) }
      /* Using nil here would be a more obvious choice, but it would
         fail when the buffer starts with a non-sticky field.  */
      else { after_field };

  /* See if we need to handle the case where MERGE_AT_BOUNDARY is nil
     and POS is at beginning of a field, which can also be interpreted
     as the end of the previous field.  Note that the case where if
     MERGE_AT_BOUNDARY is non-nil (see function comment) is actually the
     more natural one; then we avoid treating the beginning of a field
     specially.  */
  if NILP (merge_at_boundary)
    {
      let field: Lisp_Object = Fget_pos_property (pos, Qfield (), Qnil ());
      if !EQ (field, after_field)
        { at_field_end = true; }
      if !EQ (field, before_field)
        { at_field_start = true; }
      if NILP (field) && at_field_start && at_field_end
        /* If an inserted char would have a nil field while the surrounding
           text is non-nil, we're probably not looking at a
           zero-length field, but instead at a non-nil field that's
           not intended for editing (such as comint's prompts).  */
        { at_field_end = false; at_field_start = false; }
    }

  /* Note about special `boundary' fields:

     Consider the case where the point (`.') is between the fields `x' and `y':

	xxxx.yyyy

     In this situation, if merge_at_boundary is non-nil, consider the
     `x' and `y' fields as forming one big merged field, and so the end
     of the field is the end of `y'.

     However, if `x' and `y' are separated by a special `boundary' field
     (a field with a `field' char-property of 'boundary), then ignore
     this special field when merging adjacent fields.  Here's the same
     situation, but with a `boundary' field between the `x' and `y' fields:

	xxx.BBBByyyy

     Here, if point is at the end of `x', the beginning of `y', or
     anywhere in-between (within the `boundary' field), merge all
     three fields and consider the beginning as being the beginning of
     the `x' field, and the end as being the end of the `y' field.  */

  if !beg.is_null ()
    {
      if at_field_start
        /* POS is at the edge of a field, and we should consider it as
           the beginning of the following field.  */
        { *beg = XFIXNAT (pos); }
      else
        /* Find the previous field boundary.  */
        {
          let mut p: Lisp_Object = pos;
          if !NILP (merge_at_boundary) && EQ (before_field, Qboundary ())
            /* Skip a `boundary' field.  */
            { p = Fprevious_single_char_property_change (p, Qfield (), Qnil (),
                                                         beg_limit); }

          p = Fprevious_single_char_property_change (p, Qfield (), Qnil (),
                                                     beg_limit);
          *beg = if NILP (p) { BEGV! () } else { XFIXNAT (p) };
        }
    }

  if !end.is_null ()
    {
      if at_field_end
        /* POS is at the edge of a field, and we should consider it as
           the end of the previous field.  */
        { *end = XFIXNAT (pos); }
      else
        /* Find the next field boundary.  */
        {
          if !NILP (merge_at_boundary) && EQ (after_field, Qboundary ())
            /* Skip a `boundary' field.  */
            { pos = Fnext_single_char_property_change (pos, Qfield (), Qnil (),
                                                       end_limit); }

          pos = Fnext_single_char_property_change (pos, Qfield (), Qnil (),
                                                   end_limit);
          *end = if NILP (pos) { ZV! () } else { XFIXNAT (pos) };
        }
    }
}

DEFUN! ("delete-field", Fdelete_field, Sdelete_field, 0, 1, 0,
       doc: "Delete the field surrounding POS.\nA field is a region of text with the same `field' property.\nIf POS is nil, the value of point is used for POS.",
       (pos: Lisp_Object),
       {
  let mut beg: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;
  find_field (pos, Qnil (), Qnil (), &mut beg, Qnil (), &mut end);
  if beg != end
    { del_range (beg, end); }
  return Qnil ();
});

DEFUN! ("field-string", Ffield_string, Sfield_string, 0, 1, 0,
       doc: "Return the contents of the field surrounding POS as a string.\nA field is a region of text with the same `field' property.\nIf POS is nil, the value of point is used for POS.",
       (pos: Lisp_Object),
       {
  let mut beg: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;
  find_field (pos, Qnil (), Qnil (), &mut beg, Qnil (), &mut end);
  return make_buffer_string (beg, end, true);
});

DEFUN! ("field-string-no-properties", Ffield_string_no_properties,
       Sfield_string_no_properties, 0, 1, 0,
       doc: "Return the contents of the field around POS, without text properties.\nA field is a region of text with the same `field' property.\nIf POS is nil, the value of point is used for POS.",
       (pos: Lisp_Object),
       {
  let mut beg: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;
  find_field (pos, Qnil (), Qnil (), &mut beg, Qnil (), &mut end);
  return make_buffer_string (beg, end, false);
});

DEFUN! ("field-beginning", Ffield_beginning, Sfield_beginning, 0, 3, 0,
       doc: "Return the beginning of the field surrounding POS.\nA field is a region of text with the same `field' property.\nIf POS is nil, the value of point is used for POS.\nIf ESCAPE-FROM-EDGE is non-nil and POS is at the beginning of its\nfield, then the beginning of the *previous* field is returned.\nIf LIMIT is non-nil, it is a buffer position; if the beginning of the field\nis before LIMIT, then LIMIT will be returned instead.",
       (pos: Lisp_Object, escape_from_edge: Lisp_Object, limit: Lisp_Object),
       {
  let mut beg: ptrdiff_t = 0;
  find_field (pos, escape_from_edge, limit, &mut beg, Qnil (), std::ptr::null_mut ());
  return make_fixnum (beg);
});

DEFUN! ("field-end", Ffield_end, Sfield_end, 0, 3, 0,
       doc: "Return the end of the field surrounding POS.\nA field is a region of text with the same `field' property.\nIf POS is nil, the value of point is used for POS.\nIf ESCAPE-FROM-EDGE is non-nil and POS is at the end of its field,\nthen the end of the *following* field is returned.\nIf LIMIT is non-nil, it is a buffer position; if the end of the field\nis after LIMIT, then LIMIT will be returned instead.",
       (pos: Lisp_Object, escape_from_edge: Lisp_Object, limit: Lisp_Object),
       {
  let mut end: ptrdiff_t = 0;
  find_field (pos, escape_from_edge, Qnil (), std::ptr::null_mut (), limit, &mut end);
  return make_fixnum (end);
});
DEFUN! ("constrain-to-field", Fconstrain_to_field, Sconstrain_to_field, 2, 5, 0,
       doc: "Return the position closest to NEW-POS that is in the same field as OLD-POS.\nA field is a region of text with the same `field' property.\n\nIf NEW-POS is nil, then use the current point instead, and move point\nto the resulting constrained position, in addition to returning that\nposition.\n\nIf OLD-POS is at the boundary of two fields, then the allowable\npositions for NEW-POS depends on the value of the optional argument\nESCAPE-FROM-EDGE: If ESCAPE-FROM-EDGE is nil, then NEW-POS is\nconstrained to the field that has the same `field' char-property\nas any new characters inserted at OLD-POS, whereas if ESCAPE-FROM-EDGE\nis non-nil, NEW-POS is constrained to the union of the two adjacent\nfields.  Additionally, if two fields are separated by another field with\nthe special value `boundary', then any point within this special field is\nalso considered to be `on the boundary'.\n\nIf the optional argument ONLY-IN-LINE is non-nil and constraining\nNEW-POS would move it to a different line, NEW-POS is returned\nunconstrained.  This is useful for commands that move by line, like\n\\[next-line] or \\[beginning-of-line], which should generally respect field boundaries\nonly in the case where they can still move to the right line.\n\nIf the optional argument INHIBIT-CAPTURE-PROPERTY is non-nil, and OLD-POS has\na non-nil property of that name, then any field boundaries are ignored.\n\nField boundaries are not noticed if `inhibit-field-text-motion' is non-nil.",
       (new_pos: Lisp_Object, old_pos: Lisp_Object, escape_from_edge: Lisp_Object,
        only_in_line: Lisp_Object, inhibit_capture_property: Lisp_Object),
       {
  let mut new_pos: Lisp_Object = new_pos;
  let mut old_pos: Lisp_Object = old_pos;

  /* If non-zero, then the original point, before re-positioning.  */
  let mut orig_point: ptrdiff_t = 0;
  let mut fwd: bool = false;
  let mut prev_old: Lisp_Object = Qnil ();
  let mut prev_new: Lisp_Object = Qnil ();

  if NILP (new_pos)
    /* Use the current point, and afterwards, set it.  */
    {
      orig_point = PT! ();
      XSETFASTINT! (new_pos, PT! ());
    }

  CHECK_FIXNUM_COERCE_MARKER! (new_pos);
  CHECK_FIXNUM_COERCE_MARKER! (old_pos);

  fwd = XFIXNUM (new_pos) > XFIXNUM (old_pos);

  prev_old = make_fixnum (XFIXNUM (old_pos) - 1);
  prev_new = make_fixnum (XFIXNUM (new_pos) - 1);

  if NILP (Vinhibit_field_text_motion)
      && !BASE_EQ (new_pos, old_pos)
      && (!NILP (Fget_char_property (new_pos, Qfield (), Qnil ()))
          || !NILP (Fget_char_property (old_pos, Qfield (), Qnil ()))
          /* To recognize field boundaries, we must also look at the
             previous positions; we could use `Fget_pos_property'
             instead, but in itself that would fail inside non-sticky
             fields (like comint prompts).  */
          || (XFIXNAT (new_pos) > BEGV! ()
              && !NILP (Fget_char_property (prev_new, Qfield (), Qnil ())))
          || (XFIXNAT (old_pos) > BEGV! ()
              && !NILP (Fget_char_property (prev_old, Qfield (), Qnil ()))))
      && (NILP (inhibit_capture_property)
          /* Field boundaries are again a problem; but now we must
             decide the case exactly, so we need to call
             `get_pos_property' as well.  */
          || (NILP (Fget_pos_property (old_pos, inhibit_capture_property, Qnil ()))
              && (XFIXNAT (old_pos) <= BEGV! ()
                  || NILP (Fget_char_property
                           (old_pos, inhibit_capture_property, Qnil ()))
                  || NILP (Fget_char_property
                           (prev_old, inhibit_capture_property, Qnil ())))))
    /* It is possible that NEW_POS is not within the same field as
       OLD_POS; try to move NEW_POS so that it is.  */
    {
      let mut counted: ptrdiff_t = 0;
      let mut field_bound: Lisp_Object = Qnil ();

      if fwd
        { field_bound = Ffield_end (old_pos, escape_from_edge, new_pos); }
      else
        { field_bound = Ffield_beginning (old_pos, escape_from_edge, new_pos); }

      if /* See if ESCAPE_FROM_EDGE caused FIELD_BOUND to jump to the
             other side of NEW_POS, which would mean that NEW_POS is
             already acceptable, and it's not necessary to constrain it
             to FIELD_BOUND.  */
         (if XFIXNAT (field_bound) < XFIXNAT (new_pos) { fwd } else { !fwd })
         /* NEW_POS should be constrained, but only if either
            ONLY_IN_LINE is nil (in which case any constraint is OK),
            or NEW_POS and FIELD_BOUND are on the same line (in which
            case the constraint is OK even if ONLY_IN_LINE is non-nil).  */
         && (NILP (only_in_line)
             /* This is the ONLY_IN_LINE case, check that NEW_POS and
                FIELD_BOUND are on the same line by seeing whether
                there's an intervening newline or not.  */
             || { find_newline (XFIXNAT (new_pos), -1,
                                XFIXNAT (field_bound), -1,
                                if fwd { -1 } else { 1 }, &mut counted,
                                std::ptr::null_mut (), true);
                  counted == 0 })
        /* Constrain NEW_POS to FIELD_BOUND.  */
        { new_pos = field_bound; }

      if orig_point != 0 && XFIXNAT (new_pos) != orig_point
        /* The NEW_POS argument was originally nil, so automatically set PT. */
        { SET_PT (XFIXNAT (new_pos)); }
    }

  return new_pos;
});

unsafe fn bol (n: Lisp_Object, out_count: *mut ptrdiff_t) -> ptrdiff_t
{
  let mut bytepos: ptrdiff_t = 0;
  let mut charpos: ptrdiff_t = 0;
  let mut count: ptrdiff_t = 0;

  if NILP (n)
    { count = 0; }
  else if FIXNUMP (n)
    { count = clip_to_bounds (-BUF_BYTES_MAX, XFIXNUM (n) - 1, BUF_BYTES_MAX); }
  else
    {
      CHECK_INTEGER (n);
      count = if NILP (Fnatnump (n)) { -BUF_BYTES_MAX } else { BUF_BYTES_MAX };
    }
  if !out_count.is_null ()
    { *out_count = count; }
  scan_newline_from_point (count, &mut charpos, &mut bytepos);
  return charpos;
}

DEFUN! ("pos-bol", Fpos_bol, Spos_bol, 0, 1, 0,
       doc: "Return the position of the first character on the current line.\nWith optional argument N, scan forward N - 1 lines first.\nIf the scan reaches the end of the buffer, return that position.\n\nThis function ignores text display directionality; it returns the\nposition of the first character in logical order, i.e. the smallest\ncharacter position on the logical line.  See `vertical-motion' for\nmovement by screen lines.\n\nThis function does not move point.  Also see `line-beginning-position'.",
       (n: Lisp_Object),
       {
  return make_fixnum (bol (n, std::ptr::null_mut ()));
});

DEFUN! ("line-beginning-position", Fline_beginning_position, Sline_beginning_position,
       0, 1, 0,
       doc: "Return the position of the first character in the current line/field.\nWith optional argument N non-nil, move forward N - 1 lines first.\nThis function is like `pos-bol' (which see), but respects fields.\n\nThis function constrains the returned position to the current field\nunless that position would be on a different line from the original,\nunconstrained result.  If N is nil or 1, and a front-sticky field\nstarts at point, the scan stops as soon as it starts.  To ignore field\nboundaries, bind `inhibit-field-text-motion' to t.\n\nThis function does not move point.",
       (n: Lisp_Object),
       {
  let mut count: ptrdiff_t = 0;
  let charpos: ptrdiff_t = bol (n, &mut count);
  /* Return END constrained to the current input field.  */
  return Fconstrain_to_field (make_fixnum (charpos), make_fixnum (PT! ()),
                              if count != 0 { Qt () } else { Qnil () }, Qt (), Qnil ());
});

unsafe fn eol (n: Lisp_Object) -> ptrdiff_t
{
  let mut count: ptrdiff_t;

  if NILP (n)
    { count = 1; }
  else if FIXNUMP (n)
    { count = clip_to_bounds (-BUF_BYTES_MAX, XFIXNUM (n), BUF_BYTES_MAX); }
  else
    {
      CHECK_INTEGER (n);
      count = if NILP (Fnatnump (n)) { -BUF_BYTES_MAX } else { BUF_BYTES_MAX };
    }
  return find_before_next_newline (PT! (), 0, count - (count <= 0) as ptrdiff_t,
                                   std::ptr::null_mut ());
}

DEFUN! ("pos-eol", Fpos_eol, Spos_eol, 0, 1, 0,
       doc: "Return the position of the last character on the current line.\nWith argument N not nil or 1, move forward N - 1 lines first.\nIf scan reaches end of buffer, return that position.\n\nThis function ignores text display directionality; it returns the\nposition of the last character in logical order, i.e. the largest\ncharacter position on the line.\n\nThis function does not move point.  Also see `line-end-position'.",
       (n: Lisp_Object),
       {
  return make_fixnum (eol (n));
});

DEFUN! ("line-end-position", Fline_end_position, Sline_end_position, 0, 1, 0,
       doc: "Return the position of the last character in the current line/field.\nWith argument N not nil or 1, move forward N - 1 lines first.\nIf scan reaches end of buffer, return that position.\n\nThis function is like `pos-eol' (which see), but respects fields.\n\nThis function constrains the returned position to the current field\nunless that would be on a different line from the original,\nunconstrained result.  If N is nil or 1, and a rear-sticky field ends\nat point, the scan stops as soon as it starts.  To ignore field\nboundaries bind `inhibit-field-text-motion' to t.\n\nThis function does not move point.",
       (n: Lisp_Object),
       {
  /* Return END_POS constrained to the current input field.  */
  return Fconstrain_to_field (make_fixnum (eol (n)), make_fixnum (PT! ()),
                              Qnil (), Qt (), Qnil ());
});

/* Save current buffer state for save-excursion special form.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn save_excursion_save (pdl: *mut specbinding)
{
  eassert! ((*pdl).unwind_excursion.kind == SPECPDL_UNWIND_EXCURSION as u8);
  (*pdl).unwind_excursion.marker = Fpoint_marker ();
  /* Selected window if current buffer is shown in it, nil otherwise.  */
  (*pdl).unwind_excursion.window
    = if BASE_EQ ((*XWINDOW (selected_window)).contents, Fcurrent_buffer ())
      { selected_window } else { Qnil () };
}

/* Restore saved buffer before leaving `save-excursion' special form.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn save_excursion_restore (marker: Lisp_Object, window: Lisp_Object)
{
  let buffer: Lisp_Object = Fmarker_buffer (marker);
  /* If we're unwinding to top level, saved buffer may be deleted.  This
     means that all of its markers are unchained and so BUFFER is nil.  */
  if NILP (buffer)
    { return; }

  Fset_buffer (buffer);

  /* Point marker.  */
  Fgoto_char (marker);
  unchain_marker (XMARKER (marker) as *mut c_void);

  /* If buffer was visible in a window, and a different window was
     selected, and the old selected window is still showing this
     buffer, restore point in that window.  */
  if WINDOWP (window) && !BASE_EQ (window, selected_window)
    {
      /* Set window point if WINDOW is live and shows the current buffer.  */
      let contents: Lisp_Object = (*XWINDOW (window)).contents;
      if BUFFERP (contents) && XBUFFER (contents) == (current_buffer! ())
        { Fset_window_point (window, make_fixnum (PT! ())); }
    }
}

DEFUN! ("save-excursion", Fsave_excursion, Ssave_excursion, 0, UNEVALLED, 0,
       doc: "Save point, and current buffer; execute BODY; restore those things.\nExecutes BODY just like `progn'.\nThe values of point and the current buffer are restored\neven in case of abnormal exit (throw or error).\n\nIf you only want to save the current buffer but not point,\nthen just use `save-current-buffer', or even `with-current-buffer'.\n\nBefore Emacs 25.1, `save-excursion' used to save the mark state.\nTo save the mark state as well as point and the current buffer, use\n`save-mark-and-excursion'.\n\nusage: (save-excursion &rest BODY)",
       (args: Lisp_Object),
       {
  let mut val: Lisp_Object = Qnil ();
  let count: specpdl_ref = SPECPDL_INDEX ();

  record_unwind_protect_excursion ();

  val = Fprogn (args);
  return unbind_to (count, val);
});

DEFUN! ("save-current-buffer", Fsave_current_buffer, Ssave_current_buffer, 0, UNEVALLED, 0,
       doc: "Record which buffer is current; execute BODY; make that buffer current.\nBODY is executed just like `progn'.\nusage: (save-current-buffer &rest BODY)",
       (args: Lisp_Object),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();

  record_unwind_current_buffer ();
  return unbind_to (count, Fprogn (args));
});

DEFUN! ("buffer-size", Fbuffer_size, Sbuffer_size, 0, 1, 0,
       doc: "Return the number of characters in the current buffer.\nIf BUFFER is not nil, return the number of characters in that buffer\ninstead.\n\nThis does not take narrowing into account; to count the number of\ncharacters in the accessible portion of the current buffer, use\n`(- (point-max) (point-min))', and to count the number of characters\nin the accessible portion of some other BUFFER, use\n`(with-current-buffer BUFFER (- (point-max) (point-min)))'.",
       (buffer: Lisp_Object),
       {
  if NILP (buffer)
    { return make_fixnum ((Z! () - BEG as ptrdiff_t) as EMACS_INT); }
  else
    {
      CHECK_BUFFER (buffer);
      return make_fixnum (BUF_Z! (XBUFFER (buffer)) - BUF_BEG (XBUFFER (buffer)));
    }
});

DEFUN! ("point-min", Fpoint_min, Spoint_min, 0, 0, 0,
       doc: "Return the minimum permissible value of point in the current buffer.\nThis is 1, unless narrowing (a buffer restriction) is in effect.",
       (),
       {
  let mut temp: Lisp_Object = Qnil ();
  XSETFASTINT! (temp, BEGV! ());
  return temp;
});

DEFUN! ("point-min-marker", Fpoint_min_marker, Spoint_min_marker, 0, 0, 0,
       doc: "Return a marker to the minimum permissible value of point in this buffer.\nThis is the beginning, unless narrowing (a buffer restriction) is in effect.",
       (),
       {
  return build_marker ((current_buffer! ()) as *mut c_void, BEGV! (), BEGV_BYTE! ());
});

DEFUN! ("point-max", Fpoint_max, Spoint_max, 0, 0, 0,
       doc: "Return the maximum permissible value of point in the current buffer.\nThis is (1+ (buffer-size)), unless narrowing (a buffer restriction)\nis in effect, in which case it is less.",
       (),
       {
  let mut temp: Lisp_Object = Qnil ();
  XSETFASTINT! (temp, ZV! ());
  return temp;
});

DEFUN! ("point-max-marker", Fpoint_max_marker, Spoint_max_marker, 0, 0, 0,
       doc: "Return a marker to the maximum permissible value of point in this buffer.\nThis is (1+ (buffer-size)), unless narrowing (a buffer restriction)\nis in effect, in which case it is less.",
       (),
       {
  return build_marker ((current_buffer! ()) as *mut c_void, ZV! (), ZV_BYTE! ());
});

DEFUN! ("gap-position", Fgap_position, Sgap_position, 0, 0, 0,
       doc: "Return the position of the gap, in the current buffer.\nSee also `gap-size'.",
       (),
       {
  let mut temp: Lisp_Object = Qnil ();
  XSETFASTINT! (temp, GPT! ());
  return temp;
});

DEFUN! ("gap-size", Fgap_size, Sgap_size, 0, 0, 0,
       doc: "Return the size of the current buffer's gap.\nSee also `gap-position'.",
       (),
       {
  let mut temp: Lisp_Object = Qnil ();
  XSETFASTINT! (temp, GAP_SIZE! ());
  return temp;
});

DEFUN! ("position-bytes", Fposition_bytes, Sposition_bytes, 1, 1, 0,
       doc: "Return the byte position for character position POSITION.\nIf POSITION is out of range, the value is nil.",
       (position: Lisp_Object),
       {
  let pos: EMACS_INT = fix_position (position);
  if !((BEG as EMACS_INT) <= pos && pos <= (Z! () as EMACS_INT))
    { return Qnil (); }
  return make_fixnum (CHAR_TO_BYTE (pos));
});

DEFUN! ("byte-to-position", Fbyte_to_position, Sbyte_to_position, 1, 1, 0,
       doc: "Return the character position for byte position BYTEPOS.\nIf BYTEPOS is out of range, the value is nil.",
       (bytepos: Lisp_Object),
       {
  let mut pos_byte: ptrdiff_t = 0;

  CHECK_FIXNUM (bytepos);
  pos_byte = XFIXNUM (bytepos);
  if pos_byte < (BEG_BYTE as ptrdiff_t) || pos_byte > Z_BYTE! ()
    { return Qnil (); }
  if Z! () != Z_BYTE! ()
    /* There are multibyte characters in the buffer.
       The argument of BYTE_TO_CHAR must be a byte position at
       a character boundary, so search for the start of the current
       character.  */
    { while !CHAR_HEAD_P (FETCH_BYTE! (pos_byte) as c_int)
        { pos_byte -= 1; } }
  return make_fixnum (BYTE_TO_CHAR (pos_byte));
});

DEFUN! ("following-char", Ffollowing_char, Sfollowing_char, 0, 0, 0,
       doc: "Return the character following point, as a number.\nAt the end of the buffer or accessible region, return 0.",
       (),
       {
  let mut temp: Lisp_Object = Qnil ();
  if PT! () >= ZV! ()
    { XSETFASTINT! (temp, 0); }
  else
    { XSETFASTINT! (temp, FETCH_CHAR (PT_BYTE! ()) as EMACS_INT); }
  return temp;
});

DEFUN! ("preceding-char", Fprevious_char, Sprevious_char, 0, 0, 0,
       doc: "Return the character preceding point, as a number.\nAt the beginning of the buffer or accessible region, return 0.",
       (),
       {
  let mut temp: Lisp_Object = Qnil ();
  if PT! () <= BEGV! ()
    { XSETFASTINT! (temp, 0); }
  else if !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    {
      let mut pos: ptrdiff_t = PT_BYTE! ();
      pos -= prev_char_len (pos) as ptrdiff_t;
      XSETFASTINT! (temp, FETCH_CHAR (pos) as EMACS_INT);
    }
  else
    { XSETFASTINT! (temp, FETCH_BYTE! (PT_BYTE! () - 1) as EMACS_INT); }
  return temp;
});

DEFUN! ("bobp", Fbobp, Sbobp, 0, 0, 0,
       doc: "Return t if point is at the beginning of the buffer.\nIf the buffer is narrowed, this means the beginning of the narrowed part.",
       (),
       {
  if PT! () == BEGV! ()
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("eobp", Feobp, Seobp, 0, 0, 0,
       doc: "Return t if point is at the end of the buffer.\nIf the buffer is narrowed, this means the end of the narrowed part.",
       (),
       {
  if PT! () == ZV! ()
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("bolp", Fbolp, Sbolp, 0, 0, 0,
       doc: "Return t if point is at the beginning of a line.",
       (),
       {
  if PT! () == BEGV! () || FETCH_BYTE! (PT_BYTE! () - 1) == b'\n'
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("eolp", Feolp, Seolp, 0, 0, 0,
       doc: "Return t if point is at the end of a line.\n`End of a line' includes point being at the end of the buffer.",
       (),
       {
  if PT! () == ZV! () || FETCH_BYTE! (PT_BYTE! ()) == b'\n'
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("char-after", Fchar_after, Schar_after, 0, 1, 0,
       doc: "Return character in current buffer at position POS.\nPOS is an integer or a marker and defaults to point.\nIf POS is out of range, the value is nil.",
       (pos: Lisp_Object),
       {
  let mut pos_byte: ptrdiff_t = 0;

  if NILP (pos)
    {
      pos_byte = PT_BYTE! ();
      if pos_byte < BEGV_BYTE! () || pos_byte >= ZV_BYTE! ()
        { return Qnil (); }
    }
  else if MARKERP (pos)
    {
      pos_byte = marker_byte_position (pos);
      if pos_byte < BEGV_BYTE! () || pos_byte >= ZV_BYTE! ()
        { return Qnil (); }
    }
  else
    {
      let p: EMACS_INT = fix_position (pos);
      if !((BEGV! () as EMACS_INT) <= p && p < (ZV! () as EMACS_INT))
        { return Qnil (); }

      pos_byte = CHAR_TO_BYTE (p);
    }

  return make_fixnum (FETCH_CHAR (pos_byte) as EMACS_INT);
});

DEFUN! ("char-before", Fchar_before, Schar_before, 0, 1, 0,
       doc: "Return character in current buffer preceding position POS.\nPOS is an integer or a marker and defaults to point.\nIf POS is out of range, the value is nil.",
       (pos: Lisp_Object),
       {
  let mut pos: Lisp_Object = pos;
  let mut val: Lisp_Object = Qnil ();
  let mut pos_byte: ptrdiff_t = 0;

  if NILP (pos)
    {
      pos_byte = PT_BYTE! ();
      XSETFASTINT! (pos, PT! ());
    }

  if MARKERP (pos)
    {
      pos_byte = marker_byte_position (pos);

      if pos_byte <= BEGV_BYTE! () || pos_byte > ZV_BYTE! ()
        { return Qnil (); }
    }
  else
    {
      let p: EMACS_INT = fix_position (pos);

      if !((BEGV! () as EMACS_INT) < p && p <= (ZV! () as EMACS_INT))
        { return Qnil (); }

      pos_byte = CHAR_TO_BYTE (p);
    }

  if !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    {
      pos_byte -= prev_char_len (pos_byte) as ptrdiff_t;
      XSETFASTINT! (val, FETCH_CHAR (pos_byte) as EMACS_INT);
    }
  else
    {
      pos_byte -= 1;
      XSETFASTINT! (val, FETCH_BYTE! (pos_byte) as EMACS_INT);
    }
  return val;
});

DEFUN! ("user-login-name", Fuser_login_name, Suser_login_name, 0, 1, 0,
       doc: "Return the name under which the user logged in, as a string.\nThis is based on the effective uid, not the real uid.\nAlso, if the environment variables LOGNAME or USER are set,\nthat determines the value of this function.\n\nIf optional argument UID is an integer, return the login name\nof the user with that uid, or nil if there is no such user.",
       (uid: Lisp_Object),
       {
  let mut pw: *mut passwd = std::ptr::null_mut ();
  let mut id: uid_t = 0;

  /* Set up the user name info if we didn't do it before.
     (That can happen if Emacs is dumpable
     but you decide to run `temacs -l loadup' and not dump.  */
  if NILP (Vuser_login_name)
    { init_editfns (); }

  if NILP (uid)
    { return Vuser_login_name; }

  let mut id_signed: intmax_t;
  CONS_TO_INTEGER! (uid, id_signed);
  id = id_signed as uid_t;
  block_input ();
  pw = getpwuid (id);
  unblock_input ();
  return if pw.is_null () { Qnil () } else { build_string ((*pw).pw_name) };
});

DEFUN! ("user-real-login-name", Fuser_real_login_name, Suser_real_login_name,
       0, 0, 0,
       doc: "Return the name of the user's real uid, as a string.\nThis ignores the environment variables LOGNAME and USER, so it differs from\n`user-login-name' when running under `su'.",
       (),
       {
  /* Set up the user name info if we didn't do it before.
     (That can happen if Emacs is dumpable
     but you decide to run `temacs -l loadup' and not dump.  */
  if NILP (Vuser_login_name)
    { init_editfns (); }
  return Vuser_real_login_name;
});

DEFUN! ("user-uid", Fuser_uid, Suser_uid, 0, 0, 0,
       doc: "Return the effective uid of Emacs, as an integer.",
       (),
       {
  let euid: uid_t = geteuid ();
  return INT_TO_INTEGER (euid as EMACS_INT);
});

DEFUN! ("user-real-uid", Fuser_real_uid, Suser_real_uid, 0, 0, 0,
       doc: "Return the real uid of Emacs, as an integer.",
       (),
       {
  let uid: uid_t = getuid ();
  return INT_TO_INTEGER (uid as EMACS_INT);
});

DEFUN! ("group-name", Fgroup_name, Sgroup_name, 1, 1, 0,
       doc: "Return the name of the group whose numeric group ID is GID.\nThe argument GID should be an integer or a float.\nReturn nil if a group with such GID does not exists or is not known.",
       (gid: Lisp_Object),
       {
  let mut gr: *mut group = std::ptr::null_mut ();
  let mut id: gid_t = 0;

  if !NUMBERP (gid) && !CONSP (gid)
    { error (c"Invalid GID specification".as_ptr ()); }
  let mut id_signed: intmax_t;
  CONS_TO_INTEGER! (gid, id_signed);
  id = id_signed as gid_t;
  block_input ();
  gr = getgrgid (id);
  unblock_input ();
  return if gr.is_null () { Qnil () } else { build_string ((*gr).gr_name) };
});

DEFUN! ("group-gid", Fgroup_gid, Sgroup_gid, 0, 0, 0,
       doc: "Return the effective gid of Emacs, as an integer.",
       (),
       {
  let egid: gid_t = getegid ();
  return INT_TO_INTEGER (egid as EMACS_INT);
});

DEFUN! ("group-real-gid", Fgroup_real_gid, Sgroup_real_gid, 0, 0, 0,
       doc: "Return the real gid of Emacs, as an integer.",
       (),
       {
  let gid: gid_t = getgid ();
  return INT_TO_INTEGER (gid as EMACS_INT);
});

DEFUN! ("user-full-name", Fuser_full_name, Suser_full_name, 0, 1, 0,
       doc: "Return the full name of the user logged in, as a string.\nIf the full name corresponding to Emacs's userid is not known,\nreturn \"unknown\".\n\nIf optional argument UID is an integer, return the full name\nof the user with that uid, or nil if there is no such user.\nIf UID is a string, return the full name of the user with that login\nname, or nil if there is no such user.\n\nIf the full name includes commas, remove everything starting with\nthe first comma, because the \\='gecos\\=' field of the \\='/etc/passwd\\=' file\nis in general a comma-separated list.",
       (uid: Lisp_Object),
       {
  let mut pw: *mut passwd = std::ptr::null_mut ();
  let mut p: *mut c_char = std::ptr::null_mut ();
  let mut q: *mut c_char = std::ptr::null_mut ();
  let mut full: Lisp_Object = Qnil ();

  if NILP (uid)
    { return Vuser_full_name; }
  else if NUMBERP (uid)
    {
      let mut u: uid_t;
      let mut u_signed: intmax_t;
      CONS_TO_INTEGER! (uid, u_signed);
      u = u_signed as uid_t;
      block_input ();
      pw = getpwuid (u);
      unblock_input ();
    }
  else if STRINGP (uid)
    {
      block_input ();
      pw = getpwnam (SSDATA (uid));
      unblock_input ();
    }
  else
    { error (c"Invalid UID specification".as_ptr ()); }

  if pw.is_null ()
    { return Qnil (); }

  p = USER_FULL_NAME! (pw);
  if p.is_null ()
    { return Qnil (); }

  /* Chop off everything after the first comma, since 'pw_gecos' is a
     comma-separated list. */
  q = strchr (p, ',' as c_int);
  full = make_string (p, if q.is_null () { strlen (p) as ptrdiff_t } else { q.offset_from (p) });

  /* #ifdef AMPERSAND_FULL_NAME: 1 in the frozen configuration (macros.tsv).  */
  p = SSDATA (full);
  q = strchr (p, '&' as c_int);
  /* Substitute the login name for the &, upcasing the first character.  */
  if !q.is_null ()
    {
      let login: Lisp_Object = Fuser_login_name (INT_TO_INTEGER ((*pw).pw_uid as EMACS_INT));
      if !NILP (login)
        {
          USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
          let r: *mut c_char = SAFE_ALLOCA! (sa_avail,
                                             strlen (p) + SBYTES (login) as usize + 1) as *mut c_char;
          memcpy (r as *mut c_void, p as *const c_void, q.offset_from (p) as usize);
          let s: *mut c_char = lispstpcpy (r.add (q.offset_from (p) as usize), login);
          *r.add (q.offset_from (p) as usize)
            = upcase (*r.add (q.offset_from (p) as usize) as c_int) as c_char;
          strcpy (s, q.add (1));
          full = build_string (r);
          SAFE_FREE! (sa_count, sa_mark);
        }
    }

  return full;
});

DEFUN! ("system-name", Fsystem_name, Ssystem_name, 0, 0, 0,
       doc: "Return the host name of the machine you are running on, as a string.",
       (),
       {
  if EQ (Vsystem_name, cached_system_name)
    { init_and_cache_system_name (); }
  return Vsystem_name;
});

DEFUN! ("emacs-pid", Femacs_pid, Semacs_pid, 0, 0, 0,
       doc: "Return the process ID of Emacs, as an integer.",
       (),
       {
  let pid: pid_t = getpid ();
  return INT_TO_INTEGER (pid as EMACS_INT);
});

/* Insert NARGS Lisp objects in the array ARGS by calling INSERT_FUNC
   (if a type of object is Lisp_Int) or INSERT_FROM_STRING_FUNC (if a
   type of object is Lisp_String).  INHERIT is passed to
   INSERT_FROM_STRING_FUNC as the last argument.  */

unsafe fn general_insert_function (insert_func: unsafe extern "C" fn (*const c_char, ptrdiff_t),
                                   insert_from_string_func: unsafe extern "C" fn (Lisp_Object, ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t, bool),
                                   inherit: bool, nargs: ptrdiff_t, args: *mut Lisp_Object)
{
  let mut argnum: ptrdiff_t;
  let mut val: Lisp_Object;

  argnum = 0;
  while argnum < nargs
    {
      val = *args.offset (argnum as isize);
      if CHARACTERP (val)
        {
          let c: c_int = XFIXNAT (val) as c_int;
          let mut str_: [u8; MAX_MULTIBYTE_LENGTH as usize] = std::mem::zeroed ();
          let mut len: c_int;

          if !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
            { len = CHAR_STRING (c, str_.as_mut_ptr ()); }
          else
            {
              str_[0] = CHAR_TO_BYTE8 (c) as u8;
              len = 1;
            }
          insert_func (str_.as_ptr () as *const c_char, len as ptrdiff_t);
        }
      else if STRINGP (val)
        {
          insert_from_string_func (val, 0, 0,
                                   SCHARS (val),
                                   SBYTES (val),
                                   inherit);
        }
      else
        { wrong_type_argument (Qchar_or_string_p (), val); }
      argnum += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert1 (arg: Lisp_Object)
{
  let mut arg: Lisp_Object = arg;
  Finsert (1, &mut arg);
}

DEFUN! ("insert", Finsert, Sinsert, 0, MANY, 0,
       doc: "Insert the arguments, either strings or characters, at point.\nPoint and after-insertion markers move forward to end up\n after the inserted text.\nAny other markers at the point of insertion remain before the text.\n\nIf the current buffer is multibyte, unibyte strings are converted\nto multibyte for insertion (see `string-make-multibyte').\nIf the current buffer is unibyte, multibyte strings are converted\nto unibyte for insertion (see `string-make-unibyte').\n\nWhen operating on binary data, it may be necessary to preserve the\noriginal bytes of a unibyte string when inserting it into a multibyte\nbuffer; to accomplish this, apply `decode-coding-string' with the\n`no-conversion' coding system to the string and insert the result.\n\nusage: (insert &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  general_insert_function (insert, insert_from_string, false, nargs, args);
  return Qnil ();
});

DEFUN! ("insert-and-inherit", Finsert_and_inherit, Sinsert_and_inherit,
       0, MANY, 0,
       doc: "Insert the arguments at point, inheriting properties from adjoining text.\nPoint and after-insertion markers move forward to end up\n after the inserted text.\nAny other markers at the point of insertion remain before the text.\n\nIf the current buffer is multibyte, unibyte strings are converted\nto multibyte for insertion (see `unibyte-char-to-multibyte').\nIf the current buffer is unibyte, multibyte strings are converted\nto unibyte for insertion.\n\nusage: (insert-and-inherit &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  general_insert_function (insert_and_inherit, insert_from_string, true,
                           nargs, args);
  return Qnil ();
});

DEFUN! ("insert-before-markers", Finsert_before_markers, Sinsert_before_markers, 0, MANY, 0,
       doc: "Insert strings or characters at point, relocating markers after the text.\nPoint and all markers at or after the insertion point move forward to\nend up after the inserted text.  This is unlike with `insert' and other\ninsertion functions, where before-insertion markers remain before the\ninserted text.\n\nIf the current buffer is multibyte, unibyte strings are converted\nto multibyte for insertion (see `unibyte-char-to-multibyte').\nIf the current buffer is unibyte, multibyte strings are converted\nto unibyte for insertion.\n\nIf an overlay begins at the insertion point, the inserted text falls\noutside the overlay; if a nonempty overlay ends at the insertion\npoint, the inserted text falls inside that overlay.\n\nusage: (insert-before-markers &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  general_insert_function (insert_before_markers,
                           insert_from_string_before_markers, false,
                           nargs, args);
  return Qnil ();
});

DEFUN! ("insert-before-markers-and-inherit", Finsert_and_inherit_before_markers,
       Sinsert_and_inherit_before_markers, 0, MANY, 0,
       doc: "Insert text at point, relocating markers and inheriting properties.\nPoint and all markers at or after the insertion point move forward to\nend up after the inserted text.  This is unlike with `insert' and other\ninsertion functions, where before-insertion markers remain before the\ninserted text.\n\nIf the current buffer is multibyte, unibyte strings are converted\nto multibyte for insertion (see `unibyte-char-to-multibyte').\nIf the current buffer is unibyte, multibyte strings are converted\nto unibyte for insertion.\n\nusage: (insert-before-markers-and-inherit &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  general_insert_function (insert_before_markers_and_inherit,
                           insert_from_string_before_markers, true,
                           nargs, args);
  return Qnil ();
});

DEFUN! ("insert-char", Finsert_char, Sinsert_char, 1, 3,
       "(list (read-char-by-name \"Insert character (Unicode name or hex): \")              (prefix-numeric-value current-prefix-arg)              t))",
       doc: "Insert COUNT copies of CHARACTER.\n\nInteractively, prompt for CHARACTER using `read-char-by-name'.\nYou can specify CHARACTER at the prompt in one of these ways:\n\n - As its Unicode character name, e.g. \"LATIN SMALL LETTER A\".\n   Completion is available; if you type a substring of the name\n   preceded by an asterisk `*', Emacs shows all names which include\n   that substring, not necessarily at the beginning of the name.\n\n - As a hexadecimal code point, e.g. 263A.  Note that code points in\n   Emacs are equivalent to Unicode up to 10FFFF (which is the limit of\n   the Unicode code space).\n\n - As a code point with a radix specified with #, e.g. #o21430\n   (octal), #x2318 (hex), or #10r8984 (decimal).\n\nWhen called from Lisp, CHARACTER should be an integer whose value\nis valid for a character; see `characterp'.  To specify a character by\nits Unicode name in calls from Lisp, use `char-from-name'.\n\nWhen called interactively, COUNT is the prefix argument.  If omitted or\nnil, it defaults to 1.\n\nInserting the character(s) relocates point and markers in the same ways\nas the function `insert'.\n\nThe optional third argument INHERIT, if non-nil, says to inherit text\nproperties from adjoining text, if those properties are sticky.  When\ncalled interactively, INHERIT is t.",
       (character: Lisp_Object, count: Lisp_Object, inherit: Lisp_Object),
       {
  let mut count: Lisp_Object = count;
  let mut i: c_int;
  let mut stringlen: c_int;
  let mut n: ptrdiff_t;
  let mut c: c_int;
  let mut len: c_int;
  let mut str_: [u8; MAX_MULTIBYTE_LENGTH as usize] = std::mem::zeroed ();
  let mut string: [c_char; 4000] = std::mem::zeroed ();

  CHECK_CHARACTER (character);
  if NILP (count)
    { XSETFASTINT! (count, 1); }
  else
    { CHECK_FIXNUM (count); }
  c = XFIXNAT (character) as c_int;

  if !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { len = CHAR_STRING (c, str_.as_mut_ptr ()); }
  else
    { str_[0] = c as u8; len = 1; }
  if XFIXNUM (count) <= 0
    { return Qnil (); }
  if (BUF_BYTES_MAX / len as ptrdiff_t) < XFIXNUM (count)
    { buffer_overflow (); }
  n = XFIXNUM (count) * len as ptrdiff_t;
  stringlen = min (n, (std::mem::size_of::<[c_char; 4000]> ()
                       - std::mem::size_of::<[c_char; 4000]> () % len as usize) as ptrdiff_t) as c_int;
  i = 0;
  while i < stringlen
    { string[i as usize] = str_[(i % len) as usize] as c_char; i += 1; }
  while n > stringlen as ptrdiff_t
    {
      maybe_quit ();
      if !NILP (inherit)
        { insert_and_inherit (string.as_ptr (), stringlen as ptrdiff_t); }
      else
        { insert (string.as_ptr (), stringlen as ptrdiff_t); }
      n -= stringlen as ptrdiff_t;
    }
  if !NILP (inherit)
    { insert_and_inherit (string.as_ptr (), n); }
  else
    { insert (string.as_ptr (), n); }
  return Qnil ();
});

DEFUN! ("insert-byte", Finsert_byte, Sinsert_byte, 2, 3, 0,
       doc: "Insert COUNT (second arg) copies of BYTE (first arg).\nBoth arguments are required.\nBYTE is a number of the range 0..255.\n\nIf BYTE is 128..255 and the current buffer is multibyte, the\ncorresponding eight-bit character is inserted.\n\nPoint and markers are relocated as in the function `insert'.\n\nThe optional third arg INHERIT, if non-nil, says to inherit text properties\nfrom adjoining text, if those properties are sticky.",
       (byte: Lisp_Object, count: Lisp_Object, inherit: Lisp_Object),
       {
  let mut byte: Lisp_Object = byte;
  CHECK_FIXNUM (byte);
  if XFIXNUM (byte) < 0 || XFIXNUM (byte) > 255
    { args_out_of_range_3 (byte, make_fixnum (0), make_fixnum (255)); }
  if XFIXNUM (byte) >= 128
      && !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { XSETFASTINT! (byte, BYTE8_TO_CHAR (XFIXNUM (byte) as c_int) as EMACS_INT); }
  return Finsert_char (byte, count, inherit);
});

/* Making strings from buffer contents.  */

/* Return a Lisp_String containing the text of the current buffer from
   START to END.  If text properties are in use and the current buffer
   has properties in the range specified, the resulting string will also
   have them, if PROPS is true.

   We don't want to use plain old make_string here, because it calls
   make_uninit_string, which can cause the buffer arena to be
   compacted.  make_string has no way of knowing that the data has
   been moved, and thus copies the wrong data into the string.  This
   doesn't affect most of the other users of make_string, so it should
   be left as is.  But we should use this function when conjuring
   buffer substrings.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_buffer_string (start: ptrdiff_t, end: ptrdiff_t,
                                             props: bool) -> Lisp_Object
{
  let start_byte: ptrdiff_t = CHAR_TO_BYTE (start);
  let end_byte: ptrdiff_t = CHAR_TO_BYTE (end);

  return make_buffer_string_both (start, start_byte, end, end_byte, props);
}

/* Return a Lisp_String containing the text of the current buffer from
   START / START_BYTE to END / END_BYTE.

   If text properties are in use and the current buffer
   has properties in the range specified, the resulting string will also
   have them, if PROPS is true.

   We don't want to use plain old make_string here, because it calls
   make_uninit_string, which can cause the buffer arena to be
   compacted.  make_string has no way of knowing that the data has
   been moved, and thus copies the wrong data into the string.  This
   doesn't effect most of the other users of make_string, so it should
   be left as is.  But we should use this function when conjuring
   buffer substrings.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_buffer_string_both (start: ptrdiff_t, start_byte: ptrdiff_t,
                                                  end: ptrdiff_t, end_byte: ptrdiff_t,
                                                  props: bool) -> Lisp_Object
{
  let mut result: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut tem1: Lisp_Object;
  let mut beg0: ptrdiff_t;
  let mut end0: ptrdiff_t;
  let mut beg1: ptrdiff_t;
  let mut end1: ptrdiff_t;
  let mut size: ptrdiff_t;

  if start_byte < GPT_BYTE! () && GPT_BYTE! () < end_byte
    {
      /* Two regions, before and after the gap.  */
      beg0 = start_byte;
      end0 = GPT_BYTE! ();
      beg1 = GPT_BYTE! () + GAP_SIZE! () - BEG_BYTE as ptrdiff_t;
      end1 = end_byte + GAP_SIZE! () - BEG_BYTE as ptrdiff_t;
    }
  else
    {
      /* The only region.  */
      beg0 = start_byte;
      end0 = end_byte;
      beg1 = -1;
      end1 = -1;
    }

  if !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { result = make_uninit_multibyte_string (end - start, end_byte - start_byte); }
  else
    { result = make_uninit_string (end - start); }

  size = end0 - beg0;
  memcpy (SDATA (result) as *mut c_void, BYTE_POS_ADDR (beg0) as *const c_void,
          size as usize);
  if beg1 != -1
    { memcpy ((SDATA (result) as *mut c_char).add (size as usize) as *mut c_void,
              (BEG_ADDR! () as *mut c_char).add (beg1 as usize) as *const c_void,
              (end1 - beg1) as usize); }

  /* If desired, update and copy the text properties.  */
  if props
    {
      update_buffer_properties (start, end);

      tem = Fnext_property_change (make_fixnum (start), Qnil (), make_fixnum (end));
      tem1 = Ftext_properties_at (make_fixnum (start), Qnil ());

      if XFIXNUM (tem) != end || !NILP (tem1)
        { copy_intervals_to_string (result, (current_buffer! ()), start, end - start); }
    }

  return result;
}

/* Call Vbuffer_access_fontify_functions for the range START ... END
   in the current buffer, if necessary.  */

unsafe fn update_buffer_properties (start: ptrdiff_t, end: ptrdiff_t)
{
  /* If this buffer has some access functions,
     call them, specifying the range of the buffer being accessed.  */
  if !NILP (Vbuffer_access_fontify_functions)
    {
      /* But don't call them if we can tell that the work
         has already been done.  */
      if !NILP (Vbuffer_access_fontified_property)
        {
          let tem: Lisp_Object
            = Ftext_property_any (make_fixnum (start), make_fixnum (end),
                                  Vbuffer_access_fontified_property,
                                  Qnil (), Qnil ());
          if NILP (tem)
            { return; }
        }

      CALLN! (Frun_hook_with_args, Qbuffer_access_fontify_functions (),
              make_fixnum (start), make_fixnum (end));
    }
}

DEFUN! ("buffer-substring", Fbuffer_substring, Sbuffer_substring, 2, 2, 0,
       doc: "Return the contents of part of the current buffer as a string.\nThe two arguments START and END are character positions;\nthey can be in either order.\nThe string returned is multibyte if the buffer is multibyte.\n\nThis function copies the text properties of that part of the buffer\ninto the result string; if you don't want the text properties,\nuse `buffer-substring-no-properties' instead.",
       (start: Lisp_Object, end: Lisp_Object),
       {
  let mut start: Lisp_Object = start;
  let mut end: Lisp_Object = end;
  let mut b: ptrdiff_t;
  let mut e: ptrdiff_t;

  validate_region (&mut start, &mut end);
  b = XFIXNUM (start);
  e = XFIXNUM (end);

  return make_buffer_string (b, e, true);
});

DEFUN! ("buffer-substring-no-properties", Fbuffer_substring_no_properties,
       Sbuffer_substring_no_properties, 2, 2, 0,
       doc: "Return the characters of part of the buffer, without the text properties.\nThe two arguments START and END are character positions;\nthey can be in either order.",
       (start: Lisp_Object, end: Lisp_Object),
       {
  let mut start: Lisp_Object = start;
  let mut end: Lisp_Object = end;
  let mut b: ptrdiff_t;
  let mut e: ptrdiff_t;

  validate_region (&mut start, &mut end);
  b = XFIXNUM (start);
  e = XFIXNUM (end);

  return make_buffer_string (b, e, false);
});

DEFUN! ("buffer-string", Fbuffer_string, Sbuffer_string, 0, 0, 0,
       doc: "Return the contents of the current buffer as a string.\nIf narrowing is in effect, this function returns only the visible part\nof the buffer.\n\nThis function copies the text properties of that part of the buffer\ninto the result string; if you don't want the text properties,\nuse `buffer-substring-no-properties' instead.",
       (),
       {
  return make_buffer_string_both (BEGV! (), BEGV_BYTE! (), ZV! (), ZV_BYTE! (), true);
});

DEFUN! ("insert-buffer-substring", Finsert_buffer_substring, Sinsert_buffer_substring,
       1, 3, 0,
       doc: "Insert before point a substring of the contents of BUFFER.\nBUFFER may be a buffer or a buffer name.\nArguments START and END are character positions specifying the substring.\nThey default to the values of (point-min) and (point-max) in BUFFER.\n\nPoint and markers are relocated as in the function `insert'.\n\nIf the current buffer is multibyte and BUFFER is unibyte, or vice\nversa, strings are converted from unibyte to multibyte or vice versa\nusing `string-make-multibyte' or `string-make-unibyte', which see.",
       (buffer: Lisp_Object, start: Lisp_Object, end: Lisp_Object),
       {
  let mut b: EMACS_INT;
  let mut e: EMACS_INT;
  let mut temp: EMACS_INT;
  let mut bp: *mut buffer;
  let mut obuf: *mut buffer;
  let mut buf: Lisp_Object;

  buf = Fget_buffer (buffer);
  if NILP (buf)
    { nsberror (buffer); }
  bp = XBUFFER (buf);
  if !BUFFER_LIVE_P (bp)
    { error (c"Selecting deleted buffer".as_ptr ()); }

  b = if !NILP (start) { fix_position (start) } else { BUF_BEGV (bp) };
  e = if !NILP (end) { fix_position (end) } else { BUF_ZV (bp) };
  if b > e
    { temp = b; b = e; e = temp; }

  if !(BUF_BEGV (bp) <= b && e <= BUF_ZV (bp))
    { args_out_of_range (start, end); }

  obuf = (current_buffer! ());
  set_buffer_internal_1 (bp);
  update_buffer_properties (b, e);
  set_buffer_internal_1 (obuf);

  insert_from_buffer (bp, b, e - b, false);
  return Qnil ();
});

DEFUN! ("compare-buffer-substrings", Fcompare_buffer_substrings, Scompare_buffer_substrings,
       6, 6, 0,
       doc: "Compare two substrings of two buffers; return result as number.\nReturn -N if first string is less after N-1 chars, +N if first string is\ngreater after N-1 chars, or 0 if strings match.\nThe first substring is in BUFFER1 from START1 to END1 and the second\nis in BUFFER2 from START2 to END2.\nAll arguments may be nil.  If BUFFER1 or BUFFER2 is nil, the current\nbuffer is used.  If START1 or START2 is nil, the value of `point-min'\nin the respective buffers is used.  If END1 or END2 is nil, the value\nof `point-max' in the respective buffers is used.\nThe value of `case-fold-search' in the current buffer\ndetermines whether case is significant or ignored.",
       (buffer1: Lisp_Object, start1: Lisp_Object, end1: Lisp_Object, buffer2: Lisp_Object,
        start2: Lisp_Object, end2: Lisp_Object),
       {
  let mut begp1: EMACS_INT;
  let mut endp1: EMACS_INT;
  let mut begp2: EMACS_INT;
  let mut endp2: EMACS_INT;
  let mut temp: EMACS_INT;
  let mut bp1: *mut buffer;
  let mut bp2: *mut buffer;
  let trt: Lisp_Object
    = if !NILP (Vcase_fold_search)
      { BVAR! ((current_buffer! ()), case_canon_table_) } else { Qnil () };
  let mut chars: ptrdiff_t = 0;
  let mut i1: ptrdiff_t;
  let mut i2: ptrdiff_t;
  let mut i1_byte: ptrdiff_t;
  let mut i2_byte: ptrdiff_t;

  /* Find the first buffer and its substring.  */

  if NILP (buffer1)
    { bp1 = (current_buffer! ()); }
  else
    {
      let buf1: Lisp_Object = Fget_buffer (buffer1);
      if NILP (buf1)
        { nsberror (buffer1); }
      bp1 = XBUFFER (buf1);
      if !BUFFER_LIVE_P (bp1)
        { error (c"Selecting deleted buffer".as_ptr ()); }
    }

  begp1 = if !NILP (start1) { fix_position (start1) } else { BUF_BEGV (bp1) };
  endp1 = if !NILP (end1) { fix_position (end1) } else { BUF_ZV (bp1) };
  if begp1 > endp1
    { temp = begp1; begp1 = endp1; endp1 = temp; }

  if !(BUF_BEGV (bp1) <= begp1
       && begp1 <= endp1
       && endp1 <= BUF_ZV (bp1))
    { args_out_of_range (start1, end1); }

  /* Likewise for second substring.  */

  if NILP (buffer2)
    { bp2 = (current_buffer! ()); }
  else
    {
      let buf2: Lisp_Object = Fget_buffer (buffer2);
      if NILP (buf2)
        { nsberror (buffer2); }
      bp2 = XBUFFER (buf2);
      if !BUFFER_LIVE_P (bp2)
        { error (c"Selecting deleted buffer".as_ptr ()); }
    }

  begp2 = if !NILP (start2) { fix_position (start2) } else { BUF_BEGV (bp2) };
  endp2 = if !NILP (end2) { fix_position (end2) } else { BUF_ZV (bp2) };
  if begp2 > endp2
    { temp = begp2; begp2 = endp2; endp2 = temp; }

  if !(BUF_BEGV (bp2) <= begp2
       && begp2 <= endp2
       && endp2 <= BUF_ZV (bp2))
    { args_out_of_range (start2, end2); }

  i1 = begp1;
  i2 = begp2;
  i1_byte = buf_charpos_to_bytepos (bp1 as *mut c_void, i1);
  i2_byte = buf_charpos_to_bytepos (bp2 as *mut c_void, i2);

  while i1 < endp1 && i2 < endp2
    {
      /* When we find a mismatch, we must compare the
         characters, not just the bytes.  */
      let mut c1: c_int;
      let mut c2: c_int;

      if !NILP (BVAR! (bp1, enable_multibyte_characters_))
        {
          c1 = BUF_FETCH_MULTIBYTE_CHAR (bp1, i1_byte);
          i1_byte += buf_next_char_len (bp1, i1_byte) as ptrdiff_t;
          i1 += 1;
        }
      else
        {
          c1 = make_char_multibyte (BUF_FETCH_BYTE (bp1, i1) as c_int);
          i1 += 1;
        }

      if !NILP (BVAR! (bp2, enable_multibyte_characters_))
        {
          c2 = BUF_FETCH_MULTIBYTE_CHAR (bp2, i2_byte);
          i2_byte += buf_next_char_len (bp2, i2_byte) as ptrdiff_t;
          i2 += 1;
        }
      else
        {
          c2 = make_char_multibyte (BUF_FETCH_BYTE (bp2, i2) as c_int);
          i2 += 1;
        }

      if !NILP (trt)
        {
          c1 = char_table_translate (trt, c1);
          c2 = char_table_translate (trt, c2);
        }

      if c1 != c2
        { return make_fixnum (if c1 < c2 { -1 - chars } else { chars + 1 }); }

      chars += 1;
      rarely_quit (chars as u64);
    }

  /* The strings match as far as they go.
     If one is shorter, that one is less.  */
  if chars < endp1 - begp1
    { return make_fixnum (chars + 1); }
  else if chars < endp2 - begp2
    { return make_fixnum (-chars - 1); }

  /* Same length too => they are equal.  */
  return make_fixnum (0);
});

/* Set up necessary definitions for diffseq.h; see comments in
   diffseq.h for explanation.  */

/* C 1888-1889's `#undef ELEMENT` / `#undef EQUAL`: C's `ELEMENT` arm is not taken here, so
   diffseq.h's `XVECREF_YVECREF_EQUAL` arm is the live one (diffseq.h 202-206) and the port
   has no `ELEMENT`/`EQUAL` counterpart to undefine (the `ELEMENT` text is dead, P-08).  */

/// C 1890: `#define USE_HEURISTIC` — the flag diffseq.h 138's `#ifdef` tests.  The port's
/// `struct context` carries the `heuristic` field unconditionally (the frozen build defines
/// the flag); this marker is the macro's counterpart (the gnulib_config.rs `_GL_*_INLINE`
/// shape).
pub const USE_HEURISTIC: () = ();

/// C 1892-1893: `#define XVECREF_YVECREF_EQUAL(ctx, xoff, yoff) buffer_chars_equal (ctx,
/// xoff, yoff)`.
macro_rules! XVECREF_YVECREF_EQUAL {
    ($ctx:expr, $xoff:expr, $yoff:expr) => { buffer_chars_equal ($ctx, $xoff, $yoff) };
}

/// C 1895: `#define OFFSET ptrdiff_t` — diffseq.h uses the name in type position, which
/// Rust spells `OFFSET! ()`.
macro_rules! OFFSET { () => { ptrdiff_t }; }

/// C 1897-1908: the field list C splices into diffseq.h 117's `struct context`.  Rust cannot
/// splice fields into a struct from a macro, so `struct context` below spells the expansion
/// out; this macro keeps the C text and is never expanded (a `macro_rules!` body is a token
/// tree, so a field list is legal there and the `--check` body comparison can see it).
macro_rules! EXTRA_CONTEXT_FIELDS {
    () => {
        pub chars_a: *mut c_int,
        pub chars_b: *mut c_int,
        pub deletions: *mut c_uchar,
        pub insertions: *mut c_uchar,
        pub time_limit: timespec,
        pub jmp: sys_jmp_buf,
        pub quitcounter: c_ushort,
    };
}

/// C 1909: `#define NOTE_DELETE(ctx, xoff) set_bit ((ctx)->deletions, xoff)`.
macro_rules! NOTE_DELETE {
    ($ctx:expr, $xoff:expr) => { set_bit ((*$ctx).deletions, $xoff) };
}

/// C 1910: `#define NOTE_INSERT(ctx, yoff) set_bit ((ctx)->insertions, yoff)`.
macro_rules! NOTE_INSERT {
    ($ctx:expr, $yoff:expr) => { set_bit ((*$ctx).insertions, $yoff) };
}

/// C 1911: `#define EARLY_ABORT(ctx) compareseq_early_abort (ctx)`.
macro_rules! EARLY_ABORT {
    ($ctx:expr) => { compareseq_early_abort ($ctx) };
}

/* C: lisp.h 2273-2274's `#ifdef HAVE__SETJMP typedef jmp_buf sys_jmp_buf;` (HAVE__SETJMP is 1
   on the frozen target, macros.tsv).  The name is the `thread.rs` mirror's aligned wrapper
   struct (its layout note records why `jmp_buf`'s `[u8; 192]` cannot carry C's alignment),
   and the two call sites cast to `*mut jmp_buf` for `sys_setjmp`/`sys_longjmp`, exactly as
   eval.rs/bytecode.rs do.  */

/// C 1919: `#include "minmax.h"` — the mirror of the two-element header.
include!("../lib/include/minmax.rs");

/* C 1920: `#include "diffseq.h"`.  `emacs/lib/diffseq.h` has **no mirror** and this unit is
   its only includer in the tree (its `diag`/`compareseq` are `static`, and its `struct
   context` is parameterized by the includer's `EXTRA_CONTEXT_FIELDS`), so the port carries
   the header's own text here, at the point C includes it, with the two C macros that stay in
   this file's text (`USE_HEURISTIC`, `OFFSET_MAX`, `SNAKE_LIMIT`) spelled as C spells them.
   See `docs/ports/editfns.md` (mirror gap reported to the integrator).  */

/// diffseq.h 98-100: `#ifndef NOTE_ORDERED # define NOTE_ORDERED false #endif`.
pub const NOTE_ORDERED: bool = false;

/// diffseq.h 87-89: `#define OFFSET_MAX ((((OFFSET) 1u << (sizeof (OFFSET) * CHAR_BIT - 2))
/// - 1) * 2 + 1)`.
pub const OFFSET_MAX: ptrdiff_t =
  (((1u64 << (std::mem::size_of::<ptrdiff_t> () * CHAR_BIT as usize - 2)) - 1) * 2 + 1)
  as ptrdiff_t;

/// diffseq.h 152: `#define SNAKE_LIMIT 20`.
pub const SNAKE_LIMIT: ptrdiff_t = 20;

/* diffseq.h 117-150: `struct context` with this unit's `EXTRA_CONTEXT_FIELDS` spliced in
   (`ELEMENT` is undefined, so the header's `xvec`/`yvec` pair is absent, and
   `USE_HEURISTIC` is defined, so `heuristic` is present).  */
#[repr(C)]
pub struct context
{
  pub chars_a: *mut c_int,
  pub chars_b: *mut c_int,
  pub deletions: *mut c_uchar,
  pub insertions: *mut c_uchar,
  pub time_limit: timespec,
  pub jmp: sys_jmp_buf,
  pub quitcounter: c_ushort,

  pub fdiag: *mut OFFSET! (),
  pub bdiag: *mut OFFSET! (),
  /// diffseq.h 136-143's `#ifdef USE_HEURISTIC bool heuristic;`.
  pub heuristic: bool,
  pub too_expensive: OFFSET! (),
}

/* diffseq.h 152-163: `struct partition`.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct partition
{
  /* Midpoints of this partition.  */
  pub xmid: OFFSET! (),
  pub ymid: OFFSET! (),

  /* True if low half will be analyzed minimally.  */
  pub lo_minimal: bool,

  /* Likewise for high half.  */
  pub hi_minimal: bool,
}

/* diffseq.h 170-447: `diag`.  The header's local `#define XREF_YREF_EQUAL(x,y)` is
   `XVECREF_YVECREF_EQUAL (ctxt, x, y)` here (the non-`ELEMENT` arm); it is expanded at each
   of its use sites, exactly as C's macro is (a Rust `macro_rules!` cannot name the caller's
   `ctxt`, PORTING.md P-57).  */

/* Find the midpoint of the shortest edit script for a specified portion
   of the two vectors.  (The reference's full explanation is diffseq.h 183-192.)  */
unsafe fn diag (xoff: OFFSET! (), xlim: OFFSET! (), yoff: OFFSET! (), ylim: OFFSET! (),
                find_minimal: bool, part: *mut partition, ctxt: *mut context)
{
  let fd: *const OFFSET! () = (*ctxt).fdiag;       /* Give the compiler a chance. */
  let bd: *const OFFSET! () = (*ctxt).bdiag;       /* Additional help for the compiler. */
  let dmin: OFFSET! () = xoff - ylim;              /* Minimum valid diagonal. */
  let dmax: OFFSET! () = xlim - yoff;              /* Maximum valid diagonal. */
  let fmid: OFFSET! () = xoff - yoff;              /* Center diagonal of top-down search. */
  let bmid: OFFSET! () = xlim - ylim;              /* Center diagonal of bottom-up search. */
  let mut fmin: OFFSET! () = fmid;
  let mut fmax: OFFSET! () = fmid;           /* Limits of top-down search. */
  let mut bmin: OFFSET! () = bmid;
  let mut bmax: OFFSET! () = bmid;           /* Limits of bottom-up search. */
  let mut c: OFFSET! ();                     /* Cost. */
  let odd: bool = ((fmid - bmid) & 1) != 0;  /* True if southeast corner is on an odd
                                                diagonal with respect to the northwest. */

  *(fd as *mut OFFSET! ()).offset (fmid as isize) = xoff;
  *(bd as *mut OFFSET! ()).offset (bmid as isize) = xlim;

  c = 1;
  loop
    {
      let mut d: OFFSET! ();                 /* Active diagonal. */
      let mut big_snake: bool = false;

      /* Extend the top-down search by an edit step in each diagonal. */
      if fmin > dmin
        {
          fmin -= 1;
          *(fd as *mut OFFSET! ()).offset ((fmin - 1) as isize) = -1;
        }
      else
        { fmin += 1; }
      if fmax < dmax
        {
          fmax += 1;
          *(fd as *mut OFFSET! ()).offset ((fmax + 1) as isize) = -1;
        }
      else
        { fmax -= 1; }
      d = fmax;
      while d >= fmin
        {
          let mut x: OFFSET! ();
          let mut y: OFFSET! ();
          let tlo: OFFSET! () = *(fd as *mut OFFSET! ()).offset ((d - 1) as isize);
          let thi: OFFSET! () = *(fd as *mut OFFSET! ()).offset ((d + 1) as isize);
          let x0: OFFSET! () = if tlo < thi { thi } else { tlo + 1 };

          x = x0;
          y = x0 - d;
          while x < xlim && y < ylim && XVECREF_YVECREF_EQUAL! (ctxt, x, y)
            {
              x += 1;
              y += 1;
            }
          if x - x0 > SNAKE_LIMIT
            { big_snake = true; }
          *(fd as *mut OFFSET! ()).offset (d as isize) = x;
          if odd && bmin <= d && d <= bmax
             && *(bd as *mut OFFSET! ()).offset (d as isize) <= x
            {
              (*part).xmid = x;
              (*part).ymid = y;
              (*part).lo_minimal = true;
              (*part).hi_minimal = true;
              return;
            }
          d -= 2;
        }

      /* Similarly extend the bottom-up search.  */
      if bmin > dmin
        {
          bmin -= 1;
          *(bd as *mut OFFSET! ()).offset ((bmin - 1) as isize) = OFFSET_MAX;
        }
      else
        { bmin += 1; }
      if bmax < dmax
        {
          bmax += 1;
          *(bd as *mut OFFSET! ()).offset ((bmax + 1) as isize) = OFFSET_MAX;
        }
      else
        { bmax -= 1; }
      d = bmax;
      while d >= bmin
        {
          let mut x: OFFSET! ();
          let mut y: OFFSET! ();
          let tlo: OFFSET! () = *(bd as *mut OFFSET! ()).offset ((d - 1) as isize);
          let thi: OFFSET! () = *(bd as *mut OFFSET! ()).offset ((d + 1) as isize);
          let x0: OFFSET! () = if tlo < thi { tlo } else { thi - 1 };

          x = x0;
          y = x0 - d;
          while xoff < x && yoff < y && XVECREF_YVECREF_EQUAL! (ctxt, x - 1, y - 1)
            {
              x -= 1;
              y -= 1;
            }
          if x0 - x > SNAKE_LIMIT
            { big_snake = true; }
          *(bd as *mut OFFSET! ()).offset (d as isize) = x;
          if !odd && fmin <= d && d <= fmax
             && x <= *(fd as *mut OFFSET! ()).offset (d as isize)
            {
              (*part).xmid = x;
              (*part).ymid = y;
              (*part).lo_minimal = true;
              (*part).hi_minimal = true;
              return;
            }
          d -= 2;
        }

      if !find_minimal
        {
          let heuristic: bool = (*ctxt).heuristic;

          /* Heuristic: check occasionally for a diagonal that has made lots
             of progress compared with the edit distance.  If we have any
             such, find the one that has made the most progress and return it
             as if it had succeeded.

             With this heuristic, for vectors with a constant small density
             of changes, the algorithm is linear in the vector size.  */

          if 200 < c && big_snake && heuristic
            {
              {
                let mut best: OFFSET! () = 0;

                d = fmax;
                while d >= fmin
                  {
                    let dd: OFFSET! () = d - fmid;
                    let x: OFFSET! () = *(fd as *mut OFFSET! ()).offset (d as isize);
                    let y: OFFSET! () = x - d;
                    let v: OFFSET! () = (x - xoff) * 2 - dd;

                    if v > 12 * (c + (if dd < 0 { -dd } else { dd }))
                      {
                        if v > best
                            && xoff + SNAKE_LIMIT <= x && x < xlim
                            && yoff + SNAKE_LIMIT <= y && y < ylim
                          {
                            /* We have a good enough best diagonal; now insist
                               that it end with a significant snake.  */
                            let mut k: c_int = 1;
                            while XVECREF_YVECREF_EQUAL! (ctxt, x - k as OFFSET! (),
                                                          y - k as OFFSET! ())
                              {
                                if k == SNAKE_LIMIT as c_int
                                  {
                                    best = v;
                                    (*part).xmid = x;
                                    (*part).ymid = y;
                                    break;
                                  }
                                k += 1;
                              }
                          }
                      }
                    d -= 2;
                  }
                if best > 0
                  {
                    (*part).lo_minimal = true;
                    (*part).hi_minimal = false;
                    return;
                  }
              }

              {
                let mut best: OFFSET! () = 0;

                d = bmax;
                while d >= bmin
                  {
                    let dd: OFFSET! () = d - bmid;
                    let x: OFFSET! () = *(bd as *mut OFFSET! ()).offset (d as isize);
                    let y: OFFSET! () = x - d;
                    let v: OFFSET! () = (xlim - x) * 2 + dd;

                    if v > 12 * (c + (if dd < 0 { -dd } else { dd }))
                      {
                        if v > best
                            && xoff < x && x <= xlim - SNAKE_LIMIT
                            && yoff < y && y <= ylim - SNAKE_LIMIT
                          {
                            /* We have a good enough best diagonal; now insist
                               that it end with a significant snake.  */
                            let mut k: c_int = 0;
                            while XVECREF_YVECREF_EQUAL! (ctxt, x + k as OFFSET! (),
                                                          y + k as OFFSET! ())
                              {
                                if k == SNAKE_LIMIT as c_int - 1
                                  {
                                    best = v;
                                    (*part).xmid = x;
                                    (*part).ymid = y;
                                    break;
                                  }
                                k += 1;
                              }
                          }
                      }
                    d -= 2;
                  }
                if best > 0
                  {
                    (*part).lo_minimal = false;
                    (*part).hi_minimal = true;
                    return;
                  }
              }
            }

          /* Heuristic: if we've gone well beyond the call of duty, give up
             and report halfway between our best results so far.  */
          if c >= (*ctxt).too_expensive
            {
              /* Find forward diagonal that maximizes X + Y.  */
              let mut fxybest: OFFSET! () = -1;
              let mut fxbest: OFFSET! () = 0;
              d = fmax;
              while d >= fmin
                {
                  let mut x: OFFSET! () = MIN! (*(fd as *mut OFFSET! ()).offset (d as isize),
                                                xlim);
                  let mut y: OFFSET! () = x - d;
                  if ylim < y
                    {
                      x = ylim + d;
                      y = ylim;
                    }
                  if fxybest < x + y
                    {
                      fxybest = x + y;
                      fxbest = x;
                    }
                  d -= 2;
                }

              /* Find backward diagonal that minimizes X + Y.  */
              let mut bxybest: OFFSET! () = OFFSET_MAX;
              let mut bxbest: OFFSET! () = 0;
              d = bmax;
              while d >= bmin
                {
                  let mut x: OFFSET! () = MAX! (xoff, *(bd as *mut OFFSET! ()).offset (d as isize));
                  let mut y: OFFSET! () = x - d;
                  if y < yoff
                    {
                      x = yoff + d;
                      y = yoff;
                    }
                  if x + y < bxybest
                    {
                      bxybest = x + y;
                      bxbest = x;
                    }
                  d -= 2;
                }

              /* Use the better of the two diagonals.  */
              if (xlim + ylim) - bxybest < fxybest - (xoff + yoff)
                {
                  (*part).xmid = fxbest;
                  (*part).ymid = fxybest - fxbest;
                  (*part).lo_minimal = true;
                  (*part).hi_minimal = false;
                }
              else
                {
                  (*part).xmid = bxbest;
                  (*part).ymid = bxybest - bxbest;
                  (*part).lo_minimal = false;
                  (*part).hi_minimal = true;
                }
              return;
            }
        }
      c += 1;
    }
}

/* diffseq.h 449-561: `compareseq`.  The header's local `#define XREF_YREF_EQUAL(x,y)` is
   expanded at each use site, as in `diag`.  */

/* Compare in detail contiguous subsequences of the two vectors
   which are known, as a whole, to match each other.

   ... (the reference's explanation is diffseq.h 452-467).

   Return false if terminated normally, or true if terminated through early
   abort.  */
unsafe fn compareseq (mut xoff: OFFSET! (), mut xlim: OFFSET! (), mut yoff: OFFSET! (),
                      mut ylim: OFFSET! (), mut find_minimal: bool,
                      ctxt: *mut context) -> bool
{
  loop
    {
      /* Slide down the bottom initial diagonal.  */
      while xoff < xlim && yoff < ylim && XVECREF_YVECREF_EQUAL! (ctxt, xoff, yoff)
        {
          xoff += 1;
          yoff += 1;
        }

      /* Slide up the top initial diagonal. */
      while xoff < xlim && yoff < ylim && XVECREF_YVECREF_EQUAL! (ctxt, xlim - 1, ylim - 1)
        {
          xlim -= 1;
          ylim -= 1;
        }

      /* Handle simple cases. */
      if xoff == xlim
        {
          while yoff < ylim
            {
              NOTE_INSERT! (ctxt, yoff);
              if EARLY_ABORT! (ctxt)
                { return true; }
              yoff += 1;
            }
          break;
        }
      if yoff == ylim
        {
          while xoff < xlim
            {
              NOTE_DELETE! (ctxt, xoff);
              if EARLY_ABORT! (ctxt)
                { return true; }
              xoff += 1;
            }
          break;
        }

      let mut part: partition = std::mem::zeroed ();

      /* Find a point of correspondence in the middle of the vectors.  */
      diag (xoff, xlim, yoff, ylim, find_minimal, &mut part, ctxt);

      /* Use the partitions to split this problem into subproblems.  */
      let mut xoff1: OFFSET! ();
      let mut xlim1: OFFSET! ();
      let mut yoff1: OFFSET! ();
      let mut ylim1: OFFSET! ();
      let mut xoff2: OFFSET! ();
      let mut xlim2: OFFSET! ();
      let mut yoff2: OFFSET! ();
      let mut ylim2: OFFSET! ();
      let mut find_minimal1: bool;
      let mut find_minimal2: bool;
      if !NOTE_ORDERED
          && ((xlim + ylim) - (part.xmid + part.ymid)
              < (part.xmid + part.ymid) - (xoff + yoff))
        {
          /* The second problem is smaller and the caller doesn't
             care about order, so do the second problem first to
             lessen recursion.  */
          xoff1 = part.xmid; xlim1 = xlim;
          yoff1 = part.ymid; ylim1 = ylim;
          find_minimal1 = part.hi_minimal;

          xoff2 = xoff; xlim2 = part.xmid;
          yoff2 = yoff; ylim2 = part.ymid;
          find_minimal2 = part.lo_minimal;
        }
      else
        {
          xoff1 = xoff; xlim1 = part.xmid;
          yoff1 = yoff; ylim1 = part.ymid;
          find_minimal1 = part.lo_minimal;

          xoff2 = part.xmid; xlim2 = xlim;
          yoff2 = part.ymid; ylim2 = ylim;
          find_minimal2 = part.hi_minimal;
        }

      /* Recurse to do one subproblem.  */
      let early: bool = compareseq (xoff1, xlim1, yoff1, ylim1, find_minimal1, ctxt);
      if early
        { return early; }

      /* Iterate to do the other subproblem.  */
      xoff = xoff2; xlim = xlim2;
      yoff = yoff2; ylim = ylim2;
      find_minimal = find_minimal2;
    }

  return false;
}

DEFUN! ("replace-region-contents", Freplace_region_contents, Sreplace_region_contents,
       3, 6,
       "(list (if (use-region-p) (region-beginning) (point-min))               (if (use-region-p) (region-end) (point-max)) \t               (get-buffer (read-buffer-to-switch \"Source buffer: \")))",
       doc: "Replace the region between BEG and END with that of SOURCE.\nSOURCE can be a buffer, a string, or a vector [SBUF SBEG SEND]\ndenoting the substring SBEG..SEND of buffer SBUF.\n\nIf optional argument INHERIT is non-nil, the inserted text will inherit\nproperties from adjoining text.\n\nAs far as possible the replacement is non-destructive, i.e. existing\nbuffer contents, markers, point, properties, and overlays in the current\nbuffer stay intact.  Point is treated like an \"insert before\" marker:\nif point starts at END, it will always be at the end of the replacement\nwhen this function returns, whereas if point starts at BEG it will\nremain at BEG only if the replaced text is not empty.\n\nBecause this function can be very slow if there is a large number of\ndifferences between the two buffers, there are two optional arguments\nmitigating this issue.\n\nThe MAX-SECS argument, if given, defines a hard limit on the time used\nfor comparing the buffers.  If it takes longer than MAX-SECS, the\nfunction falls back to a plain `delete-region' and\n`insert-buffer-substring'.  (Note that the checks are not performed\ntoo evenly over time, so in some cases it may run a bit longer than\nallowed).  In particular, passing zero as the value of MAX-SECS\ndisables the comparison step, so this function immediately falls\nback to a plain delete/insert method.\n\nThe optional argument MAX-COSTS defines the quality of the difference\ncomputation.  If the actual costs exceed this limit, heuristics are\nused to provide a faster but suboptimal solution.  The default value\nis 1000000.\n\nNote: If the replacement is a string, it'll usually be placed internally\nin a temporary buffer.  Therefore, all else being equal, it is preferable\nto pass a buffer rather than a string as SOURCE argument.\n\nThis function returns t if a non-destructive replacement could be\nperformed.  Otherwise, i.e., if MAX-SECS was exceeded or too much\nmemory would have been needed, it returns nil.\n\nSOURCE can also be a function that will be called with no arguments\nand with current buffer narrowed to BEG..END, and should return\na buffer or a string.  But this is deprecated.",
       (beg: Lisp_Object, end: Lisp_Object, source: Lisp_Object, max_secs: Lisp_Object,
        max_costs: Lisp_Object, inherit: Lisp_Object),
       {
  let mut beg: Lisp_Object = beg;
  let mut end: Lisp_Object = end;
  let mut source: Lisp_Object = source;
  validate_region (&mut beg, &mut end);
  let min_a: ptrdiff_t = XFIXNUM (beg);
  let size_a: ptrdiff_t = XFIXNUM (end) - min_a;
  eassume! (size_a >= 0);
  let a_empty: bool = size_a == 0;
  let inh: bool = !NILP (inherit);

  if FUNCTIONP (source)
    {
      let count: specpdl_ref = SPECPDL_INDEX ();
      record_unwind_protect_excursion ();
      record_unwind_protect (Some (save_restriction_restore),
                             save_restriction_save ());
      Fnarrow_to_region (beg, end);
      source = calln! (source);
      unbind_to (count, Qnil ());
    }
  let min_b: ptrdiff_t;
  let size_b: ptrdiff_t;
  let mut b: *mut buffer;
  if STRINGP (source)
    {
      b = std::ptr::null_mut ();
      min_b = BEG as ptrdiff_t;		/* Assuming we'll copy it into a buffer.  */
      /* Like 'size_b = SCHARS (source);', except inline to pacify -Wclobbered
         with gcc 14.2.1 20250110 (Red Hat 14.2.1-7) x86-64 -O2; see
         <https://gcc.gnu.org/bugzilla/show_bug.cgi?id=21161>.  */
      size_b = (*XSTRING (source)).u.s.size as ptrdiff_t;
    }
  else if BUFFERP (source)
    {
      b = XBUFFER (source);
      min_b = BUF_BEGV (b);
      size_b = BUF_ZV (b) - min_b;
    }
  else
    {
      CHECK_TYPE (VECTORP (source),
                  list! (Qor (), Qstring (), Qbuffer (), Qvector ()), source);
      /* Let Faref signal an error if SOURCE is too small.  */
      let mut send: Lisp_Object = Faref (source, make_fixnum (2));
      let mut sbeg: Lisp_Object = AREF (source, 1);
      CHECK_BUFFER (AREF (source, 0));
      b = XBUFFER (AREF (source, 0));
      let count: specpdl_ref = SPECPDL_INDEX ();
      record_unwind_current_buffer ();
      set_buffer_internal (b);
      validate_region (&mut sbeg, &mut send);
      unbind_to (count, Qnil ());
      min_b = XFIXNUM (sbeg);
      size_b = XFIXNUM (send) - min_b;
    }
  eassume! (0 <= size_b);
  let b_empty: bool = size_b == 0;
  if !b.is_null () && !BUFFER_LIVE_P (b)
    { error (c"Selecting deleted buffer".as_ptr ()); }

  /* Handle trivial cases where at least one accessible portion is
     empty.  */

  if a_empty && b_empty
    { return Qt (); }
  else if a_empty || b_empty
           || EQ (max_secs, make_fixnum (0))
           || EQ (max_costs, make_fixnum (0))
    {
      replace_range (min_a, min_a + size_a, source, true, false, inh);
      return Qt ();
    }

  let a: *mut buffer = (current_buffer! ());
  if a == b
    { error (c"Cannot replace a buffer with itself".as_ptr ()); }

  let mut too_expensive: ptrdiff_t;
  if NILP (max_costs)
    { too_expensive = 1000000; }
  else if FIXNUMP (max_costs)
    { too_expensive = clip_to_bounds (0, XFIXNUM (max_costs), PTRDIFF_MAX); }
  else
    {
      CHECK_INTEGER (max_costs);
      too_expensive = if NILP (Fnatnump (max_costs)) { 0 } else { PTRDIFF_MAX };
    }

  let mut time_limit: timespec = make_timespec (0, -1);
  if !NILP (max_secs)
    {
      let tlim: timespec
        = timespec_add (current_timespec (),
                        lisp_time_argument (max_secs));
      let tmax: timespec = make_timespec (TYPE_MAXIMUM! (time_t), TIMESPEC_HZ as c_long - 1);
      if timespec_cmp (tlim, tmax) < 0
        { time_limit = tlim; }
    }

  let count: specpdl_ref = SPECPDL_INDEX ();

  /* The rest of the code is not prepared to handle a string SOURCE.  */
  if b.is_null ()
    {
      let workbuf: Lisp_Object
        = code_conversion_save (true, STRING_MULTIBYTE (source));
      b = XBUFFER (workbuf);
      set_buffer_internal (b);
      CALLN! (Finsert, source);
      set_buffer_internal (a);
    }
  let source_buffer: Lisp_Object = make_lisp_ptr (b as *mut c_void, Lisp_Type::Lisp_Vectorlike);

  let diags: ptrdiff_t = size_a + size_b + 3;
  let bytes_per_diag: ptrdiff_t = 2 * std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
    + std::mem::size_of::<c_int> () as ptrdiff_t;
  let surplus_char_bytes: ptrdiff_t = 3 * std::mem::size_of::<c_int> () as ptrdiff_t;
  let align_bytes: ptrdiff_t = if std::mem::align_of::<ptrdiff_t> () < std::mem::align_of::<c_int> ()
    { (std::mem::align_of::<c_int> () - std::mem::align_of::<ptrdiff_t> ()) as ptrdiff_t }
    else { 0 };
  let del_bytes: ptrdiff_t = size_a / CHAR_BIT as ptrdiff_t + 1;
  let ins_bytes: ptrdiff_t = size_b / CHAR_BIT as ptrdiff_t + 1;
  let mut buffer: *mut ptrdiff_t;
  let mut bytes_needed: ptrdiff_t = 0;
  if { let (v, o) = diags.overflowing_mul (bytes_per_diag); bytes_needed = v; o }
      || { let (v, o) = bytes_needed.overflowing_add
             (align_bytes - surplus_char_bytes + del_bytes + ins_bytes);
           bytes_needed = v; o }
      || SIZE_MAX < bytes_needed as usize
    { buffer = std::ptr::null_mut (); }
  else if bytes_needed <= MAX_ALLOCA
    /* C's bare `alloca` is the port's stack pool (PORTING.md P-05's decision; the eval.rs
       shape).  */
    { buffer = sa_stack_alloc (bytes_needed as usize) as *mut ptrdiff_t; }
  else
    {
      buffer = malloc (bytes_needed as usize) as *mut ptrdiff_t;
      if !buffer.is_null ()
        {
          if profiler_memory_running
            { malloc_probe (bytes_needed as usize); }
          record_unwind_protect_ptr (Some (xfree), buffer as *mut c_void);
        }
    }

  let mut early_abort: bool;
  let mut ctx: context = std::mem::zeroed ();

  if !buffer.is_null ()
    {
      /* Copy the characters to arrays of C ints.  This speeds up
         comparison dramatically in multibyte buffers.  */
      let chars_a: *mut c_int
        = (((buffer.add ((2 * diags) as usize) as usize) + align_bytes as usize)
           & !(align_bytes as usize)) as *mut c_int;
      let mut i: ptrdiff_t = 0;
      while i < size_a
        {
          *chars_a.offset (i as isize)
            = BUF_FETCH_CHAR_AS_MULTIBYTE (a, buf_charpos_to_bytepos (a as *mut c_void, min_a + i));
          i += 1;
        }

      let chars_b: *mut c_int = chars_a.offset (size_a as isize);
      i = 0;
      while i < size_b
        {
          *chars_b.offset (i as isize)
            = BUF_FETCH_CHAR_AS_MULTIBYTE (b, buf_charpos_to_bytepos (b as *mut c_void, min_b + i));
          i += 1;
        }

      let deletions_insertions: *mut c_uchar
        = memset (chars_b.offset (size_b as isize) as *mut c_void, 0,
                  (del_bytes + ins_bytes) as usize) as *mut c_uchar;

      /* FIXME: It is not documented how to initialize the contents of the
         context structure.  This code cargo-cults from the existing
         caller in src/analyze.c of GNU Diffutils, which appears to
         work.  */
      ctx = context
        { chars_a: chars_a,
          chars_b: chars_b,
          deletions: deletions_insertions,
          insertions: deletions_insertions.offset (del_bytes as isize),
          fdiag: buffer.offset ((size_b + 1) as isize),
          bdiag: buffer.offset ((diags + size_b + 1) as isize),
          heuristic: true,
          too_expensive: too_expensive,
          time_limit: time_limit,
          ..std::mem::zeroed () };

      /* compareseq wants zero-based indices.  We add BEGV back later.  */
      if sys_setjmp (&raw mut ctx.jmp as *mut jmp_buf) == 0
        { early_abort = compareseq (0, size_a, 0, size_b, false, &mut ctx); }
      else
        { early_abort = true; }
    }
  else
    { early_abort = true; }

  if early_abort
    {
      let src: Lisp_Object = CALLN! (Fvector, source_buffer,
                                     make_fixnum (BUF_BEGV (b)),
                                     make_fixnum (BUF_ZV (b)));
      replace_range (min_a, min_a + size_a, src, true, false, inh);
      return unbind_to (count, Qnil ());
    }

  Fundo_boundary ();
  let mut modification_hooks_inhibited: bool = false;

  /* We are going to make a lot of small modifications, and having the
     modification hooks called for each of them will slow us down.
     Instead, we announce a single modification for the entire
     modified region.  But don't do that if the caller inhibited
     modification hooks, because then they don't want that.  */
  if !inhibit_modification_hooks
    {
      prepare_to_modify_buffer (min_a, min_a + size_a, std::ptr::null_mut ());
      specbind (Qinhibit_modification_hooks (), Qt ());
      modification_hooks_inhibited = true;
    }

  let mut i: ptrdiff_t = size_a;
  let mut j: ptrdiff_t = size_b;
  let src: Lisp_Object = CALLN! (Fvector, source_buffer, Qnil (), Qnil ());
  /* Walk backwards through the lists of changes.  This was also
     cargo-culted from src/analyze.c in GNU Diffutils.  Because we
     walk backwards, we don't have to keep the positions in sync.  */
  while i >= 0 || j >= 0
    {
      ctx.quitcounter = ctx.quitcounter.wrapping_add (1);
      rarely_quit (ctx.quitcounter as u64);

      /* Check whether there is a change (insertion or deletion)
         before the current position.  */
      if (i > 0 && bit_is_set (ctx.deletions, i - 1))
          || (j > 0 && bit_is_set (ctx.insertions, j - 1))
        {
          let end_a: ptrdiff_t = min_a + i;
          let end_b: ptrdiff_t = min_b + j;
          /* Find the beginning of the current change run.  */
          while i > 0 && bit_is_set (ctx.deletions, i - 1)
            { i -= 1; }
          while j > 0 && bit_is_set (ctx.insertions, j - 1)
            { j -= 1; }

          let beg_a: ptrdiff_t = min_a + i;
          let beg_b: ptrdiff_t = min_b + j;
          eassert! (beg_a <= end_a);
          eassert! (beg_b <= end_b);
          eassert! (beg_a < end_a || beg_b < end_b);
          ASET (src, 1, make_fixed_natnum (beg_b));
          ASET (src, 2, make_fixed_natnum (end_b));
          replace_range (beg_a, end_a, src, true, false, inh);
        }
      i -= 1;
      j -= 1;
    }

  unbind_to (count, Qnil ());

  if modification_hooks_inhibited
    {
      signal_after_change (min_a, size_a, size_b);
      update_compositions (min_a, min_a + size_b, CHECK_INSIDE);
      /* We've locked the buffer's file above in
         prepare_to_modify_buffer; if the buffer is unchanged at this
         point, i.e. no insertions or deletions have been made, unlock
         the file now.  */
      if SAVE_MODIFF! () == MODIFF! ()
          && STRINGP (BVAR! (a, file_truename_))
        { Funlock_file (BVAR! (a, file_truename_)); }
    }

  return Qt ();
});

unsafe fn set_bit (a: *mut c_uchar, i: ptrdiff_t)
{
  eassume! (0 <= i);
  *a.offset ((i / CHAR_BIT as ptrdiff_t) as isize)
    |= (1u8 << (i % CHAR_BIT as ptrdiff_t) as u32);
}

unsafe fn bit_is_set (a: *const c_uchar, i: ptrdiff_t) -> bool
{
  eassume! (0 <= i);
  return (*a.offset ((i / CHAR_BIT as ptrdiff_t) as isize)
          & (1u8 << (i % CHAR_BIT as ptrdiff_t) as u32)) != 0;
}

/* Return true if the characters at position POS_A in the first stretch
   of text to be compared and at position POS_B of the second one are
   equal.  POS_A and POS_B are zero-based.  Text properties are
   ignored.

   (The reference's implementation note about the innermost loop is
   diffseq.h's comment at editfns.c 2246-2259.)  */

unsafe fn buffer_chars_equal (ctx: *mut context, pos_a: ptrdiff_t,
                              pos_b: ptrdiff_t) -> bool
{
  (*ctx).quitcounter = (*ctx).quitcounter.wrapping_add (1);
  if (*ctx).quitcounter == 0
    {
      maybe_quit ();
      if compareseq_early_abort (ctx)
        { sys_longjmp (&raw mut (*ctx).jmp as *mut jmp_buf, 1); }
    }

  return *(*ctx).chars_a.offset (pos_a as isize) == *(*ctx).chars_b.offset (pos_b as isize);
}

unsafe fn compareseq_early_abort (ctx: *mut context) -> bool
{
  if (*ctx).time_limit.tv_nsec < 0
    { return false; }
  return timespec_cmp ((*ctx).time_limit, current_timespec ()) < 0;
}

unsafe extern "C" fn subst_char_in_region_unwind (arg: Lisp_Object)
{
  bset_undo_list ((current_buffer! ()), arg);
}

unsafe extern "C" fn subst_char_in_region_unwind_1 (arg: Lisp_Object)
{
  bset_filename ((current_buffer! ()), arg);
}

DEFUN! ("subst-char-in-region", Fsubst_char_in_region,
       Ssubst_char_in_region, 4, 5, 0,
       doc: "From START to END, replace FROMCHAR with TOCHAR each time it occurs.\nIf optional arg NOUNDO is non-nil, don't record this change for undo\nand don't mark the buffer as really changed.\nBoth characters must have the same length of multi-byte form.",
       (start: Lisp_Object, end: Lisp_Object, fromchar: Lisp_Object, tochar: Lisp_Object,
        noundo: Lisp_Object),
       {
  let mut start: Lisp_Object = start;
  let mut end: Lisp_Object = end;
  let mut pos: ptrdiff_t;
  let mut pos_byte: ptrdiff_t;
  let mut stop: ptrdiff_t;
  let mut i: ptrdiff_t;
  let mut len: ptrdiff_t;
  let mut end_byte: ptrdiff_t;
  /* Keep track of the first change in the buffer:
     if 0 we haven't found it yet.
     if < 0 we've found it and we've run the before-change-function.
     if > 0 we've actually performed it and the value is its position.  */
  let mut changed: ptrdiff_t = 0;
  let mut fromstr: [u8; MAX_MULTIBYTE_LENGTH as usize] = std::mem::zeroed ();
  let mut tostr: [u8; MAX_MULTIBYTE_LENGTH as usize] = std::mem::zeroed ();
  let mut p: *mut u8;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut last_changed: ptrdiff_t = 0;
  let multibyte_p: bool
    = !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
  let mut fromc: c_int;
  let mut toc: c_int;

  /* C 2316-2326's `#ifdef HAVE_TREE_SITTER` block is dead for the frozen configuration
     (HAVE_TREE_SITTER is undef) and has no Rust counterpart (docs/ports/editfns.md).  */

  'restart: loop
    {
      validate_region (&mut start, &mut end);
      CHECK_CHARACTER (fromchar);
      CHECK_CHARACTER (tochar);
      fromc = XFIXNAT (fromchar) as c_int;
      toc = XFIXNAT (tochar) as c_int;

      if multibyte_p
        {
          len = CHAR_STRING (fromc, fromstr.as_mut_ptr ()) as ptrdiff_t;
          if CHAR_STRING (toc, tostr.as_mut_ptr ()) as ptrdiff_t != len
            { error (c"Characters in `subst-char-in-region' have different byte-lengths".as_ptr ()); }
        }
      else
        {
          len = 1;
          fromstr[0] = fromc as u8;
          tostr[0] = toc as u8;
        }

      pos = XFIXNUM (start);
      pos_byte = CHAR_TO_BYTE (pos);
      stop = CHAR_TO_BYTE (XFIXNUM (end));
      end_byte = stop;

      /* If we don't want undo, turn off putting stuff on the list.
         That's faster than getting rid of things,
         and it prevents even the entry for a first change.
         Also inhibit locking the file.  */
      if changed == 0 && !NILP (noundo)
        {
          record_unwind_protect (Some (subst_char_in_region_unwind),
                                 BVAR! ((current_buffer! ()), undo_list_));
          bset_undo_list ((current_buffer! ()), Qt ());
          /* Don't do file-locking.  */
          record_unwind_protect (Some (subst_char_in_region_unwind_1),
                                 BVAR! ((current_buffer! ()), filename_));
          bset_filename ((current_buffer! ()), Qnil ());
        }

      if pos_byte < GPT_BYTE! ()
        { stop = min (stop, GPT_BYTE! ()); }
      loop
        {
          let mut pos_byte_next: ptrdiff_t = pos_byte;

          if pos_byte >= stop
            {
              if pos_byte >= end_byte
                { break; }
              stop = end_byte;
            }
          p = BYTE_POS_ADDR (pos_byte);
          if multibyte_p
            { pos_byte_next += next_char_len (pos_byte_next) as ptrdiff_t; }
          else
            { pos_byte_next += 1; }
          if pos_byte_next - pos_byte == len
              && *p.offset (0) == fromstr[0]
              && (len == 1
                  || (*p.offset (1) == fromstr[1]
                      && (len == 2 || (*p.offset (2) == fromstr[2]
                                       && (len == 3 || *p.offset (3) == fromstr[3])))))
            {
              if changed < 0
                /* We've already seen this and run the before-change-function;
                   this time we only need to record the actual position. */
                { changed = pos; }
              else if changed == 0
                {
                  changed = -1;
                  modify_text (pos, XFIXNUM (end));

                  if !NILP (noundo)
                    {
                      let m: modiff_count = MODIFF! ();
                      if SAVE_MODIFF! () == m - 1
                        { SAVE_MODIFF! () = m; }
                      if BUF_AUTOSAVE_MODIFF! ((current_buffer! ())) == m - 1
                        { BUF_AUTOSAVE_MODIFF! ((current_buffer! ())) = m; }
                    }

                  /* The before-change-function may have moved the gap
                     or even modified the buffer so we should start over. */
                  continue 'restart;
                }

              if NILP (noundo)
                { record_change (pos, 1); }
              i = 0;
              while i < len
                {
                  *p = tostr[i as usize];
                  p = p.add (1);
                  i += 1;
                }

              last_changed = pos + 1;
            }
          pos_byte = pos_byte_next;
          pos += 1;
        }

      if changed > 0
        {
          /* C 2429-2430's `#ifdef HAVE_TREE_SITTER` block is dead (docs/ports/editfns.md).  */
          signal_after_change (changed,
                               last_changed - changed, last_changed - changed);
          update_compositions (changed, last_changed, CHECK_ALL);
        }

      return unbind_to (count, Qnil ());
    }
});

/* Helper function for Ftranslate_region_internal.

   Check if a character sequence at POS (POS_BYTE) matches an element
   of VAL.  VAL is a list (([FROM-CHAR ...] . TO) ...).  If a matching
   element is found, return it.  Otherwise return Qnil.  */

unsafe fn check_translation (pos: ptrdiff_t, pos_byte: ptrdiff_t, end: ptrdiff_t,
                             val: Lisp_Object) -> Lisp_Object
{
  let mut pos_byte: ptrdiff_t = pos_byte;
  let mut initial_buf: [c_int; 16] = std::mem::zeroed ();
  let mut buf: *mut c_int = initial_buf.as_mut_ptr ();
  let mut buf_size: ptrdiff_t = countof! (initial_buf) as ptrdiff_t;
  let mut bufalloc: *mut c_int = std::ptr::null_mut ();
  let mut buf_used: ptrdiff_t = 0;
  let mut result: Lisp_Object = Qnil ();
  let mut val: Lisp_Object = val;

  while CONSP (val)
    {
      let mut elt: Lisp_Object;
      let mut len: ptrdiff_t;
      let mut i: ptrdiff_t;

      elt = XCAR (val);
      if !CONSP (elt)
        { val = XCDR (val); continue; }
      elt = XCAR (elt);
      if !VECTORP (elt)
        { val = XCDR (val); continue; }
      len = ASIZE (elt);
      if len <= end - pos
        {
          i = 0;
          while i < len
            {
              if buf_used <= i
                {
                  let p: *mut u8 = BYTE_POS_ADDR (pos_byte);
                  let mut len1: c_int = 0;

                  if buf_used == buf_size
                    {
                      bufalloc = xpalloc (bufalloc as *mut c_void, &mut buf_size, 1, -1,
                                          std::mem::size_of::<c_int> () as ptrdiff_t)
                        as *mut c_int;
                      if std::ptr::eq (buf, initial_buf.as_mut_ptr ())
                        { memcpy (bufalloc as *mut c_void, buf as *const c_void,
                                  std::mem::size_of::<[c_int; 16]> ()); }
                      buf = bufalloc;
                    }
                  *buf.offset (buf_used as isize) = string_char_and_length (p, &mut len1);
                  buf_used += 1;
                  pos_byte += len1 as ptrdiff_t;
                }
              if XFIXNUM (AREF (elt, i)) != *buf.offset (i as isize) as EMACS_INT
                { break; }
              i += 1;
            }
          if i == len
            {
              result = XCAR (val);
              break;
            }
        }
      val = XCDR (val);
    }

  xfree (bufalloc as *mut c_void);
  return result;
}

DEFUN! ("translate-region-internal", Ftranslate_region_internal,
       Stranslate_region_internal, 3, 3, 0,
       doc: "Internal use only.\nFrom START to END, translate characters according to TABLE.\nTABLE is a string or a char-table; the Nth character in it is the\nmapping for the character with code N.\nIt returns the number of characters changed.",
       (start: Lisp_Object, end: Lisp_Object, table: Lisp_Object),
       {
  let mut start: Lisp_Object = start;
  let mut end: Lisp_Object = end;
  let mut table: Lisp_Object = table;
  let mut translatable_chars: c_int = MAX_CHAR + 1;
  let multibyte: bool = !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
  let mut string_multibyte: bool = false;

  validate_region (&mut start, &mut end);
  if STRINGP (table)
    {
      if !multibyte
        { table = string_make_unibyte (table); }
      translatable_chars = min (translatable_chars as isize, SBYTES (table)) as c_int;
      string_multibyte = STRING_MULTIBYTE (table);
    }
  else if !(CHAR_TABLE_P (table)
            && EQ ((*XCHAR_TABLE (table)).purpose, Qtranslation_table ()))
    { error (c"Not a translation table".as_ptr ()); }

  let mut pos: ptrdiff_t = XFIXNUM (start);
  let mut pos_byte: ptrdiff_t = CHAR_TO_BYTE (pos);
  let mut end_pos: ptrdiff_t = XFIXNUM (end);
  modify_text (pos, end_pos);

  let mut characters_changed: ptrdiff_t = 0;

  while pos < end_pos
    {
      let mut p: *mut u8 = BYTE_POS_ADDR (pos_byte);
      let mut str_: *mut u8 = std::ptr::null_mut ();
      let mut buf: [u8; MAX_MULTIBYTE_LENGTH as usize] = std::mem::zeroed ();
      let mut len: c_int = 0;
      let mut oc: c_int;

      if multibyte
        { oc = string_char_and_length (p, &mut len); }
      else
        { oc = *p as c_int; len = 1; }
      if oc < translatable_chars
        {
          let mut nc: c_int; /* New character.  */
          let mut str_len: c_int = 0;
          let mut val: Lisp_Object = Qnil ();

          if STRINGP (table)
            {
              /* Reload as signal_after_change in last iteration may GC.  */
              let tt: *mut u8 = SDATA (table);

              if string_multibyte
                {
                  str_ = tt.offset (string_char_to_byte (table, oc as ptrdiff_t) as isize);
                  nc = string_char_and_length (str_, &mut str_len);
                }
              else
                {
                  nc = *tt.offset (oc as isize) as c_int;
                  if !ASCII_CHAR_P (nc as isize) && multibyte
                    {
                      str_len = BYTE8_STRING (nc, buf.as_mut_ptr ());
                      str_ = buf.as_mut_ptr ();
                    }
                  else
                    {
                      str_len = 1;
                      str_ = tt.offset (oc as isize);
                    }
                }
            }
          else
            {
              nc = oc;
              val = CHAR_TABLE_REF (table, oc as isize);
              if CHARACTERP (val)
                {
                  nc = XFIXNAT (val) as c_int;
                  str_len = CHAR_STRING (nc, buf.as_mut_ptr ());
                  str_ = buf.as_mut_ptr ();
                }
              else if VECTORP (val) || CONSP (val)
                {
                  /* VAL is [TO_CHAR ...] or (([FROM-CHAR ...] .  TO) ...)
                     where TO is TO-CHAR or [TO-CHAR ...].  */
                  nc = -1;
                }
            }

          if nc != oc && nc >= 0
            {
              /* Simple one char to one char translation.  */
              if len != str_len
                {
                  let string: Lisp_Object;

                  /* This is less efficient, because it moves the gap,
                     but it should handle multibyte characters correctly.  */
                  string = make_multibyte_string (str_ as *const c_char, 1, str_len as ptrdiff_t);
                  replace_range (pos, pos + 1, string,
                                 true, false, false);
                  len = str_len;
                }
              else
                {
                  /* C 2617-2624's `#ifdef HAVE_TREE_SITTER` block is dead
                     (docs/ports/editfns.md).  */
                  record_change (pos, 1);
                  while str_len > 0
                    {
                      str_len -= 1;
                      *p = *str_;
                      p = p.add (1);
                      str_ = str_.add (1);
                    }
                  signal_after_change (pos, 1, 1);
                  update_compositions (pos, pos + 1, CHECK_BORDER);

                  /* C 2632-2639's `#ifdef HAVE_TREE_SITTER` block is dead
                     (docs/ports/editfns.md).  */
                }
              characters_changed += 1;
            }
          else if nc < 0
            {
              if CONSP (val)
                {
                  val = check_translation (pos, pos_byte, end_pos, val);
                  if NILP (val)
                    {
                      pos_byte += len as ptrdiff_t;
                      pos += 1;
                      continue;
                    }
                  /* VAL is ([FROM-CHAR ...] . TO).  */
                  len = ASIZE (XCAR (val)) as c_int;
                  val = XCDR (val);
                }
              else
                { len = 1; }

              let string: Lisp_Object
                = if VECTORP (val)
                  { Fconcat (1, &mut val) }
                  else { Fmake_string (make_fixnum (1), val, Qnil ()) };
              replace_range (pos, pos + len as ptrdiff_t, string, true, false, false);
              pos_byte += SBYTES (string);
              pos += SCHARS (string);
              characters_changed += SCHARS (string);
              end_pos += SCHARS (string) - len as ptrdiff_t;
              continue;
            }
        }
      pos_byte += len as ptrdiff_t;
      pos += 1;
    }

  return make_fixnum (characters_changed);
});

DEFUN! ("delete-region", Fdelete_region, Sdelete_region, 2, 2, "r",
       doc: "Delete the text between START and END.\nIf called interactively, delete the region between point and mark.\nThis command deletes buffer text without modifying the kill ring.",
       (start: Lisp_Object, end: Lisp_Object),
       {
  let mut start: Lisp_Object = start;
  let mut end: Lisp_Object = end;
  validate_region (&mut start, &mut end);
  del_range (XFIXNUM (start), XFIXNUM (end));
  return Qnil ();
});

DEFUN! ("delete-and-extract-region", Fdelete_and_extract_region,
       Sdelete_and_extract_region, 2, 2, 0,
       doc: "Delete the text between START and END and return it.",
       (start: Lisp_Object, end: Lisp_Object),
       {
  let mut start: Lisp_Object = start;
  let mut end: Lisp_Object = end;
  validate_region (&mut start, &mut end);
  if XFIXNUM (start) == XFIXNUM (end)
    { return empty_unibyte_string; }
  return del_range_1 (XFIXNUM (start), XFIXNUM (end), true, true);
});

/* Alist of buffers in which labeled restrictions are used.  The car
   of each list element is a buffer, the cdr is a list of triplets
   (label begv-marker zv-marker).  The last triplet of that list
   always uses the (uninterned) Qoutermost_restriction label, and
   records the restriction bounds that were current when the first
   labeled restriction was entered (which may be a narrowing that was
   set by the user and is visible on display).  This alist is used
   internally by narrow-to-region, internal--labeled-narrow-to-region,
   widen, internal--labeled-widen and save-restriction.  For
   efficiency reasons, an alist is used instead of a buffer-local
   variable: otherwise reset_outermost_restrictions, which is called
   during each redisplay cycle, would have to loop through all live
   buffers.  */
static mut labeled_restrictions: Lisp_Object = lisp_h_Qnil ();

/* Add BUF with its list of labeled RESTRICTIONS in the
   labeled_restrictions alist.  */
unsafe fn labeled_restrictions_add (buf: Lisp_Object, restrictions: Lisp_Object)
{
  labeled_restrictions = nconc2 (list1 (list2 (buf, restrictions)),
                                 labeled_restrictions);
}

/* Remove BUF and its list of labeled restrictions from the
   labeled_restrictions alist.  Do nothing if BUF is not present in
   labeled_restrictions.  */
unsafe fn labeled_restrictions_remove (buf: Lisp_Object)
{
  labeled_restrictions = Fdelq (Fassoc (buf, labeled_restrictions, Qnil ()),
                                labeled_restrictions);
}

/* Retrieve one of the labeled restriction bounds in BUF from the
   labeled_restrictions alist, as a marker, or return nil if BUF is
   not in labeled_restrictions or is a killed buffer.  When OUTERMOST
   is true, the restriction bounds that were current when the first
   labeled restriction was entered are returned.  Otherwise the bounds
   of the innermost labeled restriction are returned.  */
unsafe fn labeled_restrictions_get_bound (buf: Lisp_Object, begv: bool,
                                          outermost: bool) -> Lisp_Object
{
  if NILP (Fbuffer_live_p (buf))
    { return Qnil (); }
  let mut restrictions: Lisp_Object = assq_no_quit (buf, labeled_restrictions);
  if NILP (restrictions)
    { return Qnil (); }
  restrictions = XCAR (XCDR (restrictions));
  let bounds: Lisp_Object
    = if outermost
      { XCDR (assq_no_quit (Qoutermost_restriction (), restrictions)) }
      else { XCDR (XCAR (restrictions)) };
  eassert! (!NILP (bounds));
  let marker: Lisp_Object = if begv { XCAR (bounds) } else { XCAR (XCDR (bounds)) };
  eassert! (EQ (Fmarker_buffer (marker), buf));
  return marker;
}

/* Retrieve the label of the innermost labeled restriction in BUF.
   Return nil if BUF is not in labeled_restrictions or is a killed
   buffer.  */
unsafe fn labeled_restrictions_peek_label (buf: Lisp_Object) -> Lisp_Object
{
  if NILP (Fbuffer_live_p (buf))
    { return Qnil (); }
  let restrictions: Lisp_Object = assq_no_quit (buf, labeled_restrictions);
  if NILP (restrictions)
    { return Qnil (); }
  let label: Lisp_Object = XCAR (XCAR (XCAR (XCDR (restrictions))));
  eassert! (!NILP (label));
  return label;
}

/* Add a labeled RESTRICTION for BUF in the labeled_restrictions
   alist.  */
unsafe fn labeled_restrictions_push (buf: Lisp_Object, restriction: Lisp_Object)
{
  let restrictions: Lisp_Object = assq_no_quit (buf, labeled_restrictions);
  if NILP (restrictions)
    { labeled_restrictions_add (buf, list1 (restriction)); }
  else
    { XSETCDR (restrictions, list1 (nconc2 (list1 (restriction),
                                             XCAR (XCDR (restrictions))))); }
}

/* Remove the innermost labeled restriction in BUF from the
   labeled_restrictions alist.  Do nothing if BUF is not present in
   labeled_restrictions.  */
unsafe fn labeled_restrictions_pop (buf: Lisp_Object)
{
  let restrictions: Lisp_Object = assq_no_quit (buf, labeled_restrictions);
  if NILP (restrictions)
    { return; }
  if BASE_EQ (labeled_restrictions_peek_label (buf), Qoutermost_restriction ())
    { labeled_restrictions_remove (buf); }
  else
    { XSETCDR (restrictions, list1 (XCDR (XCAR (XCDR (restrictions))))); }
}

/* Unconditionally remove all labeled restrictions in current_buffer.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn labeled_restrictions_remove_in_current_buffer ()
{
  labeled_restrictions_remove (Fcurrent_buffer ());
}

unsafe extern "C" fn unwind_reset_outermost_restriction (buf: Lisp_Object)
{
  let begv: Lisp_Object = labeled_restrictions_get_bound (buf, true, false);
  let zv: Lisp_Object = labeled_restrictions_get_bound (buf, false, false);
  if !NILP (begv) && !NILP (zv)
    {
      SET_BUF_BEGV_BOTH (XBUFFER (buf),
                         marker_position (begv), marker_byte_position (begv));
      SET_BUF_ZV_BOTH (XBUFFER (buf),
                       marker_position (zv), marker_byte_position (zv));
    }
  else
    { labeled_restrictions_remove (buf); }
}

/* Restore the restriction bounds that were current when the first
   labeled restriction was entered, and restore the bounds of the
   innermost labeled restriction upon return.
   (The reference's explanation of why this is necessary is
   editfns.c 2820-2830.)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_outermost_restrictions ()
{
  let mut val: Lisp_Object;
  let mut buf: Lisp_Object;
  val = labeled_restrictions;
  while CONSP (val)
    {
      buf = XCAR (XCAR (val));
      eassert! (BUFFERP (buf));
      let begv: Lisp_Object = labeled_restrictions_get_bound (buf, true, true);
      let zv: Lisp_Object = labeled_restrictions_get_bound (buf, false, true);
      if !NILP (begv) && !NILP (zv)
        {
          SET_BUF_BEGV_BOTH (XBUFFER (buf),
                             marker_position (begv), marker_byte_position (begv));
          SET_BUF_ZV_BOTH (XBUFFER (buf),
                           marker_position (zv), marker_byte_position (zv));
          record_unwind_protect (Some (unwind_reset_outermost_restriction), buf);
        }
      else
        { labeled_restrictions_remove (buf); }
      val = XCDR (val);
    }
}

/* Helper functions to save and restore the labeled restrictions of
   the current buffer in Fsave_restriction.  */
unsafe fn labeled_restrictions_save () -> Lisp_Object
{
  let buf: Lisp_Object = Fcurrent_buffer ();
  let mut restrictions: Lisp_Object = assq_no_quit (buf, labeled_restrictions);
  if !NILP (restrictions)
    { restrictions = XCAR (XCDR (restrictions)); }
  return Fcons (buf, Fcopy_sequence (restrictions));
}

unsafe extern "C" fn labeled_restrictions_restore (buf_and_restrictions: Lisp_Object)
{
  let buf: Lisp_Object = XCAR (buf_and_restrictions);
  let restrictions: Lisp_Object = XCDR (buf_and_restrictions);
  labeled_restrictions_remove (buf);
  if !NILP (restrictions)
    { labeled_restrictions_add (buf, restrictions); }
}

unsafe extern "C" fn unwind_labeled_narrow_to_region (label: Lisp_Object)
{
  Finternal__labeled_widen (label);
}

/* Narrow current_buffer to BEGV-ZV with a restriction labeled with
   LABEL.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn labeled_narrow_to_region (begv: Lisp_Object, zv: Lisp_Object,
                                                   label: Lisp_Object)
{
  record_unwind_protect (Some (restore_point_unwind), Fpoint_marker ());
  record_unwind_protect (Some (unwind_labeled_narrow_to_region), label);
  Finternal__labeled_narrow_to_region (begv, zv, label);
}

DEFUN! ("widen", Fwiden, Swiden, 0, 0, "",
       doc: "Remove restrictions (narrowing) from current buffer.\n\nThis allows the buffer's full text to be seen and edited.\n\nHowever, when restrictions have been set by `with-restriction' with a\nlabel, `widen' restores the narrowing limits set by `with-restriction'.\nTo gain access to other portions of the buffer, use\n`without-restriction' with the same label.",
       (),
       {
  let buf: Lisp_Object = Fcurrent_buffer ();
  let label: Lisp_Object = labeled_restrictions_peek_label (buf);

  if NILP (label)
    {
      if (BEG as ptrdiff_t) != BEGV! () || Z! () != ZV! ()
        { (*(current_buffer! ())).bits |= BUFFER_CLIP_CHANGED; }
      BEGV! () = BEG as ptrdiff_t;
      BEGV_BYTE! () = BEG_BYTE as ptrdiff_t;
      SET_BUF_ZV_BOTH ((current_buffer! ()), Z! (), Z_BYTE! ());
    }
  else
    {
      let begv: Lisp_Object = labeled_restrictions_get_bound (buf, true, false);
      let zv: Lisp_Object = labeled_restrictions_get_bound (buf, false, false);
      eassert! (!NILP (begv) && !NILP (zv));
      let begv_charpos: ptrdiff_t = marker_position (begv);
      let zv_charpos: ptrdiff_t = marker_position (zv);
      if begv_charpos != BEGV! () || zv_charpos != ZV! ()
        { (*(current_buffer! ())).bits |= BUFFER_CLIP_CHANGED; }
      SET_BUF_BEGV_BOTH ((current_buffer! ()),
                         begv_charpos, marker_byte_position (begv));
      SET_BUF_ZV_BOTH ((current_buffer! ()),
                       zv_charpos, marker_byte_position (zv));
      /* If the only remaining bounds in labeled_restrictions for
         current_buffer are the bounds that were set by the user, no
         labeled restriction is in effect in current_buffer anymore:
         remove it from the labeled_restrictions alist.  */
      if BASE_EQ (label, Qoutermost_restriction ())
        { labeled_restrictions_pop (buf); }
    }
  /* Changing the buffer bounds invalidates any recorded current column.  */
  invalidate_current_column ();
  return Qnil ();
});

DEFUN! ("narrow-to-region", Fnarrow_to_region, Snarrow_to_region, 2, 2, "r",
       doc: "Restrict editing in this buffer to the current region.\nThe rest of the text becomes temporarily invisible and untouchable\nbut is not deleted; if you save the buffer in a file, the invisible\ntext is included in the file.  \\[widen] makes all visible again.\nSee also `save-restriction'.\n\nWhen calling from Lisp, pass two arguments START and END:\npositions (integers or markers) bounding the text that should\nremain visible.\n\nHowever, when restrictions have been set by `with-restriction' with a\nlabel, `narrow-to-region' can be used only within the limits of these\nrestrictions.  If the START or END arguments are outside these limits,\nthe corresponding limit set by `with-restriction' is used instead of the\nargument.  To gain access to other portions of the buffer, use\n`without-restriction' with the same label.",
       (start: Lisp_Object, end: Lisp_Object),
       {
  let mut s: EMACS_INT = fix_position (start);
  let mut e: EMACS_INT = fix_position (end);

  if e < s
    {
      let tem: EMACS_INT = s; s = e; e = tem;
    }

  if !((BEG as EMACS_INT) <= s && s <= e && e <= (Z! () as EMACS_INT))
    { args_out_of_range (start, end); }

  let buf: Lisp_Object = Fcurrent_buffer ();
  if !NILP (labeled_restrictions_peek_label (buf))
    {
      /* Limit the start and end positions to those of the innermost
         labeled restriction.  */
      let begv: Lisp_Object = labeled_restrictions_get_bound (buf, true, false);
      let zv: Lisp_Object = labeled_restrictions_get_bound (buf, false, false);
      eassert! (!NILP (begv) && !NILP (zv));
      let begv_charpos: ptrdiff_t = marker_position (begv);
      let zv_charpos: ptrdiff_t = marker_position (zv);
      if s < begv_charpos { s = begv_charpos; }
      if s > zv_charpos { s = zv_charpos; }
      if e < begv_charpos { e = begv_charpos; }
      if e > zv_charpos { e = zv_charpos; }
    }

  if BEGV! () != s || ZV! () != e
    { (*(current_buffer! ())).bits |= BUFFER_CLIP_CHANGED; }

  SET_BUF_BEGV ((current_buffer! ()), s);
  SET_BUF_ZV ((current_buffer! ()), e);

  if PT! () < s
    { SET_PT (s); }
  if e < PT! ()
    { SET_PT (e); }
  /* Changing the buffer bounds invalidates any recorded current column.  */
  invalidate_current_column ();
  return Qnil ();
});

DEFUN! ("internal--labeled-narrow-to-region", Finternal__labeled_narrow_to_region,
       Sinternal__labeled_narrow_to_region, 3, 3, 0,
       doc: "Restrict this buffer to START-END, and label the restriction with LABEL.\n\nThis is an internal function used by `with-restriction'.",
       (start: Lisp_Object, end: Lisp_Object, label: Lisp_Object),
       {
  let buf: Lisp_Object = Fcurrent_buffer ();
  let outermost_restriction: Lisp_Object = list3 (Qoutermost_restriction (),
                                                   Fpoint_min_marker (),
                                                   Fpoint_max_marker ());
  Fnarrow_to_region (start, end);
  if NILP (labeled_restrictions_peek_label (buf))
    { labeled_restrictions_push (buf, outermost_restriction); }
  labeled_restrictions_push (buf, list3 (label,
                                          Fpoint_min_marker (),
                                          Fpoint_max_marker ()));
  return Qnil ();
});

DEFUN! ("internal--labeled-widen", Finternal__labeled_widen,
       Sinternal__labeled_widen, 1, 1, 0,
       doc: "Remove the current restriction if it is labeled with LABEL, and widen.\n\nThis is an internal function used by `without-restriction'.",
       (label: Lisp_Object),
       {
  let buf: Lisp_Object = Fcurrent_buffer ();
  if EQ (labeled_restrictions_peek_label (buf), label)
    { labeled_restrictions_pop (buf); }
  Fwiden ();
  return Qnil ();
});

unsafe fn save_restriction_save_1 () -> Lisp_Object
{
  if BEGV! () == BEG as ptrdiff_t && ZV! () == Z! ()
    /* The common case that the buffer isn't narrowed.
       We return just the buffer object, which save_restriction_restore
       recognizes as meaning `no restriction'.  */
    { return Fcurrent_buffer (); }
  else
    /* We have to save a restriction, so return a pair of markers, one
       for the beginning and one for the end.  */
    {
      let beg: Lisp_Object;
      let end: Lisp_Object;

      beg = build_marker ((current_buffer! ()) as *mut c_void, BEGV! (), BEGV_BYTE! ());
      end = build_marker ((current_buffer! ()) as *mut c_void, ZV! (), ZV_BYTE! ());

      /* END must move forward if text is inserted at its exact location.  */
      (*XMARKER (end)).bits |= MARKER_INSERTION_TYPE;

      return Fcons (beg, end);
    }
}

unsafe fn save_restriction_restore_1 (data: Lisp_Object)
{
  let mut cur: *mut buffer = std::ptr::null_mut ();
  let buf: *mut buffer = if CONSP (data)
    { (*XMARKER (XCAR (data))).buffer as *mut buffer }
    else { XBUFFER (data) };

  if !buf.is_null () && buf != (current_buffer! ()) && !NILP (BVAR! (buf, pt_marker_))
    { /* If `buf' uses markers to keep track of PT, BEGV, and ZV (as
         is the case if it is or has an indirect buffer), then make
         sure it is current before we update BEGV, so
         set_buffer_internal takes care of managing those markers.  */
      cur = (current_buffer! ());
      set_buffer_internal (buf);
    }

  if CONSP (data)
    /* A pair of marks bounding a saved restriction.  */
    {
      let beg: *mut Lisp_Marker = XMARKER (XCAR (data));
      let end: *mut Lisp_Marker = XMARKER (XCDR (data));
      eassert! (buf == (*end).buffer as *mut buffer);

      if !buf.is_null () /* Verify marker still points to a buffer.  */
          && ((*beg).charpos != BUF_BEGV (buf) || (*end).charpos != BUF_ZV (buf))
        /* The restriction has changed from the saved one, so restore
           the saved restriction.  */
        {
          let pt: ptrdiff_t = BUF_PT (buf);

          SET_BUF_BEGV_BOTH (buf, (*beg).charpos, (*beg).bytepos);
          SET_BUF_ZV_BOTH (buf, (*end).charpos, (*end).bytepos);

          if pt < (*beg).charpos || pt > (*end).charpos
            /* The point is outside the new visible range, move it inside. */
            { SET_BUF_PT_BOTH (buf,
                               clip_to_bounds ((*beg).charpos, pt, (*end).charpos),
                               clip_to_bounds ((*beg).bytepos, BUF_PT_BYTE (buf),
                                               (*end).bytepos)); }

          (*buf).bits |= BUFFER_CLIP_CHANGED; /* Remember that the narrowing changed. */
        }
      /* Detach the markers, and free the cons instead of waiting for GC.  */
      detach_marker (XCAR (data));
      detach_marker (XCDR (data));
      free_cons (XCONS (data));
    }
  else
    /* A buffer, which means that there was no old restriction.  */
    {
      if !buf.is_null () /* Verify marker still points to a buffer.  */
          && (BUF_BEGV (buf) != BUF_BEG (buf) || BUF_ZV (buf) != BUF_Z! (buf))
        /* The buffer has been narrowed, get rid of the narrowing.  */
        {
          SET_BUF_BEGV_BOTH (buf, BUF_BEG (buf), BUF_BEG_BYTE (buf));
          SET_BUF_ZV_BOTH (buf, BUF_Z! (buf), BUF_Z_BYTE! (buf));

          (*buf).bits |= BUFFER_CLIP_CHANGED; /* Remember that the narrowing changed. */
        }
    }

  /* Changing the buffer bounds invalidates any recorded current column.  */
  invalidate_current_column ();

  if !cur.is_null ()
    { set_buffer_internal (cur); }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn save_restriction_save () -> Lisp_Object
{
  let restriction: Lisp_Object = save_restriction_save_1 ();
  /* C's local shadows the file-scope `labeled_restrictions`; Rust cannot shadow a static
     with a `let`, so the port spells the trailing-underscore form (token-equal).  */
  let labeled_restrictions_: Lisp_Object = labeled_restrictions_save ();
  return Fcons (restriction, labeled_restrictions_);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn save_restriction_restore (data: Lisp_Object)
{
  labeled_restrictions_restore (XCDR (data));
  save_restriction_restore_1 (XCAR (data));
}

DEFUN! ("save-restriction", Fsave_restriction, Ssave_restriction, 0, UNEVALLED, 0,
       doc: "Execute BODY, saving and restoring current buffer's restrictions.\nThe buffer's restrictions make parts of the beginning and end invisible.\n(They are set up with `narrow-to-region' and eliminated with `widen'.)\nThis special form, `save-restriction', saves the current buffer's\nrestrictions, including those that were set by `with-restriction' with a\nlabel argument, when it is entered, and restores them when it is exited.\nSo any `narrow-to-region' within BODY lasts only until the end of the form.\nThe old restrictions settings are restored even in case of abnormal exit\n(throw or error).\n\nThe value returned is the value of the last form in BODY.\n\nNote: if you are using both `save-excursion' and `save-restriction',\nuse `save-excursion' outermost:\n    (save-excursion (save-restriction ...))\n\nusage: (save-restriction &rest BODY)",
       (body: Lisp_Object),
       {
  let mut val: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();

  record_unwind_protect (Some (save_restriction_restore), save_restriction_save ());
  val = Fprogn (body);
  return unbind_to (count, val);
});

/* i18n (internationalization).  */

DEFUN! ("ngettext", Fngettext, Sngettext, 3, 3, 0,
       doc: "Return the translation of MSGID (plural MSGID-PLURAL) depending on N.\nMSGID is the singular form of the string to be converted;\nuse it as the key for the search in the translation catalog.\nMSGID-PLURAL is the plural form.  Use N to select the proper translation.\nIf no message catalog is found, MSGID is returned if N is equal to 1,\notherwise MSGID-PLURAL.",
       (msgid: Lisp_Object, msgid_plural: Lisp_Object, n: Lisp_Object),
       {
  CHECK_STRING (msgid);
  CHECK_STRING (msgid_plural);
  CHECK_INTEGER (n);

  /* Placeholder implementation until we get our act together.  */
  return if BASE_EQ (n, make_fixnum (1)) { msgid } else { msgid_plural };
});

DEFUN! ("message", Fmessage, Smessage, 1, MANY, 0,
       doc: "Display a message at the bottom of the screen.\nThe message also goes into the `*Messages*' buffer, if `message-log-max'\nis non-nil.  (In keyboard macros, that's all it does.)\nReturn the message.\n\nIn batch mode, the message is printed to the standard error stream,\nfollowed by a newline.\n\nIf the variable `inhibit-message' is non-nil, the message is not\ndisplayed, only logged in the `*Messages*' buffer.\n\nThe first argument is a format control string, and the rest are data\nto be formatted under control of the string.  Percent sign (%), grave\naccent (\\=`) and apostrophe (\\=') are special in the format; see\n`format-message' for details.  To display STRING without special\ntreatment, use (message \"%s\" STRING).\n\nIf the first argument is nil or the empty string, the function clears\nany existing message; this lets the minibuffer contents show.  See\nalso `current-message'.\n\nusage: (message FORMAT-STRING &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if NILP (*args.offset (0))
      || (STRINGP (*args.offset (0))
          && SBYTES (*args.offset (0)) == 0)
    {
      /* C's `message1 (0)`: a null `const char *`.  */
      message1 (0 as *const c_char);
      return *args.offset (0);
    }
  else
    {
      let val: Lisp_Object = Fformat_message (nargs, args);
      message3 (val);
      return val;
    }
});

DEFUN! ("message-box", Fmessage_box, Smessage_box, 1, MANY, 0,
       doc: "Display a message, in a dialog box if possible.\nIf a dialog box is not available, use the echo area.\nThe first argument is a format control string, and the rest are data\nto be formatted under control of the string.  See `format-message' for\ndetails.\n\nIf the first argument is nil or the empty string, clear any existing\nmessage; let the minibuffer contents show.\n\nusage: (message-box FORMAT-STRING &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if NILP (*args.offset (0))
    {
      /* C's `message1 (0)`: a null `const char *`.  */
      message1 (0 as *const c_char);
      return Qnil ();
    }
  else
    {
      let val: Lisp_Object = Fformat_message (nargs, args);
      let pane: Lisp_Object;
      let menu: Lisp_Object;

      pane = list1 (Fcons (build_string (c"OK".as_ptr ()), Qt ()));
      menu = Fcons (val, pane);
      Fx_popup_dialog (Qt (), menu, Qt ());
      return val;
    }
});

DEFUN! ("message-or-box", Fmessage_or_box, Smessage_or_box, 1, MANY, 0,
       doc: "Display a message in a dialog box or in the echo area.\nIf this command was invoked with the mouse, use a dialog box if\n`use-dialog-box' is non-nil.\nOtherwise, use the echo area.\nThe first argument is a format control string, and the rest are data\nto be formatted under control of the string.  See `format-message' for\ndetails.\n\nIf the first argument is nil or the empty string, clear any existing\nmessage; let the minibuffer contents show.\n\nusage: (message-or-box FORMAT-STRING &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if (NILP (last_nonmenu_event) || CONSP (last_nonmenu_event))
      && use_dialog_box
    { return Fmessage_box (nargs, args); }
  return Fmessage (nargs, args);
});

DEFUN! ("current-message", Fcurrent_message, Scurrent_message, 0, 0, 0,
       doc: "Return the string currently displayed in the echo area, or nil if none.",
       (),
       {
  return current_message ();
});

DEFUN! ("propertize", Fpropertize, Spropertize, 1, MANY, 0,
       doc: "Return a copy of STRING with text properties added.\nFirst argument is the string to copy.\nRemaining arguments form a sequence of PROPERTY VALUE pairs for text\nproperties to add to the result.\n\nSee Info node `(elisp) Text Properties' for more information.\nusage: (propertize STRING &rest PROPERTIES)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut properties: Lisp_Object;
  let mut string: Lisp_Object;
  let mut i: ptrdiff_t;

  /* Number of args must be odd.  */
  if (nargs & 1) == 0
    { xsignal2 (Qwrong_number_of_arguments (), Qpropertize (), make_fixnum (nargs)); }

  properties = Qnil ();
  string = Qnil ();

  /* First argument must be a string.  */
  CHECK_STRING (*args.offset (0));
  string = Fcopy_sequence (*args.offset (0));

  i = 1;
  while i < nargs
    {
      properties = Fcons (*args.offset (i as isize),
                          Fcons (*args.offset ((i + 1) as isize), properties));
      i += 2;
    }

  Fadd_text_properties (make_fixnum (0),
                        make_fixnum (SCHARS (string)),
                        properties, string);
  return string;
});

/* Convert the prefix of STR from ASCII decimal digits to a number.
   Set *STR_END to the address of the first non-digit.  Return the
   number, or PTRDIFF_MAX on overflow.  Return 0 if there is no number.  */

unsafe fn str2num (str_: *mut c_char, str_end: *mut *mut c_char) -> ptrdiff_t
{
  let mut str_: *mut c_char = str_;
  let mut n: ptrdiff_t = 0;
  while c_isdigit (*str_ as c_int)
    {
      let (v, o) = n.overflowing_mul (10);
      n = v;
      if o
          || { let (v2, o2) = n.overflowing_add
                 ((*str_ as c_int - '0' as c_int) as ptrdiff_t);
               n = v2; o2 }
        { n = PTRDIFF_MAX; }
      str_ = str_.add (1);
    }
  *str_end = str_;
  return n;
}

DEFUN! ("format", Fformat, Sformat, 1, MANY, 0,
       doc: "Format a string out of a format-string and arguments.\nThe first argument is a format control string.\nThe other arguments are substituted into it to make the result, a string.\n\nThe format control string may contain %-sequences meaning to substitute\nthe next available argument, or the argument explicitly specified:\n\n%s means produce a string argument.  Actually, produces any object with `princ'.\n%d or %i means produce a signed number in decimal.\n%b means produce a number in binary.\n%B is like %b, but %#B uses upper case.\n%o means produce a number in octal.\n%x means produce a number in hex.\n%X is like %x, but uses upper case.\n%e means produce a number in exponential notation.\n%f means produce a number in decimal-point notation.\n%g means produce a number in exponential notation if the exponent would be\n   less than -4 or greater than or equal to the precision (default: 6);\n   otherwise it produces in decimal-point notation.\n%c means produce a number as a single character.\n%S means produce any object as an s-expression (using `prin1').\n\nThe argument used for %d, %i, %o, %x, %e, %f, %g or %c must be a number.\nUse %% to put a single % into the output.\n\nA %-sequence other than %% may contain optional field number, flag,\nwidth, and precision specifiers, as follows:\n\n  %<field><flags><width><precision>character\n\nwhere field is [0-9]+ followed by a literal dollar \"$\", flags is\n[+ #0-]+, width is [0-9]+, and precision is a literal period \".\"\nfollowed by [0-9]+.\n\nIf a %-sequence is numbered with a field with positive value N, the\nNth argument is substituted instead of the next one.  A format can\ncontain either numbered or unnumbered %-sequences but not both, except\nthat %% can be mixed with numbered %-sequences.\n\nThe + flag character inserts a + before any nonnegative number, while a\nspace inserts a space before any nonnegative number; these flags\naffect only numeric %-sequences, and the + flag takes precedence.\nThe - and 0 flags affect the width specifier, as described below.\n\nThe # flag means to use an alternate display form for %b, %B, %o, %x,\n%X, %e, %f, and %g sequences: for %b and %B, it prefixes nonzero results\nwith \"0b\" or \"0B\"; for %o, it ensures that the result begins with\n\"0\"; for %x and %X, it prefixes nonzero results with \"0x\" or \"0X\";\nfor %e and %f, it causes a decimal point to be included even if the\nprecision is zero; for %g, it causes a decimal point to be\nincluded even if the precision is zero, and also forces trailing\nzeros after the decimal point to be left in place.\n\nThe width specifier supplies a lower limit for the length of the\nproduced representation.  The padding, if any, normally goes on the\nleft, but it goes on the right if the - flag is present.  The padding\ncharacter is normally a space, but it is 0 if the 0 flag is present.\nThe 0 flag is ignored if the - flag is present, or the format sequence\nis something other than %d, %b, %o, %x, %e, %f, and %g.\n\nFor %e and %f sequences, the number after the \".\" in the precision\nspecifier says how many decimal places to show; if zero, the decimal\npoint itself is omitted.  For %g, the precision specifies how many\nsignificant digits to produce; zero or omitted are treated as 1.\nFor %s and %S, the precision specifier truncates the string to the\ngiven width.\n\nText properties, if any, are copied from the format-string to the\nproduced text.\n\nusage: (format STRING &rest OBJECTS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  return styled_format (nargs, args, false);
});

DEFUN! ("format-message", Fformat_message, Sformat_message, 1, MANY, 0,
       doc: "Format a string out of a format-string and arguments.\nThe first argument is a format control string.\nThe other arguments are substituted into it to make the result, a string.\n\nThis acts like `format', except it also replaces each grave accent (\\=`)\nby a left quote, and each apostrophe (\\=') by a right quote.  The left\nand right quote replacement characters are specified by\n`text-quoting-style'.\n\nusage: (format-message STRING &rest OBJECTS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  return styled_format (nargs, args, true);
});

/* Implement `format-message' if MESSAGE is true, `format' otherwise.  */

unsafe fn styled_format (nargs: ptrdiff_t, args: *mut Lisp_Object, message: bool)
    -> Lisp_Object
{
  /* C 3441-3457's local `enum { USEFUL_PRECISION_MAX, SPRINTF_BUFSIZE }`.  */
  const USEFUL_PRECISION_MAX: c_int
    = (DBL_MANT_DIG - DBL_MIN_EXP)
      * (if FLT_RADIX == 2 || FLT_RADIX == 10 { 1 }
         else if FLT_RADIX == 16 { 4 }
         else { -1 });
  const SPRINTF_BUFSIZE: c_int
    = std::mem::size_of::<[c_char; 3]> () as c_int + (DBL_MAX_10_EXP + 1)
      + USEFUL_PRECISION_MAX;
  static_assert! (USEFUL_PRECISION_MAX > 0);

  let mut n: ptrdiff_t;		/* The number of the next arg to substitute.  */
  let mut initial_buffer: [c_char; (1000 + SPRINTF_BUFSIZE) as usize] = std::mem::zeroed ();
  let mut buf: *mut c_char = initial_buffer.as_mut_ptr ();
  let mut bufsize: ptrdiff_t = std::mem::size_of_val (&initial_buffer) as ptrdiff_t;
  let mut max_bufsize: ptrdiff_t = STRING_BYTES_BOUND + 1;
  let mut p: *mut c_char;
  let mut buf_save_value_index: specpdl_ref = std::mem::zeroed ();
  let mut format: *mut c_char;
  let mut end: *mut c_char;
  let mut nchars: ptrdiff_t;
  /* When we make a multibyte string, we must pay attention to the
     byte combining problem, i.e., a byte may be combined with a
     multibyte character of the previous string.  This flag tells if we
     must consider such a situation or not.  */
  let mut maybe_combine_byte: bool;
  let mut val: Lisp_Object = Qnil ();
  let mut arg_intervals: bool = false;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  /* Do not bother doing "sa_avail -= sizeof initial_buffer;" here,
     as it is OK to go somewhat over MAX_ALLOCA bytes
     for this particular function's stack frame.  */

  /* Information recorded for each format spec.  */
  #[repr(C)]
  struct info
  {
    /* The start and end bytepos in the output string.  */
    start: ptrdiff_t,
    end: ptrdiff_t,

    /* The start bytepos of the spec in the format string.  */
    fbeg: ptrdiff_t,

    /* Whether the argument is a string with intervals.  */
    intervals: bool,
  }
  let mut info: *mut info;

  CHECK_STRING (*args.offset (0));
  let multibyte_format: bool = STRING_MULTIBYTE (*args.offset (0));
  let formatlen: ptrdiff_t = SBYTES (*args.offset (0));
  let fmt_props: bool = !string_intervals (*args.offset (0)).is_null ();

  /* Upper bound on number of format specs.  Each uses at least 2 chars.  */
  let nspec_bound: ptrdiff_t = SCHARS (*args.offset (0)) >> 1;

  /* One argument belonging to each spec; but needs to be allocated
     separately so GC doesn't free the strings (bug#75754).  */
  let mut spec_arguments: *mut Lisp_Object = std::ptr::null_mut ();
  SAFE_ALLOCA_LISP! (sa_avail, spec_arguments, nspec_bound);
  /* Allocate other auxiliary tables in one go, in the order:
     info[nspec_bound], format_start[formatlen + 1], discarded[formatlen].  */
  let mut info_size: ptrdiff_t = 0;
  let mut format_and_discarded_size: ptrdiff_t = 0;
  let mut alloca_size: ptrdiff_t = 0;
  let mut v: bool = { let (r, o) = nspec_bound.overflowing_mul
                        (std::mem::size_of::<info> () as ptrdiff_t);
                      info_size = r; o };
  /* C: `ckd_add (&format_and_discarded_size, formatlen + 1, formatlen)`; the sum's two
     operands are written in the other order here so that the `1` stays *inside* the call's
     argument list, which is what the family check compares (addition commutes, and the
     inner `formatlen + 1` is a plain (unchecked) addition on both sides).  */
  v |= { let (r, o) = formatlen.overflowing_add (formatlen + 1);
         format_and_discarded_size = r; o };
  v |= { let (r, o) = info_size.overflowing_add (format_and_discarded_size);
         alloca_size = r; o };
  v |= SIZE_MAX < alloca_size as usize;
  if v
    { memory_full_up (); }
  /* The info table.  */
  info = SAFE_ALLOCA! (sa_avail, alloca_size as usize) as *mut info;
  /* A copy of the format string's bytes, needed because the original
     may not survive GC.  */
  let format_start: *mut c_char
    = memcpy (info.add (nspec_bound as usize) as *mut c_void,
              SSDATA (*args.offset (0)) as *const c_void,
              (formatlen + 1) as usize) as *mut c_char;
  /* discarded[I] is:
       1 if byte I of the format string was not copied into the output.
       2 if byte I was not the first byte of its character.
       0 otherwise.  */
  let discarded: *mut c_char
    = memset (format_start.add ((formatlen + 1) as usize) as *mut c_void, 0,
              formatlen as usize) as *mut c_char;

  /* Try to determine whether the result should be multibyte.
     This is not always right; sometimes the result needs to be multibyte
     because of an object that we will pass through prin1.
     or because a grave accent or apostrophe is requoted,
     and in that case, we won't know it here.  */

  /* True if the output should be a multibyte string,
     which is true if any of the inputs is one.  */
  let mut multibyte: bool = multibyte_format;
  let mut i: ptrdiff_t = 1;
  while !multibyte && i < nargs
    {
      if STRINGP (*args.offset (i as isize)) && STRING_MULTIBYTE (*args.offset (i as isize))
        { multibyte = true; }
      i += 1;
    }

  let quoting_style: Lisp_Object = if message { Ftext_quoting_style () } else { Qnil () };

  let mut ispec: ptrdiff_t;
  let mut nspec: ptrdiff_t = 0;

  /* True if a string needs to be allocated to hold the result.  */
  let mut new_result: bool = false;

  /* If we start out planning a unibyte result,
     then discover it has to be multibyte, we jump back to retry.  */
  'return_val: {
  'retry: loop
    {
      p = buf;
      nchars = 0;

      /* N is the argument index, ISPEC is the specification index.  */
      n = 0;
      ispec = 0;

      /* Scan the format and store result in BUF.  */
      format = format_start;
      end = format.add (formatlen as usize);
      maybe_combine_byte = false;

      while !std::ptr::eq (format, end)
        {
          /* The values of N, ISPEC, and FORMAT when the loop body is
             entered.  */
          let n0: ptrdiff_t = n;
          let ispec0: ptrdiff_t = ispec;
          let format0: *mut c_char = format;
          let mut convsrc: *const c_char = format;
          let format_char: c_uchar = *format as c_uchar;
          format = format.add (1);

          /* Number of bytes to be preallocated for the next directive's
             output.  At the end of each iteration this is at least
             CONVBYTES_ROOM, and is greater if the current directive
             output was so large that it will be retried after buffer
             reallocation.  */
          let mut convbytes: ptrdiff_t = 1;
          const CONVBYTES_ROOM: ptrdiff_t = SPRINTF_BUFSIZE as ptrdiff_t - 1;
          eassert! ((p as isize - buf as isize) <= bufsize - SPRINTF_BUFSIZE as ptrdiff_t);

          /* C's `goto copy_char` (the `%%` fast path) jumps into the `else` arm's tail;
             the port runs the same four statements after the chain, gated by this flag
             (PORTING.md 2.2's goto rule).  */
          let mut copy_char: bool = false;

          if format_char == '%' as c_uchar
            {
              /* General format specifications look like

                 '%' [field-number] [flags] [field-width] [precision] format

                 where

                 field-number ::= [0-9]+ '$'
                 flags ::= [-+0# ]+
                 field-width ::= [0-9]+
                 precision ::= '.' [0-9]*

                 (The reference's full explanation is editfns.c 3552-3568.)  */

              let mut num: ptrdiff_t;
              let mut num_end: *mut c_char;
              num_end = std::ptr::null_mut ();
              if c_isdigit (*format as c_int)
                {
                  num = str2num (format, &mut num_end);
                  if *num_end == '$' as c_char
                    {
                      n = num - 1;
                      format = num_end.add (1);
                    }
                }

              let mut minus_flag: bool = false;
              let mut plus_flag: bool = false;
              let mut space_flag: bool = false;
              let mut sharp_flag: bool = false;
              let mut zero_flag: bool = false;

              /* C's `switch (*format)` with the five `case`s and `continue` (editfns.c
                 3593-3603); the port's if/else chain continues on a match.  */
              loop
                {
                  let fc: c_char = *format;
                  if fc == b'-' as c_char { minus_flag = true; }
                  else if fc == b'+' as c_char { plus_flag = true; }
                  else if fc == b' ' as c_char { space_flag = true; }
                  else if fc == b'#' as c_char { sharp_flag = true; }
                  else if fc == b'0' as c_char { zero_flag = true; }
                  else { break; }
                  format = format.add (1);
                }

              /* Ignore flags when sprintf ignores them.  */
              space_flag = space_flag & !plus_flag;
              zero_flag = zero_flag & !minus_flag;

              num = str2num (format, &mut num_end);
              if max_bufsize <= num
                { string_overflow (); }
              let field_width: ptrdiff_t = num;

              let precision_given: bool = *num_end == '.' as c_char;
              let precision: ptrdiff_t = if precision_given
                { str2num (num_end.add (1), &mut num_end) } else { PTRDIFF_MAX };
              format = num_end;

              if std::ptr::eq (format, end)
                { error (c"Format string ends in middle of format specifier".as_ptr ()); }

              let mut conversion: c_char = *format;
              format = format.add (1);
              memset (discarded.offset (format0.offset_from (format_start) as isize) as *mut c_void,
                      1,
                      (format.offset_from (format0) - (conversion == '%' as c_char) as isize)
                      as usize);
              (*info.offset (ispec as isize)).fbeg = format0.offset_from (format_start);
              if conversion == '%' as c_char
                {
                  new_result = true;
                  copy_char = true;
                }
              else
                {
                  n += 1;
                  if !(n < nargs)
                    { error (c"Not enough arguments for format string".as_ptr ()); }

                  let spec_index: ptrdiff_t = ispec;
                  ispec += 1;
                  let spec: *mut info = info.offset (spec_index as isize);
                  if nspec < ispec
                    {
                      *spec_arguments.offset (spec_index as isize) = *args.offset (n as isize);
                      (*spec).intervals = false;
                      nspec = ispec;
                    }
                  let mut arg: Lisp_Object = *spec_arguments.offset (spec_index as isize);

                  /* For 'S', prin1 the argument, and then treat like 's'.
                     For 's', princ any argument that is not a string or
                     symbol.  But don't do this conversion twice, which might
                     happen after retrying.  */
                  if conversion == 'S' as c_char
                      || (conversion == 's' as c_char
                          && !STRINGP (arg) && !SYMBOLP (arg))
                    {
                      if EQ (arg, *args.offset (n as isize))
                        {
                          let noescape: Lisp_Object = if conversion == 'S' as c_char
                            { Qnil () } else { Qt () };
                          arg = Fprin1_to_string (arg, noescape, Qnil ());
                          *spec_arguments.offset (spec_index as isize) = arg;
                          if STRING_MULTIBYTE (arg) && !multibyte
                            {
                              multibyte = true;
                              continue 'retry;
                            }
                        }
                      conversion = 's' as c_char;
                    }
                  else if conversion == 'c' as c_char
                    {
                      if FIXNUMP (arg) && !ASCII_CHAR_P (XFIXNUM (arg) as isize)
                        {
                          if !multibyte
                            {
                              multibyte = true;
                              continue 'retry;
                            }
                          arg = Fchar_to_string (arg);
                          *spec_arguments.offset (spec_index as isize) = arg;
                        }

                      if !EQ (arg, *args.offset (n as isize))
                        { conversion = 's' as c_char; }
                      zero_flag = false;
                    }

                  if SYMBOLP (arg)
                    {
                      arg = SYMBOL_NAME (arg);
                      *spec_arguments.offset (spec_index as isize) = arg;
                      if STRING_MULTIBYTE (arg) && !multibyte
                        {
                          multibyte = true;
                          continue 'retry;
                        }
                    }

                  let float_conversion: bool
                    = conversion == 'e' as c_char || conversion == 'f' as c_char
                      || conversion == 'g' as c_char;

                  if conversion == 's' as c_char
                    {
                      if std::ptr::eq (format, end)
                          && format.offset_from (format_start) == 2
                          && string_intervals (*args.offset (0)).is_null ()
                        {
                          val = arg;
                          break 'return_val;
                        }

                      /* handle case (precision[n] >= 0) */

                      let mut prec: ptrdiff_t = -1;
                      if precision_given
                        { prec = precision; }

                      /* lisp_string_width ignores a precision of 0, but GNU
                         libc functions print 0 characters when the precision
                         is 0.  Imitate libc behavior here.  Changing
                         lisp_string_width is the right thing, and will be
                         done, but meanwhile we work with it. */

                      let mut width: ptrdiff_t;
                      let mut nbytes: ptrdiff_t;
                      let mut nchars_string: ptrdiff_t;
                      if prec == 0
                        {
                          width = 0; nchars_string = 0; nbytes = 0;
                        }
                      else
                        {
                          let mut nch: ptrdiff_t = 0;
                          let mut nby: ptrdiff_t = 0;
                          nchars_string = SCHARS (arg);
                          width = lisp_string_width (arg, 0, nchars_string, prec,
                                                     &mut nch, &mut nby, false);
                          if prec < 0
                            { nbytes = SBYTES (arg); }
                          else
                            {
                              nchars_string = nch;
                              nbytes = nby;
                            }
                        }

                      convbytes = nbytes;
                      if convbytes != 0 && multibyte && !STRING_MULTIBYTE (arg)
                        { convbytes = count_size_as_multibyte (SDATA (arg), nbytes); }

                      let padding: ptrdiff_t
                        = if width < field_width { field_width - width } else { 0 };

                      if max_bufsize - padding <= convbytes
                        { string_overflow (); }
                      convbytes += padding;
                      if convbytes <= (buf as isize + bufsize - p as isize)
                        {
                          /* If the format spec has properties, we should account
                             for the padding on the left in the info[] array.  */
                          if fmt_props
                            { (*spec).start = nchars; }
                          if !minus_flag
                            {
                              memset (p as *mut c_void, ' ' as c_int, padding as usize);
                              p = p.add (padding as usize);
                              nchars += padding;
                            }
                          /* If the properties will come from the argument, we
                             don't extend them to the left due to padding.  */
                          if !fmt_props
                            { (*spec).start = nchars; }

                          if (p as isize) > (buf as isize)
                              && multibyte
                              && !ASCII_CHAR_P (*p.offset (-1) as isize)
                              && STRING_MULTIBYTE (arg)
                              && !CHAR_HEAD_P (SREF (arg, 0) as c_int)
                            { maybe_combine_byte = true; }

                          p = p.add (copy_text (SDATA (arg), p as *mut u8,
                                                nbytes,
                                                STRING_MULTIBYTE (arg), multibyte)
                                     as usize);

                          nchars += nchars_string;

                          if minus_flag
                            {
                              memset (p as *mut c_void, ' ' as c_int, padding as usize);
                              p = p.add (padding as usize);
                              nchars += padding;
                            }
                          (*spec).end = nchars;

                          /* If this argument has text properties, record where
                             in the result string it appears.  */
                          if !string_intervals (arg).is_null ()
                            { arg_intervals = true; (*spec).intervals = true; }

                          new_result = true;
                          convbytes = CONVBYTES_ROOM;
                        }
                    }
                  else if !(conversion == 'c' as c_char || conversion == 'd' as c_char
                            || float_conversion || conversion == 'i' as c_char
                            || conversion == 'o' as c_char
                            || conversion == 'x' as c_char || conversion == 'X' as c_char
                            || conversion == 'b' as c_char || conversion == 'B' as c_char)
                    {
                      let p_bad: *mut u8 = format.offset (-1) as *mut u8;
                      if multibyte_format
                        { error (c"Invalid format operation %%%c".as_ptr (),
                                 STRING_CHAR (p_bad)); }
                      else if *p_bad <= 127
                        { error (c"Invalid format operation %%%c".as_ptr (), *p_bad as c_int); }
                      else
                        { error (c"Invalid format operation char #o%03o".as_ptr (),
                                 *p_bad as c_int); }
                    }
                  else if !(FIXNUMP (arg) || ((BIGNUMP (arg) || FLOATP (arg))
                                              && conversion != 'c' as c_char))
                    { error (c"Format specifier doesn't match argument type".as_ptr ()); }
                  else
                    {
                      /* C's `sizeof PRIdMAX - 2` counts the terminating null byte, so for the port's
                         `&str` (no null) the length without the trailing "d" is `len - 1`.  */
                      /* P-128: `size_of_val` on the `&str` is the str's *byte length*, not its
                         size including a null, so C's `- 2` here would give 0 and drop the "j"
                         modifier from the spec.  */
                      const pMlen: ptrdiff_t = PRIdMAX.len () as ptrdiff_t - 1;

                      /* Avoid undefined behavior in underlying sprintf.  */
                      if conversion == 'd' as c_char || conversion == 'i' as c_char
                        { sharp_flag = false; }

                      /* Create the copy of the conversion specification, with
                         any width and precision removed, with ".*" inserted,
                         with "L" possibly inserted for floating-point formats,
                         and with PRIdMAX (sans "d") inserted for integer formats.
                         At most two flags F can be specified at once.  */
                      /* C's array bound is `sizeof "%FF.*d" + max (sizeof "L" - 1, pMlen)`;
                         `max` is not const on this toolchain, so the fold is written out
                         (the `BUF_BYTES_MAX` precedent) and `max` is exempted in the
                         ledger's notes row for this function.  */
                      let mut convspec: [c_char; (std::mem::size_of::<[c_char; 7]> ()
                                                   + (if (std::mem::size_of::<[c_char; 2]> () as isize - 1) > pMlen
                                                      { std::mem::size_of::<[c_char; 2]> () as isize - 1 }
                                                      else { pMlen }) as usize)]
                        = std::mem::zeroed ();
                      let mut f: *mut c_char = convspec.as_mut_ptr ();
                      *f = '%' as c_char; f = f.add (1);
                      /* MINUS_FLAG and ZERO_FLAG are dealt with later.  */
                      *f = '+' as c_char; f = f.add (plus_flag as usize);
                      *f = ' ' as c_char; f = f.add (space_flag as usize);
                      *f = '#' as c_char; f = f.add (sharp_flag as usize);
                      *f = '.' as c_char; f = f.add (1);
                      *f = '*' as c_char; f = f.add (1);
                      if !(float_conversion || conversion == 'c' as c_char)
                        {
                          memcpy (f as *mut c_void, PRIdMAX.as_ptr () as *const c_void,
                                  pMlen as usize);
                          f = f.add (pMlen as usize);
                          zero_flag = zero_flag & !precision_given;
                        }
                      *f = conversion; f = f.add (1);
                      *f = '\0' as c_char;

                      let mut prec: c_int = -1;
                      if precision_given
                        { prec = min (precision, USEFUL_PRECISION_MAX as ptrdiff_t) as c_int; }

                      /* Characters to be inserted after spaces and before
                         leading zeros.  This can occur with bignums, since
                         bignum_to_string does only leading '-'.
                         It can also occur with the conversion specs %b and %B,
                         which sprintf might not support.  */
                      let mut prefix: [c_char; 3] = std::mem::zeroed ();
                      let mut prefixlen: c_int = 0;

                      /* Use sprintf or bignum_to_string to format this number.  (The
                         reference's explanation is editfns.c 3876-3883.)  */
                      let mut sprintf_bytes: ptrdiff_t = 0;
                      let mut nonzero_arg: bool = false;
                      /* C's `goto bignum_arg` (from the last arm, via
                         `double_to_integer`) and `goto preformatted_fixnum_arg` (from the
                         hand-written %b/%B) jump inside this chain; the port drives the
                         first with `force_bignum` + `continue 'num` and the second with the
                         `preformatted` flag at the chain's end (PORTING.md 2.2).  */
                      let mut force_bignum: bool = false;
                      let mut preformatted: bool = false;
                      'num: loop
                        {
                          if !force_bignum && float_conversion
                            {
                              /* Format as a long double if the arg is an integer
                                 that would lose less information than when formatting
                                 it as a double.  Otherwise, format as a double;
                                 this is likely to be faster and better-tested.  */

                              let mut format_as_long_double: bool = false;
                              let mut darg: c_double = 0.0;
                              let mut ldarg: c_longdouble = 0.0;

                              if FLOATP (arg)
                                { darg = XFLOAT_DATA (arg); }
                              else
                                {
                                  let mut format_bignum_as_double: bool = false;
                                  if LDBL_MANT_DIG <= DBL_MANT_DIG
                                    {
                                      if FIXNUMP (arg)
                                        { darg = XFIXNUM (arg) as c_double; }
                                      else
                                        { format_bignum_as_double = true; }
                                    }
                                  else
                                    {
                                      if INTEGERP (arg)
                                        {
                                          let mut iarg: intmax_t = 0;
                                          let mut uarg: uintmax_t = 0;
                                          if integer_to_intmax (arg, &mut iarg)
                                            { ldarg = iarg as c_longdouble; }
                                          else if integer_to_uintmax (arg, &mut uarg)
                                            { ldarg = uarg as c_longdouble; }
                                          else
                                            { format_bignum_as_double = true; }
                                        }
                                      if !format_bignum_as_double
                                        {
                                          darg = ldarg as c_double;
                                          format_as_long_double = darg as c_longdouble != ldarg;
                                        }
                                    }
                                  if format_bignum_as_double
                                    { darg = bignum_to_double (arg); }
                                }

                              if format_as_long_double
                                {
                                  *f.offset (-1) = 'L' as c_char;
                                  *f = conversion; f = f.add (1);
                                  *f = '\0' as c_char;
                                  sprintf_bytes = sprintf (p, convspec.as_ptr (), prec, ldarg)
                                    as ptrdiff_t;
                                }
                              else
                                { sprintf_bytes = sprintf (p, convspec.as_ptr (), prec, darg)
                                    as ptrdiff_t; }
                            }
                          else if !force_bignum && conversion == 'c' as c_char
                            {
                              /* Don't use sprintf here, as it might mishandle prec.  */
                              *p = XFIXNUM (arg) as c_char;
                              *p.offset (1) = '\0' as c_char;
                              sprintf_bytes = (prec != 0) as ptrdiff_t;
                            }
                          else if force_bignum || BIGNUMP (arg)
                            {
                              /* C's `bignum_arg:` (editfns.c 3936).  */
                              let base: c_int = if conversion == 'd' as c_char
                                                  || conversion == 'i' as c_char { 10 }
                                else if conversion == 'o' as c_char { 8 }
                                else if conversion == 'x' as c_char
                                  || conversion == 'X' as c_char { 16 }
                                else { 2 };
                              sprintf_bytes = bignum_bufsize (arg, base);
                              if sprintf_bytes <= (buf as isize + bufsize - p as isize)
                                {
                                  let signedbase: c_int = if conversion == 'X' as c_char
                                    { -base } else { base };
                                  sprintf_bytes = bignum_to_c_string (p, sprintf_bytes,
                                                                      arg, signedbase);
                                  nonzero_arg = true;
                                  preformatted = true;
                                }
                            }
                          else if conversion == 'd' as c_char || conversion == 'i' as c_char
                            {
                              if FIXNUMP (arg)
                                {
                                  let x: intmax_t = XFIXNUM (arg) as intmax_t;
                                  sprintf_bytes = sprintf (p, convspec.as_ptr (), prec, x)
                                    as ptrdiff_t;
                                }
                              else
                                {
                                  strcpy (f.offset (-pMlen - 1), c"f".as_ptr ());
                                  let mut x: c_double = XFLOAT_DATA (arg);

                                  /* Truncate and then convert -0 to 0, to be more
                                     consistent with %x etc.; see Bug#31938.  */
                                  x = trunc (x);
                                  x = if x != 0.0 { x } else { 0.0 };

                                  sprintf_bytes = sprintf (p, convspec.as_ptr (), 0, x)
                                    as ptrdiff_t;
                                  let signedp: bool = !c_isdigit (*p as c_int);
                                  prec = min (precision,
                                              sprintf_bytes - signedp as ptrdiff_t) as c_int;
                                }
                            }
                          else
                            {
                              let mut x: uintmax_t;
                              let mut negative: bool;
                              if FIXNUMP (arg)
                                {
                                  let i_: EMACS_INT = XFIXNUM (arg);
                                  negative = i_ < 0;
                                  x = if negative { (-i_) as uintmax_t } else { i_ as uintmax_t };
                                }
                              else
                                {
                                  let d: c_double = XFLOAT_DATA (arg);
                                  let abs_d: c_double = fabs (d);
                                  if abs_d < (UINTMAX_MAX as c_double) + 1.0
                                    {
                                      negative = d <= -1.0;
                                      x = abs_d as uintmax_t;
                                    }
                                  else
                                    {
                                      arg = double_to_integer (d);
                                      force_bignum = true;
                                      continue 'num;
                                    }
                                }
                              *p = if negative { '-' as c_char }
                                else if plus_flag { '+' as c_char } else { ' ' as c_char };

                              /* Do %b and %B by hand unless sprintf is known to
                                 support them.  (The reference's note about the year
                                 2043 is editfns.c 4058-4060.)  */
                              /* The frozen <stdio.h> does not define
                                 __STDC_VERSION_STDIO_H__, so C's `#if` takes the `#else`
                                 arm: HAVE_PRINTF_B_CONVERSION_SPECIFIER is false.  */
                              const HAVE_PRINTF_B_CONVERSION_SPECIFIER: bool = false;
                              if !HAVE_PRINTF_B_CONVERSION_SPECIFIER
                                  && (conversion == 'b' as c_char || conversion == 'B' as c_char)
                                {
                                  let bit_width: c_int = stdc_bit_width! (x) as c_int;
                                  let bits: *mut c_char = p.offset (negative as isize);
                                  let nbits: c_int = if bit_width != 0 { bit_width }
                                    else { (prec < 0) as c_int };
                                  let mut i_: c_int = 0;
                                  while i_ < nbits
                                    {
                                      *bits.offset (i_ as isize)
                                        = (((x >> (nbits - (i_ + 1)) as u32) & 1)
                                           + '0' as uintmax_t) as c_char;
                                      i_ += 1;
                                    }
                                  *bits.offset (nbits as isize) = '\0' as c_char;
                                  sprintf_bytes = negative as ptrdiff_t + nbits as ptrdiff_t;
                                  nonzero_arg = x != 0;
                                  preformatted = true;
                                }
                              else
                                {
                                  let signedp: bool = negative | plus_flag | space_flag;
                                  sprintf_bytes = sprintf (p.offset (signedp as isize),
                                                           convspec.as_ptr (), prec, x)
                                    as ptrdiff_t;
                                  sprintf_bytes += signedp as ptrdiff_t;
                                }
                            }
                          break;
                        }

                      /* C's `preformatted_fixnum_arg:` (editfns.c 3949).  */
                      if preformatted
                        {
                          let negative: bool = *p == '-' as c_char;
                          prec = min (precision,
                                      sprintf_bytes - prefixlen as ptrdiff_t) as c_int;
                          *prefix.as_mut_ptr ().offset (prefixlen as isize)
                            = if plus_flag { '+' as c_char } else { ' ' as c_char };
                          prefixlen += ((plus_flag as c_int) | (space_flag as c_int))
                            & (!negative as c_int);
                          *prefix.as_mut_ptr ().offset (prefixlen as isize) = '0' as c_char;
                          *prefix.as_mut_ptr ().offset ((prefixlen + 1) as isize)
                            = conversion;
                          prefixlen += if (sharp_flag as c_int) & (nonzero_arg as c_int) != 0
                                          && !(conversion == 'd' as c_char
                                               || conversion == 'i' as c_char
                                               || conversion == 'o' as c_char)
                            { 2 } else { 0 };
                        }

                      /* Now the length of the formatted item is known, except it omits
                         padding and excess precision.  Deal with excess precision
                         first.  (The reference's explanation is editfns.c 4108-4115.)  */
                      let mut excess_precision: ptrdiff_t
                        = if precision_given { precision - prec as ptrdiff_t } else { 0 };
                      let mut trailing_zeros: ptrdiff_t = 0;
                      if excess_precision != 0 && float_conversion
                        {
                          if !c_isdigit (*p.offset (sprintf_bytes as isize - 1) as c_int)
                              || (conversion == 'g' as c_char
                                  && !(sharp_flag
                                       && !strchr (p, '.' as c_int).is_null ()))
                            { excess_precision = 0; }
                          trailing_zeros = excess_precision;
                        }
                      let mut leading_zeros: ptrdiff_t = excess_precision - trailing_zeros;

                      /* Compute the total bytes needed for this item, including
                         excess precision and padding.  */
                      let mut numwidth: ptrdiff_t = 0;
                      if { let (r, o) = (prefixlen as ptrdiff_t + sprintf_bytes)
                             .overflowing_add (excess_precision);
                           numwidth = r; o }
                        { numwidth = PTRDIFF_MAX; }
                      let mut padding: ptrdiff_t
                        = if numwidth < field_width { field_width - numwidth } else { 0 };
                      if max_bufsize - (prefixlen as ptrdiff_t + sprintf_bytes)
                           <= excess_precision
                          || max_bufsize - padding <= numwidth
                        { string_overflow (); }
                      convbytes = numwidth + padding;

                      if convbytes <= (buf as isize + bufsize - p as isize)
                        {
                          /* Skip any sign and 0[bBxX] prefixes.  */
                          let signedp: bool = *p == '-' as c_char || *p == '+' as c_char
                            || *p == ' ' as c_char;
                          let beglen: c_int = (signedp as c_int)
                            + if !float_conversion
                                 && *p.offset (signedp as isize) == '0' as c_char
                                 && *p.offset (signedp as isize + 1) == conversion
                              { 2 } else { 0 };
                          eassert! (prefixlen == 0 || beglen == 0
                                    || (beglen == 1 && *p == '-' as c_char
                                        && !(prefix[0] == '-' as c_char
                                             || prefix[0] == '+' as c_char
                                             || prefix[0] == ' ' as c_char)));
                          if zero_flag && 0 <= char_hexdigit (*p.offset (beglen as isize) as c_int)
                            {
                              leading_zeros += padding;
                              padding = 0;
                            }
                          if leading_zeros == 0 && sharp_flag
                              && conversion == 'o' as c_char
                              && *p.offset (beglen as isize) != '0' as c_char
                            {
                              leading_zeros += 1;
                              padding -= (padding != 0) as ptrdiff_t;
                            }

                          let mut endlen: c_int = 0;
                          if trailing_zeros != 0
                              && (conversion == 'e' as c_char || conversion == 'g' as c_char)
                            {
                              let e: *mut c_char = strchr (p, 'e' as c_int);
                              if !e.is_null ()
                                { endlen = p.offset (sprintf_bytes as isize).offset_from (e)
                                    as c_int; }
                            }

                          let midlen: ptrdiff_t = sprintf_bytes - beglen as ptrdiff_t
                            - endlen as ptrdiff_t;
                          let leading_padding: ptrdiff_t = if minus_flag { 0 } else { padding };
                          let trailing_padding: ptrdiff_t = padding - leading_padding;

                          /* Insert padding and excess-precision zeros.  (The
                             reference's component diagram is editfns.c 4186-4196.)  */
                          let incr: ptrdiff_t
                            = padding + leading_zeros + prefixlen as ptrdiff_t
                              + sprintf_bytes + trailing_zeros;

                          /* Optimize for the typical case with padding or zeros.  */
                          if incr != sprintf_bytes
                            {
                              /* Move data to make room to insert spaces and '0's.
                                 As this may entail overlapping moves, process
                                 the output right-to-left and use memmove.
                                 With any luck this code is rarely executed.  */
                              let mut src: *mut c_char = p.offset (sprintf_bytes as isize);
                              let mut dst: *mut c_char = p.offset (incr as isize);
                              dst = dst.offset (-trailing_padding as isize);
                              memset (dst as *mut c_void, ' ' as c_int,
                                      trailing_padding as usize);
                              src = src.offset (-(endlen as isize));
                              dst = dst.offset (-(endlen as isize));
                              memmove (dst as *mut c_void, src as *const c_void,
                                       endlen as usize);
                              dst = dst.offset (-trailing_zeros as isize);
                              memset (dst as *mut c_void, '0' as c_int,
                                      trailing_zeros as usize);
                              src = src.offset (-midlen as isize);
                              dst = dst.offset (-midlen as isize);
                              memmove (dst as *mut c_void, src as *const c_void,
                                       midlen as usize);
                              dst = dst.offset (-leading_zeros as isize);
                              memset (dst as *mut c_void, '0' as c_int,
                                      leading_zeros as usize);
                              dst = dst.offset (-(prefixlen as isize));
                              memcpy (dst as *mut c_void, prefix.as_ptr () as *const c_void,
                                      prefixlen as usize);
                              src = src.offset (-(beglen as isize));
                              dst = dst.offset (-(beglen as isize));
                              memmove (dst as *mut c_void, src as *const c_void,
                                       beglen as usize);
                              dst = dst.offset (-leading_padding as isize);
                              memset (dst as *mut c_void, ' ' as c_int,
                                      leading_padding as usize);
                            }

                          p = p.add (incr as usize);
                          (*spec).start = nchars;
                          nchars += incr;
                          (*spec).end = nchars;
                          new_result = true;
                          convbytes = CONVBYTES_ROOM;
                        }
                    }
                }
            }
          else
            {
              let mut str_: [u8; MAX_MULTIBYTE_LENGTH as usize] = std::mem::zeroed ();

              if (format_char == '`' as c_uchar || format_char == '\'' as c_uchar)
                  && EQ (quoting_style, Qcurve ())
                {
                  if !multibyte
                    {
                      multibyte = true;
                      continue 'retry;
                    }
                  convsrc = if format_char == '`' as c_uchar
                    { uLSQM.as_ptr () } else { uRSQM.as_ptr () };
                  convbytes = 3;
                  new_result = true;
                }
              else if format_char == '`' as c_uchar && EQ (quoting_style, Qstraight ())
                {
                  convsrc = c"'".as_ptr ();
                  new_result = true;
                }
              else
                {
                  /* Copy a single character from format to buf.  */
                  if multibyte_format
                    {
                      /* Copy a whole multibyte character.  */
                      if (p as isize) > (buf as isize)
                          && !ASCII_CHAR_P (*p.offset (-1) as isize)
                          && !CHAR_HEAD_P (format_char as c_int)
                        { maybe_combine_byte = true; }

                      while !CHAR_HEAD_P (*format as c_int)
                        { format = format.add (1); }

                      convbytes = format.offset_from (format0);
                      memset (discarded.offset
                              (format0.offset_from (format_start) as isize + 1)
                              as *mut c_void,
                              2, (convbytes - 1) as usize);
                    }
                  else if multibyte && !ASCII_CHAR_P (format_char as isize)
                    {
                      let c: c_int = BYTE8_TO_CHAR (format_char as c_int);
                      convbytes = CHAR_STRING (c, str_.as_mut_ptr ()) as ptrdiff_t;
                      convsrc = str_.as_ptr () as *const c_char;
                      new_result = true;
                    }
                }

              /* C's `copy_char:` label (editfns.c 4279).  */
              copy_char = true;
            }

          if copy_char
            {
              memcpy (p as *mut c_void, convsrc as *const c_void, convbytes as usize);
              p = p.add (convbytes as usize);
              nchars += 1;
              convbytes = CONVBYTES_ROOM;
            }

          let used: ptrdiff_t = p.offset_from (buf);
          let mut buflen_needed: ptrdiff_t = 0;
          if { let (r, o) = used.overflowing_add (convbytes); buflen_needed = r; o }
            { string_overflow (); }
          if bufsize <= buflen_needed
            {
              if max_bufsize <= buflen_needed
                { string_overflow (); }

              /* Either there wasn't enough room to store this conversion,
                 or there won't be enough room to do a sprintf the next
                 time through the loop.  Allocate enough room (and then some).  */

              bufsize = if buflen_needed <= max_bufsize / 2
                { buflen_needed * 2 } else { max_bufsize };

              if std::ptr::eq (buf, initial_buffer.as_mut_ptr ())
                {
                  buf = xmalloc (bufsize as usize) as *mut c_char;
                  buf_save_value_index = SPECPDL_INDEX ();
                  record_unwind_protect_ptr (Some (xfree), buf as *mut c_void);
                  memcpy (buf as *mut c_void, initial_buffer.as_ptr () as *const c_void,
                          used as usize);
                }
              else
                {
                  buf = xrealloc (buf as *mut c_void, bufsize as usize) as *mut c_char;
                  set_unwind_protect_ptr (buf_save_value_index, Some (xfree),
                                          buf as *mut c_void);
                }

              p = buf.offset (used as isize);
              if convbytes != CONVBYTES_ROOM
                {
                  /* There wasn't enough room for this conversion; do it over.  */
                  eassert! (CONVBYTES_ROOM < convbytes);
                  format = format0;
                  n = n0;
                  ispec = ispec0;
                }
            }
        }

      break;
    }

  if bufsize < p.offset_from (buf)
    { emacs_abort (); }

  if !new_result
    {
      val = *args.offset (0);
      break 'return_val;
    }

  if maybe_combine_byte
    { nchars = multibyte_chars_in_text (buf as *const u8, p.offset_from (buf)); }
  val = make_specified_string (buf, nchars, p.offset_from (buf), multibyte);

  /* If the format string has text properties, or any of the string
     arguments has text properties, set up text properties of the
     result string.  */

  if !string_intervals (*args.offset (0)).is_null () || arg_intervals
    {
      /* Add text properties from the format string.  */
      let mut len: Lisp_Object = make_fixnum (SCHARS (*args.offset (0)));
      let mut props: Lisp_Object = text_property_list (*args.offset (0), make_fixnum (0),
                                                       len, Qnil ());
      if CONSP (props)
        {
          let mut bytepos: ptrdiff_t = 0;
          let mut position: ptrdiff_t = 0;
          let mut translated: ptrdiff_t = 0;
          let mut fieldn: ptrdiff_t = 0;

          /* Adjust the bounds of each text property
             to the proper start and end in the output string.  */

          /* Put the positions in PROPS in increasing order, so that
             we can do (effectively) one scan through the position
             space of the format string.  */
          props = Fnreverse (props);

          /* BYTEPOS is the byte position in the format string,
             POSITION is the untranslated char position in it,
             TRANSLATED is the translated char position in BUF,
             and ARGN is the number of the next arg we will come to.  */
          let mut list: Lisp_Object = props;
          while CONSP (list)
            {
              let item: Lisp_Object = XCAR (list);

              /* First adjust the property start position.  */
              let mut pos: ptrdiff_t = XFIXNUM (XCAR (item));

              /* Advance BYTEPOS, POSITION, TRANSLATED and ARGN
                 up to this position.  */
              while position < pos
                {
                  if *discarded.offset (bytepos as isize) == 0 as c_char
                    { position += 1; translated += 1; }
                  else if *discarded.offset (bytepos as isize) == 1 as c_char
                    {
                      position += 1;
                      if fieldn < nspec
                          && bytepos >= (*info.offset (fieldn as isize)).fbeg
                          && translated == (*info.offset (fieldn as isize)).start
                        {
                          translated += (*info.offset (fieldn as isize)).end
                            - (*info.offset (fieldn as isize)).start;
                          fieldn += 1;
                        }
                    }
                  bytepos += 1;
                }

              XSETCAR (item, make_fixnum (translated));

              /* Likewise adjust the property end position.  */
              pos = XFIXNUM (XCAR (XCDR (item)));

              while position < pos
                {
                  if *discarded.offset (bytepos as isize) == 0 as c_char
                    { position += 1; translated += 1; }
                  else if *discarded.offset (bytepos as isize) == 1 as c_char
                    {
                      position += 1;
                      if fieldn < nspec
                          && bytepos >= (*info.offset (fieldn as isize)).fbeg
                          && translated == (*info.offset (fieldn as isize)).start
                        {
                          translated += (*info.offset (fieldn as isize)).end
                            - (*info.offset (fieldn as isize)).start;
                          fieldn += 1;
                        }
                    }
                  bytepos += 1;
                }

              XSETCAR (XCDR (item), make_fixnum (translated));
              list = XCDR (list);
            }

          add_text_properties_from_list (val, props, make_fixnum (0));
        }

      /* Add text properties from arguments.  */
      if arg_intervals
        {
          let mut i_: ptrdiff_t = 0;
          while i_ < nspec
            {
              if (*info.offset (i_ as isize)).intervals
                {
                  len = make_fixnum (SCHARS (*spec_arguments.offset (i_ as isize)));
                  let new_len: Lisp_Object = make_fixnum
                    ((*info.offset (i_ as isize)).end - (*info.offset (i_ as isize)).start);
                  props = text_property_list (*spec_arguments.offset (i_ as isize),
                                              make_fixnum (0), len, Qnil ());
                  props = extend_property_ranges (props, len, new_len);
                  /* If successive arguments have properties, be sure that
                     the value of `composition' property be the copy.  */
                  if 1 <= i_ && (*info.offset (i_ as isize - 1)).end != 0
                    { make_composition_value_copy (props); }
                  add_text_properties_from_list (val, props,
                                                 make_fixnum
                                                 ((*info.offset (i_ as isize)).start));
                }
              i_ += 1;
            }
        }
    }
  }

  /* C's `return_val:` label (editfns.c 4354).  */
  /* If we allocated BUF or auxiliary tables with malloc, free them too.  */
  SAFE_FREE! (sa_count, sa_mark);

  return val;
}

DEFUN! ("char-equal", Fchar_equal, Schar_equal, 2, 2, 0,
       doc: "Return t if two characters match, optionally ignoring case.\nBoth arguments must be characters (i.e. integers).\nCase is ignored if `case-fold-search' is non-nil in the current buffer.",
       (c1: Lisp_Object, c2: Lisp_Object),
       {
  let mut i1: c_int;
  let mut i2: c_int;
  /* Check they're chars, not just integers, otherwise we could get array
     bounds violations in downcase.  */
  CHECK_CHARACTER (c1);
  CHECK_CHARACTER (c2);

  if XFIXNUM (c1) == XFIXNUM (c2)
    { return Qt (); }
  if NILP (Vcase_fold_search)
    { return Qnil (); }

  i1 = XFIXNAT (c1) as c_int;
  i2 = XFIXNAT (c2) as c_int;

  /* FIXME: It is possible to compare multibyte characters even when
     the current buffer is unibyte.  (The reference's full comment is
     editfns.c 4434-4440.)  */
  if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    {
      if SINGLE_BYTE_CHAR_P (i1 as intmax_t)
        { i1 = UNIBYTE_TO_CHAR (i1); }
      if SINGLE_BYTE_CHAR_P (i2 as intmax_t)
        { i2 = UNIBYTE_TO_CHAR (i2); }
    }

  return if downcase (i1) == downcase (i2) { Qt () } else { Qnil () };
});

/* Transpose the markers in two regions of the current buffer, and
   adjust the ones between them if necessary (i.e.: if the regions
   differ in size).

   (The reference's full explanation is editfns.c 4450-4463.)  */

unsafe fn transpose_markers (start1: ptrdiff_t, end1: ptrdiff_t,
                             start2: ptrdiff_t, end2: ptrdiff_t,
                             start1_byte: ptrdiff_t, end1_byte: ptrdiff_t,
                             start2_byte: ptrdiff_t, end2_byte: ptrdiff_t)
{
  let mut amt1: ptrdiff_t;
  let mut amt1_byte: ptrdiff_t;
  let mut amt2: ptrdiff_t;
  let mut amt2_byte: ptrdiff_t;
  let mut diff: ptrdiff_t;
  let mut diff_byte: ptrdiff_t;
  let mut mpos: ptrdiff_t;
  let mut marker: *mut Lisp_Marker;

  /* Update point as if it were a marker.  */
  if PT! () < start1
    { }
  else if PT! () < end1
    { TEMP_SET_PT_BOTH (PT! () + (end2 - end1),
                        PT_BYTE! () + (end2_byte - end1_byte)); }
  else if PT! () < start2
    { TEMP_SET_PT_BOTH (PT! () + (end2 - start2) - (end1 - start1),
                        (PT_BYTE! () + (end2_byte - start2_byte)
                         - (end1_byte - start1_byte))); }
  else if PT! () < end2
    { TEMP_SET_PT_BOTH (PT! () - (start2 - start1),
                        PT_BYTE! () - (start2_byte - start1_byte)); }

  /* We used to adjust the endpoints here to account for the gap, but that
     isn't good enough.  (The reference's explanation is editfns.c 4494-4500.)  */

  /* The difference between the region's lengths */
  diff = (end2 - start2) - (end1 - start1);
  diff_byte = (end2_byte - start2_byte) - (end1_byte - start1_byte);

  /* For shifting each marker in a region by the length of the other
     region plus the distance between the regions.  */
  amt1 = (end2 - start2) + (start2 - end1);
  amt2 = (end1 - start1) + (start2 - end1);
  amt1_byte = (end2_byte - start2_byte) + (start2_byte - end1_byte);
  amt2_byte = (end1_byte - start1_byte) + (start2_byte - end1_byte);

  marker = BUF_MARKERS! ((current_buffer! ()));
  while !marker.is_null ()
    {
      mpos = (*marker).bytepos;
      if mpos >= start1_byte && mpos < end2_byte
        {
          if mpos < end1_byte
            { mpos += amt1_byte; }
          else if mpos < start2_byte
            { mpos += diff_byte; }
          else
            { mpos -= amt2_byte; }
          (*marker).bytepos = mpos;
        }
      mpos = (*marker).charpos;
      if mpos >= start1 && mpos < end2
        {
          if mpos < end1
            { mpos += amt1; }
          else if mpos < start2
            { mpos += diff; }
          else
            { mpos -= amt2; }
        }
      (*marker).charpos = mpos;
      marker = (*marker).next;
    }
}

DEFUN! ("transpose-regions", Ftranspose_regions, Stranspose_regions, 4, 5,
       "(if (< (length mark-ring) 2)\t    (error \"Other region must be marked before transposing two regions\")\t  (let* ((num (if current-prefix-arg\t\t\t (prefix-numeric-value current-prefix-arg)\t\t\t0))\t\t (ring-length (length mark-ring))\t\t (eltnum (mod num ring-length))\t\t (eltnum2 (mod (1+ num) ring-length)))\t    (list (point) (mark) (elt mark-ring eltnum) (elt mark-ring eltnum2))))",
       doc: "Transpose region STARTR1 to ENDR1 with STARTR2 to ENDR2.\nThe regions should not be overlapping, because the size of the buffer is\nnever changed in a transposition.\n\nOptional fifth arg LEAVE-MARKERS, if non-nil, means don't update\nany markers that happen to be located in the regions.\n\nTransposing beyond buffer boundaries is an error.\n\nInteractively, STARTR1 and ENDR1 are point and mark; STARTR2 and ENDR2\nare the last two marks pushed to the mark ring; LEAVE-MARKERS is nil.\nIf a prefix argument N is given, STARTR2 and ENDR2 are the two\nsuccessive marks N entries back in the mark ring.  A negative prefix\nargument instead counts forward from the oldest mark in the mark\nring.",
       (startr1: Lisp_Object, endr1: Lisp_Object, startr2: Lisp_Object, endr2: Lisp_Object,
        leave_markers: Lisp_Object),
       {
  let mut startr1: Lisp_Object = startr1;
  let mut endr1: Lisp_Object = endr1;
  let mut startr2: Lisp_Object = startr2;
  let mut endr2: Lisp_Object = endr2;
  let mut start1: ptrdiff_t;
  let mut end1: ptrdiff_t;
  let mut start2: ptrdiff_t;
  let mut end2: ptrdiff_t;
  let mut start1_byte: ptrdiff_t;
  let mut end1_byte: ptrdiff_t;
  let mut start2_byte: ptrdiff_t;
  let mut end2_byte: ptrdiff_t;
  let mut gap: ptrdiff_t;
  let mut len1: ptrdiff_t;
  let mut len_mid: ptrdiff_t;
  let mut len2: ptrdiff_t;
  let mut len1_byte: ptrdiff_t;
  let mut len_mid_byte: ptrdiff_t;
  let mut len2_byte: ptrdiff_t;
  let mut start1_addr: *mut u8 = std::ptr::null_mut ();
  let mut start2_addr: *mut u8 = std::ptr::null_mut ();
  let mut temp: *mut u8 = std::ptr::null_mut ();

  let mut cur_intv: INTERVAL = std::ptr::null_mut ();
  let mut tmp_interval1: INTERVAL = std::ptr::null_mut ();
  let mut tmp_interval2: INTERVAL = std::ptr::null_mut ();
  let mut tmp_interval_mid: INTERVAL = std::ptr::null_mut ();
  let mut tmp_interval3: INTERVAL = std::ptr::null_mut ();
  let mut buf: Lisp_Object;

  XSETBUFFER! (buf, (current_buffer! ()));
  cur_intv = buffer_intervals ((current_buffer! ()));

  validate_region (&mut startr1, &mut endr1);
  validate_region (&mut startr2, &mut endr2);

  start1 = XFIXNAT (startr1);
  end1 = XFIXNAT (endr1);
  start2 = XFIXNAT (startr2);
  end2 = XFIXNAT (endr2);
  gap = GPT! ();

  /* Swap the regions if they're reversed.  We do not swap the
     corresponding Lisp objects as well, since we reference these only
     to clear text properties in both regions.  */
  if start2 < end1
    {
      let mut glumph: ptrdiff_t = start1;
      start1 = start2;
      start2 = glumph;
      glumph = end1;
      end1 = end2;
      end2 = glumph;
    }

  len1 = end1 - start1;
  len2 = end2 - start2;

  if start2 < end1
    { error (c"Transposed regions overlap".as_ptr ()); }
  /* Nothing to change for adjacent regions with one being empty */
  else if (start1 == end1 || start2 == end2) && end1 == start2
    { return Qnil (); }

  start1_byte = CHAR_TO_BYTE (start1);
  end2_byte = CHAR_TO_BYTE (end2);

  /* C 4615-4622's `#ifdef HAVE_TREE_SITTER` block is dead (docs/ports/editfns.md).  */

  /* Run the before-change-functions *before* we move the gap.  */
  modify_text (start1, end2);

  /* It must be pointed out that the really studly thing to do would
     be not to move the gap at all, ... (The reference's comment is
     editfns.c 4630-4641.)  */

  /* Hmmm... how about checking to see if the gap is large
     enough to use as the temporary storage?  (editfns.c 4643-4646.)  */

  /* Make sure the gap won't interfere, by moving it out of the text
     we will operate on.  */
  if start1 < gap && gap < end2
    {
      if gap - start1 < end2 - gap
        { move_gap_both (start1, start1_byte); }
      else
        { move_gap_both (end2, end2_byte); }
    }

  start2_byte = CHAR_TO_BYTE (start2);
  end1_byte = CHAR_TO_BYTE (end1);
  len1_byte = end1_byte - start1_byte;
  len2_byte = end2_byte - start2_byte;

  /* C 4655-4683's `#ifdef BYTE_COMBINING_DEBUG` block is dead for the frozen
     configuration (docs/ports/editfns.md).  */

  /* The possibilities are: ... (The reference's comment is editfns.c 4686-4715.)  */

  tmp_interval1 = copy_intervals (cur_intv, start1, len1);
  tmp_interval2 = copy_intervals (cur_intv, start2, len2);

  len_mid = start2 - end1;
  len_mid_byte = start2_byte - end1_byte;

  if len1 == len2
    {
      if end1 == start2		/* Merge the two parts into a single one.  */
        { record_change (start1, (end2 - start1)); }
      else
        {
          record_change (start1, len1);
          record_change (start2, len2);
        }

      tmp_interval3 = validate_interval_range (buf, &mut startr1, &mut endr1, false);
      if !tmp_interval3.is_null ()
        { set_text_properties_1 (startr1, endr1, Qnil (), buf, tmp_interval3); }

      tmp_interval3 = validate_interval_range (buf, &mut startr2, &mut endr2, false);
      if !tmp_interval3.is_null ()
        { set_text_properties_1 (startr2, endr2, Qnil (), buf, tmp_interval3); }
    }
  else
    /* Regions have different length, character-wise.  Handle undo and
       text properties for both regions as one long piece of text
       spanning both regions and the mid.  But while doing so, save the
       intervals of the mid to later restore them in their new
       position.  */
    {
      record_change (start1, (end2 - start1));
      tmp_interval_mid = copy_intervals (cur_intv, end1, len_mid);
      tmp_interval3 = validate_interval_range (buf, &mut startr1, &mut endr2, false);
      if !tmp_interval3.is_null ()
        { set_text_properties_1 (startr1, endr2, Qnil (), buf, tmp_interval3); }
    }

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  if len1_byte == len2_byte
    {
      temp = SAFE_ALLOCA! (sa_avail, len1_byte as usize) as *mut u8;
      start1_addr = BYTE_POS_ADDR (start1_byte);
      start2_addr = BYTE_POS_ADDR (start2_byte);
      memcpy (temp as *mut c_void, start1_addr as *const c_void, len1_byte as usize);
      memcpy (start1_addr as *mut c_void, start2_addr as *const c_void, len2_byte as usize);
      memcpy (start2_addr as *mut c_void, temp as *const c_void, len1_byte as usize);
    }
  else if len1_byte < len2_byte	/* Second region larger than first */
    {
      /* holds region 2 */
      temp = SAFE_ALLOCA! (sa_avail, len2_byte as usize) as *mut u8;
      start1_addr = BYTE_POS_ADDR (start1_byte);
      start2_addr = BYTE_POS_ADDR (start2_byte);
      memcpy (temp as *mut c_void, start2_addr as *const c_void, len2_byte as usize);
      memcpy (start1_addr.offset ((len_mid_byte + len2_byte) as isize) as *mut c_void,
              start1_addr as *const c_void, len1_byte as usize);
      memmove (start1_addr.offset (len2_byte as isize) as *mut c_void,
               start1_addr.offset (len1_byte as isize) as *const c_void,
               len_mid_byte as usize);
      memcpy (start1_addr as *mut c_void, temp as *const c_void, len2_byte as usize);
    }
  else
    /* Second region smaller than first.  */
    {
      /* holds region 1 */
      temp = SAFE_ALLOCA! (sa_avail, len1_byte as usize) as *mut u8;
      start1_addr = BYTE_POS_ADDR (start1_byte);
      start2_addr = BYTE_POS_ADDR (start2_byte);
      memcpy (temp as *mut c_void, start1_addr as *const c_void, len1_byte as usize);
      memcpy (start1_addr as *mut c_void, start2_addr as *const c_void, len2_byte as usize);
      memmove (start1_addr.offset (len2_byte as isize) as *mut c_void,
               start1_addr.offset (len1_byte as isize) as *const c_void,
               len_mid_byte as usize);
      memcpy (start1_addr.offset ((len2_byte + len_mid_byte) as isize) as *mut c_void,
              temp as *const c_void, len1_byte as usize);
    }
  SAFE_FREE! (sa_count, sa_mark);

  if len1 != len2
    /* Restore intervals of the mid.  */
    {
      graft_intervals_into_buffer (tmp_interval_mid, start1 + len2,
                                   len_mid, (current_buffer! ()), false);
    }
  graft_intervals_into_buffer (tmp_interval1, end2 - len1,
                               len1, (current_buffer! ()), false);
  graft_intervals_into_buffer (tmp_interval2, start1,
                               len2, (current_buffer! ()), false);

  update_compositions (start1, start1 + len2, CHECK_BORDER);
  update_compositions (end2 - len1, end2, CHECK_BORDER);

  /* When doing multiple transpositions, it might be nice
     to optimize this.  Perhaps the markers in any one buffer
     should be organized in some sorted data tree.  */
  if NILP (leave_markers)
    {
      /* FIXME: Since the undo info doesn't record the transposition as its own
         operation, we won't enjoy 'transpose_markers' during undo :-(  */
      transpose_markers (start1, end1, start2, end2,
                         start1_byte, start1_byte + len1_byte,
                         start2_byte, start2_byte + len2_byte);
    }
  else
    {
      /* The character positions of the markers remain intact, but we
         still need to update their byte positions, because the
         transposed regions might include multibyte sequences which
         make some original byte positions of the markers invalid.  */
      adjust_markers_bytepos (start1, start1_byte, end2, end2_byte, 0);
    }

  /* C 4814-4817's `#ifdef HAVE_TREE_SITTER` block is dead (docs/ports/editfns.md).  */

  signal_after_change (start1, end2 - start1, end2 - start1);
  return Qnil ();
});

/* C's `DEFVAR_LISP`s `syms_of_editfns` registers define the `V*` objects here (status
   3.29: the unit that owns the C definition owns the port's `static mut`).  */
#[unsafe(no_mangle)]
pub static mut Vinhibit_field_text_motion: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vbuffer_access_fontify_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vbuffer_access_fontified_property: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vsystem_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vuser_full_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vuser_login_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vuser_real_login_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Voperating_system_release: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_editfns ()
{
  DEFSYM! (Qbuffer_access_fontify_functions, c"buffer-access-fontify-functions");
  DEFSYM! (Qwall, c"wall");
  DEFSYM! (Qpropertize, c"propertize");

  staticpro (&raw const labeled_restrictions);

  DEFVAR_LISP! (c"inhibit-field-text-motion", Vinhibit_field_text_motion,
	       "Non-nil means text motion commands don't notice fields.");
  Vinhibit_field_text_motion = Qnil ();

  DEFVAR_LISP! (c"buffer-access-fontify-functions",
	       Vbuffer_access_fontify_functions,
	       "List of functions called by `buffer-substring' to fontify if necessary.\nEach function is called with two arguments which specify the range\nof the buffer being accessed.");
  Vbuffer_access_fontify_functions = Qnil ();

  {
    let mut obuf: Lisp_Object;
    obuf = Fcurrent_buffer ();
    /* Do this here, because init_buffer_once is too early--it won't work.  */
    Fset_buffer (Vprin1_to_string_buffer);
    /* Make sure buffer-access-fontify-functions is nil in this buffer.  */
    Fset (Fmake_local_variable (Qbuffer_access_fontify_functions ()), Qnil ());
    Fset_buffer (obuf);
  }

  DEFVAR_LISP! (c"buffer-access-fontified-property",
	       Vbuffer_access_fontified_property,
	       "Property which (if non-nil) indicates text has been fontified.\n`buffer-substring' need not call the `buffer-access-fontify-functions'\nfunctions if all the text being accessed has this property.");
  Vbuffer_access_fontified_property = Qnil ();

  DEFVAR_LISP! (c"system-name", Vsystem_name,
	       "The host name of the machine Emacs is running on.");
  cached_system_name = Qnil ();
  Vsystem_name = cached_system_name;

  DEFVAR_LISP! (c"user-full-name", Vuser_full_name,
	       "The full name of the user logged in.");

  DEFVAR_LISP! (c"user-login-name", Vuser_login_name,
	       "The user's name, taken from environment variables if possible.");
  Vuser_login_name = Qnil ();

  DEFVAR_LISP! (c"user-real-login-name", Vuser_real_login_name,
	       "The user's name, based upon the real uid only.");

  DEFVAR_LISP! (c"operating-system-release", Voperating_system_release,
	       "The kernel version of the operating system on which Emacs is running.\nThe value is a string.  It can also be nil if Emacs doesn't\nknow how to get the kernel version on the underlying OS.");

  DEFSYM! (Qoutermost_restriction, c"outermost-restriction");
  Funintern (Qoutermost_restriction (), Qnil ());

  defsubr (&raw mut Spropertize);
  defsubr (&raw mut Schar_equal);
  defsubr (&raw mut Sgoto_char);
  defsubr (&raw mut Sstring_to_char);
  defsubr (&raw mut Schar_to_string);
  defsubr (&raw mut Sbyte_to_string);
  defsubr (&raw mut Sbuffer_substring);
  defsubr (&raw mut Sbuffer_substring_no_properties);
  defsubr (&raw mut Sbuffer_string);
  defsubr (&raw mut Sget_pos_property);

  defsubr (&raw mut Spoint_marker);
  defsubr (&raw mut Smark_marker);
  defsubr (&raw mut Spoint);
  defsubr (&raw mut Sregion_beginning);
  defsubr (&raw mut Sregion_end);

  /* Symbol for the text property used to mark fields.  */
  DEFSYM! (Qfield, c"field");

  /* A special value for Qfield properties.  */
  DEFSYM! (Qboundary, c"boundary");

  defsubr (&raw mut Sfield_beginning);
  defsubr (&raw mut Sfield_end);
  defsubr (&raw mut Sfield_string);
  defsubr (&raw mut Sfield_string_no_properties);
  defsubr (&raw mut Sdelete_field);
  defsubr (&raw mut Sconstrain_to_field);

  defsubr (&raw mut Sline_beginning_position);
  defsubr (&raw mut Sline_end_position);
  defsubr (&raw mut Spos_bol);
  defsubr (&raw mut Spos_eol);

  defsubr (&raw mut Ssave_excursion);
  defsubr (&raw mut Ssave_current_buffer);

  defsubr (&raw mut Sbuffer_size);
  defsubr (&raw mut Spoint_max);
  defsubr (&raw mut Spoint_min);
  defsubr (&raw mut Spoint_min_marker);
  defsubr (&raw mut Spoint_max_marker);
  defsubr (&raw mut Sgap_position);
  defsubr (&raw mut Sgap_size);
  defsubr (&raw mut Sposition_bytes);
  defsubr (&raw mut Sbyte_to_position);

  defsubr (&raw mut Sbobp);
  defsubr (&raw mut Seobp);
  defsubr (&raw mut Sbolp);
  defsubr (&raw mut Seolp);
  defsubr (&raw mut Sfollowing_char);
  defsubr (&raw mut Sprevious_char);
  defsubr (&raw mut Schar_after);
  defsubr (&raw mut Schar_before);
  defsubr (&raw mut Sinsert);
  defsubr (&raw mut Sinsert_before_markers);
  defsubr (&raw mut Sinsert_and_inherit);
  defsubr (&raw mut Sinsert_and_inherit_before_markers);
  defsubr (&raw mut Sinsert_char);
  defsubr (&raw mut Sinsert_byte);

  defsubr (&raw mut Sngettext);

  defsubr (&raw mut Suser_login_name);
  defsubr (&raw mut Sgroup_name);
  defsubr (&raw mut Suser_real_login_name);
  defsubr (&raw mut Suser_uid);
  defsubr (&raw mut Suser_real_uid);
  defsubr (&raw mut Sgroup_gid);
  defsubr (&raw mut Sgroup_real_gid);
  defsubr (&raw mut Suser_full_name);
  defsubr (&raw mut Semacs_pid);
  defsubr (&raw mut Ssystem_name);
  defsubr (&raw mut Smessage);
  defsubr (&raw mut Smessage_box);
  defsubr (&raw mut Smessage_or_box);
  defsubr (&raw mut Scurrent_message);
  defsubr (&raw mut Sformat);
  defsubr (&raw mut Sformat_message);

  defsubr (&raw mut Sinsert_buffer_substring);
  defsubr (&raw mut Scompare_buffer_substrings);
  defsubr (&raw mut Sreplace_region_contents);
  defsubr (&raw mut Ssubst_char_in_region);
  defsubr (&raw mut Stranslate_region_internal);
  defsubr (&raw mut Sdelete_region);
  defsubr (&raw mut Sdelete_and_extract_region);
  defsubr (&raw mut Swiden);
  defsubr (&raw mut Snarrow_to_region);
  defsubr (&raw mut Sinternal__labeled_narrow_to_region);
  defsubr (&raw mut Sinternal__labeled_widen);
  defsubr (&raw mut Ssave_restriction);
  defsubr (&raw mut Stranspose_regions);
}
