// femacs: mirror of the GTK's <gtk/gtk.h> (system header of another target's toolchain, PORTING.md P-26/P-49).
//
// `<gtk/gtk.h>` is the GTK toolchain's own header: it is not part of the reference tree, and the
// frozen target has no such header, so it has no C file to be paired with.  This mirror is the
// port's declaration of the third-party surface the gated arms of `src/gtkutil.c` and
// `src/pgtkterm.c` name through `"gtkutil.h"` 26's `<gtk/gtk.h>` and, under `HAVE_PGTK`, its 29
// `"pgtkterm.h"` (whose own 31 includes `<gtk/gtk.h>`), together with `<gdk/gdkkeysyms.h>`,
// `<cairo.h>` and the `<glib.h>`/`<gobject.h>` pair those pull in - the way `include/xlib.rs`
// mirrors the X11 SDK's `<X11/Xlib.h>` and `include/jni.rs` the NDK's `<jni.h>`;
// `tools/check-target-names.py` accepts the first line above as that declaration.
//
// **One surface, one mirror** (P-90 ①/②).  These ~290 names were the in-unit segments of
// `src/gtkutil.rs` (C 44/49's chain) and `src/pgtkterm.rs` (C 28/C 48's chain), moved here in the
// P-90 mirror round; the units reach the surface through `include!` edges at those C positions, and
// the reference-header mirrors that own the C `#include` carry the edge themselves
// (`include/pgtkterm.rs` at its C 31).  `include/gtkutil.rs`'s C 26 edge is owed - `## Owed` in
// `docs/ports/include-gtk.md` names the blocker.
//
// **GLib, GObject and cairo are on this machine** (`/opt/homebrew/include/glib-2.0`, GLib 2.90.0,
// plus the generated `glibconfig.h`; `/opt/homebrew/include/cairo`, cairo 1.18.4), so their
// declarations quote the header *and the line* they were read from and `GList`/`GError`/`GType`'s
// layouts are real clang readings.  **GDK and GTK are not**: `ls -d /opt/homebrew/include/gtk-3.0
// /opt/homebrew/include/gtk-4.0` -> both absent, `pkg-config` is not installed, and no `gtk/gtk.h`
// or `gdk/gdk.h` exists under `/opt/homebrew`, `/usr/local` or `/Library`.  So every GDK/GTK
// citation names the **header alone, no line**, no layout assertion is written for a GDK/GTK
// record, and every enumerator and flag value of theirs is a **placeholder**
// (`// placeholder, not GDK's value`) - the `src/xwidget.rs` 495-520 rule: the toolkit's values are
// not measurable here, so the port keeps the arms and the bit tests distinct rather than inventing
// behaviour, and the real values are owed to a GTK toolchain (`docs/ports/include-gtk.md`).
//
// A mirror carries declarations only - types, constants, `macro_rules!` and `unsafe extern "C"`
// items - so nothing in this file produces a symbol (PORTING.md 0.1-2/0.2).

/* GLib and GObject: `/opt/homebrew/include/glib-2.0` (GLib 2.90.0), `glibconfig.h` from
   `/opt/homebrew/Cellar/glib/2.90.0/lib/glib-2.0/include`.  */
/// C: gtypes.h 52/55/56/58/60/61: `typedef char gchar;` / `typedef int gint;` / `typedef gint
/// gboolean;` / `typedef unsigned char guchar;` / `typedef unsigned long gulong;` / `typedef
/// unsigned int guint;`
pub type gchar = c_char;
pub type gint = c_int;
pub type gboolean = c_int;
pub type guchar = c_uchar;
pub type gulong = c_ulong;
pub type guint = c_uint;
/// C: gtypes.h 109: `typedef void* gpointer;`
pub type gpointer = *mut c_void;
/// C: glibconfig.h 55/81 (the generated header): `typedef unsigned int guint32;` / `typedef
/// unsigned long gsize;`
pub type guint32 = c_uint;
pub type gsize = c_ulong;
/// C: gquark.h 38: `typedef guint32 GQuark;`
pub type GQuark = guint32;
/// C: gtype.h 427: `typedef gsize GType;` (the applicable arm of C 425-429's three-way choice on
/// this machine).  Measured `sizeof_GType=8` (Evidence ②).
pub type GType = gsize;
const _: () = core::assert! (core::mem::size_of::<GType> () == 8);
/// C: gmacros.h 966-972: `#ifndef FALSE / #define FALSE (0) / #endif` and its `TRUE` pair.
pub const FALSE: gboolean = 0;
pub const TRUE: gboolean = 1;
/// C: glist.h 39-46: `typedef struct _GList GList; struct _GList { gpointer data; GList *next;
/// GList *prev; };`.  Measured on this machine: `sizeof_GList=24 align=8 data=0 next=8 prev=16`
/// (Evidence ②).  C 606/615/616/869/871 read `->next` and `->data`.
pub type GList = _GList;
#[repr(C)]
#[derive (Clone, Copy)]
pub struct _GList
{
  pub data: gpointer,        // C: `gpointer data;`
  pub next: *mut _GList,     // C: `GList *next;`
  pub prev: *mut _GList,     // C: `GList *prev;`
}
const _: () = core::assert! (core::mem::size_of::<_GList> () == 24);
const _: () = core::assert! (core::mem::align_of::<_GList> () == 8);
const _: () = core::assert! (core::mem::offset_of! (_GList, data) == 0);
const _: () = core::assert! (core::mem::offset_of! (_GList, next) == 8);
const _: () = core::assert! (core::mem::offset_of! (_GList, prev) == 16);
/// C: gobject.h 192: `typedef struct _GObject GObject;` - the checked-cast chain's target type
/// (`G_OBJECT` below); the port never allocates or dereferences one (the `src/xfns.rs` 1027-1032
/// shape).  A `use_gtk` build reaches it through `gtkutil.h` 26's `<gtk/gtk.h>`.
#[repr(C)]
pub struct GObject
{
  _private: [u8; 0],
}
/// C: gclosure.h 77/92: `typedef struct _GClosure GClosure;` / `typedef void (*GCallback)
/// (void);`.  C's function-pointer convention is `Option<unsafe extern "C" fn …>` (PORTING.md
/// P-27), which is what `G_CALLBACK` below produces.
#[repr(C)]
pub struct GClosure
{
  _private: [u8; 0],
}
pub type GCallback = Option<unsafe extern "C" fn ()>;
/// C: gclosure.h: `typedef void (*GClosureNotify) (gpointer data, GClosure *closure);`
pub type GClosureNotify =
  Option<unsafe extern "C" fn (data: gpointer, closure: *mut GClosure)>;
/// C: gsignal.h 184-206: `typedef enum { G_CONNECT_DEFAULT = 0, G_CONNECT_AFTER = 1 << 0,
/// G_CONNECT_SWAPPED = 1 << 1 } GConnectFlags;`
pub type GConnectFlags = c_int;
/// C: gerror.h 43/47-52: `typedef struct _GError GError;` and its three members.  C 677/681/707
/// declare one and read `err->code`; measured `sizeof_GError=16 domain=0 code=4 message=8`
/// (Evidence ②).
#[repr(C)]
pub struct _GError
{
  pub domain: GQuark,        // C: `GQuark domain;`
  pub code: gint,            // C: `gint code;`
  pub message: *mut gchar,   // C: `gchar *message;`
}
pub type GError = _GError;
const _: () = core::assert! (core::mem::size_of::<_GError> () == 16);
const _: () = core::assert! (core::mem::offset_of! (_GError, code) == 4);
const _: () = core::assert! (core::mem::offset_of! (_GError, message) == 8);
/// C: gconvert.h 53-66: the `GConvertError` enumeration; C 681/707 compare `err->code` against
/// `G_CONVERT_ERROR_ILLEGAL_SEQUENCE` (member 59, value 1 by C's own text - measured 1).
/// C's enum ABI is `c_int` (PORTING.md 1.3), so no record measurement is owed.
pub type GConvertError = c_int;
pub const G_CONVERT_ERROR_ILLEGAL_SEQUENCE: GConvertError = 1;
/// C: gmessages.h 59-71: `G_LOG_FLAG_RECURSION = 1 << 0`, `G_LOG_FLAG_FATAL = 1 << 1`,
/// `G_LOG_LEVEL_WARNING = 1 << 4`, `} G_GNUC_FLAG_ENUM GLogLevelFlags;`.  C 1028/1128-1129 read
/// all three; measured `sizeof_GLogLevelFlags=4` with values 1/2/16 (Evidence ②).
pub type GLogLevelFlags = c_int;
pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1 << 0;
pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 1 << 1;
pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 1 << 4;
/// C: gmessages.h 76: `typedef void (*GLogFunc) (const gchar *log_domain, GLogLevelFlags
/// log_level, const gchar *message, gpointer user_data);` - the type `my_log_handler`'s address is
/// handed to `g_log_set_handler` as.
pub type GLogFunc = Option<unsafe extern "C" fn (log_domain: *const gchar,
                                                log_level: GLogLevelFlags,
                                                message: *const gchar,
                                                user_data: gpointer)>;
/// C: gobject.h 54: `#define G_OBJECT(object) (G_TYPE_CHECK_INSTANCE_CAST ((object), G_TYPE_OBJECT,
/// GObject))`, whose own body is `((GObject *) g_type_check_instance_cast ((GTypeInstance *)
/// (object), (GType) (g_type)))` (gtype.h 528/2668-2672).  The port keeps the macro's name and its
/// result type; GLib's checked-cast chain is recorded as a deviation (the `src/pgtkterm.rs` 288-296
/// treatment of the same macro).
macro_rules! G_OBJECT {
  ($object:expr) => { ($object) as *mut GObject };
}
/// C: gtk/gtk.h: `#define G_CALLBACK(f) ((GCallback) (f))` - GTK's, not GLib's, so the citation is
/// the header and not a line.  C's cast reinterprets the function pointer; Rust refuses a
/// signature-changing `as` cast between fn pointers (`error: non-primitive cast`, measured in
/// `src/pgtkterm.rs`), so the port spells C's cast as the item address plus a pointer transmute,
/// and every callback C passes through it is declared `unsafe extern "C" fn` (PORTING.md 1.4).
macro_rules! G_CALLBACK {
  ($f:expr) => {
    Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn ()> ($f as *const ()) })
  };
}
/// C: gobject/gtype.h: `typedef struct _GTypeClass GTypeClass;` / `typedef struct
/// _GTypeInstance GTypeInstance;` - measured (Evidence ②): `sizeof` 8 each, `g_type` at 0 and
/// `g_class` at 0.  These are what `G_TYPE_FROM_INSTANCE!` below reads.
#[repr (C)]
pub struct GTypeClass
{
  pub g_type: GType,          // C: `GType g_type;`
}
#[repr (C)]
pub struct GTypeInstance
{
  pub g_class: *mut GTypeClass,   // C: `GTypeClass *g_class;`
}
const _: () = core::assert! (core::mem::size_of::<GTypeClass> () == 8);
const _: () = core::assert! (core::mem::size_of::<GTypeInstance> () == 8);
/// C: gtype.h 646/657: `#define G_TYPE_FROM_INSTANCE(instance) (G_TYPE_FROM_CLASS
/// (((GTypeInstance*) (instance))->g_class))` / `#define G_TYPE_FROM_CLASS(g_class)
/// (((GTypeClass*) (g_class))->g_type)` - the real chain, not an approximation.  C 3674 reads a
/// display's type name through the two macros below.
macro_rules! G_TYPE_FROM_CLASS {
  ($g_class:expr) => { unsafe { (*(($g_class) as *mut GTypeClass)).g_type } };
}
macro_rules! G_TYPE_FROM_INSTANCE {
  ($instance:expr) => { G_TYPE_FROM_CLASS! (unsafe { (*(($instance) as *mut GTypeInstance)).g_class }) };
}
/// C: gobject.h 98/108: `#define G_OBJECT_TYPE(object) (G_TYPE_FROM_INSTANCE (object))` /
/// `#define G_OBJECT_TYPE_NAME(object) (g_type_name (G_OBJECT_TYPE (object)))` - the name and the
/// result type are kept; `g_type_name` is this block's.
macro_rules! G_OBJECT_TYPE {
  ($object:expr) => { G_TYPE_FROM_INSTANCE! ($object) };
}
macro_rules! G_OBJECT_TYPE_NAME {
  ($object:expr) => { g_type_name (G_OBJECT_TYPE! ($object)) };
}

unsafe extern "C" {
  /// C 3674: `const gchar *type_name = G_OBJECT_TYPE_NAME (G_OBJECT (gdpy));`
  /// C: gobject/gtype.h 743: `const gchar * g_type_name (GType type);`
  pub fn g_type_name (type_: GType) -> *const c_char;
  /// C 663/679/705: `if (!g_utf8_validate (str, -1, NULL))` / `while (! (cp = g_locale_to_utf8
  /// ((char *)p, -1, &bytes_read, &bytes_written, &err)) …`
  /// C: gunicode.h 1052-1054: `gboolean g_utf8_validate (const gchar *str, gssize max_len, const
  /// gchar **end);`
  pub fn g_utf8_validate (str: *const gchar, max_len: isize,
                          end: *mut *const gchar) -> gboolean;
  /// C: gconvert.h 129-133: `gchar* g_locale_to_utf8 (const gchar *opsysstring, gssize len,
  /// gsize *bytes_read, gsize *bytes_written, GError **error);`
  pub fn g_locale_to_utf8 (opsysstring: *const gchar, len: isize,
                           bytes_read: *mut gsize, bytes_written: *mut gsize,
                           error: *mut *mut GError) -> *mut gchar;
  /// C 666: `return g_strdup (str);`
  /// C: gstrfuncs.h 284: `gchar* (g_strdup) (const gchar *str) G_GNUC_MALLOC;` (the function the
  /// 324's `#define g_strdup(x) g_strdup_inline (x)` wrapper calls).
  pub fn g_strdup (str: *const gchar) -> *mut gchar;
  /// C 2014: `char *key = g_strdup_printf ("%s.%s", res, attr);` (its result is freed by C 2016's
  /// callee, `pgtk_set_defaults_value`'s `GSettings` call path, and C never frees it here)
  /// C: gstrfuncs.h 286: `gchar* g_strdup_printf (const gchar *format, ...) G_GNUC_PRINTF (1, 2)
  /// G_GNUC_MALLOC;`
  pub fn g_strdup_printf (format: *const gchar, ...) -> *mut gchar;
  /// C 694/720: `if (cp) g_free (cp);` / `g_free (cp);`
  /// C: gmem.h 74: `void (g_free) (gpointer mem);`
  pub fn g_free (mem: gpointer);
  /// C 685/691/713/724: `g_error_free (err);`
  /// C: gerror.h 208: `void g_error_free (GError *error);`
  pub fn g_error_free (error: *mut GError);
  /// C 182/554/563/586/860: `g_object_new (emacs_menu_bar_get_type (), NULL)` /
  /// `g_object_unref (G_OBJECT (icon_buf))` / `g_object_ref (G_OBJECT (tooltip))`
  /// C: gobject.h 432-434: `gpointer g_object_new (GType object_type, const gchar
  /// *first_property_name, ...);`
  pub fn g_object_new (object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
  /// C 858/920/985: `g_object_set (G_OBJECT (widget), "has-tooltip", FALSE, NULL)` /
  /// `g_object_set (settings, "gtk-enable-tooltips", FALSE, NULL)`
  /// C: gobject.h 455-457: `void g_object_set (gpointer object, const gchar *first_property_name,
  /// ...);`
  pub fn g_object_set (object: gpointer, first_property_name: *const gchar, ...);
  /// C 917: `g_object_get (settings, "gtk-enable-tooltips", &tt_enabled, NULL);`
  /// C: gobject.h 459-461: `void g_object_get (gpointer object, const gchar
  /// *first_property_name, ...);`
  pub fn g_object_get (object: gpointer, first_property_name: *const gchar, ...);
  /// C 860/862: `g_object_ref (G_OBJECT (tooltip));` / `g_object_ref (G_OBJECT (x->ttip_lbl));`
  /// C: gobject.h 513: `gpointer g_object_ref (gpointer object);`
  pub fn g_object_ref (object: gpointer) -> gpointer;
  /// C: gobject.h 515: `void g_object_unref (gpointer object);`
  pub fn g_object_unref (object: gpointer);
  /// C 922: `g_object_set_data (G_OBJECT (x->ttip_window), "restore-tt", (gpointer)f);`
  /// C: gobject.h 597-599: `void g_object_set_data (GObject *object, const gchar *key, gpointer
  /// data);`
  pub fn g_object_set_data (object: *mut GObject, key: *const gchar, data: gpointer);
  /// C 980: `if (g_object_get_data (G_OBJECT (win), "restore-tt"))`
  /// C: gobject.h 594-595: `gpointer g_object_get_data (GObject *object, const gchar *key);`
  pub fn g_object_get_data (object: *mut GObject, key: *const gchar) -> gpointer;
  /// C 875: `g_list_free (list);`
  /// C: glist.h 53: `void g_list_free (GList *list);`
  pub fn g_list_free (list: *mut GList);
  /// C: glist.h 175: `#define g_list_next(list) ((list) ? (((GList *)(list))->next) : NULL)` -
  /// GLib's own body, which C 615/869 use as the update step of a `for` loop.
  /// C: gsignal.h 554-556: `#define g_signal_connect(instance, detailed_signal, c_handler, data)
  /// g_signal_connect_data ((instance), (detailed_signal), (c_handler), (data), NULL,
  /// (GConnectFlags) 0)` - GLib's own body; C 883-884 is the one call site.
  /// C: gsignal.h 440-446: `gulong g_signal_connect_data (gpointer instance, const gchar
  /// *detailed_signal, GCallback c_handler, gpointer data, GClosureNotify destroy_data,
  /// GConnectFlags connect_flags);`
  pub fn g_signal_connect_data (instance: gpointer, detailed_signal: *const gchar,
                                c_handler: GCallback, data: gpointer,
                                destroy_data: GClosureNotify,
                                connect_flags: GConnectFlags) -> gulong;
  /// C 1128: `id = g_log_set_handler ("Gtk", G_LOG_LEVEL_WARNING | G_LOG_FLAG_FATAL |
  /// G_LOG_FLAG_RECURSION, my_log_handler, NULL);`
  /// C: gmessages.h 84-87: `guint g_log_set_handler (const gchar *log_domain, GLogLevelFlags
  /// log_levels, GLogFunc log_func, gpointer user_data);`
  pub fn g_log_set_handler (log_domain: *const gchar, log_levels: GLogLevelFlags,
                            log_func: GLogFunc, user_data: gpointer) -> guint;
  /// C 1135: `g_log_remove_handler ("Gtk", id);`
  /// C: gmessages.h 95: `void g_log_remove_handler (const gchar *log_domain, guint handler_id);`
  pub fn g_log_remove_handler (log_domain: *const gchar, handler_id: guint);
}
macro_rules! g_list_next {
  ($list:expr) => {
    { if !($list).is_null () { (*($list as *mut GList)).next } else { std::ptr::null_mut () } }
  };
}
macro_rules! g_signal_connect {
  ($instance:expr, $signal:expr, $handler:expr, $data:expr) => {
    g_signal_connect_data (($instance) as gpointer, ($signal), G_CALLBACK! ($handler), ($data),
                           None, 0 as GConnectFlags)
  };
}

/* cairo and cairo-gobject: `/opt/homebrew/include/cairo` (cairo 1.18.4).  C 428-476's
   `xg_get_pixbuf_from_surface` and C 512-570's two cairo arms are the users; `img->cr_data` is
   C's `void *` in the mirror (`include/dispextern.rs` 2314), so the two calls that need the real
   type take C's pointer cast at the use site (P-72 rule 3).  */
/// C: cairo.h 135/164/229: `typedef struct _cairo cairo_t;` / `typedef struct _cairo_surface
/// cairo_surface_t;` / `typedef struct _cairo_pattern cairo_pattern_t;`
pub enum cairo_t {}
pub enum cairo_surface_t {}
pub enum cairo_pattern_t {}
/// C: cairo.h 431-441: `typedef enum _cairo_format { CAIRO_FORMAT_INVALID = -1, CAIRO_FORMAT_ARGB32
/// = 0, … } cairo_format_t;` - measured `CAIRO_FORMAT_ARGB32=0`,
/// `sizeof_cairo_format_t=4` (Evidence ②).  C's enum ABI is `c_int` (PORTING.md 1.3).
pub type cairo_format_t = c_int;
pub const CAIRO_FORMAT_ARGB32: cairo_format_t = 0;
/// C: cairo.h 662-696: `typedef enum _cairo_operator { CAIRO_OPERATOR_CLEAR,
/// CAIRO_OPERATOR_SOURCE, … } cairo_operator_t;` - measured `CAIRO_OPERATOR_SOURCE=1`,
/// `sizeof_cairo_operator_t=4` (Evidence ②).
pub type cairo_operator_t = c_int;
pub const CAIRO_OPERATOR_SOURCE: cairo_operator_t = 1;
/// C: cairo.h 2990-2997: `typedef enum _cairo_pattern_type { CAIRO_PATTERN_TYPE_SOLID,
/// CAIRO_PATTERN_TYPE_SURFACE, … } cairo_pattern_type_t;` - measured
/// `CAIRO_PATTERN_TYPE_SURFACE=1`, `sizeof_cairo_pattern_type_t=4` (Evidence ②).
pub type cairo_pattern_type_t = c_int;
pub const CAIRO_PATTERN_TYPE_SURFACE: cairo_pattern_type_t = 1;
/// C: cairo.h 325-374: `typedef enum _cairo_status { … } cairo_status_t;` - the return type of
/// `cairo_pattern_get_surface` below, which C 546 ignores (a statement call).
pub type cairo_status_t = c_int;
unsafe extern "C" {
  /// C 441-442: `cairo_image_surface_create_for_data (pixels, CAIRO_FORMAT_ARGB32, width, height,
  /// rowstride);`
  /// C: cairo.h 2718-2724: `cairo_surface_t *cairo_image_surface_create_for_data (unsigned char
  /// *data, cairo_format_t format, int width, int height, int stride);`
  pub fn cairo_image_surface_create_for_data (data: *mut c_uchar, format: cairo_format_t,
                                              width: c_int, height: c_int,
                                              stride: c_int) -> *mut cairo_surface_t;
  /// C 432-433: `cairo_image_surface_get_width (surface)` / `…_get_height (surface)`
  /// C: cairo.h 2731-2735: `int cairo_image_surface_get_width (cairo_surface_t *surface);` /
  /// `int cairo_image_surface_get_height (cairo_surface_t *surface);`
  pub fn cairo_image_surface_get_width (surface: *mut cairo_surface_t) -> c_int;
  pub fn cairo_image_surface_get_height (surface: *mut cairo_surface_t) -> c_int;
  /// C 443: `cairo_t *cr = cairo_create (icon_surface);`
  /// C: cairo.h 539-540: `cairo_t *cairo_create (cairo_surface_t *target);`
  pub fn cairo_create (target: *mut cairo_surface_t) -> *mut cairo_t;
  /// C 1361/1372: `cairo_set_source_rgba (cr, r, g, b, a);`
  /// C: cairo.h 708-710: `void cairo_set_source_rgba (cairo_t *cr, double red, double green,
  /// double blue, double alpha);`
  pub fn cairo_set_source_rgba (cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double,
				alpha: c_double);
  /// C 1373: `cairo_mask (cr, mask);`
  /// C: cairo.h 980: `void cairo_mask (cairo_t *cr, cairo_pattern_t *pattern);`
  pub fn cairo_mask (cr: *mut cairo_t, pattern: *mut cairo_pattern_t);
  /// C 1434: `cairo_set_line_width (cr, 1);`
  /// C: cairo.h 811: `void cairo_set_line_width (cairo_t *cr, double width);`
  pub fn cairo_set_line_width (cr: *mut cairo_t, width: c_double);
  /// C 1435: `cairo_stroke (cr);`
  /// C: cairo.h 990: `void cairo_stroke (cairo_t *cr);`
  pub fn cairo_stroke (cr: *mut cairo_t);
  /// C 444: `cairo_surface_destroy (icon_surface);`
  /// C: cairo.h 2489: `void cairo_surface_destroy (cairo_surface_t *surface);`
  pub fn cairo_surface_destroy (surface: *mut cairo_surface_t);
  /// C 445: `cairo_set_source_surface (cr, surface, 0, 0);`
  /// C: cairo.h 712-716: `void cairo_set_source_surface (cairo_t *cr, cairo_surface_t *surface,
  /// double x, double y);`
  pub fn cairo_set_source_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t,
                                   x: c_double, y: c_double);
  /// C 446: `cairo_set_operator (cr, CAIRO_OPERATOR_SOURCE);`
  /// C: cairo.h 699: `void cairo_set_operator (cairo_t *cr, cairo_operator_t op);`
  pub fn cairo_set_operator (cr: *mut cairo_t, op: cairo_operator_t);
  /// C 447: `cairo_paint (cr);`
  /// C: cairo.h 973: `void cairo_paint (cairo_t *cr);`
  pub fn cairo_paint (cr: *mut cairo_t);
  /// C 448: `cairo_destroy (cr);`
  /// C: cairo.h 546: `void cairo_destroy (cairo_t *cr);`
  pub fn cairo_destroy (cr: *mut cairo_t);
  /// C 545: `if (cairo_pattern_get_type (img->cr_data) == CAIRO_PATTERN_TYPE_SURFACE)`
  /// C: cairo.h 2999-3000: `cairo_pattern_type_t cairo_pattern_get_type (cairo_pattern_t
  /// *pattern);`
  pub fn cairo_pattern_get_type (pattern: *mut cairo_pattern_t) -> cairo_pattern_type_t;
  /// C 546: `cairo_pattern_get_surface (img->cr_data, &surface);`
  /// C: cairo.h 3135-3137: `cairo_status_t cairo_pattern_get_surface (cairo_pattern_t *pattern,
  /// cairo_surface_t **surface);`
  pub fn cairo_pattern_get_surface (pattern: *mut cairo_pattern_t,
                                    surface: *mut *mut cairo_surface_t) -> cairo_status_t;
}

/* ----------------------------------------------------------------------------------------------
   GDK.  No GDK tree on this machine, so each citation names the header alone and every one of the
   records below is the never-dereferenced `[u8; 0]` shape (`src/xfns.rs` 1004-1009's `GtkWidget`):
   the port only ever passes their pointers to GDK's and GTK's own entry points.  The three records
   C reads *members* of (`GdkRGBA` at C 787-789 and `GdkColor` at C 802-805, both GTK-version arms
   of `xg_check_special_colors`) carry exactly those members, and their layout is **UNVERIFIED**
   (P-88 ②: no header to measure; owed in the ledger).
   ---------------------------------------------------------------------------------------------- */
/// C: gdk/gdkdisplay.h: `typedef struct _GdkDisplay GdkDisplay;`
#[repr(C)]
pub struct GdkDisplay
{
  _private: [u8; 0],
}
/// C: gdk/gdkmonitor.h: `typedef struct _GdkMonitor GdkMonitor;` - the monitor handle C
/// 2042-2107's two `x-display-mm-*` subrs walk.  Opaque here (C only passes it back to GDK), and
/// **UNVERIFIED**: no GDK header on this machine (the records above).
#[repr(C)]
pub struct GdkMonitor
{
  _private: [u8; 0],
}
/// C: gdk/gdkscreen.h: `typedef struct _GdkScreen GdkScreen;`
#[repr(C)]
pub struct GdkScreen
{
  _private: [u8; 0],
}
/// C: gdk/gdkcursor.h: `typedef struct _GdkCursor GdkCursor;`
#[repr(C)]
pub struct GdkCursor
{
  _private: [u8; 0],
}
/// C: gdk/gdkwindow.h: `typedef struct _GdkWindow GdkWindow;`
#[repr(C)]
pub struct GdkWindow
{
  _private: [u8; 0],
}
/// C: gdk-pixbuf/gdk-pixbuf.h: `typedef struct _GdkPixbuf GdkPixbuf;`
#[repr(C)]
pub struct GdkPixbuf
{
  _private: [u8; 0],
}
/// C: gdk/gdkrgba.h: `typedef struct { double red; double green; double blue; double alpha; }
/// GdkRGBA;`.  C 745 declares one, C 783 assigns `col = *c;` and C 787-789 reads `col.red`,
/// `col.green`, `col.blue`.  **UNVERIFIED**: no GDK header on this machine.
#[repr(C)]
#[derive (Clone, Copy)]
pub struct GdkRGBA
{
  pub red: c_double,        // C: gdk/gdkrgba.h: `double red;`
  pub green: c_double,      // C: gdk/gdkrgba.h: `double green;`
  pub blue: c_double,       // C: gdk/gdkrgba.h: `double blue;`
  pub alpha: c_double,      // C: gdk/gdkrgba.h: `double alpha;`
}
/// C: gdk/gdkcolor.h (GTK2's GDK): `typedef struct { guint32 pixel; guint16 red; guint16 green;
/// guint16 blue; } GdkColor;`.  C 752 declares `GdkColor *grgb;` and C 802-805 reads four members.
/// **UNVERIFIED**: no GDK header on this machine (the record's width is unmeasured too).
#[cfg (not (feature = "have_gtk3"))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct GdkColor
{
  pub pixel: guint32,        // C: gdk/gdkcolor.h: `guint32 pixel;`
  pub red: c_ushort,         // C: gdk/gdkcolor.h: `guint16 red;`
  pub green: c_ushort,       // C: gdk/gdkcolor.h: `guint16 green;`
  pub blue: c_ushort,        // C: gdk/gdkcolor.h: `guint16 blue;`
}
/// C: gdk/gdktypes.h: `typedef enum { GDK_COLORSPACE_RGB } GdkColorspace;` (C 404/434 pass it to
/// `gdk_pixbuf_new`).  C's enum ABI is `c_int` (PORTING.md 1.3); **UNVERIFIED** for the same
/// reason as the records above.
pub type GdkColorspace = c_int;
pub const GDK_COLORSPACE_RGB: GdkColorspace = 0;
/// C: gdk/gdkcursor.h: `typedef enum { … GDK_LEFT_PTR, … } GdkCursorType;` (C 375 passes it to
/// `gdk_cursor_new_for_display`).  Its value is GTK's, **UNVERIFIED** here.
pub type GdkCursorType = c_int;
pub const GDK_LEFT_PTR: GdkCursorType = 0;
/// C 1556-1591's `GDK_ ## NAME` pastes: the fourteen cursor types C's fifteen `INSTALL_CURSOR`
/// calls name besides `GDK_LEFT_PTR` above (`GDK_XTERM` twice).  The enum is GdkCursorType's and
/// **no GDK header exists on this machine** (`docs/ports/include-gtk.md`), so each value is a
/// placeholder in the same sense as `GDK_LEFT_PTR` - the real one is owed to a GTK toolchain.
pub const GDK_XTERM: GdkCursorType = 0;              // placeholder, not GDK's value
pub const GDK_HAND2: GdkCursorType = 0;              // placeholder, not GDK's value
pub const GDK_WATCH: GdkCursorType = 0;              // placeholder, not GDK's value
pub const GDK_SB_H_DOUBLE_ARROW: GdkCursorType = 0;  // placeholder, not GDK's value
pub const GDK_SB_V_DOUBLE_ARROW: GdkCursorType = 0;  // placeholder, not GDK's value
pub const GDK_LEFT_SIDE: GdkCursorType = 0;          // placeholder, not GDK's value
pub const GDK_RIGHT_SIDE: GdkCursorType = 0;         // placeholder, not GDK's value
pub const GDK_TOP_SIDE: GdkCursorType = 0;           // placeholder, not GDK's value
pub const GDK_BOTTOM_SIDE: GdkCursorType = 0;        // placeholder, not GDK's value
pub const GDK_TOP_LEFT_CORNER: GdkCursorType = 0;    // placeholder, not GDK's value
pub const GDK_TOP_RIGHT_CORNER: GdkCursorType = 0;   // placeholder, not GDK's value
pub const GDK_BOTTOM_RIGHT_CORNER: GdkCursorType = 0; // placeholder, not GDK's value
pub const GDK_BOTTOM_LEFT_CORNER: GdkCursorType = 0; // placeholder, not GDK's value
/// C: gtk/gtkwidget.h: `#define GDK_WINDOW(window) ((GdkWindow *) (window))`-family checked cast
/// (C 616 converts `children->data`), and gdk/gdkx.h's `#define GDK_DISPLAY_XDISPLAY(display)` (C
/// 260).  Both keep their names and their result types; the cast chains are GLib's and owed (the
/// `src/xfns.rs` 1019-1025 treatment of `GTK_WIDGET_CLASS`).
macro_rules! GDK_WINDOW {
  ($o:expr) => { ($o) as *mut GdkWindow };
}
#[cfg (not (feature = "have_pgtk"))]
macro_rules! GDK_DISPLAY_XDISPLAY {
  ($d:expr) => { gdk_x11_display_get_xdisplay ($d) };
}
unsafe extern "C" {
  /// C 247/249: `gdpy = gdk_display_open (display_name);` and the `HAVE_PGTK` arm's
  /// `gdk_display_open (strlen (display_name) == 0 ? NULL : display_name)`
  /// C: gdk/gdkdisplay.h: `GdkDisplay *gdk_display_open (const gchar *display_name);`
  pub fn gdk_display_open (display_name: *const gchar) -> *mut GdkDisplay;
  /// C 255/322/346: `gdk_display_manager_set_default_display (gdk_display_manager_get (), gdpy);`
  /// C: gdk/gdkdisplaymanager.h: `GdkDisplayManager *gdk_display_manager_get (void);` and
  /// `void gdk_display_manager_set_default_display (GdkDisplayManager *manager, GdkDisplay
  /// *display);`
  pub fn gdk_display_manager_get () -> *mut GdkDisplayManager;
  pub fn gdk_display_manager_set_default_display (manager: *mut GdkDisplayManager,
                                                  display: *mut GdkDisplay);
  /// C 208/218: `gdk_display_get_default_screen (gdpy)` / `… (FRAME_X_DISPLAY (f))`
  /// C: gdk/gdkdisplay.h: `GdkScreen *gdk_display_get_default_screen (GdkDisplay *display);`
  pub fn gdk_display_get_default_screen (display: *mut GdkDisplay) -> *mut GdkScreen;
  /// C 312/336: `if (gdk_display_get_default () == gdpy)`
  /// C: gdk/gdkdisplay.h: `GdkDisplay *gdk_display_get_default (void);`
  pub fn gdk_display_get_default () -> *mut GdkDisplay;
  /// C 329/353: `gdk_display_close (gdpy);`
  /// C: gdk/gdkdisplay.h: `void gdk_display_close (GdkDisplay *display);`
  pub fn gdk_display_close (display: *mut GdkDisplay);
  /// C 2053/2087: `n_monitors = gdk_display_get_n_monitors (gdpy);`
  /// C: gdk/gdkdisplay.h: `gint gdk_display_get_n_monitors (GdkDisplay *display);`
  pub fn gdk_display_get_n_monitors (display: *mut GdkDisplay) -> gint;
  /// C 2059/2093: `GdkMonitor *monitor = gdk_display_get_monitor (gdpy, i);`
  /// C: gdk/gdkdisplay.h: `GdkMonitor *gdk_display_get_monitor (GdkDisplay *display,
  /// gint monitor_num);`
  pub fn gdk_display_get_monitor (display: *mut GdkDisplay, monitor_num: gint) -> *mut GdkMonitor;
  /// C 2060/2094: `gdk_monitor_get_geometry (monitor, &rec);`
  /// C: gdk/gdkmonitor.h: `void gdk_monitor_get_geometry (GdkMonitor *monitor, GdkRectangle *geometry);`
  pub fn gdk_monitor_get_geometry (monitor: *mut GdkMonitor, geometry: *mut GdkRectangle);
  /// C 2351/2389/2474/2524: `scale = pgtk_get_monitor_scale_factor (gdk_monitor_get_model
  /// (monitor));`
  /// C: gdk/gdkmonitor.h: `const char *gdk_monitor_get_model (GdkMonitor *monitor);`
  pub fn gdk_monitor_get_model (monitor: *mut GdkMonitor) -> *const c_char;
  /// C 2353/2391: `scale = gdk_monitor_get_scale_factor (monitor);`
  /// C: gdk/gdkmonitor.h: `int gdk_monitor_get_scale_factor (GdkMonitor *monitor);`
  pub fn gdk_monitor_get_scale_factor (monitor: *mut GdkMonitor) -> c_int;
  /// C 2450/2521: `gdk_display_get_monitor_at_window (gdpy, gwin)`
  /// C: gdk/gdkdisplay.h: `GdkMonitor *gdk_display_get_monitor_at_window (GdkDisplay *display,
  /// GdkWindow *window);`
  pub fn gdk_display_get_monitor_at_window (display: *mut GdkDisplay,
					    window: *mut GdkWindow) -> *mut GdkMonitor;
  /// C 2465: `if (gdk_monitor_is_primary (monitor))`
  /// C: gdk/gdkmonitor.h: `gboolean gdk_monitor_is_primary (GdkMonitor *monitor);`
  pub fn gdk_monitor_is_primary (monitor: *mut GdkMonitor) -> gboolean;
  /// C 2471: `gdk_monitor_get_workarea (monitor, &work);`
  /// C: gdk/gdkmonitor.h: `void gdk_monitor_get_workarea (GdkMonitor *monitor, GdkRectangle
  /// *workarea);`
  pub fn gdk_monitor_get_workarea (monitor: *mut GdkMonitor, workarea: *mut GdkRectangle);
  /// C 2866/2868/2872: `GdkSeat *seat = gdk_display_get_default_seat (FRAME_DISPLAY_INFO (f)->gdpy);`
  /// / `GdkDevice *dev = gdk_seat_get_pointer (seat);` / `gdk_device_get_position (dev, &scr,
  /// root_x, root_y);`
  /// C: gdk/gdkdisplay.h: `GdkSeat *gdk_display_get_default_seat (GdkDisplay *display);`
  /// C: gdk/gdkseat.h: `GdkDevice *gdk_seat_get_pointer (GdkSeat *seat);`
  /// C: gdk/gdkdevice.h: `gboolean gdk_device_get_position (GdkDevice *device, GdkScreen **screen,
  /// gint *x, gint *y);`
  pub fn gdk_display_get_default_seat (display: *mut GdkDisplay) -> *mut GdkSeat;
  pub fn gdk_seat_get_pointer (seat: *mut GdkSeat) -> *mut GdkDevice;
  pub fn gdk_device_get_position (device: *mut GdkDevice, screen: *mut *mut GdkScreen,
				  x: *mut c_int, y: *mut c_int) -> gboolean;
  /// C 3499: `gdk_device_warp (device, gscr, XFIXNUM (x), XFIXNUM (y));`
  /// C: gdk/gdkdevice.h: `void gdk_device_warp (GdkDevice *device, GdkScreen *screen, gint x,
  /// gint y);`
  pub fn gdk_device_warp (device: *mut GdkDevice, screen: *mut GdkScreen, x: gint, y: gint);
  /// C 2062: `int mm = gdk_monitor_get_height_mm (monitor);`
  /// C: gdk/gdkmonitor.h: `int gdk_monitor_get_height_mm (GdkMonitor *monitor);`
  pub fn gdk_monitor_get_height_mm (monitor: *mut GdkMonitor) -> c_int;
  /// C 2096: `int mm = gdk_monitor_get_width_mm (monitor);`
  /// C: gdk/gdkmonitor.h: `int gdk_monitor_get_width_mm (GdkMonitor *monitor);`
  pub fn gdk_monitor_get_width_mm (monitor: *mut GdkMonitor) -> c_int;
  /// C 375: `return gdk_cursor_new_for_display (gdpy, GDK_LEFT_PTR);`
  /// C: gdk/gdkcursor.h: `GdkCursor *gdk_cursor_new_for_display (GdkDisplay *display,
  /// GdkCursorType cursor_type);`
  pub fn gdk_cursor_new_for_display (display: *mut GdkDisplay,
                                     cursor_type: GdkCursorType) -> *mut GdkCursor;
  /// C 404/434: `icon_buf = gdk_pixbuf_new (GDK_COLORSPACE_RGB, TRUE, 8, width, height);`
  /// C: gdk-pixbuf/gdk-pixbuf.h: `GdkPixbuf *gdk_pixbuf_new (GdkColorspace colorspace, gboolean
  /// has_alpha, int bits_per_sample, int width, int height);`
  pub fn gdk_pixbuf_new (colorspace: GdkColorspace, has_alpha: gboolean,
                         bits_per_sample: c_int, width: c_int, height: c_int) -> *mut GdkPixbuf;
  /// C 407/438: `guchar *pixels = gdk_pixbuf_get_pixels (icon_buf);`
  /// C: gdk-pixbuf/gdk-pixbuf.h: `guchar *gdk_pixbuf_get_pixels (const GdkPixbuf *pixbuf);`
  pub fn gdk_pixbuf_get_pixels (pixbuf: *const GdkPixbuf) -> *mut guchar;
  /// C 408/439: `int rowjunkwidth = gdk_pixbuf_get_rowstride (icon_buf) - width * 4;`
  /// C: gdk-pixbuf/gdk-pixbuf.h: `int gdk_pixbuf_get_rowstride (const GdkPixbuf *pixbuf);`
  pub fn gdk_pixbuf_get_rowstride (pixbuf: *const GdkPixbuf) -> c_int;
  /// C 606: `GList *children = gdk_window_peek_children (window);`
  /// C: gdk/gdkwindow.h: `GList *gdk_window_peek_children (GdkWindow *window);`
  pub fn gdk_window_peek_children (window: *mut GdkWindow) -> *mut GList;
  /// C 608/616: `gdk_window_set_cursor (window, cursor);`
  /// C: gdk/gdkwindow.h: `void gdk_window_set_cursor (GdkWindow *window, GdkCursor *cursor);`
  pub fn gdk_window_set_cursor (window: *mut GdkWindow, cursor: *mut GdkCursor);
  /// C 915/983: `screen = gdk_window_get_screen (gwin);`
  /// C: gdk/gdkwindow.h: `GdkScreen *gdk_window_get_screen (GdkWindow *window);`
  pub fn gdk_window_get_screen (window: *mut GdkWindow) -> *mut GdkScreen;
  /// C 3494/3516: `GdkDisplay *gdpy = gdk_window_get_display (window);`
  /// C: gdk/gdkwindow.h: `GdkDisplay *gdk_window_get_display (GdkWindow *window);`
  pub fn gdk_window_get_display (window: *mut GdkWindow) -> *mut GdkDisplay;
  /// C 784: `gdk_rgba_free (c);`
  /// C: gdk/gdkrgba.h: `void gdk_rgba_free (GdkRGBA *rgba);`
  pub fn gdk_rgba_free (rgba: *mut GdkRGBA);
  /// C 1272: `gdk_flush ();`
  /// C: gdk/gdkwindow.h: `void gdk_flush (void);`
  pub fn gdk_flush ();
  /// C 1231: `gdk_window_resize (gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET (f)), outer_width,
  /// outer_height);`
  /// C: gdk/gdkwindow.h: `void gdk_window_resize (GdkWindow *window, gint width, gint height);`
  pub fn gdk_window_resize (window: *mut GdkWindow, width: gint, height: gint);
}
/// C: gdk/gdkdisplaymanager.h: `typedef struct _GdkDisplayManager GdkDisplayManager;` - the
/// opaque the two `*_default_display` entry points hand back.
#[repr(C)]
pub struct GdkDisplayManager
{
  _private: [u8; 0],
}

/* ----------------------------------------------------------------------------------------------
   GTK.  Same treatment as GDK above: no GTK tree on this machine, so each citation names the
   header alone, the `GtkWidget`-family pointees are the `[u8; 0]` shape, and the records C reads
   members of carry exactly those members with their layout **UNVERIFIED** (P-88 ②; owed).
   `GtkWidgetClass` is the `src/xfns.rs` 1034-1045 precedent: C reads one member of it, the port
   carries that member and owes the record's other members and the offset.
   ---------------------------------------------------------------------------------------------- */
/// C: gtk/gtkwidget.h: `typedef struct _GtkWidget GtkWidget;` (`G_DECLARE_DERIVABLE_TYPE` in GTK3).
/// Also the type `include/xterm.rs`'s `x_output.widget`/`menubar_widget`/`ttip_lbl` members name
/// bare (xterm.rs 378 lists it among that mirror's un-mirrored names), which is why it is declared
/// here rather than only where this file uses it.
#[repr(C)]
pub struct GtkWidget
{
  _private: [u8; 0],
}
/// C: gtk/gtkwindow.h: `typedef struct _GtkWindow GtkWindow;`
#[repr(C)]
pub struct GtkWindow
{
  _private: [u8; 0],
}
/// C: gtk/gtkmenu.h: `typedef struct _GtkMenu GtkMenu;`
#[repr(C)]
pub struct GtkMenu
{
  _private: [u8; 0],
}
/// C: gtk/gtkimage.h: `typedef struct _GtkImage GtkImage;`
#[repr(C)]
pub struct GtkImage
{
  _private: [u8; 0],
}
/// C: gtk/gtkbin.h: `typedef struct _GtkBin GtkBin;`
#[repr(C)]
pub struct GtkBin
{
  _private: [u8; 0],
}
/// C: gtk/gtkcontainer.h: `typedef struct _GtkContainer GtkContainer;`
#[repr(C)]
pub struct GtkContainer
{
  _private: [u8; 0],
}
/// C: gtk/gtklabel.h: `typedef struct _GtkLabel GtkLabel;`
#[repr(C)]
pub struct GtkLabel
{
  _private: [u8; 0],
}
/// C: gtk/gtkmenushell.h: `typedef struct _GtkMenuShell GtkMenuShell;`
#[repr(C)]
pub struct GtkMenuShell
{
  _private: [u8; 0],
}
/// C: gtk/gtkmenubar.h: `typedef struct _GtkMenuBar GtkMenuBar;` (`G_DECLARE_DERIVABLE_TYPE`, so
/// `struct _GtkMenuBar` is GTK's own and its size is GTK's).  C 86 embeds one **by value** in this
/// file's `EmacsMenuBar`; the port's `[u8; 0]` is the placeholder for a record it cannot lay out
/// (SIZE **UNVERIFIED**, owed - `## Deviations`).
#[cfg (feature = "have_gtk3")]
#[repr(C)]
pub struct GtkMenuBar
{
  _private: [u8; 0],
}
/// C: gtk/gtkmenubar.h: `typedef struct _GtkMenuBarClass GtkMenuBarClass;` - C 91 embeds one by
/// value in `EmacsMenuBarClass` (same UNVERIFIED placeholder as above).
#[cfg (feature = "have_gtk3")]
#[repr(C)]
pub struct GtkMenuBarClass
{
  _private: [u8; 0],
}
/// C: gtk/gtkwidget.h: `GtkWidgetClass`.  C 163 writes and C 173 calls exactly one member of it;
/// the port carries that member and owes the record's other members and this one's offset to a GTK
/// toolchain measurement (P-88 ②; the `src/xfns.rs` 1034-1045 precedent).
#[cfg (feature = "have_gtk3")]
#[repr(C)]
pub struct GtkWidgetClass
{
  /// C: gtk/gtkwidget.h: `void (*get_preferred_width) (GtkWidget *widget, gint *minimum, gint
  /// *natural);`
  pub get_preferred_width:
    Option<unsafe extern "C" fn (widget: *mut GtkWidget, minimum: *mut gint, natural: *mut gint)>,
  /// C: gtk/gtkwidget.h: `void (*style_updated) (GtkWidget *widget);` - `src/xfns.rs`'s
  /// in-unit copy carries it (its `gtk_widget_set_app_paintable` neighbour), and it is the
  /// second blocker of `include/gtkutil.rs`'s C 26 edge: the edge cannot land while the mirror
  /// and the unit both declare this class record (status 3.481).  The class record's layout is
  /// GTK's and **UNVERIFIED** here; like the field above, only the members the reference reads
  /// are carried.
  /// C: gtk/gtkwidget.h: `void (*get_preferred_height) (GtkWidget *widget, gint *minimum, gint
  /// *natural);` - C 73 calls it, and `src/emacsgtkfixed.rs` (status 3.493) is its reader; the
  /// record's offsets are GTK's and **UNVERIFIED** here (P-88 ②), which is why only the members
  /// the reference reads are carried and no size assertion is made on this record.
  pub get_preferred_height:
    Option<unsafe extern "C" fn (widget: *mut GtkWidget, minimum: *mut gint, natural: *mut gint)>,
  pub style_updated: Option<unsafe extern "C" fn (widget: *mut GtkWidget)>,
}
/// C: gtk/gtktooltip.h: `typedef struct _GtkTooltip GtkTooltip;`
#[repr(C)]
pub struct GtkTooltip
{
  _private: [u8; 0],
}
/// C: gtk/gtksettings.h: `typedef struct _GtkSettings GtkSettings;`
#[repr(C)]
pub struct GtkSettings
{
  _private: [u8; 0],
}
/// C: gtk/gtkstylecontext.h: `typedef struct _GtkStyleContext GtkStyleContext;` (GTK3's).
#[cfg (feature = "have_gtk3")]
#[repr(C)]
pub struct GtkStyleContext
{
  _private: [u8; 0],
}
/// C: gtk/gtkstyle.h (GTK2's): `struct _GtkStyle { … GdkColor fg[…]; GdkColor bg[…]; … }`.  C 751
/// declares `GtkStyle *gsty;` and C 798-800 reads `gsty->bg[GTK_STATE_SELECTED]` and
/// `gsty->fg[GTK_STATE_SELECTED]` - **the two members and the state enumerator are GTK2's and
/// there is no GTK2 tree on this machine**, so the port carries the record's opaque head and
/// records those two member reads as owed (SIZE and BOUNDS UNVERIFIED, P-88 ②).
#[cfg (not (feature = "have_gtk3"))]
#[repr(C)]
pub struct GtkStyle
{
  _private: [u8; 0],
}
#[cfg (not (feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 798: `gsty = gtk_widget_get_style (FRAME_GTK_WIDGET (f));`
  /// C: gtk/gtkwidget.h (GTK2): `GtkStyle *gtk_widget_get_style (GtkWidget *widget);`
  pub fn gtk_widget_get_style (widget: *mut GtkWidget) -> *mut GtkStyle;
}
/// C: gtk/gtktypes.h: `typedef struct { gint x, y; gint width, height; } GtkAllocation;`.  C 1197
/// declares one, C 1199-1200 reads `alloc.width`/`alloc.height` and C 1235 initializes one with
/// four values; **UNVERIFIED** (the `src/pgtkterm.rs` 431-445 declaration of the same record, whose
/// size/offsets that ledger owes too).
#[repr(C)]
pub struct GtkAllocation
{
  pub x: gint,          // C: gtk/gtktypes.h: `gint x;`
  pub y: gint,          // C: gtk/gtktypes.h: `gint y;`
  pub width: gint,      // C: gtk/gtktypes.h: `gint width;`
  pub height: gint,     // C: gtk/gtktypes.h: `gint height;`
}
/// C: gtk/gtkrequisition.h: `typedef struct { gint width; gint height; } GtkRequisition;`.  C 905
/// declares one and C 935-937 reads both members with `gtk_widget_get_preferred_size`;
/// **UNVERIFIED** for the same reason.
#[repr(C)]
pub struct GtkRequisition
{
  pub width: gint,      // C: gtk/gtkrequisition.h: `gint width;`
  pub height: gint,     // C: gtk/gtkrequisition.h: `gint height;`
}
/// C: gtk/gtktypes.h: `typedef enum { GTK_TEXT_DIR_NONE, GTK_TEXT_DIR_LTR, GTK_TEXT_DIR_RTL }
/// GtkTextDirection;` - the type of C 139's `struct xg_frame_tb_info.dir`.  C's enum ABI is
/// `c_int` (PORTING.md 1.3); the width is **UNVERIFIED** (no GTK header here).
pub type GtkTextDirection = c_int;
/// C: gtk/gtktypes.h / gtk/gtkstylecontext.h: `GtkStateFlags` (GTK3's) and `GtkStateType` (GTK2's,
/// used with `gtk_widget_get_style`/`GtkStyle`), plus the two GTK3 flag values C 770 ORs (the
/// `GtkStyleContext` arm) and the property name C 780 asks for.  All GTK's; **UNVERIFIED**.
#[cfg (feature = "have_gtk3")]
pub type GtkStateFlags = c_int;
#[cfg (feature = "have_gtk3")]
pub const GTK_STATE_FLAG_SELECTED: GtkStateFlags = 1 << 4;
#[cfg (feature = "have_gtk3")]
pub const GTK_STATE_FLAG_FOCUSED: GtkStateFlags = 1 << 3;
#[cfg (feature = "have_gtk3")]
pub const GTK_STYLE_PROPERTY_BACKGROUND_COLOR: *const c_char = c"background-color".as_ptr ();
/// C: gtk/gtkenums.h (GTK2's): `GTK_ORIENTATION_HORIZONTAL` (C 120's `gtk_box_new` body),
/// `GTK_STATE_SELECTED` (C 799-800) and gtk/gtkstock.h's three stock-id macros C 75-77 aliases.
/// **None of the six can be transcribed**: no GTK2 tree exists here (`## Owed`).  The GTK3-version
/// `GtkOrientation`/`GtkStateType` are the same names in GTK3, and this file only reaches them
/// through the GTK2 arm.
#[cfg (not (feature = "have_gtk3"))]
pub type GtkOrientation = c_int;
#[cfg (not (feature = "have_gtk3"))]
pub type GtkStateType = c_int;
/// C: gtk/gtk.h: the eight checked-cast wrappers C 211-223/533-594/866/871/873/914/928/978/1041
/// uses, each keeping the macro's name and its result type (the GLib cast chains are owed, as
/// above).
macro_rules! GTK_WIDGET {
  ($o:expr) => { ($o) as *mut GtkWidget };
}
macro_rules! GTK_WINDOW {
  ($o:expr) => { ($o) as *mut GtkWindow };
}
macro_rules! GTK_MENU {
  ($o:expr) => { ($o) as *mut GtkMenu };
}
macro_rules! GTK_IMAGE {
  ($o:expr) => { ($o) as *mut GtkImage };
}
macro_rules! GTK_BIN {
  ($o:expr) => { ($o) as *mut GtkBin };
}
macro_rules! GTK_CONTAINER {
  ($o:expr) => { ($o) as *mut GtkContainer };
}
macro_rules! GTK_LABEL {
  ($o:expr) => { ($o) as *mut GtkLabel };
}
macro_rules! GTK_MENU_SHELL {
  ($o:expr) => { ($o) as *mut GtkMenuShell };
}
#[cfg (feature = "have_gtk3")]
macro_rules! GTK_WIDGET_CLASS {
  ($o:expr) => { ($o) as *mut GtkWidgetClass };
}
/// C: gtk/gtk.h: the four type-check macros C 210/220/833/1039-1046 asks, each keeping the macro's
/// name and its result type (`gboolean`).
macro_rules! GTK_IS_MENU {
  ($o:expr) => { (! ($o).is_null ()) };
}
macro_rules! GTK_IS_WINDOW {
  ($o:expr) => { (! ($o).is_null ()) };
}
macro_rules! GTK_IS_LABEL {
  ($o:expr) => { (! ($o).is_null ()) };
}
#[cfg (all (feature = "have_gtk3", feature = "have_xinput2"))]
macro_rules! GTK_IS_MENU_BAR {
  ($o:expr) => { (! ($o).is_null ()) };
}
unsafe extern "C" {
  /// C 211/221: `gtk_menu_set_screen (GTK_MENU (w), gscreen);`
  /// C: gtk/gtkmenu.h: `void gtk_menu_set_screen (GtkMenu *menu, GdkScreen *screen);`
  pub fn gtk_menu_set_screen (menu: *mut GtkMenu, screen: *mut GdkScreen);
  /// C 213/223: `gtk_window_set_screen (GTK_WINDOW (w), gscreen);`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_screen (GtkWindow *window, GdkScreen *screen);`
  pub fn gtk_window_set_screen (window: *mut GtkWindow, screen: *mut GdkScreen);
  /// C 292: `return gtk_widget_get_scale_factor (FRAME_GTK_WIDGET (f));`
  /// C: gtk/gtkwidget.h: `gint gtk_widget_get_scale_factor (GtkWidget *widget);`
  pub fn gtk_widget_get_scale_factor (widget: *mut GtkWidget) -> gint;
  /// C 533/535: `old_widget = GTK_IMAGE (gtk_image_new_from_file (encoded_file));` /
  /// `gtk_image_set_from_file (old_widget, encoded_file);`
  /// C: gtk/gtkimage.h: `GtkWidget *gtk_image_new_from_file (const gchar *filename);` / `void
  /// gtk_image_set_from_file (GtkImage *image, const gchar *filename);`
  pub fn gtk_image_new_from_file (filename: *const gchar) -> *mut GtkWidget;
  pub fn gtk_image_set_from_file (image: *mut GtkImage, filename: *const gchar);
  /// C 554/556: `gtk_image_new_from_surface (surface)` / `gtk_image_set_from_surface (old_widget,
  /// surface);`
  /// C: gtk/gtkimage.h: `GtkWidget *gtk_image_new_from_surface (cairo_surface_t *surface);` / `void
  /// gtk_image_set_from_surface (GtkImage *image, cairo_surface_t *surface);`
  pub fn gtk_image_new_from_surface (surface: *mut cairo_surface_t) -> *mut GtkWidget;
  pub fn gtk_image_set_from_surface (image: *mut GtkImage, surface: *mut cairo_surface_t);
  /// C 563/565/586/588: `gtk_image_new_from_pixbuf (icon_buf)` /
  /// `gtk_image_set_from_pixbuf (old_widget, icon_buf);`
  /// C: gtk/gtkimage.h: `GtkWidget *gtk_image_new_from_pixbuf (GdkPixbuf *pixbuf);` / `void
  /// gtk_image_set_from_pixbuf (GtkImage *image, GdkPixbuf *pixbuf);`
  pub fn gtk_image_new_from_pixbuf (pixbuf: *mut GdkPixbuf) -> *mut GtkWidget;
  pub fn gtk_image_set_from_pixbuf (image: *mut GtkImage, pixbuf: *mut GdkPixbuf);
  /// C 866/1041: `gtk_bin_get_child (GTK_BIN (gwdesc))` (the `XG_BIN_CHILD` macro below is its
  /// other spelling)
  /// C: gtk/gtkbin.h: `GtkWidget *gtk_bin_get_child (GtkBin *bin);`
  pub fn gtk_bin_get_child (bin: *mut GtkBin) -> *mut GtkWidget;
  /// C 878: `gtk_window_set_title (x->ttip_window, "");`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_title (GtkWindow *window, const gchar *title);`
  pub fn gtk_window_set_title (window: *mut GtkWindow, title: *const gchar);
  /// C 3728: `gtk_window_set_interactive_debugging (enable_debug);`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_interactive_debugging (gboolean enable);`
  pub fn gtk_window_set_interactive_debugging (enable: gboolean);
  /// C 1088/1096/1104/1110: `gtk_window_set_keep_above (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)),
  /// FALSE);`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_keep_above (GtkWindow *window, gboolean setting);`
  pub fn gtk_window_set_keep_above (window: *mut GtkWindow, setting: gboolean);
  /// C 1090/1098/1112: `gtk_window_set_keep_below (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)),
  /// FALSE);`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_keep_below (GtkWindow *window, gboolean setting);`
  pub fn gtk_window_set_keep_below (window: *mut GtkWindow, setting: gboolean);
  /// C 3756: `guint32 timestamp = gtk_get_current_event_time ();`
  /// C: gtk/gtkmain.h: `guint32 gtk_get_current_event_time (void);`
  pub fn gtk_get_current_event_time () -> guint32;
  /// C 3763: `gtk_show_uri_on_window (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (XFRAME (frame))),
  /// SSDATA (uri), timestamp, &err);`
  /// C: gtk/gtkshow.h: `gboolean gtk_show_uri_on_window (GtkWindow *parent, const char *uri,
  /// guint32 timestamp, GError **error);`
  pub fn gtk_show_uri_on_window (parent: *mut GtkWindow, uri: *const c_char, timestamp: guint32,
				 error: *mut *mut GError) -> gboolean;
  /// C 880-881: `gtk_widget_realize (GTK_WIDGET (x->ttip_window));` / `gtk_widget_realize
  /// (x->ttip_lbl);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_realize (GtkWidget *widget);`
  pub fn gtk_widget_realize (widget: *mut GtkWidget);
  /// C 831/864: `GtkWidget *top = gtk_widget_get_toplevel (x->ttip_lbl);`
  /// C: gtk/gtkwidget.h: `GtkWidget *gtk_widget_get_toplevel (GtkWidget *widget);`
  pub fn gtk_widget_get_toplevel (widget: *mut GtkWidget) -> *mut GtkWidget;
  /// C 605/914/982/1231: `GdkWindow *window = gtk_widget_get_window (w);` /
  /// `gwin = gtk_widget_get_window (GTK_WIDGET (x->ttip_window));`
  /// C: gtk/gtkwidget.h: `GdkWindow *gtk_widget_get_window (GtkWidget *widget);`
  pub fn gtk_widget_get_window (widget: *mut GtkWidget) -> *mut GdkWindow;
  /// C 957/959: `gtk_widget_show (GTK_WIDGET (x->ttip_window));`
  /// C: gtk/gtkwidget.h: `void gtk_widget_show (GtkWidget *widget);`
  pub fn gtk_widget_show (widget: *mut GtkWidget);
  /// C 834/978: `gtk_widget_hide (previous_toplevel);` / `gtk_widget_hide (GTK_WIDGET (win));`
  /// C: gtk/gtkwidget.h: `void gtk_widget_hide (GtkWidget *widget);`
  pub fn gtk_widget_hide (widget: *mut GtkWidget);
  /// C 955/960/1099: `gtk_window_move (x->ttip_window, root_x / xg_get_scale (f), …)` /
  /// `gtk_window_move (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)), f->left_pos / scale, …);`
  /// C: gtk/gtkwindow.h: `void gtk_window_move (GtkWindow *window, gint x, gint y);`
  pub fn gtk_window_move (window: *mut GtkWindow, x: gint, y: gint);
  /// C 1187/1192: `gtk_window_get_size (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)), &gwidth,
  /// &gheight);`
  /// C: gtk/gtkwindow.h: `void gtk_window_get_size (GtkWindow *window, gint *width, gint
  /// *height);`
  pub fn gtk_window_get_size (window: *mut GtkWindow, width: *mut gint, height: *mut gint);
  /// C 3342: `gtk_window_get_position (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)), &left_pos,
  /// &top_pos);`
  /// C: gtk/gtkwindow.h: `void gtk_window_get_position (GtkWindow *window, gint *root_x, gint
  /// *root_y);`
  pub fn gtk_window_get_position (window: *mut GtkWindow, root_x: *mut gint, root_y: *mut gint);
  /// C 1241/1245: `gtk_window_resize (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)), outer_width,
  /// outer_height);`
  /// C: gtk/gtkwindow.h: `void gtk_window_resize (GtkWindow *window, gint width, gint height);`
  pub fn gtk_window_resize (window: *mut GtkWindow, width: gint, height: gint);
  /// C 1248: `gtk_widget_set_size_request (FRAME_GTK_WIDGET (f), outer_width, outer_height);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_set_size_request (GtkWidget *widget, gint width, gint
  /// height);`
  pub fn gtk_widget_set_size_request (widget: *mut GtkWidget, width: gint, height: gint);
  /// C 1198: `gtk_widget_get_allocation (FRAME_GTK_WIDGET (f), &alloc);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_get_allocation (GtkWidget *widget, GtkAllocation
  /// *allocation);`
  pub fn gtk_widget_get_allocation (widget: *mut GtkWidget, allocation: *mut GtkAllocation);
  /// C 1236: `gtk_widget_size_allocate (FRAME_GTK_OUTER_WIDGET (f), &alloc);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_size_allocate (GtkWidget *widget, GtkAllocation
  /// *allocation);`
  pub fn gtk_widget_size_allocate (widget: *mut GtkWidget, allocation: *mut GtkAllocation);
  /// C 1001-1005/1009-1010: `gtk_widget_set_tooltip_text (FRAME_GTK_OUTER_WIDGET (f) ?
  /// FRAME_GTK_OUTER_WIDGET (f) : FRAME_GTK_WIDGET (f), SSDATA (encoded_string));`
  /// C: gtk/gtkwidget.h: `void gtk_widget_set_tooltip_text (GtkWidget *widget, const gchar
  /// *text);`
  pub fn gtk_widget_set_tooltip_text (widget: *mut GtkWidget, text: *const gchar);
  /// C 1131: `if (!gtk_window_parse_geometry (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)), geom_str))`
  /// C: gtk/gtkwindow.h: `gboolean gtk_window_parse_geometry (GtkWindow *window, const gchar
  /// *geometry);`
  pub fn gtk_window_parse_geometry (window: *mut GtkWindow,
                                    geometry: *const gchar) -> gboolean;
  /// C 1045: `gtk_menu_shell_get_parent_shell (GTK_MENU_SHELL (fw))`
  /// C: gtk/gtkmenushell.h: `GtkWidget *gtk_menu_shell_get_parent_shell (GtkMenuShell *menu_shell);`
  pub fn gtk_menu_shell_get_parent_shell (menu_shell: *mut GtkMenuShell) -> *mut GtkWidget;
  /// C 1271: `(void)gtk_events_pending ();`
  /// C: gtk/gtkmain.h: `gboolean gtk_events_pending (void);`
  pub fn gtk_events_pending () -> gboolean;
}
// C 49 / C 44's chain: `#include <gdk/gdkkeysyms.h>` and `"gtkutil.h"` 26's `<gtk/gtk.h>` - the
// same third-party surface as the section above, for the names C 1297-2281 needs.

/* ----------------------------------------------------------------------------------------------
   C 1297-2281's additions to the toolkit surface: the GDK/GTK/GLib names this slice's text needs
   that C 44/49's chain above did not carry.  Same rule as that section (P-88 ①; the P-90 ③ move
   into `femacs/src/include/gtk.rs` is owed and covers these too): the reference tree has no file
   for any of them, so they stand here.  GLib *is* on this machine and its lines are quoted;
   **GDK and GTK are not**, so their citations name the header alone and **every size, offset and
   constant value of theirs is UNVERIFIED** - no layout assertion is written for a GDK or GTK
   record and each enumerator/flag the C text names carries a *placeholder* value (the
   `src/xwidget.rs` 495-520 rule: the toolkit's values are not measurable here, so the port keeps
   the arms and the bit tests distinct rather than inventing behaviour, and `## Owed` 6 names the
   real values as owed).  */

/* GLib/GObject, continued (gsignal.h, gobject.h, gmem.h).  */
/// C: gsignal.h 218-226: `typedef enum { G_SIGNAL_MATCH_ID = 1 << 0, G_SIGNAL_MATCH_DETAIL =
/// 1 << 1, G_SIGNAL_MATCH_CLOSURE = 1 << 2, G_SIGNAL_MATCH_FUNC = 1 << 3, … } GSignalMatchType;`
/// - C 1813/1916 pass `G_SIGNAL_MATCH_FUNC` as `g_signal_handler_find`'s `mask`.  Measured on
/// this machine (GLib 2.90.0): the enumerator is `1 << 3`.
pub type GSignalMatchType = c_int;
pub const G_SIGNAL_MATCH_FUNC: GSignalMatchType = 1 << 3;
/// C: gsignal.h 458-465: `gulong g_signal_handler_find (gpointer instance, GSignalMatchType
/// mask, guint signal_id, GQuark detail, GClosure *closure, gpointer func, gpointer data);`
unsafe extern "C" {
  /// C 1812/1915: `if (! g_signal_handler_find (G_OBJECT (gs), G_SIGNAL_MATCH_FUNC, 0, 0, 0,
  /// (gpointer) G_CALLBACK (style_changed_cb), 0))`
  pub fn g_signal_handler_find (instance: gpointer, mask: GSignalMatchType, signal_id: guint,
				detail: GQuark, closure: *mut GClosure, func: gpointer,
				data: gpointer) -> gulong;
}
/// C: gobject.h 695: `#define g_clear_object(object_ptr) g_clear_pointer ((object_ptr),
/// g_object_unref)` - whose own body is gmem.h 135-145 (the `__GNUC__` arm this machine takes):
/// take the pointer, null the slot, then `g_object_unref` the old value when it was non-NULL.
/// C 1450 is the one call site; the port carries the semantics and records GLib's macro chain
/// here rather than inventing a second `g_clear_pointer` name C never writes.
macro_rules! g_clear_object {
  ($object_ptr:expr) => {
    {
      let pp = ($object_ptr);
      let ptr: *mut c_void = *pp as *mut c_void;
      *pp = std::ptr::null_mut ();
      if ! ptr.is_null () { g_object_unref (ptr); }
    }
  };
}

/// C: gobject.h 694: `void g_clear_object (GObject **object_ptr);` - GLib declares the *function*
/// one line above the macro of the same name, and `src/pgtkterm.c` calls it with function syntax,
/// so the mirror carries both spellings (the `src/image.rs` 1849-1872 same-name pair: a
/// `macro_rules!` and a same-named `pub fn` coexist, and the call site's spelling picks).
unsafe extern "C" {
  pub fn g_clear_object (object_ptr: *mut *mut GObject);
}

/* GDK, continued.  G_DECLARE-style opaque handles, then the event records, the hint/gravity and
   event-mask enumerations and the entry points.  */
/// C: gdk/gdktypes.h: `typedef enum { GDK_NOTHING, GDK_DELETE, GDK_DESTROY, … } GdkEventType;` - C
/// 1414 writes `GDK_NOTHING`.  C's enum ABI is `c_int` (PORTING.md 1.3); **UNVERIFIED**.
pub type GdkEventType = c_int;
pub const GDK_NOTHING: GdkEventType = 0;              // placeholder, not GDK's value
/// C: gdk/gdktypes.h: `typedef guint GdkWindowState;` and the twelve flags C 5632-5687 tests.  No GDK
/// header exists on this machine, so each value is a **placeholder** with a distinct bit (the
/// `GDK_*_MASK` reading in `src/xwidget.rs` 306-316) and **UNVERIFIED**.
pub type GdkWindowState = c_uint;
pub const GDK_WINDOW_STATE_WITHDRAWN: GdkWindowState = 1 << 0;        // placeholder
pub const GDK_WINDOW_STATE_ICONIFIED: GdkWindowState = 1 << 1;        // placeholder
pub const GDK_WINDOW_STATE_MAXIMIZED: GdkWindowState = 1 << 2;        // placeholder
pub const GDK_WINDOW_STATE_STICKY: GdkWindowState = 1 << 3;           // placeholder
pub const GDK_WINDOW_STATE_FULLSCREEN: GdkWindowState = 1 << 4;       // placeholder
pub const GDK_WINDOW_STATE_ABOVE: GdkWindowState = 1 << 5;            // placeholder
pub const GDK_WINDOW_STATE_BELOW: GdkWindowState = 1 << 6;            // placeholder
pub const GDK_WINDOW_STATE_FOCUSED: GdkWindowState = 1 << 7;          // placeholder
pub const GDK_WINDOW_STATE_TILED: GdkWindowState = 1 << 8;            // placeholder
pub const GDK_WINDOW_STATE_TOP_TILED: GdkWindowState = 1 << 9;        // placeholder
pub const GDK_WINDOW_STATE_TOP_RESIZABLE: GdkWindowState = 1 << 10;   // placeholder
pub const GDK_WINDOW_STATE_RIGHT_TILED: GdkWindowState = 1 << 11;     // placeholder
pub const GDK_WINDOW_STATE_RIGHT_RESIZABLE: GdkWindowState = 1 << 12; // placeholder
pub const GDK_WINDOW_STATE_BOTTOM_TILED: GdkWindowState = 1 << 13;    // placeholder
pub const GDK_WINDOW_STATE_BOTTOM_RESIZABLE: GdkWindowState = 1 << 14;   // placeholder
pub const GDK_WINDOW_STATE_LEFT_TILED: GdkWindowState = 1 << 15;      // placeholder
pub const GDK_WINDOW_STATE_LEFT_RESIZABLE: GdkWindowState = 1 << 16;  // placeholder
/* The two `GDK_TOUCHPAD_*` constants C 5000-5002 compares are **not** here: `src/xwidget.rs`
   2249/2256 declares the same names for its own reads, and a mirror may not define a name an
   includer already carries (P-90 7) - each unit keeps its own copy.  */
/// C: gdk/gdktypes.h: `typedef struct _GdkEventAny GdkEventAny;` with `GdkEventType type;
/// GdkWindow *window; gint8 send_event;`, and `typedef union _GdkEvent GdkEvent;` with `any`
/// among its members.  C 1412-1414 sets `event.any.window` and `event.any.type`, and C 1566 takes
/// a `GdkEvent *`.  **UNVERIFIED**: no GDK header on this machine.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventAny
{
  pub r#type: GdkEventType,        // C: gdk/gdktypes.h: `GdkEventType type;`
  pub window: *mut GdkWindow,      // C: gdk/gdktypes.h: `GdkWindow *window;`
  pub send_event: i8,         // C: gdk/gdktypes.h: `gint8 send_event;`
}
#[repr (C)]
#[derive (Clone, Copy)]
pub union GdkEvent
{
  pub r#type: GdkEventType,        // C: gdk/gdktypes.h: `GdkEventType type;`
  pub any: GdkEventAny,            // C: gdk/gdktypes.h: `GdkEventAny any;`
  pub button: GdkEventButton,      // C: gdk/gdktypes.h: `GdkEventButton button;`
  pub scroll: GdkEventScroll,      // C: gdk/gdktypes.h: `GdkEventScroll scroll;`
  pub motion: GdkEventMotion,      // C: gdk/gdktypes.h: `GdkEventMotion motion;`
  pub crossing: GdkEventCrossing,  // C: gdk/gdktypes.h: `GdkEventCrossing crossing;`
  pub key: GdkEventKey,            // C: gdk/gdktypes.h: `GdkEventKey key;`
  pub configure: GdkEventConfigure,   // C: gdk/gdktypes.h: `GdkEventConfigure configure;`
  pub window_state: GdkEventWindowState,   // C: gdk/gdktypes.h: `GdkEventWindowState window_state;`
  pub touchpad_pinch: GdkEventTouchpadPinch,   // C: gdk/gdktypes.h: `GdkEventTouchpadPinch touchpad_pinch;`
}
/* The five records the union names.  **One surface, one mirror** (P-90 ①/②): they used to be
   `src/xwidget.rs`'s in-unit segment (its C 26-30 chain carries `"gtkutil.h"` 26's
   `<gtk/gtk.h>`), and they stand **ungated** here for the same reason the union does - the file
   is `include!`d under its includer's own gate (`src/xwidget.rs` uses `use_gtk`,
   `src/pgtkterm.rs` `have_pgtk`), and the pgtkterm.h 666 reader takes a `GdkEventKey *`.  The
   port carries the members the reference's text touches and nothing else, so **no layout
   assertion is written** for them: GDK is not on this machine, the sizes are **UNVERIFIED**,
   and `GdkEventKey`'s record replaces the zero-sized placeholder this mirror carried while a
   pointer was all anyone took (status 3.481).  */
/// C: gdk/gdktypes.h: `typedef struct _GdkEventConfigure GdkEventConfigure;` - the union's
/// `configure` member above, read by `src/pgtkterm.c` 5543-5564 (`window`, `x` and `y`).  GDK's
/// trailing `width`/`height` are not carried because the reference's text does not read them (the
/// `GdkEventButton` reading below).  **UNVERIFIED**.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventConfigure
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub x: c_int,
  pub y: c_int,
}
/// C: gdk/gdktypes.h: `typedef struct _GdkEventWindowState GdkEventWindowState;` - the union's
/// `window_state` member above, read by `src/pgtkterm.c` 5622-5626 (`window`, `new_window_state`).
/// `changed_mask` is carried because `new_window_state` follows it in GDK's declaration.  No layout
/// assertion is written; **UNVERIFIED**.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventWindowState
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub changed_mask: GdkWindowState,
  pub new_window_state: GdkWindowState,
}
/// C: gdk/gdktypes.h: `typedef struct _GdkEventButton GdkEventButton;` - C 148-155 of
/// `src/xwidget.c` reads and writes `x`/`y`; **UNVERIFIED**.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventButton
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub time: u32,
  pub x: c_double,                 // C: gdk/gdktypes.h: `gdouble x;`
  pub y: c_double,                 // C: gdk/gdktypes.h: `gdouble y;`
}
/// C: gdk/gdktypes.h: `typedef struct _GdkEventScroll GdkEventScroll;` - C 161-168 reads and
/// writes `x`/`y`; **UNVERIFIED**.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventScroll
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub time: u32,
  pub x: c_double,
  pub y: c_double,
}
/// C: gdk/gdktypes.h: `typedef struct _GdkEventMotion GdkEventMotion;` - C 174-181 reads and
/// writes `x`/`y`; **UNVERIFIED**.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventMotion
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub time: u32,
  pub x: c_double,
  pub y: c_double,
}
/// C: gdk/gdktypes.h: `typedef struct _GdkEventCrossing GdkEventCrossing;` - C 188-195 reads and
/// writes `x`/`y`; **UNVERIFIED**.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventCrossing
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub subwindow: *mut GdkWindow,
  pub time: u32,
  pub x: c_double,
  pub y: c_double,
}

/// C: gdk/gdktypes.h: `typedef struct _GdkEventKey GdkEventKey;` - the union's `key` member:
/// C 643-648 of `src/xwidget.c` writes `keyval`/`state` through it, and
/// `pgtk_im_filter_keypress` (pgtkterm.h 666) takes a pointer to it.  It replaces the
/// zero-sized placeholder this mirror carried while a pointer was all anyone took - the pointer
/// readers are unaffected, and the record carries the two members the reference reads (its
/// layout is GDK's and **UNVERIFIED**).  **UNVERIFIED**.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventKey
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub time: u32,
  pub state: c_uint,               // C: gdk/gdktypes.h: `guint state;`
  pub keyval: c_uint,              // C: gdk/gdktypes.h: `guint keyval;`
  // The five members the forty-third slice's `key_press_event` reads on past those two:
  // `length` (C 5345), `string` (C 5348) and `is_modifier` (C 5460), with `hardware_keycode`
  // and `group` carried because `is_modifier` follows them in GDK's declaration.  `is_modifier`
  // is GDK's `guint is_modifier : 1`; the port keeps the storage unit (P-80) and the unit reads
  // it with the one-bit mask.  **UNVERIFIED**.
  pub length: c_int,               // C: gdk/gdktypes.h: `gint length;`
  pub string: *mut c_char,         // C: gdk/gdktypes.h: `gchar *string;`
  pub hardware_keycode: u16,       // C: gdk/gdktypes.h: `guint16 hardware_keycode;`
  pub group: u8,                   // C: gdk/gdktypes.h: `guint8 group;`
  pub is_modifier: c_uint,         // C: gdk/gdktypes.h: `guint is_modifier : 1;`
}
/// C: gdk/gdktypes.h: `typedef struct _GdkEventTouchpadPinch GdkEventTouchpadPinch;` - the record
/// the union's `touchpad_pinch` member above carries, read by `src/pgtkterm.c` 5000-5023.  The port
/// keeps the members that text touches (`n_fingers` included, because `phase` follows it) in GDK's
/// declared order and writes **no layout assertion**: no GDK header exists on this machine, so the
/// offsets are **UNVERIFIED** (the `GdkEventKey` reading above).
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventTouchpadPinch
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub time: u32,
  pub angle_delta: c_double,
  pub scale: c_double,
  pub x: c_double,
  pub y: c_double,
  pub dx: c_double,
  pub dy: c_double,
  pub state: c_uint,
  pub n_fingers: c_uint,
  pub phase: c_int,                // C: `GdkTouchpadGesturePhase phase;` (an enum, so `c_int`)
}
/// C: gdk/gdktypes.h: `typedef struct _GdkVisual GdkVisual;` (C 1735/1741/1745 hold one).
#[repr (C)]
pub struct GdkVisual
{
  _private: [u8; 0],
}
/// C: gdk/gdktypes.h: `typedef struct _GdkColormap GdkColormap;` (GTK2's GDK; C 1454 holds one).
/// **UNVERIFIED** (and GTK2's GDK is a second, separate surface).
#[cfg (not (feature = "have_gtk3"))]
#[repr (C)]
pub struct GdkColormap
{
  _private: [u8; 0],
}
/// C: gdk/gdktypes.h: `typedef enum { GDK_EXPOSURE_MASK = 1 << 1, GDK_POINTER_MOTION_MASK =
/// 1 << 2, … GDK_BUTTON_PRESS_MASK = 1 << 8, GDK_BUTTON_RELEASE_MASK = 1 << 9, GDK_KEY_PRESS_MASK
/// = 1 << 10, GDK_ENTER_NOTIFY_MASK = 1 << 12, GDK_LEAVE_NOTIFY_MASK = 1 << 13,
/// GDK_FOCUS_CHANGE_MASK = 1 << 14, GDK_STRUCTURE_MASK = 1 << 15, GDK_VISIBILITY_NOTIFY_MASK =
/// 1 << 17, GDK_SCROLL_MASK = 1 << 21, GDK_TOUCH_MASK = 1 << 22, GDK_SMOOTH_SCROLL_MASK =
/// 1 << 23, GDK_ALL_EVENTS_MASK = 0x3FFFFF } GdkEventMask;` - C 1607/1713-1730/1845 pass these to
/// `gtk_widget_add_events`.  **UNVERIFIED**: each name carries a placeholder bit (`## Owed` 6).
pub type GdkEventMask = c_int;
pub const GDK_EXPOSURE_MASK: GdkEventMask = 1 << 1;             // placeholder, not GDK's value
pub const GDK_POINTER_MOTION_MASK: GdkEventMask = 1 << 2;       // placeholder, not GDK's value
pub const GDK_BUTTON_PRESS_MASK: GdkEventMask = 1 << 8;         // placeholder, not GDK's value
pub const GDK_BUTTON_RELEASE_MASK: GdkEventMask = 1 << 9;       // placeholder, not GDK's value
pub const GDK_KEY_PRESS_MASK: GdkEventMask = 1 << 10;           // placeholder, not GDK's value
pub const GDK_ENTER_NOTIFY_MASK: GdkEventMask = 1 << 12;        // placeholder, not GDK's value
pub const GDK_LEAVE_NOTIFY_MASK: GdkEventMask = 1 << 13;        // placeholder, not GDK's value
pub const GDK_FOCUS_CHANGE_MASK: GdkEventMask = 1 << 14;        // placeholder, not GDK's value
pub const GDK_STRUCTURE_MASK: GdkEventMask = 1 << 15;           // placeholder, not GDK's value
pub const GDK_VISIBILITY_NOTIFY_MASK: GdkEventMask = 1 << 17;   // placeholder, not GDK's value
pub const GDK_SCROLL_MASK: GdkEventMask = 1 << 21;              // placeholder, not GDK's value
pub const GDK_TOUCH_MASK: GdkEventMask = 1 << 22;               // placeholder, not GDK's value
pub const GDK_SMOOTH_SCROLL_MASK: GdkEventMask = 1 << 23;       // placeholder, not GDK's value
pub const GDK_ALL_EVENTS_MASK: GdkEventMask = 0x3FFFFF;         // placeholder, not GDK's value
/// C: gdk/gdkwindow.h: `typedef enum { GDK_HINT_POS = 1 << 0, GDK_HINT_MIN_SIZE = 1 << 1,
/// GDK_HINT_MAX_SIZE = 1 << 2, GDK_HINT_BASE_SIZE = 1 << 3, GDK_HINT_ASPECT = 1 << 4,
/// GDK_HINT_RESIZE_INC = 1 << 5, GDK_HINT_WIN_GRAVITY = 1 << 6, GDK_HINT_USER_POS = 1 << 7,
/// GDK_HINT_USER_SIZE = 1 << 8 } GdkWindowHints;` - C 2031-2103 builds `hint_flags` from these.
/// **UNVERIFIED**: each name carries a placeholder bit (`## Owed` 6).
pub type GdkWindowHints = c_int;
pub const GDK_HINT_POS: GdkWindowHints = 1 << 0;                // placeholder, not GDK's value
pub const GDK_HINT_MIN_SIZE: GdkWindowHints = 1 << 1;           // placeholder, not GDK's value
pub const GDK_HINT_BASE_SIZE: GdkWindowHints = 1 << 3;          // placeholder, not GDK's value
pub const GDK_HINT_RESIZE_INC: GdkWindowHints = 1 << 5;         // placeholder, not GDK's value
pub const GDK_HINT_WIN_GRAVITY: GdkWindowHints = 1 << 6;        // placeholder, not GDK's value
pub const GDK_HINT_USER_POS: GdkWindowHints = 1 << 7;           // placeholder, not GDK's value
pub const GDK_HINT_USER_SIZE: GdkWindowHints = 1 << 8;          // placeholder, not GDK's value
/// C: gdk/gdkwindow.h: `typedef enum { GDK_GRAVITY_NORTH_WEST = 1, GDK_GRAVITY_NORTH,
/// GDK_GRAVITY_NORTH_EAST, GDK_GRAVITY_WEST, GDK_GRAVITY_CENTER, GDK_GRAVITY_EAST,
/// GDK_GRAVITY_SOUTH_WEST, GDK_GRAVITY_SOUTH, GDK_GRAVITY_SOUTH_EAST, GDK_GRAVITY_STATIC }
/// GdkGravity;` and `GdkGravity gtk_window_get_gravity (GtkWindow *window);` - C 1711/1894/2054-
/// 2076 read and build these.  **UNVERIFIED**: each name carries a placeholder value
/// (`## Owed` 6), distinct so the ten comparisons stay ten.
pub type GdkGravity = c_int;
pub const GDK_GRAVITY_NORTH_WEST: GdkGravity = 1;               // placeholder, not GDK's value
pub const GDK_GRAVITY_NORTH: GdkGravity = 2;                    // placeholder, not GDK's value
pub const GDK_GRAVITY_NORTH_EAST: GdkGravity = 3;               // placeholder, not GDK's value
pub const GDK_GRAVITY_WEST: GdkGravity = 4;                     // placeholder, not GDK's value
pub const GDK_GRAVITY_CENTER: GdkGravity = 5;                   // placeholder, not GDK's value
pub const GDK_GRAVITY_EAST: GdkGravity = 6;                     // placeholder, not GDK's value
pub const GDK_GRAVITY_SOUTH_WEST: GdkGravity = 7;               // placeholder, not GDK's value
pub const GDK_GRAVITY_SOUTH: GdkGravity = 8;                    // placeholder, not GDK's value
pub const GDK_GRAVITY_SOUTH_EAST: GdkGravity = 9;               // placeholder, not GDK's value
/// C: gdk/gdkwindow.h: `typedef enum { … GDK_WINDOW_TYPE_HINT_TOOLTIP, … } GdkWindowTypeHint;`
/// (C 2754 passes it to `gtk_window_set_type_hint`).  C's enum ABI is `c_int` (PORTING.md 1.3);
/// **UNVERIFIED** like every other GDK enumerator here, and the one member this tree names is a
/// placeholder for the same reason.
pub type GdkWindowTypeHint = c_int;
pub const GDK_WINDOW_TYPE_HINT_TOOLTIP: GdkWindowTypeHint = 0;   // placeholder, not GDK's value
pub const GDK_GRAVITY_STATIC: GdkGravity = 10;                  // placeholder, not GDK's value
/// C: gdk/gdkwindow.h: `typedef struct _GdkGeometry GdkGeometry;` with `gint min_width;
/// gint min_height; gint max_width; gint max_height; gint base_width; gint base_height; gint
/// width_inc; gint height_inc; gdouble min_aspect; gdouble max_aspect; GdkGravity win_gravity;`.
/// C 1989-2099 `memset`s the whole record, writes seven of the members and `memcmp`s the whole
/// record, so the port carries all eleven members rather than the seven (a partial record would
/// make C's `sizeof` span wrong).  **UNVERIFIED**: no GDK header on this machine, so no size or
/// offset assertion is written.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkGeometry
{
  pub min_width: c_int,            // C: gdk/gdkwindow.h: `gint min_width;`
  pub min_height: c_int,           // C: gdk/gdkwindow.h: `gint min_height;`
  pub max_width: c_int,            // C: gdk/gdkwindow.h: `gint max_width;`
  pub max_height: c_int,           // C: gdk/gdkwindow.h: `gint max_height;`
  pub base_width: c_int,           // C: gdk/gdkwindow.h: `gint base_width;`
  pub base_height: c_int,          // C: gdk/gdkwindow.h: `gint base_height;`
  pub width_inc: c_int,            // C: gdk/gdkwindow.h: `gint width_inc;`
  pub height_inc: c_int,           // C: gdk/gdkwindow.h: `gint height_inc;`
  pub min_aspect: c_double,        // C: gdk/gdkwindow.h: `gdouble min_aspect;`
  pub max_aspect: c_double,        // C: gdk/gdkwindow.h: `gdouble max_aspect;`
  pub win_gravity: GdkGravity,     // C: gdk/gdkwindow.h: `GdkGravity win_gravity;`
}
unsafe extern "C" {
  /// C 1408: `gdkwin = gdk_x11_window_lookup_for_display (gdk_x11_lookup_xdisplay (dpy), wdesc);`
  /// C: gdk/gdkx.h: `gpointer gdk_x11_window_lookup_for_display (GdkDisplay *display, Window
  /// window);`
  /* `<gtk/gtk.h>`'s X11 half: these four entries are gdk/gdkx.h's and gtk/gtkx.h's, and their
     parameters name X11's `Window`/`Display`, whose home is `include/xlib.rs` - the other mirror
     this surface is not allowed to name (`include/xlib.rs` 847/880).  C reaches them only through
     `<gtk/gtk.h>` (USE_GTK) *and* the X11 headers, so the entry carries that gate: a unit whose
     graph has GLib but not GTK/X11 - `src/xgselect.rs`, whose C 27 is `<glib.h>` alone - must not
     see them (status 3.477: the `have_glib`-only flip named exactly these four).  */
  #[cfg (feature = "use_gtk")]
  pub fn gdk_x11_window_lookup_for_display (display: *mut GdkDisplay, window: Window) -> gpointer;
  /// C 1352/1365: `gdk_window_move_resize (gwin, x, y, outer_width, outer_height);`
  /// C: gdk/gdkwindow.h: `void gdk_window_move_resize (GdkWindow *window, gint x, gint y, gint
  /// width, gint height);`
  pub fn gdk_window_move_resize (window: *mut GdkWindow, x: gint, y: gint, width: gint,
				 height: gint);
  /// C 1782/1903/2255: `gdk_window_set_override_redirect (gwin, TRUE);`
  /// C: gdk/gdkwindow.h: `void gdk_window_set_override_redirect (GdkWindow *window, gboolean
  /// override_redirect);`
  pub fn gdk_window_set_override_redirect (window: *mut GdkWindow, override_redirect: gboolean);
  /// C 2174: `gdk_window_restack (gwin1, gwin2, above_flag as gboolean);`
  /// C: gdk/gdkwindow.h: `void gdk_window_restack (GdkWindow *window, GdkWindow *sibling,
  /// gboolean above);`
  pub fn gdk_window_restack (window: *mut GdkWindow, sibling: *mut GdkWindow, above: gboolean);
  /// C 2192/2197: `gdk_window_set_skip_taskbar_hint (…, NILP (skip_taskbar) ? FALSE : TRUE);`
  /// C: gdk/gdkwindow.h: `void gdk_window_set_skip_taskbar_hint (GdkWindow *window, gboolean
  /// skips_taskbar);`
  pub fn gdk_window_set_skip_taskbar_hint (window: *mut GdkWindow, skips_taskbar: gboolean);
  /// C 1515: `const char *display_name = gdk_display_get_name (gdpy);`
  /// C: gdk/gdkdisplay.h: `const gchar *gdk_display_get_name (GdkDisplay *display);`
  pub fn gdk_display_get_name (display: *mut GdkDisplay) -> *const c_char;
  /// C 1820/1923: `gdk_screen_get_display (screen)`
  /// C: gdk/gdkscreen.h: `GdkDisplay *gdk_screen_get_display (GdkScreen *screen);`
  pub fn gdk_screen_get_display (screen: *mut GdkScreen) -> *mut GdkDisplay;
  /// C 1741: `GdkVisual *visual = gdk_screen_get_rgba_visual (screen);`
  /// C: gdk/gdkscreen.h: `GdkVisual *gdk_screen_get_rgba_visual (GdkScreen *screen);`
  pub fn gdk_screen_get_rgba_visual (screen: *mut GdkScreen) -> *mut GdkVisual;
}
#[cfg (not (feature = "have_pgtk"))]
unsafe extern "C" {
  /// C 1735: `GdkVisual *visual = gdk_x11_screen_lookup_visual (screen,
  /// XVisualIDFromVisual (FRAME_X_VISUAL (f)));`
  /// C: gdk/gdkx.h: `GdkVisual *gdk_x11_screen_lookup_visual (GdkScreen *screen,
  /// VisualID xvisualid);`  `VisualID` is X11's, not this mirror's (`X11/X.h` 76:
  /// `typedef unsigned long VisualID;`, aliased by `include/xlib.rs` 842-844): a scalar typedef,
  /// so P-55's ABI-equivalent spelling is the integer it aliases (`c_ulong`), which is also what
  /// keeps this declaration inside the P-87 ③ rule that a mirror names only its own names.
  pub fn gdk_x11_screen_lookup_visual (screen: *mut GdkScreen, xvisualid: c_ulong)
				       -> *mut GdkVisual;
}
/// C: gdk/gdkcolor.h (GTK2's GDK): `void gdk_colormap_query_color (GdkColormap *colormap,
/// gulong pixel, GdkColor *result);` - C 1455's one call.  **UNVERIFIED**.
#[cfg (not (feature = "have_gtk3"))]
unsafe extern "C" {
  pub fn gdk_colormap_query_color (colormap: *mut GdkColormap, pixel: gulong,
				   result: *mut GdkColor);
}

/* GTK, continued.  */
/// C: gtk/gtkwindow.h: `typedef enum { GTK_WINDOW_TOPLEVEL, GTK_WINDOW_POPUP } GtkWindowType;` -
/// C 1588/1604/1839 read these.  **UNVERIFIED**: placeholders (`## Owed` 6).
pub type GtkWindowType = c_int;
pub const GTK_WINDOW_TOPLEVEL: GtkWindowType = 0;               // placeholder, not GTK's value
pub const GTK_WINDOW_POPUP: GtkWindowType = 1;                  // placeholder, not GTK's value
/// C: gtk/gtkenums.h (GTK3's): `typedef enum { GTK_ORIENTATION_HORIZONTAL, GTK_ORIENTATION_VERTICAL
/// } GtkOrientation;` - C 1627/1628/1849/1850 pass these to `gtk_box_new`.  The GTK2 arm's same
/// names are the GTK2 header's and stay owed (`## Owed` 6); this arm is GTK3's, and the values are
/// **UNVERIFIED** placeholders.
#[cfg (feature = "have_gtk3")]
pub type GtkOrientation = c_int;
#[cfg (feature = "have_gtk3")]
pub const GTK_ORIENTATION_HORIZONTAL: GtkOrientation = 0;       // placeholder, not GTK's value
#[cfg (feature = "have_gtk3")]
pub const GTK_ORIENTATION_VERTICAL: GtkOrientation = 1;         // placeholder, not GTK's value
/// C: gtk/gtkstyle.h (GTK2's): `typedef struct _GtkRcStyle GtkRcStyle;` - C 1583/1763-1767 hold
/// one and write its `bg_pixmap_name` array.  **UNVERIFIED** (GTK2's own surface).
#[cfg (not (feature = "have_gtk3"))]
#[repr (C)]
pub struct GtkRcStyle
{
  _private: [u8; 0],
}
/// C: gtk/gtkimmulticontext.h / gtk/gtkimcontext.h: `typedef struct _GtkIMContext GtkIMContext;`
/// - C 1586/1793-1802 hold one and pass it to `g_signal_connect` and `FRAME_X_OUTPUT (f)`.
/// **UNVERIFIED**.
#[repr (C)]
pub struct GtkIMContext
{
  _private: [u8; 0],
}
/// C: gtk/gtkcssprovider.h (GTK3's): `typedef struct _GtkCssProvider GtkCssProvider;` - C 1445-
/// 1450 hold one.  **UNVERIFIED**.
#[cfg (feature = "have_gtk3")]
#[repr (C)]
pub struct GtkCssProvider
{
  _private: [u8; 0],
}
/// C: gtk/gtkstylecontext.h (GTK3's): `typedef struct _GtkStyleProvider GtkStyleProvider;` - the
/// pointee of `GTK_STYLE_PROVIDER` C 1448 hands to `gtk_style_context_add_provider`.
/// **UNVERIFIED**.
#[cfg (feature = "have_gtk3")]
#[repr (C)]
pub struct GtkStyleProvider
{
  _private: [u8; 0],
}
/// C: gtk/gtkbox.h: `typedef struct _GtkBox GtkBox;` - the pointee of `GTK_BOX` C 1629/1630/
/// 1677/1678/1851/1852/1878 pass to `gtk_box_set_homogeneous`/`gtk_box_pack_start`.
/// **UNVERIFIED**.
#[repr (C)]
pub struct GtkBox
{
  _private: [u8; 0],
}
/// C: gobject/gparam.h: `typedef struct _GParamSpec GParamSpec;` - C 1510's callback parameter.
/// **UNVERIFIED**.
#[repr (C)]
pub struct GParamSpec
{
  _private: [u8; 0],
}
/// C: gtk/gtkenums.h (GTK2's): `GTK_STATE_NORMAL` (C 1456's `gtk_widget_modify_bg`, C 1766's
/// `bg_pixmap_name` index).  **UNVERIFIED**: placeholder (`## Owed` 6).
#[cfg (not (feature = "have_gtk3"))]
pub const GTK_STATE_NORMAL: GtkStateType = 0;                   // placeholder, not GTK's value
/// C: gtk/gtkstylecontext.h: `#define GTK_STYLE_PROVIDER_PRIORITY_APPLICATION 600` - C 1449's
/// one use.  **UNVERIFIED**: placeholder (`## Owed` 6, alongside `src/pgtkterm.rs` 547's same
/// family).
#[cfg (feature = "have_gtk3")]
pub const GTK_STYLE_PROVIDER_PRIORITY_APPLICATION: c_uint = 0;  // placeholder, not GTK's value
/// C: gtk/gtkbox.h / gtk/gtkstylecontext.h: the two checked-cast wrappers C 1629/1630/1677/1678/
/// 1851/1852/1878/1448 use, each keeping the macro's name and its result type (the GLib cast
/// chains are owed, as above).
macro_rules! GTK_BOX {
  ($o:expr) => { ($o) as *mut GtkBox };
}
#[cfg (feature = "have_gtk3")]
macro_rules! GTK_STYLE_PROVIDER {
  ($o:expr) => { ($o) as *mut GtkStyleProvider };
}
/// C: gtk/gtk.h: `GtkWidget *gtk_box_new (GtkOrientation orientation, gint spacing);` - GTK3's
/// function (the GTK2 arm's `gtk_box_new` is C 119-121's macro, carried above).  C's call sites
/// (`xg_create_frame_widgets` C 1627/1628 and `xg_create_frame_outer_widgets` C 1849/1850) keep
/// C's text in both arms, so the port's uniform spelling is the macro form, which forwards to
/// this function here (the `src/image.rs` 1849-1872 `image_get_x_image_or_dc` shape).
#[cfg (feature = "have_gtk3")]
macro_rules! gtk_box_new {
  ($ori:expr, $spacing:expr) => { gtk_box_new ($ori, $spacing) };
}
unsafe extern "C" {
  /// C 1600/1606/1844: `wtop = gtk_window_new (type);`
  /// C: gtk/gtkwindow.h: `GtkWidget *gtk_window_new (GtkWindowType type);`
  pub fn gtk_window_new (type_: GtkWindowType) -> *mut GtkWidget;
  /// C 1607/1713/1845: `gtk_widget_add_events (wtop, GDK_ALL_EVENTS_MASK);` / `… (wfixed, …)`
  /// C: gtk/gtkwidget.h: `void gtk_widget_add_events (GtkWidget *widget, gint events);`
  pub fn gtk_widget_add_events (widget: *mut GtkWidget, events: gint);
  /// C 1610: `gtk_widget_set_app_paintable (wtop, f->alpha_background != 1.0);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_set_app_paintable (GtkWidget *widget, gboolean
  /// app_paintable);`
  pub fn gtk_widget_set_app_paintable (widget: *mut GtkWidget, app_paintable: gboolean);
  /// C 1650/1651/1652/1855/1856: `gtk_widget_set_name (wtop, EMACS_CLASS);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_set_name (GtkWidget *widget, const gchar *name);`
  pub fn gtk_widget_set_name (widget: *mut GtkWidget, name: *const gchar);
  /// C 1640-1643/1949/1960/1974: `gtk_widget_destroy (wtop);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_destroy (GtkWidget *widget);`
  pub fn gtk_widget_destroy (widget: *mut GtkWidget);
  /// C 1674: `gtk_widget_set_has_window (wfixed, TRUE);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_set_has_window (GtkWidget *widget, gboolean
  /// has_window);`
  pub fn gtk_widget_set_has_window (widget: *mut GtkWidget, has_window: gboolean);
  /// C 1676/1877: `gtk_container_add (GTK_CONTAINER (wtop), wvbox);`
  /// C: gtk/gtkcontainer.h: `void gtk_container_add (GtkContainer *container, GtkWidget *widget);`
  pub fn gtk_container_add (container: *mut GtkContainer, widget: *mut GtkWidget);
  /// C 1677/1678/1878: `gtk_box_pack_start (GTK_BOX (wvbox), whbox, TRUE, TRUE, 0);`
  /// C: gtk/gtkbox.h: `void gtk_box_pack_start (GtkBox *box, GtkWidget *child, gboolean expand,
  /// gboolean fill, guint padding);`
  pub fn gtk_box_pack_start (box_: *mut GtkBox, child: *mut GtkWidget, expand: gboolean,
			     fill: gboolean, padding: guint);
  /// C 1629/1630/1851/1852: `gtk_box_set_homogeneous (GTK_BOX (wvbox), FALSE);`
  /// C: gtk/gtkbox.h: `void gtk_box_set_homogeneous (GtkBox *box, gboolean homogeneous);`
  pub fn gtk_box_set_homogeneous (box_: *mut GtkBox, homogeneous: gboolean);
  /// C 1710/1893: `f->win_gravity = gtk_window_get_gravity (GTK_WINDOW (…));`
  /// C: gtk/gtkwindow.h: `GdkGravity gtk_window_get_gravity (GtkWindow *window);`
  pub fn gtk_window_get_gravity (window: *mut GtkWindow) -> GdkGravity;
  /// C 1665/1869/2152: `gtk_window_set_decorated (GTK_WINDOW (wtop), FALSE);`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_decorated (GtkWindow *window, gboolean setting);`
  pub fn gtk_window_set_decorated (window: *mut GtkWindow, setting: gboolean);
  /// C 2749/2750/2751/2752/2753/2754: the five `gtk_window_set_*` calls on the tip window
  /// C: gtk/gtkwindow.h: `void gtk_window_set_transient_for (GtkWindow *window, GtkWindow
  /// *parent);` / `void gtk_window_set_attached_to (GtkWindow *window, GtkWidget
  /// *attach_widget);` / `void gtk_window_set_destroy_with_parent (GtkWindow *window, gboolean
  /// setting);` / `void gtk_window_set_type_hint (GtkWindow *window, GdkWindowTypeHint hint);`
  pub fn gtk_window_set_transient_for (window: *mut GtkWindow, parent: *mut GtkWindow);
  pub fn gtk_window_set_attached_to (window: *mut GtkWindow, attach_widget: *mut GtkWidget);
  pub fn gtk_window_set_destroy_with_parent (window: *mut GtkWindow, setting: gboolean);
  pub fn gtk_window_set_type_hint (window: *mut GtkWindow, hint: GdkWindowTypeHint);
  /// C 1774/1896: `gtk_window_set_resizable (GTK_WINDOW (wtop), TRUE);`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_resizable (GtkWindow *window, gboolean resizable);`
  pub fn gtk_window_set_resizable (window: *mut GtkWindow, resizable: gboolean);
  /// C 1732/1912: `GdkScreen *screen = gtk_widget_get_screen (wtop);`
  /// C: gtk/gtkwidget.h: `GdkScreen *gtk_widget_get_screen (GtkWidget *widget);`
  pub fn gtk_widget_get_screen (widget: *mut GtkWidget) -> *mut GdkScreen;
  /// C 1744/1745: `gtk_widget_set_visual (wtop, visual);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_set_visual (GtkWidget *widget, GdkVisual *visual);`
  pub fn gtk_widget_set_visual (widget: *mut GtkWidget, visual: *mut GdkVisual);
  /// C 1768/1770/2233: `gtk_widget_set_can_focus (wfixed, TRUE);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_set_can_focus (GtkWidget *widget, gboolean can_focus);`
  pub fn gtk_widget_set_can_focus (widget: *mut GtkWidget, can_focus: gboolean);
  /// C 2102: `gtk_window_set_geometry_hints (GTK_WINDOW (…), NULL, &size_hints, hint_flags);`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_geometry_hints (GtkWindow *window, GtkWidget
  /// *geometry_widget, GdkGeometry *geometry, GdkWindowHints geom_mask);`
  pub fn gtk_window_set_geometry_hints (window: *mut GtkWindow, geometry_widget: *mut GtkWidget,
					geometry: *mut GdkGeometry, geom_mask: GdkWindowHints);
  /// C 1415: `gwdesc = gtk_get_event_widget (&event);`
  /// C: gtk/gtkmain.h: `GtkWidget *gtk_get_event_widget (GdkEvent *event);`
  pub fn gtk_get_event_widget (event: *mut GdkEvent) -> *mut GtkWidget;
  /// C 2219: `gtk_window_set_focus_on_map (gwin, g_no_focus_on_map);`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_focus_on_map (GtkWindow *window, gboolean
  /// setting);`
  pub fn gtk_window_set_focus_on_map (window: *mut GtkWindow, setting: gboolean);
  /// C 2241: `gtk_window_set_accept_focus (gwin, g_no_accept_focus);`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_accept_focus (GtkWindow *window, gboolean
  /// setting);`
  pub fn gtk_window_set_accept_focus (window: *mut GtkWindow, setting: gboolean);
  /// C 2276: `gtk_window_set_icon (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)), gp);`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_icon (GtkWindow *window, GdkPixbuf *icon);`
  /// C 2760 (src/xwidget.c): `xv->widget = gtk_drawing_area_new ();` - the PGTK xwidget arm.
  /// C: gtk/gtkdrawingarea.h: `GtkWidget *gtk_drawing_area_new (void);`
  pub fn gtk_drawing_area_new () -> *mut GtkWidget;
}
#[cfg (feature = "have_gtk3")]
unsafe extern "C" {
  /// C 1627/1628/1849/1850: `wvbox = gtk_box_new (GTK_ORIENTATION_VERTICAL, 0);` (the GTK2 arm's
  /// `gtk_box_new` is C 119-121's macro above; this is GTK3's function, which the forwarding macro
  /// also names)
  /// C: gtk/gtkbox.h: `GtkWidget *gtk_box_new (GtkOrientation orientation, gint spacing);`
  pub fn gtk_box_new (orientation: GtkOrientation, spacing: gint) -> *mut GtkWidget;
  /// C 1445-1446: `GtkCssProvider *provider = gtk_css_provider_new ();` /
  /// `gtk_css_provider_load_from_data (provider, buffer, -1, NULL);`
  /// C: gtk/gtkcssprovider.h: `GtkCssProvider *gtk_css_provider_new (void);` / `void
  /// gtk_css_provider_load_from_data (GtkCssProvider *css_provider, const gchar *data, gssize
  /// length, GError **error);`
  pub fn gtk_css_provider_new () -> *mut GtkCssProvider;
  pub fn gtk_css_provider_load_from_data (css_provider: *mut GtkCssProvider, data: *const gchar,
					  length: isize, error: *mut *mut GError);
  /// C 1447-1449: `gtk_style_context_add_provider (gtk_widget_get_style_context (w),
  /// GTK_STYLE_PROVIDER (provider), GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);`
  /// C: gtk/gtkstylecontext.h: `void gtk_style_context_add_provider (GtkStyleContext *context,
  /// GtkStyleProvider *provider, guint priority);`
  pub fn gtk_style_context_add_provider (context: *mut GtkStyleContext,
					 provider: *mut GtkStyleProvider, priority: guint);
}
#[cfg (not (feature = "have_pgtk"))]
unsafe extern "C" {
  /// C 1597: `wtop = gtk_plug_new_for_display (gdpy, f->output_data.xp->parent_desc);`
  /// C: gtk/gtkx.h: `GtkWidget *gtk_plug_new_for_display (GdkDisplay *display, Window socket_id);`
  #[cfg (feature = "use_gtk")]
  pub fn gtk_plug_new_for_display (display: *mut GdkDisplay, socket_id: Window) -> *mut GtkWidget;
}
/* The GTK entry points this block also carried belong to the *surface*: GTK3 provides them in both
   arms and `src/pgtkterm.c` calls `gtk_im_multicontext_new`, so they stand ungated (P-90 ②).  */
unsafe extern "C" {
  /// C 1793-1794: `imc = gtk_im_multicontext_new ();` /
  /// `gtk_im_context_set_use_preedit (imc, TRUE);`
  /// C: gtk/gtkimmulticontext.h: `GtkIMContext *gtk_im_multicontext_new (void);` /
  /// gtk/gtkimcontext.h: `void gtk_im_context_set_use_preedit (GtkIMContext *context, gboolean
  /// use_preedit);`
  pub fn gtk_im_multicontext_new () -> *mut GtkIMContext;
  pub fn gtk_im_context_set_use_preedit (context: *mut GtkIMContext, use_preedit: gboolean);
  /// C 1690: `gtk_widget_set_double_buffered (wfixed, FALSE);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_set_double_buffered (GtkWidget *widget, gboolean
  /// double_buffered);`
  pub fn gtk_widget_set_double_buffered (widget: *mut GtkWidget, double_buffered: gboolean);
  /// C 1625: `if (!gtk_window_parse_geometry ...)`'s sibling - the GTK2 arm's resize grip:
  /// C: gtk/gtkwindow.h: `void gtk_window_set_has_resize_grip (GtkWindow *window, gboolean
  /// setting);` (`HAVE_GTK_WINDOW_SET_HAS_RESIZE_GRIP`)
  pub fn gtk_window_set_has_resize_grip (window: *mut GtkWindow, setting: gboolean);
}
#[cfg (not (feature = "gtk_check_version_ge_3_22"))]
unsafe extern "C" {
  /// C 1694/1884: `gtk_window_set_wmclass (GTK_WINDOW (wtop), SSDATA (Vx_resource_name),
  /// SSDATA (Vx_resource_class));`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_wmclass (GtkWindow *window, const gchar
  /// *wmclass_name, const gchar *wmclass_class);`
  pub fn gtk_window_set_wmclass (window: *mut GtkWindow, wmclass_name: *const gchar,
				 wmclass_class: *const gchar);
}
unsafe extern "C" {
  /// C 1772: `gtk_widget_grab_focus (wfixed);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_grab_focus (GtkWidget *widget);`
  pub fn gtk_widget_grab_focus (widget: *mut GtkWidget);
}
#[cfg (not (feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 1635: `wfixed = gtk_fixed_new ();` (GTK2's)
  /// C: gtk/gtkfixed.h: `GtkWidget *gtk_fixed_new (void);`
  pub fn gtk_fixed_new () -> *mut GtkWidget;
  /// C 1763: `style = gtk_widget_get_modifier_style (wfixed);`
  /// C: gtk/gtkwidget.h: `GtkRcStyle *gtk_widget_get_modifier_style (GtkWidget *widget);`
  pub fn gtk_widget_get_modifier_style (widget: *mut GtkWidget) -> *mut GtkRcStyle;
  /// C 1767: `gtk_widget_modify_style (wfixed, style);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_modify_style (GtkWidget *widget, GtkRcStyle *style);`
  pub fn gtk_widget_modify_style (widget: *mut GtkWidget, style: *mut GtkRcStyle);
  /// C 1456: `gtk_widget_modify_bg (FRAME_GTK_WIDGET (f), GTK_STATE_NORMAL, &bg);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_modify_bg (GtkWidget *widget, GtkStateType state,
  /// const GdkColor *color);`
  pub fn gtk_widget_modify_bg (widget: *mut GtkWidget, state: GtkStateType,
			       color: *const GdkColor);
  /// C 1454: `GdkColormap *map = gtk_widget_get_colormap (w);`
  /// C: gtk/gtkwidget.h: `GdkColormap *gtk_widget_get_colormap (GtkWidget *widget);`
  pub fn gtk_widget_get_colormap (widget: *mut GtkWidget) -> *mut GdkColormap;
}

/* The GTK/GDK entry points whose C *uses* sit inside an arm: C 207-221 and the three
   display functions' X branches call the two `gdk_x11_*` lookups and the two
   `x_display_pixel_*` measurements only under `#ifndef HAVE_PGTK`, and C 824-942's tooltip
   callbacks/preparer are that arm's too - so their declarations carry that gate rather than the
   file gate alone (C's own structure: the call is inside the arm).  */
#[cfg (not (feature = "have_pgtk"))]
unsafe extern "C" {
  /// C 207/307/321/373: `gdk_x11_lookup_xdisplay (FRAME_X_DISPLAY (f))` / `… (dpy)`
  /// C: gdk/gdkx.h: `GdkDisplay *gdk_x11_lookup_xdisplay (Display *xdisplay);`
  #[cfg (feature = "use_gtk")]
  pub fn gdk_x11_lookup_xdisplay (xdisplay: *mut Display) -> *mut GdkDisplay;
  /// C 260: `GDK_DISPLAY_XDISPLAY (gdpy)`
  /// C: gdk/gdkx.h: `Display *gdk_x11_display_get_xdisplay (GdkDisplay *display);`
  #[cfg (feature = "use_gtk")]
  pub fn gdk_x11_display_get_xdisplay (display: *mut GdkDisplay) -> *mut Display;
}
/* The *GTK* entry points this block carried are the surface's too (`src/pgtkterm.c` calls
   `gtk_widget_get_parent`), so they stand ungated; the two X11-GDK lookups above stay X-only.  */
unsafe extern "C" {
  /// C 867: `p = gtk_widget_get_parent (x->ttip_lbl);`
  /// C: gtk/gtkwidget.h: `GtkWidget *gtk_widget_get_parent (GtkWidget *widget);`
  pub fn gtk_widget_get_parent (widget: *mut GtkWidget) -> *mut GtkWidget;
  /// C 868: `list = gtk_container_get_children (GTK_CONTAINER (p));`
  /// C: gtk/gtkcontainer.h: `GList *gtk_container_get_children (GtkContainer *container);`
  pub fn gtk_container_get_children (container: *mut GtkContainer) -> *mut GList;
  /// C 861: `x->ttip_lbl = gtk_label_new ("");`
  /// C: gtk/gtklabel.h: `GtkWidget *gtk_label_new (const gchar *str);`
  pub fn gtk_label_new (str: *const gchar) -> *mut GtkWidget;
  /// C 873: `gtk_label_set_line_wrap (GTK_LABEL (w), FALSE);`
  /// C: gtk/gtklabel.h: `void gtk_label_set_line_wrap (GtkLabel *label, gboolean wrap);`
  pub fn gtk_label_set_line_wrap (label: *mut GtkLabel, wrap: gboolean);
  /// C 863/933: `gtk_tooltip_set_custom (tooltip, x->ttip_lbl);`
  /// C: gtk/gtktooltip.h: `void gtk_tooltip_set_custom (GtkTooltip *tooltip, GtkWidget *custom);`
  pub fn gtk_tooltip_set_custom (tooltip: *mut GtkTooltip, custom: *mut GtkWidget);
  /// C 916/984: `settings = gtk_settings_get_for_screen (screen);`
  /// C: gtk/gtksettings.h: `GtkSettings *gtk_settings_get_for_screen (GdkScreen *screen);`
  pub fn gtk_settings_get_for_screen (screen: *mut GdkScreen) -> *mut GtkSettings;
  /// C 928: `gtk_widget_get_display (GTK_WIDGET (x->ttip_window))`
  /// C: gtk/gtkwidget.h: `GdkDisplay *gtk_widget_get_display (GtkWidget *widget);`
  pub fn gtk_widget_get_display (widget: *mut GtkWidget) -> *mut GdkDisplay;
  /// C 934: `gtk_tooltip_set_text (x->ttip_widget, SSDATA (encoded_string));`
  /// C: gtk/gtktooltip.h: `void gtk_tooltip_set_text (GtkTooltip *tooltip, const gchar *text);`
  pub fn gtk_tooltip_set_text (tooltip: *mut GtkTooltip, text: *const gchar);
  /// C 935: `gtk_widget_get_preferred_size (GTK_WIDGET (x->ttip_window), NULL, &req);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_get_preferred_size (GtkWidget *widget, GtkRequisition
  /// *minimum_size, GtkRequisition *natural_size);` (xterm.h 1434-1445's GTK2 arm redefines the
  /// name as `gtk_widget_size_request`, which `include/xterm.rs` 1016-1021 carries).
  /// C 3319 (src/xwidget.c): `gtk_widget_size_request (XXWIDGET (xwidget)->widget_osr,
  /// &requisition);` - GTK3's deprecated alias of `gtk_widget_get_preferred_size` above
  /// (`xterm.h` 1434-1445's GTK2 arm redefines the name; GTK3 declares both).
  /// C: gtk/gtkwidget.h: `void gtk_widget_size_request (GtkWidget *widget, GtkRequisition
  /// *requisition);`
  pub fn gtk_widget_size_request (widget: *mut GtkWidget, requisition: *mut GtkRequisition);
  pub fn gtk_widget_get_preferred_size (widget: *mut GtkWidget,
                                        minimum_size: *mut GtkRequisition,
                                        natural_size: *mut GtkRequisition);
}
/* The two X11 display measurements (and the two frame utilities C 1280-1281 uses) are the X arm's
   alone: their parameter is `include/xterm.rs`'s `x_display_info`, which the pgtk arm does not
   have (C calls them under `#ifndef HAVE_PGTK`).  */
#[cfg (not (feature = "have_pgtk"))]
unsafe extern "C" {
  /// C 1062's `f->size_hint_flags & (USPosition | PPosition)`, C 1076/1082's two display
  /// measurements, C 1099/1131's two GTK calls that take the widget, and C 1280-1281's two frame
  /// utilities with their frame.h lines.
  /// C: xterm.h 1863-1864: `extern int x_display_pixel_height (struct x_display_info *);` /
  /// `extern int x_display_pixel_width (struct x_display_info *);`
  /// P-55/P-87 ③: `x_display_info` is `include/xterm.rs`'s record (xterm.h 383), not this
  /// mirror's, and the crate the layout probe builds compiles neither the record nor the
  /// `have_x_windows` arm that declares it, so this mirror must not name it.  It appears only
  /// behind a pointer, so the parameter takes the P-55 stand-in and keeps C's text in the
  /// `/// C:` line above; the consumer converts at the call site - the `include/xsettings.rs`
  /// 12-14 rule for this same pair (PORTING.md 0.1-9); C's two call sites at 1076/1082 do that.
  pub fn x_display_pixel_width (dpyinfo: *mut c_void) -> c_int;
  pub fn x_display_pixel_height (dpyinfo: *mut c_void) -> c_int;
}
/* C 768-800's `#ifdef HAVE_GTK3` arm of `xg_check_special_colors`: the three GTK3 style calls
   are that arm's, so they carry the version gate.  */
#[cfg (feature = "have_gtk3")]
unsafe extern "C" {
  /// C 769: `gsty = gtk_widget_get_style_context (FRAME_GTK_OUTER_WIDGET (f));`
  /// C: gtk/gtkstylecontext.h: `GtkStyleContext *gtk_widget_get_style_context (GtkWidget
  /// *widget);`
  pub fn gtk_widget_get_style_context (widget: *mut GtkWidget) -> *mut GtkStyleContext;
  /// C 773: `gtk_style_context_get_color (gsty, state, &col);`
  /// C: gtk/gtkstylecontext.h: `void gtk_style_context_get_color (GtkStyleContext *context,
  /// GtkStateFlags state, GdkRGBA *color);`
  pub fn gtk_style_context_get_color (context: *mut GtkStyleContext, state: GtkStateFlags,
                                      color: *mut GdkRGBA);
  /// C 780-782: `gtk_style_context_get (gsty, state, GTK_STYLE_PROPERTY_BACKGROUND_COLOR, &c,
  /// NULL);`
  /// C: gtk/gtkstylecontext.h: `void gtk_style_context_get (GtkStyleContext *context,
  /// GtkStateFlags state, ...);`
  pub fn gtk_style_context_get (context: *mut GtkStyleContext, state: GtkStateFlags, ...);
}

/* ----------------------------------------------------------------------------------------------
   The names the other two consumers' in-unit segments carried that `src/gtkutil.rs`'s did not
   (from `src/pgtkterm.rs` 377-663 and `src/pgtkfns.rs` 119-470), moved verbatim with them in the
   same P-90 round.  `src/pgtkfns.rs`'s own segment stays in place - this round cannot write that
   file - see `## Owed` in `docs/ports/include-gtk.md`.
   ---------------------------------------------------------------------------------------------- */

/// C: gmain.h 202: `typedef gboolean (*GSourceFunc) (gpointer user_data);`
pub type GSourceFunc = Option<unsafe extern "C" fn (user_data: gpointer) -> gboolean>;

pub const G_SIGNAL_MATCH_DATA: GSignalMatchType = 1 << 4;

/// C: gsignal.h 635-638: `#define g_signal_handlers_disconnect_by_func(instance, func, data)
/// g_signal_handlers_disconnect_matched ((instance), (GSignalMatchType)
/// (G_SIGNAL_MATCH_FUNC | G_SIGNAL_MATCH_DATA), 0, 0, NULL, (func), (data))` - GLib's own body,
/// and C 148-150 is the one call site.
macro_rules! g_signal_handlers_disconnect_by_func {
  ($instance:expr, $func:expr, $data:expr) => {
    g_signal_handlers_disconnect_matched (($instance) as gpointer,
                                          (G_SIGNAL_MATCH_FUNC | G_SIGNAL_MATCH_DATA)
                                          as GSignalMatchType,
                                          0, 0, std::ptr::null_mut (), ($func), ($data))
  };
}

#[repr(C)]
pub struct GtkFixed
{
  _private: [u8; 0],
}

#[repr(C)]
pub struct GdkSeat
{
  _private: [u8; 0],
}

#[repr(C)]
pub struct GdkDevice
{
  _private: [u8; 0],
}

/// C: gdk/gdkdnd.h: `typedef struct _GdkDragContext GdkDragContext;`
#[repr(C)]
pub struct GdkDragContext
{
  _private: [u8; 0],
}

/// C: gdk/gdkdevice.h: `typedef enum { GDK_SOURCE_MOUSE, … } GdkInputSource;` (C 189/193 use it as
/// a `%u` value).  C's enum ABI is `c_int` (PORTING.md 1.3), so no record measurement is owed.
pub type GdkInputSource = c_int;

/// C: gdk/gdkseat.h: `#define GDK_SEAT_CAPABILITY_ALL ((1 << 3) - 1)`; gdk/gdkseat.h,
/// gdk/gdkdevice.h, gtk/gtkwindow.h, gtk/gtkbox.h, gtk/gtkfixed.h, gtk/gtkcontainer.h and
/// gtk/gtkstylecontext.h: the eight checked-cast wrappers.  Each keeps the macro's name and its
/// result type; GDK's and GTK's cast chains are GLib's `G_TYPE_CHECK_INSTANCE_CAST`, recorded as a
/// deviation (below).
pub const GDK_SEAT_CAPABILITY_ALL: c_uint = (1 << 3) - 1;

macro_rules! GDK_SEAT {
  ($o:expr) => { ($o) as *mut GdkSeat };
}

macro_rules! GDK_DEVICE {
  ($o:expr) => { ($o) as *mut GdkDevice };
}

macro_rules! GTK_FIXED {
  ($o:expr) => { ($o) as *mut GtkFixed };
}

/// C: gtk/gtkstylecontext.h: `#define GTK_STYLE_PROVIDER_PRIORITY_USER 800`
pub const GTK_STYLE_PROVIDER_PRIORITY_USER: c_uint = 800;

/// C: cairo.h 2563: `typedef enum _cairo_surface_type { CAIRO_SURFACE_TYPE_IMAGE, …,
/// CAIRO_SURFACE_TYPE_SVG, … } cairo_surface_type_t;` (the enum closes at cairo.h 2589).  Measured
/// on this machine (`clang -std=gnu23 -I /opt/homebrew/include/cairo`, cairo 1.18.4): `sizeof` 4 -
/// C's enum ABI is `c_int` (PORTING.md 1.3).
pub type cairo_surface_type_t = c_int;

pub const CAIRO_SURFACE_TYPE_IMAGE: cairo_surface_type_t = 0;   // C: cairo.h 2564

pub const CAIRO_SURFACE_TYPE_PDF: cairo_surface_type_t = 1;     // C: cairo.h 2565

pub const CAIRO_SURFACE_TYPE_PS: cairo_surface_type_t = 2;      // C: cairo.h 2566

pub const CAIRO_SURFACE_TYPE_SVG: cairo_surface_type_t = 10;    // C: cairo.h 2574

/// C: cairo.h 3244: `typedef struct _cairo_region cairo_region_t;` - the pointee of
/// `gdk_window_set_opaque_region`'s second parameter (C 237 passes `NULL`); the port never
/// allocates or dereferences one.
pub enum cairo_region_t {}

/// C: glibconfig.h 80: `typedef signed long gssize;` (`gtk_css_provider_load_from_data`'s third
/// parameter; C 881 passes -1).
pub type gssize = c_long;

unsafe extern "C" {
/// C 273/278/281: `cairo_t *cr = FRAME_CR_ACTIVE_CONTEXT (f);` / `cairo_destroy (cr);` /
/// `cairo_reference (FRAME_CR_CONTEXT (f))`
/// C: cairo.h 543: `cairo_public cairo_t *cairo_reference (cairo_t *cr);`
pub fn cairo_reference (cr: *mut cairo_t) -> *mut cairo_t;
/// C 74: `#define FRAME_CR_SURFACE(f) (cairo_get_target (FRAME_CR_CONTEXT (f)))`
/// C: cairo.h 2120: `cairo_public cairo_surface_t *cairo_get_target (cairo_t *cr);`
pub fn cairo_get_target (cr: *mut cairo_t) -> *mut cairo_surface_t;
/// C: gsignal.h 483-492: `guint g_signal_handlers_disconnect_matched (gpointer instance,
/// GSignalMatchType mask, guint signal_id, GQuark detail, GClosure *closure, GCallback func,
/// gpointer data);`
pub fn g_signal_handlers_disconnect_matched (instance: gpointer,
                                             mask: GSignalMatchType,
                                             signal_id: guint, detail: guint,
                                             closure: *mut GClosure,
                                             func: GCallback,
                                             data: gpointer) -> guint;
/// C: gsignal.h ~ "void g_signal_handler_disconnect (gpointer instance, gulong handler_id);"
pub fn g_signal_handler_disconnect (instance: gpointer, handler_id: gulong);
/// C 822: `guint src = g_timeout_add (msec, pgtk_make_frame_visible_wait_for_event_timeout,
/// &timed_out);`
/// C: gmain.h 950-952: `guint g_timeout_add (guint interval, GSourceFunc function, gpointer
/// data);`
pub fn g_timeout_add (interval: guint, function: GSourceFunc, data: gpointer) -> guint;
/// C 832: `g_source_remove (src);`
/// C: gmain.h 855: `gboolean g_source_remove (guint tag);`
pub fn g_source_remove (tag: guint) -> gboolean;
/// C 141/146: `pgtk_regenerate_devices`' two callers pass `user_data`.
/// C: gdk/gdkseat.h: `GdkSeat *gdk_display_list_seats` is a `GDK_SEAT`-family entry point:
/// `GList *gdk_display_list_seats (GdkDisplay *display);`
pub fn gdk_display_list_seats (display: *mut GdkDisplay) -> *mut GList;
/// C 179: `devices_on_seat = gdk_seat_get_slaves (seat, GDK_SEAT_CAPABILITY_ALL);`
/// C: gdk/gdkseat.h: `GList *gdk_seat_get_slaves (GdkSeat *seat, GdkSeatCapabilities
/// capabilities);`
pub fn gdk_seat_get_slaves (seat: *mut GdkSeat, capabilities: c_uint) -> *mut GList;
/// C 193/194: `source = gdk_device_get_source (rec->device);` / `name = gdk_device_get_name
/// (rec->device);`
/// C: gdk/gdkdevice.h: `GdkInputSource gdk_device_get_source (GdkDevice *device);`
/// C: gdk/gdkdevice.h: `const gchar *gdk_device_get_name (GdkDevice *device);`
pub fn gdk_device_get_source (device: *mut GdkDevice) -> GdkInputSource;
pub fn gdk_device_get_name (device: *mut GdkDevice) -> *const gchar;
/// C 251: `device = gdk_event_get_source_device (event);`
/// C: gdk/gdkevents.h: `GdkDevice *gdk_event_get_source_device (const GdkEvent *event);`
pub fn gdk_event_get_source_device (event: *const GdkEvent) -> *mut GdkDevice;
/// C 393: `return gdk_keyval_name (keysym);`
/// C: gdk/gdkkeys.h: `gchar *gdk_keyval_name (guint keyval);`
pub fn gdk_keyval_name (keyval: guint) -> *mut gchar;
/// C 414/426: `gdk_window_raise (gtk_widget_get_window (FRAME_WIDGET (f)));` /
/// `gdk_window_lower (gtk_widget_get_window (FRAME_WIDGET (f)));`
/// C: gdk/gdkwindow.h: `void gdk_window_raise (GdkWindow *window);` / `void gdk_window_lower
/// (GdkWindow *window);`
pub fn gdk_window_raise (window: *mut GdkWindow);
pub fn gdk_window_lower (window: *mut GdkWindow);
/// C 943/952: `return gdk_screen_get_height (gscr);` / `gdk_screen_get_width (gscr);`
/// C: gdk/gdkscreen.h: `gint gdk_screen_get_height (GdkScreen *screen);` / `gint
/// gdk_screen_get_width (GdkScreen *screen);`
pub fn gdk_screen_get_height (screen: *mut GdkScreen) -> c_int;
pub fn gdk_screen_get_width (screen: *mut GdkScreen) -> c_int;
/// C 500/507: `gtk_widget_get_realized (FRAME_GTK_WIDGET (new_focus))` /
/// `gtk_widget_grab_focus (FRAME_GTK_WIDGET (new_focus));`
/// C: gtk/gtkwidget.h: `gboolean gtk_widget_get_realized (GtkWidget *widget);` / `void
/// gtk_widget_grab_focus (GtkWidget *widget);`
pub fn gtk_widget_get_realized (widget: *mut GtkWidget) -> gboolean;
/// C: `src/pgtkterm.c` 558/992/1048: `gtk_style_context_remove_provider (ctxt,
/// GTK_STYLE_PROVIDER (old));` (`gtkutil.c`'s half of the pair is the `add_provider` one, above).
/// C: gtk/gtkstylecontext.h: `void gtk_style_context_remove_provider (GtkStyleContext *context,
/// GtkStyleProvider *provider);`
/// GTK3's alone: GTK2 has no `GtkStyleContext` at all (its style type is the `GtkStyle` record
/// above), exactly like `gtk_style_context_add_provider`, `GtkCssProvider` and
/// `GTK_STYLE_PROVIDER_PRIORITY_APPLICATION` in the GTK3 block above - so the entry point carries
/// the `have_gtk3` gate of the two types it names instead of standing ungated and naming types an
/// arm without GTK3 does not have (P-87 ③; the pgtk consumers of it are GTK3 by construction).
#[cfg (feature = "have_gtk3")]
pub fn gtk_style_context_remove_provider (context: *mut GtkStyleContext,
                                          provider: *mut GtkStyleProvider);
/// C 744/1026: `gtk_widget_queue_resize (FRAME_WIDGET (f));`
/// C: gtk/gtkwidget.h: `void gtk_widget_queue_resize (GtkWidget *widget);`
pub fn gtk_widget_queue_resize (widget: *mut GtkWidget);
/// C 770/1027/1034: `gtk_widget_show_all (FRAME_GTK_OUTER_WIDGET (f));`
/// C: gtk/gtkwidget.h: `void gtk_widget_show_all (GtkWidget *widget);`
pub fn gtk_widget_show_all (widget: *mut GtkWidget);
/// C 999: `gtk_container_remove (GTK_CONTAINER (whbox_of_f), fixed);`
/// C: gtk/gtkcontainer.h: `void gtk_container_remove (GtkContainer *container,
/// GtkWidget *widget);`
pub fn gtk_container_remove (container: *mut GtkContainer, widget: *mut GtkWidget);
/// C 1032: `gtk_fixed_put (GTK_FIXED (fixed_of_p), fixed, f->left_pos, f->top_pos);`
/// C: gtk/gtkfixed.h: `void gtk_fixed_put (GtkFixed *fixed, GtkWidget *widget, gint x, gint y);`
pub fn gtk_fixed_put (fixed: *mut GtkFixed, widget: *mut GtkWidget, x: c_int, y: c_int);
/// C 723: `gtk_fixed_move (GTK_FIXED (parent), fixed, f->left_pos, f->top_pos);`
/// C: gtk/gtkfixed.h: `void gtk_fixed_move (GtkFixed *fixed, GtkWidget *widget, gint x, gint y);`
pub fn gtk_fixed_move (fixed: *mut GtkFixed, widget: *mut GtkWidget, x: c_int, y: c_int);
pub fn gtk_window_iconify (window: *mut GtkWindow);
pub fn gtk_window_deiconify (window: *mut GtkWindow);
/// C 827: `while (!found && !timed_out) gtk_main_iteration ();`
/// C: gtk/gtkmain.h: `gboolean gtk_main_iteration (void);`
pub fn gtk_main_iteration () -> gboolean;
/// C 740: `pixbuf = gdk_pixbuf_new_from_file (filename, &err);`
/// C: gdk-pixbuf/gdk-pixbuf-core.h: `GdkPixbuf *gdk_pixbuf_new_from_file (const char *filename,
/// GError **error);`
pub fn gdk_pixbuf_new_from_file (filename: *const c_char, error: *mut *mut GError)
                                 -> *mut GdkPixbuf;
/// C 774: `GdkPixbuf *pixbuf = gdk_pixbuf_new_from_xpm_data (data);`
/// C: gdk-pixbuf/gdk-pixbuf-core.h: `GdkPixbuf *gdk_pixbuf_new_from_xpm_data (const char **data);`
pub fn gdk_pixbuf_new_from_xpm_data (data: *mut *const c_char) -> *mut GdkPixbuf;
/// C 236-237: `gdk_window_set_opaque_region (gtk_widget_get_window (…), NULL);`
/// C: gdk/gdkwindow.h: `void gdk_window_set_opaque_region (GdkWindow *window,
/// cairo_region_t *region);`
pub fn gdk_window_set_opaque_region (window: *mut GdkWindow, region: *mut cairo_region_t);
/// C 808/810: `gtk_window_stick (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)));` /
/// `gtk_window_unstick (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)));`
/// C: gtk/gtkwindow.h: `void gtk_window_stick (GtkWindow *window);` / `void gtk_window_unstick
/// (GtkWindow *window);`
pub fn gtk_window_stick (window: *mut GtkWindow);
pub fn gtk_window_unstick (window: *mut GtkWindow);
}

/* ----------------------------------------------------------------------------------------------
   The GDK names `src/pgtkterm.rs`'s in-unit segment did not carry but `emacs/src/pgtkterm.h`
   names (`GdkAtom`, the three event records, `GdkEventSequence` and `GdkRectangle`).  Same P-90
   round and the same rule as the rest of this file: **UNVERIFIED**, no layout assertion.
   ---------------------------------------------------------------------------------------------- */
/// C: gdk/gdktypes.h: `typedef struct _GdkAtom *GdkAtom;` - the type of `selection_input_event`'s
/// `selection`/`target`/`property` (pgtkterm.h 545).  **UNVERIFIED**.
#[repr (C)]
pub struct _GdkAtom
{
  _private: [u8; 0],
}
pub type GdkAtom = *mut _GdkAtom;
/// C: gdk/gdkevents.h: `typedef struct _GdkEventKey GdkEventKey;` - `pgtk_im_filter_keypress`'s
/// second parameter (pgtkterm.h 666) **and** the union member `src/xwidget.c` 643-648 writes
/// `keyval`/`state` through.  The record above (next to the union) is the one definition
/// (P-90 ②: one surface, one mirror); this comment is the second reader's trail.
/// C: gdk/gdkevents.h: `typedef struct _GdkEventProperty GdkEventProperty;` -
/// `pgtk_handle_property_notify`'s parameter (pgtkterm.h 571).  **UNVERIFIED**.
#[repr (C)]
pub struct GdkEventProperty
{
  _private: [u8; 0],
}
/// C: gdk/gdkevents.h: `typedef struct _GdkEventSelection GdkEventSelection;` -
/// `pgtk_handle_selection_notify`'s parameter (pgtkterm.h 572).  **UNVERIFIED**.
#[repr (C)]
pub struct GdkEventSelection
{
  _private: [u8; 0],
}
/// C: gdk/gdkevents.h: `typedef struct _GdkEventSequence GdkEventSequence;` - the pointee of
/// `pgtk_touch_point`'s `number` (pgtkterm.h 79).  **UNVERIFIED**.
#[repr (C)]
pub struct GdkEventSequence
{
  _private: [u8; 0],
}
/// C: gdk/gdktypes.h: `typedef struct { gint x, y; gint width, height; } GdkRectangle;` -
/// `pgtk_im_set_cursor_location`'s fourth parameter (pgtkterm.h 668), passed **by value** in C, so
/// the record's members are carried rather than an opaque handle.  **UNVERIFIED**.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkRectangle
{
  pub x: c_int,        // C: `gint x, y;`
  pub y: c_int,
  pub width: c_int,    // C: `gint width, height;`
  pub height: c_int,
}

/* ----------------------------------------------------------------------------------------------
   GLib's main-context surface (status 3.476).  `src/xgselect.c` is its only consumer, and P-90 ②
   puts every GLib name in this mirror rather than inline in the unit, so the names it needs are
   declared here.  **No GLib tree exists on this machine**, so every constant below is a
   placeholder and no record layout is measured (P-88 ②: no numbers from memory); the ledger's Owed
   names the GLib host that would measure them.
   ---------------------------------------------------------------------------------------------- */

/// C: glib/gmain.h: `typedef struct _GMainContext GMainContext;` (opaque).
#[repr(C)]
pub struct GMainContext
{
  _private: [u8; 0],
}

/// C: glib/gmain.h: `typedef struct _GPollFD { gint fd; gushort events; gushort revents; } GPollFD;`
/// - the offsets follow that declaration and are UNVERIFIED.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GPollFD
{
  pub fd: c_int,          /* C: gint fd; */
  pub events: c_ushort,   /* C: gushort events; */
  pub revents: c_ushort,  /* C: gushort revents; */
}

/// C: glib/gmain.h: `#define G_PRIORITY_LOW 300`
pub const G_PRIORITY_LOW: c_int = 300;      // placeholder, not glib's value
/// C: glib/gmain.h: `#define G_IO_IN 1`
pub const G_IO_IN: c_int = 1;               // placeholder, not glib's value
/// C: glib/gmain.h: `#define G_IO_OUT 4`
pub const G_IO_OUT: c_int = 4;              // placeholder, not glib's value

/// C: glib/gmain.h: the seven entry points `src/xgselect.c` calls.  Every prototype here is
/// `UNVERIFIED` (the header is not on this machine), and `gboolean` is spelled `c_int` as the rest
/// of this mirror spells it.
/// C: GMainContext *g_main_context_default (void);
/// C: gboolean g_main_context_acquire (GMainContext *context);
/// C: void g_main_context_release (GMainContext *context);
/// C: gint g_main_context_query (GMainContext *context, gint max_priority, gint *timeout_,
/// C:   GPollFD *fds, gint n_fds);
/// C: gboolean g_main_context_pending (GMainContext *context);
/// C: void g_main_context_dispatch (GMainContext *context);
/// C: gboolean g_main_context_iteration (GMainContext *context, gboolean may_block);
unsafe extern "C" {
  pub fn g_main_context_default () -> *mut GMainContext;
  pub fn g_main_context_acquire (context: *mut GMainContext) -> c_int;
  pub fn g_main_context_release (context: *mut GMainContext);
  pub fn g_main_context_query (context: *mut GMainContext, max_priority: c_int,
                               timeout_: *mut c_int, fds: *mut GPollFD, n_fds: c_int) -> c_int;
  pub fn g_main_context_pending (context: *mut GMainContext) -> c_int;
  pub fn g_main_context_dispatch (context: *mut GMainContext);
  pub fn g_main_context_iteration (context: *mut GMainContext, may_block: c_int) -> c_int;
}
