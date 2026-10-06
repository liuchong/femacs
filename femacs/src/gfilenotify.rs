// femacs: port of src/gfilenotify.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The *build-level* file gate is `feature = "have_gfilenotify"` (P-86 ⑧): the
// frozen configuration's `configure` leaves `HAVE_GFILENOTIFY` undefined
// (`docs/baseline/config/darwin-arm64.macros.tsv` 1376), so no object is built from the
// reference either and this target's `build-tu` object is the 336-byte empty one - while the
// text is carried **in full**, because a GLib configuration compiles it.  The gate goes on every
// item *and on the include graph*.
//
// **GLib, GObject and GIO are on this machine** (`/opt/homebrew/include/glib-2.0`, GLib 2.90.0),
// so the GIO surface this file needs is quoted from the headers and its values are clang
// measurements (`## Evidence` in `docs/ports/gfilenotify.md`); the GLib/GObject half
// (`GError`, `g_object_unref`, `g_signal_connect`'s macro and `g_signal_connect_data`, `g_free`,
// `GType`, `GCallback`) lives in
// `include/gtk.rs` (P-90 ②: one surface, one mirror).  `<gio/gio.h>` itself has no mirror in
// this tree and this is its only consumer, so its declarations stand at C 21's position
// (P-88 ①); if a second consumer appears they move into a mirror of their own in the same
// commit (P-88 ⑤).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, clippy::all)]

include!("include/config.rs");                  // C 19: `#include <config.h>`
#[cfg (feature = "have_gfilenotify")]
include!("include/gtk.rs");                     // C 21's GLib half (P-90 ②)
#[cfg (feature = "have_gfilenotify")]
include!("include/globals.rs");                 // C 22's chain: lisp.h's `F*` declarations
#[cfg (feature = "have_gfilenotify")]
include!("include/lisp.rs");                    // C 22: `#include "lisp.h"`
#[cfg (feature = "have_gfilenotify")]
include!("include/coding.rs");                  // C 23: `#include "coding.h"` (`ENCODE_FILE`)
#[cfg (feature = "have_gfilenotify")]
include!("include/termhooks.rs");               // C 24: `#include "termhooks.h"` (`input_event`)
#[cfg (feature = "have_gfilenotify")]
include!("include/keyboard.rs");                // C 25: `#include "keyboard.h"` (`EVENT_INIT`,
                                                // `kbd_buffer_store_event`, `FILE_NOTIFY_EVENT`)
/* The four edges C 22-24's chain reaches through other headers, spelled here (P-86 ⑧; the
   mirror set supplies names through its consumers): `termhooks.h` names `Time` (systime.h) and
   `Emacs_Color` (dispextern.h), `lisp.h` names the two `character.h` width helpers.  */
#[cfg (feature = "have_gfilenotify")]
include!("include/systime.rs");                 // termhooks.h's `Time`
#[cfg (feature = "have_gfilenotify")]
include!("include/dispextern.rs");              // termhooks.h's `Emacs_Color`
#[cfg (feature = "have_gfilenotify")]
include!("include/character.rs");               // lisp.h's `string_char_and_length`/`raw_prev_char_len`
#[cfg (feature = "have_gfilenotify")]
include!("../lib/include/string.rs");           // lisp.h 30's `<string.h>` (`strcmp`/`strcpy`)

/* C 21: `<gio/gio.h>`, the GIO surface this file reads (P-88 ①).  Every `/// C:` line quotes
   the header declaration; the two enums and the two GObject prologue records are **clang
   measurements on this machine** (`## Evidence`), not recalled values.  */
/// C: gio/gioenums.h: `typedef enum { G_FILE_MONITOR_EVENT_CHANGED, … } GFileMonitorEvent;` -
/// measured 0..7 in declaration order (GIO 2.90.0).
#[cfg (feature = "have_gfilenotify")]
pub type GFileMonitorEvent = c_int;
#[cfg (feature = "have_gfilenotify")]
pub const G_FILE_MONITOR_EVENT_CHANGED: GFileMonitorEvent = 0;
#[cfg (feature = "have_gfilenotify")]
pub const G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT: GFileMonitorEvent = 1;
#[cfg (feature = "have_gfilenotify")]
pub const G_FILE_MONITOR_EVENT_DELETED: GFileMonitorEvent = 2;
#[cfg (feature = "have_gfilenotify")]
pub const G_FILE_MONITOR_EVENT_CREATED: GFileMonitorEvent = 3;
#[cfg (feature = "have_gfilenotify")]
pub const G_FILE_MONITOR_EVENT_ATTRIBUTE_CHANGED: GFileMonitorEvent = 4;
#[cfg (feature = "have_gfilenotify")]
pub const G_FILE_MONITOR_EVENT_PRE_UNMOUNT: GFileMonitorEvent = 5;
#[cfg (feature = "have_gfilenotify")]
pub const G_FILE_MONITOR_EVENT_UNMOUNTED: GFileMonitorEvent = 6;
#[cfg (feature = "have_gfilenotify")]
pub const G_FILE_MONITOR_EVENT_MOVED: GFileMonitorEvent = 7;
/// C: gio/gioenums.h: `typedef enum { G_FILE_MONITOR_NONE = 0, G_FILE_MONITOR_WATCH_MOUNTS = (1
/// << 0), G_FILE_MONITOR_SEND_MOVED = (1 << 1), … } GFileMonitorFlags;` - measured 0/1/2.
#[cfg (feature = "have_gfilenotify")]
pub type GFileMonitorFlags = c_int;
#[cfg (feature = "have_gfilenotify")]
pub const G_FILE_MONITOR_NONE: GFileMonitorFlags = 0;
#[cfg (feature = "have_gfilenotify")]
pub const G_FILE_MONITOR_WATCH_MOUNTS: GFileMonitorFlags = 1;
#[cfg (feature = "have_gfilenotify")]
pub const G_FILE_MONITOR_SEND_MOVED: GFileMonitorFlags = 2;
/// C: gio/gfile.h: `typedef struct _GFile GFile;` - opaque here.
#[cfg (feature = "have_gfilenotify")]
#[repr (C)]
pub struct GFile
{
  _private: [u8; 0],
}
/// C: gio/gfilemonitor.h: `typedef struct _GFileMonitor GFileMonitor;` - opaque here.
#[cfg (feature = "have_gfilenotify")]
#[repr (C)]
pub struct GFileMonitor
{
  _private: [u8; 0],
}
/// C: gio/giotypes.h: `typedef struct _GCancellable GCancellable;` - the `NULL` argument of
/// C 193's `g_file_monitor` (an untyped `NULL` in C is spelled with the record here).
#[cfg (feature = "have_gfilenotify")]
#[repr (C)]
pub struct GCancellable
{
  _private: [u8; 0],
}
/* The two GObject records and their three macros used to stand here; they are
   `include/gtk.rs`'s now (P-26's one spelling per construct, status 3.726) - this unit's include
   graph reaches that mirror (line 27), so the copies were a duplicate the moment the shared home
   gained them.  The call sites below are unchanged.  */
#[cfg (feature = "have_gfilenotify")]
unsafe extern "C" {
  /// C: gio/gfile.h: `GFile * g_file_new_for_path (const char *path);`
  pub fn g_file_new_for_path (path: *const c_char) -> *mut GFile;
  /// C: gio/gfilemonitor.h: `GFileMonitor * g_file_monitor (GFile *file, GFileMonitorFlags
  /// flags, GCancellable *cancellable, GError **error);`
  pub fn g_file_monitor (file: *mut GFile, flags: GFileMonitorFlags,
                         cancellable: *mut GCancellable, error: *mut *mut GError)
    -> *mut GFileMonitor;
  /// C: gio/gfilemonitor.h: `gboolean g_file_monitor_is_cancelled (GFileMonitor *monitor);`
  pub fn g_file_monitor_is_cancelled (monitor: *mut GFileMonitor) -> gboolean;
  /// C: gio/gfilemonitor.h: `gboolean g_file_monitor_cancel (GFileMonitor *monitor);`
  pub fn g_file_monitor_cancel (monitor: *mut GFileMonitor) -> gboolean;
  /// C: gio/gfilemonitor.h: `void g_file_monitor_set_rate_limit (GFileMonitor *monitor, gint
  /// limit_msecs);`
  pub fn g_file_monitor_set_rate_limit (monitor: *mut GFileMonitor, limit_msecs: gint);
  /// C: gio/gfile.h: `char * g_file_get_parse_name (GFile *file);`
  pub fn g_file_get_parse_name (file: *mut GFile) -> *mut c_char;
  /// C: gobject/gtype.h: `void g_type_init (void);` - deprecated in GLib 2.36 and guarded by
  /// C 301's `#if ! GLIB_CHECK_VERSION (2, 36, 0)` (both arms carried, P-51).
  pub fn g_type_init ();
}

/// C: termhooks.h 291: `, FILE_NOTIFY_EVENT` - an `enum input_event_kind` member inside C's
/// `#ifdef USE_FILE_NOTIFY`, which this file's own gate implies.  `include/termhooks.rs` carries
/// the frozen configuration's members (the gated `MULTIMEDIA_KEY_EVENT`, `NS_TEXT_EVENT`,
/// `NS_NONKEY_EVENT`, `XWIDGET_EVENT` and `XWIDGET_DISPLAY_EVENT` are absent there), so the
/// member's value in *this* configuration is 28 - and the members after it shift with it, which
/// the mirror's frozen values do not say.  Declared here (P-88 ①, the `src/xfns.rs` shape) and
/// owed to the mirror round (`## Owed` in `docs/ports/gfilenotify.md`).
#[cfg (feature = "have_gfilenotify")]
pub const FILE_NOTIFY_EVENT: c_int = 28;
/// C: lisp.h's `INLINE bool FUNCTIONP (Lisp_Object a) { return !NILP (Ffunctionp (a)); }` - no
/// mirror carries it (`include/globals.rs` has `Ffunctionp`), so the port spells the predicate
/// here, exactly as lisp.h's inline does.
#[cfg (feature = "have_gfilenotify")]
pub unsafe fn FUNCTIONP (a: Lisp_Object) -> bool
{
  !NILP (Ffunctionp (a))
}

/// termhooks.h 373-412's `enum event_kind kind : 16` shares `include/termhooks.rs`'s
/// `input_event::bits` with `part : 16` (PORTING.md 1.3/P-80).  C 95 writes `event.kind = …`, so
/// the port keeps C's field name in a per-unit macro - the `src/gtkutil.rs` 1757-1770 shape,
/// body-identical.
#[cfg (feature = "have_gfilenotify")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
}

/* C 26-29: the watch list.  C's file-scope `static Lisp_Object watch_list;` is a tentative
   definition, i.e. zero, which is `Qnil` (the `src/callproc.rs` 316 spelling).  */
#[cfg (feature = "have_gfilenotify")]
static mut watch_list: Lisp_Object = lisp_h_Qnil ();

/* C 31-123: `static gboolean dir_monitor_callback (GFileMonitor *monitor, GFile *file,
   GFile *other_file, GFileMonitorEvent event_type, gpointer user_data)` - the `GCallback`
   C 219 passes.  */
#[cfg (feature = "have_gfilenotify")]
unsafe extern "C" fn dir_monitor_callback (monitor: *mut GFileMonitor, file: *mut GFile,
                                           other_file: *mut GFile,
                                           event_type: GFileMonitorEvent,
                                           user_data: gpointer) -> gboolean
{
  /* C 41-43's locals; `name`/`oname` own the `g_free`s at C 119-120 (PORTING.md 1.5: C leaves
     the four Lisp_Objects uninitialised, Rust needs a value).  */
  let mut symbol: Lisp_Object = lisp_h_Qnil ();
  let mut monitor_object: Lisp_Object = lisp_h_Qnil ();
  let mut watch_object: Lisp_Object = lisp_h_Qnil ();
  let mut flags: Lisp_Object = lisp_h_Qnil ();
  let mut name: *mut c_char = g_file_get_parse_name (file);
  let mut oname: *mut c_char = if !other_file.is_null () { g_file_get_parse_name (other_file) }
                               else { core::ptr::null_mut () };

  /* C's `default: goto cleanup;` jumps over the body to the tail at C 118, and the body's own
     fall-through reaches the same tail - so one labelled block carries both (PORTING.md 2.2).  */
  'cleanup: {
    /* C 45-74: `switch (event_type)`, in C's arm order.  */
    match event_type
      {
        G_FILE_MONITOR_EVENT_CHANGED => { symbol = Qchanged (); }
        G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT => { symbol = Qchanges_done_hint (); }
        G_FILE_MONITOR_EVENT_DELETED => { symbol = Qdeleted (); }
        G_FILE_MONITOR_EVENT_CREATED => { symbol = Qcreated (); }
        G_FILE_MONITOR_EVENT_ATTRIBUTE_CHANGED => { symbol = Qattribute_changed (); }
        G_FILE_MONITOR_EVENT_PRE_UNMOUNT => { symbol = Qpre_unmount (); }
        G_FILE_MONITOR_EVENT_UNMOUNTED => { symbol = Qunmounted (); }
        G_FILE_MONITOR_EVENT_MOVED => { symbol = Qmoved (); }
        _ => { break 'cleanup; }
      }

    /* C 76-114: determine the callback function.  */
    monitor_object = make_pointer_integer (monitor as *mut c_void);
    watch_object = assq_no_quit (monitor_object, watch_list);

    if CONSP (watch_object)
      {
        let mut event: input_event = std::mem::zeroed ();
        let otail: Lisp_Object = if !oname.is_null () { list1 (build_string (oname)) }
                                 else { Qnil () };

        /* Check, whether event_type is expected.  */
        flags = XCAR (XCDR (XCDR (watch_object)));
        if (!NILP (Fmember (Qchange (), flags))
            && !NILP (Fmember (symbol, list5 (Qchanged (), Qchanges_done_hint (),
                                              Qdeleted (), Qcreated (), Qmoved ()))))
           || (!NILP (Fmember (Qattribute_change (), flags))
               && EQ (symbol, Qattribute_changed ()))
           || (!NILP (Fmember (Qwatch_mounts (), flags))
               && EQ (symbol, Qunmounted ()))
          {
            /* Construct an event.  */
            EVENT_INIT (&raw mut event);
            input_event_bits! (kind, &raw mut event, FILE_NOTIFY_EVENT);
            event.frame_or_window = Qnil ();
            event.arg = list2 (Fcons (monitor_object,
                                      Fcons (symbol,
                                             Fcons (build_string (name), otail))),
                               XCAR (XCDR (XCDR (XCDR (watch_object)))));

            /* Store it into the input event queue.  */
            kbd_buffer_store_event (&raw mut event as *mut c_void);
            /* XD_DEBUG_MESSAGE ("%s", XD_OBJECT_TO_STRING (event.arg));  */
          }

        /* Cancel monitor if file or directory is deleted or unmounted.  */
        if !NILP (Fmember (symbol, list3 (Qdeleted (), Qmoved (), Qunmounted ())))
           && strcmp (name, SSDATA (XCAR (XCDR (watch_object)))) == 0
           && g_file_monitor_is_cancelled (monitor) == 0
          { g_file_monitor_cancel (monitor); }
      }
  }

  /* Cleanup.  */
  g_free (name as gpointer);
  g_free (oname as gpointer);

  TRUE
}

/* C 125-227: the first subr.  */
DEFUN! ("gfile-add-watch", Fgfile_add_watch, Sgfile_add_watch, 3, 3, 0,
       doc: "Add a watch for filesystem events pertaining to FILE.\n\nThis arranges for filesystem events pertaining to FILE to be reported\nto Emacs.  Use `gfile-rm-watch' to cancel the watch.\n\nValue is a descriptor for the added watch.  If the file cannot be\nwatched for some reason, this function signals a `file-notify-error' error.\n\nFLAGS is a list of conditions to set what will be watched for.  It can\ninclude the following symbols:\n\n  `change'           -- watch for file changes\n  `attribute-change' -- watch for file attributes changes, like\n                        permissions or modification time\n  `watch-mounts'     -- watch for mount events\n  `send-moved'       -- pair `deleted' and `created' events caused by\n                        file renames and send a single `renamed' event\n                        instead\n\nWhen any event happens, Emacs will call the CALLBACK function passing\nit a single argument EVENT, which is of the form\n\n  (DESCRIPTOR ACTION FILE [FILE1])\n\nDESCRIPTOR is the same object as the one returned by this function.\nACTION is the description of the event.  It could be any one of the\nfollowing:\n\n  `changed'           -- FILE has changed\n  `changes-done-hint' -- a hint that this was probably the last change\n                         in a set of changes\n  `deleted'           -- FILE was deleted\n  `created'           -- FILE was created\n  `attribute-changed' -- a FILE attribute was changed\n  `pre-unmount'       -- the FILE location will soon be unmounted\n  `unmounted'         -- the FILE location was unmounted\n  `moved'             -- FILE was moved to FILE1\n\nFILE is the name of the file whose event is being reported.  FILE1\nwill be reported only in case of the `moved' event.",
       (file: Lisp_Object, flags: Lisp_Object, callback: Lisp_Object),
       {
  /* C's `file = Fdirectory_file_name (…)` rebinds the argument.  */
  let mut file = file;
  let mut watch_object: Lisp_Object = lisp_h_Qnil ();
  let mut gfile: *mut GFile;
  let mut monitor: *mut GFileMonitor;
  /* C 171: `GFileMonitorFlags gflags = G_FILE_MONITOR_NONE;` - the name collides with
     lisp.h's `struct gflags` at this scope (PORTING.md 0.2's renaming rule), so the
     local carries the trailing `_`.  */
  let mut gflags_: GFileMonitorFlags = G_FILE_MONITOR_NONE;
  let mut gerror: *mut GError = core::ptr::null_mut ();

  /* Check parameters.  */
  CHECK_STRING (file);
  file = Fdirectory_file_name (Fexpand_file_name (file, Qnil ()));
  if NILP (Ffile_exists_p (file))
    { report_file_error (c"File does not exist".as_ptr (), file); }

  if !FUNCTIONP (callback)
    { wrong_type_argument (Qinvalid_function (), callback); }

  /* Assemble flags.  */
  if !NILP (Fmember (Qwatch_mounts (), flags))
    { gflags_ |= G_FILE_MONITOR_WATCH_MOUNTS; }
  if !NILP (Fmember (Qsend_moved (), flags))
    { gflags_ |= G_FILE_MONITOR_SEND_MOVED; }

  /* Create GFile name.  */
  gfile = g_file_new_for_path (SSDATA (ENCODE_FILE (file)));

  /* Enable watch.  */
  monitor = g_file_monitor (gfile, gflags_, core::ptr::null_mut (), &raw mut gerror);
  g_object_unref (gfile as gpointer);
  if !gerror.is_null ()
    {
      let mut msg: [c_char; 1024] = [0; 1024];
      strcpy (&raw mut msg as *mut c_char, (*gerror).message);
      g_error_free (gerror);
      xsignal1 (Qfile_notify_error (), build_string (&raw mut msg as *mut c_char));
    }
  if monitor.is_null ()
    { xsignal2 (Qfile_notify_error (), build_string (c"Cannot watch file".as_ptr ()), file); }

  let watch_descriptor: Lisp_Object = make_pointer_integer_unsafe (monitor as *mut c_void);

  if !(FIXNUMP (watch_descriptor)
       && XFIXNUMPTR (watch_descriptor) as *mut GFileMonitor == monitor)
    {
      g_object_unref (monitor as gpointer);
      xsignal2 (Qfile_notify_error (), build_string (c"Unsupported file watcher".as_ptr ()),
                file);
    }

  /* The default rate limit is 800 msec.  We adapt this.  */
  g_file_monitor_set_rate_limit (monitor, 100);

  /* Subscribe to the "changed" signal.  C 219 spells the handler with a cast,
     `(GCallback) dir_monitor_callback`, and `g_signal_connect` itself is GLib's *macro*
     (gsignal.h 554-556) rather than an entry point: `include/gtk.rs` carries the macro
     (it applies `G_CALLBACK!` itself) and `g_signal_connect_data` underneath, so the call
     below goes through the mirror (P-90 ②).  */
  g_signal_connect! (monitor as gpointer, c"changed".as_ptr (),
                     dir_monitor_callback, core::ptr::null_mut ());

  /* Store watch object in watch list.  */
  watch_object = list4 (watch_descriptor, file, flags, callback);
  watch_list = Fcons (watch_object, watch_list);

  watch_descriptor
});

/* C 229-255: the second subr.  */
DEFUN! ("gfile-rm-watch", Fgfile_rm_watch, Sgfile_rm_watch, 1, 1, 0,
       doc: "Remove an existing WATCH-DESCRIPTOR.\n\nWATCH-DESCRIPTOR should be an object returned by `gfile-add-watch'.",
       (watch_descriptor: Lisp_Object),
       {
  let watch_object: Lisp_Object = assq_no_quit (watch_descriptor, watch_list);

  if !CONSP (watch_object)
    { xsignal2 (Qfile_notify_error (), build_string (c"Not a watch descriptor".as_ptr ()),
                 watch_descriptor); }

  eassert! (FIXNUMP (watch_descriptor));
  let monitor: *mut GFileMonitor = XFIXNUMPTR (watch_descriptor) as *mut GFileMonitor;
  if g_file_monitor_is_cancelled (monitor) == 0
     && g_file_monitor_cancel (monitor) == 0
    { xsignal2 (Qfile_notify_error (), build_string (c"Could not rm watch".as_ptr ()),
                 watch_descriptor); }

  /* Remove watch descriptor from watch list.  */
  watch_list = Fdelq (watch_object, watch_list);

  /* Cleanup.  */
  g_object_unref (monitor as gpointer);

  Qt ()
});

/* C 257-276: the third subr.  */
DEFUN! ("gfile-valid-p", Fgfile_valid_p, Sgfile_valid_p, 1, 1, 0,
       doc: "Check a watch specified by its WATCH-DESCRIPTOR.\n\nWATCH-DESCRIPTOR should be an object returned by `gfile-add-watch'.\n\nA watch can become invalid if the file or directory it watches is\ndeleted, or if the watcher thread exits abnormally for any other\nreason.  Removing the watch by calling `gfile-rm-watch' also makes it\ninvalid.",
       (watch_descriptor: Lisp_Object),
       {
  let watch_object: Lisp_Object = Fassoc (watch_descriptor, watch_list, Qnil ());
  if NILP (watch_object)
    { Qnil () }
  else
    {
      let monitor: *mut GFileMonitor = XFIXNUMPTR (watch_descriptor) as *mut GFileMonitor;
      if g_file_monitor_is_cancelled (monitor) != 0 { Qnil () } else { Qt () }
    }
});

/* C 278-295: the fourth subr.  */
DEFUN! ("gfile-monitor-name", Fgfile_monitor_name, Sgfile_monitor_name, 1, 1, 0,
       doc: "Return the internal monitor name for WATCH-DESCRIPTOR.\n\nThe result is a symbol, either `GInotifyFileMonitor',\n`GKqueueFileMonitor', `GFamFileMonitor', or `GPollFileMonitor'.\n\nWATCH-DESCRIPTOR should be an object returned by `gfile-add-watch'.\nIf WATCH-DESCRIPTOR is not valid, nil is returned.",
       (watch_descriptor: Lisp_Object),
       {
  if NILP (Fgfile_valid_p (watch_descriptor))
    { Qnil () }
  else
    {
      let monitor: *mut GFileMonitor = XFIXNUMPTR (watch_descriptor) as *mut GFileMonitor;
      intern (G_OBJECT_TYPE_NAME! (monitor))
    }
});

/* C 298-305: `void globals_of_gfilenotify (void)`.  */
#[cfg (feature = "have_gfilenotify")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn globals_of_gfilenotify ()
{
  /* C 301-303: `#if ! GLIB_CHECK_VERSION (2, 36, 0)` - both arms carried (P-51): the feature
     names the non-frozen side, so the frozen build takes the empty arm.  */
  #[cfg (feature = "glib_check_version_lt_2_36")]
  {
    g_type_init ();
  }
  watch_list = Qnil ();                    /* C 304 */
}

/* C 307-336: `void syms_of_gfilenotify (void)`.  */
#[cfg (feature = "have_gfilenotify")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_gfilenotify ()
{
  defsubr (&raw mut Sgfile_add_watch);
  defsubr (&raw mut Sgfile_rm_watch);
  defsubr (&raw mut Sgfile_valid_p);
  defsubr (&raw mut Sgfile_monitor_name);

  /* Filter objects.  */
  DEFSYM! (Qchange, "change");
  DEFSYM! (Qattribute_change, "attribute-change");
  DEFSYM! (Qwatch_mounts, "watch-mounts"); /* G_FILE_MONITOR_WATCH_MOUNTS  */
  DEFSYM! (Qsend_moved, "send-moved");	/* G_FILE_MONITOR_SEND_MOVED  */

  /* Event types.  */
  DEFSYM! (Qchanged, "changed");	/* G_FILE_MONITOR_EVENT_CHANGED  */
  DEFSYM! (Qchanges_done_hint, "changes-done-hint");
				/* G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT  */
  DEFSYM! (Qdeleted, "deleted");	/* G_FILE_MONITOR_EVENT_DELETED  */
  DEFSYM! (Qcreated, "created");	/* G_FILE_MONITOR_EVENT_CREATED  */
  DEFSYM! (Qattribute_changed, "attribute-changed");
				/* G_FILE_MONITOR_EVENT_ATTRIBUTE_CHANGED  */
  DEFSYM! (Qpre_unmount, "pre-unmount");	/* G_FILE_MONITOR_EVENT_PRE_UNMOUNT  */
  DEFSYM! (Qunmounted, "unmounted");	/* G_FILE_MONITOR_EVENT_UNMOUNTED  */
  DEFSYM! (Qmoved, "moved");	/* G_FILE_MONITOR_EVENT_MOVED  */

  staticpro (&raw mut watch_list);

  Fprovide (intern_c_string (c"gfilenotify".as_ptr ()), Qnil ());
}
