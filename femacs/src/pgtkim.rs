// femacs: port of src/pgtkim.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `pgtkim.o`
// sits in `PGTK_OBJ` (`emacs/src/Makefile.in` 325, `PGTK_OBJ=@PGTK_OBJ@`, filled by `configure.ac`
// 3875-3878 - `PGTK_OBJ="pgtkfns.o pgtkterm.o pgtkselect.o pgtkmenu.o pgtkim.o xsettings.o"` under
// `if test "$window_system" = "pgtk"`), and `docs/baseline/frozen-build-sources.txt` (106 lines)
// lists no pgtk object for this target - the frozen `build-tu` object for this stem is the port's
// own, **not** a reference object to compare against.  `extract-abi.py`'s checklist for the file is
// therefore empty while the unit is the 11-line note (P-86 ②), so the ledger `docs/ports/pgtkim.md`
// carries that clause's substitute: the **C line-range table**, the `## Evidence` readings and the
// `## Deviations`.  This batch is P-86 ④'s first slice and it is the **whole file, C 1-311**.
//
//   * the **file gate** is `feature = "have_pgtk"` (`HAVE_PGTK`, `configure.ac` 3175-3176 - the
//     `window_system = pgtk` case sets `HAVE_PGTK=yes` and `AC_DEFINE([HAVE_PGTK], [1], …)`, and
//     `configure.ac` 3173 is where that case sets `term_header=pgtkterm.h`), on every item *and on
//     the include graph* (P-86 ⑧ - an `include!` outside the gate would put GTK names in the frozen
//     configuration, where this file is not compiled at all);
//   * C's top level has **no `#if` of its own** (the file is only ever reached through the pgtk
//     build), so the gate is the only condition this slice carries and every C line below is live
//     text to the preprocessor;
//   * **`docs/port-map.md` binds this ledger to `src/pgtkim.c` alone** - the reference has no
//     `src/pgtkim.h`, so this unit is the single-reference shape and there is no header mirror of
//     its own to `include!` (P-90 ②);
//   * the include graph reaches `include/pgtkterm.rs` (C 23's `"pgtkterm.h"`: `pgtk_display_info`,
//     its `im` member, `FRAME_DISPLAY_INFO`, `FRAME_GTK_WIDGET`, `check_pgtk_display_info`,
//     `pgtk_enqueue_string`, `pgtk_enqueue_preedit`) and `include/gtk.rs` (the GTK/GDK/cairo/GLib
//     surface `"pgtkterm.h"` 31's `<gtk/gtk.h>` names).  `include/pgtkterm.rs` carries *prototypes*
//     for the six `pgtk_im_*` functions this unit **defines**; a Rust definition shadows a
//     glob-imported declaration (PORTING.md 0.1-10/P-28), which is what makes C's "prototype plus
//     definition" shape work here - but the mirror's prototype for `pgtk_im_set_cursor_location`
//     spells a different signature than C 213-215's (`(struct frame *, int, int, GdkRectangle)`
//     against the header's 667-668 `(struct frame *, int, int, int, int)`): the definition below is
//     C's, and the mirror's misquote is recorded in `docs/ports/pgtkim.md`'s `## Findings`;
//   * **the toolkit layer is declared here (P-88 ①), and this batch is `text-only` for it
//     (P-86 ③⑵).**  `<pango/pango-attributes.h>` (C 55-149's types and entry points) and the eight
//     `gtk_im_context_*` entry points C 44-261 calls reach this file through `"pgtkterm.h"` 31's
//     `<gtk/gtk.h>`, and neither header has a file in the reference tree.  **GLib/GObject and cairo
//     are on this machine** (`/opt/homebrew/include/glib-2.0` GLib 2.90.0,
//     `/opt/homebrew/include/cairo` cairo 1.18.4), which is what `include/gtk.rs` cites from;
//     **GTK, GDK and Pango are not** (`ls -d /opt/homebrew/include/gtk-3.0
//     /opt/homebrew/include/gtk-4.0 /opt/homebrew/include/pango-1.0 /opt/homebrew/include/gdk-pixbuf-2.0`
//     -> all four absent; `pkg-config` is not installed; no `gtk/gtk.h`, `gdk/gdk.h` or
//     `pango/pango-attributes.h` under `/opt/homebrew`, `/usr/local` or `/Library`), so this unit's
//     citations of theirs name **the header alone, no line**, every value is a placeholder and no
//     layout claim is measured - `docs/ports/pgtkim.md` names the missing toolchain and the mirror
//     each gap belongs to.  What the arm flip below *does* prove is real: a `rustc` compile of this
//     unit under `have_pgtk`, which needs no system headers (the declarations are the port's own
//     and the symbols are only wanted at link time).
//
// `docs/ports/pgtkim.md` carries the C line ranges, the acceptance readings, every deviation and
// the items the integrator owes.  No behaviour of this file was executed anywhere: there is no PGTK
// target to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 21: `#include <config.h>`

/* C 23's chain, first: `"pgtkterm.h"` 23-26 reach `lisp.h` (via `dispextern.h`), and `lisp.h` is
   where `eassert`/`eassume`/`XUNTAG`/`XSETPVECTYPESIZE` come from.  The edge stands in the unit and
   *ahead of the mirror* because `macro_rules!` resolve in textual order: `include/pgtkterm.rs`
   expands `dispextern.rs`/`frame.rs`/`character.rs`/`font.rs`/`sysselect.rs` before its own
   `include!("lisp.rs")`, so a unit that reached the mirror first would call macros whose definitions
   still follow it (measured: 44 `cannot find macro` errors, every one anchored in those mirrors -
   `docs/ports/pgtkim.md`'s `## Findings` 2).  Every landed consumer of that mirror carries the same
   edge ahead of it (`src/pgtkterm.rs` 114, `src/gtkutil.rs`, `src/image.rs`, `src/xwidget.rs`).  */
#[cfg (feature = "have_pgtk")]
include!("include/lisp.rs");                   // pgtkterm.h 23-26's chain: `lisp.h` first

/* C 23: `#include "pgtkterm.h"`.  The mirror is `include/pgtkterm.rs`, and its own C 31
   `<gtk/gtk.h>` edge is still owed (`## Owed` of `docs/ports/include-pgtkterm.md`), so this unit
   carries the surface's stand-in edge at the C position that makes the names visible - the
   `src/pgtkterm.rs` 81 / `src/pgtkfns.rs` C 28-chain shape.  It carries **this file's own gate**,
   `have_pgtk` (C's whole text is that configuration's), and it drops when that mirror's edge lands
   (P-88 ⑤).  */
#[cfg (feature = "have_pgtk")]
include!("include/gtk.rs");                    // pgtkterm.h 31: `#include <gtk/gtk.h>` (stand-in)
#[cfg (feature = "have_pgtk")]
include!("include/pgtkterm.rs");               // C 23: `#include "pgtkterm.h"`
/* `strlen` (C 99-100) is `<string.h>`'s, which C reaches through `"lisp.h"` 30; `lisp.rs` mirrors
   the five headers it includes and `<string.h>` is not among them yet, so the edge stands here at
   the C position that needs it (the `src/pgtkfns.rs` C 26/`src/atimer.rs` 39 reading).  `sprintf`
   (C 59) needs no edge of its own: `lisp.rs` 35 already mirrors `<stdio.h>`.  */
#[cfg (feature = "have_pgtk")]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (strlen)

/* ----------------------------------------------------------------------------------------------
   C 23's chain: the Pango surface.  `src/pgtkim.c`'s only includes are C 21's `<config.h>` and
   C 23's `"pgtkterm.h"`, and Pango reaches it through that header's 31 `#include <gtk/gtk.h>`
   (under `HAVE_PGTK`).  `<pango/pango-attributes.h>` has **no file in the reference tree**, so
   P-88 ① puts its declarations in the unit rather than in a new file under `femacs/src/include/`:
   the surface is named at C 23's position, the C names are spelled as the header spells them, and
   the four `PangoAttrType` / three `PangoUnderline` members C 108-135 names are carried with a
   placeholder note.  The mirror they belong to is named in `docs/ports/pgtkim.md`'s `## Owed`
   (P-90 ①: `src/gtkutil.c` C 2892-2969 is the second consumer, so the surface wants one home of
   its own rather than a second in-unit copy).

   **Every number below is a placeholder and UNVERIFIED** (P-88 ②: no values from memory).  The
   enumerators carry the ordinal the header's own member list documents - transcribed as a
   *citation*, not as a measurement - and no size or offset is asserted for the three records,
   because no Pango header exists on this machine to measure.  `## Owed` of the ledger names the
   toolchain that measures them.  ------------------------------------------------------------------ */
/// C: pango/pango-attributes.h: `typedef struct _PangoAttrClass PangoAttrClass;` - the first
/// member's pointee.  **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct PangoAttrClass
{
  _private: [u8; 0],
}
/// C: pango/pango-attributes.h: `typedef struct _PangoAttrList PangoAttrList;` (C 70 holds one).
/// **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct PangoAttrList
{
  _private: [u8; 0],
}
/// C: pango/pango-attributes.h: `typedef struct _PangoAttrIterator PangoAttrIterator;` (C 89
/// holds one).  **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct PangoAttrIterator
{
  _private: [u8; 0],
}
/// C: pango/pango-color.h (reached through `pango/pango-attributes.h`): `struct _PangoColor
/// { guint16 red; guint16 green; guint16 blue; };` - the record `pac->color` is (C 60).
/// `guint16` is spelled `c_ushort`, the spelling `include/gtk.rs` 366-368 already uses for the same
/// GLib type.  **UNVERIFIED** (its own alignment and the three offsets are owed).
#[cfg (feature = "have_pgtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct PangoColor
{
  pub red: c_ushort,      // C: `guint16 red;`
  pub green: c_ushort,    // C: `guint16 green;`
  pub blue: c_ushort,     // C: `guint16 blue;`
}
/// C: pango/pango-attributes.h: `struct _PangoAttribute { const PangoAttrClass *klass; guint
/// start_index; guint end_index; };` - the prologue the two derived records below start with, and
/// the reason their payloads sit behind a pointer-sized member.  **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct PangoAttribute
{
  pub klass: *const PangoAttrClass,   // C: `const PangoAttrClass *klass;`
  pub start_index: c_uint,            // C: `guint start_index;`
  pub end_index: c_uint,              // C: `guint end_index;`
}
/// C: pango/pango-attributes.h: `struct _PangoAttrInt { PangoAttribute attr; int value; };` -
/// C 107-111's cast target and its `ul->value`.  **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct PangoAttrInt
{
  pub attr: PangoAttribute,   // C: `PangoAttribute attr;`
  pub value: c_int,           // C: `int value;`
}
/// C: pango/pango-attributes.h: `struct _PangoAttrColor { PangoAttribute attr; PangoColor color;
/// };` - C 115-137's cast target and its `pac->color`.  **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct PangoAttrColor
{
  pub attr: PangoAttribute,   // C: `PangoAttribute attr;`
  pub color: PangoColor,      // C: `PangoColor color;`
}
/// C: pango/pango-attributes.h: `typedef enum { PANGO_ATTR_INVALID, PANGO_ATTR_FAMILY,
/// PANGO_ATTR_STYLE, PANGO_ATTR_WEIGHT, PANGO_ATTR_VARIANT, PANGO_ATTR_STRETCH, PANGO_ATTR_SIZE,
/// PANGO_ATTR_FONT_DESC, PANGO_ATTR_FOREGROUND, PANGO_ATTR_BACKGROUND, PANGO_ATTR_UNDERLINE,
/// PANGO_ATTR_STRIKETHROUGH, PANGO_ATTR_RISE, PANGO_ATTR_SHAPE, PANGO_ATTR_SCALE,
/// PANGO_ATTR_FALLBACK, PANGO_ATTR_LETTER_SPACING, PANGO_ATTR_UNDERLINE_COLOR, … } PangoAttrType;`
/// - the port carries the four members C 108-135 names.  Each value is the member's ordinal in
/// that list, **UNVERIFIED** and owed (`## Owed`).
#[cfg (feature = "have_pgtk")]
pub type PangoAttrType = c_int;
#[cfg (feature = "have_pgtk")]
pub const PANGO_ATTR_FOREGROUND: PangoAttrType = 8;         // placeholder, not Pango's value
#[cfg (feature = "have_pgtk")]
pub const PANGO_ATTR_BACKGROUND: PangoAttrType = 9;         // placeholder, not Pango's value
#[cfg (feature = "have_pgtk")]
pub const PANGO_ATTR_UNDERLINE: PangoAttrType = 10;         // placeholder, not Pango's value
#[cfg (feature = "have_pgtk")]
pub const PANGO_ATTR_UNDERLINE_COLOR: PangoAttrType = 17;   // placeholder, not Pango's value
/// C: pango/pango-attributes.h: `typedef enum { PANGO_UNDERLINE_NONE, PANGO_UNDERLINE_SINGLE, … }
/// PangoUnderline;` - C 111's test.  **UNVERIFIED**; the value is the first member's ordinal.
#[cfg (feature = "have_pgtk")]
pub const PANGO_UNDERLINE_NONE: c_int = 0;                  // placeholder, not Pango's value
/// C: pango/pango-attributes.h: `PangoAttrIterator *pango_attr_list_get_iterator (PangoAttrList
/// *list);` - C 90.  **UNVERIFIED** (header absent).
#[cfg (feature = "have_pgtk")]
unsafe extern "C"
{
  pub fn pango_attr_list_get_iterator (list: *mut PangoAttrList) -> *mut PangoAttrIterator;
  /// C: pango/pango-attributes.h: `void pango_attr_iterator_range (PangoAttrIterator *iterator,
  /// gint *start, gint *end);` - C 97.
  pub fn pango_attr_iterator_range (iterator: *mut PangoAttrIterator, start: *mut c_int,
                                    end: *mut c_int);
  /// C: pango/pango-attributes.h: `PangoAttribute *pango_attr_iterator_get (PangoAttrIterator
  /// *iterator, PangoAttrType type);` - C 108/119/128/134, cast to the two derived records.
  pub fn pango_attr_iterator_get (iterator: *mut PangoAttrIterator, r#type: PangoAttrType)
                                  -> *mut PangoAttribute;
  /// C: pango/pango-attributes.h: `gboolean pango_attr_iterator_next (PangoAttrIterator
  /// *iterator);` - C 142's loop condition.
  pub fn pango_attr_iterator_next (iterator: *mut PangoAttrIterator) -> gboolean;
  /// C: pango/pango-attributes.h: `void pango_attr_list_unref (PangoAttrList *list);` - C 148.
  pub fn pango_attr_list_unref (list: *mut PangoAttrList);
}

/* ----------------------------------------------------------------------------------------------
   C 23's chain, second segment: the eight `gtk_im_context_*` entry points.  They are GTK's
   (`gtk/gtkimcontext.h`), and the GTK surface's single home is `include/gtk.rs` - which already
   carries this unit's two neighbours (`gtk_im_multicontext_new` and
   `gtk_im_context_set_use_preedit`, its C 1276-1284), so these eight are a **named mirror gap** and
   belong in that same file (P-90 ②).  They stand here at C 23's position because this unit is their
   first consumer, and `docs/ports/pgtkim.md`'s `## Owed` names the mirror the move goes to.
   **UNVERIFIED** (no GTK header on this machine: signatures are the header's, no line, no layout).
   ---------------------------------------------------------------------------------------------- */
#[cfg (feature = "have_pgtk")]
unsafe extern "C"
{
  /// C: gtk/gtkimcontext.h: `void gtk_im_context_reset (GtkIMContext *context);` - C 176/193/232.
  pub fn gtk_im_context_reset (context: *mut GtkIMContext);
  /// C: gtk/gtkimcontext.h: `void gtk_im_context_focus_in (GtkIMContext *context);` - C 180.
  pub fn gtk_im_context_focus_in (context: *mut GtkIMContext);
  /// C: gtk/gtkimcontext.h: `void gtk_im_context_focus_out (GtkIMContext *context);` - C 194/233.
  pub fn gtk_im_context_focus_out (context: *mut GtkIMContext);
  /// C: gtk/gtkimcontext.h: `void gtk_im_context_set_client_window (GtkIMContext *context,
  /// GdkWindow *window);` - C 177-179/195/234 (the `NULL` arm is C's `NULL`).
  pub fn gtk_im_context_set_client_window (context: *mut GtkIMContext, window: *mut GdkWindow);
  /// C: gtk/gtkimcontext.h: `gboolean gtk_im_context_filter_keypress (GtkIMContext *context,
  /// GdkEventKey *event);` - C 207.
  pub fn gtk_im_context_filter_keypress (context: *mut GtkIMContext, event: *mut GdkEventKey)
                                         -> gboolean;
  /// C: gtk/gtkimcontext.h: `void gtk_im_context_get_preedit_string (GtkIMContext *context, gchar
  /// **str, PangoAttrList **attrs, int *cursor_pos);` - C 78.
  pub fn gtk_im_context_get_preedit_string (context: *mut GtkIMContext, str: *mut *mut gchar,
                                            attrs: *mut *mut PangoAttrList,
                                            cursor_pos: *mut c_int);
  /// C: gtk/gtkimcontext.h: `void gtk_im_context_set_surrounding (GtkIMContext *context, const
  /// gchar *text, gint len, gint cursor_index);` - C 44 (the `""` text and the `-1` length).
  pub fn gtk_im_context_set_surrounding (context: *mut GtkIMContext, text: *const gchar,
                                         len: gint, cursor_index: gint);
  /// C: gtk/gtkimcontext.h: `void gtk_im_context_set_cursor_location (GtkIMContext *context, const
  /// GdkRectangle *area);` - C 221 (`&area` of C 220's local).
  pub fn gtk_im_context_set_cursor_location (context: *mut GtkIMContext,
                                             area: *const GdkRectangle);
}

/* C 25-39: the "commit" handler.  C's `gpointer user_data` is passed where a
   `struct pgtk_display_info *` is wanted; the port spells the conversion C performs implicitly
   (PORTING.md 1.4), the `src/gfilenotify.rs` 199 shape - and the declaration is `unsafe extern "C"
   fn` because that is what `G_CALLBACK!` transmutes (`include/gtk.rs` 141-148).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn im_context_commit_cb (imc: *mut GtkIMContext, str: *mut gchar,
                                           user_data: gpointer)
{
  let dpyinfo: *mut pgtk_display_info = user_data as *mut pgtk_display_info;
  let f: *mut frame = (*dpyinfo).im.focused_frame;

  if (*dpyinfo).im.context.is_null ()
    { return; }
  if f.is_null ()
    { return; }

  pgtk_enqueue_string (f, str);
}

/* C 41-46: the "retrieve-surrounding" handler.  C's empty text is `""`; the port spells it as the
   C string literal Rust needs.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn im_context_retrieve_surrounding_cb (imc: *mut GtkIMContext,
                                                         user_data: gpointer) -> gboolean
{
  gtk_im_context_set_surrounding (imc, c"".as_ptr (), -1, 0);
  return TRUE;
}

/* C 48-53: the "delete-surrounding" handler, whose `offset` and `n_chars` C does not read.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn im_context_delete_surrounding_cb (imc: *mut GtkIMContext, offset: c_int,
                                                       n_chars: c_int, user_data: gpointer)
                                                       -> gboolean
{
  return TRUE;
}

/* C 55-62: the colour helper.  C leaves `buf` uninitialised and C's `guint16 >> 8` is an `int`
   after the integer promotion (`%02x` reads an `unsigned int`), so the port promotes the three
   members before the shift and spells C's plain `char buf[256]` as the zero-filled array Rust
   requires (PORTING.md 0.1-13).  */
#[cfg (feature = "have_pgtk")]
unsafe fn make_color_string (pac: *mut PangoAttrColor) -> Lisp_Object
{
  let mut buf: [c_char; 256] = [0; 256];
  sprintf (buf.as_mut_ptr (), c"#%02x%02x%02x".as_ptr (),
           ((*pac).color.red as c_int) >> 8, ((*pac).color.green as c_int) >> 8,
           ((*pac).color.blue as c_int) >> 8);
  return build_string (buf.as_ptr ());
}

/* C 64-149: the "preedit-changed" handler.  C's `do { … } while (pango_attr_iterator_next (iter));`
   is carried as a `loop` whose trailing `if ! … { break; }` is C's condition; C 101-102's `continue`
   is the same edge - it jumps to that condition, not past it - so the port spells it by guarding the
   rest of the body with C's own negated test (`if st < ed { … }`) rather than by a Rust `continue`,
   which would skip the `break` test and spin (PORTING.md 2.2; `## Deviations` of the ledger).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn im_context_preedit_changed_cb (imc: *mut GtkIMContext, user_data: gpointer)
{
  let dpyinfo: *mut pgtk_display_info = user_data as *mut pgtk_display_info;
  let f: *mut frame = (*dpyinfo).im.focused_frame;
  /* C 69-71 declares `char *str; PangoAttrList *attrs; int pos;` and lets
     `gtk_im_context_get_preedit_string` fill all three in; the port gives them C's zero value first
     (PORTING.md 0.1-13) so the raw pointers below have an initialised place to point at.  */
  let mut str: *mut c_char = std::ptr::null_mut ();
  let mut attrs: *mut PangoAttrList = std::ptr::null_mut ();
  let mut pos: c_int = 0;

  if (*dpyinfo).im.context.is_null ()
    { return; }
  if f.is_null ()
    { return; }

  gtk_im_context_get_preedit_string (imc, &raw mut str, &raw mut attrs, &raw mut pos);

  /* C 81-86:
     (
       (TEXT (ul . COLOR) (bg . COLOR) (fg . COLOR))
       ...
     )  */
  let mut list: Lisp_Object = Qnil ();

  let mut iter: *mut PangoAttrIterator;
  iter = pango_attr_list_get_iterator (attrs);
  loop
    {
      let mut st: c_int = 0;
      let mut ed: c_int = 0;
      let mut has_underline: c_int = 0;
      let mut part: Lisp_Object = Qnil ();

      pango_attr_iterator_range (iter, &raw mut st, &raw mut ed);

      if (ed as usize) > strlen (str)
	{ ed = strlen (str) as c_int; }
      if st >= ed
	{ }
      else
	{
	  let text: Lisp_Object = make_string (str.add (st as usize),
					       (ed - st) as ptrdiff_t);
	  part = Fcons (text, part);

	  let ul: *mut PangoAttrInt =
	    pango_attr_iterator_get (iter, PANGO_ATTR_UNDERLINE) as *mut PangoAttrInt;
	  if ! ul.is_null ()
	    {
	      if (*ul).value != PANGO_UNDERLINE_NONE
		{ has_underline = 1; }
	    }

	  let mut pac: *mut PangoAttrColor;
	  if has_underline != 0
	    {
	      pac = pango_attr_iterator_get (iter, PANGO_ATTR_UNDERLINE_COLOR)
		    as *mut PangoAttrColor;
	      if ! pac.is_null ()
		{ part = Fcons (Fcons (Qul (), make_color_string (pac)), part); }
	      else
		{ part = Fcons (Fcons (Qul (), Qt ()), part); }
	    }

	  pac = pango_attr_iterator_get (iter, PANGO_ATTR_FOREGROUND) as *mut PangoAttrColor;
	  if ! pac.is_null ()
	    { part = Fcons (Fcons (Qfg (), make_color_string (pac)), part); }

	  pac = pango_attr_iterator_get (iter, PANGO_ATTR_BACKGROUND) as *mut PangoAttrColor;
	  if ! pac.is_null ()
	    { part = Fcons (Fcons (Qbg (), make_color_string (pac)), part); }

	  part = Fnreverse (part);
	  list = Fcons (part, list);
	}

      if pango_attr_iterator_next (iter) == 0
	{ break; }
    }

  list = Fnreverse (list);
  pgtk_enqueue_preedit (f, list);

  g_free (str as gpointer);
  pango_attr_list_unref (attrs);
}

/* C 151-163: the "preedit-end" handler.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn im_context_preedit_end_cb (imc: *mut GtkIMContext, user_data: gpointer)
{
  let dpyinfo: *mut pgtk_display_info = user_data as *mut pgtk_display_info;
  let f: *mut frame = (*dpyinfo).im.focused_frame;

  if (*dpyinfo).im.context.is_null ()
    { return; }
  if f.is_null ()
    { return; }

  pgtk_enqueue_preedit (f, Qnil ());
}

/* C 165-169: the "preedit-start" handler, whose body is empty and whose two parameters C does not
   name.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn im_context_preedit_start_cb (imc: *mut GtkIMContext, user_data: gpointer)
{
}

/* C 171-183.  `pgtk_im_focus_in` is one of the six this unit defines and `include/pgtkterm.rs`
   declares (its C 664); the definition shadows that glob import (PORTING.md 0.1-10).  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_im_focus_in (f: *mut frame)
{
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (f);
  if ! (*dpyinfo).im.context.is_null ()
    {
      gtk_im_context_reset ((*dpyinfo).im.context);
      gtk_im_context_set_client_window ((*dpyinfo).im.context,
					gtk_widget_get_window (FRAME_GTK_WIDGET! (f)));
      gtk_im_context_focus_in ((*dpyinfo).im.context);
    }
  (*dpyinfo).im.focused_frame = f;
}

/* C 185-199.  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_im_focus_out (f: *mut frame)
{
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (f);
  if (*dpyinfo).im.focused_frame == f
    {
      if ! (*dpyinfo).im.context.is_null ()
	{
	  gtk_im_context_reset ((*dpyinfo).im.context);
	  gtk_im_context_focus_out ((*dpyinfo).im.context);
	  gtk_im_context_set_client_window ((*dpyinfo).im.context, std::ptr::null_mut ());
	}
      (*dpyinfo).im.focused_frame = std::ptr::null_mut ();
    }
}

/* C 201-211.  C's `if (gtk_im_context_filter_keypress (…))` is a `gboolean` test, spelled `!= 0`.  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_im_filter_keypress (f: *mut frame, ev: *mut GdkEventKey) -> bool
{
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (f);
  if ! (*dpyinfo).im.context.is_null ()
    {
      if gtk_im_context_filter_keypress ((*dpyinfo).im.context, ev) != 0
	{ return true; }
    }
  return false;
}

/* C 213-223.  C's parameter list is the one `pgtkterm.h` 667-668 declares - four `int`s, `height`
   last; `include/pgtkterm.rs`'s prototype spells something else, and its note is wrong (see
   `docs/ports/pgtkim.md`'s `## Findings`).  C 220's aggregate initialiser is Rust's struct literal,
   and C's `&area` is `&raw const` (a raw pointer rather than a Rust reference, PORTING.md 1.2).  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_im_set_cursor_location (f: *mut frame, x: c_int, y: c_int,
						       width: c_int, height: c_int)
{
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (f);
  if ! (*dpyinfo).im.context.is_null () && (*dpyinfo).im.focused_frame == f
    {
      let mut area: GdkRectangle = GdkRectangle { x, y, width, height };
      gtk_im_context_set_cursor_location ((*dpyinfo).im.context, &raw const area);
    }
}

/* C 225-267: the context's owner.  C's six `g_signal_connect` calls are GLib's *macro*
   (`gsignal.h` 554-556; `include/gtk.rs` 204-235), so the port calls the mirror's `g_signal_connect!`
   and the reference emits `g_signal_connect_data` alone - the `src/gfilenotify.rs` 343-346 reading
   of the same construct.  */
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_im_use_context (dpyinfo: *mut pgtk_display_info, use_p: bool)
{
  if ! use_p
    {
      if ! (*dpyinfo).im.context.is_null ()
	{
	  gtk_im_context_reset ((*dpyinfo).im.context);
	  gtk_im_context_focus_out ((*dpyinfo).im.context);
	  gtk_im_context_set_client_window ((*dpyinfo).im.context, std::ptr::null_mut ());

	  g_object_unref ((*dpyinfo).im.context as gpointer);
	  (*dpyinfo).im.context = std::ptr::null_mut ();
	}
    }
  else
    {
      if (*dpyinfo).im.context.is_null ()
	{
	  (*dpyinfo).im.context = gtk_im_multicontext_new ();
	  g_signal_connect! ((*dpyinfo).im.context, c"commit".as_ptr (),
			     im_context_commit_cb, dpyinfo as gpointer);
	  g_signal_connect! ((*dpyinfo).im.context, c"retrieve-surrounding".as_ptr (),
			     im_context_retrieve_surrounding_cb,
			     dpyinfo as gpointer);
	  g_signal_connect! ((*dpyinfo).im.context, c"delete-surrounding".as_ptr (),
			     im_context_delete_surrounding_cb,
			     dpyinfo as gpointer);
	  g_signal_connect! ((*dpyinfo).im.context, c"preedit-changed".as_ptr (),
			     im_context_preedit_changed_cb,
			     dpyinfo as gpointer);
	  g_signal_connect! ((*dpyinfo).im.context, c"preedit-end".as_ptr (),
			     im_context_preedit_end_cb, dpyinfo as gpointer);
	  g_signal_connect! ((*dpyinfo).im.context, c"preedit-start".as_ptr (),
			     im_context_preedit_start_cb,
			     dpyinfo as gpointer);
	  gtk_im_context_set_use_preedit ((*dpyinfo).im.context, TRUE);

	  if ! (*dpyinfo).im.focused_frame.is_null ()
	    { pgtk_im_focus_in ((*dpyinfo).im.focused_frame); }
	}
    }
}

/* C 269-275.  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_im_init (dpyinfo: *mut pgtk_display_info)
{
  (*dpyinfo).im.context = std::ptr::null_mut ();

  pgtk_im_use_context (dpyinfo, ! NILP (Vpgtk_use_im_context_on_new_connection));
}

/* C 277-283.  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_im_finish (dpyinfo: *mut pgtk_display_info)
{
  if ! (*dpyinfo).im.context.is_null ()
    { g_object_unref ((*dpyinfo).im.context as gpointer); }
  (*dpyinfo).im.context = std::ptr::null_mut ();
}

/* C 285-294: the file's only subr.  C's docstring is carried verbatim (C 286).  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-use-im-context", Fpgtk_use_im_context, Spgtk_use_im_context, 1, 2, 0,
       doc: "Set whether to use GtkIMContext.",                   // C 286
       (use_p: Lisp_Object, terminal: Lisp_Object),
       {
  let dpyinfo: *mut pgtk_display_info = check_pgtk_display_info (terminal);

  pgtk_im_use_context (dpyinfo, ! NILP (use_p));

  return Qnil ();
});

/* C 306-310's `DEFVAR_LISP` variable.  C's storage is `globals.f_<vname>` (the macro's expansion)
   and `struct emacs_globals globals` is defined once, in alloc.c, so `include/globals.rs` mirrors
   the name as a *declaration* and the unit that owns it defines it here (status 3.29, the
   `src/gnutls.rs` 1824 shape; `Vpgtk_use_im_context_on_new_connection` is `include/globals.rs`
   6681's row).  The gate is the file's, so the frozen configuration emits no such symbol.  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub static mut Vpgtk_use_im_context_on_new_connection: Lisp_Object = lisp_h_Qnil ();

/* C 296-311.  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_pgtkim ()
{
  defsubr (&raw mut Spgtk_use_im_context);

  DEFSYM! (Qpgtk_refresh_preedit, "pgtk-refresh-preedit");
  DEFSYM! (Qul, "ul");
  DEFSYM! (Qfg, "fg");
  DEFSYM! (Qbg, "bg");

  DEFVAR_LISP! (c"pgtk-use-im-context-on-new-connection",
		Vpgtk_use_im_context_on_new_connection,
		"Whether to use GtkIMContext on a new connection.\nIf you want to change it after connection, use the `pgtk-use-im-context'\nfunction.  ");
  Vpgtk_use_im_context_on_new_connection = Qt ();
}

/* C 311's closing brace is the reference's last line; C 20's, C 22's and C 24's blank lines are the
   only lines this unit carries no row for (`docs/ports/pgtkim.md`'s `## C line ranges`).  */
