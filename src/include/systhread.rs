// femacs: mirror of src/systhread.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): the `SYSTHREAD_H` guard, the three dummy thread types this
// target selects, the `thread_creation_function` typedef and the thirteen prototypes.
//
// The frozen configuration has `THREADS_ENABLED` undefined
// (docs/baseline/config/darwin-arm64.macros.tsv 3678), so C 130-137's `#else` arm is the live
// text and everything in C 24-128 is the threaded build's: the `#include <pthread.h>`, the
// `SYSTHREAD_ALIGN_ROOM`/`SYSTHREAD_ALIGN_PTR` pair with its `ALIGNOF_PTHREAD_{COND,MUTEX}_T`
// comparison, the `pthread_mutex_t`/`pthread_cond_t`/`pthread_t` typedefs, the WINDOWSNT
// `w32thread_critsect`/`w32thread_cond_t` records and the `#error port me` fallback.  None of
// them is transcribed; docs/ports/include-systhread.md records the whole region.
//
// The prototypes live in `systhread_decls` + glob import (the tempname.rs shape), because
// src/systhread.c's port defines every one of them and an explicit item shadows a glob import
// (PORTING.md 0.1-10).

// C 19-20: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const SYSTHREAD_H: bool = true;

// [U1] ../../lib/include/attribute.rs imported via super

/* C gets these from `<pthread.h>` (systhread.h 28 includes it).  They live in this mirror so
   every mirror that names them can see them, and the unit includes this mirror too (P-240).  */
/* The shapes below are per target, because the port has to declare these types itself (C uses
   the system's).  bionic's are smaller than darwin's: measured with `_Static_assert(sizeof(T)==N)`
   under NDK 29 / aarch64 (`docs/baseline/android-pthread-sizes.txt`, P-232) - mutex 40 not 64,
   attr 56 not 64, mutexattr 8 not 16.  Using darwin's numbers on Android is what made
   `thread_state`'s size and offsets wrong there (P-231).  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_android"))]
#[derive (Copy, Clone)]
#[repr (C, align (4))]
pub struct pthread_mutex_t { pub __opaque: [c_uchar; 40] }       // bionic: sizeof 40, align 4
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_android"))]
const _: () = core::assert! (core::mem::size_of::<pthread_mutex_t> () == 40);
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "have_android")))]
#[derive (Copy, Clone)]
#[repr (C, align (8))]
pub struct pthread_mutex_t { pub __opaque: [c_uchar; 64] }       // clang: sizeof 64, align 8
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "have_android")))]
const _: () = core::assert! (core::mem::size_of::<pthread_mutex_t> () == 64);
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_android"))]
#[derive (Copy, Clone)]
#[repr (C, align (4))]
pub struct pthread_cond_t { pub __opaque: [c_uchar; 48] }        // bionic: sizeof 48, align 4
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "have_android")))]
const _: () = core::assert! (core::mem::size_of::<pthread_cond_t> () == 48);
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_android"))]
const _: () = core::assert! (core::mem::size_of::<pthread_cond_t> () == 48);
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "have_android")))]
#[derive (Copy, Clone)]
#[repr (C, align (8))]
pub struct pthread_cond_t { pub __opaque: [c_uchar; 48] }        // clang: sizeof 48, align 8
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_android"))]
#[derive (Copy, Clone)]
#[repr (C, align (4))]
pub struct pthread_attr_t { pub __opaque: [c_uchar; 56] }        // bionic: sizeof 56, align 4
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_android"))]
const _: () = core::assert! (core::mem::size_of::<pthread_attr_t> () == 56);
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "have_android")))]
#[derive (Copy, Clone)]
#[repr (C, align (8))]
pub struct pthread_attr_t { pub __opaque: [c_uchar; 64] }        // clang: sizeof 64
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "have_android")))]
const _: () = core::assert! (core::mem::size_of::<pthread_attr_t> () == 64);
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_android"))]
#[derive (Copy, Clone)]
#[repr (C, align (4))]
pub struct pthread_mutexattr_t { pub __opaque: [c_uchar; 8] }    // bionic: sizeof 8, align 4
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_android"))]
const _: () = core::assert! (core::mem::size_of::<pthread_mutexattr_t> () == 8);
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "have_android")))]
#[derive (Copy, Clone)]
#[repr (C, align (8))]
pub struct pthread_mutexattr_t { pub __opaque: [c_uchar; 16] }   // clang: sizeof 16
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "have_android")))]
const _: () = core::assert! (core::mem::size_of::<pthread_mutexattr_t> () == 16);
/* bionic spells `pthread_t` as `long` (8 bytes, align 8) where clang's is an opaque pointer;
   `android.rs` carried the NDK's spelling of the three types this mirror already declares with
   the frozen target's shapes, and the two copies collided (`3 x E0428` on `android.c`, measured),
   so the mirror carries both arms and the unit's copies went (docs/baseline/android-pthread-sizes.txt:
   cond_t 48 and pthread_t 8 both match the NDK).  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_android"))]
pub type pthread_t = c_long;                                     // bionic: sizeof 8
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_android"))]
const _: () = core::assert! (core::mem::size_of::<pthread_t> () == 8);
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "have_android")))]
pub type pthread_t = *mut c_void;                                // clang: sizeof 8 (an opaque pointer)
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "have_android")))]
const _: () = core::assert! (core::mem::size_of::<pthread_t> () == 8);
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
pub const PTHREAD_MUTEX_ERRORCHECK: c_int = 1;                   // clang: 1
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
pub const PTHREAD_CREATE_DETACHED: c_int = 2;                    // clang: 2

/* C 24-128: `#ifdef THREADS_ENABLED`'s two arms and the `#error` fallback.  The frozen
   configuration leaves `THREADS_ENABLED` undefined (docs/baseline/config/darwin-arm64.macros.tsv
   3678), so C 130-137's dummy arm below is the text this machine compiles; each arm here is
   carried under the lower-cased feature of the macro its own `#if` tests (P-86 clause 8), so a
   `threads_enabled` build reads the arm C would read.  The `<pthread.h>` surface itself
   (`pthread_mutex_t` and the 22 entry points) is *not* here: the tree already has two units that
   declare those names for their own arms (`src/android.rs`, `src/sysdep.rs`), and a second
   spelling in a mirror would be the duplicate `tools/check-mirror-dupes.py` fails on - the
   declaration lives in the one consumer this header's pthread arm has, `src/systhread.rs`
   (P-88 clause 1).  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
pub type sys_thread_t = pthread_t;   // C 84-85

/* C 46-67: the `ALIGNOF_PTHREAD_{COND,MUTEX}_T <= ALIGNOF_MAX_ALIGN_T` comparison pair.  Both
   macros are measured 8 on this target (macros.tsv 71-73), so C takes the *identity* arm; the
   other arm exists for the over-aligning platforms C's comment names (32-bit HPPA with glibc
   2.42) and is carried under the two derived features (P-86 clause 1).  */
/* C 51-67's `#else` branch defines `SYSTHREAD_ALIGN_ROOM (type)` as well as the pointer macro;
   the typical branch defines only the pointer macro, so neither the room type nor a room macro
   exists in the arm this machine reads.  The two `sys_*_t` arm unions below are that macro's
   expansion for the two types C applies it to.  */

/* C 46-50/60: the typical case's macro - `# define SYSTHREAD_ALIGN_PTR(type, ptr) (ptr)`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            feature = "alignof_pthread_cond_t_le_alignof_max_align_t",
            feature = "alignof_pthread_mutex_t_le_alignof_max_align_t"))]
macro_rules! SYSTHREAD_ALIGN_PTR { ($type:ty, $ptr:expr) => { $ptr }; }

/* C 56-61/64-66: the unusual case's macro, with `alignof (double)` = 8 spelled out.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (all (feature = "alignof_pthread_cond_t_le_alignof_max_align_t",
                      feature = "alignof_pthread_mutex_t_le_alignof_max_align_t"))))]
macro_rules! SYSTHREAD_ALIGN_PTR {
  ($type:ty, $ptr:expr) => {
    ((($ptr).room.as_ptr () as usize
      + (core::mem::align_of::<$type> () - 8))
     & !(core::mem::align_of::<$type> () - 8)) as *mut $type
  };
}

/* C 70-81: `sys_mutex_t`/`sys_cond_t` for the pthread arm.  Each inner `#if` is the same measured
   comparison, so on this target both are the plain typedefs.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            feature = "alignof_pthread_mutex_t_le_alignof_max_align_t"))]
pub type sys_mutex_t = pthread_mutex_t;
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "alignof_pthread_mutex_t_le_alignof_max_align_t")))]
#[repr (C)]
#[derive (Copy, Clone)]
pub union Systhread_align_room_mutex_t {
  pub i: f64,
  pub room: [c_char; core::mem::size_of::<pthread_mutex_t> ()
                    + core::mem::align_of::<pthread_mutex_t> () - 8],
}
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "alignof_pthread_mutex_t_le_alignof_max_align_t")))]
pub type sys_mutex_t = Systhread_align_room_mutex_t;

#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            feature = "alignof_pthread_cond_t_le_alignof_max_align_t"))]
pub type sys_cond_t = pthread_cond_t;
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "alignof_pthread_cond_t_le_alignof_max_align_t")))]
#[repr (C)]
#[derive (Copy, Clone)]
pub union Systhread_align_room_cond_t {
  pub i: f64,
  pub room: [c_char; core::mem::size_of::<pthread_cond_t> ()
                    + core::mem::align_of::<pthread_cond_t> () - 8],
}
#[cfg (all (feature = "threads_enabled", feature = "have_pthread",
            not (feature = "alignof_pthread_cond_t_le_alignof_max_align_t")))]
pub type sys_cond_t = Systhread_align_room_cond_t;

/* C 88-121: the WINDOWSNT arm - C's own declaration of the six windows.h data types under
   different names, the `CONDV_*` enum and the three typedefs.  Text only on this machine (no
   Windows toolchain: `which -a x86_64-w64-mingw32-gcc` is empty), so every width is C's own
   spelling, not a measurement.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[repr (C)]
#[derive (Copy, Clone)]
pub struct w32thread_critsect {
  pub DebugInfo: *mut c_void,       // C: `struct _CRITICAL_SECTION_DEBUG *`
  pub LockCount: c_long,
  pub RecursionCount: c_long,
  pub OwningThread: *mut c_void,
  pub LockSemaphore: *mut c_void,
  pub SpinCount: c_ulong,
}
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub const CONDV_SIGNAL: c_int = 0;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub const CONDV_BROADCAST: c_int = 1;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub const CONDV_MAX: c_int = 2;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[repr (C)]
#[derive (Copy, Clone)]
pub struct w32thread_cond_t {
  pub wait_count: c_uint,
  pub wait_count_lock: w32thread_critsect,
  pub events: [*mut c_void; CONDV_MAX as usize],
  pub initialized: bool,
}
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type sys_mutex_t = w32thread_critsect;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type sys_cond_t = w32thread_cond_t;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type sys_thread_t = c_ulong;

/* C 123-127: `#else /* !WINDOWSNT */` -> `#error port me`.  C's own fallback, spelled with the
   port's `compile_error!` (the `include/frame.rs` 319-321 precedent).  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"),
            not (feature = "windowsnt")))]
compile_error! ("port me");

/* For the no-threads case we can simply use dummy definitions.  */
#[cfg (not (feature = "threads_enabled"))]
pub type sys_mutex_t = c_int;   // C 133
#[cfg (not (feature = "threads_enabled"))]
pub type sys_cond_t = c_int;    // C 134
#[cfg (not (feature = "threads_enabled"))]
pub type sys_thread_t = c_int;  // C 135

// C 139: `typedef void *(thread_creation_function) (void *);` - a function *type* in C, which
// its users pass as `thread_creation_function *`; the port names the function-pointer type
// the way the atimer.h mirror names `atimer_callback` (P-27), and call sites write it bare.
pub type thread_creation_function
  = Option<unsafe extern "C" fn (arg: *mut c_void) -> *mut c_void>;

pub mod systhread_decls {
  use super::*;

  unsafe extern "C" {
    pub fn sys_mutex_init (sys_mutex: *mut sys_mutex_t);                        // C 141
    pub fn sys_mutex_lock (sys_mutex: *mut sys_mutex_t);                        // C 142
    pub fn sys_mutex_unlock (sys_mutex: *mut sys_mutex_t);                      // C 143
    pub fn sys_cond_init (c: *mut sys_cond_t);                                  // C 145
    pub fn sys_cond_wait (c: *mut sys_cond_t, m: *mut sys_mutex_t);             // C 146
    pub fn sys_cond_signal (c: *mut sys_cond_t);                                // C 147
    pub fn sys_cond_broadcast (c: *mut sys_cond_t);                             // C 148
    pub fn sys_cond_destroy (c: *mut sys_cond_t);                               // C 149
    // C 151-153/155-156 carry NODISCARD in front of the three declarations below; the token
    // has no Rust form (PORTING.md 0.3, attribute.h 187), so it is the no-op macro defined in
    // attribute.rs and not written at the declaration site.
    pub fn sys_thread_self () -> sys_thread_t;                                  // C 151
    pub fn sys_thread_equal (t: sys_thread_t, u: sys_thread_t) -> bool;         // C 152
    pub fn sys_thread_create (t: *mut sys_thread_t, func: thread_creation_function,
                              datum: *mut c_void) -> bool;                      // C 154-155
    pub fn sys_thread_yield ();                                                 // C 157
    pub fn sys_thread_set_name (name: *const c_char);                           // C 158
  }
}
pub use self::systhread_decls::*;
