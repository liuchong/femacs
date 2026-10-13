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

/* C 63: `#include <android/bitmap.h>` - the NDK's own header, not part of the reference tree, so
   it has no mirror and the names this file's two bitmap entry points use are declared here (P-88,
   the `<android/log.h>` block below is the same device and `femacs/src/sfntfont-android.rs` 69-88
   is the sibling unit's copy of it).  The record is declared in a module of its own because the
   sibling's `include/android.rs` 44-46 keeps `<android/bitmap.h>`'s tag opaque (`android_lock_bitmap`
   names it only through a pointer) and `include!` is textual: a module-local declaration is a
   different scope, so the mirror's tag and C's five members can both exist.  Measured on this
   machine's NDK 29.0.14206865
   (`toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/include/android/bitmap.h`): `sizeof` 20,
   offsets 0/4/8/12/16, `ANDROID_BITMAP_FORMAT_RGBA_8888` 1 (bitmap.h 108) and the three functions
   at bitmap.h 139/172/181.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_BITMAP_FORMAT_RGBA_8888: c_int = 1;                  // bitmap.h 108
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_BITMAP_FORMAT_A_8: c_int = 8;                       // bitmap.h 114
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /* int AndroidBitmap_getInfo (JNIEnv *, jobject, AndroidBitmapInfo *);  */
  pub fn AndroidBitmap_getInfo (env: *mut JNIEnv, jbitmap: jobject,
                                info: *mut android_bitmap::AndroidBitmapInfo) -> c_int;
  /* int AndroidBitmap_lockPixels (JNIEnv *, jobject, void **);  */
  pub fn AndroidBitmap_lockPixels (env: *mut JNIEnv, jbitmap: jobject,
                                   addr_ptr: *mut *mut c_void) -> c_int;
  /* int AndroidBitmap_unlockPixels (JNIEnv *, jobject);  */
  pub fn AndroidBitmap_unlockPixels (env: *mut JNIEnv, jbitmap: jobject) -> c_int;
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub mod android_bitmap {
  /* bitmap.h 99-118.  */
  #[repr(C)]
  #[derive(Clone, Copy)]
  pub struct AndroidBitmapInfo
  {
    pub width: u32, pub height: u32, pub stride: u32, pub format: i32, pub flags: u32,
  }
}
/* The mirror's opaque `<android/bitmap.h>` tag, which `android_lock_bitmap`'s declaration
   names (`include/android.rs` 44-46 and 269): C's `AndroidBitmapInfo *` argument and this unit's
   five-member record are the same type in C, so the call site casts the record's pointer to
   this alias - the `femacs/src/sfntfont-android.rs` 91/710 device the ledger's Owed 12 names.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
type AndroidBitmapInfo_tag = AndroidBitmapInfo;

/* The record's measured layout (the NDK above; the sibling unit's `android_bitmap` module is the
   same five members).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::size_of::<android_bitmap::AndroidBitmapInfo> () == 20);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (android_bitmap::AndroidBitmapInfo, width) == 0
                             && std::mem::offset_of! (android_bitmap::AndroidBitmapInfo, height) == 4
                             && std::mem::offset_of! (android_bitmap::AndroidBitmapInfo, stride) == 8
                             && std::mem::offset_of! (android_bitmap::AndroidBitmapInfo, format) == 12
                             && std::mem::offset_of! (android_bitmap::AndroidBitmapInfo, flags) == 16);


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

/* C 2194-2253: six small definitions of the JNI entry-point group - the two log/one-line entries
   (`emacsAbort`, `quit`), the shutdown pair and the low-memory pair.  Each `NATIVE_NAME (x)`
   becomes `#[unsafe(export_name = "Java_org_femacs_EmacsNative_x")]` (the unit's device), and the
   two `static` workers whose address C 2232/2252 takes are `unsafe extern "C" fn` items.
   **C 2086-2192's `NATIVE_NAME (initEmacs)` is deliberately not in this slice**: it calls bare
   `alloca` (C 2108/2121) and PORTING.md §3.2 forbids porting such a function before P-05's
   `alloca` spike picks the device - the table records that row and this slice resumes at C 2194.  */

/* C: lisp.h 5145: `extern void shut_down_emacs (int, Lisp_Object);` - C 2224's call; the mirror does
   not carry the prototype.  C: lisp.h 4477: `extern void garbage_collect (void);` - C 2244's call.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: lisp.h 5145: `extern void shut_down_emacs (int, Lisp_Object);` (emacs.c).
  pub fn shut_down_emacs (no_rip: c_int, stuff: Lisp_Object);
  /// C: lisp.h 4477: `extern void garbage_collect (void);` (alloc.c).
  pub fn garbage_collect ();
}

/* C 61-62's `<android/log.h>`: the three levels the unit already carries are measured, and
   `ANDROID_LOG_VERBOSE` is the next one down in the NDK's enum (UNKNOWN 0, DEFAULT 1, VERBOSE 2,
   DEBUG 3, INFO 4, WARN 5, ERROR 6, FATAL 7, SILENT 8 - `ANDROID_LOG_INFO == 4` and
   `ANDROID_LOG_FATAL == 7` are the unit's own measurements, so VERBOSE is `2`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_LOG_VERBOSE: c_int = 2;

/* C 2194-2200: `NATIVE_NAME (emacsAbort)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_emacsAbort")]
pub unsafe extern "C" fn emacsAbort (env: *mut JNIEnv, object: jobject)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2197

  emacs_abort ();                                                       // C 2199
}

/* C 2202-2214: `NATIVE_NAME (quit)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_quit")]
pub unsafe extern "C" fn quit (env: *mut JNIEnv, object: jobject)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2205

  let __func__: *const c_char = c"quit".as_ptr ();                       // C's `__func__`
  __android_log_print (ANDROID_LOG_VERBOSE, __func__,                   // C 2207
		       c"Sending SIGIO and setting Vquit_flag".as_ptr ());   // C 2208

  /* Raise sigio to interrupt anything that could be reading
     input.  */                                                        // C 2210-2211
  Vquit_flag = Qt ();                                                   // C 2212
  kill (getpid (), SIGIO);                                              // C 2213
}

/* Call shut_down_emacs subsequent to a call to the service's
   onDestroy callback.  CLOSURE is ignored.  */                        // C 2216-2217

/* C 2219-2225.  `static` and the *address is taken* (C 2232), so `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_shut_down_emacs (closure: *mut c_void)
{
  let __func__: *const c_char = c"android_shut_down_emacs".as_ptr ();     // C's `__func__`
  __android_log_print (ANDROID_LOG_INFO, __func__,                      // C 2222
		       c"The Emacs service is being shut down".as_ptr ());   // C 2223
  shut_down_emacs (0, Qnil ());                                         // C 2224
}

/* C 2227-2233: `NATIVE_NAME (shutDownEmacs)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_shutDownEmacs")]
pub unsafe extern "C" fn shutDownEmacs (env: *mut JNIEnv, object: jobject)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2230

  android_run_in_emacs_thread (Some (android_shut_down_emacs),          // C 2232
			       std::ptr::null_mut ());
}

/* Carry out garbage collection and clear all image caches on the
   Android terminal.  Called when the system has depleted most of its
   memory and desires that background processes release unused
   core.  */                                                           // C 2235-2238

/* C 2240-2245.  `static` and the *address is taken* (C 2252), so `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_on_low_memory (closure: *mut c_void)
{
  Fclear_image_cache (Qt (), Qnil ());                                  // C 2243
  garbage_collect ();                                                   // C 2244
}

/* C 2247-2253: `NATIVE_NAME (onLowMemory)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_onLowMemory")]
pub unsafe extern "C" fn onLowMemory (env: *mut JNIEnv, object: jobject)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2250

  android_run_in_emacs_thread (Some (android_on_low_memory),            // C 2252
			       std::ptr::null_mut ());
}

/* C 2255-2356: five uniform event posters - `sendConfigureNotify`, `sendKeyPress`,
   `sendKeyRelease`, `sendFocusIn` and `sendFocusOut`.  Each builds the same `union android_event`
   C's way (zero-initialised, then field by field, PORTING.md 1.5), posts it with
   `android_write_event` and returns the serial; C 2355's `return ++event_serial;` is the
   two-statement form of the pre-increment (the `event_serial` write then the read).  */

/* C 2255-2276: `NATIVE_NAME (sendConfigureNotify)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendConfigureNotify")]
pub unsafe extern "C" fn sendConfigureNotify (env: *mut JNIEnv, object: jobject, window: jlong,
					      time: jlong, x: jint, y: jint, width: jint,
					      height: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2261

  let mut event: android_event = std::mem::zeroed ();                   // C 2263 (PORTING.md 1.5)

  event.xconfigure.r#type = ANDROID_CONFIGURE_NOTIFY;                   // C 2265
  event_serial += 1;
  event.xconfigure.serial = event_serial as c_ulong;                    // C 2266 (the `++event_serial`)
  event.xconfigure.window = window as android_window;                   // C 2267
  event.xconfigure.time = time as android_time;                         // C 2268
  event.xconfigure.x = x;                                               // C 2269
  event.xconfigure.y = y;                                               // C 2270
  event.xconfigure.width = width;                                       // C 2271
  event.xconfigure.height = height;                                     // C 2272

  android_write_event (&raw mut event);                                 // C 2274
  return event_serial as jlong;                                         // C 2275
}

/* C 2278-2299: `NATIVE_NAME (sendKeyPress)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendKeyPress")]
pub unsafe extern "C" fn sendKeyPress (env: *mut JNIEnv, object: jobject, window: jlong,
				       time: jlong, state: jint, keycode: jint,
				       unicode_char: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2284

  let mut event: android_event = std::mem::zeroed ();                   // C 2286 (PORTING.md 1.5)

  event.xkey.r#type = ANDROID_KEY_PRESS;                                // C 2288
  event_serial += 1;
  event.xkey.serial = event_serial as c_ulong;                          // C 2289 (the `++event_serial`)
  event.xkey.window = window as android_window;                         // C 2290
  event.xkey.time = time as android_time;                               // C 2291
  event.xkey.state = state as c_uint;                                   // C 2292
  event.xkey.keycode = keycode as c_uint;                               // C 2293
  event.xkey.unicode_char = unicode_char as c_uint;                     // C 2294
  event.xkey.counter = 0;                                               // C 2295

  android_write_event (&raw mut event);                                 // C 2297
  return event_serial as jlong;                                         // C 2298
}

/* C 2301-2322: `NATIVE_NAME (sendKeyRelease)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendKeyRelease")]
pub unsafe extern "C" fn sendKeyRelease (env: *mut JNIEnv, object: jobject, window: jlong,
					 time: jlong, state: jint, keycode: jint,
					 unicode_char: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2307

  let mut event: android_event = std::mem::zeroed ();                   // C 2309 (PORTING.md 1.5)

  event.xkey.r#type = ANDROID_KEY_RELEASE;                              // C 2311
  event_serial += 1;
  event.xkey.serial = event_serial as c_ulong;                          // C 2312 (the `++event_serial`)
  event.xkey.window = window as android_window;                         // C 2313
  event.xkey.time = time as android_time;                               // C 2314
  event.xkey.state = state as c_uint;                                   // C 2315
  event.xkey.keycode = keycode as c_uint;                               // C 2316
  event.xkey.unicode_char = unicode_char as c_uint;                     // C 2317
  event.xkey.counter = 0;                                               // C 2318

  android_write_event (&raw mut event);                                 // C 2320
  return event_serial as jlong;                                         // C 2321
}

/* C 2324-2339: `NATIVE_NAME (sendFocusIn)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendFocusIn")]
pub unsafe extern "C" fn sendFocusIn (env: *mut JNIEnv, object: jobject, window: jlong,
				      time: jlong) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2328

  let mut event: android_event = std::mem::zeroed ();                   // C 2330 (PORTING.md 1.5)

  event.xfocus.r#type = ANDROID_FOCUS_IN;                               // C 2332
  event_serial += 1;
  event.xfocus.serial = event_serial as c_ulong;                        // C 2333 (the `++event_serial`)
  event.xfocus.window = window as android_window;                       // C 2334
  event.xfocus.time = time as android_time;                             // C 2335

  android_write_event (&raw mut event);                                 // C 2337
  return event_serial as jlong;                                         // C 2338
}

/* C 2341-2356: `NATIVE_NAME (sendFocusOut)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendFocusOut")]
pub unsafe extern "C" fn sendFocusOut (env: *mut JNIEnv, object: jobject, window: jlong,
				       time: jlong) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2345

  let mut event: android_event = std::mem::zeroed ();                   // C 2347 (PORTING.md 1.5)

  event.xfocus.r#type = ANDROID_FOCUS_OUT;                              // C 2349
  event_serial += 1;
  event.xfocus.serial = event_serial as c_ulong;                        // C 2350 (the `++event_serial`)
  event.xfocus.window = window as android_window;                       // C 2351
  event.xfocus.time = time as android_time;                             // C 2352

  android_write_event (&raw mut event);                                 // C 2354
  event_serial += 1;
  return event_serial as jlong;                                         // C 2355 (the `++event_serial`)
}

/* C 2358-2373: `NATIVE_NAME (sendWindowAction)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendWindowAction")]
pub unsafe extern "C" fn sendWindowAction (env: *mut JNIEnv, object: jobject, window: jlong,
					 action: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2362

  let mut event: android_event = std::mem::zeroed ();                   // C 2364 (PORTING.md 1.5)

  event.xaction.r#type = ANDROID_WINDOW_ACTION;                         // C 2366
  event_serial += 1;
  event.xaction.serial = event_serial as c_ulong;                       // C 2367 (the `++event_serial`)
  event.xaction.window = window as android_window;                      // C 2368
  event.xaction.action = action as c_uint;                              // C 2369

  android_write_event (&raw mut event);                                 // C 2371
  return event_serial as jlong;                                         // C 2372
}

/* C 2375-2393: `NATIVE_NAME (sendEnterNotify)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendEnterNotify")]
pub unsafe extern "C" fn sendEnterNotify (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 time: jlong) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2380

  let mut event: android_event = std::mem::zeroed ();                   // C 2382 (PORTING.md 1.5)

  event.xcrossing.r#type = ANDROID_ENTER_NOTIFY;                        // C 2384
  event_serial += 1;
  event.xcrossing.serial = event_serial as c_ulong;                     // C 2385 (the `++event_serial`)
  event.xcrossing.window = window as android_window;                    // C 2386
  event.xcrossing.x = x;                                                // C 2387
  event.xcrossing.y = y;                                                // C 2388
  event.xcrossing.time = time as c_ulong;                               // C 2389

  android_write_event (&raw mut event);                                 // C 2391
  return event_serial as jlong;                                         // C 2392
}

/* C 2395-2413: `NATIVE_NAME (sendLeaveNotify)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendLeaveNotify")]
pub unsafe extern "C" fn sendLeaveNotify (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 time: jlong) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2400

  let mut event: android_event = std::mem::zeroed ();                   // C 2402 (PORTING.md 1.5)

  event.xcrossing.r#type = ANDROID_LEAVE_NOTIFY;                        // C 2404
  event_serial += 1;
  event.xcrossing.serial = event_serial as c_ulong;                     // C 2405 (the `++event_serial`)
  event.xcrossing.window = window as android_window;                    // C 2406
  event.xcrossing.x = x;                                                // C 2407
  event.xcrossing.y = y;                                                // C 2408
  event.xcrossing.time = time as c_ulong;                               // C 2409

  android_write_event (&raw mut event);                                 // C 2411
  return event_serial as jlong;                                         // C 2412
}

/* C 2415-2433: `NATIVE_NAME (sendMotionNotify)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendMotionNotify")]
pub unsafe extern "C" fn sendMotionNotify (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 time: jlong) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2420

  let mut event: android_event = std::mem::zeroed ();                   // C 2422 (PORTING.md 1.5)

  event.xmotion.r#type = ANDROID_MOTION_NOTIFY;                         // C 2424
  event_serial += 1;
  event.xmotion.serial = event_serial as c_ulong;                       // C 2425 (the `++event_serial`)
  event.xmotion.window = window as android_window;                      // C 2426
  event.xmotion.x = x;                                                  // C 2427
  event.xmotion.y = y;                                                  // C 2428
  event.xmotion.time = time as c_ulong;                                 // C 2429

  android_write_event (&raw mut event);                                 // C 2431
  return event_serial as jlong;                                         // C 2432
}

/* C 2435-2456: `NATIVE_NAME (sendButtonPress)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendButtonPress")]
pub unsafe extern "C" fn sendButtonPress (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 time: jlong, state: jint,
					 button: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2441

  let mut event: android_event = std::mem::zeroed ();                   // C 2443 (PORTING.md 1.5)

  event.xbutton.r#type = ANDROID_BUTTON_PRESS;                          // C 2445
  event_serial += 1;
  event.xbutton.serial = event_serial as c_ulong;                       // C 2446 (the `++event_serial`)
  event.xbutton.window = window as android_window;                      // C 2447
  event.xbutton.x = x;                                                  // C 2448
  event.xbutton.y = y;                                                  // C 2449
  event.xbutton.time = time as c_ulong;                                 // C 2450
  event.xbutton.state = state as c_uint;                                // C 2451
  event.xbutton.button = button as c_uint;                              // C 2452

  android_write_event (&raw mut event);                                 // C 2454
  return event_serial as jlong;                                         // C 2455
}

/* C 2458-2479: `NATIVE_NAME (sendButtonRelease)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendButtonRelease")]
pub unsafe extern "C" fn sendButtonRelease (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 time: jlong, state: jint,
					 button: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2464

  let mut event: android_event = std::mem::zeroed ();                   // C 2466 (PORTING.md 1.5)

  event.xbutton.r#type = ANDROID_BUTTON_RELEASE;                        // C 2468
  event_serial += 1;
  event.xbutton.serial = event_serial as c_ulong;                       // C 2469 (the `++event_serial`)
  event.xbutton.window = window as android_window;                      // C 2470
  event.xbutton.x = x;                                                  // C 2471
  event.xbutton.y = y;                                                  // C 2472
  event.xbutton.time = time as c_ulong;                                 // C 2473
  event.xbutton.state = state as c_uint;                                // C 2474
  event.xbutton.button = button as c_uint;                              // C 2475

  android_write_event (&raw mut event);                                 // C 2477
  return event_serial as jlong;                                         // C 2478
}

/* C 2481-2502: `NATIVE_NAME (sendTouchDown)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendTouchDown")]
pub unsafe extern "C" fn sendTouchDown (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 time: jlong, pointer_id: jint,
					 flags: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2487

  let mut event: android_event = std::mem::zeroed ();                   // C 2489 (PORTING.md 1.5)

  event.touch.r#type = ANDROID_TOUCH_DOWN;                              // C 2491
  event_serial += 1;
  event.touch.serial = event_serial as c_ulong;                         // C 2492 (the `++event_serial`)
  event.touch.window = window as android_window;                        // C 2493
  event.touch.x = x;                                                    // C 2494
  event.touch.y = y;                                                    // C 2495
  event.touch.time = time as c_ulong;                                   // C 2496
  event.touch.pointer_id = pointer_id as c_uint;                        // C 2497
  event.touch.flags = flags;                                            // C 2498

  android_write_event (&raw mut event);                                 // C 2500
  return event_serial as jlong;                                         // C 2501
}

/* C 2504-2525: `NATIVE_NAME (sendTouchUp)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendTouchUp")]
pub unsafe extern "C" fn sendTouchUp (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 time: jlong, pointer_id: jint,
					 flags: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2510

  let mut event: android_event = std::mem::zeroed ();                   // C 2512 (PORTING.md 1.5)

  event.touch.r#type = ANDROID_TOUCH_UP;                                // C 2514
  event_serial += 1;
  event.touch.serial = event_serial as c_ulong;                         // C 2515 (the `++event_serial`)
  event.touch.window = window as android_window;                        // C 2516
  event.touch.x = x;                                                    // C 2517
  event.touch.y = y;                                                    // C 2518
  event.touch.time = time as c_ulong;                                   // C 2519
  event.touch.pointer_id = pointer_id as c_uint;                        // C 2520
  event.touch.flags = flags;                                            // C 2521

  android_write_event (&raw mut event);                                 // C 2523
  return event_serial as jlong;                                         // C 2524
}

/* C 2527-2548: `NATIVE_NAME (sendTouchMove)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendTouchMove")]
pub unsafe extern "C" fn sendTouchMove (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 time: jlong, pointer_id: jint,
					 flags: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2533

  let mut event: android_event = std::mem::zeroed ();                   // C 2535 (PORTING.md 1.5)

  event.touch.r#type = ANDROID_TOUCH_MOVE;                              // C 2537
  event_serial += 1;
  event.touch.serial = event_serial as c_ulong;                         // C 2538 (the `++event_serial`)
  event.touch.window = window as android_window;                        // C 2539
  event.touch.x = x;                                                    // C 2540
  event.touch.y = y;                                                    // C 2541
  event.touch.time = time as c_ulong;                                   // C 2542
  event.touch.pointer_id = pointer_id as c_uint;                        // C 2543
  event.touch.flags = flags;                                            // C 2544

  android_write_event (&raw mut event);                                 // C 2546
  return event_serial as jlong;                                         // C 2547
}

/* C 2550-2572: `NATIVE_NAME (sendWheel)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendWheel")]
pub unsafe extern "C" fn sendWheel (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 time: jlong, state: jint,
					 x_delta: f32, y_delta: f32) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2556

  let mut event: android_event = std::mem::zeroed ();                   // C 2558 (PORTING.md 1.5)

  event.wheel.r#type = ANDROID_WHEEL;                                   // C 2560
  event_serial += 1;
  event.wheel.serial = event_serial as c_ulong;                         // C 2561 (the `++event_serial`)
  event.wheel.window = window as android_window;                        // C 2562
  event.wheel.x = x;                                                    // C 2563
  event.wheel.y = y;                                                    // C 2564
  event.wheel.time = time as c_ulong;                                   // C 2565
  event.wheel.state = state;                                            // C 2566
  event.wheel.x_delta = x_delta as f64;                                 // C 2567
  event.wheel.y_delta = y_delta as f64;                                 // C 2568

  android_write_event (&raw mut event);                                 // C 2570
  return event_serial as jlong;                                         // C 2571
}

/* C 2574-2588: `NATIVE_NAME (sendIconified)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendIconified")]
pub unsafe extern "C" fn sendIconified (env: *mut JNIEnv, object: jobject, window: jlong) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2578

  let mut event: android_event = std::mem::zeroed ();                   // C 2580 (PORTING.md 1.5)

  event.iconified.r#type = ANDROID_ICONIFIED;                           // C 2582
  event_serial += 1;
  event.iconified.serial = event_serial as c_ulong;                     // C 2583 (the `++event_serial`)
  event.iconified.window = window as android_window;                    // C 2584

  android_write_event (&raw mut event);                                 // C 2586
  return event_serial as jlong;                                         // C 2587
}

/* C 2590-2604: `NATIVE_NAME (sendDeiconified)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendDeiconified")]
pub unsafe extern "C" fn sendDeiconified (env: *mut JNIEnv, object: jobject, window: jlong) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2594

  let mut event: android_event = std::mem::zeroed ();                   // C 2596 (PORTING.md 1.5)

  event.iconified.r#type = ANDROID_DEICONIFIED;                         // C 2598
  event_serial += 1;
  event.iconified.serial = event_serial as c_ulong;                     // C 2599 (the `++event_serial`)
  event.iconified.window = window as android_window;                    // C 2600

  android_write_event (&raw mut event);                                 // C 2602
  return event_serial as jlong;                                         // C 2603
}

/* C 2606-2623: `NATIVE_NAME (sendContextMenu)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendContextMenu")]
pub unsafe extern "C" fn sendContextMenu (env: *mut JNIEnv, object: jobject, window: jlong,
					 menu_event_id: jint,
					 menu_event_serial: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2611

  let mut event: android_event = std::mem::zeroed ();                   // C 2613 (PORTING.md 1.5)

  event.menu.r#type = ANDROID_CONTEXT_MENU;                             // C 2615
  event_serial += 1;
  event.menu.serial = event_serial as c_ulong;                          // C 2616 (the `++event_serial`)
  event.menu.window = window as android_window;                         // C 2617
  event.menu.menu_event_id = menu_event_id;                             // C 2618
  event.menu.menu_event_serial = menu_event_serial;                     // C 2619

  android_write_event (&raw mut event);                                 // C 2621
  return event_serial as jlong;                                         // C 2622
}

/* C 2625-2644: `NATIVE_NAME (sendExpose)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendExpose")]
pub unsafe extern "C" fn sendExpose (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 width: jint, height: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2630

  let mut event: android_event = std::mem::zeroed ();                   // C 2632 (PORTING.md 1.5)

  event.xexpose.r#type = ANDROID_EXPOSE;                                // C 2634
  event_serial += 1;
  event.xexpose.serial = event_serial as c_ulong;                       // C 2635 (the `++event_serial`)
  event.xexpose.window = window as android_window;                      // C 2636
  event.xexpose.x = x;                                                  // C 2637
  event.xexpose.y = y;                                                  // C 2638
  event.xexpose.width = width;                                          // C 2639
  event.xexpose.height = height;                                        // C 2640

  android_write_event (&raw mut event);                                 // C 2642
  return event_serial as jlong;                                         // C 2643
}

/* C 2646-2664: `NATIVE_NAME (sendDndDrag)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendDndDrag")]
pub unsafe extern "C" fn sendDndDrag (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2650

  let mut event: android_event = std::mem::zeroed ();                   // C 2652 (PORTING.md 1.5)

  event.dnd.r#type = ANDROID_DND_DRAG_EVENT;                            // C 2654
  event_serial += 1;
  event.dnd.serial = event_serial as c_ulong;                           // C 2655 (the `++event_serial`)
  event.dnd.window = window as android_window;                          // C 2656
  event.dnd.x = x;                                                      // C 2657
  event.dnd.y = y;                                                      // C 2658
  event.dnd.uri_or_string = std::ptr::null_mut ();                      // C 2659
  event.dnd.length = 0;                                                 // C 2660

  android_write_event (&raw mut event);                                 // C 2662
  return event_serial as jlong;                                         // C 2663
}

/* C 2666-2706: `NATIVE_NAME (sendDndUri)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendDndUri")]
pub unsafe extern "C" fn sendDndUri (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 string: jstring) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2671

  let mut event: android_event = std::mem::zeroed ();                   // C 2673 (PORTING.md 1.5)
  let characters: *const jchar;                                         // C 2674
  let mut length: jsize;                                                // C 2675
  let mut buffer: *mut c_ushort;                                        // C 2676

  event.dnd.r#type = ANDROID_DND_URI_EVENT;                             // C 2678
  event_serial += 1;
  event.dnd.serial = event_serial as c_ulong;                           // C 2679 (the `++event_serial`)
  event.dnd.window = window as android_window;                          // C 2680
  event.dnd.x = x;                                                      // C 2681
  event.dnd.y = y;                                                      // C 2682

  length = ((*(*env)).GetStringLength.unwrap ()) (env, string);         // C 2684
  buffer = malloc (length as usize * std::mem::size_of::<c_ushort> ()) as *mut c_ushort; // C 2685

  /* Out of memory.  */                                                 // C 2687
  if buffer.is_null ()                                                  // C 2688
    { return 0; }                                                       // C 2689

  characters = ((*(*env)).GetStringChars.unwrap ()) (env, string,       // C 2691
					       std::ptr::null_mut ());

  if characters.is_null ()                                              // C 2693
    /* The JVM has run out of memory; return and let the out of memory
       error take its course.  */                                       // C 2694-2695
    { return 0; }                                                       // C 2696

  memcpy (buffer as *mut c_void, characters as *const c_void,           // C 2698
					       length as usize * std::mem::size_of::<c_ushort> ());
  ((*(*env)).ReleaseStringChars.unwrap ()) (env, string, characters);   // C 2699

  event.dnd.uri_or_string = buffer;                                     // C 2701
  event.dnd.length = length as size_t;                                  // C 2702

  android_write_event (&raw mut event);                                 // C 2704
  return event_serial as jlong;                                         // C 2705
}

/* C 2708-2748: `NATIVE_NAME (sendDndText)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendDndText")]
pub unsafe extern "C" fn sendDndText (env: *mut JNIEnv, object: jobject, window: jlong,
					 x: jint, y: jint,
					 string: jstring) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2713

  let mut event: android_event = std::mem::zeroed ();                   // C 2715 (PORTING.md 1.5)
  let characters: *const jchar;                                         // C 2716
  let mut length: jsize;                                                // C 2717
  let mut buffer: *mut c_ushort;                                        // C 2718

  event.dnd.r#type = ANDROID_DND_TEXT_EVENT;                            // C 2720
  event_serial += 1;
  event.dnd.serial = event_serial as c_ulong;                           // C 2721 (the `++event_serial`)
  event.dnd.window = window as android_window;                          // C 2722
  event.dnd.x = x;                                                      // C 2723
  event.dnd.y = y;                                                      // C 2724

  length = ((*(*env)).GetStringLength.unwrap ()) (env, string);         // C 2726
  buffer = malloc (length as usize * std::mem::size_of::<c_ushort> ()) as *mut c_ushort; // C 2727

  /* Out of memory.  */                                                 // C 2729
  if buffer.is_null ()                                                  // C 2730
    { return 0; }                                                       // C 2731

  characters = ((*(*env)).GetStringChars.unwrap ()) (env, string,       // C 2733
					       std::ptr::null_mut ());

  if characters.is_null ()                                              // C 2735
    /* The JVM has run out of memory; return and let the out of memory
       error take its course.  */                                       // C 2736-2737
    { return 0; }                                                       // C 2738

  memcpy (buffer as *mut c_void, characters as *const c_void,           // C 2740
					       length as usize * std::mem::size_of::<c_ushort> ());
  ((*(*env)).ReleaseStringChars.unwrap ()) (env, string, characters);   // C 2741

  event.dnd.uri_or_string = buffer;                                     // C 2743
  event.dnd.length = length as size_t;                                  // C 2744

  android_write_event (&raw mut event);                                 // C 2746
  return event_serial as jlong;                                         // C 2747
}

/* C 2750-2779: `NATIVE_NAME (sendNotificationDeleted)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendNotificationDeleted")]
pub unsafe extern "C" fn sendNotificationDeleted (env: *mut JNIEnv, object: jobject,
					 tag: jstring) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2754

  let mut event: android_event = std::mem::zeroed ();                   // C 2756 (PORTING.md 1.5)
  let characters: *const c_char;                                        // C 2757

  event.notification.r#type = ANDROID_NOTIFICATION_DELETED;             // C 2759
  event_serial += 1;
  event.notification.serial = event_serial as c_ulong;                  // C 2760 (the `++event_serial`)
  event.notification.window = ANDROID_NONE as android_window;           // C 2761

  /* TAG is guaranteed to be an ASCII string, of which the JNI character
     encoding is a superset.  */                                        // C 2763-2764
  characters = ((*(*env)).GetStringUTFChars.unwrap ()) (env, tag, std::ptr::null_mut ()); // C 2765
  if characters.is_null ()                                              // C 2766
    { return 0; }                                                       // C 2767

  event.notification.tag = strdup (characters);                         // C 2769
  ((*(*env)).ReleaseStringUTFChars.unwrap ()) (env, tag, characters);   // C 2770
  if event.notification.tag.is_null ()                                  // C 2771
    { return 0; }                                                       // C 2772

  event.notification.action = std::ptr::null_mut ();                    // C 2774
  event.notification.length = 0;                                        // C 2775

  android_write_event (&raw mut event);                                 // C 2777
  return event_serial as jlong;                                         // C 2778
}

/* C 2781-2827: `NATIVE_NAME (sendNotificationAction)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendNotificationAction")]
pub unsafe extern "C" fn sendNotificationAction (env: *mut JNIEnv, object: jobject,
					 tag: jstring, action: jstring) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2785

  let mut event: android_event = std::mem::zeroed ();                   // C 2787 (PORTING.md 1.5)
  let mut characters: *const c_void;                                    // C 2788
  let mut length: jsize;                                                // C 2789
  let mut buffer: *mut c_ushort;                                        // C 2790

  event.notification.r#type = ANDROID_NOTIFICATION_ACTION;              // C 2792
  event_serial += 1;
  event.notification.serial = event_serial as c_ulong;                  // C 2793 (the `++event_serial`)
  event.notification.window = ANDROID_NONE as android_window;           // C 2794

  /* TAG is guaranteed to be an ASCII string, of which the JNI character
     encoding is a superset.  */                                        // C 2796-2797
  characters = ((*(*env)).GetStringUTFChars.unwrap ()) (env, tag,       // C 2798
					       std::ptr::null_mut ()) as *const c_void;
  if characters.is_null ()                                              // C 2799
    { return 0; }                                                       // C 2800

  event.notification.tag = strdup (characters as *const c_char);        // C 2802
  ((*(*env)).ReleaseStringUTFChars.unwrap ()) (env, tag, characters as *const c_char); // C 2803
  if event.notification.tag.is_null ()                                  // C 2804
    { return 0; }                                                       // C 2805

  length = ((*(*env)).GetStringLength.unwrap ()) (env, action);         // C 2807
  buffer = malloc (length as usize * std::mem::size_of::<c_ushort> ()) as *mut c_ushort; // C 2808
  characters = ((*(*env)).GetStringChars.unwrap ()) (env, action,       // C 2809
					       std::ptr::null_mut ()) as *const c_void;

  if characters.is_null ()                                              // C 2811
    {                                                                   // C 2812
      /* The JVM has run out of memory; return and let the out of memory
	 error take its course.  */                                            // C 2813-2814
      xfree (event.notification.tag as *mut c_void);                    // C 2815
      return 0;                                                         // C 2816
    }                                                                   // C 2817

  memcpy (buffer as *mut c_void, characters, length as usize * std::mem::size_of::<c_ushort> ()); // C 2819
  ((*(*env)).ReleaseStringChars.unwrap ()) (env, action, characters as *const jchar); // C 2820

  event.notification.action = buffer;                                   // C 2822
  event.notification.length = length as size_t;                         // C 2823

  android_write_event (&raw mut event);                                 // C 2825
  return event_serial as jlong;                                         // C 2826
}

/* C 2829-2862: `NATIVE_NAME (sendConfigurationChanged)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_sendConfigurationChanged")]
pub unsafe extern "C" fn sendConfigurationChanged (env: *mut JNIEnv, object: jobject,
					 detail: c_int, dpi_x: jfloat,
					 dpi_y: jfloat, dpi_scaled: jfloat,
					 ui_mode: c_int) -> jlong
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2835

  let mut event: android_event = std::mem::zeroed ();                   // C 2837 (PORTING.md 1.5)

  event.config.r#type = ANDROID_CONFIGURATION_CHANGED;                  // C 2839
  event_serial += 1;
  event.config.serial = event_serial as c_ulong;                        // C 2840 (the `++event_serial`)
  event.config.window = ANDROID_NONE as android_window;                 // C 2841
  event.config.detail = detail;                                         // C 2842

  /* C 2844's `switch (detail)`, with its two `case` labels, the `break` that ends each and
     its `default`, becomes the `match` below (PORTING.md 2.2, the macro-body-notes row);
     Rust has no `switch`/`case`/`default`/`break`, so C 2845's and 2858's braces and the two
     `break;` lines have no line of their own here.  */
  match detail                                                          // C 2844
    {
      ANDROID_PIXEL_DENSITY_CHANGED =>                                  // C 2846
	{
	  event.config.u.pixel_density.dpi_x = dpi_x as f64;                   // C 2847
	  event.config.u.pixel_density.dpi_y = dpi_y as f64;                   // C 2848
	  event.config.u.pixel_density.dpi_scaled = dpi_scaled as f64;         // C 2849
	}

      ANDROID_UI_MODE_CHANGED =>                                        // C 2852
	{ event.config.u.ui_mode = ui_mode; }                                  // C 2853

      _ =>                                                              // C 2856 (C's `default:`)
	{ emacs_abort (); }                                                    // C 2857
    }

  android_write_event (&raw mut event);                                 // C 2860
  return event_serial as jlong;                                         // C 2861
}

/* C 2864-2873: `NATIVE_NAME (shouldForwardMultimediaButtons)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_shouldForwardMultimediaButtons")]
pub unsafe extern "C" fn shouldForwardMultimediaButtons (env: *mut JNIEnv,
					 object: jobject) -> jboolean
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2868

  /* Yes, android_pass_multimedia_buttons_to_system is being
     read from the UI thread.  */                                       // C 2870-2871
  return (! android_pass_multimedia_buttons_to_system) as jboolean;     // C 2872
}

/* C 2875-2880: `NATIVE_NAME (getQuitKeycode)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_getQuitKeycode")]
pub unsafe extern "C" fn getQuitKeycode (env: *mut JNIEnv, object: jobject) -> jint
{
  /* Likewise.  */                                                      // C 2878
  return android_quit_keycode as jint;                                  // C 2879
}

/* C 2882-2888: `NATIVE_NAME (shouldForwardCtrlSpace)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_shouldForwardCtrlSpace")]
pub unsafe extern "C" fn shouldForwardCtrlSpace (env: *mut JNIEnv, object: jobject)
					 -> jboolean
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 2886

  return (! android_intercept_control_space) as jboolean;               // C 2887
}

/* C 2890-2983: `NATIVE_NAME (blitRect)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_blitRect")]
pub unsafe extern "C" fn blitRect (env: *mut JNIEnv, object: jobject,
				   src: jobject, dest: jobject,
				   mut x1: jint, mut y1: jint, mut x2: jint, mut y2: jint)
{
  let mut src_info: android_bitmap::AndroidBitmapInfo = std::mem::zeroed ();   // C 2895 (PORTING.md 1.5)
  let mut dest_info: android_bitmap::AndroidBitmapInfo = std::mem::zeroed ();  // C 2895 (PORTING.md 1.5)
  let mut src_data_1: *mut c_uchar = std::ptr::null_mut ();   // C 2896 (uninitialised; PORTING.md 1.5)
  let mut dest_data_1: *mut c_uchar = std::ptr::null_mut ();  // C 2896 (uninitialised; PORTING.md 1.5)
  let mut src_data: *mut c_void = std::ptr::null_mut ();      // C 2897 (uninitialised; PORTING.md 1.5)
  let mut dest_data: *mut c_void = std::ptr::null_mut ();     // C 2897 (uninitialised; PORTING.md 1.5)

  /* N.B. that X2 and Y2 represent the pixel past the edge of the
     rectangle; thus, the width is x2 - x1 and the height is y2 -
     y1.  */                                                           // C 2899-2901

  memset (&raw mut src_info as *mut c_void, 0,
	 std::mem::size_of::<android_bitmap::AndroidBitmapInfo> ());          // C 2903
  memset (&raw mut dest_info as *mut c_void, 0,
	 std::mem::size_of::<android_bitmap::AndroidBitmapInfo> ());          // C 2904
  AndroidBitmap_getInfo (env, src, &raw mut src_info);                 // C 2905
  AndroidBitmap_getInfo (env, dest, &raw mut dest_info);               // C 2906

  /* If the stride is 0 after a call to `getInfo', assume it
     failed.  */                                                       // C 2908-2909

  if src_info.stride == 0 || dest_info.stride == 0                     // C 2911
    { return; }                                                        // C 2912

  /* If formats differ, abort.  */                                     // C 2914
  eassert! (src_info.format == dest_info.format                        // C 2915
	    && src_info.format == ANDROID_BITMAP_FORMAT_RGBA_8888);       // C 2916

  /* Lock the image data.  */                                          // C 2918
  src_data = std::ptr::null_mut ();                                    // C 2919
  AndroidBitmap_lockPixels (env, src, &raw mut src_data);              // C 2920

  if src_data.is_null ()                                               // C 2922
    { return; }                                                        // C 2923

  dest_data = std::ptr::null_mut ();                                   // C 2925
  AndroidBitmap_lockPixels (env, dest, &raw mut dest_data);            // C 2926

  /* C 2928-2929's `goto fail1`, C 2955-2956's `goto fail2` and the `fail2:`/`fail1:` labels at
     C 2979/2981 become the two nested labelled blocks below (PORTING.md 2.2, the
     android_create_lib_link shape): `goto fail1` is `break 'fail1` (past the destination
     unlock, straight to C's `fail1:` tail) and `goto fail2` is `break 'fail2` (out of the inner
     block, to `AndroidBitmap_unlockPixels (env, dest)`); C's fall-through from the `fail2:`
     line to the `fail1:` line is the outer block's end.  */

  'fail1: {
    if dest_data.is_null ()                                            // C 2928
      { break 'fail1; }                                                // C 2929 (C's `goto fail1`)

    /* Now clip the rectangle to the bounds of the source and
       destination bitmap.  */                                         // C 2931-2932

    x1 = MAX! (x1, 0);                                                 // C 2934
    y1 = MAX! (y1, 0);                                                 // C 2935
    x2 = MAX! (x2, 0);                                                 // C 2936
    y2 = MAX! (y2, 0);                                                 // C 2937

    if x1 as c_uint >= src_info.width                                  // C 2939 (C's int-to-unsigned conversion)
      || x1 as c_uint >= dest_info.width                               // C 2940
      { x1 = MIN! (dest_info.width - 1, src_info.width - 1) as jint; }  // C 2941

    if x2 as c_uint > src_info.width                                   // C 2943
      || x2 as c_uint > dest_info.width                                // C 2944
      { x2 = MIN! (src_info.width, dest_info.width) as jint; }         // C 2945

    if y1 as c_uint >= src_info.height                                 // C 2947
      || y1 as c_uint >= dest_info.height                              // C 2948
      { y1 = MIN! (dest_info.height - 1, src_info.height - 1) as jint; }  // C 2949

    if y2 as c_uint > src_info.height                                  // C 2951
      || y2 as c_uint > dest_info.height                               // C 2952
      { y2 = MIN! (src_info.height, dest_info.height) as jint; }       // C 2953

    'fail2: {
      if x1 >= x2 || y1 >= y2                                          // C 2955
	{ break 'fail2; }                                              // C 2956 (C's `goto fail2`)

      /* Determine the address of the first line to copy.  */          // C 2958

      src_data_1 = src_data as *mut c_uchar;                           // C 2960
      dest_data_1 = dest_data as *mut c_uchar;                         // C 2961
      src_data_1 = src_data_1.wrapping_offset ((x1 * 4) as isize);     // C 2962
      src_data_1 = src_data_1.wrapping_offset ((y1 * src_info.stride as jint) as isize);  // C 2963
      dest_data_1 = dest_data_1.wrapping_offset ((x1 * 4) as isize);   // C 2964
      dest_data_1 = dest_data_1.wrapping_offset ((y1 * dest_info.stride as jint) as isize);  // C 2965

      /* Start copying each line.  */                                  // C 2967

      while y1 != y2                                                   // C 2969
	{
	  memcpy (dest_data_1 as *mut c_void, src_data_1 as *const c_void,
		  ((x2 - x1) * 4) as size_t);                          // C 2971
	  src_data_1 = src_data_1.wrapping_offset (src_info.stride as isize);   // C 2972
	  dest_data_1 = dest_data_1.wrapping_offset (dest_info.stride as isize);  // C 2973
	  y1 += 1;                                                     // C 2974 (the `y1++`)
	}
    }

    /* Complete the copy and unlock the bitmap.  */                    // C 2977

    /* fail2: */
    AndroidBitmap_unlockPixels (env, dest);                            // C 2980
  }

  /* fail1: */
  AndroidBitmap_unlockPixels (env, src);                               // C 2982
}

/* C 2985-3002: `NATIVE_NAME (notifyPixelsChanged)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_notifyPixelsChanged")]
pub unsafe extern "C" fn notifyPixelsChanged (env: *mut JNIEnv, object: jobject,
					      bitmap: jobject)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                    // C 2989

  let mut data: *mut c_void = std::ptr::null_mut ();                   // C 2991 (uninitialised; PORTING.md 1.5)

  /* Lock and unlock the bitmap.  This calls
     SkBitmap->notifyPixelsChanged.  */                               // C 2993-2994

  if AndroidBitmap_lockPixels (env, bitmap, &raw mut data) < 0         // C 2996
    /* The return value is less than 0 if an error occurs.
       Good luck finding this in the documentation.  */                // C 2997-2998
    { return; }                                                        // C 2999

  AndroidBitmap_unlockPixels (env, bitmap);                            // C 3001
}

/* C 3003-3008: the blank, C 3004's `/* Forward declarations of deadlock prevention functions.  */`
   and C 3006-3008's three `static void` prototypes (`android_begin_query`, `android_end_query`,
   `android_answer_query_spin`).  A Rust item needs no forward declaration - one crate sees the
   whole file, so the three definitions live further down it - the `android_run_select_thread`
   device of C 339 in this same unit.  */

/* C 3010-3032: the three one-line deadlock-prevention entries Java calls on the UI thread - each is
   C's `JNIEXPORT void JNICALL NATIVE_NAME (name) (JNIEnv *env, jobject object)` with a one-call
   body.  Their callees are the definitions at C 7067/7092/7047, carried by the C 6880-7096 region
   at the end of this file in the same round: C resolves the dependency through C 3006-3008's
   forward declarations at link time, a Rust crate at compile time, so the machinery had to land
   first (the ledger's Owed 13).  */

/* C 3010-3016: `NATIVE_NAME (beginSynchronous)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_beginSynchronous")]
pub unsafe extern "C" fn beginSynchronous (env: *mut JNIEnv, object: jobject)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 3013

  android_begin_query ();                                               // C 3015
}

/* C 3018-3024: `NATIVE_NAME (endSynchronous)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_endSynchronous")]
pub unsafe extern "C" fn endSynchronous (env: *mut JNIEnv, object: jobject)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 3021

  android_end_query ();                                                 // C 3023
}

/* C 3026-3032: `NATIVE_NAME (answerQuerySpin)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_answerQuerySpin")]
pub unsafe extern "C" fn answerQuerySpin (env: *mut JNIEnv, object: jobject)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 3029

  android_answer_query_spin ();                                         // C 3031
}

/* C 3034-3040: C 3034's form feed, the blank and C 3036-3039's comment `/* System thread setup.
   Android doesn't always block signals Emacs is interested in ...  */`.  */

/* C 3041-3063: `NATIVE_NAME (setupSystemThread)`.  C's own definition takes `(void)`: Java passes
   the usual two arguments and C names neither, so this port's item takes none either.  The two
   constants the body reads are the Android arm's `<signal.h>` values, which is why they are
   spelled through `bionic_signal` below (C 3053/3058).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_setupSystemThread")]
pub unsafe extern "C" fn setupSystemThread ()
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 3044

  let mut sigset: sigset_t = std::mem::zeroed ();                       // C 3046 (PORTING.md 1.5)

  /* Block everything except for SIGSEGV and SIGBUS; those two are
     used by the runtime.  */                                           // C 3048-3049

  sigfillset (&raw mut sigset);                                         // C 3051
  sigdelset (&raw mut sigset, SIGSEGV);                                 // C 3052
  sigdelset (&raw mut sigset, bionic_signal::SIGBUS);                   // C 3053 (bionic 7, below)

  /* Save the signal mask that was previously used.  It will be
     restored in `initEmacs'.  */                                       // C 3055-3056

  if pthread_sigmask (bionic_signal::SIG_BLOCK, &raw mut sigset,
		      &raw mut startup_signal_mask) != 0                    // C 3058 (bionic 0, below)
    {
      let __func__: *const c_char = c"setupSystemThread".as_ptr ();
      __android_log_print (ANDROID_LOG_WARN, __func__,                  // C 3059
			   c"pthread_sigmask: %s".as_ptr (),
			   strerror (*errno ()));                               // C 3060
    }
  else
    { signal_mask_changed_p = true; }                                   // C 3062
}

/* C 3065-3069: the `#pragma clang diagnostic pop` / `#else` / `#pragma GCC diagnostic pop` /
   `#endif` region that closes the pair C 1453-1459 opened.  A pragma has no Rust counterpart and
   the port emits none either way (the process.rs 340 / timefns.rs 79-81 precedent; C 1453-1459's
   row in this ledger is the same reading).  */

/* C 3070-3071: the blank and C 3071's form feed.  */

/* The two names below are the Android arm's `<signal.h>` values, which the frozen
   `lib/include/signal.rs` mirror (the Darwin header) cannot carry: it has `SIGBUS` 10 and
   `SIG_BLOCK` 1, the frozen target's numbers, while bionic has 7 and 0.  They are measured on this
   machine's NDK 29.0.14206865
   (`clang --target=aarch64-linux-android24 -dM -E -include signal.h`: `#define SIGBUS 7`,
   `#define SIG_BLOCK 0`), and `SIGSEGV` is 11 on both, so the mirror's own name is used for it.
   A module of its own because `include!` is textual and the mirror already puts `SIGBUS`/`SIG_BLOCK`
   in the crate root - a module-local declaration is a different scope, so the Android arm's value
   and the frozen arm's value can both exist (the `android_bitmap` device of slice 11; the mirror
   round that gives `include/signal.rs` its bionic arm removes this module).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub mod bionic_signal {
  /// bionic `<signal.h>`: `#define SIGBUS 7` (the frozen mirror's Darwin `SIGBUS` is 10).
  pub const SIGBUS: core::ffi::c_int = 7;
  /// bionic `<signal.h>`: `#define SIG_BLOCK 0` (the frozen mirror's Darwin `SIG_BLOCK` is 1).
  pub const SIG_BLOCK: core::ffi::c_int = 0;
}

/* C 6880-6881: the form feed and the blank.  */

/* C 6882-6892: the deadlock-protection comment is C 6894-6919's; the three file-scope objects it
   describes come first - C's `static char android_servicing_query;`, `static void
   (*android_query_function) (void *);` and `static void *android_query_context;`, all tentative
   definitions, i.e. zero (PORTING.md 1.5).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut android_servicing_query: c_char = 0;                       // C 6886 (tentative, i.e. zero)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut android_query_function: Option<unsafe extern "C" fn (*mut c_void)> = None;   // C 6889 (tentative)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut android_query_context: *mut c_void = std::ptr::null_mut ();   // C 6892 (tentative)

/* C 6894-6919: the `/* Deadlock protection. … */` comment (thirteen paragraphs of the file's own
   prose).  */

/* C 6931 and the eight sites below use the `__atomic_*` builtins (PORTING.md §8/§6: the same
   object and the same memory order through `core::sync::atomic`).  C's builtins are type-generic;
   in this file they are applied to `char android_servicing_query` and `bool android_urgent_query`,
   both one byte wide, so these helpers are the one-byte width (values in C's own `char`
   type, the view as a byte) and each use site spells out the
   pointer it hands them (the `femacs/src/androidterm.rs` 6163/8057 pair is the same device for its
   two `c_ulong` objects).  `__atomic_clear`'s C value is the zero of the object's type, i.e. 0.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const __ATOMIC_ACQUIRE: core::sync::atomic::Ordering = core::sync::atomic::Ordering::Acquire;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const __ATOMIC_RELEASE: core::sync::atomic::Ordering = core::sync::atomic::Ordering::Release;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const __ATOMIC_ACQ_REL: core::sync::atomic::Ordering = core::sync::atomic::Ordering::AcqRel;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn __atomic_load_n (p: *const c_uchar, order: core::sync::atomic::Ordering) -> c_char
{
  (*(p as *const core::sync::atomic::AtomicU8)).load (order) as c_char
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn __atomic_store_n (p: *mut c_uchar, v: c_char, order: core::sync::atomic::Ordering)
{
  (*(p as *mut core::sync::atomic::AtomicU8)).store (v as c_uchar, order)
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn __atomic_exchange_n (p: *mut c_uchar, v: c_char,
			       order: core::sync::atomic::Ordering) -> c_char
{
  (*(p as *mut core::sync::atomic::AtomicU8)).swap (v as c_uchar, order) as c_char
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn __atomic_clear (p: *mut c_uchar, order: core::sync::atomic::Ordering)
{
  (*(p as *mut core::sync::atomic::AtomicU8)).store (0, order)
}

/* C 6921-6936: `android_before_select` (non-`static`, so `#[unsafe(no_mangle)]`: the mirror
   declares it at `include/android.rs` 316 and C's own `void android_before_select (void)` is the
   definition that matches, PORTING.md 0.2).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_before_select ()                     // C 6925-6936
{
  /* Since Emacs is reading keyboard input again, signify that queries
     from input methods are no longer ``urgent''.  */                 // C 6928-6929

  __atomic_clear (&raw mut android_urgent_query as *mut c_uchar,
		  __ATOMIC_RELEASE);                                   // C 6931

  /* Check for and run anything the UI thread wants to run on the main
     thread.  */                                                      // C 6933-6934

  android_check_query ();                                              // C 6935
}

/* C 6938-6967: `android_check_query` (non-`static`; the mirror declares it at
   `include/android.rs` 317).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_check_query ()                       // C 6941-6967
{
  let mut proc: Option<unsafe extern "C" fn (*mut c_void)>;                // C 6944
  let mut closure: *mut c_void;                                            // C 6945

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  proc = None;
  closure = std::ptr::null_mut ();

  if __atomic_load_n (&raw const android_servicing_query as *const c_uchar,
		      __ATOMIC_ACQUIRE) == 0                          // C 6947
    { return; }                                                             // C 6948

  /* First, load the procedure and closure.  */                              // C 6950
  closure = android_query_context;                                          // C 6951
  proc = android_query_function;                                            // C 6952

  if proc.is_none ()                                                        // C 6954
    { return; }                                                             // C 6955

  (proc.unwrap ()) (closure);                                               // C 6957

  /* Finish the query.  */                                                  // C 6959
  android_query_context = std::ptr::null_mut ();                            // C 6960
  android_query_function = None;                                            // C 6961
  __atomic_store_n (&raw mut android_servicing_query as *mut c_uchar, 0,
		    __ATOMIC_RELEASE);                                 // C 6962
  __atomic_clear (&raw mut android_urgent_query as *mut c_uchar,
		  __ATOMIC_RELEASE);                                   // C 6963

  /* Signal completion.  */                                                 // C 6965
  sem_post (&raw mut android_query_sem);                                    // C 6966
}

/* C 6969-7009: `android_check_query_urgent` (non-`static`; the mirror declares it at
   `include/android.rs` 318).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_check_query_urgent ()                // C 6976-7009
{
  let mut proc: Option<unsafe extern "C" fn (*mut c_void)>;            // C 6979
  let mut closure: *mut c_void;                                        // C 6980

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  proc = None;
  closure = std::ptr::null_mut ();

  if __atomic_load_n (&raw const android_urgent_query as *const c_uchar,
		      __ATOMIC_ACQUIRE) == 0                          // C 6982
    { return; }                                                        // C 6983

  let __func__: *const c_char = c"android_check_query_urgent".as_ptr ();
  __android_log_print (ANDROID_LOG_VERBOSE, __func__,                  // C 6985
		       c"Responding to urgent query...".as_ptr ());            // C 6986

  if __atomic_load_n (&raw const android_servicing_query as *const c_uchar,
		      __ATOMIC_ACQUIRE) == 0                          // C 6988
    { return; }                                                        // C 6989

  /* First, load the procedure and closure.  */                          // C 6991
  closure = android_query_context;                                      // C 6992
  proc = android_query_function;                                        // C 6993

  if proc.is_none ()                                                    // C 6995
    { return; }                                                         // C 6996

  (proc.unwrap ()) (closure);                                           // C 6998

  /* Finish the query.  Don't clear `android_urgent_query'; instead,
     do that the next time Emacs enters the keyboard loop.  */         // C 7000-7001

  android_query_context = std::ptr::null_mut ();                        // C 7003
  android_query_function = None;                                        // C 7004
  __atomic_store_n (&raw mut android_servicing_query as *mut c_uchar, 0,
		    __ATOMIC_RELEASE);                                 // C 7005

  /* Signal completion.  */                                             // C 7007
  sem_post (&raw mut android_query_sem);                                // C 7008
}

/* C 7011-7041: `android_answer_query` (C's `static void`; only called from C 7082).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_answer_query ()                                     // C 7015-7041
{
  let mut proc: Option<unsafe extern "C" fn (*mut c_void)>;            // C 7018
  let mut closure: *mut c_void;                                        // C 7019

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  proc = None;
  closure = std::ptr::null_mut ();

  eassert! (__atomic_load_n (&raw const android_servicing_query as *const c_uchar,
			     __ATOMIC_ACQUIRE) == 1);                  // C 7021-7023

  /* First, load the procedure and closure.  */                          // C 7025
  closure = android_query_context;                                      // C 7026
  proc = android_query_function;                                        // C 7027

  if proc.is_none ()                                                    // C 7029
    { return; }                                                         // C 7030

  (proc.unwrap ()) (closure);                                           // C 7032

  /* Finish the query.  */                                              // C 7034
  android_query_context = std::ptr::null_mut ();                        // C 7035
  android_query_function = None;                                        // C 7036
  __atomic_clear (&raw mut android_urgent_query as *mut c_uchar,
		  __ATOMIC_RELEASE);                                   // C 7037

  /* Signal completion.  */                                             // C 7039
  sem_post (&raw mut android_query_sem);                                // C 7040
}

/* C 7043-7058: `android_answer_query_spin` (C's `static void`; called from C 3031).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_answer_query_spin ()                                // C 7046-7058
{
  let mut n: c_int;                                                    // C 7049

  n = 0;                                                               // (C leaves `n` uninitialised; PORTING.md 1.5)

  while { n = __atomic_load_n (&raw const android_servicing_query as *const c_uchar,
			       __ATOMIC_ACQUIRE) as c_int; n == 0 }         // C 7051-7052
    { eassert! (n == 0); }                                             // C 7053

  /* Note that this function is supposed to be called before
     `android_begin_query' starts, so clear the service flag.  */     // C 7055-7056
  android_check_query ();                                              // C 7057
}

/* C 7060-7086: `android_begin_query` (C's `static void`; called from C 3015).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_begin_query ()                                      // C 7066-7086
{
  let mut old: c_char;                                                 // C 7069

  old = 0;                                                             // (C leaves `old` uninitialised; PORTING.md 1.5)

  /* Load the previous value of `android_servicing_query' and then set
     it to 2.  */                                                     // C 7071-7072

  old = __atomic_exchange_n (&raw mut android_servicing_query as *mut c_uchar,
			     2, __ATOMIC_ACQ_REL);                         // C 7074-7075

  /* See if a query was previously in progress.  */                    // C 7077
  if old == 1                                                          // C 7078
    {
      /* Answer the query that is currently being made.  */            // C 7080
      eassert! (! android_query_function.is_none ());                  // C 7081
      android_answer_query ();                                         // C 7082
    }

  /* `android_servicing_query' is now 2.  */                            // C 7085
}

/* C 7088-7096: `android_end_query` (C's `static void`; called from C 3023).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_end_query ()                                        // C 7091-7096
{
  __atomic_store_n (&raw mut android_servicing_query as *mut c_uchar, 0,
		    __ATOMIC_RELEASE);                                 // C 7094
  __atomic_clear (&raw mut android_urgent_query as *mut c_uchar,
		  __ATOMIC_RELEASE);                                   // C 7095
}

/* C 3072-3083: C 3072's blank, C 3073-3079's `/* Java functions called by C. … */` comment, the
   blank and C 3081-3082's `/* Destroy the specified handle and mark it as free on the Java side
   as well.  */` comment.  */

/* C 3084-3120: `android_destroy_handle` (C's `static void`; called from C 4851 and C 5868).  C's
   three function-local `static` objects are Rust's function-local `static mut` (the `android_select`
   device at 1244 of this unit).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_destroy_handle (handle: android_handle)            // C 3084-3120
{
  static mut old: jclass = std::ptr::null_mut ();                      // C 3087 (tentative, i.e. zero)
  static mut class: jclass = std::ptr::null_mut ();                    // C 3087 (tentative, i.e. zero)
  static mut method: jmethodID = std::ptr::null_mut ();                // C 3088 (tentative, i.e. zero)

  if class.is_null ()                                                  // C 3090
    {
      class
	= ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,   // C 3092-3093
							c"org/femacs/EmacsHandleObject".as_ptr ());   // C 3094
      eassert! (! class.is_null ());                                   // C 3095

      method
	= ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,   // C 3097-3098
							class,
							c"destroyHandle".as_ptr (),
							c"()V".as_ptr ());                          // C 3099
      eassert! (! method.is_null ());                                  // C 3100

      old = class;                                                     // C 3102
      class
	= ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,   // C 3103-3105
							class as jobject);
      android_exception_check_1 (old);                                 // C 3106
      ANDROID_DELETE_LOCAL_REF! (old);                                 // C 3107
    }

  ((*(*android_java_env)).CallVoidMethod.unwrap ()) (android_java_env,   // C 3110
						     handle as jobject,                // C 3110
						     method);                          // C 3111

  /* Just clear any exception thrown.  If destroying the handle
     fails from an out-of-memory error, then Emacs loses some
     resources, but that is not as big deal as signaling.  */         // C 3113-3115
  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 3116

  /* Delete the global reference regardless of any error.  */          // C 3118
  ((*(*android_java_env)).DeleteGlobalRef.unwrap ()) (android_java_env,   // C 3119
						      handle as jobject);
}

/* C 3122-3143: `android_change_window_attributes` (non-`static`; the mirror declares it at
   `include/android.rs` 305).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_change_window_attributes (handle: android_window,
							   value_mask: android_window_value_mask,
							   attrs: *mut android_set_window_attributes)   // C 3122-3143
{
  let mut method: jmethodID;                                           // C 3127
  let mut window: jobject;                                             // C 3128
  let mut pixel: jint;                                                 // C 3129

  /* C leaves all three uninitialised (PORTING.md 1.5).  */
  method = std::ptr::null_mut ();
  window = std::ptr::null_mut ();
  pixel = 0;

  window = android_resolve_handle (handle);                            // C 3131

  if value_mask & ANDROID_CW_BACK_PIXEL != 0                           // C 3133
    {
      method = window_class.change_window_background;                  // C 3135
      pixel = (*attrs).background_pixel as jint;                       // C 3136
      ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())      // C 3137
	(android_java_env,                                            // C 3138
	 window,                                                     // C 3139
	 window_class.class,                                         // C 3140
	 method, pixel);
      android_exception_check ();                                      // C 3141
    }
}

/* C 3163's `__builtin_expect (global == NULL, 0)` - PORTING.md 2.6: the builtin is kept as the
   branch-hint helper the C spells, and the hint itself has no Rust counterpart (the deviation is
   in the ledger's slice section).  The shape is `femacs/lib/memmem.rs` 32-35, the other unit that
   carries this builtin; C's builtin is not a header macro, so the port defines it at its first use
   site in the unit.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! __builtin_expect {
  ($cond:expr, $expected:expr) => { $cond };
}

/* C 3144-3148: C 3144's blank, C 3145-3147's `/* Return a reference to the local reference HANDLE
   suitable for indefinite retention and save its value into HANDLE, deleting HANDLE, or signal an
   error if such a reference cannot be allocated.  */` comment and the blank.  */

/* C 3149-3172: `android_globalize_reference` (C's `static android_handle`; called from C 3222 and
   the asset-notification group further down).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_globalize_reference (handle: jobject) -> android_handle   // C 3149-3172
{
  let mut global: jobject;                                             // C 3152

  global = std::ptr::null_mut ();                                      // (C leaves `global` uninitialised; PORTING.md 1.5)

  /* Though Android 8.0 and later can support an unlimited number of
     active local references, they remain inappropriate in threading
     configurations for being local to the current thread.  */         // C 3154-3156

  global = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,   // C 3158-3159
							    handle);
  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 3160
  ANDROID_DELETE_LOCAL_REF! (handle);                                  // C 3161

  if __builtin_expect! (global.is_null (), 0)                          // C 3163
    { error (c"JNI global reference reserves exhausted".as_ptr ()); }   // C 3164

  /* Save the value of this handle into HANDLE.  */                    // C 3166
  ((*(*android_java_env)).SetLongField.unwrap ()) (android_java_env, global,   // C 3167
						   handle_class.handle,
						   global as jlong);                 // C 3169
  const _: () = core::assert! (std::mem::size_of::<jlong> ()
			       >= std::mem::size_of::<intptr_t> ());       // C 3170 (C's `static_assert`)
  return global as intptr_t;                                           // C 3171
}

/* C 3173-3176: C 3173's blank, C 3174-3175's `/* Create a new window with the given width, height
   and attributes.  */` comment and the blank.  */

/* C 3177-3225: `android_create_window` (non-`static`; the mirror does not declare it - C's own
   non-`static` definition is what backs `#[unsafe(no_mangle)]`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_create_window (parent: android_window, x: c_int, y: c_int,
						width: c_int, height: c_int,
						value_mask: android_window_value_mask,
						attrs: *mut android_set_window_attributes)
  -> android_window                                                    // C 3177-3225
{
  static mut class: jclass = std::ptr::null_mut ();                    // C 3183 (tentative, i.e. zero)
  static mut constructor: jmethodID = std::ptr::null_mut ();           // C 3184 (tentative, i.e. zero)
  let mut object: jobject;                                             // C 3185
  let mut parent_object: jobject;                                      // C 3185
  let mut old: jobject;                                                // C 3185
  let mut window: android_window;                                      // C 3186
  let mut override_redirect: bool;                                     // C 3187

  /* C leaves the five locals uninitialised (PORTING.md 1.5).  */
  object = std::ptr::null_mut ();
  parent_object = std::ptr::null_mut ();
  old = std::ptr::null_mut ();
  window = 0;
  override_redirect = false;

  parent_object = android_resolve_handle (parent);                     // C 3189

  if class.is_null ()                                                  // C 3192
    {
      class
	= ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,   // C 3194-3195
							c"org/femacs/EmacsWindow".as_ptr ());   // C 3195
      eassert! (! class.is_null ());                                   // C 3196

      constructor
	= ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env, class,   // C 3198
							  c"<init>".as_ptr (),
							  c"(Lorg/femacs/EmacsWindow;IIIIZ)V".as_ptr ());   // C 3199-3201
      eassert! (! constructor.is_null ());                             // C 3202

      old = class;                                                     // C 3204
      class = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, class);   // C 3205
      android_exception_check_1 (old);                                 // C 3206
      ANDROID_DELETE_LOCAL_REF! (old);                                 // C 3207
    }

  /* N.B. that ANDROID_CW_OVERRIDE_REDIRECT can only be set at window
     creation time.  */                                               // C 3210-3211
  override_redirect = ((value_mask
			& ANDROID_CW_OVERRIDE_REDIRECT
			!= 0)
		       && (*attrs).override_redirect);                   // C 3212-3214

  object = ((*(*android_java_env)).NewObject.unwrap ()) (android_java_env, class,   // C 3216
							 constructor, parent_object,
							 x as jint, y as jint,              // C 3218
							 width as jint, height as jint,    // C 3219
							 override_redirect as jboolean as c_uint);   // C 3220 (C's default argument promotion)
  android_exception_check ();                                          // C 3221
  window = android_globalize_reference (object);                       // C 3222
  android_change_window_attributes (window, value_mask, attrs);        // C 3223
  return window;                                                       // C 3224
}

/* C 3227-3235: `android_set_window_background`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_window_background (window: android_window,
							pixel: c_ulong)        // C 3227-3235
{
  let mut attrs: android_set_window_attributes;                        // C 3230

  attrs = std::mem::zeroed ();                                         // (C leaves `attrs` uninitialised; PORTING.md 1.5)

  attrs.background_pixel = pixel;                                      // C 3232
  android_change_window_attributes (window, ANDROID_CW_BACK_PIXEL,     // C 3233
				    &raw mut attrs);                           // C 3234
}

/* C 3237-3241: `android_destroy_window`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_destroy_window (window: android_window)   // C 3237-3241
{
  android_destroy_handle (window);                                     // C 3240
}

/* C 3243-3268: `android_init_android_rect_class` (C's `static void`; only called from C 6099).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_android_rect_class ()                          // C 3243-3268
{
  let mut old: jclass;                                                 // C 3246

  old = std::ptr::null_mut ();                                         // (C leaves `old` uninitialised; PORTING.md 1.5)

  if ! android_rect_class.is_null ()                                   // C 3248
    /* Already initialized.  */                                        // C 3249
    { return; }                                                        // C 3250

  android_rect_class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,   // C 3252-3253
						    c"android/graphics/Rect".as_ptr ());   // C 3254
  eassert! (! android_rect_class.is_null ());                           // C 3255

  android_rect_constructor
    = ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env, android_rect_class,   // C 3257
						      c"<init>".as_ptr (),
						      c"(IIII)V".as_ptr ());                // C 3259
  eassert! (! emacs_gc_constructor.is_null ());                        // C 3260 (C's own expression, kept as written)

  old = android_rect_class;                                            // C 3262
  android_rect_class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,   // C 3263-3264
						       android_rect_class as jobject) as jclass;   // C 3265
  android_exception_check_1 (old);                                     // C 3266
  ANDROID_DELETE_LOCAL_REF! (old);                                     // C 3267
}
/* C 3269-3366: `android_init_emacs_gc_class` (C's `static void`; called from C 3375, 3431,
   3614 and the GC group further down).  C spells the fifteen `GetFieldID` calls out longhand
   (there is no macro in C 3303-3365), so the port does too.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_emacs_gc_class ()                        // C 3269-3366
{
  let mut old: jclass;                                                  // C 3273

  old = std::ptr::null_mut ();                                     // (C leaves `old` uninitialised; PORTING.md 1.5) 

  if ! emacs_gc_class.is_null ()                                   // C 3275 
    /* Already initialized.  */                                    // C 3276 
    { return; }                                                   // C 3277 

  emacs_gc_class                                                        
	= ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,   // C 3279-3280
						    c"org/femacs/EmacsGC".as_ptr ());   // C 3281                 
  eassert! (! emacs_gc_class.is_null ());                           // C 3282 

  emacs_gc_constructor                                                  
	= ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,   // C 3284-3285
						      emacs_gc_class,                                             
						      c"<init>".as_ptr (),                                        
						      c"()V".as_ptr ());                  // C 3287               
  eassert! (! emacs_gc_constructor.is_null ());                     // C 3288 

  emacs_gc_mark_dirty                                                   
	= ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,   // C 3290-3291
						      emacs_gc_class,                                             
						      c"markDirty".as_ptr (),                                     
						      c"(Z)V".as_ptr ());                 // C 3293               
  eassert! (! emacs_gc_mark_dirty.is_null ());                      // C 3294 

  old = emacs_gc_class;                                             // C 3296 
  emacs_gc_class                                                        
	= ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,   // C 3297-3298
							  emacs_gc_class as jobject) as jclass;   // C 3299              
  android_exception_check_1 (old);                                  // C 3300 
  ANDROID_DELETE_LOCAL_REF! (old);                                  // C 3301 

  emacs_gc_foreground
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3303-3304
						     emacs_gc_class,                                              
						     c"foreground".as_ptr (),                                     
						     c"I".as_ptr ());                                             
  emacs_gc_background
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3307-3308
						     emacs_gc_class,                                              
						     c"background".as_ptr (),                                     
						     c"I".as_ptr ());                                             
  emacs_gc_function
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3311-3312
						     emacs_gc_class,                                              
						     c"function".as_ptr (),                                       
						     c"I".as_ptr ());                                             
  emacs_gc_clip_rects
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3315-3316
						     emacs_gc_class,                                              
						     c"clip_rects".as_ptr (),                                     
						     c"[Landroid/graphics/Rect;".as_ptr ());                      
  emacs_gc_clip_x_origin
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3320-3321
						     emacs_gc_class,                                              
						     c"clip_x_origin".as_ptr (),                                  
						     c"I".as_ptr ());                                             
  emacs_gc_clip_y_origin
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3324-3325
						     emacs_gc_class,                                              
						     c"clip_y_origin".as_ptr (),                                  
						     c"I".as_ptr ());                                             
  emacs_gc_stipple
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3328-3329
						     emacs_gc_class,                                              
						     c"stipple".as_ptr (),                                        
						     c"Lorg/femacs/EmacsPixmap;".as_ptr ());                      
  emacs_gc_clip_mask
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3333-3334
						     emacs_gc_class,                                              
						     c"clip_mask".as_ptr (),                                      
						     c"Lorg/femacs/EmacsPixmap;".as_ptr ());                      
  emacs_gc_fill_style
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3338-3339
						     emacs_gc_class,                                              
						     c"fill_style".as_ptr (),                                     
						     c"I".as_ptr ());                                             
  emacs_gc_ts_origin_x
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3342-3343
						     emacs_gc_class,                                              
						     c"ts_origin_x".as_ptr (),                                    
						     c"I".as_ptr ());                                             
  emacs_gc_ts_origin_y
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3346-3347
						     emacs_gc_class,                                              
						     c"ts_origin_y".as_ptr (),                                    
						     c"I".as_ptr ());                                             
  emacs_gc_line_style
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3350-3351
						     emacs_gc_class,                                              
						     c"line_style".as_ptr (),                                     
						     c"I".as_ptr ());                                             
  emacs_gc_line_width
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3354-3355
						     emacs_gc_class,                                              
						     c"line_width".as_ptr (),                                     
						     c"I".as_ptr ());                                             
  emacs_gc_dash_offset
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3358-3359
						     emacs_gc_class,                                              
						     c"dash_offset".as_ptr (),                                    
						     c"I".as_ptr ());                                             
  emacs_gc_dashes
	= ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,   // C 3362-3363
						     emacs_gc_class,                                              
						     c"dashes".as_ptr (),                                         
						     c"[I".as_ptr ());                                            
}

/* C 3368-3409: `android_create_gc`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_create_gc (mask: android_gc_value_mask,
					    values: *mut android_gc_values)
  -> *mut android_gc                                                    // C 3368-3409
{
  let mut gc: *mut android_gc;                                          // C 3372
  let mut object: jobject;                                              // C 3373

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  gc = std::ptr::null_mut ();
  object = std::ptr::null_mut ();

  android_init_emacs_gc_class ();                                       // C 3375

  gc = xmalloc (std::mem::size_of::<android_gc> ()) as *mut android_gc;   // C 3377
  (*gc).gcontext   = 0;                                                 // C 3378
  (*gc).foreground = 0;                                                 // C 3379
  (*gc).background = 0xffffff;                                          // C 3380
  (*gc).clip_rects = std::ptr::null_mut ();                             // C 3381

  /* This means to not apply any clipping.  */                           // C 3383
  (*gc).num_clip_rects = -1;                                            // C 3384

  /* Apply the other default values.  */                                // C 3386
  (*gc).function = ANDROID_GC_COPY;                                     // C 3387
  (*gc).fill_style = ANDROID_FILL_SOLID;                                // C 3388
  (*gc).clip_x_origin = 0;                                              // C 3389
  (*gc).clip_y_origin = 0;                                              // C 3390
  (*gc).clip_mask = ANDROID_NONE as android_pixmap;                     // C 3391
  (*gc).stipple = ANDROID_NONE as android_pixmap;                       // C 3392
  (*gc).ts_x_origin = 0;                                                // C 3393
  (*gc).ts_y_origin = 0;                                                // C 3394
  (*gc).line_style = ANDROID_LINE_SOLID;                                // C 3395
  (*gc).line_width = 0;                                                 // C 3396
  (*gc).dash_offset = 0;                                                // C 3397
  (*gc).dashes = std::ptr::null_mut ();                                 // C 3398
  (*gc).n_segments = 0;                                                 // C 3399

  object = ((*(*android_java_env)).NewObject.unwrap ()) (android_java_env,   // C 3401
							 emacs_gc_class,
							 emacs_gc_constructor);
  android_exception_check ();                                           // C 3404

  (*gc).gcontext = android_globalize_reference (object);                // C 3406
  android_change_gc (gc, mask, values);                                 // C 3407
  return gc;                                                            // C 3408
}

/* C 3411-3419: `android_free_gc`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_free_gc (gc: *mut android_gc)        // C 3411-3419
{
  android_destroy_handle ((*gc).gcontext);                              // C 3414

  xfree ((*gc).dashes as *mut c_void);                                  // C 3416
  xfree ((*gc).clip_rects as *mut c_void);                              // C 3417
  xfree (gc as *mut c_void);                                            // C 3418
}

/* C 3421-3601: `android_change_gc` (non-`static`; called from C 3407 and the drawing helpers).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_change_gc (gc: *mut android_gc, mut mask: android_gc_value_mask,
					    values: *mut android_gc_values)  // C 3421-3601
{
  let mut what: jobject;                                                // C 3426
  let mut gcontext: jobject;                                            // C 3426
  let mut array: jobject;                                               // C 3426
  let mut clip_changed: jboolean;                                       // C 3427

  /* C leaves the three pointers uninitialised (PORTING.md 1.5).  */
  what = std::ptr::null_mut ();
  gcontext = std::ptr::null_mut ();
  array = std::ptr::null_mut ();

  clip_changed = false as jboolean;                                     // C 3429 (C's `false`, PORTING.md 1.5)

  android_init_emacs_gc_class ();                                       // C 3431
  gcontext = android_resolve_handle ((*gc).gcontext);                   // C 3432

  if mask & ANDROID_GC_FOREGROUND != 0                                  // C 3434
    {
      ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3436-3437
						      gcontext,
						      emacs_gc_foreground,
						      (*values).foreground as jint);      // C 3439
      (*gc).foreground = (*values).foreground;                          // C 3440
    }

  if mask & ANDROID_GC_BACKGROUND != 0                                  // C 3443
    {
      ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3445-3446
						      gcontext,
						      emacs_gc_background,
						      (*values).background as jint);      // C 3448
      (*gc).background = (*values).background;                          // C 3449
    }

  if mask & ANDROID_GC_FUNCTION != 0                                    // C 3452
    {
      ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3454-3455
						      gcontext,
						      emacs_gc_function,
						      (*values).function);                // C 3457
      (*gc).function = (*values).function;                              // C 3458
    }

  if mask & ANDROID_GC_CLIP_X_ORIGIN != 0                               // C 3461
    {
      ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3463-3464
						      gcontext,
						      emacs_gc_clip_x_origin,
						      (*values).clip_x_origin);           // C 3466
      (*gc).clip_x_origin = (*values).clip_x_origin;                    // C 3467
      clip_changed = true as jboolean;                                 // C 3468 (C's `true`)
    }

  if mask & ANDROID_GC_CLIP_Y_ORIGIN != 0                               // C 3471
    {
      ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3473-3474
						      gcontext,
						      emacs_gc_clip_y_origin,
						      (*values).clip_y_origin);           // C 3476
      (*gc).clip_y_origin = (*values).clip_y_origin;                    // C 3477
      clip_changed = true as jboolean;                                 // C 3478 (C's `true`)
    }

  if mask & ANDROID_GC_CLIP_MASK != 0                                   // C 3481
    {
      what = android_resolve_handle ((*values).clip_mask);              // C 3483
      ((*(*android_java_env)).SetObjectField.unwrap ()) (android_java_env,   // C 3484-3485
							gcontext,
							emacs_gc_clip_mask,
							what);                          // C 3487
      (*gc).clip_mask = (*values).clip_mask;                            // C 3488

      /* Changing GCClipMask also clears the clip rectangles.  */       // C 3490
      ((*(*android_java_env)).SetObjectField.unwrap ()) (android_java_env,   // C 3491-3492
							gcontext,
							emacs_gc_clip_rects,
							std::ptr::null_mut ());         // C 3494

      xfree ((*gc).clip_rects as *mut c_void);                          // C 3496
      (*gc).clip_rects = std::ptr::null_mut ();                         // C 3497
      (*gc).num_clip_rects = -1;                                        // C 3498
      clip_changed = true as jboolean;                                 // C 3499 (C's `true`)
    }

  if mask & ANDROID_GC_STIPPLE != 0                                     // C 3502
    {
      what = android_resolve_handle ((*values).stipple);                // C 3504
      ((*(*android_java_env)).SetObjectField.unwrap ()) (android_java_env,   // C 3505-3506
							gcontext,
							emacs_gc_stipple,
							what);                          // C 3508
      (*gc).stipple = (*values).stipple;                                // C 3509
    }

  if mask & ANDROID_GC_FILL_STYLE != 0                                  // C 3512
    {
      ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3514-3515
						      gcontext,
						      emacs_gc_fill_style,
						      (*values).fill_style);              // C 3517
      (*gc).fill_style = (*values).fill_style;                          // C 3518
    }

  if mask & ANDROID_GC_TILE_STIP_X_ORIGIN != 0                          // C 3521
    {
      ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3523-3524
						      gcontext,
						      emacs_gc_ts_origin_x,
						      (*values).ts_x_origin);             // C 3526
      (*gc).ts_x_origin = (*values).ts_x_origin;                        // C 3527
    }

  if mask & ANDROID_GC_TILE_STIP_Y_ORIGIN != 0                          // C 3530
    {
      ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3532-3533
						      gcontext,
						      emacs_gc_ts_origin_y,
						      (*values).ts_y_origin);             // C 3535
      (*gc).ts_y_origin = (*values).ts_y_origin;                        // C 3536
    }

  if mask & ANDROID_GC_LINE_STYLE != 0                                  // C 3539
    {
      ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3541-3542
						      gcontext,
						      emacs_gc_line_style,
						      (*values).line_style);              // C 3544
      (*gc).line_style = (*values).line_style;                          // C 3545
    }

  if mask & ANDROID_GC_LINE_WIDTH != 0                                  // C 3548
    {
      ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3550-3551
						      gcontext,
						      emacs_gc_line_width,
						      (*values).line_width);              // C 3553
      (*gc).line_width = (*values).line_width;                          // C 3554
    }

  if mask & ANDROID_GC_DASH_OFFSET != 0                                 // C 3557
    {
      ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3559-3560
						      gcontext,
						      emacs_gc_dash_offset,
						      (*values).dash_offset);             // C 3562
      (*gc).dash_offset = (*values).dash_offset;                        // C 3563
    }

  if mask & ANDROID_GC_DASH_LIST != 0                                   // C 3566
    {
      /* Compare the new dash pattern with the old.  */                 // C 3568
      if ! (*gc).dashes.is_null () && (*gc).n_segments == 1             // C 3569
	  && *(*gc).dashes == (*values).dash                       // C 3570
	/* If they be identical, nothing needs to change.  */          // C 3571
	{ mask &= ! ANDROID_GC_DASH_LIST; }                           // C 3572
      else
	{
	  if (*gc).n_segments != 1
	    { (*gc).dashes = xrealloc ((*gc).dashes as *mut c_void,   // C 3575-3576
				       std::mem::size_of::<c_int> ()) as *mut c_int; }
	  (*gc).n_segments = 1;                                         // C 3577
	  *(*gc).dashes = (*values).dash;                               // C 3578
	  array = ((*(*android_java_env)).NewIntArray.unwrap ()) (android_java_env, 1);   // C 3579
	  android_exception_check ();                                   // C 3580
	  ((*(*android_java_env)).SetIntArrayRegion.unwrap ())          // C 3581
	    (android_java_env,
	     array, 0, 1,
	     (&raw mut (*values).dash) as *mut jint);                   // C 3583
	  ((*(*android_java_env)).SetObjectField.unwrap ()) (android_java_env,   // C 3584-3585
							     gcontext,
							     emacs_gc_dashes,
							     array);                // C 3587
	  ANDROID_DELETE_LOCAL_REF! (array);                            // C 3588
	}
    }

  if mask != 0                                                          // C 3592
    {
      ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())       // C 3594
	(android_java_env,
	 gcontext,
	 emacs_gc_class,
	 emacs_gc_mark_dirty,
	 clip_changed as jboolean as c_uint);                 // C 3598 (C's default argument promotion)
      android_exception_check ();                                       // C 3599
    }
}

/* C 3602-3680: `android_set_clip_rectangles` (non-`static`; called from the toolkit's
   `set_clip_rectangles` hook).  C 3624's `for (i = 0; i < n_clip_rects; ++i)` is the port's
   `for i in 0..n_clip_rects` (the `sfntfont-android.rs` 723 / 883 device), so C 3611's `int i`
   has no declaration of its own.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_clip_rectangles (gc: *mut android_gc, clip_x_origin: c_int,
						      clip_y_origin: c_int,
						      clip_rects: *mut android_rectangle,
						      n_clip_rects: c_int)   // C 3602-3680
{
  let mut array: jobjectArray;                                          // C 3609
  let mut rect: jobject;                                                // C 3610
  let mut gcontext: jobject;                                            // C 3610

  /* C leaves the three locals uninitialised (PORTING.md 1.5); C 3611's `int i` is the loop's.  */
  array = std::ptr::null_mut ();
  rect = std::ptr::null_mut ();
  gcontext = std::ptr::null_mut ();

  android_init_android_rect_class ();                                   // C 3613
  android_init_emacs_gc_class ();                                       // C 3614

  gcontext = android_resolve_handle ((*gc).gcontext);                   // C 3616

  array = ((*(*android_java_env)).NewObjectArray.unwrap ()) (android_java_env,   // C 3618
							     n_clip_rects,
							     android_rect_class,
							     std::ptr::null_mut ());   // C 3621
  android_exception_check ();                                           // C 3622

  for i in 0..n_clip_rects                                             // C 3624
    {
      rect = ((*(*android_java_env)).NewObject.unwrap ()) (android_java_env,   // C 3626
							   android_rect_class,
							   android_rect_constructor,
							   (*clip_rects.offset (i as isize)).x as jint,   // C 3629
							   (*clip_rects.offset (i as isize)).y as jint,   // C 3630
							   ((*clip_rects.offset (i as isize)).x as c_uint
							    + (*clip_rects.offset (i as isize)).width) as jint,   // C 3631-3632 (C's int-to-unsigned conversion)
							   ((*clip_rects.offset (i as isize)).y as c_uint
							    + (*clip_rects.offset (i as isize)).height) as jint);   // C 3633-3634 (C's int-to-unsigned conversion)

      /* The meaning of this call is to check whether or not an
	 allocation error happened, and to delete ARRAY and signal an
	 out-of-memory error if that is the case.  */                 // C 3636-3638
      android_exception_check_1 (array);                                // C 3639

      ((*(*android_java_env)).SetObjectArrayElement.unwrap ())          // C 3641
	(android_java_env,
	 array, i, rect);                                        // C 3642
      ANDROID_DELETE_LOCAL_REF! (rect);                                 // C 3643
    }

  ((*(*android_java_env)).SetObjectField.unwrap ()) (android_java_env,   // C 3646
						     gcontext,
						     emacs_gc_clip_rects,
						     array as jobject);         // C 3649
  ANDROID_DELETE_LOCAL_REF! (array);                                    // C 3650

  ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3652
						  gcontext,
						  emacs_gc_clip_x_origin,
						  clip_x_origin);                     // C 3655
  ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3656
						  gcontext,
						  emacs_gc_clip_y_origin,
						  clip_y_origin);                     // C 3659

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 3661
    (android_java_env,
     gcontext,
     emacs_gc_class,
     emacs_gc_mark_dirty,
     true as jboolean as c_uint);                                       // C 3665 (C's default argument promotion)
  android_exception_check ();                                           // C 3666

  /* Cache the clip rectangles on the C side for
     sfntfont-android.c.  */                                           // C 3668-3669
  if ! (*gc).clip_rects.is_null ()                                      // C 3670
    { xfree ((*gc).clip_rects as *mut c_void); }                        // C 3671

  /* If gc->num_clip_rects is 0, then no drawing will be performed at
     all.  */                                                          // C 3673-3674
  (*gc).clip_rects = xmalloc (std::mem::size_of::<android_rectangle> ()   // C 3675
			      * n_clip_rects as usize) as *mut android_rectangle;   // C 3676
  (*gc).num_clip_rects = n_clip_rects;                                  // C 3677
  memcpy ((*gc).clip_rects as *mut c_void, clip_rects as *const c_void,   // C 3678
	  n_clip_rects as usize * std::mem::size_of::<android_rectangle> ());   // C 3679
}

/* C 3682-3749: `android_set_dashes` (non-`static`).  C 3695's `goto set_offset`, C 3738-3739's
   `if (array)` guard and the `set_offset:` label at C 3741 become the labelled block below and the
   tail after it (PORTING.md 2.2, the `android_create_lib_link` shape): `goto set_offset` is
   `break 'set_offset`.  C 3722's `static_assert (sizeof (int) == sizeof (jint))` is the port's
   in-body `const _: () = core::assert! (…)` (the `dispnew.rs` 6324-6325 shape) and owns a
   `macro-body-notes` row, the `android_globalize_reference` device of slice 15.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_dashes (gc: *mut android_gc, dash_offset: c_int,
					     dash_list: *mut c_int, n: c_int)   // C 3682-3749
{
  let mut array: jobject;                                               // C 3687
  let mut gcontext: jobject;                                            // C 3687

  /* C leaves both uninitialised (PORTING.md 1.5); C 3686's `int i` is the loop's.  */
  array = std::ptr::null_mut ();
  gcontext = std::ptr::null_mut ();

  gcontext = android_resolve_handle ((*gc).gcontext);                   // C 3689

  'set_offset: {
    if n == (*gc).n_segments                                         // C 3691
      && ((*gc).dashes.is_null ()                                    // C 3692
	  || memcmp ((*gc).dashes as *const c_void, dash_list as *const c_void,   // C 3692-3693
		     std::mem::size_of::<c_int> () * n as usize) == 0)   // C 3693
      /* No change in the dash list.  */                             // C 3694
      { break 'set_offset; }                                          // C 3695 (C's `goto set_offset`)

    if n == 0                                                         // C 3697
      {
	/* Reset the dash list to its initial empty state.  */        // C 3699
	xfree ((*gc).dashes as *mut c_void);                          // C 3700
	(*gc).dashes = std::ptr::null_mut ();                         // C 3701
	array = std::ptr::null_mut ();                                // C 3702
      }
    else
      {
	/* If the size of the array has not changed, it can be reused.  */   // C 3706

	if n != (*gc).n_segments                                      // C 3708
	  {
	    (*gc).dashes = xrealloc ((*gc).dashes as *mut c_void,     // C 3710
				     std::mem::size_of::<c_int> () * n as usize) as *mut c_int;
	    array = ((*(*android_java_env)).NewIntArray.unwrap ())    // C 3711
	      (android_java_env, n);
	    android_exception_check ();                               // C 3712
	  }
	else
	  { array = ((*(*android_java_env)).GetObjectField.unwrap ())   // C 3715
	      (android_java_env,
	       gcontext,
	       emacs_gc_dashes); }                                    // C 3717

	/* Copy the list of segments into both arrays.  */            // C 3719
	for i in 0..n                                                 // C 3720
	  { *(*gc).dashes.offset (i as isize) = *dash_list.offset (i as isize); }   // C 3721
	const _: () = core::assert! (std::mem::size_of::<c_int> ()    // C 3722 (C's `static_assert`)
			     == std::mem::size_of::<jint> ());
	((*(*android_java_env)).SetIntArrayRegion.unwrap ())          // C 3723
	  (android_java_env,
	   array, 0, n,
	   dash_list as *mut jint);                                    // C 3725
      }

    /* Replace the dash array in the GContext object if required.  */   // C 3728
    if n != (*gc).n_segments                                          // C 3729
      {
	((*(*android_java_env)).SetObjectField.unwrap ())            // C 3731
	  (android_java_env,
	   gcontext,
	   emacs_gc_dashes,
	   array);                                                    // C 3734
	(*gc).n_segments = n;                                         // C 3735
      }

    if ! array.is_null ()                                             // C 3738
      { ANDROID_DELETE_LOCAL_REF! (array); }                          // C 3739
  }

  /* set_offset: */
  /* And the offset.  */                                                // C 3742
  if dash_offset != (*gc).dash_offset                                   // C 3743
    { ((*(*android_java_env)).SetIntField.unwrap ()) (android_java_env,   // C 3744
						    gcontext,
						    emacs_gc_dash_offset,
						    dash_offset);             // C 3747
    }
  (*gc).dash_offset = dash_offset;                                      // C 3748
}

/* C 3750-3766: `android_reparent_window`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_reparent_window (w: android_window,
						  parent_handle: android_window,
						  x: c_int, y: c_int)         // C 3750-3766
{
  let mut window: jobject;                                              // C 3755
  let mut parent: jobject;                                              // C 3755
  let mut method: jmethodID;                                            // C 3756

  /* C leaves the three locals uninitialised (PORTING.md 1.5).  */
  window = std::ptr::null_mut ();
  parent = std::ptr::null_mut ();
  method = std::ptr::null_mut ();

  window = android_resolve_handle (w);                                  // C 3758
  parent = android_resolve_handle (parent_handle);                      // C 3759

  method = window_class.reparent_to;                                    // C 3761
  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 3762
    (android_java_env, window,
     window_class.class, method,
     parent, x as jint, y as jint);                                     // C 3764
  android_exception_check ();                                           // C 3765
}

/* C 3768-3780: `android_clear_window`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_clear_window (handle: android_window)   // C 3768-3780
{
  let mut window: jobject;                                              // C 3771

  window = std::ptr::null_mut ();                                       // (C leaves `window` uninitialised; PORTING.md 1.5)

  window = android_resolve_handle (handle);                             // C 3773

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 3775
    (android_java_env,
     window,
     window_class.class,
     window_class.clear_window);                                        // C 3778
  android_exception_check ();                                           // C 3779
}

/* C 3782-3796: `android_map_window`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_map_window (handle: android_window)   // C 3782-3796
{
  let mut window: jobject;                                              // C 3785
  let mut map_window: jmethodID;                                        // C 3786

  /* C leaves both locals uninitialised (PORTING.md 1.5).  */
  window = std::ptr::null_mut ();
  map_window = std::ptr::null_mut ();

  window = android_resolve_handle (handle);                             // C 3788
  map_window = window_class.map_window;                                 // C 3789

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 3791
    (android_java_env,
     window,
     window_class.class,
     map_window);                                                       // C 3794
  android_exception_check ();                                           // C 3795
}

/* C 3798-3812: `android_unmap_window`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_unmap_window (handle: android_window)   // C 3798-3812
{
  let mut window: jobject;                                              // C 3801
  let mut unmap_window: jmethodID;                                      // C 3802

  /* C leaves both locals uninitialised (PORTING.md 1.5).  */
  window = std::ptr::null_mut ();
  unmap_window = std::ptr::null_mut ();

  window = android_resolve_handle (handle);                             // C 3804
  unmap_window = window_class.unmap_window;                             // C 3805

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 3807
    (android_java_env,
     window,
     window_class.class,
     unmap_window);                                                     // C 3810
  android_exception_check ();                                           // C 3811
}

/* C 3814-3831: `android_resize_window` (C's `unsigned int` width/height, cast to `jint` at the
   call).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_resize_window (handle: android_window, width: c_uint,
						height: c_uint)     // C 3814-3831
{
  let mut window: jobject;                                              // C 3818
  let mut resize_window: jmethodID;                                     // C 3819

  /* C leaves both locals uninitialised (PORTING.md 1.5).  */
  window = std::ptr::null_mut ();
  resize_window = std::ptr::null_mut ();

  window = android_resolve_handle (handle);                             // C 3821
  resize_window = window_class.resize_window;                           // C 3822

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 3824
    (android_java_env,
     window,
     window_class.class,
     resize_window,
     width as jint,                                                     // C 3828
     height as jint);                                                   // C 3829
  android_exception_check ();                                           // C 3830
}

/* C 3833-3848: `android_move_window`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_move_window (handle: android_window, x: c_int, y: c_int)   // C 3833-3848
{
  let mut window: jobject;                                              // C 3836
  let mut move_window: jmethodID;                                       // C 3837

  /* C leaves both locals uninitialised (PORTING.md 1.5).  */
  window = std::ptr::null_mut ();
  move_window = std::ptr::null_mut ();

  window = android_resolve_handle (handle);                             // C 3839
  move_window = window_class.move_window;                               // C 3840

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 3842
    (android_java_env,
     window,
     window_class.class,
     move_window,
     x as jint, y as jint);                                             // C 3846
  android_exception_check ();                                           // C 3847
}

/* C 3850-3866: `android_swap_buffers`.  C 3857's `for (i = 0; i < num_windows; ++i)` is the port's
   `for i in 0..num_windows` (the `sfntfont-android.rs` 723 device), so C 3855's `int i` has no
   declaration of its own, and C 3859's `swap_info[i]` is `(*swap_info.offset (i as isize))` (the
   raw pointer has no `Index`; the `androidterm.rs` 2524 device).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_swap_buffers (swap_info: *mut android_swap_info,
						  num_windows: c_int)   // C 3850-3866
{
  let mut window: jobject;                                              // C 3854

  window = std::ptr::null_mut ();                                       // (C leaves `window` uninitialised; PORTING.md 1.5)

  for i in 0..num_windows                                               // C 3857
    {
      window = android_resolve_handle ((*swap_info.offset (i as isize)).swap_window);   // C 3859
      ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())       // C 3860
	(android_java_env,
	 window,
	 window_class.class,
	 window_class.swap_buffers);                            // C 3863
      android_exception_check ();                                       // C 3864
    }
}

/* C 3868-3902: `android_get_gc_values` (C's `if (mask & X)` chain, whose last arm carries the
   three-line comment; the handle-carrying fields are the ones C 3899-3901 skip).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_get_gc_values (gc: *mut android_gc,
						mask: android_gc_value_mask,
						values: *mut android_gc_values)   // C 3868-3902
{
  if mask & ANDROID_GC_FOREGROUND != 0                                  // C 3873
    /* GCs never have 32 bit colors, so we don't have to worry about
       sign extension here.  */                                        // C 3874-3875
    { (*values).foreground = (*gc).foreground; }                        // C 3876

  if mask & ANDROID_GC_BACKGROUND != 0                                  // C 3878
    { (*values).background = (*gc).background; }                        // C 3879

  if mask & ANDROID_GC_FUNCTION != 0                                    // C 3881
    { (*values).function = (*gc).function; }                            // C 3882

  if mask & ANDROID_GC_CLIP_X_ORIGIN != 0                               // C 3884
    { (*values).clip_x_origin = (*gc).clip_x_origin; }                  // C 3885

  if mask & ANDROID_GC_CLIP_Y_ORIGIN != 0                               // C 3887
    { (*values).clip_y_origin = (*gc).clip_y_origin; }                  // C 3888

  if mask & ANDROID_GC_FILL_STYLE != 0                                  // C 3890
    { (*values).fill_style = (*gc).fill_style; }                        // C 3891

  if mask & ANDROID_GC_TILE_STIP_X_ORIGIN != 0                          // C 3893
    { (*values).ts_x_origin = (*gc).ts_x_origin; }                      // C 3894

  if mask & ANDROID_GC_TILE_STIP_Y_ORIGIN != 0                          // C 3896
    { (*values).ts_y_origin = (*gc).ts_y_origin; }                      // C 3897

  /* Fields involving handles are not used by Emacs, and thus not
     implemented.  In addition, the size of GCClipMask and GCDashList is
     not static, precluding their retrieval.  */                       // C 3899-3901
}

/* C 3904-3911: `android_set_foreground`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_foreground (gc: *mut android_gc,
						 foreground: c_ulong)   // C 3904-3911
{
  let mut gcv: android_gc_values;                                       // C 3907

  gcv = std::mem::zeroed ();                                            // (C leaves `gcv` uninitialised; PORTING.md 1.5)

  gcv.foreground = foreground;                                          // C 3909
  android_change_gc (gc, ANDROID_GC_FOREGROUND, &raw mut gcv);          // C 3910
}

/* C 3912-3932: `android_fill_rectangle` (non-`static`; the toolkit's `fill_rectangle` hook).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_fill_rectangle (handle: android_drawable, gc: *mut android_gc,
						 x: c_int, y: c_int, width: c_uint,
						 height: c_uint)   // C 3912-3932
{
  let mut drawable: jobject;                                            // C 3918
  let mut gcontext: jobject;                                            // C 3918

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  drawable = std::ptr::null_mut ();
  gcontext = std::ptr::null_mut ();

  drawable = android_resolve_handle (handle);                           // C 3920
  gcontext = android_resolve_handle ((*gc).gcontext);                   // C 3921

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 3923
    (android_java_env,
     emacs_service,
     service_class.class,
     service_class.fill_rectangle,
     drawable,
     gcontext,
     x as jint, y as jint,                                              // C 3929
     width as jint,                                                     // C 3930
     height as jint);                                                   // C 3931
}

/* C 3934-4027: `android_create_pixmap_from_bitmap_data`.  C 3943's `AndroidBitmapInfo info` is
   this unit's five-member record (the C 63 stand-in) and C 3954's `android_lock_bitmap` takes the
   mirror's opaque tag, so the call casts through `AndroidBitmapInfo_tag` (the Owed 12 device).
   C 3965's `switch (info.format)`, its two `case`s, the two `break`s and its `default` become the
   `match` below (PORTING.md 2.2), which is what the `macro-body-notes` row for this function
   records; C's two nested `for`s are the port's ranges, and C 3985/3988/3991/4006/4009/4012's
   `data[...]`/`depth_24[...]`/`depth_8[...]` are `.offset (…)` (Rust's raw pointers have no
   `Index`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_create_pixmap_from_bitmap_data (mut data: *mut c_char,
								 width: c_uint,
								 height: c_uint,
								 mut foreground: c_ulong,
								 mut background: c_ulong,
								 depth: c_uint)
  -> android_pixmap                                                     // C 3934-4027
{
  let mut pixmap: android_pixmap;                                       // C 3941
  let mut object: jobject;                                              // C 3942
  let mut info: android_bitmap::AndroidBitmapInfo;                      // C 3943
  let mut depth_24: *mut c_uint;                                        // C 3944
  let mut depth_8: *mut c_uchar;                                        // C 3945
  let mut bitmap_data: *mut c_void;                                     // C 3946
  let mut r: c_uint;                                                    // C 3948
  let mut g: c_uint;                                                    // C 3948
  let mut b: c_uint;                                                    // C 3948

  /* C leaves the seven locals uninitialised (PORTING.md 1.5); C 3947's `x, y` are the loops'.  */
  pixmap = 0;
  object = std::ptr::null_mut ();
  info = std::mem::zeroed ();
  depth_24 = std::ptr::null_mut ();
  depth_8 = std::ptr::null_mut ();
  bitmap_data = std::ptr::null_mut ();
  r = 0;
  g = 0;
  b = 0;

  /* Create a pixmap with the right dimensions and depth.  */          // C 3950
  pixmap = android_create_pixmap (width, height, depth as c_int);      // C 3951

  /* Lock the bitmap data.  */                                         // C 3953
  bitmap_data = android_lock_bitmap (pixmap,                           // C 3954
				     (&raw mut info) as *mut AndroidBitmapInfo_tag,
				     &raw mut object) as *mut c_void;

  /* Merely return if locking the bitmap fails.  */                    // C 3956
  if bitmap_data.is_null ()                                            // C 3957
    { return pixmap; }                                                 // C 3958

  eassert! (info.format == ANDROID_BITMAP_FORMAT_RGBA_8888              // C 3960
	    || info.format == ANDROID_BITMAP_FORMAT_A_8);              // C 3961

  /* Begin copying each line.  */                                      // C 3963

  match info.format                                                    // C 3965
    {
      ANDROID_BITMAP_FORMAT_RGBA_8888 =>                               // C 3967
	{
	  /* Swizzle the pixels into ABGR format.  Android uses Skia's
	     ``native color type'', which is ABGR.  This is despite the
	     format being named ``ARGB'', and more confusingly
	     `ANDROID_BITMAP_FORMAT_RGBA_8888' in bitmap.h.  */      // C 3969-3972

	  r = background as c_uint & 0x00ff0000;                          // C 3974
	  g = background as c_uint & 0x0000ff00;                          // C 3975
	  b = background as c_uint & 0x000000ff;                          // C 3976
	  background = (r >> 16 | g | b << 16 | 0xff000000) as c_ulong;   // C 3977
	  r = foreground as c_uint & 0x00ff0000;                          // C 3978
	  g = foreground as c_uint & 0x0000ff00;                          // C 3979
	  b = foreground as c_uint & 0x000000ff;                          // C 3980
	  foreground = (r >> 16 | g | b << 16 | 0xff000000) as c_ulong;   // C 3981

	  for y in 0..height                                             // C 3983
	    {
	      depth_24 = (bitmap_data as *mut c_char)                     // C 3985
		.offset ((y * info.stride) as isize) as *mut c_uint;

	      for x in 0..width                                          // C 3987
		{
		  *depth_24.offset (x as isize)
		    = (if *(data.offset ((x / 8) as isize)) as c_int   // C 3988
			 & (1 << (x % 8)) != 0
			 { foreground } else { background }) as c_uint;
		}

	      data = data.offset (((width + 7) / 8) as isize);           // C 3991
	    }
	}

      ANDROID_BITMAP_FORMAT_A_8 =>                                     // C 3996
	{
	  /* 8-bit pixmaps are created, but in spite of that they are
	     employed only to represent bitmaps.  */                 // C 3998-3999

	  foreground = if foreground != 0 { 255 } else { 0 };             // C 4001
	  background = if background != 0 { 255 } else { 0 };             // C 4002

	  for y in 0..height                                             // C 4004
	    {
	      depth_8 = (bitmap_data as *mut c_char)                      // C 4006
		.offset ((y * info.stride) as isize) as *mut c_uchar;

	      for x in 0..width                                          // C 4008
		{
		  *depth_8.offset (x as isize)
		    = (if *(data.offset ((x / 8) as isize)) as c_int   // C 4009
			 & (1 << (x % 8)) != 0
			 { foreground } else { background }) as c_uchar;
		}

	      data = data.offset (((width + 7) / 8) as isize);           // C 4012
	    }
	}

      _ =>                                                             // C 4017 (C's `default:`)
	{ emacs_abort (); }                                             // C 4018
    }

  /* Unlock the bitmap itself.  */                                      // C 4021
  AndroidBitmap_unlockPixels (android_java_env, object);                // C 4022
  ANDROID_DELETE_LOCAL_REF! (object);                                   // C 4023

  /* Return the pixmap.  */                                            // C 4025
  return pixmap;                                                        // C 4026
}

/* C 4029-4036: `android_set_clip_mask`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_clip_mask (gc: *mut android_gc,
						pixmap: android_pixmap)   // C 4029-4036
{
  let mut gcv: android_gc_values;                                       // C 4032

  gcv = std::mem::zeroed ();                                            // (C leaves `gcv` uninitialised; PORTING.md 1.5)

  gcv.clip_mask = pixmap;                                               // C 4034
  android_change_gc (gc, ANDROID_GC_CLIP_MASK, &raw mut gcv);           // C 4035
}

/* C 4038-4046: `android_set_fill_style`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_fill_style (gc: *mut android_gc,
						fill_style: android_fill_style)   // C 4038-4046
{
  let mut gcv: android_gc_values;                                       // C 4042

  gcv = std::mem::zeroed ();                                            // (C leaves `gcv` uninitialised; PORTING.md 1.5)

  gcv.fill_style = fill_style;                                          // C 4044
  android_change_gc (gc, ANDROID_GC_FILL_STYLE, &raw mut gcv);          // C 4045
}

/* C 4734-4747: `android_create_pixmap`.  **Carried early** (the file's order puts it far below):
   C 3951 calls it and the mirror does not declare it, so a Rust crate needs the definition first -
   the C 3421 / Owed 13 lesson of slices 13 and 16, recorded as a two-region slice in the ledger.
   C's three `(jint)` casts of the `unsigned int` arguments are C's own narrowing.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_create_pixmap (width: c_uint, height: c_uint,
						depth: c_int)
  -> android_pixmap                                                    // C 4734-4747
{
  let mut object: jobject;                                              // C 4738

  object = std::ptr::null_mut ();                                       // (C leaves `object` uninitialised; PORTING.md 1.5)

  object = ((*(*android_java_env)).NewObject.unwrap ()) (android_java_env,   // C 4740
							 pixmap_class.class,
							 pixmap_class.constructor_mutable,
							 width as jint, height as jint,      // C 4743
							 depth as jint);                     // C 4744
  android_exception_check ();                                           // C 4745
  return android_globalize_reference (object);                          // C 4746
}

/* C 4047-4060: C 4047's blank, C 4048's form feed, the blank, C 4050-4052's `/* Pixmap bit blit
   implementation. … */` comment, the blank, C 4054's `/* Function called to actually perform the
   copy.  */` comment, the blank and C 4056-4060's `typedef void (*android_blit_func) (…)`.  A
   function-pointer type is `Option<unsafe extern "C" fn (…) -> …>` in the port (the `dbusbind.rs`
   181-193 / `decompress.rs` 98-102 device; the return type is `()`, so there is no arrow), and its
   three `AndroidBitmapInfo *` parameters are this unit's five-member record (the C 63 stand-in).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type android_blit_func = Option<unsafe extern "C" fn (c_int, c_int, c_int, c_int, c_int, c_int,
							   *mut android_gc,
							   *mut c_uchar,
							   *mut android_bitmap::AndroidBitmapInfo,
							   *mut c_uchar,
							   *mut android_bitmap::AndroidBitmapInfo,
							   *mut c_uchar,
							   *mut android_bitmap::AndroidBitmapInfo)>;

/* C 4061-4063: the blank, C 4062's form feed and the blank.  */

/* C 4064: `#ifdef __aarch64__` … C 4136: `#endif /* __aarch64__ */` - the first of this file's
   four NEON regions (the ledger's `aarch64` gate).  `core::arch::aarch64` carries the ACLE names
   C's `<arm_neon.h>` defines, so the calls keep C's names (the `femacs/src/sfntfont-android.rs`
   321-325 device); C 4073-4075's four vector locals are declared with the type their values have
   (see the deviations).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "aarch64"))]
use core::arch::aarch64::*;

/* C 4066-4068: C 4066-4067's comment `/* Copy N pixels from SRC to DST, using MASK as a depth 1
   clip mask.  */` and the blank.  */

/* C 4069-4134: `android_neon_mask_line` (C's `static void`; the address C 4470 passes to the blit
   dispatcher).  C 4085's `again:` label with C 4121's `goto again` is a **backward** jump, so the
   port makes it a labelled loop (`continue 'again`; the `alloc.rs` 7278 / `bidi.rs` 2210 shape) and
   C's fall out of the `if (n)` body when the condition fails is that loop's `break`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "aarch64"))]
unsafe fn android_neon_mask_line (mut src: *mut c_uint, mut dst: *mut c_uint,
				  mut mask: *mut c_uchar, mut n: c_int)   // C 4069-4134
{
  let mut src_low: uint32x4_t;                                          // C 4073
  let mut src_high: uint32x4_t;                                         // C 4073
  let mut dst_low: uint32x4_t;                                          // C 4073
  let mut dst_high: uint32x4_t;                                         // C 4073
  let mut vmask: int16x8_t;                                             // C 4074
  let mut ext_mask_low: int32x4_t;                                      // C 4075
  let mut ext_mask_high: int32x4_t;                                     // C 4075
  let mut low: uint32x4_t;                                              // C 4075 (C's `int32x4_t`, see the deviations)
  let mut high: uint32x4_t;                                             // C 4075 (C's `int32x4_t`, see the deviations)
  let mut rem: c_int;                                                   // C 4076
  let mut i: c_int;                                                     // C 4076

  /* C leaves the ten locals uninitialised (PORTING.md 1.5).  */
  src_low = std::mem::zeroed ();
  src_high = std::mem::zeroed ();
  dst_low = std::mem::zeroed ();
  dst_high = std::mem::zeroed ();
  vmask = std::mem::zeroed ();
  ext_mask_low = std::mem::zeroed ();
  ext_mask_high = std::mem::zeroed ();
  low = std::mem::zeroed ();
  high = std::mem::zeroed ();
  rem = 0;
  i = 0;

  /* Calculate the remainder.  */                                       // C 4078
  rem = n & 7; n &= ! 7;                                                // C 4079 (C's comma expression, split)

  /* Process eight pixels at a time.  */                                // C 4081

  if n != 0                                                             // C 4083
    {
      'again: loop                                                      // C 4085 (C's `again:` label)
	{
	  /* Load the low and high four pixels from the source.  */    // C 4086
	  src_low = vld1q_u32 (src);                                    // C 4087
	  src_high = vld1q_u32 (src.offset (4));                        // C 4088

	  /* Do the same with the destination.  */                     // C 4090
	  dst_low = vld1q_u32 (dst);                                    // C 4091
	  dst_high = vld1q_u32 (dst.offset (4));                        // C 4092

	  /* Load and sign extend the mask.  */                        // C 4094
	  vmask = vmovl_s8 (vreinterpret_s8_u8 (vld1_u8 (mask)));       // C 4095
	  ext_mask_low = vmovl_s16 (vget_low_s16 (vmask));              // C 4096
	  ext_mask_high = vmovl_s16 (vget_high_s16 (vmask));            // C 4097

	  /* Reinterpret the mask.  */                                 // C 4099
	  low = vreinterpretq_u32_s32 (ext_mask_low);                   // C 4100
	  high = vreinterpretq_u32_s32 (ext_mask_high);                 // C 4101

	  /* Apply the mask.  */                                       // C 4103
	  dst_low = vbicq_u32 (dst_low, low);                           // C 4104
	  src_low = vandq_u32 (src_low, low);                           // C 4105
	  dst_high = vbicq_u32 (dst_high, high);                        // C 4106
	  src_high = vandq_u32 (src_high, high);                        // C 4107

	  /* Write the result after combining both masked vectors.  */ // C 4109
	  vst1q_u32 (dst, vorrq_u32 (dst_low, src_low));                // C 4110
	  vst1q_u32 (dst.offset (4), vorrq_u32 (dst_high, src_high));   // C 4111

	  /* Adjust src, dst and mask.  */                             // C 4113
	  dst = dst.offset (8);                                         // C 4114
	  src = src.offset (8);                                         // C 4115
	  mask = mask.offset (8);                                       // C 4116

	  /* See if this loop should continue.  */                     // C 4118
	  n -= 8;                                                      // C 4119
	  if n > 0                                                      // C 4120
	    { continue 'again; }                                        // C 4121 (C's `goto again`)
	  else
	    { break; }                                                  // (C's fall out of the `if (n)` body)
	}
    }

  /* Process the remaining pixels.  */                                  // C 4124

  for i in 0..rem                                                       // C 4126
    {
      /* Sign extend the mask.  */                                      // C 4128
      n = *(mask as *mut core::ffi::c_schar).offset (i as isize) as c_int;   // C 4129

      /* Combine src and dst.  */                                       // C 4131
      *dst.offset (i as isize) = *src.offset (i as isize) & n as c_uint
	| (*dst.offset (i as isize) & ! (n as c_uint));             // C 4132
    }
}

/* C 4136's `#endif /* __aarch64__ */`.  */

/* C 4137-4150: C 4137's blank, C 4138's form feed, the blank, C 4140-4148's comment and C 4150's
   `static void`.  */

/* C 4151-4439: `android_blit_copy` (C's `static void`; C 4541 takes its address into an
   `android_blit_func`, so the item is an `unsafe extern "C" fn` - the androidterm.rs header's
   "whose address C takes" class).  C 4163-4165's `#ifndef __aarch64__ int j;` is the
   `not (feature = "aarch64")` local and C 4418-4431 is that condition's two arms.  C 4190-4250's
   `ckd_mul`/`ckd_add` chains are P-15's spelling (the wrapped value and the flag; the
   `image.rs` 6618-6622 / `androidselect.rs` 750-757 device), and C 4327-4331's four-operand `||`
   chain is written as C's short-circuit leaves it, with one `return` per overflow.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_blit_copy (src_x: c_int, mut src_y: c_int, mut width: c_int,
					 mut height: c_int, dst_x: c_int, mut dst_y: c_int,
					 gc: *mut android_gc,
					 src: *mut c_uchar, src_info: *mut android_bitmap::AndroidBitmapInfo,
					 dst: *mut c_uchar, dst_info: *mut android_bitmap::AndroidBitmapInfo,
					 mut mask: *mut c_uchar, mask_info: *mut android_bitmap::AndroidBitmapInfo)
                                                                        // C 4151-4439
{
  let mut start: uintptr_t;                                             // C 4157
  let mut end: uintptr_t;
  let mut mask_offset: c_int;                                           // C 4158
  let mut pixel: size_t;                                                // C 4159
  let mut offset: size_t;
  let mut offset1: size_t;
  let mut src_current: *mut c_uchar;                                    // C 4160
  let mut dst_current: *mut c_uchar;
  let mut mask_current: *mut c_uchar;                                   // C 4161
  let mut overflow: bool;                                               // C 4162 (C's `int` flag, only ever 0/1)
  let mut temp: c_int;                                                  // C 4162
  let mut i: c_int;
/* C 4163-4165's `#ifndef __aarch64__ int j;` has no declaration of its own: its only user
     is C 4419-4428's `for (j = 0; ...)` loop, which the port writes as a range (the slice 17/18
     device), and that loop sits inside the same gate.  */
  let mut backwards: bool;                                              // C 4166
  let mut long_src: *mut c_uint;                                        // C 4167
  let mut long_dst: *mut c_uint;

  /* C leaves the locals uninitialised (PORTING.md 1.5).  */
  start = 0;
  end = 0;
  mask_offset = 0;
  pixel = 0;
  offset = 0;
  offset1 = 0;
  src_current = std::ptr::null_mut ();
  dst_current = std::ptr::null_mut ();
  mask_current = std::ptr::null_mut ();
  overflow = false;
  temp = 0;
  i = 0;
  backwards = false;
  long_src = std::ptr::null_mut ();
  long_dst = std::ptr::null_mut ();

  /* Assert that the specified coordinates are within bounds.  */       // C 4169
  eassert! (src_x >= 0 && src_y >= 0                                    // C 4170
					     && dst_x >= 0 && dst_y >= 0);                                 // C 4171
  eassert! (src_x + width <= (*src_info).width as c_int);               // C 4172
  eassert! (src_y + height <= (*src_info).height as c_int);             // C 4173
  eassert! (dst_x + width <= (*dst_info).width as c_int);               // C 4174
  eassert! (dst_y + height <= (*dst_info).height as c_int);             // C 4175

  /* Now check that each bitmap has the correct format.  */             // C 4177
  eassert! ((*src_info).format == (*dst_info).format                    // C 4178
					     && (*src_info).format == ANDROID_BITMAP_FORMAT_RGBA_8888); // C 4179
  pixel = std::mem::size_of::<c_uint> () as size_t;                     // C 4180

  /* Android doesn't have A1 bitmaps, so A8 is used to represent        // C 4182
     packed bitmaps of depth 1.  */                                     // C 4183
  eassert! (mask.is_null () || (*mask_info).format                      // C 4184
	    == ANDROID_BITMAP_FORMAT_A_8);

  /* Calculate the address of the first pixel of the first row to be    // C 4186
     copied in both src and dst.  Compare them to determine the         // C 4187
     direction in which the copy is to take place.  */                  // C 4188

  /* C 4190-4193's four `ckd_*` calls (P-15's spelling).  */
  let (ckd_start, ckd_over) = (src_y as uintptr_t).overflowing_mul      // C 4190
	((*src_info).stride as uintptr_t);
  start = ckd_start;
  overflow = ckd_over;
  let (ckd_end, ckd_over) = (src_x as uintptr_t).overflowing_mul        // C 4191
	(pixel as uintptr_t);
  end = ckd_end;
  overflow |= ckd_over;
  let (ckd_start, ckd_over) = end.overflowing_add (start);              // C 4192
  start = ckd_start;
  overflow |= ckd_over;
  let (ckd_start, ckd_over) =                                           // C 4193
	(src as uintptr_t).overflowing_add (start);
  start = ckd_start;
  overflow |= ckd_over;

  if overflow                                                           // C 4195
    { return; }                                                         // C 4196

  src_current = start as *mut c_uchar;                                  // C 4198

  /* C 4200-4203, the same four calls for the destination.  */
  let (ckd_start, ckd_over) = (dst_y as uintptr_t).overflowing_mul      // C 4200
	((*dst_info).stride as uintptr_t);
  start = ckd_start;
  overflow = ckd_over;
  let (ckd_end, ckd_over) = (dst_x as uintptr_t).overflowing_mul        // C 4201
	(pixel as uintptr_t);
  end = ckd_end;
  overflow |= ckd_over;
  let (ckd_start, ckd_over) = end.overflowing_add (start);              // C 4202
  start = ckd_start;
  overflow |= ckd_over;
  let (ckd_start, ckd_over) =                                           // C 4203
	(dst as uintptr_t).overflowing_add (start);
  start = ckd_start;
  overflow |= ckd_over;

  if overflow                                                           // C 4205
    { return; }                                                         // C 4206

  dst_current = start as *mut c_uchar;                                  // C 4208
  backwards = false;                                                    // C 4209

  /* Now see if copying should proceed from the bottom up.  */          // C 4211

  if src == dst && dst_current >= src_current                           // C 4213
    {
      backwards = true;                                                 // C 4215

      /* Walk src and dst from bottom to top, in order to avoid         // C 4217
	 overlap.  Calculate the coordinate of the last pixel of the           // C 4218
	 last row in both src and dst.  */                                     // C 4219

      /* C 4221-4222's `overflow = ckd_mul (&start, src_y + height - 1, src_info->stride);`.  */
      let (ckd_start, ckd_over)
	= ((src_y + height - 1) as uintptr_t).overflowing_mul
	  ((*src_info).stride as uintptr_t);
      start = ckd_start;
      overflow = ckd_over;

      if ! mask.is_null ()                                              // C 4224
	/* If a mask is set, put the pointers before the end of the
	   row.  */                                                            // C 4225-4226
	{
          /* C 4227's `overflow |= ckd_mul (&end, src_x + width - 1, pixel);`.  */
          let (ckd_end, ckd_over)
	    = ((src_x + width - 1) as uintptr_t).overflowing_mul
	      (pixel as uintptr_t);
          end = ckd_end;
          overflow |= ckd_over;
	}
      else                                                              // C 4228
	{ end = (src_x as size_t * pixel) as uintptr_t; }                  // C 4229

      /* C 4231-4232's two `ckd_add` calls.  */
      let (ckd_start, ckd_over) = start.overflowing_add (end);
      start = ckd_start;
      overflow |= ckd_over;
      let (ckd_start, ckd_over)
	= (src as uintptr_t).overflowing_add (start);
      start = ckd_start;
      overflow |= ckd_over;

      if overflow                                                       // C 4234
	{ return; }                                                            // C 4235

      src_current = start as *mut c_uchar;                              // C 4237

      /* C 4239-4240's `overflow = ckd_mul (&start, dst_y + height - 1, dst_info->stride);`.  */
      let (ckd_start, ckd_over)
	= ((dst_y + height - 1) as uintptr_t).overflowing_mul
	  ((*dst_info).stride as uintptr_t);
      start = ckd_start;
      overflow = ckd_over;

      if ! mask.is_null ()                                              // C 4242
	/* If a mask is set, put the pointers before the end of the
	   row.  */                                                            // C 4243-4244
	{
          /* C 4245's `overflow |= ckd_mul (&end, dst_x + width - 1, pixel);`.  */
          let (ckd_end, ckd_over)
	    = ((dst_x + width - 1) as uintptr_t).overflowing_mul
	      (pixel as uintptr_t);
          end = ckd_end;
          overflow |= ckd_over;
	}
      else                                                              // C 4246
	{ end = (dst_x as size_t * pixel) as uintptr_t; }                  // C 4247

      /* C 4249-4250's two `ckd_add` calls.  */
      let (ckd_start, ckd_over) = start.overflowing_add (end);
      start = ckd_start;
      overflow |= ckd_over;
      let (ckd_start, ckd_over)
	= (dst as uintptr_t).overflowing_add (start);
      start = ckd_start;
      overflow |= ckd_over;

      if overflow                                                       // C 4252
	{ return; }                                                            // C 4253

      dst_current = start as *mut c_uchar;                              // C 4255
    }                                                                   // C 4256

  if mask.is_null ()                                                    // C 4258
    {
      /* Change the direction of the copy depending on how SRC and DST  // C 4260
	 overlap.  */                                                          // C 4261

      for i in 0..height                                             // C 4263
	{
	  memmove (dst_current as *mut c_void, src_current as *const c_void,   // C 4265
		   width as usize * pixel);                                           // C 4266

	  if backwards                                                         // C 4268
	    {
	      /* Proceed to the last row.  */                                  // C 4270
	      src_current
		= src_current.wrapping_offset (-((*src_info).stride as isize));   // C 4271
	      dst_current
		= dst_current.wrapping_offset (-((*dst_info).stride as isize));   // C 4272
	    }
	  else                                                                 // C 4274
	    {
	      /* Proceed to the next row.  */                                  // C 4276
	      src_current
		= src_current.wrapping_offset ((*src_info).stride as isize);      // C 4277
	      dst_current
		= dst_current.wrapping_offset ((*dst_info).stride as isize);      // C 4278
	    }
	}
    }                                                                     // C 4281
  else                                                                  // C 4282
    {
      /* Adjust the source and destination Y.  The start is MAX         // C 4284
         (dst_y, gc->clip_y_origin); the difference between that value  // C 4285
         and dst_y is the offset to apply to src_y. */                  // C 4286

      temp    = dst_y;                                                  // C 4288
      dst_y   = MAX! (dst_y, (*gc).clip_y_origin);                      // C 4289
      src_y  += dst_y - temp;                                           // C 4290
      height -= dst_y - temp;                                           // C 4291

      /* Verify that the bounds are correct.  */                        // C 4293
      eassert! (dst_y + height                                          // C 4294
		<= (*gc).clip_y_origin                                                // C 4295
		   + (*mask_info).height as c_int);
      eassert! (dst_y >= (*gc).clip_y_origin);                          // C 4296

      /* There is a mask.  For each scan line... */                     // C 4298

      if backwards                                                      // C 4300
	{
	  /* Calculate the number of pixels at the end of the                  // C 4302
	     mask.  */                                                         // C 4303

	  mask_offset  = dst_x + width;                                        // C 4305
	  /* C 4306's `mask_offset -= mask_info->width + gc->clip_x_origin;` - C's arithmetic is
	     in `unsigned int`, so the port keeps it there and converts back.  */
	  mask_offset -= ((*mask_info).width
			  + (*gc).clip_x_origin as c_uint) as c_int;

	  if mask_offset < 0                                                   // C 4308
	    { mask_offset = 0; }                                               // C 4309

	  /* Calculate the last column of the mask that will be                // C 4311
	     consulted.  */                                                    // C 4312

	  temp = dst_x - (*gc).clip_x_origin;                                  // C 4314
	  /* C 4315-4316's `temp += MIN (mask_info->width - temp, width - mask_offset);` (the two
	     subtractions C does in `unsigned int`).  */
	  temp += MIN! ((*mask_info).width.wrapping_sub (temp as c_uint),
			(width - mask_offset) as c_uint) as c_int;

	  if temp < 0                                                          // C 4318
	    { return; }                                                        // C 4319

	  /* Now calculate the last row of the mask that will be               // C 4321
	     consulted.  */                                                    // C 4322
	  i = dst_y - (*gc).clip_y_origin + height;                            // C 4323

	  /* Turn both into offsets.  */                                       // C 4325

	  /* C 4327-4331's four-operand `||` chain, written as C's short-circuit leaves it (the
	     `image.rs` 9622-9628 device): each call runs only when the previous ones reported no
	     overflow, and the first overflow returns.  */
	  let (ckd_offset, ckd_over)
	    = (temp as size_t).overflowing_mul (pixel);                   // C 4327
	  offset = ckd_offset;
	  if ckd_over
	    { return; }
	  let (ckd_offset1, ckd_over)
	    = (i as size_t).overflowing_mul ((*mask_info).stride as size_t);   // C 4328
	  offset1 = ckd_offset1;
	  if ckd_over
	    { return; }
	  let (ckd_offset, ckd_over) = offset.overflowing_add (offset1);   // C 4329
	  offset = ckd_offset;
	  if ckd_over
	    { return; }
	  let (ckd_start, ckd_over)
	    = (mask as uintptr_t).overflowing_add (offset);               // C 4330
	  start = ckd_start;
	  if ckd_over
	    { return; }

	  if height <= 0                                                       // C 4333
	    { return; }                                                        // C 4334

	  mask_current = start as *mut c_uchar;                                // C 4336
	  mask = mask_current;

	  while { let old = height; height -= 1; old != 0 }              // C 4338 (C's `while (height--)`)
	    {
	      /* Skip backwards past the end of the mask.  */                  // C 4340

	      long_src
		= src_current.wrapping_offset (-((mask_offset as usize)
					       .wrapping_mul (pixel) as isize)) as *mut c_uint;   // C 4342
	      long_dst
		= dst_current.wrapping_offset (-((mask_offset as usize)
					       .wrapping_mul (pixel) as isize)) as *mut c_uint;   // C 4343
	      mask = mask_current;                                       // C 4344

	      /* For each pixel covered by the mask... */                      // C 4346
	      temp = MIN! ((*mask_info).width.wrapping_sub (temp as c_uint),
			   (width - mask_offset) as c_uint) as c_int;      // C 4347
	      while { let old = temp; temp -= 1; old != 0 }              // C 4348 (C's `while (temp--)`)
		{
		  /* Copy the destination it to the source, masked by                 // C 4350
		     the mask.  */                                                    // C 4351

		  /* Sign extend the mask.  */                                        // C 4353
		  i = *(mask as *mut core::ffi::c_schar) as c_int;       // C 4354 (C's `mask--` is the step below)
		  mask = mask.wrapping_offset (-1);

		  /* Apply the mask.  */                                              // C 4356
		  *long_dst = (*long_src & i as c_uint)
		    | (*long_dst & ! (i as c_uint));                    // C 4357

		  long_dst = long_dst.wrapping_offset (-1);                  // C 4359
		  long_src = long_src.wrapping_offset (-1);                  // C 4360
		}

	      /* Return to the last row.  */                                   // C 4363
	      src_current
		= src_current.wrapping_offset (-((*src_info).stride as isize));   // C 4364
	      dst_current
		= dst_current.wrapping_offset (-((*dst_info).stride as isize));   // C 4365
	      mask_current
		= mask_current.wrapping_offset (-((*mask_info).stride as isize));   // C 4366
	    }
	}
      else                                                              // C 4369
	{
	  /* Calculate the first column of the mask that will be               // C 4371
	     consulted.  */                                                    // C 4372

	  mask_offset = dst_x - (*gc).clip_x_origin;                           // C 4374

	  /* Adjust the mask by that much.  */                                 // C 4376

	  if mask_offset > 0                                                   // C 4378
	    { mask = mask.wrapping_offset (mask_offset as isize); }            // C 4379
	  else
	    {
	      /* Offset src and dst by the mask offset.  */                    // C 4382
	      src_current
		= src_current.wrapping_offset (((-mask_offset) as usize)
					      .wrapping_mul (pixel) as isize);      // C 4383
	      dst_current
		= dst_current.wrapping_offset (((-mask_offset) as usize)
					      .wrapping_mul (pixel) as isize);      // C 4384
	      width += mask_offset;                                            // C 4385
	    }

	  /* Make sure it's not out of bounds.  */                             // C 4388

	  eassert! (dst_y - (*gc).clip_y_origin >= 0);                         // C 4390
	  /* C 4391-4392's `(dst_y - gc->clip_y_origin) + height > mask_info->height`: the
	     comparison is in `unsigned int` on C's side.  */
	  if ((dst_y - (*gc).clip_y_origin + height) as c_uint
		> (*mask_info).height
	      || width <= 0)                                            // C 4392
	    { return; }

	  /* Now move mask to the position of the first row.  */               // C 4395

	  /* C 4397-4398's `mask += ((dst_y - gc->clip_y_origin) * mask_info->stride);` (the product
	     in `unsigned int`, as C's conversion gives it).  */
	  mask = mask.wrapping_offset (((dst_y - (*gc).clip_y_origin) as c_uint)
				       .wrapping_mul ((*mask_info).stride) as isize);

	  /* Determine how many bytes need to be copied.  */                   // C 4400

	  if mask_offset > 0                                                   // C 4402
	    { temp = MIN! ((*mask_info).width.wrapping_sub (mask_offset as c_uint),
			   width as c_uint) as c_int; }                 // C 4403
	  else                                                                 // C 4404
	    { temp = MIN! ((*mask_info).width, width as c_uint) as c_int; }   // C 4405

	  if temp <= 0 || height <= 0                                          // C 4407
	    { return; }                                                        // C 4408

	  /* Copy bytes according to the mask.  */                             // C 4410

	  while { let old = height; height -= 1; old != 0 }              // C 4412 (C's `while (height--)`)
	    {
	      long_src = src_current as *mut c_uint;                           // C 4414
	      long_dst = dst_current as *mut c_uint;                           // C 4415
	      mask_current = mask;                                             // C 4416

	      /* C 4418's `#ifndef __aarch64__` arm and C 4429-4431's `#else` arm.  */
	      #[cfg (not (feature = "aarch64"))]
	      for j in 0..temp                                          // C 4419
		{
		  /* Sign extend the mask.  */                                        // C 4421
		  i = *(mask_current as *mut core::ffi::c_schar) as c_int;   // C 4422 (C's `mask_current++` is the step below)
		  mask_current = mask_current.wrapping_offset (1);

		  /* Apply the mask.  */                                              // C 4424
		  *long_dst = (*long_src & i as c_uint)
		    | (*long_dst & ! (i as c_uint));                    // C 4425
		  long_dst = long_dst.wrapping_offset (1);                   // C 4426
		  long_src = long_src.wrapping_offset (1);                   // C 4427
		}
	      #[cfg (feature = "aarch64")]
	      android_neon_mask_line (long_src, long_dst, mask, temp);   // C 4430 	      android_neon_mask_line (long_src, long_dst, mask, temp);   // C 4430

	      src_current
		= src_current.wrapping_offset ((*src_info).stride as isize);   // C 4433
	      dst_current
		= dst_current.wrapping_offset ((*dst_info).stride as isize);   // C 4434
	      mask
		= mask.wrapping_offset ((*mask_info).stride as isize);         // C 4435
	    }
	}
    }
}
/* `dispextern.h`'s `gui_intersect_rectangles` and the record it takes, which C 4564-4570 call:
   the prototype is dispextern.h 3676 under C 3660-3674's `#ifdef HAVE_WINDOW_SYSTEM` (an Android
   configuration defines it - `configure.ac` 7518) and the record is C 44-75's
   `#ifndef HAVE_WINDOW_SYSTEM` text, which `include/dispextern.rs` 4043-4049 mirrors - but that
   mirror's own neighbours need `frame`/`window`/`font`, so pulling it into this unit drags those
   in as well (measured: `E0425` on `android_display_info`, `frame`, `window`, `font`).  The port
   therefore declares the two names here (the P-88 device the NDK headers, `<signal.h>` and
   `strdup` use above; the mirror round that gives this unit the `dispextern.rs` edge removes
   them).  The record's layout is asserted against dispextern.h's.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Emacs_Rectangle
{
  pub x: c_int,
  pub y: c_int,
  pub width: c_uint,
  pub height: c_uint,
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::size_of::<Emacs_Rectangle> () == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (Emacs_Rectangle, x) == 0
                             && std::mem::offset_of! (Emacs_Rectangle, y) == 4
                             && std::mem::offset_of! (Emacs_Rectangle, width) == 8
                             && std::mem::offset_of! (Emacs_Rectangle, height) == 12);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /* bool gui_intersect_rectangles (const Emacs_Rectangle *, const Emacs_Rectangle *,
                                   Emacs_Rectangle *);  */
  pub fn gui_intersect_rectangles (r1: *const Emacs_Rectangle, r2: *const Emacs_Rectangle,
                                   out: *mut Emacs_Rectangle) -> bool;
}

/* C 4440-4608: `android_copy_area` (non-`static`; the toolkit's `copy_area` hook).  C's three
   labels `fail` (C 4605), `fail1` (C 4599) and `fail2` (C 4593) with their fall-through become
   three nested labelled blocks (PORTING.md 2.2, the `blitRect` shape of slice 11): `goto fail` is
   `break 'fail`, `goto fail1` is `break 'fail1`, `goto fail2` is `break 'fail2`, and each label's
   tail runs after its block closes - so C 4594-4598 runs before C 4600-4604, which runs before
   C 4606-4607.  C 4538's `switch (gc->function)` becomes the `match` below (the ledger's notes
   row) and C 4544-4547's commented-out case is carried as the comment it is.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_copy_area (src: android_drawable, dest: android_drawable,
						   gc: *mut android_gc, src_x: c_int, src_y: c_int,
						   mut width: c_uint, mut height: c_uint,
						   dest_x: c_int, dest_y: c_int)   // C 4440-4608
{
  let mut src_object: jobject;                                          // C 4448
  let mut dest_object: jobject;                                           let mut dest_object: jobject;
  let mut mask: jobject;                                                  let mut mask: jobject;
  let mut do_blit: android_blit_func;                                   // C 4449
  let mut src_info: android_bitmap::AndroidBitmapInfo;                  // C 4450
  let mut dest_info: android_bitmap::AndroidBitmapInfo;                   let mut dest_info: android_bitmap::AndroidBitmapInfo;
  let mut mask_info: android_bitmap::AndroidBitmapInfo;                   let mut mask_info: android_bitmap::AndroidBitmapInfo;
  let mut src_data: *mut c_void;                                        // C 4451
  let mut dest_data: *mut c_void;                                         let mut dest_data: *mut c_void;
  let mut mask_data: *mut c_void;                                         let mut mask_data: *mut c_void;
  let mut n_clip_rects: c_int;                                          // C 4452
  let mut flag: bool;                                                   // C 4453
  let mut bounds: android_rectangle;                                    // C 4454
  let mut rect: android_rectangle;                                        let mut rect: android_rectangle;
  let mut temp: android_rectangle;                                        let mut temp: android_rectangle;
  let mut clip_rectangles: *mut android_rectangle;                        let mut clip_rectangles: *mut android_rectangle;

  /* C leaves the locals uninitialised (PORTING.md 1.5); C 4452's `int i` is the loop's.  */
  src_object = std::ptr::null_mut ();
  dest_object = std::ptr::null_mut ();
  mask = std::ptr::null_mut ();
  do_blit = None;
  src_info = std::mem::zeroed ();
  dest_info = std::mem::zeroed ();
  mask_info = std::mem::zeroed ();
  src_data = std::ptr::null_mut ();
  dest_data = std::ptr::null_mut ();
  mask_data = std::ptr::null_mut ();
  n_clip_rects = 0;
  flag = false;
  bounds = std::mem::zeroed ();
  rect = std::mem::zeroed ();
  temp = std::mem::zeroed ();
  clip_rectangles = std::ptr::null_mut ();

  /* Perform the copy.  Loop over each clip rectangle, unless none are  // C 4456
     set.  Also, obtain bitmaps for src and dst, and possibly the mask  // C 4457
     as well if it is present.  */                                      // C 4458

  src_data = android_lock_bitmap (src,
				 (&raw mut src_info) as *mut AndroidBitmapInfo_tag,
				 &raw mut src_object) as *mut c_void;                // C 4460
  if src_data.is_null ()                                                // C 4461
    { return; }                                                         // C 4462

  mask = std::ptr::null_mut ();                                         // C 4464
  mask_data = mask;

  /* C 4466-4476's `if (src != dest)` with its `goto fail`.  */
  'fail: {
    if src != dest                                                     // C 4466
	{
	  dest_data = android_lock_bitmap (dest,
					    (&raw mut dest_info) as *mut AndroidBitmapInfo_tag,
					    &raw mut dest_object) as *mut c_void;   // C 4468
	  if dest_data.is_null ()                                              // C 4469
	    { break 'fail; }                                                   // C 4470 (C's `goto fail`)
	}
    else                                                               // C 4472
	{
	  dest_data = src_data;                                                // C 4474
	  dest_info = src_info;                                                // C 4475
	}

    /* Obtain the bitmap for the mask if necessary.  */                 // C 4478

    'fail1: {
      if (*gc).clip_mask != 0                                           // C 4480
	{
	  mask_data = android_lock_bitmap ((*gc).clip_mask,
					       (&raw mut mask_info) as *mut AndroidBitmapInfo_tag,
					       &raw mut mask) as *mut c_void;      // C 4482
	  if mask_data.is_null ()                                              // C 4484
	    { break 'fail1; }                                                  // C 4485 (C's `goto fail1`)
	}

      /* Calculate the number of clip rectangles.  */                   // C 4488
      n_clip_rects = (*gc).num_clip_rects;                              // C 4489

      /* If n_clip_rects is -1, then no clipping is in effect.  Set rect // C 4491
	 to the bounds of the destination.  */                                 // C 4492

      flag = n_clip_rects == -1;                                        // C 4494
      'fail2: {
	if flag                                                                // C 4495
	  {
	    n_clip_rects = 1;                                                  // C 4497
	    clip_rectangles = &raw mut rect;                                   // C 4498
	  }
	else if n_clip_rects == 0                                              // C 4500 (C's `!n_clip_rects`)
	  { break 'fail2; }                                                    // C 4501 (C's `goto fail2`)
	else                                                                   // C 4502
	  { clip_rectangles = (*gc).clip_rects; }                              // C 4503

	/* Set rect to the bounds of the destination.  */                      // C 4505

	rect.x = 0;                                                            // C 4507
	rect.y = 0;                                                            // C 4508
	rect.width = dest_info.width;                                          // C 4509
	rect.height = dest_info.height;                                        // C 4510

	if ! mask_data.is_null ()                                              // C 4512
	  {
	    /* Clip width and height to that of the mask.  */                  // C 4514

	    /* C 4516-4520's two mixed-type tests and subtractions (C's are `unsigned int`).  */
	    if (src_x + width as c_int) as c_uint > mask_info.width   // C 4516
	      { width = mask_info.width.wrapping_sub (src_x as c_uint) as c_uint; }   // C 4517

	    if (src_y + height as c_int) as c_uint > mask_info.height   // C 4519
	      { height = mask_info.height.wrapping_sub (src_y as c_uint) as c_uint; }   // C 4520
	  }

	/* Clip width and height to that of the source.  */                    // C 4523

	if (src_x + width as c_int) as c_uint > src_info.width      // C 4525
	  { width = src_info.width.wrapping_sub (src_x as c_uint) as c_uint; }   // C 4526

	if (src_y + height as c_int) as c_uint > src_info.height    // C 4528
	  { height = src_info.height.wrapping_sub (src_y as c_uint) as c_uint; }   // C 4529

	/* Return if the copy is outside the source.  */                       // C 4531

	if width as c_int <= 0 || height as c_int <= 0              // C 4533
	  { break 'fail2; }

	/* Look up the right function for the alu.  */                         // C 4536

	/* C 4538's `switch (gc->function)`, its one `case`, the `break` that ends it and its
	   `default` become the `match` below (PORTING.md 2.2); C 4544-4547's two commented-out
	   lines and the four-line comment are carried as the comments they are.  */
	match (*gc).function
	  {
	    ANDROID_GC_COPY =>                                                 // C 4540
	      {
		/* case ANDROID_GC_INVERT: */
		/* do_blit = android_blit_invert; */
		/* A GC with its operation set to ANDROID_GC_INVERT is never given
		   to CopyArea.  */
		/* C 4541's `do_blit = android_blit_copy;`: the port's `android_blit_func` is an
		   `Option`, so the assignment wraps the item in `Some`.  */
		do_blit = Some (android_blit_copy);
	      }

	    _ =>                                                               // C 4548 (C's `default:`)
	      { emacs_abort (); }                                              // C 4549
	  }

	/* Load the bounds of the destination rectangle.  */                   // C 4552
	bounds.x = dest_x;                                                     // C 4553
	bounds.y = dest_y;                                                     // C 4554
	bounds.width = width as c_uint;                                        // C 4555 (C's `int`-into-`unsigned`)
	bounds.height = height as c_uint;                                      // C 4556

	/* For each clip rectangle... */                                       // C 4558
	for i in 0..n_clip_rects                                      // C 4559
	  {
	    /* Calculate its intersection with the destination                 // C 4561
	       rectangle.  */                                                  // C 4562

	    /* C 4564-4565's `gui_intersect_rectangles (&clip_rectangles[i], &bounds, &temp)`:
	       the prototype's `Emacs_Rectangle *` is a different (same-shaped) struct from
	       `android_rectangle`, and C's implicit conversion is spelled out.  */
	    if ! gui_intersect_rectangles
		 (clip_rectangles.offset (i as isize) as *const Emacs_Rectangle,
		  (&raw const bounds) as *const Emacs_Rectangle,
		  (&raw mut temp) as *mut Emacs_Rectangle)               // C 4565
	      { continue; }                                                    // C 4566

	    /* And that of the destination itself.  */                         // C 4568

	    if ! flag && ! gui_intersect_rectangles                  // C 4570
		 ((&raw const temp) as *const Emacs_Rectangle,
		  (&raw const rect) as *const Emacs_Rectangle,
		  (&raw mut temp) as *mut Emacs_Rectangle)
	      { continue; }                                                    // C 4571

	    /* Now perform the copy.  */                                       // C 4573
	    (do_blit.unwrap ()) (src_x + temp.x as c_int - dest_x,   // C 4574 (C's `temp.x relative to src_x`)
				 src_y + temp.y as c_int - dest_y,     // C 4575 (C's `temp.y relative to src_y`)
				 temp.width as c_int,                  // C 4576 (C's `Width of area to copy.`)
				 temp.height as c_int,                 // C 4577 (C's `Height of area to copy.`)
				 temp.x as c_int, temp.y as c_int,     // C 4578 (C's `Coordinates to copy to.`)
				 gc,                                   // C 4579 (C's `GC.`)
				 src_data as *mut c_uchar, &raw mut src_info,          // C 4580 (C's `Source drawable.`)
				 dest_data as *mut c_uchar, &raw mut dest_info,        // C 4581 (C's `Destination drawable.`)
				 mask_data as *mut c_uchar, &raw mut mask_info);       // C 4582 (C's `Mask drawable.`)
	  }

	/* Now damage the destination drawable accordingly, should it be a     // C 4585
	   window.  */                                                         // C 4586

	if ((*(*android_java_env)).IsInstanceOf.unwrap ())           // C 4588
	     (android_java_env, dest as jobject, window_class.class) != 0
	  { android_damage_window (dest, &raw mut bounds); }                   // C 4591
      }

      /* fail2: */                                                      // C 4593
      if ! mask.is_null ()                                              // C 4594
	{
	  AndroidBitmap_unlockPixels (android_java_env, mask);                 // C 4596
	  ANDROID_DELETE_LOCAL_REF! (mask);                                    // C 4597
	}
    }

    /* fail1: */                                                        // C 4599
    if src != dest                                                      // C 4600
      {
	AndroidBitmap_unlockPixels (android_java_env, dest_object);            // C 4602
	ANDROID_DELETE_LOCAL_REF! (dest_object);                               // C 4603
      }
  }

  /* fail: */                                                           // C 4605
  AndroidBitmap_unlockPixels (android_java_env, src_object);            // C 4606
  ANDROID_DELETE_LOCAL_REF! (src_object);                               // C 4607
}

/* C 4610-4616: C 4610's form feed, the blank, C 4612's `void` and `android_free_pixmap`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_free_pixmap (pixmap: android_pixmap)   // C 4610-4616
{
  android_destroy_handle (pixmap);                                      // C 4615
}

/* C 4618-4625: `android_set_background`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_background (gc: *mut android_gc,   // C 4618
						 background: c_ulong)                                             // C 4618-4625
{
  let mut gcv: android_gc_values;                                       // C 4621

  gcv = std::mem::zeroed ();                                    // (C leaves `gcv` uninitialised; PORTING.md 1.5)   gcv = std::mem::zeroed ();                                    // (C leaves `gcv` uninitialised; PORTING.md 1.5)

  gcv.background = background;                                          // C 4623
  android_change_gc (gc, ANDROID_GC_BACKGROUND, &raw mut gcv);          // C 4624
}

/* C 4626-4667: C 4626's blank, C 4627's `void` and `android_fill_polygon` (non-`static`; the
   toolkit's `fill_polygon` hook).  C 4645's `for (i = 0; i < npoints; ++i)` is the port's range,
   so C 4634's `int i` has no declaration of its own, and C 4650-4651's `points[i]` is
   `(*points.offset (i as isize))` (Rust's raw pointer has no `Index`; slice 17's device).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_fill_polygon (drawable: android_drawable, gc: *mut android_gc,
						      points: *mut android_point, npoints: c_int,
						      shape: android_shape, mode: android_coord_mode)
                                                                     // C 4626-4667
{
  let mut array: jobjectArray;                                          // C 4632
  let mut point: jobject;                                               // C 4633
  let mut drawable_object: jobject;
  let mut gcontext: jobject;

  /* C leaves the four locals uninitialised (PORTING.md 1.5); C 4634's `int i` is the loop's,
     and C's `shape`/`mode` are read by no statement of the body.  */
  array = std::ptr::null_mut ();
  point = std::ptr::null_mut ();
  drawable_object = std::ptr::null_mut ();
  gcontext = std::ptr::null_mut ();

  drawable_object = android_resolve_handle (drawable);                  // C 4636
  gcontext = android_resolve_handle ((*gc).gcontext);                   // C 4637

  array = ((*(*android_java_env)).NewObjectArray.unwrap ()) (android_java_env, // C 4639
						       npoints,                                                   // C 4640
						       point_class.class,                                         // C 4641
						       std::ptr::null_mut ());                                    // C 4642
  android_exception_check ();                                           // C 4643

  for i in 0..npoints                                                   // C 4645
    {
      point = ((*(*android_java_env)).NewObject.unwrap ()) (android_java_env, // C 4647
						      point_class.class,                                          // C 4648
						      point_class.constructor,                                    // C 4649
						      (*points.offset (i as isize)).x as jint,                    // C 4650
						      (*points.offset (i as isize)).y as jint);                   // C 4651
      android_exception_check_1 (array);                                // C 4652

      ((*(*android_java_env)).SetObjectArrayElement.unwrap ())          // C 4654
	(android_java_env,                                                     // C 4655
	 array, i, point);
      ANDROID_DELETE_LOCAL_REF! (point);                                // C 4656
    }

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 4659
	(android_java_env,                                                     // C 4660
	 emacs_service,                                                        // C 4661
	 service_class.class,                                                  // C 4662
	 service_class.fill_polygon,                                           // C 4663
	 drawable_object,                                                      // C 4664
	 gcontext, array);
  android_exception_check_1 (array);                                    // C 4665
  ANDROID_DELETE_LOCAL_REF! (array);                                    // C 4666
}

/* C 4669-4689: `android_draw_rectangle`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_draw_rectangle (handle: android_drawable, // C 4669-4689
						      gc: *mut android_gc,
						      x: c_int, y: c_int, width: c_uint, height: c_uint)
{
  let mut drawable: jobject;                                            // C 4673
  let mut gcontext: jobject;

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  drawable = std::ptr::null_mut ();
  gcontext = std::ptr::null_mut ();

  drawable = android_resolve_handle (handle);                           // C 4675
  gcontext = android_resolve_handle ((*gc).gcontext);                   // C 4676

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 4678
	(android_java_env,                                                     // C 4679
	 emacs_service,                                                        // C 4680
	 service_class.class,                                                  // C 4681
	 service_class.draw_rectangle,                                         // C 4682
	 drawable, gcontext,
	 x as jint, y as jint,                                                 // C 4683
	 width as jint, height as jint);                                       // C 4684

  /* In lieu of android_exception_check, clear all exceptions after     // C 4686
     calling this frequently called graphics operation.  */             // C 4687
  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env); // C 4688
}

/* C 4691-4710: `android_draw_point`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_draw_point (handle: android_drawable,  // C 4691-4710
						  gc: *mut android_gc,
						  x: c_int, y: c_int)
{
  let mut drawable: jobject;                                            // C 4695
  let mut gcontext: jobject;

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  drawable = std::ptr::null_mut ();
  gcontext = std::ptr::null_mut ();

  drawable = android_resolve_handle (handle);                           // C 4697
  gcontext = android_resolve_handle ((*gc).gcontext);                   // C 4698

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 4700
	(android_java_env,                                                     // C 4701
	 emacs_service,                                                        // C 4702
	 service_class.class,                                                  // C 4703
	 service_class.draw_point,                                             // C 4704
	 drawable, gcontext,
	 x as jint, y as jint);                                                // C 4705

  /* In lieu of android_exception_check, clear all exceptions after     // C 4707
     calling this frequently called graphics operation.  */             // C 4708
  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env); // C 4709
}

/* C 4712-4732: `android_draw_line`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_draw_line (handle: android_drawable,   // C 4712-4732
						 gc: *mut android_gc,
						 x: c_int, y: c_int, x2: c_int, y2: c_int)
{
  let mut drawable: jobject;                                            // C 4716
  let mut gcontext: jobject;

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  drawable = std::ptr::null_mut ();
  gcontext = std::ptr::null_mut ();

  drawable = android_resolve_handle (handle);                           // C 4718
  gcontext = android_resolve_handle ((*gc).gcontext);                   // C 4719

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 4721
	(android_java_env,                                                     // C 4722
	 emacs_service,                                                        // C 4723
	 service_class.class,                                                  // C 4724
	 service_class.draw_line,                                              // C 4725
	 drawable, gcontext,
	 x as jint, y as jint,                                                 // C 4726
	 x2 as jint, y2 as jint);                                              // C 4727

  /* In lieu of android_exception_check, clear all exceptions after     // C 4729
     calling this frequently called graphics operation.  */             // C 4730
  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env); // C 4731
}
/* C 4748-4759: C 4748's blank, C 4749's `void` and `android_set_ts_origin`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_ts_origin (gc: *mut android_gc, x: c_int, y: c_int) // C 4748-4759
{
  let mut gcv: android_gc_values;                                       // C 4752

  gcv = std::mem::zeroed ();                                    // (C leaves `gcv` uninitialised; PORTING.md 1.5)

  gcv.ts_x_origin = x;                                                  // C 4754
  gcv.ts_y_origin = y;                                                  // C 4755
  android_change_gc (gc, (ANDROID_GC_TILE_STIP_X_ORIGIN                 // C 4756
			  | ANDROID_GC_TILE_STIP_Y_ORIGIN),                                  // C 4757
		     &raw mut gcv);                                                   // C 4758
}

/* C 4761-4775: C 4761's blank, C 4762's `void` and `android_clear_area`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_clear_area (handle: android_window, x: c_int, y: c_int, // C 4761-4775
					      width: c_uint, height: c_uint)
{
  let mut window: jobject;                                              // C 4765

  window = std::ptr::null_mut ();                               // (C leaves `window` uninitialised; PORTING.md 1.5)

  window = android_resolve_handle (handle);                             // C 4767

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 4769
	(android_java_env,                                                     // C 4770
	 window,                                                               // C 4771
	 window_class.class,                                                   // C 4772
	 window_class.clear_area,                                              // C 4773
	 x as jint, y as jint,
	 width as jint, height as jint);                                       // C 4774
}

/* C 4777-4783: C 4777's blank, C 4778's `android_pixmap` and `android_create_bitmap_from_data`,
   whose C 4781-4782 call is this unit's other `android_create_pixmap_from_bitmap_data` site
   (`1`, `0`, `1` are C's foreground, background and depth arguments).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_create_bitmap_from_data (bits: *mut c_char, width: c_uint, // C 4777-4783
							      height: c_uint)
  -> android_pixmap
{
  return android_create_pixmap_from_bitmap_data (bits, width, height,   // C 4781
						  1, 0, 1);                      // C 4782
}

/* C 4785-4819: C 4785's `struct android_image *` and `android_create_image`.  C 4796-4800 and
   C 4807/4812 write `unsigned int` values into `int` fields, so the port spells those
   conversions out (C's own implicit ones).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_create_image (depth: c_uint,           // C 4785-4819
						 format: android_image_format,
						 data: *mut c_char, width: c_uint, height: c_uint)
  -> *mut android_image
{
  let mut image: *mut android_image;                                    // C 4789

  image = std::ptr::null_mut ();                                // (C leaves `image` uninitialised; PORTING.md 1.5)

  image = xmalloc (std::mem::size_of::<android_image> ()) as *mut android_image; // C 4791

  /* Fill in the fields required by image.c.  N.B. that                 // C 4793
     android_destroy_image ostensibly will free data, but image.c       // C 4794
     mostly sets and frees data itself.  */                             // C 4795
  (*image).width = width as c_int;                                      // C 4796 (C's `unsigned`-into-`int`)
  (*image).height = height as c_int;                                    // C 4797
  (*image).data = data;                                                 // C 4798
  (*image).depth = depth as c_int;                                      // C 4799
  (*image).format = format;                                             // C 4800

  /* Now fill in the image dimensions.  There are only two depths       // C 4802
     supported by this function.  */                                    // C 4803

  if depth == 1                                                         // C 4805
    {
      (*image).bytes_per_line = ((width + 7) / 8) as c_int;             // C 4807
      (*image).bits_per_pixel = 1;                                      // C 4808
    }
  else if depth == 24                                                   // C 4810
    {
      (*image).bytes_per_line = (width * 4) as c_int;                   // C 4812
      (*image).bits_per_pixel = 32;                                     // C 4813
    }
  else                                                                  // C 4815
    { emacs_abort (); }                                                 // C 4816

  return image;                                                         // C 4818
}

/* C 4821-4830: C 4821's blank, C 4822's `void` and `android_destroy_image`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_destroy_image (ximg: *mut android_image) // C 4821-4830
{
  /* If XIMG->data is NULL, then it has already been freed by           // C 4824
     image.c.  */                                                       // C 4825

  if ! (*ximg).data.is_null ()                                          // C 4827
    { xfree ((*ximg).data as *mut c_void); }                            // C 4828
  xfree (ximg as *mut c_void);                                          // C 4829
}

/* C 4832-4873: `android_put_pixel`.  C 4845's `switch (ximg->depth)`, its two `case`s and the
   two `break`s become the `match` below (the ledger's notes row), and C 4848/4858's
   `ximg->data + y * …` pointer arithmetic is `.wrapping_offset (…)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_put_pixel (ximg: *mut android_image, x: c_int, y: c_int, // C 4832-4873
					  mut pixel: c_ulong)
{
  let mut byte: *mut c_char;                                            // C 4836
  let mut word: *mut c_char;
  let mut r: c_uint;                                                    // C 4837
  let mut g: c_uint;
  let mut b: c_uint;
  let mut pixel_int: c_uint;                                            // C 4838

  /* C leaves the six locals uninitialised (PORTING.md 1.5).  */
  byte = std::ptr::null_mut ();
  word = std::ptr::null_mut ();
  r = 0;
  g = 0;
  b = 0;
  pixel_int = 0;

  /* Ignore out-of-bounds accesses.  */                                 // C 4840

  if x >= (*ximg).width || y >= (*ximg).height                          // C 4842
	|| x < 0 || y < 0
    { return; }                                                         // C 4843

  match (*ximg).depth                                                   // C 4845
    {
      1 =>                                                              // C 4847
	{
	  byte = (*ximg).data
	    .wrapping_offset ((y * (*ximg).bytes_per_line + x / 8) as isize);   // C 4848

	  if pixel != 0                                                        // C 4850
	    { *byte |= (1 << (x % 8)) as c_char; }                             // C 4851
	  else
	    { *byte &= ! ((1 << (x % 8)) as c_char); }                         // C 4853
	}

      24 =>                                                             // C 4856
	{
	  /* Unaligned accesses are problematic on Android devices.  */        // C 4857
	  word = (*ximg).data
	    .wrapping_offset ((y * (*ximg).bytes_per_line + x * 4) as isize);   // C 4858

	  /* Swizzle the pixel into ABGR format.  Android uses Skia's          // C 4860
	     ``native color type'', which is ABGR.  This is despite the        // C 4861
	     format being named ``ARGB'', and more confusingly                 // C 4862
	     `ANDROID_BITMAP_FORMAT_RGBA_8888' in bitmap.h.  */                // C 4863
	  r = (pixel & 0x00ff0000) as c_uint;                                  // C 4864 (C's `unsigned long`-into-`unsigned int`)
	  g = (pixel & 0x0000ff00) as c_uint;                                  // C 4865
	  b = (pixel & 0x000000ff) as c_uint;                                  // C 4866
	  pixel = ((r >> 16) | g | (b << 16) | 0xff000000) as c_ulong;         // C 4867 (C's widening back)

	  pixel_int = pixel as c_uint;                                         // C 4869
	  memcpy (word as *mut c_void, (&raw const pixel_int) as *const c_void, // C 4870
		  std::mem::size_of::<c_uint> ());
	}
      /* C's `switch` has no `default`: an unmatched depth falls out of it to the function's
	 end, which is this arm.  */
      _ => {}
    }
}

/* C 4875-4905: `android_get_pixel`.  Its `switch` is the same device as C 4845's, and its last
   statement (`emacs_abort ()`) is C's code after the `switch`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_get_pixel (ximg: *mut android_image, x: c_int, y: c_int) // C 4875-4905
  -> c_ulong
{
  let mut byte: *mut c_char;                                            // C 4878
  let mut word: *mut c_char;
  let mut pixel: c_uint;                                                // C 4879
  let mut r: c_uint;
  let mut g: c_uint;
  let mut b: c_uint;

  /* C leaves the six locals uninitialised (PORTING.md 1.5).  */
  byte = std::ptr::null_mut ();
  word = std::ptr::null_mut ();
  pixel = 0;
  r = 0;
  g = 0;
  b = 0;

  if x >= (*ximg).width || y >= (*ximg).height                          // C 4881
	|| x < 0 || y < 0
    { return 0; }                                                       // C 4883

  match (*ximg).depth                                                   // C 4885
    {
      1 =>                                                              // C 4887
	{
	  byte = (*ximg).data
	    .wrapping_offset ((y * (*ximg).bytes_per_line + x / 8) as isize);   // C 4888
	  return if (*byte as c_int & (1 << (x % 8))) != 0 { 1 } else { 0 };   // C 4889
	}

      24 =>                                                             // C 4891
	{
	  word = (*ximg).data
	    .wrapping_offset ((y * (*ximg).bytes_per_line + x * 4) as isize);   // C 4892
	  memcpy ((&raw mut pixel) as *mut c_void, word as *const c_void,      // C 4893
		  std::mem::size_of::<c_uint> ());

	  /* Convert the pixel back to RGB.  */                                // C 4895
	  b = pixel & 0x00ff0000;                                              // C 4896
	  g = pixel & 0x0000ff00;                                              // C 4897
	  r = pixel & 0x000000ff;                                              // C 4898
	  pixel = ((r << 16) | g | (b >> 16)) & ! 0xff000000;                  // C 4899

	  return pixel as c_ulong;                                             // C 4901
	}
      /* C's `switch` has no `default`: an unmatched depth falls out of it to C 4904's
	 `emacs_abort ()`, which is this arm plus the statement after the match.  */
      _ => {}
    }

  emacs_abort ();                                                       // C 4904
}
/* C 4906-5040: C 4906's blank, C 4907's `struct android_image *` and `android_get_image`.
   C's `memory_full` is declared in-unit (the `androidfont.rs` 105 device: lisp.h 4432 declares
   it, alloc.c defines it).  C 4984's `switch (bitmap_info.format)` becomes the `match` below
   (the ledger's notes row).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: lisp.h 4432: `extern AVOID memory_full (size_t);` (alloc.c defines it).
  pub fn memory_full (size: size_t) -> !;
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_get_image (handle: android_drawable,   // C 4906-5040
						 format: android_image_format)
  -> *mut android_image
{
  let mut drawable: jobject;                                            // C 4911
  let mut bitmap: jobject;
  let mut bitmap_info: android_bitmap::AndroidBitmapInfo;               // C 4912
  let mut byte_size: size_t;                                            // C 4913
  let mut data: *mut c_void;                                            // C 4914
  let mut image: *mut android_image;                                    // C 4915
  let mut data1: *mut c_uchar;                                          // C 4916
  let mut data2: *mut c_uchar;

  /* C leaves the eight locals uninitialised (PORTING.md 1.5); C 4917's `int i, x` are the
     loops'.  */
  drawable = std::ptr::null_mut ();
  bitmap = std::ptr::null_mut ();
  bitmap_info = std::mem::zeroed ();
  byte_size = 0;
  data = std::ptr::null_mut ();
  image = std::ptr::null_mut ();
  data1 = std::ptr::null_mut ();
  data2 = std::ptr::null_mut ();

  drawable = android_resolve_handle (handle);                           // C 4919

  /* Look up the drawable and get the bitmap corresponding to it.       // C 4921
     Then, lock the bitmap's bits.  */                                  // C 4922
  bitmap = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env, // C 4923
							  drawable,                                                      // C 4924
							  drawable_class.get_bitmap);                                    // C 4925
  android_exception_check ();                                           // C 4926

  /* Clear the bitmap info structure.  */                               // C 4928
  memset ((&raw mut bitmap_info) as *mut c_void, 0,                     // C 4929
	  std::mem::size_of::<android_bitmap::AndroidBitmapInfo> ());

  /* The NDK doc seems to imply this function can fail but doesn't say  // C 4931
     what value it gives when it does! */                               // C 4932
  AndroidBitmap_getInfo (android_java_env, bitmap,                      // C 4933
			 &raw mut bitmap_info);

  if bitmap_info.stride == 0                                            // C 4935
    {
      ANDROID_DELETE_LOCAL_REF! (bitmap);                               // C 4937
      memory_full (0);                                                  // C 4938
    }

  /* Compute how big the image data will be.  Fail if it would be too   // C 4941
     big.  */                                                           // C 4942

  if bitmap_info.format != ANDROID_BITMAP_FORMAT_A_8                    // C 4944
    {
      /* C 4946's `ckd_mul (&byte_size, bitmap_info.stride, bitmap_info.height)` (P-15: the
         wrapped product and the flag).  */
      let (ckd_byte_size, ckd_over)
	= (bitmap_info.stride as size_t).overflowing_mul (bitmap_info.height as size_t);
      byte_size = ckd_byte_size;
      if ckd_over
	{
	  ANDROID_DELETE_LOCAL_REF! (bitmap);                                  // C 4948
	  memory_full_up ();                                                   // C 4949
	}
    }
  else                                                                  // C 4952
    /* This A8 image will be packed into A1 later on.  */               // C 4953
	/* C 4954's `byte_size = (bitmap_info.width + 7) / 8;` (the `unsigned int` arithmetic
	   widened to `size_t`).  */
	{ byte_size = (((bitmap_info.width + 7) / 8) as size_t); }

  /* Lock the image data.  Once again, the NDK documentation says the   // C 4956
     call can fail, but does not say how to determine whether or not    // C 4957
     it has failed, nor how the address is aligned.  */                 // C 4958
  data = std::ptr::null_mut ();                                         // C 4959
  AndroidBitmap_lockPixels (android_java_env, bitmap, &raw mut data);   // C 4960

  if data.is_null ()                                                    // C 4962
    {
      /* Take a NULL pointer to mean that AndroidBitmap_lockPixels      // C 4964
	 failed.  */                                                           // C 4965
      ANDROID_DELETE_LOCAL_REF! (bitmap);                               // C 4966
      memory_full (0);                                                  // C 4967
    }

  /* Copy the data into a new struct android_image.  */                 // C 4970
  image = xmalloc (std::mem::size_of::<android_image> ()) as *mut android_image; // C 4971
  (*image).width = bitmap_info.width as c_int;                          // C 4972 (C's `unsigned`-into-`int`)
  (*image).height = bitmap_info.height as c_int;                        // C 4973
  (*image).data = malloc (byte_size) as *mut c_char;                    // C 4974

  if (*image).data.is_null ()                                           // C 4976
    {
      ANDROID_DELETE_LOCAL_REF! (bitmap);                               // C 4978
      xfree (image as *mut c_void);                                     // C 4979
      memory_full (byte_size);                                          // C 4980
    }

  /* Use the format of the bitmap to determine the image depth.  */     // C 4983
  match bitmap_info.format                                              // C 4984
    {
      ANDROID_BITMAP_FORMAT_RGBA_8888 =>                                // C 4986
	{
	  (*image).depth = 24;                                                 // C 4987
	  (*image).bits_per_pixel = 32;                                        // C 4988
	}

      /* A8 images are used by Emacs to represent bitmaps.  They have   // C 4991
	 to be packed manually.  */                                            // C 4992
      ANDROID_BITMAP_FORMAT_A_8 =>                                      // C 4993
	{
	  (*image).depth = 1;                                                  // C 4994
	  (*image).bits_per_pixel = 1;                                         // C 4995
	}

      /* Other formats are currently not supported.  */                 // C 4998
      _ =>                                                              // C 4999 (C's `default:`)
	{ emacs_abort (); }                                                    // C 5000
    }

  (*image).format = format;                                             // C 5003

  if (*image).depth == 24                                               // C 5005
    {
      (*image).bytes_per_line = bitmap_info.stride as c_int;            // C 5007

      /* Copy the bitmap data over.  */                                 // C 5009
      memcpy ((*image).data as *mut c_void, data, byte_size);           // C 5010
    }
  else                                                                  // C 5012
    {
      /* Pack the A8 image data into bits manually.  */                 // C 5014
      (*image).bytes_per_line = (((*image).width + 7) / 8) as c_int;    // C 5015

      data1 = (*image).data as *mut c_uchar;                            // C 5017
      data2 = data as *mut c_uchar;                                     // C 5018

      for i in 0..(*image).height                                       // C 5020
	{
	  for x in 0..(*image).width                                       // C 5022
	    {
	      /* Some bits in data1 might be initialized at this point,
		 but they will all be set properly later.  */
	      *data1.offset ((x / 8) as isize) = (if *data2.offset (x as isize) != 0
					     { *(data1.offset ((x / 8) as isize))
					       | ((1 << (x % 8)) as c_uchar) }
					     else
					     { *(data1.offset ((x / 8) as isize))
					       & ! ((1 << (x % 8)) as c_uchar) });   // C 5026-5027
	    }

	  data1 = data1.offset ((*image).bytes_per_line as isize);             // C 5029
	  data2 = data2.offset (bitmap_info.stride as isize);                  // C 5030
	}
    }

  /* Unlock the bitmap pixels.  */                                      // C 5034
  AndroidBitmap_unlockPixels (android_java_env, bitmap);                // C 5035

  /* Delete the bitmap reference.  */                                   // C 5037
  ANDROID_DELETE_LOCAL_REF! (bitmap);                                   // C 5038
  return image;                                                         // C 5039
}

/* C 5042-5128: C 5042's blank, C 5043's `void` and `android_put_image`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_put_image (handle: android_pixmap,     // C 5042-5128
						 image: *mut android_image)
{
  let mut drawable: jobject;                                            // C 5045
  let mut bitmap: jobject;
  let mut bitmap_info: android_bitmap::AndroidBitmapInfo;               // C 5046
  let mut data: *mut c_void;                                            // C 5047
  let mut data_1: *mut c_uchar;                                         // C 5048
  let mut data_2: *mut c_uchar;

  /* C leaves the six locals uninitialised (PORTING.md 1.5); C 5049's `int i, x` are the
     loops'.  */
  drawable = std::ptr::null_mut ();
  bitmap = std::ptr::null_mut ();
  bitmap_info = std::mem::zeroed ();
  data = std::ptr::null_mut ();
  data_1 = std::ptr::null_mut ();
  data_2 = std::ptr::null_mut ();

  drawable = android_resolve_handle (handle);                           // C 5051

  /* Look up the drawable and get the bitmap corresponding to it.       // C 5053
     Then, lock the bitmap's bits.  */                                  // C 5054
  bitmap = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env, // C 5055
							  drawable,                                                      // C 5056
							  drawable_class.get_bitmap);                                    // C 5057
  android_exception_check ();                                           // C 5058

  /* Clear the bitmap info structure.  */                               // C 5060
  memset ((&raw mut bitmap_info) as *mut c_void, 0,                     // C 5061
	  std::mem::size_of::<android_bitmap::AndroidBitmapInfo> ());

  /* The NDK doc seems to imply this function can fail but doesn't say  // C 5063
     what value it gives when it does! */                               // C 5064
  AndroidBitmap_getInfo (android_java_env, bitmap,                      // C 5065
			 &raw mut bitmap_info);

  if bitmap_info.stride == 0                                            // C 5067
    {
      ANDROID_DELETE_LOCAL_REF! (bitmap);                               // C 5069
      memory_full (0);                                                  // C 5070
    }

  if bitmap_info.width as c_int != (*image).width                       // C 5073
	|| bitmap_info.height as c_int != (*image).height
    /* This is not yet supported.  */                                   // C 5075
    { emacs_abort (); }                                                 // C 5076

  /* Make sure the bitmap formats are compatible with each other.  */   // C 5078

  if ((*image).depth == 24                                              // C 5080
	&& bitmap_info.format != ANDROID_BITMAP_FORMAT_RGBA_8888)
	|| ((*image).depth == 1
	    && bitmap_info.format != ANDROID_BITMAP_FORMAT_A_8)
    { emacs_abort (); }                                                 // C 5084

  /* Lock the image data.  Once again, the NDK documentation says the   // C 5086
     call can fail, but does not say how to determine whether or not    // C 5087
     it has failed, nor how the address is aligned.  */                 // C 5088
  data = std::ptr::null_mut ();                                         // C 5089
  AndroidBitmap_lockPixels (android_java_env, bitmap, &raw mut data);   // C 5090

  if data.is_null ()                                                    // C 5092
    {
      /* Take a NULL pointer to mean that AndroidBitmap_lockPixels      // C 5094
	 failed.  */                                                           // C 5095
      ANDROID_DELETE_LOCAL_REF! (bitmap);                               // C 5096
      memory_full (0);                                                  // C 5097
    }

  data_1 = data as *mut c_uchar;                                        // C 5100
  data_2 = (*image).data as *mut c_uchar;                               // C 5101

  /* Copy the bitmap data over scanline-by-scanline.  */                // C 5103
  for i in 0..(*image).height                                           // C 5104
    {
      if (*image).depth != 1                                            // C 5106
	{ memcpy (data_1 as *mut c_void, data_2 as *const c_void,              // C 5107
		  ((*image).width * ((*image).bits_per_pixel / 8)) as size_t); }
      else                                                              // C 5109
	{
	  /* Android internally uses a 1 byte-per-pixel format for             // C 5111
	     ALPHA_8 images.  Expand the image from the 1                      // C 5112
	     bit-per-pixel X format correctly.  */                             // C 5113

	  for x in 0..(*image).width                                  // C 5115
	    {
	      *data_1.offset (x as isize)
		= (if ((*data_2.offset ((x / 8) as isize) as c_int)
		      & (1 << (x % 8))) != 0 { 0xff } else { 0 }) as c_uchar;   // C 5116
	    }
	}

      data_1 = data_1.offset (bitmap_info.stride as isize);             // C 5119
      data_2 = data_2.offset ((*image).bytes_per_line as isize);        // C 5120
    }

  /* Unlock the bitmap pixels.  */                                      // C 5123
  AndroidBitmap_unlockPixels (android_java_env, bitmap);                // C 5124

  /* Delete the bitmap reference.  */                                   // C 5126
  ANDROID_DELETE_LOCAL_REF! (bitmap);                                   // C 5127
}

/* C 5130-5145: C 5130's blank, C 5131's `void` and `android_bell`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_bell ()                                // C 5130-5145
{
  let mut duration: jint;                                               // C 5133

  duration = 0;                                                 // (C leaves `duration` uninitialised; PORTING.md 1.5)

  /* Restrict android_keyboard_bell_duration to values between 10 and   // C 5135
     1000.  */                                                          // C 5136
  duration = (MIN! (1000 as isize, MAX! (0 as isize,
			     android_keyboard_bell_duration))) as jint;   // C 5137

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 5139
	(android_java_env,                                                     // C 5140
	 emacs_service,                                                        // C 5141
	 service_class.class,                                                  // C 5142
	 service_class.ring_bell,                                              // C 5143
	 duration);                                                            // C 5143
  android_exception_check ();                                           // C 5144
}
/* C 5146-5162: android_set_input_focus.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_input_focus (handle: android_window, time: c_ulong) // C 5146-5162
{
  let mut window: jobject;                                              // C 5150
  let mut make_input_focus: jmethodID;                                  // C 5151

  /* C leaves both locals uninitialised (PORTING.md 1.5).  */
  window = std::ptr::null_mut ();
  make_input_focus = std::ptr::null_mut ();

  window = android_resolve_handle (handle);                             // C 5153
  make_input_focus = window_class.make_input_focus;                     // C 5154

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 5156
	(android_java_env,
	 window,                                                               // C 5157
	 window_class.class,                                                   // C 5158
	 make_input_focus,                                                     // C 5159
	 time as jlong);                                                       // C 5160 (C's `unsigned long`-into-`jlong`)
  android_exception_check ();                                           // C 5161
}
/* C 5164-5178: android_raise_window.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_raise_window (handle: android_window)  // C 5164-5178
{
  let mut window: jobject;                                              // C 5167
  let mut raise: jmethodID;                                             // C 5168

  /* C leaves both locals uninitialised (PORTING.md 1.5).  */
  window = std::ptr::null_mut ();
  raise = std::ptr::null_mut ();

  window = android_resolve_handle (handle);                             // C 5170
  raise = window_class.raise;                                           // C 5171

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 5173
	(android_java_env,
	 window,                                                               // C 5174
	 window_class.class,                                                   // C 5175
	 raise);                                                               // C 5176
  android_exception_check ();                                           // C 5177
}
/* C 5180-5194: android_lower_window.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_lower_window (handle: android_window)  // C 5180-5194
{
  let mut window: jobject;                                              // C 5183
  let mut lower: jmethodID;                                             // C 5184

  /* C leaves both locals uninitialised (PORTING.md 1.5).  */
  window = std::ptr::null_mut ();
  lower = std::ptr::null_mut ();

  window = android_resolve_handle (handle);                             // C 5186
  lower = window_class.lower;                                           // C 5187

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 5189
	(android_java_env,
	 window,                                                               // C 5190
	 window_class.class,                                                   // C 5191
	 lower);                                                               // C 5192
  android_exception_check ();                                           // C 5193
}
/* C 5196-5224: android_reconfigure_wm_window.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_reconfigure_wm_window (handle: android_window,
					       value_mask: android_wc_value_mask,
					       values: *mut android_window_changes) // C 5196-5224
{
  let mut sibling: jobject;                                             // C 5201
  let mut window: jobject;

  /* C leaves both locals uninitialised (PORTING.md 1.5).  */
  sibling = std::ptr::null_mut ();
  window = std::ptr::null_mut ();

  window = android_resolve_handle (handle);                             // C 5203

  if value_mask & ANDROID_CW_STACK_MODE == 0                            // C 5205 (C's `!(…)`)
    { return; }                                                         // C 5206

  /* If value_mask & ANDROID_CW_SIBLING, place HANDLE above or below    // C 5208
     values->sibling pursuant to values->stack_mode; else, reposition   // C 5209
     it at the top or the bottom of its parent.  */                     // C 5210

  sibling = std::ptr::null_mut ();                                      // C 5212

  if value_mask & ANDROID_CW_SIBLING != 0                               // C 5214
    { sibling = android_resolve_handle ((*values).sibling); }           // C 5215

  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())           // C 5217
	(android_java_env,
	 window,                                                               // C 5218
	 window_class.class,                                                   // C 5219
	 window_class.reconfigure,                                             // C 5220
	 sibling,                                                              // C 5221
	 (*values).stack_mode as jint);                                        // C 5222 (C's `int`-into-`jint`)
  android_exception_check ();                                           // C 5223
}
/* C 5226-5280: `android_query_tree`.  C 5233's `jsize nelements, i` - `i` has no declaration of
   its own (C 5263's loop is the port's range), and C 5263-5266's brace-less `for` with its
   two-line comment takes Rust braces.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_query_tree (handle: android_window,    // C 5226-5280
					       root_return: *mut android_window,
					       parent_return: *mut android_window,
					       children_return: *mut *mut android_window,
					       nchildren_return: *mut c_uint)
  -> c_int
{
  let mut window: jobject;                                              // C 5232
  let mut array: jobject;
  let mut nelements: jsize;                                             // C 5233
  let mut children: *mut android_window;                                // C 5234
  let mut longs: *mut jlong;                                            // C 5235
  let mut method: jmethodID;                                            // C 5236

  /* C leaves the six locals uninitialised (PORTING.md 1.5).  */
  window = std::ptr::null_mut ();
  array = std::ptr::null_mut ();
  nelements = 0;
  children = std::ptr::null_mut ();
  longs = std::ptr::null_mut ();
  method = std::ptr::null_mut ();

  window = android_resolve_handle (handle);                             // C 5238

  /* window can be NULL, so this is a service method.  */               // C 5240
  method = service_class.query_tree;                                    // C 5241
  array
	= ((*(*android_java_env)).CallNonvirtualObjectMethod.unwrap ())   // C 5243
	    (android_java_env,                                                 // C 5243
	     emacs_service,                                                    // C 5244
	     service_class.class,                                              // C 5245
	     method, window);                                                  // C 5246
  android_exception_check ();                                           // C 5247

  /* The first element of the array is the parent window.  The rest     // C 5249
     are the children.  */                                              // C 5250
  nelements = ((*(*android_java_env)).GetArrayLength.unwrap ()) (android_java_env, // C 5251
							     array);                                                     // C 5252
  eassert! (nelements != 0);                                            // C 5253

  /* Now fill in the children.  */                                      // C 5255
  children = xnmalloc ((nelements - 1) as isize,                        // C 5256
			 std::mem::size_of::<android_window> () as isize) as *mut android_window;

  longs
	= ((*(*android_java_env)).GetLongArrayElements.unwrap ())   // C 5258
	    (android_java_env, array,                                          // C 5259
	     std::ptr::null_mut ());                                           // C 5260
  android_exception_check_nonnull (longs as *mut c_void, array);        // C 5261

  for i in 1..nelements                                                 // C 5263
	{
	  /* Subtract one from the index into children, since the parent is    // C 5264
	     not included.  */                                                 // C 5265
	  *children.offset ((i - 1) as isize) = *longs.offset (i as isize) as android_window; // C 5266 (C's `jlong`-into-`android_window`)
	}

  /* Finally, return the parent and other values.  */                   // C 5268
  *root_return = 0;                                                     // C 5269
  *parent_return = *longs.offset (0) as android_window;                 // C 5270 (the same conversion)
  *children_return = children;                                          // C 5271
  *nchildren_return = (nelements - 1) as c_uint;                        // C 5272

  /* Release the array contents.  */                                    // C 5274
  ((*(*android_java_env)).ReleaseLongArrayElements.unwrap ()) (android_java_env, array, // C 5275
							     longs, JNI_ABORT);                                          // C 5276

  ANDROID_DELETE_LOCAL_REF! (array);                                    // C 5278
  return 1;                                                             // C 5279
}
