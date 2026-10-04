// femacs: port of src/android.h @ fd7afced403.

// Header mirror (PORTING.md 0.1): declarations only.  `android.h` is the Android backend's
// initialisation header - `android.c`/`androidvfs.c`/`androidfns.c` implement it and the rest of
// the C core calls it - and `ANDROID_STUBIFY` is undefined in the frozen macro table, so both
// `#ifndef ANDROID_STUBIFY` regions (C 29-43 and C 47-267 and C 272-362) are live.  The dead
// regions are `THREADS_ENABLED`'s one line (C 323-325, `extern JavaVM *android_jvm`) and the
// `__i386__` arm of `JNI_STACK_ALIGNMENT_PROLOGUE` (C 349-357); both are recorded in
// docs/ports/include-android.md (PORTING.md P-08).
//
// `<jni.h>` and `<android/bitmap.h>` (C 30, C 39) are the NDK's own headers, not part of the
// reference tree, so they have no mirror and the names this header uses are declared here
// (PORTING.md P-49/P-70, the `<kernel/OS.h>`/`<gmp.h>` class).  Two of the six macros have no
// Rust text at all and own a row in `docs/ports/excluded-macros.txt`: `NATIVE_NAME` pastes an
// identifier (`##`, the NUMNAME class) and `ANDROID_DELETE_LOCAL_REF` reads a `<jni.h>` vtable
// entry the port cannot model (both are described in the ledger).

/* On Android, Emacs is built as a shared library loaded from Java
   using the Java Native Interface.  Emacs's `main' function is
   renamed `android_emacs_init', and runs with some modifications
   inside a separate thread, communicating with the Java code through
   a table of function pointers.  */

include!("androidgui.rs");                     // C 41: `#include "androidgui.h"`
include!("lisp.rs");                           // C 42: `#include "lisp.h"`
include!("../../lib/include/pwd.rs");          // C 30: `#include <pwd.h>`
include!("../../lib/include/sys_stat.in.rs");  // C 33: `#include <sys/stat.h>`
include!("../../lib/include/sys_select.rs");   // C 34: `#include <sys/select.h>`
include!("../../lib/include/dirent.rs");       // C 36: `#include <dirent.h>`
include!("../../lib/include/stdio.rs");        // C 37: `#include <stdio.h>`
include!("../../lib/include/unistd.in.rs");    // `ssize_t` (the <sys/stat.h>/<sys/select.h> chain)
include!("../../lib/include/time.in.rs");      // `struct timespec` (the `<sys/select.h>` chain)

// C 26: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _ANDROID_H_: bool = true;

// C 30's `#include <jni.h>`: the NDK's header now has its own mirror (`include/jni.rs`, the
// measured ABI with all 233 `JNINativeInterface` entries), so this file includes it instead of
// declaring the five names itself - one definition point per name (PORTING.md 0.1-3/P-70); the
// names this header's live text uses are `JNIEnv`, `jobject`, `jclass`, `jmethodID` and
// `jstring`, and `_jmethodID` stays opaque there.
include!("jni.rs");

/* `<android/bitmap.h>` (C 39), which `android_lock_bitmap` names only through a pointer, so it
   stays opaque (PORTING.md P-70).  */
pub struct AndroidBitmapInfo;

/* C 164: `struct android_vdir;` - C's own forward declaration (the directory-listing emulation
   defines it in androidvfs.c), which the P-70 stand-in spells the same way.  */
pub struct android_vdir;

/* Structure describing the android.os.ParcelFileDescriptor class used
   to wrap file descriptors sent over IPC.  */
// C 58-64.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_parcel_file_descriptor_class
{
  pub class: jclass,
  pub close: jmethodID,
  pub get_fd: jmethodID,
  pub detach_fd: jmethodID,
}

/* External asset manager interface.  */
// C 170-177.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_fd_or_asset
{
  /* The file descriptor.  */
  pub fd: c_int,

  /* The asset.  If set, FD is not a real file descriptor.  */
  pub asset: *mut c_void,
}

// C 196-237: the battery state.  C's comments on every member record the units.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_battery_state
{
  /* Battery charge level in integer percentage.  */
  pub capacity: intmax_t,

  /* Battery charge level in microampere-hours.  */
  pub charge_counter: intmax_t,

  /* Battery current in microampere-hours.  */
  pub current_average: intmax_t,

  /* Instantaneous battery current in microampere-hours.  */
  pub current_now: intmax_t,

  /* Estimate as to the amount of time remaining until the battery is
     charged, in milliseconds.  */
  pub remaining: intmax_t,

  /* Battery status.  The value is either:

       2, if the battery is charging.
       3, if the battery is discharging.
       5, if the battery is full.
       4, if the battery is not full or discharging,
          but is not charging either.
       1, if the battery state is unknown.  */
  pub status: c_int,

  /* The power source of the battery.  Value is:

       0, if on battery power.
       1, for line power.
       8, for dock power.
       2, for USB power.
       4, for wireless power.  */
  pub plugged: c_int,

  /* The temperature of the battery in 10 * degrees centigrade.  */
  pub temperature: c_int,
}

// C 275-319: the Java-side service table.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_emacs_service
{
  pub class: jclass,
  pub fill_rectangle: jmethodID,
  pub fill_polygon: jmethodID,
  pub draw_rectangle: jmethodID,
  pub draw_line: jmethodID,
  pub draw_point: jmethodID,
  pub ring_bell: jmethodID,
  pub query_tree: jmethodID,
  pub get_screen_width: jmethodID,
  pub get_screen_height: jmethodID,
  pub detect_mouse: jmethodID,
  pub detect_keyboard: jmethodID,
  pub name_keysym: jmethodID,
  pub browse_url: jmethodID,
  pub restart_emacs: jmethodID,
  pub update_ic: jmethodID,
  pub reset_ic: jmethodID,
  pub open_content_uri: jmethodID,
  pub check_content_uri: jmethodID,
  pub query_battery: jmethodID,
  pub update_extracted_text: jmethodID,
  pub update_cursor_anchor_info: jmethodID,
  pub get_document_authorities: jmethodID,
  pub request_directory_access: jmethodID,
  pub get_document_trees: jmethodID,
  pub document_id_from_name: jmethodID,
  pub get_tree_uri: jmethodID,
  pub stat_document: jmethodID,
  pub access_document: jmethodID,
  pub open_document_directory: jmethodID,
  pub read_directory_entry: jmethodID,
  pub open_document: jmethodID,
  pub create_document: jmethodID,
  pub create_directory: jmethodID,
  pub delete_document: jmethodID,
  pub rename_document: jmethodID,
  pub move_document: jmethodID,
  pub valid_authority: jmethodID,
  pub external_storage_available: jmethodID,
  pub request_storage_access: jmethodID,
  pub cancel_notification: jmethodID,
  pub relinquish_uri_rights: jmethodID,
}

// C 106: `static_assert (sizeof (android_handle) == sizeof (jobject));` - the handle and the JNI
// handle must stay the same width (PORTING.md P-60 keeps file-level assertions alive).
const _: () = assert! (std::mem::size_of::<android_handle> () == std::mem::size_of::<jobject> ());

// C 107: `#define android_resolve_handle(handle) ((jobject) (handle))`.  A cast is not a value, so
// the port spells it as the inline function `process.rs`'s `ChannelMask` uses for the same shape.
#[inline(always)]
pub fn android_resolve_handle (handle: android_handle) -> jobject
{
  handle as jobject
}

// C 154-159: `#define android_get_current_api_level() ({ int value; value =
// (android_get_current_api_level) (); eassume (value >= __ANDROID_API__); value; })`.  The
// parenthesised call defeats the macro in C; in Rust a macro needs `!`, so the body's plain
// `android_get_current_api_level ()` is the function below, and `__ANDROID_API__` stays C's
// name: the NDK's toolchain defines it, so it is resolved where a ported call site expands this
// (the frozen target never does - the macro stands inside `android.c`'s Android-only regions).
macro_rules! android_get_current_api_level {
  () => { {
      let value: c_int = android_get_current_api_level ();
      eassume! (value >= __ANDROID_API__);
      value
    } };
}

// C 340-342: `#define ANDROID_DELETE_LOCAL_REF(ref) ((*android_java_env)->DeleteLocalRef
// (android_java_env, ref))`.  No Rust text: it reads a `<jni.h>` vtable entry the port cannot
// model (`JNINativeInterface` has no mirror), so the name owns a row in
// docs/ports/excluded-macros.txt and the ledger records the decision.

// C 344: `#define NATIVE_NAME(name) Java_org_gnu_emacs_EmacsNative_##name` pastes the argument
// into the exported JNI symbol name.  Rust cannot paste identifiers, so the name owns a row in
// docs/ports/excluded-macros.txt; the port writes the symbol at each definition site, where the
// C name survives as `#[unsafe(export_name = "Java_org_gnu_emacs_EmacsNative_<name>")]`.

// C 352-359: the `__i386__` arm is dead on this target (C 349-357) and is recorded in the
// ledger; the live arm is C's `((void) 0)`, and the macro is used as a statement.
macro_rules! JNI_STACK_ALIGNMENT_PROLOGUE {
  () => { { let _ = 0; } };
}

/* The 85 declarations C spells `extern` (13 of them the file-scope objects).  They sit in two
   glob-imported modules: `android.c`'s own port defines most of the functions and all of the
   objects, and a same-named declaration at this scope would be `E0428` (PORTING.md 0.1-10/P-17/
   P-21, the `process_objs`/`charset_decls` shape).  `extract-abi.py` inventories the 13 objects
   but not the prototypes (its `functions` list is definitions only), so the prototype block is
   the part of this mirror no gate compares; it was checked name-for-name against the reference
   (85 of 85).  */
pub mod android_objs {
  use super::*;

  unsafe extern "C" {
    pub static mut android_init_gui: bool;
    pub static mut android_cache_dir: *mut c_char;
    pub static mut fd_class: android_parcel_file_descriptor_class;
    pub static mut android_pixel_density_x: f64;
    pub static mut android_pixel_density_y: f64;
    pub static mut android_scaled_pixel_density: f64;
    pub static mut android_ui_mode: c_int;
    pub static mut event_serial: c_uint;
    pub static mut android_java_env: *mut JNIEnv;
    pub static mut java_string_class: jclass;
    pub static mut emacs_service: jobject;
    pub static mut service_class: android_emacs_service;
    pub static mut emacs_installation_time: timespec;
  }
}
use android_objs::*;

pub mod android_decls {
  use super::*;

  unsafe extern "C" {
    pub fn android_emacs_init (arg0: c_int, arg1: *mut *mut c_char, arg2: *mut c_char) -> c_int;
    pub fn android_select (arg0: c_int, arg1: *mut fd_set, arg2: *mut fd_set, arg3: *mut fd_set, arg4: *mut timespec) -> c_int;
    pub fn android_user_full_name (arg0: *mut passwd) -> *mut c_char;
    pub fn android_init_fd_class (arg0: *mut JNIEnv);
    pub fn android_is_special_directory (arg0: *const c_char, arg1: *const c_char) -> bool;
    pub fn android_get_home_directory () -> *const c_char;
    pub fn android_vfs_init (arg0: *mut JNIEnv, arg1: jobject);
    pub fn android_open (arg0: *const c_char, arg1: c_int, arg2: mode_t) -> c_int;
    pub fn android_fstat (arg0: c_int, arg1: *mut stat) -> c_int;
    pub fn android_fstatat (arg0: c_int, arg1: *const c_char, arg2: *mut stat, arg3: c_int) -> c_int;
    pub fn android_faccessat (arg0: c_int, arg1: *const c_char, arg2: c_int, arg3: c_int) -> c_int;
    pub fn android_close (arg0: c_int) -> c_int;
    pub fn android_fdopen (arg0: c_int, arg1: *const c_char) -> *mut FILE;
    pub fn android_fclose (arg0: *mut FILE) -> c_int;
    pub fn android_unlink (arg0: *const c_char) -> c_int;
    pub fn android_symlink (arg0: *const c_char, arg1: *const c_char) -> c_int;
    pub fn android_rmdir (arg0: *const c_char) -> c_int;
    pub fn android_mkdir (arg0: *const c_char, arg1: mode_t) -> c_int;
    pub fn android_renameat_noreplace (arg0: c_int, arg1: *const c_char, arg2: c_int, arg3: *const c_char) -> c_int;
    pub fn android_rename (arg0: *const c_char, arg1: *const c_char) -> c_int;
    pub fn android_fchmodat (arg0: c_int, arg1: *const c_char, arg2: mode_t, arg3: c_int) -> c_int;
    pub fn android_readlinkat (arg0: c_int, arg1: *const c_char, arg2: *mut c_char, arg3: size_t) -> ssize_t;
    pub fn android_lock_bitmap (arg0: android_drawable, arg1: *mut AndroidBitmapInfo, arg2: *mut jobject) -> *mut c_uchar;
    pub fn android_damage_window (arg0: android_window, arg1: *mut android_rectangle);
    pub fn android_get_screen_width () -> c_int;
    pub fn android_get_screen_height () -> c_int;
    pub fn android_get_mm_width () -> c_int;
    pub fn android_get_mm_height () -> c_int;
    pub fn android_detect_mouse () -> bool;
    pub fn android_detect_keyboard () -> bool;
    pub fn android_set_dont_focus_on_map (arg0: android_window, arg1: bool);
    pub fn android_set_dont_accept_focus (arg0: android_window, arg1: bool);
    pub fn android_set_wm_name (arg0: android_window, arg1: jstring);
    pub fn android_verify_jni_string (arg0: *const c_char) -> c_int;
    pub fn android_build_string (arg0: Lisp_Object, ...) -> jstring;
    pub fn android_build_jstring (arg0: *const c_char) -> jstring;
    pub fn android_exception_check ();
    pub fn android_exception_check_1 (arg0: jobject);
    pub fn android_exception_check_2 (arg0: jobject, arg1: jobject);
    pub fn android_exception_check_3 (arg0: jobject, arg1: jobject, arg2: jobject);
    pub fn android_exception_check_4 (arg0: jobject, arg1: jobject, arg2: jobject, arg3: jobject);
    pub fn android_exception_check_5 (arg0: jobject, arg1: jobject, arg2: jobject, arg3: jobject, arg4: jobject);
    pub fn android_exception_check_6 (arg0: jobject, arg1: jobject, arg2: jobject, arg3: jobject, arg4: jobject, arg5: jobject);
    pub fn android_exception_check_nonnull (arg0: *mut c_void, arg1: jobject);
    pub fn android_exception_check_nonnull_1 (arg0: *mut c_void, arg1: jobject, arg2: jobject);
    pub fn android_get_keysym_name (arg0: c_int, arg1: *mut c_char, arg2: size_t);
    pub fn android_wait_event ();
    pub fn android_toggle_on_screen_keyboard (arg0: android_window, arg1: bool);
    pub fn android_restart_emacs () -> !;
    pub fn android_request_directory_access () -> c_int;
    pub fn android_external_storage_available_p () -> bool;
    pub fn android_request_storage_access ();
    pub fn android_get_current_api_level () -> c_int;
    pub fn android_opendir (arg0: *const c_char) -> *mut android_vdir;
    pub fn android_dirfd (arg0: *mut android_vdir) -> c_int;
    /// C: `struct dirent *android_readdir (struct android_vdir *);` - `struct dirent` has no
    /// mirror (`lib/include/dirent.rs` keeps it opaque, and no unit reads its fields), so the
    /// pointer keeps the untyped form (PORTING.md P-55).
    pub fn android_readdir (arg0: *mut android_vdir) -> *mut c_void;
    pub fn android_closedir (arg0: *mut android_vdir);
    pub fn android_open_asset (arg0: *const c_char, arg1: c_int, arg2: mode_t) -> android_fd_or_asset;
    pub fn android_close_asset (arg0: android_fd_or_asset) -> c_int;
    pub fn android_asset_read_quit (arg0: android_fd_or_asset, arg1: *mut c_void, arg2: size_t) -> ssize_t;
    pub fn android_asset_read (arg0: android_fd_or_asset, arg1: *mut c_void, arg2: size_t) -> ssize_t;
    pub fn android_asset_lseek (arg0: android_fd_or_asset, arg1: off_t, arg2: c_int) -> off_t;
    pub fn android_asset_fstat (arg0: android_fd_or_asset, arg1: *mut stat) -> c_int;
    pub fn android_browse_url (arg0: Lisp_Object, arg1: Lisp_Object) -> Lisp_Object;
    pub fn android_query_battery (arg0: *mut android_battery_state) -> c_int;
    pub fn android_display_toast (arg0: *const c_char);
    pub fn android_before_select ();
    pub fn android_check_query ();
    pub fn android_check_query_urgent ();
    pub fn android_run_in_emacs_thread (arg0: Option<unsafe extern "C" fn (*mut c_void)>, arg1: *mut c_void) -> c_int;
    pub fn android_write_event (arg0: *mut android_event);
    pub fn android_rewrite_spawn_argv (arg0: *const *const *const c_char) -> c_int;
  }
}
use android_decls::*;
