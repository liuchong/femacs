// femacs: port of src/androidselect.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` except the two C passes to `record_unwind_protect_ptr`,
// which are `unsafe extern "C" fn` items (the androidmenu.rs / `callproc.rs` `call_process_kill`
// shape).
//
// This is P-86's second unit: the frozen configuration compiles no `androidselect.o`, so
// `extract-abi.py`'s checklist for the file is empty and P-86 carries **both arms** of every
// `#if` here instead.  Two gates, in C's order:
//
//   * the file gate is `all (feature = "have_android", feature = "not_android_stubify")`, on every
//     item *and on C 21-34's include graph* - which is why the frozen configuration compiles
//     nothing here.  `configure.ac` 2594-2602 defines `HAVE_ANDROID` in both Android branches,
//     2605-2606 defines `ANDROID_STUBIFY` only in the branch `XCONFIGURE != android` takes, and
//     2647-2648 adds `androidselect.o` to `ANDROID_OBJ` only in the other one (`REALLY_ANDROID`,
//     2654), so this file's build condition is C's `HAVE_ANDROID && !defined ANDROID_STUBIFY`;
//   * C 432/447's `#if __ANDROID_API__ <= 11` and C 478's `#if __ANDROID_API__ <= 30` are the two
//     derived features `android_api_le_11` / `android_api_le_30` (the xterm.rs
//     `have_xcb_shape_input_rects` shape).  The chain keeps C's three arms and C's own
//     `android_get_current_api_level ()` tests, whose `>= __ANDROID_API__` invariant
//     (android.h 152-158) is what makes the kept ladder C's chain in every configuration; the
//     ledger records that argument and the two feature names this tree does not carry yet.
//
// `docs/ports/androidselect.md` carries the C line ranges, the acceptance readings and every
// deviation (the `<dlfcn.h>` / `<stdio.h>` / termhooks.h names no mirror carries, the two
// `Lisp_Object` to `jobject` conversions, `INT_STRLEN_BOUND`, the measured NDK values, the three
// `goto` targets and the line-count basis).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");

/* C 21-34: the file's own include list.  None of it is reached by a translation unit the frozen
   configuration builds, so the whole graph sits under the file gate (see the header).  `<assert.h>`
   (C 21) names nothing this file uses - the only assertion entry point here is `eassert`, from
   lisp.h - so its gnulib template (`lib/include/assert.in.rs`) is not reached.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/minmax.rs");          // C 22: `#include <minmax.h>` (MIN)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/unistd.in.rs");       // C 23: `#include <unistd.h>` (lseek, SEEK_SET, getpid)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/boot-time.rs");       // C 26: `#include <boot-time.h>` (get_boot_time)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/sys_types.in.rs");    // C 27: `#include <sys/types.h>` (off_t, pid_t)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/limits.in.rs");       // lisp.h 33's `#include <limits.h>` (INT_MIN, INT_MAX)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/lisp.rs");                   // C 29: `#include "lisp.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/blockinput.rs");             // C 30: `#include "blockinput.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/coding.rs");                 // C 31: `#include "coding.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/android.rs");                // C 32: `#include "android.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/androidterm.rs");            // C 33: `#include "androidterm.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/termhooks.rs");              // C 34: `#include "termhooks.h"`

/* C 24: `#include <dlfcn.h>`.  No mirror carries it in this tree, so the unit declares the four
   entries its two `android_api_le_11` / `android_api_le_30` arms name - the `dynlib.rs`
   `<dlfcn.h>` shape ("declared in-unit, the getgroups.c shape").  The two `RTLD_*` values are the
   NDK's own, measured on this machine (`~/Library/Android/sdk/ndk/29.0.14206865`'s clang for
   `--target=aarch64-linux-android24`, the target `spikes/jni-probe.sh` uses):
   `#define RTLD_LAZY 0x00001`, `#define RTLD_GLOBAL 0x00100`; `dynlib.rs` carries the *frozen*
   target's 1 and 8 because `dynlib.c` is that configuration's, and this file exists only in an
   Android one (the ledger records the pair).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub fn dlopen (file: *const c_char, mode: c_int) -> *mut c_void;
  pub fn dlsym (handle: *mut c_void, name: *const c_char) -> *mut c_void;
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const RTLD_LAZY: c_int = 0x1;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const RTLD_GLOBAL: c_int = 0x100;

/* C 23-24's `<stdio.h>` entries and C 22's `<limits.h>` style: `sprintf` is in the `stdio.rs`
   mirror (lisp.h 34 includes it), but `sscanf` and `BUFSIZ` are not, and this file is the first
   unit that names them (the `xfaces.rs` 540 / `sysdep.rs` 430 in-unit `sscanf` shape).  `BUFSIZ`
   is bionic's `<stdio.h>` value, measured the same way as the two `RTLD_*` above:
   `#define BUFSIZ 1024`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub fn sscanf (s: *const c_char, format: *const c_char, ...) -> c_int;
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const BUFSIZ: c_int = 1024;

/* termhooks.h 353-355: `NOTIFICATION_EVENT`, the member C writes inside `#ifdef HAVE_ANDROID`
   at the end of the `enum event_kind` that starts at termhooks.h 72.  `include/termhooks.rs`
   carries the enum up to
   TOOLKIT_THEME_CHANGED_EVENT (36) and neither conditional member, so the value is C's chain
   with `HAVE_ANDROID` defined and `HAVE_HAIKU` undefined (the second conditional member,
   `NOTIFICATION_CLICKED_EVENT`, would take 37 first if that macro were defined too - recorded in
   the ledger as a mirror gap, the androidmenu.rs use-site shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const NOTIFICATION_EVENT: event_kind = 37;

/* <intprops.h>'s two macros this file's one buffer size and one range test name.  C reaches them
   through lisp.h 42; the frozen `lib/include/intprops.rs` keeps C's own text and is not compiled,
   so the units that need the semantics write the Rust primitive at the use site (the
   `androidmenu.rs` / `editfns.rs` TYPE_MAXIMUM pair, and `lib/strftime.rs` 542's
   INT_STRLEN_BOUND, whose body this one repeats over `size_of::<$t>`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! INT_STRLEN_BOUND {
  ($t:ty) => { ((std::mem::size_of::<$t> () * 8 - 1) * 146 + 484) / 485 + 1 };
}

/* C's `ANDROID_DELETE_LOCAL_REF` (android.h 340-342) expands to a `<jni.h>` vtable read.  The
   mirror records it as an excluded macro whose premise ("`JNINativeInterface` has no mirror")
   predates `include/jni.rs`, which now carries that table (all 230 entries, `spikes/jni-probe.sh`);
   the port defines the expansion under C's own name so the token survives at every use site (the
   androidmenu.rs shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! ANDROID_DELETE_LOCAL_REF {
  ($r:expr) => { ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, $r) };
}

/* termhooks.h 373-412's `enum event_kind kind : 16` shares the mirror's `input_event::bits` unit
   with `part : 16` (PORTING.md 1.3/P-80's member-named shape); the keyboard.rs 4080 / xterm.rs
   1367 per-unit macro, body-identical.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/* Two entry points lisp.h declares and this file's bodies call: `FUNCTIONP` (lisp.h 4799,
   eval.c's) and `eassert`/`error`/`signal_error`/`emacs_abort` (the mirror's).  `FUNCTIONP` is in
   neither lisp.rs nor globals.rs, so it is declared at the use site (the `editfns.rs` 141 shape);
   the ledger records it as a mirror gap.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);` (eval.c defines it).
  pub fn FUNCTIONP (object: Lisp_Object) -> bool;
}

/* C 36-45: the file's selection-support note.  C's text, kept as text (P-86 ①).  */
/* Selection support on Android is confined to copying and pasting of plain text and MIME data
   from the clipboard.  There is no primary selection.  While newer versions of Android are
   supposed to have the necessary interfaces for transferring other kinds of selection data, doing
   so is too complicated, and involves registering ``content providers'' and all kinds of other
   stuff; for this reason, the reference does not support setting the clipboard contents to
   anything other than plain text.  */

/* Structure describing the EmacsClipboard class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_clipboard                            // C 51-61
{
  pub class: jclass,
  pub set_clipboard: jmethodID,
  pub owns_clipboard: jmethodID,
  pub clipboard_exists: jmethodID,
  pub get_clipboard: jmethodID,
  pub make_clipboard: jmethodID,
  pub get_clipboard_targets: jmethodID,
  pub get_clipboard_data: jmethodID,
}

/* Methods associated with the EmacsClipboard class.  C's `static struct` is a tentative
   definition, i.e. zero.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut clipboard_class: android_emacs_clipboard           // C 64
  = android_emacs_clipboard { class: std::ptr::null_mut (),
                              set_clipboard: std::ptr::null_mut (),
                              owns_clipboard: std::ptr::null_mut (),
                              clipboard_exists: std::ptr::null_mut (),
                              get_clipboard: std::ptr::null_mut (),
                              make_clipboard: std::ptr::null_mut (),
                              get_clipboard_targets: std::ptr::null_mut (),
                              get_clipboard_data: std::ptr::null_mut () };

/* Reference to the EmacsClipboard object.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut clipboard: jobject = std::ptr::null_mut ();         // C 67

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_emacs_clipboard ()                     // C 71-116
{
  let mut old: jclass;

  clipboard_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
                                                    c"org/femacs/EmacsClipboard".as_ptr ());
  eassert! (! clipboard_class.class.is_null ());

  old = clipboard_class.class;
  clipboard_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if clipboard_class.class.is_null ()
    { emacs_abort (); }

  /* C 90-95: `FIND_METHOD`, the macro this function defines and `#undef`s at C 115; Rust's
     `macro_rules!` scope ends with the block, so the `#undef` has no counterpart.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      clipboard_class.$c_name
        = ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
                                                          clipboard_class.class,
                                                          $name, $signature);
      eassert! (! clipboard_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (set_clipboard, c"setClipboard".as_ptr (), c"(Ljava/lang/String;)V".as_ptr ());
  FIND_METHOD! (owns_clipboard, c"ownsClipboard".as_ptr (), c"()I".as_ptr ());
  FIND_METHOD! (clipboard_exists, c"clipboardExists".as_ptr (), c"()Z".as_ptr ());
  FIND_METHOD! (get_clipboard, c"getClipboard".as_ptr (), c"()Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (get_clipboard_targets, c"getClipboardTargets".as_ptr (),
                c"()[Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (get_clipboard_data, c"getClipboardData".as_ptr (),
                c"(Ljava/lang/String;)Landroid/content/res/AssetFileDescriptor;".as_ptr ());

  /* C 107-113: the one use that is not a `FIND_METHOD`, and which C writes after the six.  */
  clipboard_class.make_clipboard
    = ((*(*android_java_env)).GetStaticMethodID.unwrap ()) (android_java_env,
                                                           clipboard_class.class,
                                                           c"makeClipboard".as_ptr (),
                                                           c"()Lorg/femacs/EmacsClipboard;"
                                                            .as_ptr ());
  eassert! (! clipboard_class.make_clipboard.is_null ());
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
DEFUN! ("android-clipboard-owner-p", Fandroid_clipboard_owner_p,
       Sandroid_clipboard_owner_p, 0, 0, 0,
       doc: "Return whether or not Emacs owns the clipboard.\nAlternatively, return the symbol `lambda' if that could not be\ndetermined.",   // C 121-147
       (),
       {
  let mut rc: jint;

  if !android_init_gui
    { error (c"Accessing clipboard without display connection".as_ptr ()); }

  block_input ();
  rc = ((*(*android_java_env)).CallIntMethod.unwrap ()) (android_java_env,
                                                         clipboard,
                                                         clipboard_class.owns_clipboard);
  android_exception_check ();
  unblock_input ();

  /* If rc is 0 or 1, then the reference knows whether or not it owns the clipboard.  If rc is
     -1, then it does not.  */

  if rc < 0
    { return Qlambda (); }

  return if rc != 0 { Qt () } else { Qnil () };
});

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
DEFUN! ("android-set-clipboard", Fandroid_set_clipboard,
       Sandroid_set_clipboard, 1, 1, 0,
       doc: "Set the clipboard text to STRING.",                // C 149-175
       (string: Lisp_Object),
       {
  let mut string = string;             // C 157-159 reassigns the parameter
  let mut text: jstring;

  if !android_init_gui
    { error (c"Accessing clipboard without display connection".as_ptr ()); }

  CHECK_STRING (string);
  string = code_convert_string_norecord (string, Qandroid_jni (), true);

  text = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,
                                                          SSDATA (string));
  android_exception_check ();

  ((*(*android_java_env)).CallVoidMethod.unwrap ()) (android_java_env,
                                                     clipboard,
                                                     clipboard_class.set_clipboard,
                                                     text);
  android_exception_check_1 (text);
  ANDROID_DELETE_LOCAL_REF! (text);

  return Qnil ();
});

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
DEFUN! ("android-get-clipboard", Fandroid_get_clipboard,
       Sandroid_get_clipboard, 0, 0, 0,
       doc: "Return the current contents of the clipboard.\nValue is a multibyte string containing decoded clipboard\ntext.\nAlternatively, return nil if the clipboard is empty.",   // C 177-219
       (),
       {
  let mut string: Lisp_Object;
  let mut text: jstring;
  let mut method: jmethodID;
  let mut length: jsize;
  let mut data: *const c_char;

  if !android_init_gui
    { error (c"No Android display connection".as_ptr ()); }

  method = clipboard_class.get_clipboard;
  text
    = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env,
                                                           clipboard,
                                                           method);
  android_exception_check ();

  if text.is_null ()
    { return Qnil (); }

  /* Retrieve a pointer to the raw JNI-encoded bytes of the string.  */
  length = ((*(*android_java_env)).GetStringUTFLength.unwrap ()) (android_java_env,
                                                                  text);
  data = ((*(*android_java_env)).GetStringUTFChars.unwrap ()) (android_java_env, text,
                                                               std::ptr::null_mut ());
  android_exception_check_nonnull (data as *mut c_void, text);

  /* Copy them into a unibyte string for decoding.  */
  string = make_unibyte_string (data, length as ptrdiff_t);
  ((*(*android_java_env)).ReleaseStringUTFChars.unwrap ()) (android_java_env, text,
                                                            data);
  ANDROID_DELETE_LOCAL_REF! (text);

  /* Now decode the resulting string.  */
  return code_convert_string_norecord (string, Qandroid_jni (), false);
});

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
DEFUN! ("android-clipboard-exists-p", Fandroid_clipboard_exists_p,
       Sandroid_clipboard_exists_p, 0, 0, 0,
       doc: "Return whether or not clipboard contents exist.",   // C 221-239
       (),
       {
  let mut rc: jboolean;
  let mut method: jmethodID;

  if !android_init_gui
    { error (c"No Android display connection".as_ptr ()); }

  method = clipboard_class.clipboard_exists;
  rc = ((*(*android_java_env)).CallBooleanMethod.unwrap ()) (android_java_env,
                                                             clipboard,
                                                             method);
  android_exception_check ();

  return if rc != 0 { Qt () } else { Qnil () };
});

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
DEFUN! ("android-browse-url-internal", Fandroid_browse_url_internal,
       Sandroid_browse_url_internal, 1, 2, 0,
       doc: "Open URL in an external application.\n\nURL should be a URL-encoded URL with a scheme specified unless SEND is\nnon-nil.  Signal an error upon failure.\n\nIf SEND is nil, start a program that is able to display the URL, such\nas a web browser.  Otherwise, try to share URL using programs such as\nemail clients.\n\nIf URL is a file URI, convert it into a `content' address accessible to\nother programs.  Files inside the /content or /assets directories cannot\nbe opened through such addresses, which this function does not provide\nfor.  Use `android-browse-url' instead.",   // C 241-271
       (url: Lisp_Object, send: Lisp_Object),
       {
  let mut value: Lisp_Object;

  if !android_init_gui
    { error (c"No Android display connection".as_ptr ()); }

  CHECK_STRING (url);
  value = android_browse_url (url, send);

  /* Signal an error upon failure.  */
  if !NILP (value)
    { signal_error (c"Error browsing URL".as_ptr (), value); }

  return Qnil ();
});

/* MIME clipboard support.  This provides support for reading MIME data (but not text) from the
   clipboard.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
DEFUN! ("android-get-clipboard-targets", Fandroid_get_clipboard_targets,
       Sandroid_get_clipboard_targets, 0, 0, 0,
       doc: "Return a list of data types in the clipboard.\nValue is a list of MIME types as strings, each defining a single extra\ndata type available from the clipboard.",   // C 278-339
       (),
       {
  let mut all_targets: jarray;
  let mut string: jstring;
  let mut method: jmethodID;
  let mut length: size_t;
  let mut i: size_t = 0;              // C 289's `for (i = 0; ...)` initialiser
  let mut data: *const c_char;
  let mut targets: Lisp_Object;
  let mut tem: Lisp_Object;

  if !android_init_gui
    { error (c"No Android display connection".as_ptr ()); }

  targets = Qnil ();
  block_input ();
  method = clipboard_class.get_clipboard_targets;
  all_targets = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env,
                                                                     clipboard, method);
  android_exception_check ();

  /* C 302-303's `goto fail` and C 336-339's `fail:` label: the port writes the jump target as a
     labeled block (PORTING.md 2.2), whose end falls into C's label body.  */
 'fail: {
  if all_targets.is_null ()
    { break 'fail; }

  length = ((*(*android_java_env)).GetArrayLength.unwrap ()) (android_java_env,
                                                              all_targets) as size_t;
  while i < length
    {
      /* Retrieve the MIME type.  */
      string
        = ((*(*android_java_env)).GetObjectArrayElement.unwrap ()) (android_java_env,
                                                                    all_targets,
                                                                    i as jsize);
      android_exception_check_nonnull (string, all_targets);

      /* Cons it onto the list of targets.  */
      data = ((*(*android_java_env)).GetStringUTFChars.unwrap ()) (android_java_env,
                                                                   string,
                                                                   std::ptr::null_mut ());
      android_exception_check_nonnull_1 (data as *mut c_void, string,
                                         all_targets);

      /* Decode the string.  */
      tem = build_unibyte_string (data as *mut c_char);
      tem = code_convert_string_norecord (tem, Qandroid_jni (), false);
      targets = Fcons (tem, targets);

      /* Delete the retrieved data.  */
      ((*(*android_java_env)).ReleaseStringUTFChars.unwrap ()) (android_java_env,
                                                                string, data);
      ANDROID_DELETE_LOCAL_REF! (string);

      i += 1;
    }
  unblock_input ();

  ANDROID_DELETE_LOCAL_REF! (all_targets);
  return Fnreverse (targets);
 }

  /* C 336-339: `fail:`.  */
  unblock_input ();
  return Qnil ();
});

/* Structure describing the AssetFileDescriptor class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_asset_file_descriptor                   // C 343-352
{
  pub class: jclass,
  pub close: jmethodID,
  pub get_length: jmethodID,
  pub get_start_offset: jmethodID,
  pub get_file_descriptor: jmethodID,
  pub get_parcel_file_descriptor: jmethodID,
  pub get_fd: jmethodID,
}

/* Methods associated with the AssetFileDescriptor class.  Zero, as C's tentative definition
   is.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut asset_fd_class: android_asset_file_descriptor       // C 355
  = android_asset_file_descriptor { class: std::ptr::null_mut (),
                                    close: std::ptr::null_mut (),
                                    get_length: std::ptr::null_mut (),
                                    get_start_offset: std::ptr::null_mut (),
                                    get_file_descriptor: std::ptr::null_mut (),
                                    get_parcel_file_descriptor: std::ptr::null_mut (),
                                    get_fd: std::ptr::null_mut () };

/* Initialize virtual function IDs and class pointers in connection with the AssetFileDescriptor
   class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_asset_file_descriptor ()                // C 360-395
{
  let mut old: jclass;

  asset_fd_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
                                                    c"android/content/res/AssetFileDescriptor"
                                                     .as_ptr ());
  eassert! (! asset_fd_class.class.is_null ());

  old = asset_fd_class.class;
  asset_fd_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if asset_fd_class.class.is_null ()
    { emacs_abort (); }

  /* C 380-385: the `FIND_METHOD` this function defines and `#undef`s at C 394.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      asset_fd_class.$c_name
        = ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
                                                          asset_fd_class.class,
                                                          $name, $signature);
      eassert! (! asset_fd_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (close, c"close".as_ptr (), c"()V".as_ptr ());
  FIND_METHOD! (get_length, c"getLength".as_ptr (), c"()J".as_ptr ());
  FIND_METHOD! (get_start_offset, c"getStartOffset".as_ptr (), c"()J".as_ptr ());
  FIND_METHOD! (get_file_descriptor, c"getFileDescriptor".as_ptr (),
                c"()Ljava/io/FileDescriptor;".as_ptr ());
  FIND_METHOD! (get_parcel_file_descriptor, c"getParcelFileDescriptor".as_ptr (),
                c"()Landroid/os/ParcelFileDescriptor;".as_ptr ());
}

/* Free the memory inside PTR, a pointer to a char pointer.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_xfree_inside (ptr: *mut c_void)   // C 399-403
{
  xfree ((*(ptr as *mut *mut c_char)) as *mut c_void);
}

/* Close the referent of, then delete, the local reference to an asset file descriptor referenced
   by AFD.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn close_asset_fd (afd: *mut c_void)         // C 408-418
{
  let mut afd_1: *mut jobject;

  afd_1 = afd as *mut jobject;
  ((*(*android_java_env)).CallVoidMethod.unwrap ()) (android_java_env, *afd_1,
                                                     asset_fd_class.close);
  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);
  ANDROID_DELETE_LOCAL_REF! (*afd_1);
}

/* Return the offset, file descriptor and length of the data contained in the asset file
   descriptor AFD, in *FD, *OFFSET, and *LENGTH.  AFD will not be released if an exception is
   detected; it is the responsibility of the caller to arrange that it be.

   Value is 0 upon success, 1 otherwise.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn extract_fd_offsets (afd: jobject, fd: *mut c_int, offset: *mut jlong,
                              length: *mut jlong) -> c_int      // C 427-528
{
  let mut java_fd: jobject;
  let mut handle: *mut c_void;
  /* C 432-434: `#if __ANDROID_API__ <= 11` guards this static in C.  The port keeps the
     declaration (the arm that reads it is kept as text too), so the gate it would carry is the
     one the ledger records for that `#if`.  */
  static mut jniGetFDFromFileDescriptor:
    Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> c_int> = None;
  /* C 435: the same type with no `#if` in front of it.  */
  static mut AFileDescriptor_getFd:
    Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> c_int> = None;
  let mut method: jmethodID;

  method = asset_fd_class.get_start_offset;
  *offset = ((*(*android_java_env)).CallLongMethod.unwrap ()) (android_java_env,
                                                               afd, method);
  android_exception_check ();
  method = asset_fd_class.get_length;
  *length = ((*(*android_java_env)).CallLongMethod.unwrap ()) (android_java_env,
                                                               afd, method);
  android_exception_check ();

  /* C 447-500: the `#if __ANDROID_API__ <= 11` / `#else` / `#if __ANDROID_API__ <= 30` /
     `#else` chain.  Rust cannot put `#[cfg]` on an `else` clause, so the port keeps C's three
     arms and its own runtime tests and turns each `#if` into the compile-time operand
     `cfg! (feature = …)` of that arm's test: the arms are exclusive at run time as well
     (`android_get_current_api_level () >= __ANDROID_API__`, android.h 152-158), so a
     configuration that switches an arm off produces exactly C's chain for that
     `__ANDROID_API__` (the ledger's Deviations derive all three cases).  */
 'failure: {
  if cfg! (feature = "android_api_le_11") && android_get_current_api_level () <= 11
    {
      /* Load libnativehelper and link to a private interface that is the only means of
	 retrieving the file descriptor from an asset file descriptor on these systems.  */

      if jniGetFDFromFileDescriptor.is_none ()
	{
	  handle = dlopen (c"libnativehelper.so".as_ptr (),
			   RTLD_LAZY | RTLD_GLOBAL);
	  if handle.is_null ()
	    { break 'failure; }
	  jniGetFDFromFileDescriptor
	    = std::mem::transmute (dlsym (handle,
					  c"jniGetFDFromFileDescriptor".as_ptr ()));
	  if jniGetFDFromFileDescriptor.is_none ()
	    { break 'failure; }
	}

      method = asset_fd_class.get_file_descriptor;
      java_fd = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env,
								    afd, method);
      android_exception_check ();
      *fd = (jniGetFDFromFileDescriptor.unwrap ()) (android_java_env, java_fd);
      ANDROID_DELETE_LOCAL_REF! (java_fd);

      if *fd >= 0
	{ return 0; }
    }
  else if cfg! (feature = "android_api_le_30") && android_get_current_api_level () <= 30
    {
      /* Convert this AssetFileDescriptor into a ParcelFileDescriptor, whose getFd method will
	 return its native file descriptor.  */
      method = asset_fd_class.get_parcel_file_descriptor;
      java_fd = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env,
								    afd, method);
      android_exception_check ();

      /* Initialize fd_class if not already complete.  */
      android_init_fd_class (android_java_env);
      *fd = ((*(*android_java_env)).CallIntMethod.unwrap ()) (android_java_env,
							      java_fd,
							      fd_class.get_fd);
      android_exception_check_1 (java_fd);
      ANDROID_DELETE_LOCAL_REF! (java_fd);

      if *fd >= 0
	{ return 0; }
    }
  else
    {
      /* Load libnativehelper (now a public interface) and link to AFileDescriptor_getFd.  */
      if AFileDescriptor_getFd.is_none ()
	{
	  handle = dlopen (c"libnativehelper.so".as_ptr (),
			   RTLD_LAZY | RTLD_GLOBAL);
	  if handle.is_null ()
	    { break 'failure; }
	  AFileDescriptor_getFd
	    = std::mem::transmute (dlsym (handle, c"AFileDescriptor_getFd".as_ptr ()));
	  if AFileDescriptor_getFd.is_none ()
	    { break 'failure; }
	}

      method = asset_fd_class.get_file_descriptor;
      java_fd = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env,
								    afd, method);
      android_exception_check ();
      *fd = (AFileDescriptor_getFd.unwrap ()) (android_java_env, java_fd);
      ANDROID_DELETE_LOCAL_REF! (java_fd);

      if *fd >= 0
	{ return 0; }
    }
 }

  /* C 526-528: `failure: return 1;`.  */
  return 1;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
DEFUN! ("android-get-clipboard-data", Fandroid_get_clipboard_data,
       Sandroid_get_clipboard_data, 1, 1, 0,
       doc: "Return the clipboard data of the given MIME TYPE.\nValue is a unibyte string containing the entire contents of the\nclipboard, after its owner has converted the data to the given\nMIME type.  Value is nil if the conversion fails, or if the data\nis not present.\n\nValue is also nil if the clipboard data consists of a single URL which\ndoes not have any corresponding data.  In that case, use\n`android-get-clipboard' instead.",   // C 530-638
       (r#type: Lisp_Object),
       {
  let mut afd: jobject;
  let mut mime_type: jstring;
  let mut method: jmethodID;
  let mut fd: c_int = 0;              // C 535; C 576 takes its address (PORTING.md 1.5)
  let mut rc: ptrdiff_t = 0;          // C 536; the loop below is C's `while (true)`
  let mut offset: jlong = 0;          // C 536; C 576 takes its address (PORTING.md 1.5)
  let mut length: jlong = 0;          // C 536; C 576 takes its address (PORTING.md 1.5)
  let mut r#ref: specpdl_ref;
  let mut buffer: *mut c_char;
  let mut start: *mut c_char;

  if !android_init_gui
    { error (c"No Android display connection".as_ptr ()); }

  CHECK_STRING (r#type);

  /* Convert TYPE into a Java string.  */
  block_input ();
  mime_type = android_build_string (r#type, std::ptr::null_mut::<c_void> ());
  method = clipboard_class.get_clipboard_data;
  afd = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env,
                                                             clipboard, method,
                                                             mime_type);
  android_exception_check_1 (mime_type);
  ANDROID_DELETE_LOCAL_REF! (mime_type);

  /* C 567-568's `goto fail` and C 635-638's `fail:` label: the port writes the jump target as a
     labeled block (PORTING.md 2.2), whose end falls into C's label body.  */
 'fail: {
  if afd.is_null ()
    { break 'fail; }

  /* Extract the file descriptor from the AssetFileDescriptor object.  */
  r#ref = SPECPDL_INDEX ();
  record_unwind_protect_ptr (Some (close_asset_fd), &raw mut afd as *mut c_void);

  if extract_fd_offsets (afd, &raw mut fd, &raw mut offset, &raw mut length) != 0
    {
      unblock_input ();
      return unbind_to (r#ref, Qnil ());
    }
  unblock_input ();

  /* Now begin reading from fd.  */

  if length != -1
    {
      buffer = xmalloc (MIN! (length, PTRDIFF_MAX as jlong) as usize) as *mut c_char;
      record_unwind_protect_ptr (Some (xfree), buffer as *mut c_void);

      rc = emacs_read_quit (fd, buffer as *mut c_void,
			    MIN! (length, PTRDIFF_MAX as jlong) as ptrdiff_t);

      /* Return nil upon an IO problem.  */
      if rc < 0
	{ return unbind_to (r#ref, Qnil ()); }

      /* Return the data as a unibyte string.  */
      return unbind_to (r#ref, make_unibyte_string (buffer, rc));
    }

  /* Otherwise, read BUFSIZ bytes at a time.  */
  buffer = xmalloc (BUFSIZ as usize) as *mut c_char;
  length = 0;
  start = buffer;

  record_unwind_protect_ptr (Some (android_xfree_inside), &raw mut buffer as *mut c_void);

  /* Seek to the start of the data.  */

  if offset != 0
    {
      if lseek (fd, offset, SEEK_SET) < 0
	{ return unbind_to (r#ref, Qnil ()); }
    }

  while true
    {
      rc = emacs_read_quit (fd, start as *mut c_void, BUFSIZ as ptrdiff_t);

      /* C 619-620: `ckd_add (&length, length, rc) || PTRDIFF_MAX - length < BUFSIZ`.  The
	 macro is gnulib's `<stdckdint.h>` pair (wrapped sum plus overflow flag, lisp.h 27's
	 include) and the port writes its two halves out at the site, P-15's spelling
	 (`lib/timespec-add.rs` 36-38, `lib/mktime.rs` 36).  */
      let (ckd_length, ckd_overflow) = length.overflowing_add (rc as jlong);
      length = ckd_length;
      if ckd_overflow
	  || (PTRDIFF_MAX - length as isize) < BUFSIZ as isize
	{ memory_full_up (); }

      if rc < 0
	{ return unbind_to (r#ref, Qnil ()); }

      if rc < BUFSIZ as ptrdiff_t
	{ break; }

      buffer = xrealloc (buffer as *mut c_void,
			 (length + BUFSIZ as jlong) as usize) as *mut c_char;
      start = buffer.offset (length as isize);
    }

  return unbind_to (r#ref, make_unibyte_string (buffer, rc));
 }

  /* C 635-638: `fail:`.  */
  unblock_input ();
  return Qnil ();
});

/* Desktop notifications.  `android-desktop-notify' implements a facsimile of
   `notifications-notify'.  */

/* Structure describing the EmacsDesktopNotification class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_desktop_notification             // C 647-652
{
  pub class: jclass,
  pub init: jmethodID,
  pub display: jmethodID,
}

/* Methods provided by the EmacsDesktopNotification class.  Zero, as C's tentative definition
   is.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut notification_class: android_emacs_desktop_notification   // C 655
  = android_emacs_desktop_notification { class: std::ptr::null_mut (),
                                        init: std::ptr::null_mut (),
                                        display: std::ptr::null_mut () };

/* Hash table pairing notification identifiers with callbacks.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut notification_table: Lisp_Object = lisp_h_Qnil ();   // C 658 (tentative, i.e. Qnil)

/* Initialize virtual function IDs and class pointers tied to the EmacsDesktopNotification
   class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_emacs_desktop_notification ()          // C 663-695
{
  let mut old: jclass;

  notification_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
                                                    c"org/femacs/EmacsDesktopNotification"
                                                     .as_ptr ());
  eassert! (! notification_class.class.is_null ());

  old = notification_class.class;
  notification_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if notification_class.class.is_null ()
    { emacs_abort (); }

  /* C 682-687: the `FIND_METHOD` this function defines and `#undef`s at C 694.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      notification_class.$c_name
        = ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
                                                          notification_class.class,
                                                          $name, $signature);
      eassert! (! notification_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (init, c"<init>".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;II[Ljava/lang/String;[Ljava/lang/String;J)V".as_ptr ());
  FIND_METHOD! (display, c"display".as_ptr (), c"()V".as_ptr ());
}

/* Return the numeric resource ID designating the icon within the ``android.R.drawable'' package
   by the supplied NAME.

   If no icon is found, return that of ``android.R.drawable.ic_dialog_alert''.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_locate_icon (name: *const c_char) -> jint  // C 703-730
{
  let mut drawable: jclass;
  let mut field: jfieldID;
  let mut rc: jint;

  if android_verify_jni_string (name) != 0
    /* If NAME isn't valid, return the default value.  */
    { return 17301543; }              /* android.R.drawable.ic_dialog_alert.  */

  drawable = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
                                                           c"android/R$drawable".as_ptr ());
  android_exception_check ();

  field = ((*(*android_java_env)).GetStaticFieldID.unwrap ()) (android_java_env,
                                                               drawable, name, c"I".as_ptr ());
  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);

  if field.is_null ()
    { rc = 17301543; }                /* android.R.drawable.ic_dialog_alert.  */
  else
    { rc = ((*(*android_java_env)).GetStaticIntField.unwrap ()) (android_java_env,
                                                                 drawable, field); }

  ANDROID_DELETE_LOCAL_REF! (drawable);
  return rc;
}

/* Display a desktop notification with the provided TITLE, BODY, REPLACES_ID, GROUP, ICON,
   URGENCY, ACTIONS, TIMEOUT, RESIDENT, ACTION_CB and CLOSE_CB.  Return an identifier for the
   resulting notification.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_notifications_notify_1 (title: Lisp_Object, body: Lisp_Object,
                                          replaces_id: Lisp_Object,
                                          group: Lisp_Object, icon: Lisp_Object,
                                          urgency: Lisp_Object, actions: Lisp_Object,
                                          timeout: Lisp_Object, resident: Lisp_Object,
                                          action_cb: Lisp_Object, close_cb: Lisp_Object)
  -> intmax_t                                                 // C 737-929
{
  let mut actions = actions;          // C 880 reassigns the parameter
  static mut counter: intmax_t = 0;   // C 752 (tentative, i.e. zero)
  let mut id: intmax_t = 0;         // C 753; C 774 and C 807 write it through its address (PORTING.md 1.5)
  let mut title1: jstring; let mut body1: jstring;
  let mut group1: jstring; let mut identifier1: jstring;
  let mut r#type: jint; let mut icon1: jint;
  let mut notification: jobject;
  let mut action_keys: jobjectArray; let mut action_titles: jobjectArray;
  /* C 742-745: `char identifier[INT_STRLEN_BOUND (int) + INT_STRLEN_BOUND (long int) +
     INT_STRLEN_BOUND (intmax_t) + sizeof "…"]`.  `sizeof "…"` is 4 (`"%d.%ld.%jd"` and its
     NUL), and the three instantiations are the macro's own body over C's three types.  */
  let mut identifier: [c_char;
                       INT_STRLEN_BOUND! (c_int) + INT_STRLEN_BOUND! (c_long)
                       + INT_STRLEN_BOUND! (intmax_t) + 4]
    = [0; INT_STRLEN_BOUND! (c_int) + INT_STRLEN_BOUND! (c_long)
       + INT_STRLEN_BOUND! (intmax_t) + 4];
  let mut boot_time: timespec = std::mem::zeroed ();  // C 746; C 847 writes tv_sec first
  let mut key: Lisp_Object; let mut value: Lisp_Object; let mut tem: Lisp_Object;
  let mut nitems: jint; let mut i: jint = 0;   // C 756's `for (i = 0; ...)` initialiser
  let mut item: jstring;
  let mut length: Lisp_Object;
  let mut timeout_val: jlong;

  if EQ (urgency, Qlow ())
    { r#type = 2; }                   /* IMPORTANCE_LOW */
  else if EQ (urgency, Qnormal ())
    { r#type = 3; }                   /* IMPORTANCE_DEFAULT */
  else if EQ (urgency, Qcritical ())
    { r#type = 4; }                   /* IMPORTANCE_HIGH */
  else
    { signal_error (c"Invalid notification importance given".as_ptr (), urgency); }

  /* Decode the timeout.  */

  timeout_val = 0;

  if !NILP (timeout)
    {
      CHECK_INTEGER (timeout);

      if !integer_to_intmax (timeout, &raw mut id)
	  || id > TYPE_MAXIMUM! (jlong)
	  || id < TYPE_MINIMUM! (jlong)
	{ signal_error (c"Invalid timeout".as_ptr (), timeout); }

      if id > 0
	{ timeout_val = id; }
    }

  nitems = 0;

  /* If ACTIONS is provided, split it into two arrays of Java strings holding keys and
     titles.  */

  if !NILP (actions)
    {
      /* Count the number of items to be inserted.  */

      length = Flength (actions);
      if !TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, length, INT_MAX as EMACS_INT)
	{ error (c"Action list too long".as_ptr ()); }
      nitems = XFIXNAT (length) as jint;
      if (nitems & 1) != 0
	{ error (c"Length of action list is invalid".as_ptr ()); }
      nitems /= 2;

      /* Verify that the list consists exclusively of strings.  */
      tem = actions;
      FOR_EACH_TAIL! (tem, { CHECK_STRING (XCAR (tem)); });
    }

  if NILP (replaces_id)
    {
      /* Generate a new identifier.  C 811: `ckd_add (&counter, counter, 1)`, the same P-15
	 pair (the flag is discarded, as C discards the macro's value).  */
      let (ckd_counter, _ckd_overflow) = counter.overflowing_add (1);
      counter = ckd_counter;
      id = counter;
    }
  else
    {
      CHECK_INTEGER (replaces_id);
      if !integer_to_intmax (replaces_id, &raw mut id)
	{ id = -1; }                  /* Overflow.  */
    }

  /* Locate the integer ID linked to ICON.  */
  icon1 = android_locate_icon (SSDATA (icon));

  /* Generate a unique identifier for this notification.  Because Android persists
     notifications past system shutdown, also include the boot time within IDENTIFIER.  Scale it
     down to avoid being perturbed by minor instabilities in the returned boot time, however.  */

  boot_time.tv_sec = 0;
  get_boot_time (&raw mut boot_time);
  sprintf (&raw mut identifier as *mut c_char, c"%d.%ld.%jd".as_ptr (),
	   getpid () as c_int, (boot_time.tv_sec / 2) as c_long, id);

  /* Encode all strings into their Java counterparts.  */
  title1 = android_build_string (title, std::ptr::null_mut::<c_void> ());
  body1  = android_build_string (body, title1, std::ptr::null_mut::<c_void> ());
  group1 = android_build_string (group, body1, title1, std::ptr::null_mut::<c_void> ());
  identifier1
    = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, &raw mut identifier as *mut c_char);
  android_exception_check_3 (title1, body1, group1);

  /* Create the arrays for action identifiers and titles if provided.  */

  if nitems != 0
    {
      action_keys = ((*(*android_java_env)).NewObjectArray.unwrap ()) (android_java_env,
								       nitems,
								       java_string_class,
								       std::ptr::null_mut ());
      android_exception_check_4 (title as jobject, body1, group1, identifier1);
      action_titles = ((*(*android_java_env)).NewObjectArray.unwrap ()) (android_java_env,
									 nitems,
									 java_string_class,
									 std::ptr::null_mut ());
      android_exception_check_5 (title as jobject, body1, group1, identifier1,
				 action_keys);

      while i < nitems
	{
	  key = XCAR (actions);
	  value = XCAR (XCDR (actions));
	  actions = XCDR (XCDR (actions));

	  /* Create a string for this action.  */
	  item = android_build_string (key, body1, group1, identifier1,
				       action_keys, action_titles,
				       std::ptr::null_mut::<c_void> ());
	  ((*(*android_java_env)).SetObjectArrayElement.unwrap ()) (android_java_env,
								    action_keys, i,
								    item);
	  ANDROID_DELETE_LOCAL_REF! (item);

	  /* Create a string for this title.  */
	  item = android_build_string (value, body1, group1, identifier1,
				       action_keys, action_titles,
				       std::ptr::null_mut::<c_void> ());
	  ((*(*android_java_env)).SetObjectArrayElement.unwrap ()) (android_java_env,
								    action_titles, i,
								    item);
	  ANDROID_DELETE_LOCAL_REF! (item);

	  i += 1;
	}
    }
  else
    {
      action_keys = std::ptr::null_mut ();
      action_titles = std::ptr::null_mut ();
    }

  /* Create the notification.  */
  notification
    = ((*(*android_java_env)).NewObject.unwrap ()) (android_java_env,
						    notification_class.class,
						    notification_class.init,
						    title1, body1, group1,
						    identifier1, icon1, r#type,
						    action_keys, action_titles,
						    timeout_val);
  android_exception_check_6 (title1, body1, group1, identifier1,
			     action_titles, action_keys);

  /* Delete unused local references.  */
  ANDROID_DELETE_LOCAL_REF! (title1);
  ANDROID_DELETE_LOCAL_REF! (body1);
  ANDROID_DELETE_LOCAL_REF! (group1);
  ANDROID_DELETE_LOCAL_REF! (identifier1);
  ANDROID_DELETE_LOCAL_REF! (action_keys);
  ANDROID_DELETE_LOCAL_REF! (action_titles);

  /* Display the notification.  */
  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ()) (android_java_env,
							       notification,
							       notification_class.class,
							       notification_class.display);
  android_exception_check_1 (notification);
  ANDROID_DELETE_LOCAL_REF! (notification);

  /* If callbacks are provided, save them into notification_table.  */

  if !NILP (action_cb) || !NILP (close_cb) || !NILP (resident)
    {
      Fputhash (build_string (&raw const identifier as *const c_char),
		list3 (action_cb, close_cb, resident),
		notification_table);
    }

  /* Return the ID.  */
  return id;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
DEFUN! ("android-notifications-notify", Fandroid_notifications_notify,
       Sandroid_notifications_notify, 0, MANY, 0, doc:
       "Display a desktop notification.\nARGS must contain keywords followed by values.  Each of the following\nkeywords is understood:\n\n  :title\tThe notification title.\n  :body\t\tThe notification body.\n  :replaces-id\tThe ID of a previous notification to supersede.\n  :group\tThe notification group, or nil.\n  :urgency\tOne of the symbols `low', `normal' or `critical',\n\t\tdefining the importance of the notification group.\n  :icon\t\tThe name of a drawable resource to display as the\n\t\tnotification's icon.\n  :actions\tA list of actions of the form:\n\t\t  (KEY TITLE KEY TITLE ...)\n\t\twhere KEY and TITLE are both strings.\n\t\tThe action for which CALLBACK is called when the\n\t\tnotification itself is selected is named \"default\",\n\t\tits existence is implied, and its TITLE is ignored.\n\t\tNo more than three actions defined here will be\n\t\tdisplayed, not counting any with \"default\" as its\n\t\tkey.\n  :timeout\tNumber of milliseconds from the display of the\n\t\tnotification at which it will be automatically\n\t\tdismissed, or a value of zero or smaller if it\n\t\tis to remain until user action is taken to dismiss\n\t\tit.\n  :resident     When set the notification will not be automatically\n\t\tdismissed when it or an action is selected.\n  :on-action\tFunction to call when an action is invoked.\n\t\tThe notification id and the key of the action are\n\t\tprovided as arguments to the function.\n  :on-close\tFunction to call if the notification is dismissed,\n\t\twith the notification id and the symbol `undefined'\n\t\tfor arguments.\n\nThe notification group and timeout are ignored on Android 7.1 and\nearlier versions of Android.  On more recent versions, the group\nidentifies a category that will be displayed in the system Settings\nmenu, and the urgency provided always extends to affect all\nnotifications displayed within that category, though it may be ignored\nif higher than any previously-specified urgency or if the user have\nalready configured a different urgency for this category from Settings.\nIf the group is not provided, it defaults to the string \"Desktop\nNotifications\" with the urgency suffixed.\n\nEach caller should strive to provide one unchanging combination of\nnotification group and urgency for each kind of notification it sends,\ninasmuch as the system may, subject to user configuration, disregard\nthe urgency specified within a notification, should it not be the\nfirst notification sent to its notification group.\n\nThe provided icon should be the name of a \"drawable resource\" present\nwithin the \"android.R.drawable\" class designating an icon with a\ntransparent background.  Should no icon be provided (or the icon is\nabsent from this system), it defaults to \"ic_dialog_alert\".\n\nActions specified with :actions cannot be displayed on Android 4.0 and\nearlier versions of the system.\n\nWhen the system is running Android 13 or later, notifications sent\nwill be silently disregarded unless permission to display\nnotifications is expressly granted from the \"App Info\" settings panel\ncorresponding to Emacs.\n\nA title and body must be supplied.  Value is an integer (fixnum or\nbignum) uniquely designating the notification displayed, which may\nsubsequently be specified as the `:replaces-id' of another call to\nthis function.\n\nusage: (android-notifications-notify &rest ARGS)",   // C 931-1082
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut title: Lisp_Object; let mut body: Lisp_Object;
  let mut replaces_id: Lisp_Object; let mut group: Lisp_Object;
  let mut urgency: Lisp_Object; let mut timeout: Lisp_Object;
  let mut resident: Lisp_Object;
  let mut icon: Lisp_Object;
  let mut key: Lisp_Object; let mut value: Lisp_Object;
  let mut actions: Lisp_Object; let mut action_cb: Lisp_Object;
  let mut close_cb: Lisp_Object;
  let mut i: ptrdiff_t = 0;           // C 1010's `for (i = 0; ...)` initialiser
  AUTO_STRING! (default_icon, c"ic_dialog_alert".as_ptr ());

  if !android_init_gui
    { error (c"No Android display connection".as_ptr ()); }

  /* Clear each variable above.  */
  title = Qnil (); body = Qnil (); replaces_id = Qnil (); group = Qnil ();
  icon = Qnil (); urgency = Qnil (); actions = Qnil ();
  timeout = Qnil (); resident = Qnil (); action_cb = Qnil (); close_cb = Qnil ();

  /* If NARGS is odd, error.  */

  if (nargs & 1) != 0
    { error (c"Odd number of arguments in call to `android-notifications-notify'".as_ptr ()); }

  /* Next, iterate through ARGS, searching for arguments.  */

  while i < nargs
    {
      key = *args.offset (i);
      value = *args.offset (i + 1);

      if EQ (key, QCtitle ())
	{ title = value; }
      else if EQ (key, QCbody ())
	{ body = value; }
      else if EQ (key, QCreplaces_id ())
	{ replaces_id = value; }
      else if EQ (key, QCgroup ())
	{ group = value; }
      else if EQ (key, QCurgency ())
	{ urgency = value; }
      else if EQ (key, QCicon ())
	{ icon = value; }
      else if EQ (key, QCactions ())
	{ actions = value; }
      else if EQ (key, QCtimeout ())
	{ timeout = value; }
      else if EQ (key, QCresident ())
	{ resident = value; }
      else if EQ (key, QCon_action ())
	{ action_cb = value; }
      else if EQ (key, QCon_close ())
	{ close_cb = value; }

      i += 2;
    }

  /* Demand at least TITLE and BODY be present.  */

  if NILP (title) || NILP (body)
    { error (c"Title or body not provided".as_ptr ()); }

  /* Now check the type and possibly expand each non-nil argument.  */

  CHECK_STRING (title);
  CHECK_STRING (body);

  if NILP (urgency)
    { urgency = Qlow (); }

  if NILP (group)
    {
      AUTO_STRING! (format, c"Desktop Notifications (%s importance)".as_ptr ());
      group = CALLN! (Fformat, format, urgency);
    }

  if NILP (icon)
    { icon = default_icon; }
  else
    { CHECK_STRING (icon); }

  return make_int (android_notifications_notify_1 (title, body, replaces_id,
						   group, icon, urgency,
						   actions, timeout, resident,
						   action_cb, close_cb));
});

/* Run callbacks in response to a notification being deleted.  Save any input generated for the
   keyboard within *IE.  EVENT should be the notification deletion event.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_notification_deleted (event: *mut android_notification_event,
                                                       ie: *mut input_event)   // C 1088-1108
{
  let mut item: Lisp_Object;
  let mut tag: Lisp_Object;
  let mut id: intmax_t = 0;           // C 1093; C 1128 writes it through its address (PORTING.md 1.5)

  tag  = build_string ((*event).tag);
  item = Fgethash (tag, notification_table, Qnil ());

  if !NILP (item)
    { Fremhash (tag, notification_table); }

  if CONSP (item) && FUNCTIONP (XCAR (XCDR (item)))
      && sscanf ((*event).tag, c"%*d.%*ld.%jd".as_ptr (), &raw mut id) > 0
    {
      input_event_bits! (kind, ie, NOTIFICATION_EVENT);
      (*ie).arg = list3 (XCAR (XCDR (item)), make_int (id),
			 Qundefined ());
    }
}

/* Run callbacks in response to one of a notification's actions being invoked, saving any input
   generated for the keyboard within *IE.  EVENT should be the notification deletion event, and
   ACTION the action key.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_notification_action (event: *mut android_notification_event,
                                                      ie: *mut input_event,
                                                      action: Lisp_Object)   // C 1116-1156
{
  let mut item: Lisp_Object;
  let mut tag: Lisp_Object;
  let mut id: intmax_t = 0;           // C 1121; C 1128 writes it through its address (PORTING.md 1.5)
  let mut tag_object: jstring;
  let mut method: jmethodID;

  tag  = build_string ((*event).tag);
  item = Fgethash (tag, notification_table, Qnil ());

  if CONSP (item) && FUNCTIONP (XCAR (item))
      && sscanf ((*event).tag, c"%*d.%*ld.%jd".as_ptr (), &raw mut id) > 0
    {
      input_event_bits! (kind, ie, NOTIFICATION_EVENT);
      (*ie).arg = list3 (XCAR (item), make_int (id), action);
    }

  /* Test whether ITEM is resident.  Non-resident notifications must be removed when
     activated.  */

  if !CONSP (item) || NILP (XCAR (XCDR (XCDR (item))))
    {
      method = service_class.cancel_notification;
      tag_object
	= ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,
							   (*event).tag);
      android_exception_check ();

      ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ()) (android_java_env,
								   emacs_service,
								   service_class.class,
								   method, tag_object);
      android_exception_check_1 (tag_object);
      ANDROID_DELETE_LOCAL_REF! (tag_object);

      /* Remove the notification from the callback table.  */
      if !NILP (item)
	{ Fremhash (tag, notification_table); }
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn init_androidselect ()                           // C 1160-1187
{
  let mut tem: jobject;
  let mut make_clipboard: jmethodID;

  if !android_init_gui
    { return; }

  android_init_emacs_clipboard ();
  android_init_asset_file_descriptor ();
  android_init_emacs_desktop_notification ();

  make_clipboard = clipboard_class.make_clipboard;
  tem
    = ((*(*android_java_env)).CallStaticObjectMethod.unwrap ()) (android_java_env,
								 clipboard_class.class,
								 make_clipboard);
  if tem.is_null ()
    { emacs_abort (); }

  clipboard = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, tem);

  if clipboard.is_null ()
    { emacs_abort (); }

  ANDROID_DELETE_LOCAL_REF! (tem);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn syms_of_androidselect ()                        // C 1189-1220
{
  DEFSYM! (QCtitle, ":title");
  DEFSYM! (QCbody, ":body");
  DEFSYM! (QCreplaces_id, ":replaces-id");
  DEFSYM! (QCgroup, ":group");
  DEFSYM! (QCurgency, ":urgency");
  DEFSYM! (QCicon, ":icon");
  DEFSYM! (QCactions, ":actions");
  DEFSYM! (QCtimeout, ":timeout");
  DEFSYM! (QCresident, ":resident");
  DEFSYM! (QCon_action, ":on-action");
  DEFSYM! (QCon_close, ":on-close");

  DEFSYM! (Qlow, "low");
  DEFSYM! (Qnormal, "normal");
  DEFSYM! (Qcritical, "critical");

  defsubr (&raw mut Sandroid_clipboard_owner_p);
  defsubr (&raw mut Sandroid_set_clipboard);
  defsubr (&raw mut Sandroid_get_clipboard);
  defsubr (&raw mut Sandroid_clipboard_exists_p);
  defsubr (&raw mut Sandroid_browse_url_internal);
  defsubr (&raw mut Sandroid_get_clipboard_targets);
  defsubr (&raw mut Sandroid_get_clipboard_data);

  defsubr (&raw mut Sandroid_notifications_notify);

  notification_table = CALLN! (Fmake_hash_table, QCtest (), Qequal ());
  staticpro (&raw mut notification_table);
}
