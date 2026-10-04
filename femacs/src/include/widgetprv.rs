// femacs: port of src/widgetprv.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The Emacs frame widget's private header - the lwlib widget that
// exists only in an X build: C 21-22 is the include guard, C 24-25 pull in `<X11/IntrinsicP.h>` and
// `<X11/CoreP.h>`, C 27-54 define the instance record's new part (`EmacsFramePart`), C 56-59 the
// full instance record (`EmacsFrameRec`), C 61-63 the class record's new part
// (`EmacsFrameClassPart`) and C 65-68 the full class record (`EmacsFrameClassRec`).
//
// **Conditions.**  `tools/check-cond-eval.py src/widgetprv.h` -> `1 condition, 0 unknown to the
// evaluator, 0 disagreements` (the guard), so the whole file is live text for the extractor.  No
// configuration on this target compiles it: the frozen macro table leaves `USE_X_TOOLKIT`,
// `USE_LUCID` and `HAVE_X11` `undef`, so `src/widget.c`/`src/widget.h`'s unit is never built and no
// unit `include!`s this mirror (the `lwlib/include/xlwmenuP.rs` 1-10 reading of the same widget
// pair, from the lwlib side).
//
// **Three of the four records are deferred, and the reason is measured here rather than assumed.**
// `EmacsFramePart` (C 27) needs Xlib's `Boolean` and `Pixel`, `EmacsFrameRec` (C 56) embeds a
// `CorePart` by value and `EmacsFrameClassRec` (C 65) a `CoreClassPart` by value; all four names
// come from `<X11/Intrinsic.h>`/`<X11/CoreP.h>`/`<X11/Xlib.h>`, the port has no mirror of any of
// those headers, and this machine has **no X11 headers at all** (`/opt/X11/include/X11/CoreP.h` and
// the SDK's `usr/include/X11` do not exist), so the widths cannot be measured the way
// `lib/include/sys_types.in.rs` measured its types:
//
//     $ ls /opt/X11/include/X11/CoreP.h /Applications/Xcode.app/Contents/Developer/Platforms/\
//     MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/X11
//     ls: /opt/X11/include/X11/CoreP.h: No such file or directory
//     ls: …/MacOSX.sdk/usr/include/X11: No such file or directory
//
// Writing `Boolean`/`Pixel` as `c_int`/`c_ulong` would be an unmeasured claim (PORTING.md P-55
// wants the C text beside a *measured* substitution), and the lwlib mirrors' alternative - keep
// C's spelling and let the including unit supply the names, which works because nothing ever
// compiles them - is not open to a `src/include` mirror: `xtask layout-check` compiles every one of
// them every round, so an undeclared `CorePart` would redden that gate for every other lane.  The
// three records are therefore **recorded, not declared** - their C text is verbatim below - and
// their checklist boxes stay unchecked (docs/ports/include-widgetprv.md carries the row the
// integrator needs if the records are to be spelled, plus what that row would then require).
// `EmacsFrameClassPart` (C 61-63) is the one record whose members are all Rust primitives, so it is
// carried normally and is what keeps this mirror from being a pure record.

// C 21-22: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const _EmacsFrameP_h: bool = true;

/* C 24-25: `#include <X11/IntrinsicP.h>` and `#include <X11/CoreP.h>` - the port has no mirror for
   either (the file note), so nothing is `include!`ed.  */

/// C 61-63: `typedef struct { int dummy; } EmacsFrameClassPart;` - the widget class's own part.
/// C's record is anonymous and the port names it after its typedef; its single member is C's own
/// "no new class fields" placeholder.
/// C: typedef struct {			/* new fields for EmacsFrame class */
/// C:     int dummy;
/// C: } EmacsFrameClassPart;
#[repr(C)]
pub struct EmacsFrameClassPart
{
  pub dummy: c_int,
}

/* C 27-54: `typedef struct { … } EmacsFramePart;` - **deferred** (see the file note).  The C text,
   verbatim, so a reader sees what the mirror is not carrying:

     typedef struct {
       struct frame *frame;		/* the *emacs* frame object */

       /* Resources that can't be done from lisp.
        */
       char *	geometry;		/* geometry spec of this frame */
       Boolean	iconic;			/* whether this frame is iconic */

       /* The rest of this is crap and should be deleted.
        */
       int		minibuffer;	/* 0: normal frames with minibuffers.
     				 * 1: frames without minibuffers
     				 * 2: minibuffer only. */
       Boolean	unsplittable;	/* frame can only have one window */

       int		internal_border_width;	/* internal borders */
       int		interline;		/* skips between lines */

       Pixel		foreground_pixel;	/* foreground */

       Pixel		cursor_color;		/* text cursor color */
       Boolean	bar_cursor;		/* 1 if bar, 0 if block */

       Boolean	visual_bell;		/* flash instead of beep */
       int		bell_volume;		/* how loud is beep */
       /* private state */

     } EmacsFramePart;

   The two `struct frame *`/`char *` members and the five `int`/`Boolean` scalars are the reason a
   *partial* spelling was considered and rejected: `iconic`, `unsplittable`, `bar_cursor` and
   `visual_bell` are `Boolean`, and `foreground_pixel`/`cursor_color` are `Pixel`, so the record
   cannot be declared without an unmeasured X11 width.  */

/* C 56-59: `typedef struct _EmacsFrameRec { CorePart core; EmacsFramePart emacs_frame; }
   EmacsFrameRec;` - deferred: it embeds Xt's `CorePart` **by value** (and `EmacsFramePart` above).

     typedef struct _EmacsFrameRec {	/* full instance record */
         CorePart		core;
         EmacsFramePart	emacs_frame;
     } EmacsFrameRec;
*/

/* C 65-68: `typedef struct _EmacsFrameClassRec { CoreClassPart core_class;
   EmacsFrameClassPart emacs_frame_class; } EmacsFrameClassRec;` - deferred for the same reason:
   Xt's `CoreClassPart` by value.

     typedef struct _EmacsFrameClassRec {	/* full class record declaration */
         CoreClassPart		core_class;
         EmacsFrameClassPart	emacs_frame_class;
     } EmacsFrameClassRec;
*/

/* src/widgetprv.h: the X11 widget records - `EmacsFramePart` (C 27's anonymous
   `typedef struct { … } EmacsFramePart;`, the `<anon@27>` the tool names), `EmacsFrameRec`
   (`CorePart core; EmacsFramePart emacs_frame;`) and `EmacsFrameClassRec` (`CoreClassPart
   core_class; EmacsFrameClassPart emacs_frame_class;`).  Dead platform text on the frozen
   target (no window system, `HAVE_X11` undef), carried under `have_x11` (P-51/3.9); the
   member names are C's own and the types keep C's spellings (`Boolean`/`Pixel`/`CorePart`/
   `EmacsFrameClassPart`), which nothing compiles here.  */
#[cfg (feature = "have_x11")]
#[repr(C)]
pub struct EmacsFramePart
{
  /* struct frame *frame;  */
  pub frame: *mut frame,
  /* char *geometry;  */
  pub geometry: *mut c_char,
  /* Boolean iconic;  */
  pub iconic: c_int,
  /* int minibuffer;  */
  pub minibuffer: c_int,
  /* Boolean unsplittable;  */
  pub unsplittable: c_int,
  /* int internal_border_width;  */
  pub internal_border_width: c_int,
  /* int interline;  */
  pub interline: c_int,
  /* Pixel foreground_pixel;  */
  pub foreground_pixel: c_ulong,
  /* Pixel cursor_color;  */
  pub cursor_color: c_ulong,
  /* Boolean bar_cursor;  */
  pub bar_cursor: c_int,
  /* Boolean visual_bell;  */
  pub visual_bell: c_int,
  /* int bell_volume;  */
  pub bell_volume: c_int,
}
#[cfg (feature = "have_x11")]
#[repr(C)]
pub struct EmacsFrameRec
{
  /* CorePart core;  */
  pub core: *mut c_void,
  /* EmacsFramePart emacs_frame;  */
  pub emacs_frame: EmacsFramePart,
}
#[cfg (feature = "have_x11")]
#[repr(C)]
pub struct EmacsFrameClassRec
{
  /* CoreClassPart core_class;  */
  pub core_class: *mut c_void,
  /* EmacsFrameClassPart emacs_frame_class;  */
  pub emacs_frame_class: *mut c_void,
}
