// femacs: port of src/android.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` except the ones whose address C takes or that C passes
// as a thread body / signal handler, which are `unsafe extern "C" fn` items (a function-pointer
// parameter cannot be a Rust-ABI item; the androidmenu.rs / androidselect.rs shape).
//
// **This file is P-86's largest unit (C 7,594 lines) and is being ported in slices.**  Slice 1 is
// C 1-1037 (through `android_select`) and slice 2 is C 1038-1869 (`android_run_debug_thread`
// through `android_init_emacs_pixmap`); the next live definition is `android_init_graphics_point`
// (C 1871).  The ledger `docs/ports/android.md` carries the C line-range table for C 1-1869 plus
// the "the remainder" row, the `## Evidence` readings and the `## Deviations`.
//
// The frozen configuration compiles no `android.o`: `configure.ac` 2598 puts `android.o` in
// `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])` (2602), and `docs/baseline/frozen-build-sources.txt`
// lists no `android.o` for this target, so `extract-abi.py`'s checklist for the file is empty and
// its checkbox sections cannot be coverage evidence.  What follows is P-86 ②'s substitute: the
// ledger's line-range table and readings.  The gates, in C's order:
//
//   * the file gate (`HAVE_ANDROID`; `android.o` is in `ANDROID_OBJ` in *both* Android branches,
//     2597-2598) is `feature = "have_android"`, on every item *and on C 22-55's include graph* -
//     which is why the frozen configuration compiles nothing here;
//   * C 61's `#ifndef ANDROID_STUBIFY` ... C 7519's `#else` ... C 7594's `#endif` is
//     `not_android_stubify`, the feature the tree registered for C's
//     `HAVE_ANDROID && !defined ANDROID_STUBIFY` (the androidmenu.rs/androidselect.rs gate).
//     This slice is wholly inside the real arm; the stub arm (C 7519-7594) is in the remainder.
//   * `__ANDROID_API__ < 21` / `>= 21` (C 297, 310, 367, 426, 509, 522, 604, 672, 713, 738, 923,
//     973) is the derived feature `android_api_lt_21`; its negation is C's `>= 21` arm
//     (androidselect.rs's `android_api_le_11`/`android_api_le_30` shape);
//   * `THREADS_ENABLED` (C 176, 325, 401, 445, 614, 685, 752, 930, 1030) is `threads_enabled`;
//   * `__i386__` (C 241) is `i386`; `__aarch64__` (C 70) is `aarch64`.
//
// `HAVE_WINDOW_SYSTEM` (configure.ac 7518) has no `#if` in this slice and therefore no gate: the
// frame.rs flag block 3.408 laid down is gated on `have_window_system`, and this unit carries
// `have_android` without it, so that block does not apply here.
//
// The `<pthread.h>` and `<semaphore.h>` names (C 32-33) and the four `<signal.h>` names the
// frozen `signal.rs` mirror does not carry have no mirror in the tree and are declared in this
// unit (PORTING.md P-88/P-70, the `sysdep.rs` `<pthread.h>` block and androidselect.rs's
// `<dlfcn.h>` block); their values are the NDK's, measured - the ledger records them.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 20: `#include <config.h>` (ungated)

/* C 22-55: the file's own include list.  It sits outside C 61's `#ifndef`, so the whole graph is
   on the file gate alone - no translation unit the frozen configuration builds reaches any of it.  */
#[cfg (feature = "have_android")]
include!("../lib/include/allocator.rs");              // C 22: `#include <allocator.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/careadlinkat.rs");           // C 24: `#include <careadlinkat.h>` (slice 2)
#[cfg (feature = "have_android")]
include!("../lib/include/errno.rs");                  // C 25: `#include <errno.h>` (errno, EINTR)
#[cfg (feature = "have_android")]
include!("../lib/include/fcntl.in.rs");               // C 26: `#include <fcntl.h>` (open, O_*)
#[cfg (feature = "have_android")]
include!("../lib/include/fingerprint.rs");            // C 27: `#include <fingerprint.h>` (slice 2)
#[cfg (feature = "have_android")]
include!("../lib/include/limits.in.rs");              // C 30: `#include <limits.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/signal.rs");                 // C 34: `#include <signal.h>` (sigset_t, SIG*)
#[cfg (feature = "have_android")]
include!("../lib/include/stat-time.rs");              // C 35: `#include <stat-time.h>` (get_stat_mtime)
#[cfg (feature = "have_android")]
include!("../lib/include/string.rs");                 // C 37: `#include <string.h>` (strerror, strlen)
#[cfg (feature = "have_android")]
include!("../lib/include/stdlib.rs");                 // C 37's chain: `<stdlib.h>` (malloc, free)
#[cfg (feature = "have_android")]
include!("../lib/include/unistd.in.rs");              // C 39: `#include <unistd.h>` (read/write/close)
#[cfg (feature = "have_android")]
include!("../lib/include/sys_stat.in.rs");            // C 42: `#include <sys/stat.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/sys_select.rs");             // C 43: `#include <sys/select.h>` (fd_set, pselect)
#[cfg (feature = "have_android")]
include!("../lib/include/minmax.rs");                 // C 46: `#include <minmax.h>` (MIN)
#[cfg (feature = "have_android")]
include!("include/android.rs");                       // C 48: `#include "android.h"`
#[cfg (feature = "have_android")]
include!("include/androidgui.rs");                    // C 49: `#include "androidgui.h"`
#[cfg (feature = "have_android")]
include!("include/lisp.rs");                          // C 51: `#include "lisp.h"`
#[cfg (feature = "have_android")]
include!("include/blockinput.rs");                    // C 52: `#include "blockinput.h"`
#[cfg (feature = "have_android")]
include!("include/coding.rs");                        // C 53: `#include "coding.h"`
#[cfg (feature = "have_android")]
include!("include/systime.rs");                       // C 55: `#include "systime.h"`

/* C 23 `<assert.h>` names nothing this file uses (the only assertion entry point here is
   `eassert`, from lisp.h) so its gnulib template `lib/include/assert.in.rs` is not reached.
   C 31 `<math.h>`, C 36 `<stdckdint.h>`, C 38 `<timespec.h>`, C 41 `<sys/param.h>` and
   C 54 `<epaths.h>` name nothing in C 1-1037 either, so no item of theirs is carried in this
   slice.  C 28 `<intprops.h>` is the blind mirror no unit includes; the one name this slice
   takes from it is defined below (the androidselect.rs shape).  */
/* Slice 2 first names one item of four more of these headers, so their edges were added at
   C's own line above: C 24 `<careadlinkat.h>` (`careadlinkat`, C 1387), C 27
   `<fingerprint.h>` (`fingerprint`, C 1466-1470), C 35 `<stat-time.h>` (`get_stat_mtime`,
   C 1609) and - at the use sites below - C 29 `<libgen.h>`, C 30 `<limits.h>`'s `PATH_MAX`
   (already reached), the `<stdio.h>`/`<string.h>`/`<stdlib.h>` entries the mirrors do not
   carry (`fdopen`, `getline`, `strdup`, `setenv`) and lisp.h 4247's `hexbuf_digest`.
   C 29 `<libgen.h>` has no mirror in the tree at all (`ls femacs/lib/include | grep libgen`
   is empty), so its one name is declared in the unit (P-88 ①).  */

/* C 28: `#include <intprops.h>` - `INT_STRLEN_BOUND (long)`, C 471.  The mirror is blind
   (docs: `lib/include/intprops.rs`) and no unit includes it; the units that need the semantics
   write the Rust primitive at the use site (the androidselect.rs 116-119 shape, over
   `size_of::<$t>` instead of gnulib's `TYPE_WIDTH`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! INT_STRLEN_BOUND {
  ($t:ty) => { ((std::mem::size_of::<$t> () * 8 - 1) * 146 + 484) / 485 + 1 };
}

/* Whether or not Emacs is running inside the application process and
   Android windowing should be enabled.  */
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub static mut android_init_gui: bool = false;                 // C 59 (tentative, i.e. false)

/* C 61: `#ifndef ANDROID_STUBIFY`.  The real arm runs to C 7518; the `#else` at C 7519 and the
   `#endif` at C 7594 are in the remainder of the file (not this slice), so this slice carries the
   real arm only - `all (feature = "have_android", feature = "not_android_stubify")` - and the
   ledger's remainder row names C 7519 as the stub arm still owed.  */

/* C 63: `#include <android/bitmap.h>` - names nothing in this slice (`android_lock_bitmap` is C
   2649 and past it; the `AndroidBitmapInfo` the mirror uses stays opaque there).  */

/* C 64: `#include <android/log.h>` - the NDK's own header, not part of the reference tree, so it
   has no mirror and the four names the two arms of this slice use are declared here (P-88).  The
   three levels are the NDK's measured enums: `ANDROID_LOG_INFO == 4`, `ANDROID_LOG_WARN == 5`,
   `ANDROID_LOG_FATAL == 7` (measured with this machine's NDK 29.0.14206865 clang,
   `--target=aarch64-linux-android24`; the ledger records the command).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_LOG_INFO: c_int = 4;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_LOG_WARN: c_int = 5;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_LOG_FATAL: c_int = 7;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `int __android_log_print (int prio, const char *tag, const char *fmt, ...)` (log.h 112)
  pub fn __android_log_print (prio: c_int, tag: *const c_char, fmt: *const c_char, ...) -> c_int;
}

/* C 66-68: `#include <linux/unistd.h>` and `#include <sys/syscall.h>` - no name of either appears
   in C 1-1037 (`syscall` is C 5330's arm, past this slice); the P-88 declaration segment belongs
   to the slice that first names one.

   C 70-72: `#ifdef __aarch64__` / `#include <arm_neon.h>` / `#endif` - the NDK header has no
   mirror and no name of it appears before C 4064 (the four `#ifdef __aarch64__` NEON regions), so
   this slice carries both arms as this comment.  */

/* C 32-33: `#include <pthread.h>` and `#include <semaphore.h>`.  Neither header has a mirror in
   the tree (`ls femacs/lib/include | grep -E 'pthread|semaphore'` shows only `pthread_sigmask.rs`,
   which `signal.rs` already reaches).  The names this slice uses are declared here (P-88/P-70, the
   sysdep.rs `<pthread.h>` block shape); the six types are the NDK's measured aarch64/bionic
   records, not the frozen target's, and the ledger records the measurement.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C, align(4))]
pub struct pthread_mutex_t { pub __opaque: [c_uchar; 40] }       // bionic, sizeof 40 align 4
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C, align(4))]
pub struct pthread_cond_t { pub __opaque: [c_uchar; 48] }        // bionic, sizeof 48 align 4
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type pthread_t = c_long;                                     // bionic, sizeof 8
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type pthread_key_t = c_int;                                  // bionic, sizeof 4
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C, align(4))]
pub struct sem_t { pub __opaque: [c_uchar; 16] }                 // bionic, sizeof 16 align 4
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /* int pthread_mutex_init (pthread_mutex_t *, const pthread_mutexattr_t *);  */
  pub fn pthread_mutex_init (mutex: *mut pthread_mutex_t, attr: *const c_void) -> c_int;
  /* int pthread_mutex_lock (pthread_mutex_t *);  */
  pub fn pthread_mutex_lock (mutex: *mut pthread_mutex_t) -> c_int;
  /* int pthread_mutex_unlock (pthread_mutex_t *);  */
  pub fn pthread_mutex_unlock (mutex: *mut pthread_mutex_t) -> c_int;
  /* int pthread_mutex_destroy (pthread_mutex_t *);  */
  pub fn pthread_mutex_destroy (mutex: *mut pthread_mutex_t) -> c_int;
  /* int pthread_cond_init (pthread_cond_t *, const pthread_condattr_t *);  */
  pub fn pthread_cond_init (cond: *mut pthread_cond_t, attr: *const c_void) -> c_int;
  /* int pthread_cond_wait (pthread_cond_t *, pthread_mutex_t *);  */
  pub fn pthread_cond_wait (cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
  /* int pthread_cond_broadcast (pthread_cond_t *);  */
  pub fn pthread_cond_broadcast (cond: *mut pthread_cond_t) -> c_int;
  /* pthread_t pthread_self (void);  */
  pub fn pthread_self () -> pthread_t;
  /* int pthread_create (pthread_t *, const pthread_attr_t *,
                         void *(*)(void *), void *);  */
  pub fn pthread_create (thread: *mut pthread_t, attr: *const c_void,
                         start: Option<unsafe extern "C" fn (*mut c_void) -> *mut c_void>,
                         arg: *mut c_void) -> c_int;
  /* int pthread_join (pthread_t, void **);  */
  pub fn pthread_join (thread: pthread_t, retval: *mut *mut c_void) -> c_int;
  /* int pthread_key_create (pthread_key_t *, void (*)(void *));  */
  pub fn pthread_key_create (key: *mut pthread_key_t,
                             destructor: Option<unsafe extern "C" fn (*mut c_void)>) -> c_int;
  /* void *pthread_getspecific (pthread_key_t);  */
  pub fn pthread_getspecific (key: pthread_key_t) -> *mut c_void;
  /* int pthread_setspecific (pthread_key_t, const void *);  */
  pub fn pthread_setspecific (key: pthread_key_t, value: *const c_void) -> c_int;
  /* int pthread_kill (pthread_t, int);  */
  pub fn pthread_kill (thread: pthread_t, sig: c_int) -> c_int;
  /* int sem_init (sem_t *, int, unsigned int);  */
  pub fn sem_init (sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
  /* int sem_post (sem_t *);  */
  pub fn sem_post (sem: *mut sem_t) -> c_int;
  /* int sem_wait (sem_t *);  */
  pub fn sem_wait (sem: *mut sem_t) -> c_int;
  /* int sem_close (sem_t *);  */
  pub fn sem_close (sem: *mut sem_t) -> c_int;
  /* pid_t gettid (void);  -- bionic's <unistd.h>  */
  pub fn gettid () -> pid_t;
  /* int pipe2 (int[2], int);  -- bionic's <unistd.h>  */
  pub fn pipe2 (fds: *mut c_int, flags: c_int) -> c_int;
}

/* C 34's `<signal.h>`: the frozen `signal.rs` mirror is the Darwin one, and four names this slice
   uses are not in it.  `sigfillset`/`sigdelset`/`sigwait` are declared here (the P-88 shape) and
   `SA_SIGINFO`, `sig_atomic_t` are the NDK's measured values; the mirror's `sigset_t`/`sigaction`
   are Darwin-shaped (4/24 bytes against bionic's 8/32) - the ledger records that as the mirror
   gap, and the arm pre-flight patches the mirror copy.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type sig_atomic_t = c_int;                                   // bionic, sizeof 4
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub struct siginfo_t;                                            // only ever a pointer here
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const SA_SIGINFO: c_int = 0x00000004;                        // bionic, measured
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /* int sigfillset (sigset_t *);  */
  pub fn sigfillset (set: *mut sigset_t) -> c_int;
  /* int sigdelset (sigset_t *, int);  */
  pub fn sigdelset (set: *mut sigset_t, signo: c_int) -> c_int;
  /* int sigwait (const sigset_t *, int *);  */
  pub fn sigwait (set: *const sigset_t, sig: *mut c_int) -> c_int;
  /* int kill (pid_t, int);  -- the frozen signal.rs mirror does not carry it  */
  pub fn kill (pid: pid_t, sig: c_int) -> c_int;
}

/* Toolkit (graphics and window) class tables.  */

/* Structure describing the EmacsPixmap class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_pixmap                            // C 74-78
{
  pub class: jclass,
  pub constructor_mutable: jmethodID,
}

/* Structure describing the android.graphics.Point class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_graphics_point                          // C 80-84
{
  pub class: jclass,
  pub constructor: jmethodID,
}

/* Structure describing the EmacsDrawable class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_drawable                          // C 86-90
{
  pub class: jclass,
  pub get_bitmap: jmethodID,
}

/* Structure describing the EmacsWindow class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_window                            // C 92-120
{
  pub class: jclass,
  pub swap_buffers: jmethodID,
  pub toggle_on_screen_keyboard: jmethodID,
  pub lookup_string: jmethodID,
  pub set_fullscreen: jmethodID,
  pub change_window_background: jmethodID,
  pub reparent_to: jmethodID,
  pub map_window: jmethodID,
  pub unmap_window: jmethodID,
  pub resize_window: jmethodID,
  pub move_window: jmethodID,
  pub move_resize_window: jmethodID,
  pub make_input_focus: jmethodID,
  pub raise: jmethodID,
  pub lower: jmethodID,
  pub reconfigure: jmethodID,
  pub get_window_geometry: jmethodID,
  pub translate_coordinates: jmethodID,
  pub set_dont_accept_focus: jmethodID,
  pub set_dont_focus_on_map: jmethodID,
  pub define_cursor: jmethodID,
  pub damage_rect: jmethodID,
  pub recreate_activity: jmethodID,
  pub clear_window: jmethodID,
  pub clear_area: jmethodID,
  pub set_wm_name: jmethodID,
}

/* Structure describing the EmacsCursor class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_cursor                            // C 122-126
{
  pub class: jclass,
  pub constructor: jmethodID,
}

/* Structure describing the KeyCharacterMap class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_key_character_map                       // C 128-132
{
  pub class: jclass,
  pub get_dead_char: jmethodID,
}

/* Structure describing the EmacsHandleObject class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_handle                            // C 134-139
{
  pub class: jclass,
  pub destroy_handle: jmethodID,
  pub handle: jfieldID,
}

/* The API level of the current device.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut android_api_level: c_int = 0;                   // C 142 (tentative, i.e. zero)

/* The directory used to store site-lisp.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_site_load_path: *mut c_char = std::ptr::null_mut ();   // C 145

/* The directory used to store native libraries.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_lib_dir: *mut c_char = std::ptr::null_mut ();          // C 148

/* The directory used to store game files.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_game_path: *mut c_char = std::ptr::null_mut ();        // C 151

/* The directory used to store temporary files.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_cache_dir: *mut c_char = std::ptr::null_mut ();        // C 154

/* The list of archive files within which the Java virtual macine
   looks for class files.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut android_class_path: *mut c_char = std::ptr::null_mut ();           // C 158

/* The display's pixel densities.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_pixel_density_x: f64 = 0.0;                            // C 161
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_pixel_density_y: f64 = 0.0;                            // C 161

/* The display pixel density used to convert between point and pixel
   font sizes.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_scaled_pixel_density: f64 = 0.0;                       // C 165

/* The display's current UI mode.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_ui_mode: c_int = 0;                                    // C 168

/* The Android application data directory.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut android_files_dir: *mut c_char = std::ptr::null_mut ();            // C 171

/* The Java environment being used for the main thread.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_java_env: *mut JNIEnv = std::ptr::null_mut ();         // C 174

/* C 176-181: `#ifdef THREADS_ENABLED` - the Java VM new threads attach to.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "threads_enabled")]
#[unsafe(no_mangle)]
pub static mut android_jvm: *mut JavaVM = std::ptr::null_mut ();              // C 179

/* The EmacsGC class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_class: jclass = std::ptr::null_mut ();                    // C 184

/* Various fields.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_foreground: jfieldID = std::ptr::null_mut ();             // C 187
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_background: jfieldID = std::ptr::null_mut ();
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_function: jfieldID = std::ptr::null_mut ();               // C 188
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_clip_rects: jfieldID = std::ptr::null_mut ();
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_clip_x_origin: jfieldID = std::ptr::null_mut ();          // C 189
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_clip_y_origin: jfieldID = std::ptr::null_mut ();
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_stipple: jfieldID = std::ptr::null_mut ();                // C 190
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_clip_mask: jfieldID = std::ptr::null_mut ();
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_fill_style: jfieldID = std::ptr::null_mut ();             // C 191
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_ts_origin_x: jfieldID = std::ptr::null_mut ();
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_ts_origin_y: jfieldID = std::ptr::null_mut ();            // C 192
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_line_style: jfieldID = std::ptr::null_mut ();
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_line_width: jfieldID = std::ptr::null_mut ();             // C 193
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_dash_offset: jfieldID = std::ptr::null_mut ();
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_dashes: jfieldID = std::ptr::null_mut ();                 // C 194

/* The constructor and one function.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_constructor: jmethodID = std::ptr::null_mut ();           // C 197
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut emacs_gc_mark_dirty: jmethodID = std::ptr::null_mut ();

/* The Rect class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut android_rect_class: jclass = std::ptr::null_mut ();                // C 200

/* Its constructor.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut android_rect_constructor: jmethodID = std::ptr::null_mut ();       // C 203

/* The EmacsService object.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut emacs_service: jobject = std::ptr::null_mut ();                // C 206

/* Various methods associated with the EmacsService.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut service_class: android_emacs_service = unsafe { std::mem::zeroed () };    // C 209 (tentative)

/* Various methods associated with the EmacsPixmap class.  C's `static struct` is a tentative
   definition, i.e. every member zero (the androidmenu.rs `menu_class` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut pixmap_class: android_emacs_pixmap = unsafe { std::mem::zeroed () };          // C 212
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut point_class: android_graphics_point = unsafe { std::mem::zeroed () };         // C 215
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut drawable_class: android_emacs_drawable = unsafe { std::mem::zeroed () };      // C 218
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut window_class: android_emacs_window = unsafe { std::mem::zeroed () };          // C 221
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut cursor_class: android_emacs_cursor = unsafe { std::mem::zeroed () };          // C 224
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut key_character_map_class: android_key_character_map = unsafe { std::mem::zeroed () };   // C 227
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut handle_class: android_emacs_handle = unsafe { std::mem::zeroed () };          // C 231

/* The time at which Emacs was installed, which also supplies the
   mtime of asset files.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut emacs_installation_time: timespec = unsafe { std::mem::zeroed () };       // C 235 (tentative)

/* The last event serial used.  This is a 32 bit value, but it is
   stored in unsigned long to be consistent with X.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut event_serial: c_uint = 0;                   // C 239 (tentative, i.e. zero)

/* C 241-246: `#ifdef __i386__` - unused pointer used to control compiler optimizations.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "i386")]
#[unsafe(no_mangle)]
pub static mut unused_pointer: *mut c_void = std::ptr::null_mut ();           // C 244

/* Whether or not the default signal mask has been changed.  If so,
   the signal mask must be restored before calling
   android_emacs_init.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut signal_mask_changed_p: bool = false;            // C 251 (tentative, i.e. false)

/* The signal mask at the time Emacs was started.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut startup_signal_mask: sigset_t = unsafe { std::mem::zeroed () };   // C 254 (tentative)

/* Event handling functions.  Events are stored on a (circular) queue
   that is read synchronously.  The Android port replaces pselect with
   a function android_select, which runs pselect in a separate thread,
   but more importantly also waits for events to be available on the
   android event queue.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_event_container                          // C 264-271
{
  /* The next and last events in this queue.  */
  pub next: *mut android_event_container,
  pub last: *mut android_event_container,

  /* The event itself.  */
  pub event: android_event,
}

/* Thread-specific component of the Android event queue.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_thread_event_queue                       // C 275-308
{
  /* Mutex protecting the select data.  */
  pub select_mutex: pthread_mutex_t,

  /* The thread used to run select.  */
  pub select_thread: pthread_t,

  /* Arguments to pselect used by the select thread.  */
  pub select_readfds: *mut fd_set,
  pub select_writefds: *mut fd_set,
  pub select_exceptfds: *mut fd_set,
  pub select_timeout: *mut timespec,
  pub select_nfds: c_int,

  /* Semaphores posted around invocations of pselect.  */
  pub start_sem: sem_t,
  pub select_sem: sem_t,

  /* Value of pselect.  */
  pub select_rc: c_int,

  /* C 297-304's `#if __ANDROID_API__ < 21` / `#else` pair: the select self-pipe (the old arm) or
     the cancellation flag (the `>= 21` arm).  C's `volatile` has no Rust field form and is
     dropped (the ledger records it).  */
  #[cfg (feature = "android_api_lt_21")]
  pub select_pipe: [c_int; 2],                              // C 299
  #[cfg (not (feature = "android_api_lt_21"))]
  pub cancel_signal_received: sig_atomic_t,                 // C 303

  /* Whether this thread must exit.  */
  pub canceled: c_int,
}

/* C 310-312: `#if __ANDROID_API__ >= 21` / `#define SELECT_SIGNAL SIGUSR1` / `#endif`.  The
   symbolic constant keeps its name; `SIGUSR1` comes from the `signal.rs` mirror.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (not (feature = "android_api_lt_21"))]
pub const SELECT_SIGNAL: c_int = SIGUSR1;                  // C 311

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_event_queue                              // C 314-333
{
  /* Mutex protecting the event queue.  */
  pub mutex: pthread_mutex_t,

  /* Condition variables for the reading side.  */
  pub read_var: pthread_cond_t,

  /* Circular queue of events.  */
  pub events: android_event_container,

  /* C 325-329's `#ifndef THREADS_ENABLED`: if threads are disabled, the thread-specific
     component of the main and only thread.  */
  #[cfg (not (feature = "threads_enabled"))]
  pub thread: android_thread_event_queue,

  /* The number of events in the queue.  */
  pub num_events: c_int,
}

/* The global event queue.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut event_queue: android_event_queue = unsafe { std::mem::zeroed () };   // C 336 (tentative)

/* C 339: `static void *android_run_select_thread (void *);` - a forward declaration, which has no
   Rust counterpart (a function item is visible for the whole module regardless of position; the
   definition is C 501-668 below).  */

/* Initialize a thread-local component of the Android event queue
   THREAD.  Create and initialize a thread whose purpose is to execute
   `select' in an interruptible manner, and initialize variables or file
   descriptors with which to communicate with it.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_thread_events (thread: *mut android_thread_event_queue)   // C 346-399
{
  (*thread).canceled = false as c_int;
  (*thread).select_readfds = std::ptr::null_mut ();
  (*thread).select_writefds = std::ptr::null_mut ();
  (*thread).select_exceptfds = std::ptr::null_mut ();
  (*thread).select_timeout = std::ptr::null_mut ();
  (*thread).select_nfds = 0;

  if pthread_mutex_init (&raw mut (*thread).select_mutex, std::ptr::null ()) != 0
    {
      let __func__: *const c_char = c"android_init_thread_events".as_ptr ();
      __android_log_print (ANDROID_LOG_FATAL, __func__,
			   c"pthread_mutex_init: %s".as_ptr (),
			   strerror (*errno ()));
      emacs_abort ();
    }

  sem_init (&raw mut (*thread).select_sem, 0, 0);
  sem_init (&raw mut (*thread).start_sem, 0, 0);

  #[cfg (feature = "android_api_lt_21")]
  {
    /* Set up the file descriptor used to wake up pselect.  */
    if pipe2 (&raw mut (*thread).select_pipe as *mut c_int, O_CLOEXEC) < 0
      {
	let __func__: *const c_char = c"android_init_thread_events".as_ptr ();
	__android_log_print (ANDROID_LOG_FATAL, __func__,
			     c"pipe2: %s".as_ptr (), strerror (*errno ()));
	emacs_abort ();
      }

    /* Make sure the read end will fit in fd_set.  */
    if (*thread).select_pipe[0] >= FD_SETSIZE
      {
	let __func__: *const c_char = c"android_init_thread_events".as_ptr ();
	__android_log_print (ANDROID_LOG_FATAL, __func__,
			     c"read end of select pipe exceeds FD_SETSIZE!".as_ptr ());
	emacs_abort ();
      }
  }

  /* Start the select thread.  */
  if pthread_create (&raw mut (*thread).select_thread, std::ptr::null (),
		     Some (android_run_select_thread), thread as *mut c_void) != 0
    {
      let __func__: *const c_char = c"android_init_thread_events".as_ptr ();
      __android_log_print (ANDROID_LOG_FATAL, __func__,
			   c"pthread_create: %s".as_ptr (), strerror (*errno ()));
      emacs_abort ();
    }

  /* Wait for the thread to be initialized.  */
  while sem_wait (&raw mut (*thread).select_sem) < 0 {}
}

/* C 401-436: `#ifdef THREADS_ENABLED` - destroy a thread-local component of the Android event
   queue provided as DATA, and release DATA's storage itself.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "threads_enabled")]
unsafe extern "C" fn android_finalize_thread_events (data: *mut c_void)      // C 408-431
{
  let mut rc: c_int;
  let mut thread: *mut android_thread_event_queue;

  /* Cancel the thread and pause till it exits.  */
  thread = data as *mut android_thread_event_queue;
  (*thread).canceled = 1;
  sem_post (&raw mut (*thread).start_sem);
  rc = pthread_join ((*thread).select_thread, std::ptr::null_mut ());
  if rc != 0
    { emacs_abort (); }

  /* Release the select thread, semaphores, etc.  */
  pthread_mutex_destroy (&raw mut (*thread).select_mutex);
  sem_close (&raw mut (*thread).select_sem);
  sem_close (&raw mut (*thread).start_sem);
  #[cfg (feature = "android_api_lt_21")]
  {
    close ((*thread).select_pipe[0]);
    close ((*thread).select_pipe[1]);
  }
  xfree (thread as *mut c_void);
}

/* TLS keys associating polling threads with Emacs threads.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "threads_enabled")]
static mut poll_thread: pthread_key_t = 0;                 // C 434 (tentative)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "threads_enabled")]
static mut poll_thread_internal: pthread_key_t = 0;        // C 434 (tentative)

/* Return the thread-specific component of the event queue appertaining
   to this thread, or create it as well as a polling thread if
   absent.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_get_poll_thread () -> *mut android_thread_event_queue   // C 442-459
{
  #[cfg (not (feature = "threads_enabled"))]
  {
    return &raw mut event_queue.thread;
  }
  #[cfg (feature = "threads_enabled")]
  {
    let mut queue: *mut android_thread_event_queue;

    queue = pthread_getspecific (poll_thread) as *mut android_thread_event_queue;
    if queue.is_null ()
      {
	queue = xmalloc (std::mem::size_of::<android_thread_event_queue> ())
		  as *mut android_thread_event_queue;
	android_init_thread_events (queue);
	pthread_setspecific (poll_thread, queue as *const c_void);
      }
    return queue;
  }
}

/* Set the task name of the current task to NAME, a string at most 21
   characters in length.

   This name is displayed as that of the task (LWP)'s pthread in
   GDB.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_task_name (name: *const c_char)      // C 467-499
{
  /* C's `char proc_name[INT_STRLEN_BOUND (long) + sizeof "/proc/self/task//comm"]` is 20 + 21.  */
  let mut proc_name: [c_char; INT_STRLEN_BOUND! (c_long) + 21] =
    [0; INT_STRLEN_BOUND! (c_long) + 21];
  let mut fd: c_int;
  let mut lwp: pid_t;
  let mut length: usize;

  lwp = gettid ();
  sprintf (&raw mut proc_name as *mut c_char, c"/proc/self/task/%ld/comm".as_ptr (),
	   lwp as c_long);
  fd = open (&raw mut proc_name as *const c_char, O_WRONLY | O_TRUNC);

  'failure: {
    if fd < 0
      { break 'failure; }

    length = strlen (name);

    if write (fd, name as *const c_void, MIN! (16usize, length)) < 0
      { break 'failure; }

    close (fd);
    return;
  }

  /* failure: */
  let __func__: *const c_char = c"android_set_task_name".as_ptr ();
  __android_log_print (ANDROID_LOG_WARN, __func__,
		       c"Failed to set task name for LWP %ld: %s".as_ptr (),
		       lwp as c_long, strerror (*errno ()));

  /* Close the file descriptor if it is already set.  */
  if fd >= 0
    { close (fd); }
}

/* C 501-670; C 501's `static void *android_run_select_thread (void *thread_data)` is passed to
   `pthread_create`, so it is an `unsafe extern "C" fn` item (a function-pointer argument cannot be
   a Rust-ABI item).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_run_select_thread (thread_data: *mut c_void) -> *mut c_void
{
  /* Apparently this is required too.  */
  JNI_STACK_ALIGNMENT_PROLOGUE! ();

  let mut rc: c_int;
  let mut data: *mut android_thread_event_queue;
  #[cfg (feature = "android_api_lt_21")]
  let mut nfds: c_int;
  #[cfg (feature = "android_api_lt_21")]
  let mut readfds: fd_set = unsafe { std::mem::zeroed () };
  #[cfg (feature = "android_api_lt_21")]
  let mut byte: c_char = 0;
  #[cfg (not (feature = "android_api_lt_21"))]
  let mut signals: sigset_t = unsafe { std::mem::zeroed () };
  #[cfg (not (feature = "android_api_lt_21"))]
  let mut waitset: sigset_t = unsafe { std::mem::zeroed () };
  #[cfg (not (feature = "android_api_lt_21"))]
  let mut sig: c_int = 0;

  /* Set the name of this thread's LWP for debugging purposes.  */
  android_set_task_name (c"Emacs polling thread".as_ptr ());
  data = thread_data as *mut android_thread_event_queue;

  /* C 522-603: the `__ANDROID_API__ < 21` arm.  A completely different implementation is used
     when building for Android versions earlier than 21, because pselect with a signal mask does
     not work properly ... a file descriptor is selected instead.  */
  #[cfg (feature = "android_api_lt_21")]
  {
    /* Release the user after initialization.  */
    sem_post (&raw mut (*data).select_sem);

    while true
      {
	/* Wait for the thread to be released.  */
	while sem_wait (&raw mut (*data).start_sem) < 0 {}
	if (*data).canceled != 0
	  { return std::ptr::null_mut (); }

	/* Get the select lock and call pselect.  API 8 does not have
	   working pselect in any sense.  Instead, pselect wakes up on
	   select_pipe[0].  */

	pthread_mutex_lock (&raw mut (*data).select_mutex);
	nfds = (*data).select_nfds;

	if ! (*data).select_readfds.is_null ()
	  { readfds = *(*data).select_readfds; }
	else
	  { FD_ZERO (&raw mut readfds); }

	if nfds < (*data).select_pipe[0] + 1
	  { nfds = (*data).select_pipe[0] + 1; }
	FD_SET ((*data).select_pipe[0], &raw mut readfds);

	rc = pselect (nfds, &raw mut readfds,
		      (*data).select_writefds,
		      (*data).select_exceptfds,
		      (*data).select_timeout,
		      std::ptr::null ());

	/* Subtract 1 from rc if readfds contains the select pipe, and
	   also remove it from that set.  */

	if rc != -1 && FD_ISSET ((*data).select_pipe[0], &readfds)
	  {
	    rc -= 1;
	    FD_CLR ((*data).select_pipe[0], &raw mut readfds);

	    /* If no file descriptors aside from the select pipe are
	       ready, then pretend that an error has occurred.  */
	    if rc == 0
	      { rc = -1; }
	  }

	/* Save the read file descriptor set back again.  */

	if ! (*data).select_readfds.is_null ()
	  { *(*data).select_readfds = readfds; }

	(*data).select_rc = rc;
	pthread_mutex_unlock (&raw mut (*data).select_mutex);

	/* Signal the main thread that there is now data to read.  Hold
	   the event queue lock during this process to make sure this
	   does not happen before the main thread begins to wait for the
	   condition variable.  */

	pthread_mutex_lock (&raw mut event_queue.mutex);
	pthread_cond_broadcast (&raw mut event_queue.read_var);
	pthread_mutex_unlock (&raw mut event_queue.mutex);

	/* Read a single byte from the select pipe.  */
	read ((*data).select_pipe[0], &raw mut byte as *mut c_void, 1);

	/* Signal the Emacs thread that pselect is done.  If read_var
	   was signaled by android_write_event, event_queue.mutex could
	   still be locked, so this must come before.  */
	sem_post (&raw mut (*data).select_sem);
      }
  }

  /* C 604-667: the `__ANDROID_API__ >= 21` arm.  */
  #[cfg (not (feature = "android_api_lt_21"))]
  {
    sigfillset (&raw mut signals);
    if pthread_sigmask (SIG_BLOCK, &raw const signals, std::ptr::null_mut ()) != 0
      {
	let __func__: *const c_char = c"android_run_select_thread".as_ptr ();
	__android_log_print (ANDROID_LOG_FATAL, __func__,
			     c"pthread_sigmask: %s".as_ptr (), strerror (*errno ()));
      }

    sigdelset (&raw mut signals, SELECT_SIGNAL);
    sigemptyset (&raw mut waitset);
    sigaddset (&raw mut waitset, SELECT_SIGNAL);
    #[cfg (feature = "threads_enabled")]
    {
      pthread_setspecific (poll_thread_internal, thread_data as *const c_void);
    }
    /* Release the user after initialization.  */
    sem_post (&raw mut (*data).select_sem);

    while true
      {
	/* Wait for the thread to be released.  */
	while sem_wait (&raw mut (*data).start_sem) < 0 {}
	if (*data).canceled != 0
	  { return std::ptr::null_mut (); }

	/* Get the select lock and call pselect.  */
	(*data).cancel_signal_received = 0;
	pthread_mutex_lock (&raw mut (*data).select_mutex);
	rc = pselect ((*data).select_nfds,
		      (*data).select_readfds,
		      (*data).select_writefds,
		      (*data).select_exceptfds,
		      (*data).select_timeout,
		      &raw const signals);
	(*data).select_rc = rc;
	pthread_mutex_unlock (&raw mut (*data).select_mutex);

	/* Signal the main thread that there is now data to read.  Hold
	   the event queue lock during this process to make sure this
	   does not happen before the main thread begins to wait for the
	   condition variable.  */

	pthread_mutex_lock (&raw mut event_queue.mutex);
	pthread_cond_broadcast (&raw mut event_queue.read_var);
	pthread_mutex_unlock (&raw mut event_queue.mutex);

	/* Test a separate flag `data->cancel_signal_received' rather than
	   rc and errno.  */
	if (*data).cancel_signal_received == 0
	  /* Now, wait for SELECT_SIGNAL, unless pselect was interrupted
	     and the signal has already been delivered.  The Emacs thread
	     will always send this signal after read_var is triggered or
	     the UI thread has sent an event.  */
	  { sigwait (&raw const waitset, &raw mut sig); }

	/* Signal the Emacs thread that pselect is done.  If read_var
	   was signaled by android_write_event, event_queue.mutex could
	   still be locked, so this must come before.  */
	sem_post (&raw mut (*data).select_sem);
      }
  }

  return std::ptr::null_mut ();
}

/* C 672-693: `#if __ANDROID_API__ >= 21`.  C's `static void android_handle_poll_signal (int,
   siginfo_t *, void *)` is assigned to `sa_sigaction`, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (not (feature = "android_api_lt_21"))]
unsafe extern "C" fn android_handle_poll_signal (sig: c_int, siginfo: *mut siginfo_t,
						 arg: *mut c_void)          // C 674-691
{
  let mut queue: *mut android_thread_event_queue;

  /* Although pthread_getspecific is not AS-safe, its implementation has
     been verified to be safe to invoke from a single handler called
     within pselect in a controlled manner ...  */
  #[cfg (feature = "threads_enabled")]
  {
    queue = pthread_getspecific (poll_thread_internal) as *mut android_thread_event_queue;
  }
  #[cfg (not (feature = "threads_enabled"))]
  {
    queue = &raw mut event_queue.thread;
  }
  (*queue).cancel_signal_received = 1;
}

/* Semaphore used to indicate completion of a query.
   This should ideally be defined further down.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut android_query_sem: sem_t = unsafe { std::mem::zeroed () };   // C 697 (tentative)

/* ID of the Emacs thread.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut main_thread_id: pthread_t = 0;                     // C 700 (tentative)

/* Set up the global event queue by initializing the mutex and two
   condition variables, and the linked list of events.  This must be
   called before starting the Emacs thread.  Also, initialize the
   thread used to run pselect.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_events ()                            // C 710-764
{
  #[cfg (not (feature = "android_api_lt_21"))]
  let mut sa: sigaction = unsafe { std::mem::zeroed () };

  if pthread_mutex_init (&raw mut event_queue.mutex, std::ptr::null ()) != 0
    {
      let __func__: *const c_char = c"android_init_events".as_ptr ();
      __android_log_print (ANDROID_LOG_FATAL, __func__,
			   c"pthread_mutex_init: %s".as_ptr (), strerror (*errno ()));
      emacs_abort ();
    }

  if pthread_cond_init (&raw mut event_queue.read_var, std::ptr::null ()) != 0
    {
      let __func__: *const c_char = c"android_init_events".as_ptr ();
      __android_log_print (ANDROID_LOG_FATAL, __func__,
			   c"pthread_cond_init: %s".as_ptr (), strerror (*errno ()));
      emacs_abort ();
    }

  event_queue.events.next = &raw mut event_queue.events;
  event_queue.events.last = &raw mut event_queue.events;

  main_thread_id = pthread_self ();

  /* C 738-751: `#if __ANDROID_API__ >= 21` - before any event threads are initialized,
     guarantee that the disposition of SELECT_SIGNAL is correct.  */
  #[cfg (not (feature = "android_api_lt_21"))]
  {
    sigfillset (&raw mut sa.sa_mask);
    sa.sa_sigaction = Some (android_handle_poll_signal);
    sa.sa_flags = SA_SIGINFO;
    if sigaction (SELECT_SIGNAL, &raw const sa, std::ptr::null_mut ()) != 0
      {
	let __func__: *const c_char = c"android_init_events".as_ptr ();
	__android_log_print (ANDROID_LOG_FATAL, __func__,
			     c"sigaction: %s".as_ptr (), strerror (*errno ()));
	emacs_abort ();
      }
  }
  /* C 752-763: `#ifndef THREADS_ENABLED` / `#else`.  */
  #[cfg (not (feature = "threads_enabled"))]
  {
    android_init_thread_events (&raw mut event_queue.thread);
  }
  #[cfg (feature = "threads_enabled")]
  {
    if pthread_key_create (&raw mut poll_thread, Some (android_finalize_thread_events)) != 0
	|| pthread_key_create (&raw mut poll_thread_internal, None) != 0
      {
	let __func__: *const c_char = c"android_init_events".as_ptr ();
	__android_log_print (ANDROID_LOG_FATAL, __func__,
			     c"pthread_key_create: %s".as_ptr (), strerror (*errno ()));
	emacs_abort ();
      }
  }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_pending () -> c_int                // C 766-776
{
  let mut i: c_int;

  pthread_mutex_lock (&raw mut event_queue.mutex);
  i = event_queue.num_events;
  pthread_mutex_unlock (&raw mut event_queue.mutex);

  return i;
}

/* Wait for events to become available synchronously.  Return once an
   event arrives.  Also, reply to the UI thread whenever it requires a
   response.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_wait_event ()                      // C 782-801
{
  /* Run queries from the UI thread to the Emacs thread.  */
  android_check_query ();

  pthread_mutex_lock (&raw mut event_queue.mutex);

  /* Wait for events to appear if there are none available to
     read.  */
  if event_queue.num_events == 0
    { pthread_cond_wait (&raw mut event_queue.read_var, &raw mut event_queue.mutex); }

  pthread_mutex_unlock (&raw mut event_queue.mutex);

  /* Check for queries again.  If a query is sent after the call to
     `android_check_query' above, `read_var' will be signaled.  */
  android_check_query ();
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_next_event (event_return: *mut android_event)      // C 803-832
{
  let mut container: *mut android_event_container;

  pthread_mutex_lock (&raw mut event_queue.mutex);

  /* Wait for events to appear if there are none available to
     read.  */
  if event_queue.num_events == 0
    { pthread_cond_wait (&raw mut event_queue.read_var, &raw mut event_queue.mutex); }

  /* Obtain the event from the end of the queue.  */
  container = event_queue.events.last;
  eassert! (container != &raw mut event_queue.events);

  /* Remove the event from the queue and copy it to the caller
     supplied buffer.  */
  (*(*container).last).next = (*container).next;
  (*(*container).next).last = (*container).last;
  *event_return = (*container).event;
  event_queue.num_events -= 1;

  /* Free the container.  */
  free (container as *mut c_void);

  /* Unlock the queue.  */
  pthread_mutex_unlock (&raw mut event_queue.mutex);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_check_if_event
  (event_return: *mut android_event,
   predicate: Option<unsafe extern "C" fn (*mut android_event, *mut c_void) -> bool>,
   arg: *mut c_void) -> bool                              // C 834-866
{
  let mut container: *mut android_event_container;

  pthread_mutex_lock (&raw mut event_queue.mutex);

  /* Loop over each event.  */
  container = event_queue.events.last;
  while container != &raw mut event_queue.events
    {
      /* See if the predicate matches.  */
      if (predicate.unwrap ()) (&raw mut (*container).event, arg)
	{
	  /* Copy out the event and return true.  */
	  *event_return = (*container).event;
	  event_queue.num_events -= 1;

	  /* Unlink container.  */
	  (*(*container).last).next = (*container).next;
	  (*(*container).next).last = (*container).last;
	  free (container as *mut c_void);
	  pthread_mutex_unlock (&raw mut event_queue.mutex);
	  return true;
	}

      container = (*container).last;
    }

  pthread_mutex_unlock (&raw mut event_queue.mutex);
  return false;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_write_event (event: *mut android_event)   // C 868-909
{
  let mut container: *mut android_event_container;

  container = malloc (std::mem::size_of::<android_event_container> ())
		as *mut android_event_container;

  if container.is_null ()
    { return; }

  /* If the event queue hasn't been initialized yet, return false.  */
  if event_queue.events.next.is_null ()
    { return; }

  pthread_mutex_lock (&raw mut event_queue.mutex);
  (*container).next = event_queue.events.next;
  (*container).last = &raw mut event_queue.events;
  (*(*container).next).last = container;
  (*(*container).last).next = container;
  (*container).event = *event;
  event_queue.num_events += 1;
  pthread_cond_broadcast (&raw mut event_queue.read_var);
  pthread_mutex_unlock (&raw mut event_queue.mutex);

  /* Now set pending_signals to true, and raise SIGIO to interrupt any
     ongoing reads if the event is important.  */
  pending_signals = true;

  match (*event).r#type
    {
      /* Key press and window action events are considered important,
	 as they either end up quitting or asking for responses to the
	 IME.  */
      ANDROID_KEY_PRESS | ANDROID_WINDOW_ACTION =>
	{ kill (getpid (), SIGIO); }
      _ => {}
    }
}

/* Whether or not the UI thread has been waiting for a significant
   amount of time for a function to run in the main thread, and Emacs
   should answer the query ASAP.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut android_urgent_query: bool = false;             // C 916 (tentative)

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_select (nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set,
			      exceptfds: *mut fd_set,
			      timeout: *mut timespec) -> c_int      // C 918-1037
{
  let mut nfds_return: c_int;
  let mut nevents: c_int;
  #[cfg (feature = "android_api_lt_21")]
  static mut byte: c_char = 0;                             // C 924 (tentative, i.e. zero)
  let mut data: *mut android_thread_event_queue;

  /* When threads are enabled, the following is executed before the
     global lock is released.  */
  #[cfg (not (feature = "threads_enabled"))]
  {
    android_before_select ();
  }

  pthread_mutex_lock (&raw mut event_queue.mutex);

  if event_queue.num_events != 0
    {
      /* Zero READFDS, WRITEFDS and EXCEPTFDS, lest the caller
	 mistakenly interpret this return value as indicating that an
	 inotify file descriptor is readable, and try to poll an
	 unready one.  */

      if ! readfds.is_null ()
	{ FD_ZERO (readfds); }

      if ! writefds.is_null ()
	{ FD_ZERO (writefds); }

      if ! exceptfds.is_null ()
	{ FD_ZERO (exceptfds); }
      pthread_mutex_unlock (&raw mut event_queue.mutex);
      return 1;
    }

  nfds_return = 0;

  data = android_get_poll_thread ();

  pthread_mutex_lock (&raw mut (*data).select_mutex);
  (*data).select_nfds = nfds;
  (*data).select_readfds = readfds;
  (*data).select_writefds = writefds;
  (*data).select_exceptfds = exceptfds;
  (*data).select_timeout = timeout;
  pthread_mutex_unlock (&raw mut (*data).select_mutex);

  /* Release the select thread.  */
  sem_post (&raw mut (*data).start_sem);

  /* Start waiting for the event queue condition to be set.  */
  pthread_cond_wait (&raw mut event_queue.read_var, &raw mut event_queue.mutex);

  /* C 973-982: `#if __ANDROID_API__ >= 21` / `#else` - interrupt the select thread (now, in case
     it's still in pselect; or by writing to the select pipe).  */
  #[cfg (not (feature = "android_api_lt_21"))]
  {
    pthread_kill ((*data).select_thread, SELECT_SIGNAL);
  }
  #[cfg (feature = "android_api_lt_21")]
  {
    if write ((*data).select_pipe[1], &raw mut byte as *const c_void, 1) != 1
      {
	let __func__: *const c_char = c"android_select".as_ptr ();
	__android_log_print (ANDROID_LOG_FATAL, __func__,
			     c"write: %s".as_ptr (), strerror (*errno ()));
      }
  }

  /* Are there any events in the event queue?  */
  nevents = event_queue.num_events;
  pthread_mutex_unlock (&raw mut event_queue.mutex);

  /* Wait for pselect to return in any case.  This must be done with the
     event queue mutex unlocked.  Otherwise, the pselect thread can hang
     if it tries to lock the event queue mutex to signal read_var after
     the UI thread has already done so.  */
  while sem_wait (&raw mut (*data).select_sem) < 0 {}

  /* If there are now events in the queue, return 1.  */
  if nevents != 0
    { nfds_return = 1; }

  /* Add the return value of pselect if it has also discovered ready
     file descriptors.  */

  if (*data).select_rc >= 0
    { nfds_return += (*data).select_rc; }
  else if nfds_return == 0
    /* If pselect was interrupted and nfds_return is 0 (meaning that no
       events have been read), indicate that an error has taken
       place.  */
    { nfds_return = (*data).select_rc; }

  if (*data).select_rc < 0 && nfds_return >= 0
    {
      /* Clear the file descriptor sets if events will be delivered
	 but no file descriptors have become ready to prevent the
	 caller from misinterpreting a non-zero return value.  */

      if ! readfds.is_null ()
	{ FD_ZERO (readfds); }

      if ! writefds.is_null ()
	{ FD_ZERO (writefds); }

      if ! exceptfds.is_null ()
	{ FD_ZERO (exceptfds); }
    }

  /* This is to shut up process.c when pselect gets EINTR.  */
  if nfds_return < 0
    { *errno () = EINTR; }

  #[cfg (not (feature = "threads_enabled"))]
  {
    /* Now check for and run anything the UI thread wants to run in the
       main thread.  */
    android_check_query ();
  }

  return nfds_return;
}

/* **Slice boundary.**  C 1039 is the form feed that separates this function from the next live
   definition, `android_run_debug_thread` (C 1041-1078).  Everything from C 1041 to C 7594 is the
   remainder: it holds `android_user_full_name`, `android_is_special_directory`, the JNI
   `NATIVE_NAME` entry points, the two file-scope vectors C guards with `__aarch64__`, and C
   7519's `#else` stub arm (`ANDROID_STUBIFY`).  The ledger's last range row names it.  */


/* ============================================================================================
   **Slice 2 (C 1038-1869).**  `android_run_debug_thread` (C 1041-1073) through
   `android_init_emacs_pixmap` (C 1839-1868), in C's order.  The next live definition is
   `android_init_graphics_point` (C 1870-1898).  `docs/ports/android.md` carries this slice's
   C line-range rows, the readings and the deviations.  Everything below is inside C 61's
   `#ifndef ANDROID_STUBIFY` real arm, so every item carries
   `all (feature = "have_android", feature = "not_android_stubify")`; C's
   `#ifdef THREADS_ENABLED` at C 1495-1499 carries `feature = "threads_enabled"` on its live arm
   (C's `#ifdef` has no `#else` arm: the `#endif` closes it directly).
   ============================================================================================ */

/* -------------------------------------------------------------------------------------------
   Slice 2's declaration segment (PORTING.md P-88 ①/③): the names this slice takes from
   headers whose mirror is absent or does not carry them.  Each declaration keeps C's
   prototype in a `/// C: …` line (P-55).
   ------------------------------------------------------------------------------------------- */

/* C 37's `<stdio.h>`.  `lib/include/stdio.rs` is the frozen target's *system* header mirror
   (P-56) and carries neither entry this slice names; the gnulib *template* mirror
   `lib/include/stdio.in.rs` does carry both (its 238 and 248), but no unit reaches it
   (`rg -ln 'stdio.in.rs' femacs/src femacs/lib` is empty), so the unit declares them here
   (the `sysdep.rs` in-unit shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `<stdio.h>`: `FILE *fdopen (int, const char *);`
  pub fn fdopen (fd: c_int, mode: *const c_char) -> *mut FILE;
  /// C: `<stdio.h>`: `ssize_t getline (char **, size_t *, FILE *);`
  pub fn getline (lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
}

/* C 37's `<string.h>`: the mirror carries `strlen`/`strncmp`/`memset`/`stpcpy`/`strerror` but
   no `strdup` declaration (`string.in.rs` 194-199 keeps only the dead `_strdup` redirect), so
   the unit declares it (the `sysdep.rs` 727 / `lib-src/emacsclient.rs` 302 shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `<string.h>`: `char *strdup (const char *);`
  pub fn strdup (s: *const c_char) -> *mut c_char;
}

/* C 37's chain `<stdlib.h>`: `lib/include/stdlib.rs` is the frozen target's system header
   mirror and carries `malloc`/`realloc`/`calloc`/`free`/`strtod` but not `setenv`; the
   `stdlib.in.rs` entry (its 248) is unreachable the same way, so the unit declares it (the
   `sysdep.rs` 956 shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `<stdlib.h>`: `int setenv (const char *, const char *, int);`
  pub fn setenv (name: *const c_char, value: *const c_char, replace: c_int) -> c_int;
}

/* C 29's `<libgen.h>` has no mirror anywhere in the tree (`ls femacs/lib/include | grep libgen`
   is empty; the declaration is the build's system one), so `dirname` is declared here - this
   unit is the first of the port to name it (the `sysdep.rs` `<pthread.h>` block shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `<libgen.h>`: `char *dirname (char *);`
  pub fn dirname (path: *mut c_char) -> *mut c_char;
}

/* C 51's `lisp.h` 4247 declares `hexbuf_digest` and the `lisp.rs` mirror does not carry it
   (`emacs.rs` 192-193 declares the same prototype in its own unit), so this unit declares it
   too (the `sysdep.rs` shape; P-88 ⑤'s "each consumer declares it" reading).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: lisp.h 4247: `extern void hexbuf_digest (char *, void const *, int);`
  pub fn hexbuf_digest (buf: *mut c_char, data: *const c_void, size: c_int);
}

/* C's `<config.h>` values this slice names.  `config.rs` carries only the generated
   configuration keys, so the two report strings and `DIRECTORY_SEP` live here (the
   `dispnew.rs` 70/170 macro shape, the `fileio.rs` 309-312 constant shape).  C 1511-1513 puts
   `PACKAGE_STRING`/`PACKAGE_BUGREPORT` between string literals, and `concat!` expands a
   `macro_rules!` call, so those two are macros whose bodies are the frozen values
   (`build/emacs-ref/src/config.h` 2205/2199); `DIRECTORY_SEP` is a constant because C 1117
   needs it as a pattern.  `PACKAGE_STRING` is what *this* program calls itself (it is the
   string C 1511-1513 logs at startup), so it names the port (PORTING.md 0.1-19's name half,
   status 3.433); the frozen configuration's own value is a measurement and stays in
   `docs/baseline/config/`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! PACKAGE_STRING { () => { "femacs 32.0.50" }; }
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! PACKAGE_BUGREPORT { () => { "https://github.com/liuchong/femacs/issues" }; }
/* C 1116-1117's two `case` labels need patterns; `case '/'` cannot be written `b'/' as c_char`
   (a Rust pattern holds no cast - the `data.rs` 2938 shape is a value site), so the character
   is the constant `config.h` 120 defines for it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const DIRECTORY_SEP: c_char = b'/' as c_char;

/* C 344's `NATIVE_NAME` is an excluded macro (`docs/ports/excluded-macros.txt`: Rust cannot
   paste identifiers) whose replacement is the definition site's
   `#[unsafe(export_name = "Java_org_femacs_EmacsNative_<name>")]`.  **The pasted name is our
   program's own**: C's macro prefixes the *reference's* own Java package (its body is in
   `src/android.h` 344), while the class this tree loads is `org.femacs.EmacsNative`
   (PORTING.md 0.1-19's name half, status 3.503).  C 340-342's
   `ANDROID_DELETE_LOCAL_REF` is excluded for the old "`JNINativeInterface` has no mirror"
   reason that `include/jni.rs` has since answered, so the unit defines its expansion under C's
   own name (the androidselect.rs 121-129 shape; P-86 ⑪).  `JNIEXPORT`/`JNICALL` (jni.h) have
   no mirror and no Rust text: the `pub unsafe extern "C"` signature carries them.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! ANDROID_DELETE_LOCAL_REF {
  ($r:expr) => { ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, $r) };
}

/* C 1038-1040: the blank line, C 1039's form feed and the blank that follow `android_select`;
   the previous slice's text ends there.  */

/* C 1041: `static void *android_run_debug_thread (void *data)`.  C passes its address to
   `pthread_create` (C 1535), so the item is an `unsafe extern "C" fn` (the
   `android_run_select_thread` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_run_debug_thread (data: *mut c_void) -> *mut c_void
{
  let mut file: *mut FILE;
  let mut fd: c_int;
  let mut line: *mut c_char;
  let mut n: size_t;

  /* C leaves these four uninitialised (PORTING.md 1.5).  */
  file = std::ptr::null_mut ();
  fd = 0;
  line = std::ptr::null_mut ();
  n = 0;

  /* Set the name of this thread's LWP for debugging purposes.  */
  android_set_task_name (c"`android_debug'".as_ptr ());

  fd = data as intptr_t as c_int;
  file = fdopen (fd, c"r".as_ptr ());

  if file.is_null ()
    { return std::ptr::null_mut (); }

  line = std::ptr::null_mut ();

  while true
    {
      if getline (&raw mut line, &raw mut n, file) < 0
	{
	  free (line as *mut c_void);
	  break;
	}

      let __func__: *const c_char = c"android_run_debug_thread".as_ptr ();
      __android_log_print (ANDROID_LOG_INFO, __func__, c"%s".as_ptr (), line);
    }

  fclose (file);
  return std::ptr::null_mut ();
}

/* C 1074-1079: the blank line, C 1075's form feed, the blank, the two-line comment
   `/* Intercept USER_FULL_NAME and return something that makes sense if pw->pw_gecos is
   NULL.  */` and the blank after it.  */

/* C 1081: `char *android_user_full_name (struct passwd *pw)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_user_full_name (pw: *mut passwd) -> *mut c_char
{
  /* C 1083's `#ifdef HAVE_STRUCT_PASSWD_PW_GECOS` is 1 in the frozen configuration
     (`build/emacs-ref/src/config.h` 1510), so C 1084-1086's arm is the live one and C 1089's
     `return "Android user";` (the `#else` arm) is dead on this target (the dired.rs
     `dirent_type` shape, P-51).  */
  if (*pw).pw_gecos.is_null ()
    { return c"Android user".as_ptr () as *mut c_char; }

  return (*pw).pw_gecos;
}

/* C 1092-1098: the blank line, C 1093's form feed, the blank, the three-line comment
   `/* Return whether or not the specified file NAME designates a file in the directory DIR,
   which should be an absolute file name.  NAME must be in canonical form.  */` and the blank
   after it.  */

/* C 1100: `bool android_is_special_directory (const char *name, const char *dir)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_is_special_directory (mut name: *const c_char, dir: *const c_char) -> bool
{
  let len: size_t;

  /* Compare up to strlen (DIR) bytes of NAME with DIR.  */

  len = strlen (dir);
  if strncmp (name, dir, len) != 0
    { return false; }

  /* Now see if the character of NAME after len is either a directory
     separator or a terminating NULL.  */

  name = name.add (len);
  /* C 1114-1119's `switch (*name)`: two labels and no `default`, so the `_` arm is C's
     fall-through out of the switch (the android_write_event shape, PORTING.md 0.3).  */
  match *name
    {
      0 => return true,			/* C 1116: `case '\0'`.  */
      DIRECTORY_SEP => return true,	/* C 1117: `case '/'`.  */
      _ => {}
    }

  /* The file name doesn't match.  */
  return false;
}

/* C 1124-1360: C 1124's blank line, then `#if 0` (C 1125) … `#endif /* 0 */` (C 1360) around
   two dead definitions - `android_url_encode` (C 1133-1225, a URL-encoder over a `restrict`
   byte range whose multibyte-length and `STRING_CHAR` calls are themselves commented out at
   C 1154 and C 1161) and `android_get_content_name` (C 1232-1358, the `/content' file name to
   `content://' URI translator; its two `android_url_encode` calls are C 1316 and C 1340, and
   nothing calls it - androidvfs.c 2957 has a different, static function of the same name).  C's own
   `#if 0` is dead in every configuration and `tools/extract-abi.py`'s `dead_branch_map` reads
   the literal `#if 0` as the reference's own dead region, so neither definition is in the
   inventory and neither owns a `- missing …` line: the port records the region instead of
   carrying it (the character.rs 91-97 / cm.rs 54-100 precedent).  */

/* C 1361-1364: the blank line and the comment `/* Return the current user's ``home''
   directory, which is actually the app data directory on Android.  */`.  */

/* C 1365: `const char *android_get_home_directory (void)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_get_home_directory () -> *const c_char
{
  return android_files_dir as *const c_char;
}

/* C 1370-1374: the blank line and the comment `/* Return the name of the file behind a file
   descriptor FD by reading /proc/self/fd/.  Value is allocated memory holding the file name
   upon success, and 0 upon failure.  */`.  */

/* C 1375: `static char *android_proc_name (int fd)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_proc_name (fd: c_int) -> *mut c_char
{
  /* C 1378-1379: `char format[sizeof "/proc/self/fd/" + INT_STRLEN_BOUND (int)]`; the
     literal's `sizeof` is 15 (`/proc/self/fd/` is 14 bytes plus the terminating NUL).  */
  let mut format: [c_char; std::mem::size_of::<[c_char; 15]> () + INT_STRLEN_BOUND! (c_int)] =
    [0; std::mem::size_of::<[c_char; 15]> () + INT_STRLEN_BOUND! (c_int)];
  /* C 1380-1384: the function-local `static struct allocator allocator = { malloc, realloc,
     free, NULL, }` - the type and the object share C's one name, and Rust's type and value
     namespaces keep them apart here too.  */
  static mut allocator: allocator = allocator
    {
      /* Fill the allocator with C library malloc functions.  xmalloc
	 and so aren't thread safe.  */
      allocate: Some (malloc),
      reallocate: Some (realloc),
      free: Some (free),
      die: None,
    };

  sprintf (&raw mut format as *mut c_char, c"/proc/self/fd/%d".as_ptr (), fd);
  return careadlinkat (AT_FDCWD, &raw const format as *const c_char, std::ptr::null_mut (), 0,
		       &raw const allocator, Some (readlinkat));
}

/* C 1390-1403: the blank line and the thirteen-line comment `/* Try to guarantee the
   existence of the `lib' directory within the parent directory of the application files
   directory. … */`.  */

/* C 1404: `static void android_create_lib_link (void)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_create_lib_link ()
{
  let mut filename: *mut c_char;
  let mut lib_directory: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
  let mut fd: c_int;

  /* C leaves `filename` and `fd` uninitialised (PORTING.md 1.5).  */
  filename = std::ptr::null_mut ();
  fd = 0;

  /* C 1407-1447's `goto success` / `goto failure` and the `success:`/`failure:` labels become
     the two labelled blocks below (PORTING.md 2.2, the android_set_task_name shape): `goto
     failure` is `break 'failure` (straight to C's `failure: return;`) and `goto success` is
     `break 'success` (out of the inner block, to `close (fd)`); C's fall-through from
     `close (fd)` to the `failure:` tail is the `return;` after the outer block.  */

  'failure: {
    /* Find the directory containing the files directory.  */
    filename = dirname (android_files_dir);
    if filename.is_null ()
      { break 'failure; }

    /* Now make `lib_directory' the name of the library directory
       within.  */
    snprintf (&raw mut lib_directory as *mut c_char, PATH_MAX as usize,
	      c"%s/lib".as_ptr (), filename);

    /* Try to open this directory.  */
    fd = open (&raw const lib_directory as *const c_char, O_DIRECTORY);

    'success: {
      /* If the directory can be opened normally, close it and return
	 now.  */
      if fd >= 0
	{ break 'success; }

      /* Try to unlink the directory in case it's a dangling symbolic
	 link.  */
      unlink (&raw const lib_directory as *const c_char);

      /* Otherwise, try to symlink lib_directory to the actual library
	 directory.  */

      if symlink (android_lib_dir, &raw const lib_directory as *const c_char) != 0
	{
	  /* Print a warning message if creating the link fails.  */
	  let __func__: *const c_char = c"android_create_lib_link".as_ptr ();
	  __android_log_print (ANDROID_LOG_WARN, __func__,
			       c"Failed to create symbolic link from application library directory `%s' to its actual location at `%s'".as_ptr (),
			       &raw const lib_directory as *const c_char, android_files_dir);
	}
    }

    /* success: */
    close (fd);
  }

  /* failure: */
  return;
}

/* C 1448-1460: the blank line, C 1449's form feed, the blank, the comment `/* JNI functions
   called by Java.  */`, the blank and C 1453-1459's `#ifdef __clang__` / `#pragma clang
   diagnostic push/ignored` / `#else` / `#pragma GCC diagnostic …` / `#endif` pair.  A pragma
   has no Rust counterpart and the port emits none either way: the two arms differ only in
   which compiler's diagnostic is pushed, and the push has no counterpart (the process.rs 340 /
   timefns.rs 79-81 precedent).  */

/* C 1461-1462: `JNIEXPORT jstring JNICALL NATIVE_NAME (getFingerprint) (JNIEnv *env, jobject
   object)`; C's paste prefixes the reference's own Java package (`src/android.h` 344) and the port
   exports its own class's name, `Java_org_femacs_EmacsNative_getFingerprint` (PORTING.md 0.1-19's name half /
   status 3.503 - the macro's replacement is written at each definition site,
   `docs/ports/excluded-macros.txt`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_getFingerprint")]
pub unsafe extern "C" fn getFingerprint (env: *mut JNIEnv, object: jobject) -> jstring
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();

  /* C 1466's `char buffer[sizeof fingerprint * 2 + 1]`: `fingerprint` is `unsigned char[32]`
     (lib/include/fingerprint.rs 26), so the array is 65 `char`s.  */
  let mut buffer: [c_char; std::mem::size_of::<[c_uchar; 32]> () * 2 + 1] =
    [0; std::mem::size_of::<[c_uchar; 32]> () * 2 + 1];

  memset (&raw mut buffer as *mut c_void, 0, std::mem::size_of_val (&buffer));
  hexbuf_digest (&raw mut buffer as *mut c_char, (&raw const fingerprint) as *const c_void,
		 std::mem::size_of::<[c_uchar; 32]> () as c_int);

  return ((*(*env)).NewStringUTF.unwrap ()) (env, &raw const buffer as *const c_char);
}

/* C 1474: the blank line between the two entry points.  */


/* C 1475-1486: `JNIEXPORT void JNICALL NATIVE_NAME (setEmacsParams) (JNIEnv *env, jobject
   object, jobject local_asset_manager, jobject files_dir, jobject libs_dir, jobject cache_dir,
   jfloat pixel_density_x, jfloat pixel_density_y, jfloat scaled_density, jint ui_mode, jobject
   class_path, jobject emacs_service_object, jint api_level)`; the symbol is C's pasted
   the reference's own paste of `setEmacsParams`; the port's own export below is
   `Java_org_femacs_EmacsNative_setEmacsParams` (status 3.503).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_setEmacsParams")]
pub unsafe extern "C" fn setEmacsParams (env: *mut JNIEnv, object: jobject,
					 local_asset_manager: jobject,
					 files_dir: jobject, libs_dir: jobject,
					 cache_dir: jobject,
					 pixel_density_x: jfloat,
					 pixel_density_y: jfloat,
					 scaled_density: jfloat,
					 ui_mode: jint,
					 class_path: jobject,
					 emacs_service_object: jobject,
					 api_level: jint)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();

  let mut pipefd: [c_int; 2] = [0, 0];
  let mut thread: pthread_t;
  let mut java_string: *const c_char;
  let mut statb: stat;

  /* C leaves `thread`, `java_string` and `statb` uninitialised (PORTING.md 1.5).  */
  thread = 0;
  java_string = std::ptr::null ();
  statb = unsafe { std::mem::zeroed () };

  /* C 1495-1499: `#ifdef THREADS_ENABLED` / save the Java VM / `#endif`.  C's `#ifdef` has no
     `#else` arm - the `#endif` closes it directly - so there is no second arm to carry and the
     arm below is the whole of it.  */
  #[cfg (feature = "threads_enabled")]
  {
    /* Save the Java VM.  */
    if ((*(*env)).GetJavaVM.unwrap ()) (env, &raw mut android_jvm) != 0
      { emacs_abort (); }
  }

  /* Set the Android API level early, as it is used by
     `android_vfs_init'.  */
  android_api_level = api_level;

  /* This function should only be called from the main thread.  */
  /* C's three `jfloat` parameters are assigned to the file's `double` objects: the float to
     double promotion is spelled out (Rust has no implicit conversion - PORTING.md 0.3).  */
  android_pixel_density_x = pixel_density_x as f64;
  android_pixel_density_y = pixel_density_y as f64;
  android_scaled_pixel_density = scaled_density as f64;
  android_ui_mode = ui_mode;

  /* C 1511-1513's format is the adjacent-literal concatenation `"Initializing " PACKAGE_STRING
     "...\nPlease report bugs to " PACKAGE_BUGREPORT ".  Thanks.\n"`, which the preprocessor
     joins into one literal; `concat!` expands a `macro_rules!` call, so the port keeps C's two
     `<config.h>` object macros as macros above and its pieces verbatim (the dispnew.rs 70/170
     shape).  */
  let __func__: *const c_char = c"setEmacsParams".as_ptr ();
  __android_log_print (ANDROID_LOG_INFO, __func__,
		       concat! ("Initializing ", PACKAGE_STRING! (),
				"...\nPlease report bugs to ", PACKAGE_BUGREPORT! (),
				".  Thanks.\n").as_ptr () as *const c_char);

  if ! emacs_service_object.is_null ()
    {
      /* Create a pipe and duplicate it to stdout and stderr.  Next,
	 make a thread that prints stderr to the system log.

         Notice that this function is called in one of two ways.  The
         first is when Emacs is being started as a GUI application by
         the system, and the second is when Emacs is being started by
         libandroid-emacs.so as an ordinary noninteractive Emacs.

         In the second case, stderr is usually connected to a PTY, so
         this is unnecessary.  */

      if pipe2 (pipefd.as_mut_ptr (), O_CLOEXEC) < 0
	{ emacs_abort (); }

      if dup2 (pipefd[1], 2) < 0
	{ emacs_abort (); }
      close (pipefd[1]);

      if pthread_create (&raw mut thread, std::ptr::null (), Some (android_run_debug_thread),
			 pipefd[0] as intptr_t as *mut c_void) != 0
	{ emacs_abort (); }
    }

  /* Now set the path to the site load directory.  */

  java_string = ((*(*env)).GetStringUTFChars.unwrap ()) (env, files_dir as jstring,
							 std::ptr::null_mut ());

  if java_string.is_null ()
    { emacs_abort (); }

  android_files_dir = strdup (java_string);

  if android_files_dir.is_null ()
    { emacs_abort (); }

  ((*(*env)).ReleaseStringUTFChars.unwrap ()) (env, files_dir as jstring, java_string);

  java_string = ((*(*env)).GetStringUTFChars.unwrap ()) (env, libs_dir as jstring,
							 std::ptr::null_mut ());

  if java_string.is_null ()
    { emacs_abort (); }

  android_lib_dir = strdup (java_string);

  /* C 1564 tests `android_files_dir`, not the object it has just set - and C 1578 does the same
     after setting `android_cache_dir`.  The port keeps C's text (PORTING.md 0.3); the
     `## Deviations` section of the ledger records it as a defect carried over from the
     reference, not introduced here.  */
  if android_files_dir.is_null ()
    { emacs_abort (); }

  ((*(*env)).ReleaseStringUTFChars.unwrap ()) (env, libs_dir as jstring, java_string);

  java_string = ((*(*env)).GetStringUTFChars.unwrap ()) (env, cache_dir as jstring,
							 std::ptr::null_mut ());

  if java_string.is_null ()
    { emacs_abort (); }

  android_cache_dir = strdup (java_string);

  if android_files_dir.is_null ()
    { emacs_abort (); }

  ((*(*env)).ReleaseStringUTFChars.unwrap ()) (env, cache_dir as jstring, java_string);

  if ! class_path.is_null ()
    {
      java_string = ((*(*env)).GetStringUTFChars.unwrap ()) (env, class_path as jstring,
							     std::ptr::null_mut ());

      if java_string.is_null ()
	{ emacs_abort (); }

      android_class_path = strdup (java_string);

      if android_class_path.is_null ()
	{ emacs_abort (); }

      ((*(*env)).ReleaseStringUTFChars.unwrap ()) (env, class_path as jstring, java_string);
    }

  /* Derive the installation date from the modification time of the
     file constitituing the class path.  */

  emacs_installation_time = invalid_timespec ();

  if ! class_path.is_null ()
    {
      if stat (android_class_path, &raw mut statb) == 0
	{ emacs_installation_time = get_stat_mtime (&raw mut statb); }
    }

  /* Calculate the site-lisp path.  */

  android_site_load_path = malloc ((PATH_MAX + 1) as usize) as *mut c_char;

  if android_site_load_path.is_null ()
    { emacs_abort (); }

  android_game_path = malloc ((PATH_MAX + 1) as usize) as *mut c_char;

  if android_game_path.is_null ()
    { emacs_abort (); }

  snprintf (android_site_load_path, PATH_MAX as usize, c"%s/site-lisp".as_ptr (),
	    android_files_dir);
  snprintf (android_game_path, PATH_MAX as usize, c"%s/scores".as_ptr (), android_files_dir);

  __android_log_print (ANDROID_LOG_INFO, __func__,
		       concat! ("Site-lisp directory: %s\n",
				"Files directory: %s\n",
				"Native code directory: %s\n",
				"Game score path: %s\n",
				"Class path: %s\n").as_ptr () as *const c_char,
		       android_site_load_path,
		       android_files_dir,
		       android_lib_dir, android_game_path,
		       (if ! android_class_path.is_null ()
			{ android_class_path }
			else
			{ c"None".as_ptr () as *mut c_char }));

  if ! android_class_path.is_null ()
    /* Set EMACS_CLASS_PATH to the class path where
       EmacsNoninteractive can be found.  */
    { setenv (c"EMACS_CLASS_PATH".as_ptr (), android_class_path, 1); }

  /* If the system is Android 5.0 or later, set LANG to en_US.utf8,
     which is understood by the C library.  In other instances set it
     to C, a meaningless value, for good measure.  */

  if ! emacs_service_object.is_null ()
    {
      if api_level >= 21
	{ setenv (c"LANG".as_ptr (), c"en_US.utf8".as_ptr (), 1); }
      else
	{ setenv (c"LANG".as_ptr (), c"C".as_ptr (), 1); }
    }

  /* Make a reference to the Emacs service.  */

  if ! emacs_service_object.is_null ()
    {
      emacs_service = ((*(*env)).NewGlobalRef.unwrap ()) (env, emacs_service_object);

      if emacs_service.is_null ()
	{ emacs_abort (); }

      /* If the service is set this Emacs is being initialized as part
	 of the Emacs application itself.

         Try to create a symlink from where scripts expect Emacs to
         place its library files to the directory that actually holds
         them; earlier versions of Android used to do this
         automatically, but that feature has been removed.  */

      android_create_lib_link ();
    }

  /* Set up events.  */
  android_init_events ();

  /* Set up the Android virtual filesystem layer.  */
  android_vfs_init (env, local_asset_manager);

  /* OK, setup is now complete.  The caller may call initEmacs
     now.  */
}

/* C 1688-1689: `JNIEXPORT jobject JNICALL NATIVE_NAME (getProcName) (JNIEnv *env, jobject
   object, jint fd)`; C's paste prefixes the reference's own package; the port exports
   `Java_org_femacs_EmacsNative_getProcName` (status 3.503).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_getProcName")]
pub unsafe extern "C" fn getProcName (env: *mut JNIEnv, object: jobject, fd: jint) -> jobject
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();

  let mut buffer: *mut c_char;
  let mut length: size_t;
  let mut array: jbyteArray;

  /* C leaves all three uninitialised (PORTING.md 1.5).  */
  buffer = std::ptr::null_mut ();
  length = 0;
  array = std::ptr::null_mut ();

  /* C 1706's `goto finish` and C 1711's `finish:` label become the labelled block below
     (PORTING.md 2.2, the android_set_task_name shape); C's `finish:` tail is the one copy
     after it.  */
  'finish: {
    buffer = android_proc_name (fd);
    if buffer.is_null ()
      { return std::ptr::null_mut (); }

    /* Return a byte array, as Java strings cannot always encode file
       names.  */
    length = strlen (buffer);
    array = ((*(*env)).NewByteArray.unwrap ()) (env, length as jsize);
    if array.is_null ()
      { break 'finish; }

    ((*(*env)).SetByteArrayRegion.unwrap ()) (env, array, 0, length as jsize,
					      buffer as *const jbyte);
  }

  /* finish: */
  free (buffer as *mut c_void);
  return array;
}

/* C 1715-1717: the blank line and the comment `/* Initialize service_class, aborting if
   something goes wrong.  */`.  */

/* C 1718: `static void android_init_emacs_service (void)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_emacs_service ()
{
  let mut old: jclass;

  /* C leaves `old` uninitialised (PORTING.md 1.5).  */
  old = std::ptr::null_mut ();

  service_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
						    c"org/femacs/EmacsService".as_ptr ());
  eassert! (! service_class.class.is_null ());

  old = service_class.class;
  service_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if service_class.class.is_null ()
    { emacs_abort (); }

  /* C 1737-1742's `#define FIND_METHOD …` and C 1836's `#undef`; Rust's `macro_rules!` scope
     ends with the block, so the `#undef` has no counterpart (the androidmenu.rs 176-187
     shape).  The macro's parameter names are C's, so the body's tokens survive.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      service_class.$c_name
	= ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
							  service_class.class,
							  $name, $signature);
      eassert! (! service_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (fill_rectangle, c"fillRectangle".as_ptr (),
                c"(Lorg/femacs/EmacsDrawable;Lorg/femacs/EmacsGC;IIII)V".as_ptr ());
  FIND_METHOD! (fill_polygon, c"fillPolygon".as_ptr (),
                c"(Lorg/femacs/EmacsDrawable;Lorg/femacs/EmacsGC;[Landroid/graphics/Point;)V".as_ptr ());
  FIND_METHOD! (draw_rectangle, c"drawRectangle".as_ptr (),
                c"(Lorg/femacs/EmacsDrawable;Lorg/femacs/EmacsGC;IIII)V".as_ptr ());
  FIND_METHOD! (draw_line, c"drawLine".as_ptr (),
                c"(Lorg/femacs/EmacsDrawable;Lorg/femacs/EmacsGC;IIII)V".as_ptr ());
  FIND_METHOD! (draw_point, c"drawPoint".as_ptr (),
                c"(Lorg/femacs/EmacsDrawable;Lorg/femacs/EmacsGC;II)V".as_ptr ());
  FIND_METHOD! (ring_bell, c"ringBell".as_ptr (), c"(I)V".as_ptr ());
  FIND_METHOD! (query_tree, c"queryTree".as_ptr (), c"(Lorg/femacs/EmacsWindow;)[J".as_ptr ());
  FIND_METHOD! (get_screen_width, c"getScreenWidth".as_ptr (), c"(Z)I".as_ptr ());
  FIND_METHOD! (get_screen_height, c"getScreenHeight".as_ptr (), c"(Z)I".as_ptr ());
  FIND_METHOD! (detect_mouse, c"detectMouse".as_ptr (), c"()Z".as_ptr ());
  FIND_METHOD! (detect_keyboard, c"detectKeyboard".as_ptr (), c"()Z".as_ptr ());
  FIND_METHOD! (name_keysym, c"nameKeysym".as_ptr (), c"(I)Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (browse_url, c"browseUrl".as_ptr (),
                c"(Ljava/lang/String;Z)Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (restart_emacs, c"restartEmacs".as_ptr (), c"()V".as_ptr ());
  FIND_METHOD! (update_ic, c"updateIC".as_ptr (), c"(Lorg/femacs/EmacsWindow;IIII)V".as_ptr ());
  FIND_METHOD! (reset_ic, c"resetIC".as_ptr (), c"(Lorg/femacs/EmacsWindow;I)V".as_ptr ());
  FIND_METHOD! (open_content_uri, c"openContentUri".as_ptr (),
                c"(Ljava/lang/String;ZZZ)I".as_ptr ());
  FIND_METHOD! (check_content_uri, c"checkContentUri".as_ptr (),
                c"(Ljava/lang/String;ZZ)Z".as_ptr ());
  FIND_METHOD! (query_battery, c"queryBattery".as_ptr (), c"()[J".as_ptr ());
  FIND_METHOD! (update_extracted_text, c"updateExtractedText".as_ptr (),
                c"(Lorg/femacs/EmacsWindow;Landroid/view/inputmethod/ExtractedText;I)V".as_ptr ());
  FIND_METHOD! (update_cursor_anchor_info, c"updateCursorAnchorInfo".as_ptr (),
                c"(Lorg/femacs/EmacsWindow;FFFF)V".as_ptr ());
  FIND_METHOD! (get_document_authorities, c"getDocumentAuthorities".as_ptr (),
                c"()[Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (request_directory_access, c"requestDirectoryAccess".as_ptr (), c"()I".as_ptr ());
  FIND_METHOD! (get_document_trees, c"getDocumentTrees".as_ptr (),
                c"(Ljava/lang/String;)[Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (document_id_from_name, c"documentIdFromName".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;)I".as_ptr ());
  FIND_METHOD! (get_tree_uri, c"getTreeUri".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (stat_document, c"statDocument".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;Z)[J".as_ptr ());
  FIND_METHOD! (access_document, c"accessDocument".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;Z)I".as_ptr ());
  FIND_METHOD! (open_document_directory, c"openDocumentDirectory".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;)Landroid/database/Cursor;".as_ptr ());
  FIND_METHOD! (read_directory_entry, c"readDirectoryEntry".as_ptr (),
                c"(Landroid/database/Cursor;)Lorg/femacs/EmacsDirectoryEntry;".as_ptr ());
  FIND_METHOD! (open_document, c"openDocument".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;ZZZ)Landroid/os/ParcelFileDescriptor;".as_ptr ());
  FIND_METHOD! (create_document, c"createDocument".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (create_directory, c"createDirectory".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (delete_document, c"deleteDocument".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)I".as_ptr ());
  FIND_METHOD! (rename_document, c"renameDocument".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)I".as_ptr ());
  FIND_METHOD! (move_document, c"moveDocument".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (valid_authority, c"validAuthority".as_ptr (), c"(Ljava/lang/String;)Z".as_ptr ());
  FIND_METHOD! (external_storage_available, c"externalStorageAvailable".as_ptr (),
                c"()Z".as_ptr ());
  FIND_METHOD! (request_storage_access, c"requestStorageAccess".as_ptr (), c"()V".as_ptr ());
  FIND_METHOD! (cancel_notification, c"cancelNotification".as_ptr (),
                c"(Ljava/lang/String;)V".as_ptr ());
  FIND_METHOD! (relinquish_uri_rights, c"relinquishUriRights".as_ptr (),
                c"(Ljava/lang/String;)V".as_ptr ());
}

/* C 1839: `static void android_init_emacs_pixmap (void)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_emacs_pixmap ()
{
  let mut old: jclass;

  /* C leaves `old` uninitialised (PORTING.md 1.5).  */
  old = std::ptr::null_mut ();

  pixmap_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
						    c"org/femacs/EmacsPixmap".as_ptr ());
  eassert! (! pixmap_class.class.is_null ());

  old = pixmap_class.class;
  pixmap_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if pixmap_class.class.is_null ()
    { emacs_abort (); }

  /* C 1858-1863's `#define FIND_METHOD …` and C 1867's `#undef`.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      pixmap_class.$c_name
	= ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
							  pixmap_class.class,
							  $name, $signature);
      eassert! (! pixmap_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (constructor_mutable, c"<init>".as_ptr (), c"(III)V".as_ptr ());
}

/* **Slice boundary.**  C 1869-1870 are the blank lines that separate this slice from the next
   live definition, `android_init_graphics_point` (C 1871-1898).  Everything from C 1870 to
   C 7594 is the remainder: the seven other `android_init_*` class tables, `NATIVE_NAME
   (initEmacs)` and the rest of the JNI entry points, the two `__aarch64__` NEON regions
   (C 4064-4431) and C 7519's `#else` stub arm (`ANDROID_STUBIFY`).  The ledger's last range
   row names it.  */

/* C 1870-1898: `android_init_graphics_point`.  `static`; only called (C 6079), so a plain
   `unsafe fn`.  C 1889-1894's `#define FIND_METHOD …` and C 1897's `#undef` are the port's
   in-function `macro_rules!` (the `android_init_emacs_pixmap` device of slice 2), and every
   `org/gnu/emacs/` in C's class names and signatures is this project's `org/femacs/`
   (PORTING.md 0.1-19's name half, the `android.rs` 1426-1428 rename).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_graphics_point ()
{
  let mut old: jclass;                                                  // C 1873

  point_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,   // C 1875-1876
						    c"android/graphics/Point".as_ptr ());   // C 1877
  eassert! (! point_class.class.is_null ());                            // C 1878

  old = point_class.class;                                              // C 1880
  point_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,   // C 1881-1883
						       old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);                                      // C 1884

  if point_class.class.is_null ()                                       // C 1886
    { emacs_abort (); }                                                 // C 1887

  /* C 1889-1894's `#define FIND_METHOD …` and C 1897's `#undef`.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      point_class.$c_name
	= ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
							  point_class.class,
							  $name, $signature);
      eassert! (! point_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (constructor, c"<init>".as_ptr (), c"(II)V".as_ptr ());   // C 1896
}

/* C 1900-1928: `android_init_emacs_drawable`.  `static`; only called (C 6084).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_emacs_drawable ()
{
  let mut old: jclass;                                                  // C 1903

  drawable_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,   // C 1905-1906
						    c"org/femacs/EmacsDrawable".as_ptr ());   // C 1907
  eassert! (! drawable_class.class.is_null ());                         // C 1908

  old = drawable_class.class;                                           // C 1910
  drawable_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,   // C 1911-1913
						       old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);                                      // C 1914

  if drawable_class.class.is_null ()                                    // C 1916
    { emacs_abort (); }                                                 // C 1917

  /* C 1919-1924's `#define FIND_METHOD …` and C 1927's `#undef`.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      drawable_class.$c_name
	= ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
							  drawable_class.class,
							  $name, $signature);
      eassert! (! drawable_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (get_bitmap, c"getBitmap".as_ptr (),                     // C 1926
		c"()Landroid/graphics/Bitmap;".as_ptr ());
}

/* C 1930-1992: `android_init_emacs_window`.  `static`; only called (C 6091).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_emacs_window ()
{
  let mut old: jclass;                                                  // C 1933

  window_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,   // C 1935-1936
						    c"org/femacs/EmacsWindow".as_ptr ());   // C 1937
  eassert! (! window_class.class.is_null ());                           // C 1938

  old = window_class.class;                                             // C 1940
  window_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,   // C 1941-1943
						       old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);                                      // C 1944

  if window_class.class.is_null ()                                      // C 1946
    { emacs_abort (); }                                                 // C 1947

  /* C 1949-1954's `#define FIND_METHOD …` and C 1991's `#undef`.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      window_class.$c_name
	= ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
							  window_class.class,
							  $name, $signature);
      eassert! (! window_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (swap_buffers, c"swapBuffers".as_ptr (), c"()V".as_ptr ());   // C 1956
  FIND_METHOD! (toggle_on_screen_keyboard,                              // C 1957-1958
		c"toggleOnScreenKeyboard".as_ptr (), c"(Z)V".as_ptr ());
  FIND_METHOD! (lookup_string, c"lookupString".as_ptr (),                // C 1959
		c"(I)Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (set_fullscreen, c"setFullscreen".as_ptr (), c"(Z)V".as_ptr ());   // C 1960
  FIND_METHOD! (change_window_background, c"changeWindowBackground".as_ptr (),   // C 1961-1962
		c"(I)V".as_ptr ());
  FIND_METHOD! (reparent_to, c"reparentTo".as_ptr (),                    // C 1963-1964
		c"(Lorg/femacs/EmacsWindow;II)V".as_ptr ());
  FIND_METHOD! (map_window, c"mapWindow".as_ptr (), c"()V".as_ptr ());   // C 1965
  FIND_METHOD! (unmap_window, c"unmapWindow".as_ptr (), c"()V".as_ptr ());   // C 1966
  FIND_METHOD! (resize_window, c"resizeWindow".as_ptr (), c"(II)V".as_ptr ());   // C 1967
  FIND_METHOD! (move_window, c"moveWindow".as_ptr (), c"(II)V".as_ptr ());   // C 1968
  FIND_METHOD! (move_resize_window, c"moveResizeWindow".as_ptr (), c"(IIII)V".as_ptr ());   // C 1969
  FIND_METHOD! (make_input_focus, c"makeInputFocus".as_ptr (), c"(J)V".as_ptr ());   // C 1970
  FIND_METHOD! (raise, c"raise".as_ptr (), c"()V".as_ptr ());            // C 1971
  FIND_METHOD! (lower, c"lower".as_ptr (), c"()V".as_ptr ());            // C 1972
  FIND_METHOD! (reconfigure, c"reconfigure".as_ptr (),                   // C 1973
		c"(Lorg/femacs/EmacsWindow;I)V".as_ptr ());
  FIND_METHOD! (get_window_geometry, c"getWindowGeometry".as_ptr (),     // C 1974-1975
		c"()[I".as_ptr ());
  FIND_METHOD! (translate_coordinates, c"translateCoordinates".as_ptr (),   // C 1976-1977
		c"(II)[I".as_ptr ());
  FIND_METHOD! (set_dont_focus_on_map, c"setDontFocusOnMap".as_ptr (), c"(Z)V".as_ptr ());   // C 1978
  FIND_METHOD! (set_dont_accept_focus, c"setDontAcceptFocus".as_ptr (), c"(Z)V".as_ptr ());   // C 1979
  FIND_METHOD! (define_cursor, c"defineCursor".as_ptr (),                // C 1980-1981
		c"(Lorg/femacs/EmacsCursor;)V".as_ptr ());
  /* In spite of the declaration of this function being located within
     EmacsDrawable, the ID of the `damage_rect' method is retrieved
     from EmacsWindow, which avoids virtual function dispatch within
     android_damage_window.  */                                        // C 1982-1985
  FIND_METHOD! (damage_rect, c"damageRect".as_ptr (), c"(IIII)V".as_ptr ());   // C 1986
  FIND_METHOD! (recreate_activity, c"recreateActivity".as_ptr (), c"()V".as_ptr ());   // C 1987
  FIND_METHOD! (clear_window, c"clearWindow".as_ptr (), c"()V".as_ptr ());   // C 1988
  FIND_METHOD! (clear_area, c"clearArea".as_ptr (), c"(IIII)V".as_ptr ());   // C 1989
  FIND_METHOD! (set_wm_name, c"setWmName".as_ptr (),                     // C 1990
		c"(Ljava/lang/String;)V".as_ptr ());
}

/* C 1994-2084: three more class tables - `android_init_emacs_cursor` (C 1995-2022),
   `android_init_key_character_map` (2024-2048, the one that fetches a **static** method instead of
   a `FIND_METHOD` macro) and `android_init_emacs_handle` (2050-2084, one macro call plus a direct
   `GetFieldID`).  The `org/femacs/` rename applies to C 2001/2057's class names.  */

/* C 1994-2022: `android_init_emacs_cursor`.  `static`; only called (C 6097).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_emacs_cursor ()
{
  let mut old: jclass;                                                  // C 1997

  cursor_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,   // C 1999-2000
						    c"org/femacs/EmacsCursor".as_ptr ());   // C 2001
  eassert! (! cursor_class.class.is_null ());                            // C 2002

  old = cursor_class.class;                                             // C 2004
  cursor_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,   // C 2005-2007
						       old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);                                      // C 2008

  if cursor_class.class.is_null ()                                      // C 2010
    { emacs_abort (); }                                                 // C 2011

  /* C 2013-2018's `#define FIND_METHOD …` and C 2021's `#undef`.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      cursor_class.$c_name
	= ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
							  cursor_class.class,
							  $name, $signature);
      eassert! (! cursor_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (constructor, c"<init>".as_ptr (), c"(I)V".as_ptr ());    // C 2020
}

/* C 2024-2048: `android_init_key_character_map`.  `static`; only called (C 6103).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_key_character_map ()
{
  let mut old: jclass;                                                  // C 2027

  key_character_map_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,   // C 2029-2030
						    c"android/view/KeyCharacterMap".as_ptr ());   // C 2031
  eassert! (! key_character_map_class.class.is_null ());                // C 2032

  old = key_character_map_class.class;                                  // C 2034
  key_character_map_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,   // C 2035-2037
						       old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);                                      // C 2038

  if key_character_map_class.class.is_null ()                           // C 2040
    { emacs_abort (); }                                                 // C 2041

  key_character_map_class.get_dead_char
    = ((*(*android_java_env)).GetStaticMethodID.unwrap ()) (android_java_env,   // C 2043-2044
							    key_character_map_class.class,   // C 2045
							    c"getDeadChar".as_ptr (),   // C 2046
							    c"(II)I".as_ptr ());   // C 2046
  eassert! (! key_character_map_class.get_dead_char.is_null ());        // C 2047
}

/* C 2050-2084: `android_init_emacs_handle`.  `static`; only called (C 6109).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_emacs_handle ()
{
  let mut old: jclass;                                                  // C 2053

  handle_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,   // C 2055-2056
						    c"org/femacs/EmacsHandleObject".as_ptr ());   // C 2057
  eassert! (! handle_class.class.is_null ());                            // C 2058

  old = handle_class.class;                                             // C 2060
  handle_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,   // C 2061-2063
						       old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);                                      // C 2064

  if handle_class.class.is_null ()                                      // C 2066
    { emacs_abort (); }                                                 // C 2067

  /* C 2069-2074's `#define FIND_METHOD …` and C 2077's `#undef`.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      handle_class.$c_name
	= ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
							  handle_class.class,
							  $name, $signature);
      eassert! (! handle_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (destroy_handle, c"destroyHandle".as_ptr (), c"()V".as_ptr ());   // C 2076

  handle_class.handle
    = ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 2079-2080
						     handle_class.class,   // C 2081
						     c"handle".as_ptr (), c"J".as_ptr ());   // C 2082
  eassert! (! handle_class.handle.is_null ());                           // C 2083
}
