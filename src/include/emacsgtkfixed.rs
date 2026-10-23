// femacs: port of src/emacsgtkfixed.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header is the GTK 3 widget C shrinks to fit
// (the file says so itself), and on the frozen configuration **nothing here is compiled**:
// `HAVE_PGTK` is undefined (docs/baseline/config/darwin-arm64.macros.tsv), `src/emacsgtkfixed.c`
// is not in the frozen build's source list (its platform object variables are empty, so
// `src/emacsgtkfixed.rs` is the empty-shape note), and C 24's `<gtk/gtk.h>` has no mirror
// on this target - the port's platform rule (PORTING.md P-08/0.1-9) is that the platform
// target's own headers own those names.
//
// Two records are therefore **deferred rather than invented**: `struct _EmacsFixed` (C 39-45) and
// `struct _EmacsFixedClass` (C 47-50) each hold a GTK record **by value** (`GtkFixed container`,
// `GtkFixedClass parent_class`), and no compiler on this target has `<gtk/gtk.h>` to measure them
// with.  A zero-sized stand-in would be a silent layout lie and a byte array would encode a size
// nobody measured, so the checklist boxes stay unchecked and the extractor names the deferral on
// stderr (the `haikufont_info`/`w32common.h` reading, status 3.154/3.164).  They land when a GTK
// target is frozen.

// C 21-22: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const EMACSGTKFIXED_H: bool = true;

/// C 26 and C 35: `struct frame;` - a forward declaration this header does not define (it is
/// repeated twice in C, at 26 and 35); `emacs_fixed_new`'s parameter below is untyped for that
/// reason and a consumer converts at the call site (P-55/P-72 rule 3).
/// C: struct frame;

/// C 28/C 58: `G_BEGIN_DECLS`/`G_END_DECLS` are glib's `extern "C" { … }` braces, which Rust's
/// own `extern` blocks make unnecessary; they carry no name of their own.

/// C 30-33: the two type macros live inside `#ifdef HAVE_PGTK` (undefined here - dead text).
/// C: #define EMACS_TYPE_FIXED emacs_fixed_get_type ()
/// C: #define EMACS_IS_FIXED(obj) G_TYPE_CHECK_INSTANCE_TYPE (obj, EMACS_TYPE_FIXED)

/// C 37: `typedef struct _EmacsFixedPrivate EmacsFixedPrivate;` - a typedef of an incomplete
/// record, so it is a `/// C:` note rather than a Rust item; the record's one reader is the
/// deferred `struct _EmacsFixed` below.
/// C: typedef struct _EmacsFixedPrivate EmacsFixedPrivate;

/// C 39-45 / C 47-50: `struct _EmacsFixed` and `struct _EmacsFixedClass` - deferred, see the note
/// at the top of this file.

/// C 52-54: `emacs_fixed_get_type` is under `#ifdef HAVE_PGTK` (dead text here).
/// C: GType emacs_fixed_get_type (void);

/// C 56: the widget constructor `src/emacsgtkfixed.c` defines - the one live declaration of the
/// header (`struct frame *` is P-55's untyped pointer; `GtkWidget *` is GTK's, which the platform
/// target owns).
/// C: extern GtkWidget *emacs_fixed_new (struct frame *f);
pub mod emacsgtkfixed_decls {
  use super::*;

  unsafe extern "C" {
    pub fn emacs_fixed_new (f: *mut c_void) -> *mut c_void;
  }
}
use emacsgtkfixed_decls::*;

/* src/emacsgtkfixed.h 37-50: the GTK widget's records - `struct _EmacsFixed { GtkFixed
   container; EmacsFixedPrivate *priv; }` and `struct _EmacsFixedClass { GtkFixedClass
   parent_class; }`.  Dead platform text on the frozen target (no GTK/PGTK), carried under
   the header's own `HAVE_PGTK` guard name (P-51/3.9); `GtkFixed`/`GtkFixedClass` and the
   private-struct pointer keep C's spellings, which nothing compiles here.  */
#[cfg (feature = "have_pgtk")]
#[repr(C)]
pub struct _EmacsFixed
{
  /* GtkFixed container;  */
  pub container: *mut c_void,
  /* EmacsFixedPrivate *priv;  (C's `priv` is a Rust keyword: `r#priv`)  */
  pub r#priv: *mut c_void,
}
#[cfg (feature = "have_pgtk")]
#[repr(C)]
pub struct _EmacsFixedClass
{
  /* GtkFixedClass parent_class;  */
  pub parent_class: *mut c_void,
}
