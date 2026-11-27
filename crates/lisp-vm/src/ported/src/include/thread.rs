// femacs: mirror of src/thread.h @ fd7afced403.
//
// Complete header mirror (PORTING.md 0.1): the frozen configuration's live text - the two
// records, the checkpoint accessors, the four Lisp-object records and the prototypes.  Every
// `#if` in the header is decided by docs/baseline/config/darwin-arm64.macros.tsv, and the
// dead arms are recorded in docs/ports/include-thread.md rather than transcribed (P-08):
// `HAVE_ANDROID` (the `JNIEnv *java_env` member at thread.h 98, and `"android.h"` at 32-38),
// `WINDOWSNT` (24-26's `<sys/socket.h>`) and `MSDOS` (28-31's `<time.h>`/`<signal.h>`).
//
// The declarations of the functions `src/thread.c` itself defines sit in `pub mod
// thread_decls` and the two objects it defines in `pub mod thread_objs`, each with a glob
// import, so that unit's own `#[unsafe(no_mangle)]` definitions shadow the glob instead of
// colliding with it (PORTING.md 0.1-10/P-21, the `menu.rs`/`keyboard.rs` shape).
//
// Two names C declares in `lisp.h` are declared here, because this mirror names them and a
// mirror may only name a type it declares itself or reaches through its own `include!`s
// (PORTING.md P-72): `struct handler` (lisp.h 3839, whose
// `handlertype`/`nonlocal_exit`/`specpdl_ref` members are already in `lisp.rs`) and
// `sys_jmp_buf` (lisp.h 2273).  `struct
// re_registers` comes through `regex-emacs.rs`, exactly as C's thread.h 21 includes
// regex-emacs.h for it.  When `lisp.rs` carries the two, the copies here are the ones to drop
// (the dired.rs `struct dirent` and keyboard.rs `KBOARD` precedents).
//
// Degradations this file carries (P-55): `struct catchtag`, `struct buffer` and `struct
// bc_frame` have no mirror yet and appear only as pointers in thread.h, so those members keep
// the untyped pointer with a `/// C: …` line and the use sites cast (PORTING.md P-72 rule ③).

// thread.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const THREAD_H: bool = true;

// [U1] systhread.rs imported via super

/* C reaches `JNIEnv` through android.h (thread.h 33-44 includes it).  The port already mirrors
   `<jni.h>` as include/jni.rs, where `JNIEnv` is the real `*const JNINativeInterface`, so this
   mirror includes that rather than declaring an opaque stand-in of its own - the stand-in was
   the duplicate that broke eight Android units (status 4.643).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = ();
// [U1] ../../lib/include/signal.rs imported via super
// [U1] ../../lib/include/sys_select.rs imported via super
// [U1] regex-emacs.rs imported via super

/// C: lisp.h 2272-2273's `typedef jmp_buf sys_jmp_buf` (the `HAVE__SETJMP` arm, macro table
/// 1691).  `lisp.rs`'s `jmp_buf` is the opaque `[u8; 192]` its layout note records, but C's
/// `jmp_buf` is `int[48]`, so `_Alignof (sys_jmp_buf)` is 4 (measured with clang against the
/// frozen target) where `[u8; 192]` is 1; `struct thread_state` embeds one at `m_getcjmp@216`
/// and `sizeof thread_state` is 472.  The record wraps the array to keep that alignment; a
/// unit that hands it to `sys_setjmp` spells the cast then.
#[repr(C, align(4))]
#[derive(Clone, Copy)]
#[cfg (not (any (feature = "have_android", target_os = "linux", target_os = "windows")))]
pub struct sys_jmp_buf { pub bytes: [u8; 192] }
#[cfg (not (any (feature = "have_android", target_os = "linux", target_os = "windows")))]
const _: () = core::assert! (std::mem::size_of::<sys_jmp_buf> () == 192);
#[cfg (target_os = "windows")]
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct sys_jmp_buf { pub bytes: [u8; 256] }
#[cfg (target_os = "windows")]
const _: () = core::assert! (std::mem::size_of::<sys_jmp_buf> () == 256);
/* bionic's `sigjmp_buf` is 256 on aarch64.  */
#[cfg (feature = "have_android")]
#[derive (Copy, Clone)]
#[repr (C, align (8))]
pub struct sys_jmp_buf { pub bytes: [u8; 256] }
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<sys_jmp_buf> () == 256);
#[cfg (all (target_os = "linux", target_arch = "aarch64"))]
#[derive (Copy, Clone)]
#[repr (C, align (8))]
pub struct sys_jmp_buf { pub bytes: [u8; 312] }
#[cfg (all (target_os = "linux", target_arch = "aarch64"))]
const _: () = core::assert! (std::mem::size_of::<sys_jmp_buf> () == 312);
#[cfg (all (target_os = "linux", target_arch = "x86_64"))]
#[derive (Copy, Clone)]
#[repr (C, align (8))]
pub struct sys_jmp_buf { pub bytes: [u8; 200] }
#[cfg (all (target_os = "linux", target_arch = "x86_64"))]
const _: () = core::assert! (std::mem::size_of::<sys_jmp_buf> () == 200);
#[cfg (not (any (feature = "have_android", target_os = "linux", target_os = "windows")))]
const _: () = core::assert! (std::mem::align_of::<sys_jmp_buf> () == 4);
/* `_Alignof(sys_jmp_buf)` is 8 on android, glibc Linux, and Win64 isolate host.  */
#[cfg (any (feature = "have_android", target_os = "linux", target_os = "windows"))]
const _: () = core::assert! (std::mem::align_of::<sys_jmp_buf> () == 8);

/* Byte-code interpreter thread state.  */
// thread.h 54-60.  `fp` is `struct bc_frame *`, a type no mirror carries and only a pointer
// here (P-55); measured `sizeof (struct bc_thread_state)` = 24, `fp@0`, `stack@8`,
// `stack_end@16`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bc_thread_state
{
  pub fp: *mut c_void,          // C: `struct bc_frame *fp;`
  pub stack: *mut c_char,       // 62
  pub stack_end: *mut c_char,   // 63
}
const _: () = core::assert! (std::mem::size_of::<bc_thread_state> () == 24);
const _: () = core::assert! (std::mem::offset_of! (bc_thread_state, stack) == 8);
const _: () = core::assert! (std::mem::offset_of! (bc_thread_state, stack_end) == 16);

/// C: lisp.h 3839-3873's `struct handler`.  thread.h only ever takes its address
/// (`m_handlerlist`/`m_handlerlist_sentinel`, 118-127), but `src/thread.c` pushes one, walks
/// the two links and reads `tag_or_ch`/`val`, so the record is declared here until `lisp.rs`
/// carries it (P-72; the ledger records the deviation).  `HAVE_X_WINDOWS`'s
/// `x_error_handler_depth` is dead on this target.  The frozen baseline already pins it:
/// `sizeof handler 288`, `tag_or_ch@8`, `next@32`, `nextfree@40`, `val@24`, `jmp@60`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct handler
{
  pub r#type: handlertype,               // C: `enum handlertype type;` (lisp.rs 1812)
  pub tag_or_ch: Lisp_Object,            // 3842
  pub nonlocal_exit: nonlocal_exit,      // 3845 (lisp.rs 1821)
  pub val: Lisp_Object,                  // 3846
  pub next: *mut handler,                // 3848
  pub nextfree: *mut handler,            // 3849
  pub bytecode_top: *mut Lisp_Object,    // 3857
  pub bytecode_dest: c_int,              // 3858
  pub jmp: sys_jmp_buf,                  // 3862
  pub f_lisp_eval_depth: intmax_t,       // 3863
  pub pdlcount: specpdl_ref,             // 3864 (lisp.rs 1834)
  pub act_rec: *mut c_void,              // C: `struct bc_frame *act_rec;` (3865, P-55)
  pub poll_suppress_count: c_int,        // 3866
  pub interrupt_input_blocked: c_int,    // 3867
  // lisp.h 3868-3870: C's `#ifdef HAVE_X_WINDOWS` arm - the depth `eval.c`'s unwind restores
  // (`x_unwind_errors_to (catch->x_error_handler_depth)`, eval.c 1442, and the store at 1862).
  #[cfg (feature = "have_x_windows")]
  pub x_error_handler_depth: c_int,      // 3869 (measured 288; 288 -> 296, P-82 reading)
}

/* The buffer in which the last search was performed, or Qt if the last search was done in a
   string; Qnil if no searching has been done yet.  */
// thread.h 70-236.  The layout is the frozen target's, measured with clang against the frozen
// configuration: 472 bytes, alignment 8.  `event_object` is the last Lisp field (C 92-93) and
// is what `PVECHEADERSIZE`/`PSEUDOVECSIZE` count, so its offset is asserted here; so are the
// two records this file embeds (`m_getcjmp`, the 192-byte `sys_jmp_buf`, and `bc`).
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct thread_state
{
  pub header: vectorlike_header,                  // 71
  pub m_last_thing_searched: Lisp_Object,         // 74
  pub m_saved_last_thing_searched: Lisp_Object,   // 77
  pub name: Lisp_Object,                          // 80
  pub function: Lisp_Object,                      // 83
  pub result: Lisp_Object,                        // 86
  pub error_symbol: Lisp_Object,                  // 89
  pub error_data: Lisp_Object,                    // 90
  pub buffer_disposition: Lisp_Object,            // 93
  pub event_object: Lisp_Object,
  /* C: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` / `JNIEnv *java_env;` (thread.h
     97-99, between `event_object` and `m_stack_bottom`).  Without it the Android arm's
     `thread_state` was 8 bytes short of C's at this point, and 72 short overall (P-236).  */
  #[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
  pub java_env: *mut JNIEnv,                  // 97
  pub m_stack_bottom: *const c_char,              // C: `char const *m_stack_bottom;` (102)
  pub stack_top: *const c_void,                   // 110
  pub m_catchlist: *mut c_void,                   // C: `struct catchtag *m_catchlist;` (112)
  pub m_handlerlist: *mut handler,                // 118
  pub m_handlerlist_sentinel: *mut handler,       // 124
  pub m_specpdl: *mut specbinding,                // 130
  pub m_specpdl_end: *mut specbinding,            // 134
  pub m_specpdl_ptr: *mut specbinding,            // 138
  pub m_lisp_eval_depth: intmax_t,                // 142
  pub m_current_buffer: *mut c_void,              // C: `struct buffer *m_current_buffer;` (146)
  pub m_search_regs: re_registers,                // 166
  pub m_saved_search_regs: re_registers,          // 170
  pub m_waiting_for_user_input_p: c_int,          // 179
  pub m_waiting_for_input: bool,                  // 182
  pub m_getcjmp: sys_jmp_buf,                     // 190
  pub thread_id: sys_thread_t,                    // 193
  pub thread_condvar: sys_cond_t,                 // 197
  pub wait_condvar: *mut sys_cond_t,              // 202
  pub thread_name: *mut c_char,                   // 205
  pub not_holding_lock: c_int,                    // 213
  pub next_thread: *mut thread_state,             // 216
  pub bc: bc_thread_state,                        // 218
}
/* The Android arm has a different shape: measured with `_Static_assert` against C's own
   headers under NDK 29 / aarch64, which reads 592 for this one (darwin reads
   472).  `docs/baseline/android-layout-probe.md`.  */
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<thread_state> () == 592);
#[cfg (all (not (feature = "have_android"), not (target_os = "linux"), not (target_os = "windows"), not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::size_of::<thread_state> () == 472);
#[cfg (all (target_os = "windows", not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::size_of::<thread_state> () == 536);
#[cfg (all (target_os = "linux", target_arch = "aarch64", not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::size_of::<thread_state> () == 592);
#[cfg (all (target_os = "linux", target_arch = "x86_64", not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::size_of::<thread_state> () == 480);
// Expected Darwin pthread layout; target C ABI verification is required by P-387.
#[cfg (all (target_os = "macos", feature = "threads_enabled", feature = "have_pthread"))]
const _: () = core::assert! (std::mem::size_of::<thread_state> () == 520);
const _: () = core::assert! (std::mem::offset_of! (thread_state, event_object) == 72);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (thread_state, m_specpdl) == 128);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, m_specpdl) == 120);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (thread_state, m_getcjmp) == 224);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, m_getcjmp) == 216);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (thread_state, thread_id) == 480);
#[cfg (all (not (feature = "have_android"), not (target_os = "linux"), not (target_os = "windows")))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, thread_id) == 408);
#[cfg (target_os = "windows")]
const _: () = core::assert! (std::mem::offset_of! (thread_state, thread_id) == 472);
#[cfg (all (target_os = "linux", target_arch = "aarch64"))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, thread_id) == 528);
#[cfg (all (target_os = "linux", target_arch = "x86_64"))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, thread_id) == 416);
/* The Android arm has a different shape.  */
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (thread_state, not_holding_lock) == 552);
#[cfg (all (not (feature = "have_android"), not (target_os = "linux"), not (target_os = "windows"), not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, not_holding_lock) == 432);
#[cfg (all (target_os = "windows", not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, not_holding_lock) == 496);
#[cfg (all (target_os = "linux", target_arch = "aarch64", not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, not_holding_lock) == 552);
#[cfg (all (target_os = "linux", target_arch = "x86_64", not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, not_holding_lock) == 440);
#[cfg (all (target_os = "macos", feature = "threads_enabled", feature = "have_pthread"))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, not_holding_lock) == 480);
/* The Android arm has a different shape.  */
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (thread_state, bc) == 568);
#[cfg (all (not (feature = "have_android"), not (target_os = "linux"), not (target_os = "windows"), not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, bc) == 448);
#[cfg (all (target_os = "windows", not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, bc) == 512);
#[cfg (all (target_os = "linux", target_arch = "aarch64", not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, bc) == 568);
#[cfg (all (target_os = "linux", target_arch = "x86_64", not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, bc) == 456);
#[cfg (all (target_os = "macos", feature = "threads_enabled", feature = "have_pthread"))]
const _: () = core::assert! (std::mem::offset_of! (thread_state, bc) == 496);

/* thread.h 116-192: the checkpoint machinery.  Each macro is C's own body (PORTING.md
   0.1-18: a mirror defines its macros before their uses).  */

/* Last thing that wasn't a string or buffer searched.  */
macro_rules! last_thing_searched { () => { (*(*current_thread.as_mut_ptr ())).m_last_thing_searched } }        // 116
macro_rules! saved_last_thing_searched { () => { (*(*current_thread.as_mut_ptr ())).m_saved_last_thing_searched } } // 119

/* Chain of condition handlers currently in effect.  */
macro_rules! catchlist { () => { (*(*current_thread.as_mut_ptr ())).m_catchlist } }                            // 124
macro_rules! handlerlist { () => { (*(*current_thread.as_mut_ptr ())).m_handlerlist } }                        // 127
macro_rules! handlerlist_sentinel { () => { (*(*current_thread.as_mut_ptr ())).m_handlerlist_sentinel } }      // 130

/* Pointer to beginning of specpdl.  */
macro_rules! specpdl { () => { (*(*current_thread.as_mut_ptr ())).m_specpdl } }                                // 131
macro_rules! specpdl_end { () => { (*(*current_thread.as_mut_ptr ())).m_specpdl_end } }                        // 135
macro_rules! specpdl_ptr { () => { (*(*current_thread.as_mut_ptr ())).m_specpdl_ptr } }                        // 139
macro_rules! lisp_eval_depth { () => { (*(*current_thread.as_mut_ptr ())).m_lisp_eval_depth } }                // 143
// thread.h 147's `current_buffer` accessor lives in `include/buffer.rs`: the field is typed
// `*mut c_void` here (the incomplete `struct buffer` has no mirror in thread.h) and only the
// buffer mirror can name the pointee, so the cast and the accessor belong there.
macro_rules! search_regs { () => { (*(*current_thread.as_mut_ptr ())).m_search_regs } }                        // 167
macro_rules! saved_search_regs { () => { (*(*current_thread.as_mut_ptr ())).m_saved_search_regs } }            // 170
macro_rules! waiting_for_user_input_p { () => { (*(*current_thread.as_mut_ptr ())).m_waiting_for_user_input_p } } // 181
macro_rules! waiting_for_input { () => { (*(*current_thread.as_mut_ptr ())).m_waiting_for_input } }            // 185
macro_rules! getcjmp { () => { (*(*current_thread.as_mut_ptr ())).m_getcjmp } }                                // 192
macro_rules! stack_bottom { () => { (*(*current_thread.as_mut_ptr ())).m_stack_bottom } }                      // 104

/* thread.h 43-49: `NEAR_STACK_TOP`.  `HAVE___BUILTIN_FRAME_ADDRESS` is 1 (macro table 1693),
   so the live arm is `((void) (addr), __builtin_frame_address (0))`; the port renders the
   builtin with the frame-pointer register (PORTING.md 4.3's decision for comp.c) and the
   literal token is in docs/ports/macro-body-notes.tsv.  No live unit expands it yet.  */
macro_rules! NEAR_STACK_TOP { ($addr:expr) => {
  {
    let _ = $addr;
    // SAFETY: on the frozen target x29 is the frame pointer; reading it touches no memory.
    unsafe {
      let fp: *const c_void;
      core::arch::asm! ("mov {}, x29", out (reg) fp,
                        options (nomem, nostack, preserves_flags));
      fp
    }
  }
} }

// thread.h 220-226: `THREADP`.
#[inline(always)]
pub fn THREADP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_THREAD) }

// thread.h 228-233: `CHECK_THREAD`.
#[inline(always)]
pub fn CHECK_THREAD (x: Lisp_Object)
{
  CHECK_TYPE (THREADP (x), Qthreadp (), x);
}

// thread.h 235-240: `XTHREAD`.
#[inline(always)]
pub fn XTHREAD (a: Lisp_Object) -> *mut thread_state
{
  eassert! (THREADP (a));
  XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, thread_state)
}

/* A mutex in lisp is represented by a system condition variable.  */
// thread.h 250-256: `typedef struct { struct thread_state *owner; unsigned int count;
// sys_cond_t condition; } lisp_mutex_t;`  Measured 16 bytes, alignment 8.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct lisp_mutex_t
{
  pub owner: *mut thread_state,   // 252
  pub count: c_uint,              // 254
  pub condition: sys_cond_t,      // 256
}
/* The Android arm has a different shape: measured with `_Static_assert` against C's own
   headers under NDK 29 / aarch64, which reads 64 for this one (darwin reads
   16).  `docs/baseline/android-layout-probe.md`.  */
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<lisp_mutex_t> () == 64);
#[cfg (all (not (feature = "have_android"), not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::size_of::<lisp_mutex_t> () == 16);
#[cfg (all (target_os = "macos", feature = "threads_enabled", feature = "have_pthread"))]
const _: () = core::assert! (std::mem::size_of::<lisp_mutex_t> () == 64);

// thread.h 260-268: `struct Lisp_Mutex`.  Measured 32 bytes, alignment 8.
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Mutex
{
  pub header: vectorlike_header,  // 262
  pub name: Lisp_Object,          // 265
  pub mutex: lisp_mutex_t,        // 268
}
/* The Android arm has a different shape: measured with `_Static_assert` against C's own
   headers under NDK 29 / aarch64, which reads 80 for this one (darwin reads
   32).  `docs/baseline/android-layout-probe.md`.  */
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<Lisp_Mutex> () == 80);
#[cfg (all (not (feature = "have_android"), not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::size_of::<Lisp_Mutex> () == 32);
#[cfg (all (target_os = "macos", feature = "threads_enabled", feature = "have_pthread"))]
const _: () = core::assert! (std::mem::size_of::<Lisp_Mutex> () == 80);

// thread.h 269-273: `MUTEXP`.
#[inline(always)]
pub fn MUTEXP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_MUTEX) }

// thread.h 275-280: `CHECK_MUTEX`.
#[inline(always)]
pub fn CHECK_MUTEX (x: Lisp_Object)
{
  CHECK_TYPE (MUTEXP (x), Qmutexp (), x);
}

// thread.h 282-287: `XMUTEX`.
#[inline(always)]
pub fn XMUTEX (a: Lisp_Object) -> *mut Lisp_Mutex
{
  eassert! (MUTEXP (a));
  XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Mutex)
}

/* A condition variable as a lisp object.  */
// thread.h 291-301: `struct Lisp_CondVar`.  Measured 32 bytes, alignment 8.
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_CondVar
{
  pub header: vectorlike_header,  // 293
  pub mutex: Lisp_Object,         // 296
  pub name: Lisp_Object,          // 299
  pub cond: sys_cond_t,           // 301
}
/* `sizeof(struct Lisp_CondVar)` is 72 on android/aarch64 (32 on darwin): the Android arm holds a
   `sys_cond_t` (48) where darwin's holds a smaller one - measured with the same `_Static_assert`
   probe against C's headers (P-236).  */
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<Lisp_CondVar> () == 72);
#[cfg (all (not (feature = "have_android"), not (feature = "threads_enabled")))]
const _: () = core::assert! (std::mem::size_of::<Lisp_CondVar> () == 32);
#[cfg (all (target_os = "macos", feature = "threads_enabled", feature = "have_pthread"))]
const _: () = core::assert! (std::mem::size_of::<Lisp_CondVar> () == 72);

// thread.h 303-307: `CONDVARP`.
#[inline(always)]
pub fn CONDVARP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_CONDVAR) }

// thread.h 309-314: `CHECK_CONDVAR`.
#[inline(always)]
pub fn CHECK_CONDVAR (x: Lisp_Object)
{
  CHECK_TYPE (CONDVARP (x), Qcondition_variable_p (), x);
}

// thread.h 316-321: `XCONDVAR`.
#[inline(always)]
pub fn XCONDVAR (a: Lisp_Object) -> *mut Lisp_CondVar
{
  eassert! (CONDVARP (a));
  XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_CondVar)
}

// thread.h 322-323's two objects are `src/thread.c`'s own definitions, so they sit in
// `pub mod thread_objs` + glob import: the owning unit's `#[unsafe(no_mangle)] pub static
// mut` shadows the glob (PORTING.md 0.1-10/P-21, the globals.rs `variables` shape) instead of
// colliding with it (E0428).
pub mod thread_objs {
  use super::*;
  unsafe extern "C" {
    pub static current_thread: VmGlobal<*mut thread_state>;   // thread.h 322
    pub static all_threads: VmGlobal<*mut thread_state>;      // thread.h 323
  }
}
pub use self::thread_objs::*;

/// C: thread.h 337-338: `typedef int select_func (int, fd_set *, fd_set *, fd_set *,
// const struct timespec *, const sigset_t *);` - a function *type* in C, which its users
/// pass as `select_func *`; the port names the function-pointer type the way the atimer.h
/// mirror names `atimer_callback` (P-27), and call sites write `Option<select_func>`.
pub type select_func
  = unsafe extern "C" fn (c_int, *mut fd_set, *mut fd_set, *mut fd_set,
                          *const timespec, *const sigset_t) -> c_int;

// thread.h 325-345's prototypes are the ones `src/thread.c` defines, so they sit in
// `pub mod thread_decls` + glob import, for the same reason as `thread_objs` (0.1-10).
pub mod thread_decls {
  use super::*;

  unsafe extern "C" {
    pub fn finalize_one_thread (state: *mut thread_state);              // 325
    pub fn finalize_one_mutex (mutex: *mut Lisp_Mutex);                 // 326
    pub fn finalize_one_condvar (condvar: *mut Lisp_CondVar);           // 327
    pub fn maybe_reacquire_global_lock ();                              // 328
    pub fn init_threads ();                                             // 330
    pub fn syms_of_threads ();                                          // 331
    pub fn main_thread_p (ptr: *const c_void) -> bool;                  // 332
    pub fn in_current_thread () -> bool;                                // 333
    pub fn thread_select (func: Option<select_func>, max_fds: c_int,    // 341
                          rfds: *mut fd_set, wfds: *mut fd_set, efds: *mut fd_set,
                          timeout: *mut timespec, sigmask: *mut sigset_t) -> c_int;
    /// C: thread.h 342: `bool thread_check_current_buffer (struct buffer *);` - the
    /// parameter is the untyped pointer (P-55); the use sites cast.
    pub fn thread_check_current_buffer (buffer: *mut c_void) -> bool;
    pub fn thread_all_before_buffer_killed (buffer: Lisp_Object);       // 345
  }
}
pub use self::thread_decls::*;
