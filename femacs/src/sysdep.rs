// femacs: port of src/sysdep.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, union, enum, macro, local), same control flow, same error model.  C's one `DEFUN`
// uses `DEFUN!` (PORTING.md 5.1).  C's non-`static` definitions carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn`, except the ones whose
// address C takes for a signal action or an unwind protect, which are
// `unsafe extern "C" fn` (PORTING.md 1.4).
//
// **Carried arms.**  `src/sysdep.c` is *not* in `docs/baseline/config/cond-eval-verified.txt`
// and `python3 tools/check-cond-eval.py src/sysdep.c` reports
// `283 conditions, 0 unknown, 2 disagreements [NOT VERIFIED]`: the frozen configuration's
// macro table cannot decide this file's system-header conditions (`<termios.h>`,
// `<sys/ioctl.h>`, `<syssignal.h>`), so `tools/extract-abi.py`'s `blind_conditions` keeps
// *both* arms of every `#if` live and the per-function token check demands the tokens of C's
// text on both sides.  Per the integrator's 2026-10-03 decision this file therefore *carries*
// the text of the arms the frozen build does not compile, each under a
// `#[cfg (feature = "…")]` that names the C condition's non-frozen side (PORTING.md P-51;
// `docs/status.md` 3.49, `femacs/src/timefns.rs` and `femacs/src/callproc.rs` are the
// precedents).  None of these features is ever set: `xtask dead-arms` can only flip the keys
// `config.rs` carries and these are this file's platform/preprocessor predicates
// (docs/status.md 3.9), so the compiled build is exactly what clang compiles.  The live/dead
// split below was measured with clang on every condition of the file (the same probe
// `tools/check-cond-eval.py` uses), and it is recorded in `docs/ports/sysdep.md`.
//
// The two remaining `check-cond-eval.py` disagreements (C 1135's `#ifdef INLCR /* …` and
// C 1279's `#ifndef HAIKU /* …`) are a *tool* defect, not a table gap: `tools/cinventory.py`
// keeps the trailing comment inside the condition text, so the probe it feeds clang is
// ill-formed (`unterminated /* comment`) and clang answers "dead" for a branch the frozen
// build compiles.  The evaluator is right in both; the port follows clang (both arms are live)
// and `docs/ports/sysdep.md` reports the defect.
//
// `#if` classification (PORTING.md 7.2): this file's conditions are the platform's and the
// preprocessor's own predicates — structural where they select a type or a whole function
// (carried under `#[cfg (feature = …)]`), constant where they select statements inside one
// function (carried as `#[cfg (feature = …)] { … }` blocks), and the `#include` selections
// (C 24-142) are recorded in `docs/ports/sysdep.md`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut, unused_labels, unused_comparisons, overflowing_literals,
         unnecessary_cast)]

/* ------------------------------------------------------------------------------------------
   C 20-142: the `#include`s.  The mirrors are named with C's own header in the comment; the
   headers this target's port has no mirror for (`<execinfo.h>`, `"sysstdio.h"`, `<pwd.h>`,
   `<grp.h>`, `<sys/param.h>`, `<sys/file.h>`, `"sysselect.h"`, `"systty.h"`, `"syswait.h"`,
   `"termopts.h"`, `<sys/utsname.h>`, `<libproc.h>`, `<mach/mach_time.h>`, `<pthread.h>`,
   `<netdb.h>`, `<sys/socket.h>`) contribute the declarations further down, each with C's own
   prototype in a `/// C:` line (PORTING.md P-72 rule 3, the terminal.rs/callproc.rs shape).
   C 41-142's platform arms (`HAVE_LINUX_FS_H`, `CYGWIN`, `__OpenBSD__`, `__FreeBSD__`,
   `WINDOWSNT`, `HAVE_SYS_SYSTEMINFO_H`, `MSDOS`, `HAVE_ANDROID`, `HAVE_PERSONALITY_ADDR_NO_RANDOMIZE`)
   are dead for the frozen build and carry no `#include` the port needs.  */

include!("include/config.rs");                 // C 20: `#include <config.h>`
include!("include/conf_post.rs");               // C 20: the AH_BOTTOM `#include <conf_post.h>` inside it
include!("include/sysstdio.rs");                // C 23: `#include "sysstdio.h"`
include!("../lib/include/limits.in.rs");       // C 28: `#include <limits.h>`
include!("../lib/include/stdcountof.rs");      // lisp.h 30's <stdcountof.h>: `countof`
include!("../lib/include/stdlib.rs");          // C 29: `#include <stdlib.h>`
include!("../lib/include/sys_random.in.rs");   // C 30: `#include <sys/random.h>`
include!("../lib/include/unistd.in.rs");       // C 31: `#include <unistd.h>`
include!("../lib/include/c-ctype.rs");         // C 33: `#include <c-ctype.h>`
include!("../lib/include/close-stream.rs");    // C 34: `#include <close-stream.h>`
include!("include/lisp.rs");                   // C 37: `#include "lisp.h"`
include!("include/character.rs");              // keyboard.h 24: `#include "character.h"`
include!("include/itree.rs");                  // buffer.h 28: `#include "itree.h"`
include!("include/buffer.rs");                 // keyboard.h 23: `#include "buffer.h"`
include!("include/category.rs");               // character.h 27: `#include "category.h"`
include!("include/dispextern.rs");             // window.h 23: `#include "dispextern.h"`
include!("include/composite.rs");              // keyboard.h 28: `#include "composite.h"`
include!("include/coding.rs");                 // termhooks.h 26: `#include "coding.h"`
include!("include/sysselect.rs");                // C 38: `#include "sysselect.h"`
include!("include/blockinput.rs");             // C 39: `#include "blockinput.h"`
include!("../lib/include/sys_types.in.rs");    // C 87: `#include <sys/types.h>`
include!("../lib/include/sys_stat.in.rs");     // C 88: `#include <sys/stat.h>`
include!("../lib/include/errno.rs");           // C 89: `#include <errno.h>`
include!("../lib/include/fcntl.in.rs");        // C 101: `#include <fcntl.h>`
include!("../lib/include/signal.rs");          // C 104: `#include "syssignal.h"` -> <signal.h>
include!("include/syssignal.rs");              // C 104: `#include "syssignal.h"`
include!("include/systime.rs");                // C 105: `#include "systime.h"`
include!("include/systty.rs");                 // C 106: `#include "systty.h"`
include!("include/syswait.rs");                // C 107: `#include "syswait.h"`
include!("../lib/include/pwd.rs");           // C: `#include <pwd.h>`
include!("../lib/include/grp.rs");           // C: `#include <grp.h>`
include!("../lib/include/sys_utsname.rs");   // C: `#include <sys/utsname.h>`
include!("../lib/include/string.rs");          // lisp.h's <string.h>: memcpy, strchr, …
include!("../lib/include/wchar.rs");           // C 4566: `#include <wchar.h>`
include!("../lib/include/wctype.rs");          // C 4567: `#include <wctype.h>`
include!("../lib/include/math.rs");            // C 4570: `#include <math.h>` (the `exp`/`log`)
include!("../lib/include/sys_sysctl.rs");      // C 50/51: `#include <sys/sysctl.h>`
include!("../lib/include/sys_resource.rs");    // C 109: `#include <sys/resource.h>`
include!("include/keyboard.rs");               // C 118: `#include "keyboard.h"`
include!("include/window.rs");                 // frame.h 24: `#include "window.h"`
include!("include/frame.rs");                  // C 119: `#include "frame.h"`
include!("include/termhooks.rs");              // C 120: `#include "termhooks.h"`
include!("include/termchar.rs");               // C 121: `#include "termchar.h"`
include!("include/process.rs");                // C 123: `#include "process.h"`
include!("include/cm.rs");                     // C 124: `#include "cm.h"`
/* C 137: `#include "android.h"`, inside CC's `#ifdef HAVE_ANDROID`.  The mirror declares C's 85 prototypes in
   its two modules and glob-imports the second (`include/android.rs` 324), so including it is what
   supplies `android_is_special_directory`/`android_rewrite_spawn_argv`/`android_init_gui` - the
   names this unit read as `cannot find function` on `aarch64-linux-android` (measured).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/android.rs");               // C 137: `#include "android.h"`

/* ==========================================================================================
   The declarations C gets from headers the port has no mirror for.  Each has C's own text
   (or the header's measured macro body) in a `/// C:` line; the layouts are the frozen
   target's, measured with the frozen clang and pinned by the `const _` assertions each
   record carries.  */

/// C 145: `extern void tputs (const char *, int, int (*)(int));` — `sysdep.c` declares it
/// itself ("including term.h is problematic on some systems").
unsafe extern "C" {
  pub fn tputs (cp: *const c_char, affcnt: c_int,
                putc: Option<unsafe extern "C" fn (c_int) -> c_int>) -> c_int;
}

/* `<termios.h>`, through C 106's `#include "systty.h"`: the four typedefs, `NCCS` and
   `struct termios` now come from `../lib/include/termios.rs` (the port's system mirror,
   measured with the frozen clang and pinned by its own `const _` assertions).  The local
   measured copy this unit carried until 2026-10-04 was an exact duplicate and was removed
   in the commit that added the mirror - `docs/ports/include-termios.md` records why the
   convergence had to wait for the lanes, and `docs/ports/sysdep.md` records it here.  */

/* struct winsize
   {
     unsigned short ws_row; unsigned short ws_col;
     unsigned short ws_xpixel; unsigned short ws_ypixel;
   };  */
/// `<sys/ioctl.h>`'s `struct winsize`, measured: `sizeof` 8, `ws_row`@0, `ws_col`@2,
/// `ws_xpixel`@4, `ws_ypixel`@6.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct winsize {
  pub ws_row: c_ushort,
  pub ws_col: c_ushort,
  pub ws_xpixel: c_ushort,
  pub ws_ypixel: c_ushort,
}

const _: () = core::assert! (std::mem::size_of::<winsize> () == 8);
const _: () = core::assert! (std::mem::offset_of! (winsize, ws_row) == 0);
const _: () = core::assert! (std::mem::offset_of! (winsize, ws_col) == 2);

/* struct rusage { struct timeval ru_utime, ru_stime; long ru_maxrss; long ru_ixrss;
                  long ru_idrss; long ru_isrss; long ru_minflt; long ru_majflt;
                  long ru_nswap; long ru_inblock; long ru_oublock; long ru_msgsnd;
                  long ru_msgrcv; long ru_nsignals; long ru_nvcsw; long ru_nivcsw; };  */
/// `<sys/resource.h>`'s `struct rusage`, measured: `sizeof` 144, `ru_utime`@0, `ru_stime`@16,
/// `ru_maxrss`@32, `ru_minflt`@64, `ru_majflt`@72.  (`sys_resource.rs` carries only
/// `getrlimit`/`setrlimit`/`rlimit`, so the record and `getrusage` are declared here.)
#[repr(C)]
pub struct rusage {
  pub ru_utime: timeval,
  pub ru_stime: timeval,
  pub ru_maxrss: c_long,
  pub ru_ixrss: c_long,
  pub ru_idrss: c_long,
  pub ru_isrss: c_long,
  pub ru_minflt: c_long,
  pub ru_majflt: c_long,
  pub ru_nswap: c_long,
  pub ru_inblock: c_long,
  pub ru_oublock: c_long,
  pub ru_msgsnd: c_long,
  pub ru_msgrcv: c_long,
  pub ru_nsignals: c_long,
  pub ru_nvcsw: c_long,
  pub ru_nivcsw: c_long,
}

const _: () = core::assert! (std::mem::size_of::<rusage> () == 144);
const _: () = core::assert! (std::mem::offset_of! (rusage, ru_utime) == 0);
const _: () = core::assert! (std::mem::offset_of! (rusage, ru_stime) == 16);
const _: () = core::assert! (std::mem::offset_of! (rusage, ru_maxrss) == 32);
const _: () = core::assert! (std::mem::offset_of! (rusage, ru_minflt) == 64);
const _: () = core::assert! (std::mem::offset_of! (rusage, ru_majflt) == 72);

/* systty.h 66-75: `struct emacs_tty { struct termios main; };` now comes from
   `include/systty.rs` (C 106), with the mirror's measured size assertion.  */

/* signal.h: `struct __siginfo { int si_signo; int si_errno; int si_code; pid_t si_pid;
   uid_t si_uid; int si_status; void *si_addr; union __sigval si_value; long si_band; };`  */
/* bionic's `siginfo_t` is 128 bytes with `si_addr`@16 (`si_signo`/`si_errno`/`si_code`, then the
   8-aligned `_sifields` union whose `_sigfault.si_addr` is first); the host record below is 104 with
   `si_addr`@24.  Measured against the NDK for API 21/28/36 (status 4.831).  */
#[cfg (feature = "have_android")]
/// bionic's `siginfo_t`: the three tags, then the `_sifields` union (128 bytes in all).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct siginfo_t {
  pub si_signo: c_int,
  pub si_errno: c_int,
  pub si_code: c_int,
  pub __pad0: c_int,
  pub si_addr: *mut c_char,
  pub __rest: [u64; 13],
}

#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<siginfo_t> () == 128);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (siginfo_t, si_code) == 8);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (siginfo_t, si_addr) == 16);

#[cfg (not (feature = "have_android"))]
/// The host's record, measured: `sizeof` 104, `si_addr`@24.  Only `si_addr` is reached (C 1885).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct siginfo_t {
  pub si_signo: c_int,
  pub si_errno: c_int,
  pub si_code: c_int,
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_status: c_int,
  pub si_addr: *mut c_char,
  pub si_value: u64,
  pub si_band: c_long,
}

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (siginfo_t, si_addr) == 24);

/* signal.h: `typedef struct __darwin_sigaltstack { void *ss_sp; __darwin_size_t ss_size;
   int ss_flags; } stack_t;`  */
/* bionic's `stack_t` swaps the host's last two members: `ss_sp`@0, `ss_flags`@8, `ss_size`@16
   (NDK, API 21/28/36).  With the host order `sigaltstack` read `ss_flags` as 0x10000 and failed
   with EINVAL, so the SIGSEGV handler never got its alternate stack (status 4.831).  */
#[cfg (feature = "have_android")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct stack_t {
  pub ss_sp: *mut c_void,
  pub ss_flags: c_int,
  pub ss_size: usize,
}

#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<stack_t> () == 24);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (stack_t, ss_sp) == 0);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (stack_t, ss_flags) == 8);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (stack_t, ss_size) == 16);

#[cfg (not (feature = "have_android"))]
/// The host's `stack_t`, measured: `sizeof` 24, `ss_sp`@0, `ss_size`@8, `ss_flags`@16.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct stack_t {
  pub ss_sp: *mut c_void,
  pub ss_size: usize,
  pub ss_flags: c_int,
}

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<stack_t> () == 24);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (stack_t, ss_sp) == 0);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (stack_t, ss_size) == 8);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (stack_t, ss_flags) == 16);

/// `<stddef.h>`'s `max_align_t`: the frozen target's widest scalar alignment is 8
/// (measured: `sizeof (max_align_t)` 8, `_Alignof (max_align_t)` 8), so the port spells it
/// `u64` — the `scratch_buffer.rs` mirror's row for the same type says the same.
pub type max_align_t = u64;

/// `<mach/mach_time.h>`'s `mach_timebase_info_data_t`, measured: `sizeof` 8, `numer`@0,
/// `denom`@4 (C 4386's live `#if HAVE_RUSAGE_INFO_CURRENT` arm uses it).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mach_timebase_info_data_t {
  pub numer: u32,
  pub denom: u32,
}

const _: () = core::assert! (std::mem::size_of::<mach_timebase_info_data_t> () == 8);
const _: () = core::assert! (std::mem::offset_of! (mach_timebase_info_data_t, denom) == 4);

/* <libproc.h> and <sys/resource.h>: the two Darwin rusage records C 4378-4473 reaches.
   `rusage_info_current` is `struct rusage_info_v6` under Darwin's `RUSAGE_INFO_CURRENT`
   (measured 6).  Only the three members the arm reads are named; the rest of the record is
   the measured tail.  */
/// `<sys/resource.h>`'s `rusage_info_v6`, measured: `sizeof` 464, `ri_user_time`@16,
/// `ri_system_time`@24, `ri_pageins`@48.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct rusage_info_current {
  pub _pad0: [u64; 2],
  pub ri_user_time: u64,
  pub ri_system_time: u64,
  pub _pad1: [u64; 2],
  pub ri_pageins: u64,
  pub _pad2: [u64; 51],
}

const _: () = core::assert! (std::mem::size_of::<rusage_info_current> () == 464);
const _: () = core::assert! (std::mem::offset_of! (rusage_info_current, ri_user_time) == 16);
const _: () = core::assert! (std::mem::offset_of! (rusage_info_current, ri_system_time) == 24);
const _: () = core::assert! (std::mem::offset_of! (rusage_info_current, ri_pageins) == 48);

/// `<libproc.h>`'s `struct proc_taskinfo`, measured: `sizeof` 96, `pti_virtual_size`@0,
/// `pti_resident_size`@8, `pti_threadnum`@84.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct proc_taskinfo {
  pub pti_virtual_size: u64,
  pub pti_resident_size: u64,
  pub _pad0: [u8; 68],
  pub pti_threadnum: c_int,
  pub _pad1: [u8; 8],
}

const _: () = core::assert! (std::mem::size_of::<proc_taskinfo> () == 96);
const _: () = core::assert! (std::mem::offset_of! (proc_taskinfo, pti_virtual_size) == 0);
const _: () = core::assert! (std::mem::offset_of! (proc_taskinfo, pti_resident_size) == 8);
const _: () = core::assert! (std::mem::offset_of! (proc_taskinfo, pti_threadnum) == 84);

/* <sys/sysctl.h>: Darwin's `struct kinfo_proc`, whose `kp_proc` (a `struct extern_proc`)
   and `kp_eproc` (a `struct eproc`) the live Darwin arm of `system_process_attributes`
   reads.  Only the members that arm reaches are named; each is pinned at its measured
   offset, and the padding between them reproduces the measured record exactly (the
   FreeBSD/OpenBSD arms' own shapes are declared inside their `#[cfg]` blocks below).  */
/// `<sys/sysctl.h>`'s `struct extern_proc`, measured: `sizeof` 296, `p_un`@0 (the
/// `__p_starttime` arm, C 4443's `proc.kp_proc.p_starttime`), `p_stat`@36, `p_pid`@40,
/// `p_nice`@242, `p_comm`@243 (a `char[17]`).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct extern_proc {
  pub p_starttime: timeval,
  pub _pad0: [u8; 20],
  pub p_stat: c_int,
  pub p_pid: pid_t,
  pub _pad1: [u8; 198],
  pub p_nice: c_char,
  pub p_comm: [c_char; 17],
  pub _pad2: [u8; 36],
}

const _: () = core::assert! (std::mem::size_of::<extern_proc> () == 296);
const _: () = core::assert! (std::mem::offset_of! (extern_proc, p_starttime) == 0);
const _: () = core::assert! (std::mem::offset_of! (extern_proc, p_stat) == 36);
const _: () = core::assert! (std::mem::offset_of! (extern_proc, p_pid) == 40);
const _: () = core::assert! (std::mem::offset_of! (extern_proc, p_nice) == 242);
const _: () = core::assert! (std::mem::offset_of! (extern_proc, p_comm) == 243);
const _: () = core::assert! (std::mem::size_of::<[c_char; 17]> () == 17);

/// `<sys/proc.h>`'s `struct _pcred`, measured: `sizeof` 104, `p_svgid`@92.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pcred {
  pub _pad0: [u8; 92],
  pub p_svgid: uid_t,
  pub _pad1: [u8; 8],
}

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<pcred> () == 104);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (pcred, p_svgid) == 92);

/// `<sys/proc.h>`'s `struct _ucred`, measured: `sizeof` 76, `cr_uid`@4, `cr_groups`@12
/// (`gid_t[16]`).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ucred {
  pub _pad0: [u8; 4],
  pub cr_uid: uid_t,
  pub cr_ngroups: c_short,
  pub _pad1: [u8; 2],
  pub cr_groups: [gid_t; 16],
}

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<ucred> () == 76);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (ucred, cr_uid) == 4);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (ucred, cr_groups) == 12);

/// `<sys/proc.h>`'s `struct eproc`, measured: `sizeof` 352, `e_pcred`@16, `e_ucred`@120,
/// `e_ppid`@264, `e_pgid`@268, `e_tdev`@276, `e_tpgid`@280.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct eproc {
  pub _pad0: [u8; 16],
  pub e_pcred: pcred,
  pub e_ucred: ucred,
  pub _pad1: [u8; 68],
  pub e_ppid: pid_t,
  pub e_pgid: pid_t,
  pub _pad2: [u8; 4],
  pub e_tdev: dev_t,
  pub e_tpgid: pid_t,
  pub _pad3: [u8; 68],
}

#[cfg (not (feature = "have_android"))]
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<eproc> () == 352);
#[cfg (not (feature = "have_android"))]
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (eproc, e_pcred) == 16);
#[cfg (not (feature = "have_android"))]
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (eproc, e_ucred) == 120);
#[cfg (not (feature = "have_android"))]
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (eproc, e_ppid) == 264);
#[cfg (not (feature = "have_android"))]
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (eproc, e_pgid) == 268);
#[cfg (not (feature = "have_android"))]
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (eproc, e_tdev) == 276);
#[cfg (not (feature = "have_android"))]
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (eproc, e_tpgid) == 280);

/// `<sys/sysctl.h>`'s `struct kinfo_proc`, measured: `sizeof` 648, `kp_proc`@0,
/// `kp_eproc`@296.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct kinfo_proc {
  pub kp_proc: extern_proc,
  pub kp_eproc: eproc,
}

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<kinfo_proc> () == 648);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (kinfo_proc, kp_proc) == 0);
#[cfg (not (feature = "have_android"))]
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (kinfo_proc, kp_eproc) == 296);
#[cfg (not (feature = "have_android"))]
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (kinfo_proc, kp_eproc.e_ucred.cr_uid) == 420);
#[cfg (not (feature = "have_android"))]
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (kinfo_proc, kp_eproc.e_pcred.p_svgid) == 404);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (kinfo_proc, kp_proc.p_comm) == 243);

/// `<sys/param.h>`: `#define MAXPATHLEN 1024`, `#define NODEV ((dev_t) -1)` (measured).
pub const MAXPATHLEN: usize = 1024;
/// `<sys/sysctl.h>`: `#define CTL_KERN 1`, `KERN_PROC 14`, `KERN_PROC_PID 1`,
/// `KERN_PROC_ALL 0`, `KERN_PROCARGS2 49`, `HW_MEMSIZE 24`, `HW_NCPU 3` (measured; the
/// `sys_sysctl.rs` mirror carries only `CTL_HW`/`HW_NCPU`/`sysctl`).
pub const CTL_KERN: c_int = 1;
pub const KERN_PROC: c_int = 14;
pub const KERN_PROC_PID: c_int = 1;
pub const KERN_PROC_ALL: c_int = 0;
/// `<sys/sysctl.h>`: `KERN_PROC_PROC` (FreeBSD's variant of `KERN_PROC_ALL`).
pub const KERN_PROC_PROC: c_int = 2;
pub const KERN_PROCARGS2: c_int = 49;
pub const HW_MEMSIZE: c_int = 24;
/// `<sys/resource.h>`: `#define RUSAGE_SELF 0` (measured).
pub const RUSAGE_SELF: c_int = 0;
/// `<libproc.h>`: `#define PROC_PIDPATHINFO_MAXSIZE 4096`, `#define PROC_PIDTASKINFO 4`
/// (measured), and `<sys/resource.h>`'s `#define RUSAGE_INFO_CURRENT RUSAGE_INFO_V6` (6).
pub const PROC_PIDPATHINFO_MAXSIZE: c_int = 4096;
pub const PROC_PIDTASKINFO: c_int = 4;
pub const RUSAGE_INFO_CURRENT: c_int = 6;
/// `<sys/param.h>`'s `#define NODEV ((dev_t) -1)` (measured: `dev_t` is a 4-byte unsigned on the
/// host and an 8-byte one on bionic, so the constant is spelled as C's all-ones conversion).
pub const NODEV: dev_t = (-1i64) as dev_t;   /* C's `((dev_t) -1)` */

/// `<signal.h>`: `#define SA_RESTART 0x0002` (measured).
pub const SA_RESTART: c_int = 0x0002;
/// `<signal.h>`: `SA_ONSTACK 0x0001`, `SA_SIGINFO 0x0040` (measured; `SA_RESTART` is 2 and
/// `signal.rs`'s mirror does not carry the two the file needs — `syssignal.h`'s mirror
/// carries `SA_SIGINFO`?  it does not, so they are declared here).
pub const SA_ONSTACK: c_int = 1;
pub const SA_SIGINFO: c_int = 0x40;
/// `<signal.h>`: `SIG_DFL`'s and `SIG_IGN`'s handler words (`signal.rs` carries `SIG_IGN`).
pub const SIG_ERR: usize = !0usize;

/// `"termopts.h"`'s two objects (no mirror): `extern bool interrupt_input;`
/// `extern bool interrupts_deferred;`.  C's own comment for the pair is at C 547-548's
/// neighbours in the header; `sysdep.c` is their owner for `interrupts_deferred` (C 790) and
/// keyboard.c's for `interrupt_input`, so only the latter is declared here.
unsafe extern "C" {
  pub static mut interrupt_input: bool;
}

/* `"sysstdio.h"`'s three entries (C 60-62: `errputc`, `errwrite`, `close_output_streams`)
   are *defined* in this unit, so C's declarations for them have no counterpart here
   (PORTING.md 0.1-12; the units that call them declare them at their own use site).  */

/// `<stdio.h>`: the entries `"sysstdio.h"` and this unit reach but `stdio.rs` does not carry
/// (`stdio.rs` has `fclose`/`feof`/`ferror`/`fread`/`fwrite`/`fputs`/`fputc`/`sprintf`/`EOF`).
unsafe extern "C" {
  /* C 2914-2943's `fflush_unlocked`/`fputc_unlocked`/`fwrite_unlocked` are not entries of
     `<stdio.h>`: they are `lib/unlocked-io.h`'s macros for this target, whose live arm maps
     each of them onto the ordinary entry (`lib/include/unlocked-io.rs`, measured).  The call
     sites below invoke those macros.  The three declarations that used to stand here were this
     unit's own invention - no C object references such a symbol (`nm -u
     build/emacs-ref/src/sysdep.o` names `_fflush`, `_fputc`, `_fwrite`), and the real link
     refused them: `_fflush_unlocked` is in no library of this target (docs/ports/link.md's
     reading three).  */
  /* int setvbuf (FILE *, char *, int, size_t);  */
  pub fn setvbuf (stream: *mut FILE, buf: *mut c_char, mode: c_int, size: usize) -> c_int;
  /* FILE *fdopen (int, const char *);  */
  pub fn fdopen (fd: c_int, mode: *const c_char) -> *mut FILE;
  /* int fscanf (FILE *, const char *, ...);  */
  pub fn fscanf (stream: *mut FILE, format: *const c_char, ...) -> c_int;
  /* int sscanf (const char *, const char *, ...);  */
  pub fn sscanf (s: *const c_char, format: *const c_char, ...) -> c_int;
  /* `int snprintf (char *, size_t, const char *, ...)` comes from `lib/include/stdio.rs`
     (status 3.263): the mirror carries it, so this block does not (0.1-3).  */
  /* char *strerror (int);  */
}

/// C 81-83's `#ifndef STDERR_FILENO` / `# define STDERR_FILENO fileno (GetStdHandle
/// (STD_ERROR_HANDLE))` — the dead `WINDOWSNT` arm, carried with the name the port's const
/// above carries; the macro body is C's own text (the guard is what makes the two one name
/// in C, and `cfg` is what makes them one name here).
#[cfg (feature = "no_stderr_fileno")]
macro_rules! STDERR_FILENO {
  () => { fileno (GetStdHandle (STD_ERROR_HANDLE)) };
}

/// `<stdio.h>`: `#define BUFSIZ 1024`, `_IOFBF 0`, `_IOLBF 1` (measured).
pub const BUFSIZ: usize = 1024;
pub const _IOFBF: c_int = 0;
pub const _IOLBF: c_int = 1;

/* `<unistd.h>`'s `STDIN_FILENO`/`STDOUT_FILENO`/`STDERR_FILENO` now come from the
   unistd.in.rs mirror (2026-10-04); this unit's use-site copies are gone, and the mirror is
   the single declaration point (PORTING.md 0.1-3/P-26).  C 81-83's `#define STDERR_FILENO
   fileno (GetStdHandle (STD_ERROR_HANDLE))` is the dead `WINDOWSNT` arm and is carried with
   the rest of that arm below.  */

/// `<unistd.h>`: the entries this unit reaches that `unistd.in.rs` does not carry.
unsafe extern "C" {
  /* void _exit (int);  */
  pub fn _exit (status: c_int) -> !;
  /* int execve (const char *, char *const[], char *const[]);  */
  pub fn execve (path: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char)
    -> c_int;
  /* int execvp (const char *, char *const[]);  */
  pub fn execvp (file: *const c_char, argv: *const *mut c_char) -> c_int;
  /* int execlp (const char *, const char *, ...);  */
  pub fn execlp (file: *const c_char, arg: *const c_char, ...) -> c_int;
  /* pid_t getpgrp (void);  */
  pub fn getpgrp () -> pid_t;
  /* int setpgid (pid_t, pid_t);  */
  pub fn setpgid (pid: pid_t, pgid: pid_t) -> c_int;
  /* pid_t tcsetpgrp (int, pid_t);  */
  pub fn tcsetpgrp (fd: c_int, pgrp_id: pid_t) -> c_int;
  /* int tcgetattr (int, struct termios *);  */
  pub fn tcgetattr (fd: c_int, t: *mut termios) -> c_int;
  /* int tcsetattr (int, int, const struct termios *);  */
  pub fn tcsetattr (fd: c_int, actions: c_int, t: *const termios) -> c_int;
  /* int tcflow (int, int);  */
  pub fn tcflow (fd: c_int, action: c_int) -> c_int;
  /* int tcdrain (int);  */
  pub fn tcdrain (fd: c_int) -> c_int;
  /* speed_t cfgetospeed (const struct termios *);  */
  pub fn cfgetospeed (t: *const termios) -> speed_t;
  /* int cfsetispeed (struct termios *, speed_t);  */
  pub fn cfsetispeed (t: *mut termios, speed: speed_t) -> c_int;
  /* int cfsetospeed (struct termios *, speed_t);  */
  pub fn cfsetospeed (t: *mut termios, speed: speed_t) -> c_int;
  /* int ioctl (int, unsigned long, ...);  */
  pub fn ioctl (fd: c_int, request: c_ulong, ...) -> c_int;
  /* int sigaltstack (const stack_t *, stack_t *);  */
  pub fn sigaltstack (ss: *const stack_t, oss: *mut stack_t) -> c_int;
  /* int sigsuspend (const sigset_t *);  */
  pub fn sigsuspend (mask: *const sigset_t) -> c_int;
  /* void siglongjmp (sigjmp_buf, int);  */
  pub fn siglongjmp (env: *mut sigjmp_buf, val: c_int) -> !;
  /* char const *sigdescr_np (int);  */
  pub fn sigdescr_np (sig: c_int) -> *const c_char;
}

/// `<unistd.h>`'s `sigfillset` is a *macro* on this target (`#define sigfillset(set)
/// (*(set) = ~0, 0)`), like `signal.rs`'s `sigemptyset`/`sigaddset`; the call site keeps
/// C's spelling.
#[inline(always)]
pub unsafe fn sigfillset (set: *mut sigset_t) -> c_int {
  *set = sigset_t { __val: !0 };
  0
}

/// `"syswait.h"`: `#define WNOHANG 1` and the status decoders (measured; C 517's
/// `WNOHANG | options` is the only use).
pub const WNOHANG: c_int = 1;

/// `"termopts.h"` and `<sys/param.h>`: `#define PATH_MAX 1024`, `#define PIPE_BUF 512`
/// (measured; `<limits.h>`'s `PIPE_BUF`).
pub const PIPE_BUF: usize = 512;

/// `<pwd.h>`/`<grp.h>`: the two lookups C 3590/3600 calls.
unsafe extern "C" {
}

/// `<sys/utsname.h>`: C 1620's dead `HAVE_GETHOSTNAME` arm.

/// `<stdlib.h>`/`<unistd.h>`: the two memory-entropy and the clock entries this unit calls.
unsafe extern "C" {
  /* long random (void); -- the frozen configuration's `HAVE_RANDOM` arm is `srandom`.  */
  pub fn srandom (seed: c_uint);
  /* long lrand48 (void);  */
  pub fn srand48 (seed: c_long);
  /* void srand (unsigned int);  */
  pub fn srand (seed: c_uint);
  /* long lrand48 (void);  */
  pub fn lrand48 () -> c_long;
  /* int rand (void);  */
  pub fn rand () -> c_int;
  /* void qsort (void *, size_t, size_t, int (*) (const void *, const void *));  */
  pub fn getloadavg (loadavg: *mut f64, nelem: c_int) -> c_int;
}

/// `"sysstdio.h"`'s and `<stdio.h>`'s `PIPE_BUF` fallback is `MAX_ALLOCA` where the header
/// has no `PIPE_BUF`; the frozen target has one (measured 512), so C 2884's
/// `char buf[min (PIPE_BUF, MAX_ALLOCA)]` is the 512-byte buffer.

/// `<sys/sysctl.h>`/`<libproc.h>`: the four Darwin process entries C 4286-4470 calls.
unsafe extern "C" {
  /* int sysctlbyname (const char *, void *, size_t *, void *, size_t);  */
  pub fn sysctlbyname (name: *const c_char, oldp: *mut c_void, oldlenp: *mut usize,
                       newp: *mut c_void, newlen: usize) -> c_int;
  /* int proc_pidpath (int, void *, uint32_t);  */
  pub fn proc_pidpath (pid: c_int, buffer: *mut c_void, buffersize: u32) -> c_int;
  /* int proc_pidinfo (int, int, uint64_t, void *, int);  */
  pub fn proc_pidinfo (pid: c_int, flavor: c_int, arg: u64, buffer: *mut c_void, buffersize: c_int)
    -> c_int;
  /* int proc_pid_rusage (int, int, rusage_info_t *);  */
  pub fn proc_pid_rusage (pid: c_int, flavor: c_int, buffer: *mut c_void) -> c_int;
  /* void mach_timebase_info (mach_timebase_info_t);  */
  pub fn mach_timebase_info (info: *mut mach_timebase_info_data_t) -> c_int;
  /* char *devname (dev_t, mode_t);  */
  pub fn devname (dev: dev_t, type_: mode_t) -> *mut c_char;
  /* int getrusage (int, struct rusage *);  */
  pub fn getrusage (who: c_int, usage: *mut rusage) -> c_int;
}

/// `<execinfo.h>`: the two backtrace entries C 2311-2331 calls.
unsafe extern "C" {
  /* int backtrace (void **, int);  */
  pub fn backtrace (buffer: *mut *mut c_void, size: c_int) -> c_int;
  /* void backtrace_symbols_fd (void *const *, int, int);  */
  pub fn backtrace_symbols_fd (buffer: *const *mut c_void, size: c_int, fd: c_int);
}

/// `<pthread.h>`: the four thread entries the `FORWARD_SIGNAL_TO_MAIN_THREAD` arms call
/// (`signal.rs` carries only `pthread_sigmask`).
unsafe extern "C" {
  /* pthread_t pthread_self (void);  */
  pub fn pthread_self () -> pthread_t;
  /* int pthread_equal (pthread_t, pthread_t);  */
  pub fn pthread_equal (t1: pthread_t, t2: pthread_t) -> c_int;
  /* int pthread_kill (pthread_t, int);  */
  pub fn pthread_kill (thread: pthread_t, sig: c_int) -> c_int;
}

/// `<pthread.h>`, measured: Darwin's `pthread_t` is a pointer to `struct _opaque_pthread_t`.
#[cfg (not (all (feature = "threads_enabled", feature = "have_pthread")))]
pub type pthread_t = *mut c_void;

/* `"systty.h"`: `#define CDISABLE _POSIX_VDISABLE` (255 on this target) now comes from
   `include/systty.rs` (C 106).  */

/// `<locale.h>`: the two categories C 4633/4646 passes to `emacs_setlocale`.
pub const LC_COLLATE: c_int = 2;
pub const LC_CTYPE: c_int = 0;

/// `<locale.h>`: the two entries C 4635/4642 calls.
unsafe extern "C" {
  /* char *setlocale (int, const char *);  */
  pub fn setlocale (category: c_int, locale: *const c_char) -> *mut c_char;
}

/// `<wchar.h>`/`<wctype.h>`: the two collation entries (the `_l` forms and the constants are
/// in the `__STDC_ISO_10646__` arm below, which the frozen build does not compile).
unsafe extern "C" {
  /* int wcscoll (const wchar_t *, const wchar_t *);  */
  pub fn wcscoll (a: *const wchar_t, b: *const wchar_t) -> c_int;
}

/// cm.h 163's `emacs_tputs` assigns `current_tty`, which `src/cm.c` owns: the mirror keeps
/// the object in `cm_globals` because `cm.rs` defines it at its own crate root (the P-21
/// shape), so a call site imports the name the way C's `cm.h` declaration makes it visible.
use cm_globals::current_tty;

/// `src/emacs.c`'s `emacs_strerror` (its mirror lands with that unit).
unsafe extern "C" { pub fn emacs_strerror (error_num: c_int) -> *const c_char; }

/// `<unistd.h>`: `int gethostname (char *, size_t);` (`unistd.in.rs` does not carry it).
unsafe extern "C" { pub fn gethostname (name: *mut c_char, len: usize) -> c_int; }

/// `src/xfaces.c`/`src/dispnew.c` and `src/terminal.c`'s entries the live arms reach
/// (`init_frame_faces`, `is_tty_frame`, `tty_turn_off_insert`); their mirrors land with those
/// units, so the port declares them at their use site with C's own prototype.
unsafe extern "C" {
  /* void init_frame_faces (struct frame *);  */
  pub fn init_frame_faces (f: *mut frame);
  /* bool is_tty_frame (struct frame *);  */
  pub fn is_tty_frame (f: *mut frame) -> bool;
  /* void tty_turn_off_insert (struct tty_display_info *);  */
  pub fn tty_turn_off_insert (tty: *mut tty_display_info);
}
/* `cursorX` is **not** declared here: `termhooks.h` 979-984 declares the function only under
   `WINDOWSNT` and otherwise `#define`s it to `curX` (cm.h), so on the frozen target C's
   `sysdep.c` 1526 compiles to `curX (tty_out)` and the reference object has no `_cursorX` at
   all - while the port's extern made it ask for one (found by `xtask gate-load`, status 3.114,
   the `MOUSE_HL_INFO` class from 3.109).  The call site uses `cursorX!`, the mirror's macro.  */

/// `src/keyboard.c`/`src/emacs.c`/`src/alloc.c` and `src/sfntfont.c`'s entries the signal
/// arms reach; their mirrors land with those units, so the port declares them at their use
/// site with C's own prototype.
unsafe extern "C" {
  /* void add_user_signal (int, const char *);  */
  pub fn add_user_signal (sig: c_int, name: *const c_char);
  /* _Noreturn void terminate_due_to_signal (int, int);  */
  pub fn terminate_due_to_signal (sig: c_int, backtrace_limit: c_int) -> !;
  /* void malloc_warning (const char *);  */
  pub fn malloc_warning (message: *const c_char);
  /* void force_auto_save_soon (void);  */

  /* bool sfntfont_detect_sigbus (void *);  */
  pub fn sfntfont_detect_sigbus (addr: *mut c_void) -> bool;
  /* void internal_terminal_init (void);  */
  pub fn internal_terminal_init ();
  /* void dos_ttraw (struct tty_display_info *);  */
  pub fn dos_ttraw (tty: *mut tty_display_info);
  /* void dos_ttcooked (void);  */
  pub fn dos_ttcooked ();
  /* void init_frame_faces (struct frame *);  */
}

/// `<signal.h>`: the platform-specific signals C 1690-2163 names and this target does not
/// define (measured: every one of them is absent), so those arms are dead and carried.  The
/// values are the vendors' (AIX and the System V ports); the arms are never compiled, which
/// is why the port may keep them the way the timefns.rs `CLOCK_MONOTONIC_COARSE` row does.
#[cfg (feature = "sigdanger")]
pub const SIGDANGER: c_int = 33;
#[cfg (feature = "sigpre")]
pub const SIGPRE: c_int = 36;
#[cfg (feature = "sigore")]
pub const SIGORE: c_int = 37;
#[cfg (feature = "sigume")]
pub const SIGUME: c_int = 38;
#[cfg (feature = "sigdlk")]
pub const SIGDLK: c_int = 39;
#[cfg (feature = "sigcpulim")]
pub const SIGCPULIM: c_int = 40;
#[cfg (feature = "siggrant")]
pub const SIGGRANT: c_int = 34;
#[cfg (feature = "sigmigrate")]
pub const SIGMIGRATE: c_int = 41;
#[cfg (feature = "sigmsg")]
pub const SIGMSG: c_int = 42;
#[cfg (feature = "sigretract")]
pub const SIGRETRACT: c_int = 35;
#[cfg (feature = "sigsak")]
pub const SIGSAK: c_int = 43;
#[cfg (feature = "sigsound")]
pub const SIGSOUND: c_int = 44;
#[cfg (feature = "sig_talrm")]
pub const SIGTALRM: c_int = 45;

/// frame.h 1477's `FRAME_GARBAGED_P (f)` is C's *lvalue* bit macro: the mirror reads it
/// through the function of that name, and a Rust function is not a place, so this one write
/// (C 1352) goes through the macro below, which touches the same bit of unit 1 that
/// `frame.rs`'s reader reads (the `wset_redisplay`/`fset_redisplay` shape status 3.53
/// records for window.h).  The C name stays in the port's text for the token check.
macro_rules! FRAME_GARBAGED_P {
  ($f:expr, $v:expr) => {
    (*($f)).bits1 = ((*($f)).bits1 & !(1u32 << FRAME_GARBAGED_SHIFT))
	| (((($v) as c_int) as u32) << FRAME_GARBAGED_SHIFT)
  };
}

/// `<Availability.h>`: `#define __MAC_OS_X_VERSION_MIN_REQUIRED __ENVIRONMENT_OS_VERSION_MIN_REQUIRED__`,
/// measured as 260000 with the frozen clang (the SDK's deployment target for this machine).
pub const __MAC_OS_X_VERSION_MIN_REQUIRED: c_int = 260000;

/// `"sys/param.h"`'s `# define frame freebsd_frame` (C 66, the dead `__FreeBSD__` include
/// arm): the port keeps the macro name and the name it denotes (PORTING.md 10.13's row for a
/// rename).
#[cfg (feature = "freebsd")]
pub const freebsd_frame: () = ();
#[cfg (feature = "freebsd")]
macro_rules! frame { () => { freebsd_frame } }

/// C 3808's `#define _FILE_OFFSET_BITS 64` (the dead `SOLARIS2` arm's re-enable).
#[cfg (feature = "solaris2")]
pub const _FILE_OFFSET_BITS: c_int = 64;

/// `<config.h>`: `#define NULL_DEVICE "/dev/null"` (the frozen macro table).
const NULL_DEVICE: &str = "/dev/null";

/// `<config.h>`: `#define DIRECTORY_SEP '/'`, and the two predicates the frozen
/// `config.h` defines as function-like macros (`DRAFT`'s `IS_DEVICE_SEP(c)` is `0` on this
/// target; config.rs does not carry them, so the port keeps C's macros, P-37).
const DIRECTORY_SEP: c_char = b'/' as c_char;
macro_rules! IS_DIRECTORY_SEP { ($c:expr) => { $c == DIRECTORY_SEP }; }
macro_rules! IS_DEVICE_SEP { ($c:expr) => { false }; }

/// `<string.h>`: the three entries `string.rs` does not carry and this unit reaches.
unsafe extern "C" {
  /* char *strdup (const char *);  */
  pub fn strdup (s: *const c_char) -> *mut c_char;
  /* size_t strnlen (const char *, size_t);  */
  pub fn strnlen (s: *const c_char, maxlen: usize) -> usize;
  /* int strcasecmp (const char *, const char *);  */
  pub fn strcasecmp (s1: *const c_char, s2: *const c_char) -> c_int;
}

/// `"android.h"`'s wrappers the carried `HAVE_ANDROID` arms name (the header has no mirror;
/// the arms are never compiled, P-51).
#[cfg (feature = "have_android")]
unsafe extern "C" {
}

/// `<stdlib.h>`: `long random (void);` — the frozen configuration's `HAVE_RANDOM` arm
/// (C 2248/2264/2272's `random ()` calls are this entry).  The declaration is on its own
/// line so that the extractor's `fn NAME` scan sees the name (a one-line `unsafe extern`
/// block hides it).
unsafe extern "C" {
  pub fn random () -> c_long;
}

/// `<stdio.h>`: `int rename (const char *, const char *);` (`stdio.rs` does not carry it).
unsafe extern "C" { pub fn rename (old: *const c_char, new: *const c_char) -> c_int; }

/// `<config.h>`'s `ADDRESS_SANITIZER` (`conf_post.h` C 89-94) now comes from the
/// `include/conf_post.rs` mirror, which spells the frozen `false` - the value that makes
/// C 2971's ternary take its `close_stream` arm in the `close_output_streams` body below.

/// `<stdlib.h>`: `void exit (int);` — `force_open` is the only caller.  `EXIT_FAILURE` itself
/// now comes from the `stdlib.rs` mirror (which carried only `malloc` when this unit was
/// written), and `perror`/`printf` from the `stdio.rs` mirror the same way (2026-10-04).
unsafe extern "C" { pub fn exit (status: c_int) -> !; }

/* ==========================================================================================
   `<termios.h>`, `<sys/ioctl.h>`, `<sys/param.h>` and `<signal.h>`: the names this unit
   reaches that no mirror carries.  All values are measured with the frozen clang (the same
   command `docs/ports/sysdep.md` records); `_PC_PIPE_BUF` and the signals come from
   `docs/baseline/config/darwin-arm64.macros.tsv`, which already knows them.  */

/// `<termios.h>`'s input flags (measured): IGNBRK 1, BRKINT 2, IGNPAR 4, PARMRK 8, INPCK 16,
/// ISTRIP 32, INLCR 64, IGNCR 128, ICRNL 256, IXON 512, IXOFF 1024, IXANY 2048.
pub const IGNBRK: tcflag_t = 0x00000001;
pub const BRKINT: tcflag_t = 0x00000002;
pub const IGNPAR: tcflag_t = 0x00000004;
pub const PARMRK: tcflag_t = 0x00000008;
pub const INPCK: tcflag_t = 0x00000010;
pub const ISTRIP: tcflag_t = 0x00000020;
pub const INLCR: tcflag_t = 0x00000040;
pub const IGNCR: tcflag_t = 0x00000080;
pub const ICRNL: tcflag_t = 0x00000100;
pub const IXON: tcflag_t = 0x00000200;
pub const IXOFF: tcflag_t = 0x00000400;
pub const IXANY: tcflag_t = 0x00000800;
/// `<termios.h>`: `IUCLC` and `OLCUC` do **not** exist on this target (measured: undeclared),
/// so C 549-560's arms are dead; the port keeps the two names with the Linux values
/// (the timefns.rs `CLOCK_MONOTONIC_COARSE` shape) inside the arm that is carried.
#[cfg (feature = "iuclc")]
pub const IUCLC: tcflag_t = 0x00000100;
#[cfg (feature = "olcuc")]
pub const OLCUC: tcflag_t = 0x00000002;

/// `<termios.h>`'s output flags (measured): OPOST 1, ONLCR 2, TABDLY 3076 (0x00000c04,
/// whose low bit is OXTABS), TAB3 4 (OXTABS), NLDLY 0x00000300, CRDLY 0x00003000, BSDLY
/// 0x00008000, VTDLY 0x00010000, FFDLY 0x00004000.
pub const OPOST: tcflag_t = 0x00000001;
pub const ONLCR: tcflag_t = 0x00000002;
pub const TABDLY: tcflag_t = 0x00000c04;
pub const TAB3: tcflag_t = 0x00000004;
pub const OXTABS: tcflag_t = 0x00000004;
pub const NLDLY: tcflag_t = 0x00000300;
pub const CRDLY: tcflag_t = 0x00003000;
pub const BSDLY: tcflag_t = 0x00008000;
pub const VTDLY: tcflag_t = 0x00010000;
pub const FFDLY: tcflag_t = 0x00004000;
/// `<termios.h>`: `#define TAB0 0` (measured; the `speeds`-style names the file reaches).
pub const TAB0: tcflag_t = 0;

/// `<termios.h>`'s control flags (measured): CSIZE 0x00000300, CS5 0, CS6 0x00000100,
/// CS7 0x00000200, CS8 0x00000300, CSTOPB 0x00000400, CREAD 0x00000800, PARENB 0x00001000,
/// PARODD 0x00002000, HUPCL 0x00004000, CLOCAL 0x00008000, CCTS_OFLOW 0x00010000,
/// CRTS_IFLOW 0x00020000, CRTSCTS 0x00030000.
pub const CSIZE: tcflag_t = 0x00000300;
pub const CS5: tcflag_t = 0x00000000;
pub const CS6: tcflag_t = 0x00000100;
pub const CS7: tcflag_t = 0x00000200;
pub const CS8: tcflag_t = 0x00000300;
pub const CSTOPB: tcflag_t = 0x00000400;
pub const CREAD: tcflag_t = 0x00000800;
pub const PARENB: tcflag_t = 0x00001000;
pub const PARODD: tcflag_t = 0x00002000;
pub const HUPCL: tcflag_t = 0x00004000;
pub const CLOCAL: tcflag_t = 0x00008000;
pub const CCTS_OFLOW: tcflag_t = 0x00010000;
pub const CRTS_IFLOW: tcflag_t = 0x00020000;
pub const CRTSCTS: tcflag_t = CCTS_OFLOW | CRTS_IFLOW;
/// `<termios.h>`: `CNEW_RTSCTS` does not exist on this target, so C 3272-3288's arm is dead
/// and carried with the BSD spelling of the same flag (the arm is never compiled).
#[cfg (feature = "cnew_rtscts")]
pub const CNEW_RTSCTS: tcflag_t = CCTS_OFLOW | CRTS_IFLOW;

/// `<termios.h>`'s local flags (measured): ECHOKE 1, ECHOE 2, ECHOK 4, ECHO 8, ECHONL 16,
/// ECHOPRT 32, ECHOCTL 64, ISIG 128, ICANON 256, ALTWERASE 512, IEXTEN 1024, EXTPROC 2048,
/// TOSTOP 4194304, FLUSHO 8388608, PENDIN 536870912, NOFLSH 2147483648.
pub const ECHOKE: tcflag_t = 0x00000001;
pub const ECHOE: tcflag_t = 0x00000002;
pub const ECHOK: tcflag_t = 0x00000004;
pub const ECHO: tcflag_t = 0x00000008;
pub const ECHONL: tcflag_t = 0x00000010;
pub const ECHOPRT: tcflag_t = 0x00000020;
pub const ECHOCTL: tcflag_t = 0x00000040;
pub const ISIG: tcflag_t = 0x00000080;
pub const ICANON: tcflag_t = 0x00000100;
pub const ALTWERASE: tcflag_t = 0x00000200;
pub const IEXTEN: tcflag_t = 0x00000400;
pub const EXTPROC: tcflag_t = 0x00000800;
pub const TOSTOP: tcflag_t = 0x00400000;
pub const FLUSHO: tcflag_t = 0x00800000;
pub const PENDIN: tcflag_t = 0x20000000;
pub const NOFLSH: tcflag_t = 0x80000000;

/// `<termios.h>`'s `c_cc` indices (measured): VEOF 0, VEOL 1, VEOL2 2, VERASE 3, VWERASE 4,
/// VKILL 5, VREPRINT 6, VINTR 8, VQUIT 9, VSUSP 10, VDSUSP 11, VSTART 12, VSTOP 13,
/// VLNEXT 14, VDISCARD 15, VMIN 16, VTIME 17.
pub const VEOF: usize = 0;
pub const VEOL: usize = 1;
pub const VEOL2: usize = 2;
pub const VERASE: usize = 3;
pub const VWERASE: usize = 4;
pub const VKILL: usize = 5;
pub const VREPRINT: usize = 6;
pub const VINTR: usize = 8;
pub const VQUIT: usize = 9;
pub const VSUSP: usize = 10;
pub const VDSUSP: usize = 11;
pub const VSTART: usize = 12;
pub const VSTOP: usize = 13;
pub const VLNEXT: usize = 14;
pub const VDISCARD: usize = 15;
pub const VMIN: usize = 16;
pub const VTIME: usize = 17;
/// `<termios.h>`: `VSWTCH` and `V_DSUSP` are other systems' spellings (Solaris/BSD); the
/// frozen target has neither (measured), so C 1199-1210's arms are dead and carried.
#[cfg (feature = "vswtch")]
pub const VSWTCH: usize = 7;
#[cfg (feature = "v_dsusp")]
pub const V_DSUSP: usize = 11;

/// `<termios.h>`'s `tcsetattr` actions and `tcflow`'s actions (measured): TCSANOW 0,
/// TCSADRAIN 1, TCSAFLUSH 2, TCOON 2.
pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;
pub const TCOON: c_int = 2;

/// `<sys/ioctl.h>`'s request words (measured): TIOCSTI 2147578994 (0x80017472),
/// TIOCSTART 536900718 (0x2000746e), TIOCGWINSZ 1074295912 (0x40087468),
/// TIOCSWINSZ 2148037735 (0x80087467), TIOCEXCL 536900621 (0x2000740d),
/// FIONBIO 2147772030 (0x8004667e).
pub const TIOCSTI: c_ulong = 2147578994;
pub const TIOCSTART: c_ulong = 536900718;
pub const TIOCGWINSZ: c_ulong = 1074295912;
pub const TIOCSWINSZ: c_ulong = 2148037735;
pub const TIOCEXCL: c_ulong = 536900621;
pub const FIONBIO: c_ulong = 2147772030;
/// `<sys/ioctl.h>`: `TCXONC`, `TIOCSSIZE` and `TIOCGSIZE` do not exist on this target
/// (measured), so C 1281, 1412 and 1467's arms are dead and carried.  SunOS's `TIOCSSIZE`
/// value is 0x5477 and `TIOCGSIZE` is `TIOCGWINSZ` on the systems that have it (BSD);
/// `TCXONC` is `4` in `<termios.h>`'s function codes.
#[cfg (feature = "tcxonc")]
pub const TCXONC: c_ulong = 4;
#[cfg (feature = "tiocssize")]
pub const TIOCSSIZE: c_ulong = 0x5477;
#[cfg (feature = "tiocgsize")]
pub const TIOCGSIZE: c_ulong = TIOCGWINSZ;

/// `<unistd.h>`: `#define _PC_PIPE_BUF 6` (measured).
pub const _PC_PIPE_BUF: c_int = 6;

/* `struct save_signal` and the two helpers around it (C 531-538).  */
/* Record a signal code and the action for it.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct save_signal {
  pub code: c_int,
  pub action: sigaction,
}

/* `struct save_signal` is `{ int code; struct sigaction action; }`, so its size follows the
   target's `sigaction`: 8 + 16 = 24 where the frozen target's is 16 bytes, 8 + 32 = 40 where
   bionic's is 32 (measured, P-237).  */
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<save_signal> () == 24);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<save_signal> () == 40);
const _: () = core::assert! (std::mem::offset_of! (save_signal, code) == 0);
const _: () = core::assert! (std::mem::offset_of! (save_signal, action) == 8);

/// `<signal.h>`: `#define SIGPOLL SIGIO` on the systems that have the name (this target
/// leaves it undefined, measured), so C 681/1705/2112's `USABLE_SIGPOLL` arms are dead and
/// carried with the value `SIGIO` has here.
#[cfg (feature = "usable_sigpoll")]
pub const SIGPOLL: c_int = 23;

/// `<signal.h>`/`<sys/wait.h>`/`<unistd.h>`: the entries the live arms call and no mirror
/// carries.
unsafe extern "C" {
  /* int kill (pid_t, int);  */
  pub fn kill (pid: pid_t, sig: c_int) -> c_int;
  /* pid_t waitpid (pid_t, int *, int);  */
  pub fn waitpid (pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
  /* pid_t vfork (void);  -- conf_post.h 290's `VFORK ()` is `vfork ()` here.  */
  pub fn vfork () -> pid_t;
  /* char *get_current_directory (bool);  -- keyboard.c's entry (its mirror lands with it).  */

  /* int system (const char *);  */
  pub fn system (command: *const c_char) -> c_int;
  /* int setenv (const char *, const char *, int);  */
  pub fn setenv (name: *const c_char, value: *const c_char, replace: c_int) -> c_int;
  /* int putenv (char *);  */
  pub fn putenv (string: *mut c_char) -> c_int;
  /* int unsetenv (const char *);  */
  pub fn unsetenv (name: *const c_char) -> c_int;
}

/// `conf_post.h` 284-290's `VFORK ()` now comes from the `include/conf_post.rs` mirror too
/// (`DARWIN_OS` is defined, so the `_Pragma`-wrapped `vfork ()` arm is the live one; the
/// `fork` arm is the other configuration's).  `subshell` below keeps C's call spelling.

/// `"termopts.h"`'s second object: keyboard.c owns the storage, so the unit only declares it.
unsafe extern "C" { pub static mut interrupts_deferred: bool; }

/// `<fcntl.h>`: `#define FASYNC O_ASYNC` — the frozen target's `O_ASYNC` is 0x0040
/// (measured).
pub const FASYNC: c_int = 0x0040;

/// `<termios.h>`/`<stdlib.h>`: the four conversions and the list helpers the serial and
/// signal arms reach; their mirrors land with those units, so the port declares them at
/// their use site with C's own prototype (C 3010/1257's `cfmakeraw`/`cfsetspeed` are the
/// *system* entries here — `HAVE_CFMAKERAW`/`HAVE_CFSETSPEED` are 1, so the port's own
/// wrappers for them are the dead arms further down).
unsafe extern "C" {
  /* void cfmakeraw (struct termios *);  */
  pub fn cfmakeraw (termios_p: *mut termios);
  /* int cfsetspeed (struct termios *, speed_t);  */
  pub fn cfsetspeed (termios_p: *mut termios, vitesse: speed_t) -> c_int;
  /* Lisp_Object plist_get (Lisp_Object, Lisp_Object);  */
  /* Lisp_Object plist_member (Lisp_Object, Lisp_Object);  */

  /* Lisp_Object plist_put (Lisp_Object, Lisp_Object, Lisp_Object);  */
  pub fn plist_put (plist: Lisp_Object, prop: Lisp_Object, val: Lisp_Object) -> Lisp_Object;
  /* void process_pending_signals (void);  */

}

/// `<termios.h>`'s `Bnnn` speeds, measured on the frozen target (Darwin's `Bnnn` *is* the
/// rate): B0 0, B50 50, B75 75, B110 110, B134 134, B150 150, B200 200, B300 300, B600 600,
/// B1200 1200, B1800 1800, B2400 2400, B4800 4800, B9600 9600, B19200 19200, B38400 38400,
/// B57600 57600, B76800 76800, B115200 115200, B230400 230400.
pub const B0: speed_t = 0;
pub const B50: speed_t = 50;
pub const B75: speed_t = 75;
pub const B110: speed_t = 110;
pub const B134: speed_t = 134;
pub const B150: speed_t = 150;
pub const B200: speed_t = 200;
pub const B300: speed_t = 300;
pub const B600: speed_t = 600;
pub const B1200: speed_t = 1200;
pub const B1800: speed_t = 1800;
pub const B2400: speed_t = 2400;
pub const B4800: speed_t = 4800;
pub const B9600: speed_t = 9600;
pub const B19200: speed_t = 19200;
pub const B38400: speed_t = 38400;
pub const B57600: speed_t = 57600;
pub const B76800: speed_t = 76800;
pub const B115200: speed_t = 115200;
pub const B230400: speed_t = 230400;

/// `<termios.h>`: the speeds this target does *not* define (measured), so C 3087-3131's
/// `speeds[]` arms are dead and carried.  The value the port gives each name is the rate
/// itself — this target's own `Bnnn` convention — and the arm is never compiled.
pub const B153600: speed_t = 153600;
pub const B307200: speed_t = 307200;
pub const B460800: speed_t = 460800;
pub const B500000: speed_t = 500000;
pub const B576000: speed_t = 576000;
pub const B921600: speed_t = 921600;
pub const B1000000: speed_t = 1000000;
pub const B1152000: speed_t = 1152000;
pub const B1500000: speed_t = 1500000;
pub const B2000000: speed_t = 2000000;
pub const B2500000: speed_t = 2500000;
pub const B3000000: speed_t = 3000000;
pub const B3500000: speed_t = 3500000;
pub const B4000000: speed_t = 4000000;

/// `<termios.h>`: `CBAUD` is the baud-rate field mask on the systems whose `termios` has it
/// (this target does not define the name, measured), so C 563-585's `HPUX`/`AIX` arms are
/// dead and carried with the BSD value (0x00001f00; `CBAUDEX` is 0x00010000).
#[cfg (feature = "hpux")]
pub const CBAUD: tcflag_t = 0x00001f00;
#[cfg (feature = "aix")]
pub const CBAUD_AIX: tcflag_t = 0x00001f00;

/// `"msdos.h"`'s `dos_keyread` (C 371's dead `MSDOS` arm).
#[cfg (feature = "msdos")]
unsafe extern "C" { pub fn dos_keyread () -> c_int; }

/// `"w32.h"`'s entries C 651-742's dead `WINDOWSNT` arm reaches (`MAX_UTF8_PATH` is
/// `nt/inc/ms-w32.h` 156's `(MAXPATHLEN * 4)`).
#[cfg (feature = "windowsnt")]
pub const MAX_UTF8_PATH: usize = MAXPATHLEN * 4;
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type DWORD = u32;
#[cfg (feature = "windowsnt")]
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
#[cfg (feature = "windowsnt")]
pub const _P_WAIT: c_int = 0;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _get_osfhandle (fd: c_int) -> isize;
  pub fn GetConsoleMode (h: HANDLE, mode: *mut DWORD) -> c_int;
  pub fn SetConsoleMode (h: HANDLE, mode: DWORD) -> c_int;
  pub fn FlushConsoleInputBuffer (h: HANDLE) -> c_int;
  pub fn _spawnlp (mode: c_int, cmdname: *const c_char, arg0: *const c_char,
                   ...) -> isize;
}

/* ==========================================================================================
   C 146-2400's file-scope objects, in C's own order.  */

/// C 146-152: `static const int baud_convert[]`.
static baud_convert: [c_int; 16] =
  [0, 50, 75, 110, 135, 150, 200, 300, 600, 1200, 1800, 2400, 4800, 9600, 19200, 38400];

// C 153-204: the whole `HAVE_PERSONALITY_ADDR_NO_RANDOMIZE` region; the frozen configuration
// does not define the macro (macros.tsv 1794), so clang compiles nothing there and the port
// carries it (P-51).  C 154's `#include <sys/personality.h>` needs the Linux-only
// `personality`/`ADDR_NO_RANDOMIZE`, which the port declares at their use sites.
#[cfg (feature = "have_personality_addr_no_randomize")]
unsafe extern "C" {
  pub fn personality (persona: c_ulong) -> c_int;
}

/// `<sys/personality.h>`: `#define ADDR_NO_RANDOMIZE 0x0040000` (Linux's value; the arm that
/// reaches it is the carried `HAVE_PERSONALITY_ADDR_NO_RANDOMIZE` one).
#[cfg (feature = "have_personality_addr_no_randomize")]
pub const ADDR_NO_RANDOMIZE: c_int = 0x0040000;

/* If not -1, the personality that should be restored before exec.  */
#[cfg (feature = "have_personality_addr_no_randomize")]
static mut exec_personality: c_int = 0;

/// C 162-202's `maybe_disable_address_randomization`, carried: the frozen configuration
/// leaves `HAVE_PERSONALITY_ADDR_NO_RANDOMIZE` undefined, so C compiles the `#ifdef`'s arm
/// nowhere on this target (see the file header).
#[cfg (feature = "have_personality_addr_no_randomize")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn maybe_disable_address_randomization (argc: c_int,
                                                              argv: *mut *mut c_char) -> c_int {
  /* Undocumented Emacs option used only by this function.  */
  static aslr_disabled_option: [c_char; 19] = *b"--__aslr-disabled\0\0";

  if argc < 2 || strcmp (*argv.add (1), aslr_disabled_option.as_ptr ()) != 0
    {
      /* If pdumping, disabling ASLR lessens differences in the .pdmp file.  */
      let mut disable_aslr: bool = will_dump_p ();
      /* C's `#ifdef __PPC64__` arm (`disable_aslr = true;`) is dead on this target.  */
      #[cfg (feature = "ppc64")]
      { disable_aslr = true; }
      exec_personality =
        if disable_aslr { personality (0xffffffff) } else { -1 };
      if (exec_personality & ADDR_NO_RANDOMIZE) != 0
	{ exec_personality = -1; }
      if exec_personality != -1
	  && personality ((exec_personality | ADDR_NO_RANDOMIZE) as c_ulong) != -1
	{
	  let newargv: *mut *mut c_char =
	    malloc (((argc + 2) as usize) * std::mem::size_of::<*mut c_char> ())
	      as *mut *mut c_char;
	  if !newargv.is_null ()
	    {
	      /* Invoke self with undocumented option.  */
	      *newargv = *argv;
	      *newargv.add (1) = aslr_disabled_option.as_ptr () as *mut c_char;
	      memcpy (&mut *newargv.add (2) as *mut *mut c_char as *mut c_void,
		      &mut *argv.add (1) as *mut *mut c_char as *const c_void,
		      (argc as usize) * std::mem::size_of::<*mut c_char> ());
	      execvp (*newargv, newargv);
	    }

	  /* If malloc or execvp fails, warn and then try anyway.  */
	  perror (*argv);
	  free (newargv as *mut c_void);
	}
    }
  else
    {
      /* Our earlier incarnation already disabled ASLR.  */
      let argc = argc - 1;
      memmove (&mut *argv.add (1) as *mut *mut c_char as *mut c_void,
	       &mut *argv.add (2) as *mut *mut c_char as *const c_void,
	       (argc as usize) * std::mem::size_of::<*mut c_char> ());
    }

  return argc;
}

// C 206-223: `#ifndef WINDOWSNT` — live (`WINDOWSNT` is undefined).
/* Execute the program in FILE, with argument vector ARGV and environ
   ENVP.  Return an error number if unsuccessful.  This is like execve
   except it reenables ASLR in the executed program if necessary, and
   on error it returns an error number rather than -1.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_exec_file (file: *const c_char, argv: *const *mut c_char,
                                          envp: *const *mut c_char) -> c_int {
  #[cfg (feature = "have_personality_addr_no_randomize")]
  if exec_personality != -1
    { personality (exec_personality as c_ulong); }

  execve (file, argv, envp);
  return *errno ();
}

/* If FD is not already open, arrange for it to be open with FLAGS.  */
unsafe fn force_open (fd: c_int, flags: c_int) {
  if dup2 (fd, fd) < 0 && *errno () == EBADF
    {
      let n: c_int = open (NULL_DEVICE.as_ptr () as *const c_char, flags);
      if n < 0 || (fd != n && (dup2 (n, fd) < 0 || emacs_close (n) != 0))
	{
	  emacs_perror (NULL_DEVICE.as_ptr () as *const c_char);
	  exit (EXIT_FAILURE);
	}
    }
}

/* A stream that is like stderr, except line buffered.  It is NULL
   during startup, or if line buffering is not in use.  */
static mut buferr: *mut FILE = std::ptr::null_mut ();

/* Make sure stdin, stdout, and stderr are open to something, so that
   their file descriptors are not hijacked by later system calls.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_standard_fds () {
  /* Open stdin for *writing*, and stdout and stderr for *reading*.
     That way, any attempt to do normal I/O will result in an error,
     just as if the files were closed, and the file descriptors will
     not be reused by later opens.  */
  force_open (STDIN_FILENO, O_WRONLY);
  force_open (STDOUT_FILENO, O_RDONLY);
  force_open (STDERR_FILENO, O_RDONLY);

  /* Set buferr if possible on platforms defining _PC_PIPE_BUF, as
     they support the notion of atomic writes to pipes.  */
  {
    buferr = emacs_fdopen (STDERR_FILENO, c"w".as_ptr ());
    if !buferr.is_null ()
      { setvbuf (buferr, std::ptr::null_mut (), _IOLBF, 0); }
  }
}

/* Return the current working directory.  The result should be freed
   with 'free'.  Return NULL (setting errno) on errors.  If the
   current directory is unreachable, return either NULL or a string
   beginning with '('.  */

unsafe fn get_current_dir_name_or_unreachable () -> *mut c_char {
  /* Use malloc, not xmalloc, since this function can be called before
     the xmalloc exception machinery is available.  */

  let mut pwd: *mut c_char;

  /* The maximum size of a directory name, including the terminating null.
     Leave room so that the caller can append a trailing slash.  */
  let dirsize_max: ptrdiff_t = (PTRDIFF_MAX as usize).min (SIZE_MAX) as ptrdiff_t - 1;

  /* The maximum size of a buffer for a file name, including the
     terminating null.  This is bounded by PATH_MAX, if available.  */
  let mut bufsize_max: ptrdiff_t = dirsize_max;
  /* C 286-288's `#ifdef PATH_MAX` is live.  */
  {
    bufsize_max = min (bufsize_max, PATH_MAX as ptrdiff_t);
  }

  /* C 290-306's `#if HAVE_GET_CURRENT_DIR_NAME && !BROKEN_GET_CURRENT_DIR_NAME` arm is dead
     (`HAVE_GET_CURRENT_DIR_NAME` is undefined); carried (P-51).  */
  #[cfg (feature = "have_get_current_dir_name")]
  {
    let use_libc: bool = true;
    if use_libc
      {
	/* For an unreachable directory, this returns a string that starts
	   with "(unreachable)"; see Bug#27871.  */
	pwd = get_current_dir_name ();
	if !pwd.is_null ()
	  {
	    if strnlen (pwd, dirsize_max as usize) < dirsize_max as usize
	      { return pwd; }
	    free (pwd as *mut c_void);
	    *errno () = ERANGE;
	  }
	return std::ptr::null_mut ();
      }
  }

  let pwdlen: ptrdiff_t;
  let mut dotstat: stat = std::mem::zeroed ();
  let mut pwdstat: stat = std::mem::zeroed ();
  pwd = getenv (c"PWD".as_ptr ());

  /* If PWD is accurate, use it instead of calling getcwd.  PWD is
     sometimes a nicer name, and using it may avoid a fatal error if a
     parent directory is searchable but not readable.  */
  if !pwd.is_null ()
      && { pwdlen = strnlen (pwd, bufsize_max as usize) as ptrdiff_t;
	   pwdlen < bufsize_max }
      && IS_DIRECTORY_SEP! (*pwd.add (
	   if pwdlen != 0 && IS_DEVICE_SEP! (*pwd.add (1)) { 2 } else { 0 }))
      && emacs_fstatat (AT_FDCWD, pwd, &mut pwdstat as *mut stat as *mut c_void, 0) == 0
      && emacs_fstatat (AT_FDCWD, c".".as_ptr (), &mut dotstat as *mut stat as *mut c_void, 0)
	 == 0
      && dotstat.st_ino == pwdstat.st_ino
      && dotstat.st_dev == pwdstat.st_dev
    { return strdup (pwd); }
  else
    {
      let mut buf_size: ptrdiff_t = min (bufsize_max, 1024);
      loop
        {
	  let buf: *mut c_char = malloc (buf_size as usize) as *mut c_char;
	  if buf.is_null ()
	    { return std::ptr::null_mut (); }
          if getcwd (buf, buf_size as usize) == buf
	    { return buf; }
	  free (buf as *mut c_void);
	  if *errno () != ERANGE || buf_size == bufsize_max
	    { return std::ptr::null_mut (); }
	  buf_size = if buf_size <= bufsize_max / 2 { 2 * buf_size } else { bufsize_max };
        }
    }
}

/* Return the current working directory.  The result should be freed
   with 'free'.  Return NULL (setting errno) on errors; an unreachable
   directory (e.g., its name starts with '(') counts as an error.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_get_current_dir_name () -> *mut c_char {
  let dir: *mut c_char = get_current_dir_name_or_unreachable ();
  if !dir.is_null () && *dir == b'(' as c_char
    {
      free (dir as *mut c_void);
      *errno () = ENOENT;
      return std::ptr::null_mut ();
    }
  return dir;
}

/* Discard pending input on all input descriptors.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn discard_tty_input () {
  // C 364-386: `#ifndef WINDOWSNT` is live; its `MSDOS` arm (C 370-372) is dead and carried.
  {
    let mut buf: emacs_tty = std::mem::zeroed ();

    if noninteractive
      { return; }

    #[cfg (feature = "msdos")]
    {
      while dos_keyread () != -1
	{ }
    }
    #[cfg (not (feature = "msdos"))]
    {
      let mut tty: *mut tty_display_info;
      {
	let mut tty_: *mut tty_display_info = tty_list;
	loop
	  {
	    tty = tty_;
	    if tty.is_null () { break; }
	    for tty in std::iter::once (tty) {
	      if ! (*tty).input.is_null ()         /* Is the device suspended?  */
		{
		  emacs_get_tty (fileno ((*tty).input), &mut buf);
		  emacs_set_tty (fileno ((*tty).input), &mut buf, false);
		}
	    }
	    tty_ = (*tty).next;
	  }
      }
    }
  }
}

// C 390-412: `#ifdef SIGTSTP` — live (`SIGTSTP` is 18 in the frozen table).

/* Arrange for character C to be read as the next input from
   the terminal.
   Return 0 on success, -1 otherwise.
   XXX What if we have multiple ttys?
*/

#[unsafe(no_mangle)]
pub unsafe extern "C" fn stuff_char (mut c: c_char) -> c_int {
/* Should perhaps error if in batch mode */
  /* C 402-407's `#ifdef TIOCSTI` is live.  */
  {
    if FRAMEP (selected_frame)
      && FRAME_LIVE_P (XFRAME (selected_frame))
      && FRAME_TERMCAP_P (XFRAME (selected_frame))
      { return ioctl (fileno ((*CURTTY! ()).input), TIOCSTI, &mut c as *mut c_char); }
  }

  return -1;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_baud_rate (fd: c_int) {
  let mut emacs_ospeed: c_int;

  if noninteractive
    { emacs_ospeed = 0; }
  else
    {
      /* C 423-431's `#ifdef DOS_NT` arm is dead and carried.  */
      #[cfg (feature = "dos_nt")]
      { emacs_ospeed = 15; }
      #[cfg (not (feature = "dos_nt"))]
      {
	let mut sg: termios = std::mem::zeroed ();

	sg.c_cflag = B9600;
	tcgetattr (fd, &mut sg);
	emacs_ospeed = cfgetospeed (&sg) as c_int;
      }
    }

  baud_rate = if (emacs_ospeed as usize) < countof! (baud_convert)
	      { baud_convert[emacs_ospeed as usize] as EMACS_INT } else { 9600 };
  if baud_rate == 0
    { baud_rate = 1200; }
}

// C 442: `#ifndef MSDOS` — live.

/* PORTING.md P-126: the diagnostic switch below is sampled ONCE, at startup, and kept in a
   plain flag.  `getenv` is not async-signal-safe: on this target it takes an os_unfair_lock, so
   calling it from `handle_child_signal` - or being delivered while the interrupted code is
   inside `sysdep`'s own `get_child_status` instrument, which is what the handler's
   `child_status_changed` call runs - makes libplatform abort the whole program ("BUG IN CLIENT
   OF LIBPLATFORM: Trying to recursively lock an os_unfair_lock"; macOS reports that abort as
   SIGKILL).  A flag read is safe there.  P-115's shape is otherwise unchanged: default off, one
   stderr line per event; the variable must be set before startup, which is how the probes set
   it.  */
static FEMACS_CHILD_TRACE: core::sync::atomic::AtomicBool =
  core::sync::atomic::AtomicBool::new (false);

/* C has no counterpart: P-122/P-125's instruments ask this instead of `getenv`.  */
#[inline]
unsafe fn femacs_child_trace () -> bool {
  return FEMACS_CHILD_TRACE.load (core::sync::atomic::Ordering::Relaxed);
}

/* Wait for the subprocess with process id CHILD to terminate or change status.
   CHILD must be a child process that has not been reaped.
   If STATUS is non-null, store the waitpid-style exit status into *STATUS
   and tell wait_reading_process_output that it needs to look around.
   Use waitpid-style OPTIONS when waiting.
   If INTERRUPTIBLE, this function is interruptible by a signal.

   Return CHILD if successful, 0 if no status is available, and a
   negative value (setting errno) if waitpid is buggy.  */
unsafe fn get_child_status (child: pid_t, status: *mut c_int, options: c_int,
                            interruptible: bool) -> pid_t {
  /* PORTING.md P-122's instrument (FEMACS_CHILD_TRACE, P-115's shape): every reap in the port
     goes through here, so a missing line means the reap itself never happened.  */
  if femacs_child_trace ()
    {
      fprintf (stderr, c"child-get: child=%d options=%d\n".as_ptr (), child as c_int, options);
    }

  /* PORTING.md 1.5: C declares `pid_t pid;` and the loop below assigns it.  */
  let mut pid: pid_t = 0;

  /* Invoke waitpid only with a known process ID; do not invoke
     waitpid with a nonpositive argument.  Otherwise, Emacs might
     reap an unwanted process by mistake.  For example, invoking
     waitpid (-1, ...) can mess up glib by reaping glib's subprocesses,
     so that another thread running glib won't find them.  */
  eassert! (child > 0);

  while true
  {
    /* Note: The MS-Windows emulation of waitpid calls maybe_quit
       internally.  */
    if interruptible
      { maybe_quit (); }

    pid = waitpid (child, status, options);
    if 0 <= pid
      { break; }
    if *errno () != EINTR
      {
	/* Most likely, waitpid is buggy and the operating system
	   lost track of the child somehow.  Return -1 and let the
	   caller try to figure things out.  Possibly the bug could
	   cause Emacs to kill the wrong process.  Oh well.  */
	return pid;
      }
  }

  /* If successful and status is requested, tell wait_reading_process_output
     that it needs to wake up and look around.  */
  if pid != 0 && !status.is_null () && !input_available_clear_time.is_null ()
    { *input_available_clear_time = make_timespec (0, 0); }

  /* PORTING.md P-122 ⑻'s instrument (FEMACS_CHILD_TRACE, P-115's shape): what waitpid
     answered - 0 means "no status available yet", which is the branch a missed reap takes.  */
  if femacs_child_trace ()
    {
      fprintf (stderr, c"child-get-done: pid=%d errno=%d\n".as_ptr (),
	       pid as c_int, *errno ());
    }

  return pid;
}

/* Wait for the subprocess with process id CHILD to terminate.
   CHILD must be a child process that has not been reaped.
   If STATUS is non-null, store the waitpid-style exit status into *STATUS
   and tell wait_reading_process_output that it needs to look around.
   If INTERRUPTIBLE, this function is interruptible by a signal.
   Return true if successful, false (setting errno) if CHILD cannot be
   waited for because waitpid is buggy.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wait_for_termination (child: pid_t, status: *mut c_int,
                                               interruptible: bool) -> bool {
  return 0 <= get_child_status (child, status, 0, interruptible);
}

/* Report whether the subprocess with process id CHILD has changed status.
   Termination counts as a change of status.
   CHILD must be a child process that has not been reaped.
   If STATUS is non-null, store the waitpid-style exit status into *STATUS
   and tell wait_reading_process_output that it needs to look around.
   Use waitpid-style OPTIONS to check status, but do not wait.

   Return CHILD if successful, 0 if no status is available because
   the process's state has not changed.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn child_status_changed (child: pid_t, status: *mut c_int,
                                               options: c_int) -> pid_t {
  return get_child_status (child, status, WNOHANG | options, false);
}

/*  Set up the terminal at the other end of a pseudo-terminal that
    we will be controlling an inferior through.
    It should not echo or do line-editing, since that is done
    in Emacs.  No padding needed for insertion into an Emacs buffer.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn child_setup_tty (out: c_int) {
  // C 530: `#ifndef WINDOWSNT` is live.
  {
    let mut s: emacs_tty = std::mem::zeroed ();

    emacs_get_tty (out, &mut s);
    s.main.c_oflag |= OPOST;	/* Enable output postprocessing */
    s.main.c_oflag &= !ONLCR;	/* Disable map of NL to CR-NL on output */
    /* C 536-546: `#ifdef NLDLY` is live, and its inner `#ifdef FFDLY` too.  */
    {
      s.main.c_oflag &= !(NLDLY|CRDLY|TABDLY|BSDLY|VTDLY|FFDLY);
				/* No output delays */
    }
    s.main.c_lflag &= !ECHO;	/* Disable echo */
    s.main.c_lflag |= ISIG;	/* Enable signals */
    /* C 549-551: `#ifdef IUCLC` is dead on this target; carried.  */
    #[cfg (feature = "iuclc")]
    {
      s.main.c_iflag &= !IUCLC;	/* Disable downcasing on input.  */
    }
    /* C 552-554: `#ifdef ISTRIP` is live.  */
    {
      s.main.c_iflag &= !ISTRIP;	/* don't strip 8th bit on input */
    }
    /* C 555-557: `#ifdef OLCUC` is dead on this target; carried.  */
    #[cfg (feature = "olcuc")]
    {
      s.main.c_oflag &= !OLCUC;	/* Disable upcasing on output.  */
    }
    s.main.c_oflag &= !TAB3;	/* Disable tab expansion */
    s.main.c_cflag = (s.main.c_cflag & !CSIZE) | CS8; /* Don't strip 8th bit */
    s.main.c_cc[VERASE] = CDISABLE;	/* disable erase processing */
    s.main.c_cc[VKILL] = CDISABLE;	/* disable kill processing */

    /* C 563-565: `#ifdef HPUX` is dead; carried.  */
    #[cfg (feature = "hpux")]
    {
      s.main.c_cflag = (s.main.c_cflag & !CBAUD) | B9600; /* baud rate sanity */
    }

    /* C 567-574: `#ifdef SIGNALS_VIA_CHARACTERS` is live.  */
    {
      /* the QUIT and INTR character are used in process_send_signal
	 so set them here to something useful.  */
      if s.main.c_cc[VQUIT] == CDISABLE
	{ s.main.c_cc[VQUIT] = b'\\' as cc_t & 0o37; }	/* Control-\ */
      if s.main.c_cc[VINTR] == CDISABLE
	{ s.main.c_cc[VINTR] = b'C' as cc_t & 0o37; }	/* Control-C */
    }

    /* C 576-585: `#ifdef AIX` is dead; carried.  */
    #[cfg (feature = "aix")]
    {
      /* Also, PTY overloads NUL and BREAK.
	 don't ignore break, but don't signal either, so it looks like NUL.  */
      s.main.c_iflag &= !IGNBRK;
      s.main.c_iflag &= !BRKINT;
      /* rms: Formerly it set s.main.c_cc[VINTR] to 0377 here
	 unconditionally.  Then a SIGNALS_VIA_CHARACTERS conditional
	 would force it to 0377.  That looks like duplicated code.  */
      s.main.c_cflag = (s.main.c_cflag & !CBAUD) | B9600; /* baud rate sanity */
    }

    /* We originally enabled ICANON (and set VEOF to 04), and then had
       process.c send additional EOF chars to flush the output when faced
       with long lines, but this leads to weird effects when the
       subprocess has disabled ICANON and ends up seeing those spurious
       extra EOFs.  So we don't send EOFs any more in
       process.c:send_process.  First we tried to disable ICANON by
       default, so if a subsprocess sets up ICANON, it's his problem (or
       the Elisp package that talks to it) to deal with lines that are
       too long.  But this disables some features, such as the ability
       to send EOF signals.  So we re-enabled ICANON but there is no
       more "send eof to flush" going on (which is wrong and unportable
       in itself).  The correct way to handle too much output is to
       buffer what could not be written and then write it again when
       select returns ok for writing.  This has it own set of
       problems.  Write is now asynchronous, is that a problem?  How much
       do we buffer, and what do we do when that limit is reached?  */

    s.main.c_lflag |= ICANON;	/* Enable line editing and eof processing */
    s.main.c_cc[VEOF] = b'D' as cc_t & 0o37;	/* Control-D */
    /* C 606-609: `#if 0` — dead text, recorded in docs/ports/sysdep.md, no Rust counterpart.  */

    emacs_set_tty (out, &mut s, false);
  }
}

/* Record a signal code and the action for it.  */
// C 529-534's `struct save_signal` is declared next to the use-site declarations above.

/// C 630-644's `sys_suspend`, whose `#ifndef DOS_NT` arm is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_suspend () {
  /* C 632-640: `#ifndef DOS_NT` is live; the `#else` arm is carried.  */
  #[cfg (not (feature = "dos_nt"))]
  {
    kill (0, SIGTSTP);
  }
  #[cfg (feature = "dos_nt")]
  {
/* On a system where suspending is not implemented,
   instead fork a subshell and let it talk directly to the terminal
   while we wait.  */
    sys_subshell ();
  }
}

/* Fork a subshell.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_subshell () {
  #[cfg (feature = "dos_nt")]
  let mut st: c_int = 0;
  #[cfg (feature = "msdos")]
  let mut oldwd: [c_char; MAXPATHLEN + 1] = [0; MAXPATHLEN + 1]; /* Fixed length is safe on MSDOS.  */
  #[cfg (all (feature = "dos_nt", not (feature = "msdos")))]
  let mut oldwd: [c_char; MAX_UTF8_PATH] = [0; MAX_UTF8_PATH];
  #[cfg (not (feature = "dos_nt"))]
  let mut status: c_int = 0;
  /* PORTING.md 1.5: C declares `pid_t pid;` and both arms of the `#ifdef DOS_NT` assign it.  */
  let mut pid: pid_t = 0;
  let mut saved_handlers: [save_signal; 5] = [std::mem::zeroed (); 5];
  let mut str_: *mut c_char = SSDATA (get_current_directory (true));

  #[cfg (feature = "dos_nt")]
  { pid = 0; }
  #[cfg (not (feature = "dos_nt"))]
  {
    {
      let str_volatile: *mut c_char = str_;
      pid = VFORK! ();
      str_ = str_volatile;
    }
  }

  if pid < 0
    { error (c"Can't spawn subshell".as_ptr ()); }

  saved_handlers[0].code = SIGINT;
  saved_handlers[1].code = SIGQUIT;
  saved_handlers[2].code = SIGTERM;
  /* C 678-686: `#ifdef USABLE_SIGIO` is live; the `USABLE_SIGPOLL` and `#else` arms are
     carried.  */
  {
    saved_handlers[3].code = SIGIO;
    saved_handlers[4].code = 0;
  }
  #[cfg (feature = "usable_sigpoll")]
  {
    saved_handlers[3].code = SIGPOLL;
    saved_handlers[4].code = 0;
  }
  #[cfg (feature = "no_usable_sigio")]
  {
    saved_handlers[3].code = 0;
  }

  #[cfg (feature = "dos_nt")]
  { save_signal_handlers (saved_handlers.as_mut_ptr ()); }

  if pid == 0
    {
      let mut sh: *const c_char = std::ptr::null ();

      #[cfg (feature = "dos_nt")]
      {
	getcwd (oldwd.as_mut_ptr (), oldwd.len ());
	if sh.is_null ()
	  { sh = egetenv (c"SUSPEND".as_ptr ()); }	/* KFS, 1994-12-14 */
      }
      if sh.is_null ()
	{ sh = egetenv (c"SHELL".as_ptr ()); }
      if sh.is_null ()
	{ sh = c"sh".as_ptr (); }

      /* Use our buffer's default directory for the subshell.  */
      if chdir (str_) != 0
	{
	  #[cfg (not (feature = "dos_nt"))]
	  {
	    emacs_perror (str_);
	    _exit (EXIT_CANCELED);
	  }
	}

      #[cfg (feature = "msdos")]
      {
	let epwd: *mut c_char = getenv (c"PWD".as_ptr ());
	let mut old_pwd: [c_char; MAXPATHLEN + 1 + 4] = [0; MAXPATHLEN + 1 + 4];

	/* If PWD is set, pass it with corrected value.  */
	if !epwd.is_null ()
	  {
	    strcpy (old_pwd.as_mut_ptr (), epwd);
	    setenv (c"PWD".as_ptr (), str_, 1);
	  }
	st = system (sh);
	chdir (oldwd.as_mut_ptr ());	/* FIXME: Do the right thing on chdir failure.  */
	if !epwd.is_null ()
	  { putenv (old_pwd.as_mut_ptr ()); }	/* restore previous value */
      }
      #[cfg (not (feature = "msdos"))]
      {
	#[cfg (feature = "windowsnt")]
	{
	  /* Waits for process completion */
	  pid = _spawnlp (_P_WAIT, sh, sh, std::ptr::null ());
	  chdir (oldwd.as_mut_ptr ());	/* FIXME: Do the right thing on chdir failure.  */
	  if pid == -1
	    { write (1, c"Can't execute subshell".as_ptr () as *const c_void, 22); }
	}
	#[cfg (not (feature = "windowsnt"))]
	{
	  execlp (sh, sh, std::ptr::null::<c_char> ());
	  emacs_perror (sh);
	  _exit (if *errno () == ENOENT { EXIT_ENOENT } else { EXIT_CANNOT_INVOKE });
	}
      }
    }

  /* Do this now if we did not do it before.  */
  #[cfg (not (feature = "msdos"))]
  { save_signal_handlers (saved_handlers.as_mut_ptr ()); }

  #[cfg (not (feature = "dos_nt"))]
  { wait_for_termination (pid, &mut status, false); }
  restore_signal_handlers (saved_handlers.as_mut_ptr ());
}

unsafe extern "C" fn save_signal_handlers (saved_handlers: *mut save_signal) {
  let mut saved_handlers = saved_handlers;
  while (*saved_handlers).code != 0
    {
      let mut action: sigaction = std::mem::zeroed ();
      emacs_sigaction_init (&mut action, std::mem::transmute::<usize, signal_handler_t> (SIG_IGN));
      sigaction ((*saved_handlers).code, &action, &mut (*saved_handlers).action);
      saved_handlers = saved_handlers.add (1);
    }
}

unsafe extern "C" fn restore_signal_handlers (saved_handlers: *mut save_signal) {
  let mut saved_handlers = saved_handlers;
  while (*saved_handlers).code != 0
    {
      sigaction ((*saved_handlers).code, &(*saved_handlers).action, std::ptr::null_mut ());
      saved_handlers = saved_handlers.add (1);
    }
}

// C 779-781: `#ifdef USABLE_SIGIO` — live (USABLE_SIGIO is 1).
static mut old_fcntl_flags: [c_int; FD_SETSIZE as usize] = [0; FD_SETSIZE as usize];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_sigio (fd: c_int) {
  /* C 786-790: `#ifdef USABLE_SIGIO` is live.  */
  {
    old_fcntl_flags[fd as usize] = fcntl (fd, F_GETFL, 0) & !FASYNC;
    fcntl (fd, F_SETFL, old_fcntl_flags[fd as usize] | FASYNC);
    interrupts_deferred = false;
  }
}

// C 793-805: `#ifndef HAVE_ANDROID`, `#ifndef DOS_NT` and `#ifdef F_SETOWN` are all live.
unsafe fn reset_sigio (fd: c_int) {
  /* C 799-801: `#ifdef USABLE_SIGIO` is live.  */
  {
    fcntl (fd, F_SETFL, old_fcntl_flags[fd as usize]);
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn request_sigio () {
  /* C 810-828: `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` is live.  */
  {
    let mut unblocked: sigset_t = std::mem::zeroed ();

    if noninteractive
      { return; }

    sigemptyset (&mut unblocked);
    {
      sigaddset (&mut unblocked, SIGWINCH);
    }
    {
      /* C 820-824: `# ifdef USABLE_SIGIO` is live; the `#else` arm is carried.  */
    #[cfg (not (feature = "usable_sigpoll"))]
    {
      sigaddset (&mut unblocked, SIGIO);
    }
    #[cfg (feature = "usable_sigpoll")]
    {
      sigaddset (&mut unblocked, SIGPOLL);
    }
    }
    pthread_sigmask (SIG_UNBLOCK, &unblocked, std::ptr::null_mut ());

    interrupts_deferred = false;
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unrequest_sigio () {
  /* C 834-851: `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` is live.  */
  {
    let mut blocked: sigset_t = std::mem::zeroed ();

    if noninteractive
      { return; }

    sigemptyset (&mut blocked);
    {
      sigaddset (&mut blocked, SIGWINCH);
    }
    {
      /* C 844-848: `# ifdef USABLE_SIGIO` is live; the `#else` arm is carried.  */
    #[cfg (not (feature = "usable_sigpoll"))]
    {
      sigaddset (&mut blocked, SIGIO);
    }
    #[cfg (feature = "usable_sigpoll")]
    {
      sigaddset (&mut blocked, SIGPOLL);
    }
    }
    pthread_sigmask (SIG_BLOCK, &blocked, std::ptr::null_mut ());
    interrupts_deferred = true;
  }
}

// C 854: `#ifndef MSDOS` — live.
/* Block SIGCHLD.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn block_child_signal (oldset: *mut sigset_t) {
  let mut blocked: sigset_t = std::mem::zeroed ();
  sigemptyset (&mut blocked);
  sigaddset (&mut blocked, SIGCHLD);
  sigaddset (&mut blocked, SIGINT);
  pthread_sigmask (SIG_BLOCK, &blocked, oldset);
}

/* Unblock SIGCHLD.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unblock_child_signal (oldset: *const sigset_t) {
  pthread_sigmask (SIG_SETMASK, oldset, std::ptr::null_mut ());
}

/* Block SIGINT.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn block_interrupt_signal (oldset: *mut sigset_t) {
  let mut blocked: sigset_t = std::mem::zeroed ();
  sigemptyset (&mut blocked);
  sigaddset (&mut blocked, SIGINT);
  pthread_sigmask (SIG_BLOCK, &blocked, oldset);
}

/* Restore previously saved signal mask.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_signal_mask (oldset: *const sigset_t) {
  pthread_sigmask (SIG_SETMASK, oldset, std::ptr::null_mut ());
}

/* Saving and restoring the process group of Emacs's terminal.  */

/* The process group of which Emacs was a member when it initially
   started.

   If Emacs was in its own process group (i.e. inherited_pgroup ==
   getpid ()), then we know we're running under a shell with job
   control (Emacs would never be run as part of a pipeline).
   Everything is fine.

   If Emacs was not in its own process group, then we know we're
   running under a shell (or a caller) that doesn't know how to
   separate itself from Emacs (like sh).  Emacs must be in its own
   process group in order to receive SIGIO correctly.  In this
   situation, we put ourselves in our own pgroup, forcibly set the
   tty's pgroup to our pgroup, and make sure to restore and reinstate
   the tty's pgroup just like any other terminal setting.  If
   inherited_group was not the tty's pgroup, then we'll get a
   SIGTTmumble when we try to change the tty's pgroup, and a CONT if
   it goes foreground in the future, which is what should happen.  */

static mut inherited_pgroup: pid_t = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_foreground_group () {
  let pgrp: pid_t = getpgrp ();
  inherited_pgroup = if getpid () == pgrp { 0 } else { pgrp };
}

/* Block and unblock SIGTTOU.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn block_tty_out_signal (oldset: *mut sigset_t) {
  /* C 930-935: `#ifdef SIGTTOU` is live.  */
  {
    let mut blocked: sigset_t = std::mem::zeroed ();
    sigemptyset (&mut blocked);
    sigaddset (&mut blocked, SIGTTOU);
    pthread_sigmask (SIG_BLOCK, &blocked, oldset);
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unblock_tty_out_signal (oldset: *const sigset_t) {
  /* C 941-943: `#ifdef SIGTTOU` is live.  */
  {
    pthread_sigmask (SIG_SETMASK, oldset, std::ptr::null_mut ());
  }
}

/* Safely set a controlling terminal FD's process group to PGID.
   If we are not in the foreground already, POSIX requires tcsetpgrp
   to deliver a SIGTTOU signal, which would stop us.  This is an
   annoyance, so temporarily ignore the signal.

   In practice, platforms lacking SIGTTOU also lack tcsetpgrp, so
   skip all this unless SIGTTOU is defined.  */
unsafe fn tcsetpgrp_without_stopping (fd: c_int, pgid: pid_t) {
  /* C 956-963: `#ifdef SIGTTOU` is live.  */
  {
    let mut oldset: sigset_t = std::mem::zeroed ();
    block_input ();
    block_tty_out_signal (&mut oldset);
    tcsetpgrp (fd, pgid);
    unblock_tty_out_signal (&oldset);
    unblock_input ();
  }
}

/* Split off the foreground process group to Emacs alone.  When we are
   in the foreground, but not started in our own process group,
   redirect the tty device handle FD to point to our own process
   group.  FD must be the file descriptor of the controlling tty.  */
unsafe fn narrow_foreground_group (fd: c_int) {
  if inherited_pgroup != 0 && setpgid (0, 0) == 0
    { tcsetpgrp_without_stopping (fd, getpid ()); }
}

// C 977-987: `#ifndef HAVE_ANDROID` — live.

/* Set the tty to our original foreground group.  */
unsafe fn widen_foreground_group (fd: c_int) {
  if inherited_pgroup != 0 && setpgid (0, inherited_pgroup) == 0
    { tcsetpgrp_without_stopping (fd, inherited_pgroup); }
}

/* Getting and setting emacs_tty structures.  */

/* Set *TC to the parameters associated with the terminal FD,
   or clear it if the parameters are not available.
   Return 0 on success, -1 on failure.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_get_tty (fd: c_int, settings: *mut emacs_tty) -> c_int {
  /* Retrieve the primary parameters - baud rate, character size, etcetera.  */
  memset (&mut (*settings).main as *mut termios as *mut c_void, 0,
	  std::mem::size_of::<termios> ());
  /* C 1000-1015: `#ifdef DOS_NT` is dead and carried.  */
  #[cfg (feature = "dos_nt")]
  {
    #[cfg (feature = "windowsnt")]
    {
      let h: HANDLE = _get_osfhandle (fd) as HANDLE;
      let mut console_mode: DWORD = 0;

      if !h.is_null () && h != INVALID_HANDLE_VALUE && GetConsoleMode (h, &mut console_mode) != 0
	{
	  (*settings).main = console_mode;
	  return 0;
	}
    }
    return -1;
  }
  #[cfg (not (feature = "dos_nt"))]
  {
    /* We have those nifty POSIX tcmumbleattr functions.  */
    return tcgetattr (fd, &mut (*settings).main);
  }
}

/* Set the parameters of the tty on FD according to the contents of
   *SETTINGS.  If FLUSHP, discard input.
   Return 0 if all went well, and -1 (setting errno) if anything failed.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_set_tty (fd: c_int, settings: *mut emacs_tty, flushp: bool)
  -> c_int {
  /* Set the primary parameters - baud rate, character size, etcetera.  */
  #[cfg (feature = "dos_nt")]
  {
    #[cfg (feature = "windowsnt")]
    {
      let h: HANDLE = _get_osfhandle (fd) as HANDLE;

      if !h.is_null () && h != INVALID_HANDLE_VALUE
	{
	  let mut new_mode: DWORD = 0;

	  /* Assume the handle is open for input.  */
	  if flushp
	    { FlushConsoleInputBuffer (h); }
	  new_mode = (*settings).main;
	  SetConsoleMode (h, new_mode);
	}
    }
  }
  #[cfg (not (feature = "dos_nt"))]
  {
    let mut i: c_int;
    /* We have those nifty POSIX tcmumbleattr functions.
       William J. Smith <wjs@wiis.wang.com> writes:
       "POSIX 1003.1 defines tcsetattr to return success if it was
       able to perform any of the requested actions, even if some
       of the requested actions could not be performed.
       We must read settings back to ensure tty setup properly.
       AIX requires this to keep tty from hanging occasionally."  */
    /* This make sure that we don't loop indefinitely in here.  */
    i = 0;
    while i < 10
      {
	if tcsetattr (fd, if flushp { TCSAFLUSH } else { TCSADRAIN }, &(*settings).main) < 0
	  {
	    if *errno () == EINTR
	      { i += 1; continue; }
	    else
	      { return -1; }
	  }
	else
	  {
	    let mut new: termios = std::mem::zeroed ();

	    memset (&mut new as *mut termios as *mut c_void, 0, std::mem::size_of::<termios> ());
	    /* Get the current settings, and see if they're what we asked for.  */
	    tcgetattr (fd, &mut new);
	    /* We cannot use memcmp on the whole structure here because under
	     * aix386 the termios structure has some reserved field that may
	     * not be filled in.
	     */
	    if   new.c_iflag == (*settings).main.c_iflag
	      && new.c_oflag == (*settings).main.c_oflag
	      && new.c_cflag == (*settings).main.c_cflag
	      && new.c_lflag == (*settings).main.c_lflag
	      && memcmp (&new.c_cc as *const cc_t as *const c_void,
			 &(*settings).main.c_cc as *const cc_t as *const c_void,
			 NCCS as usize) == 0
	      { break; }
	    else
	      { i += 1; continue; }
	  }
      }
  }

  /* We have survived the tempest.  */
  return 0;
}

// C 1088-1090: `#ifdef F_SETOWN` — live.
static mut old_fcntl_owner: [c_int; FD_SETSIZE as usize] = [0; FD_SETSIZE as usize];

/// termchar.h 66 and 187-217's `bool_bf` runs, in P-80's member-name form: C's field names
/// stay in the port's text and dispatch to the mask bytes the mirror declares
/// (`femacs/src/include/termchar.rs`'s `bits_initted`/`bits_caps` and their `TTY_*` masks).
/// This unit reaches two of the members — `term_initted` (read at C 1148/1215/1216, written
/// at C 1356) and `flow_control` (read at C 1156/1217/1246/1251/1282/1286/1291) — which is
/// why the family has exactly those two.
macro_rules! tty_bits {
  (term_initted, $t:expr) =>
    { (*($t)).bits_initted & TTY_TERM_INITTED != 0 };
  (term_initted, $t:expr, $value:expr) =>
    { (*($t)).bits_initted =
	(if ($value) as c_int != 0 { (*($t)).bits_initted | TTY_TERM_INITTED }
	 else { (*($t)).bits_initted & !TTY_TERM_INITTED }) };
  (flow_control, $t:expr) =>
    { (*($t)).bits_caps & TTY_FLOW_CONTROL != 0 };
}

/* Initialize the terminal mode on all tty devices that are currently
   open. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_all_sys_modes () {
  let mut tty: *mut tty_display_info;
  {
    let mut tty_: *mut tty_display_info = tty_list;
    loop
      {
	tty = tty_;
	if tty.is_null () { break; }
	for tty in std::iter::once (tty) {
	  init_sys_modes (tty);
	}
	tty_ = (*tty).next;
      }
  }
}

/* Initialize the terminal mode on the given tty device. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_sys_modes (tty_out: *mut tty_display_info) {
  let mut tty: emacs_tty = std::mem::zeroed ();
  // C 1109-1111: `#ifndef DOS_NT` — live.
  let mut terminal: Lisp_Object = Qnil ();

  Vtty_erase_char = Qnil ();

  if noninteractive
    { return; }

  if (*tty_out).output.is_null ()
    { return; }                     /* The tty is suspended. */

  narrow_foreground_group (fileno ((*tty_out).input));

  if (*tty_out).old_tty.is_null ()
    { (*tty_out).old_tty = xmalloc (std::mem::size_of::<emacs_tty> ()); }

  emacs_get_tty (fileno ((*tty_out).input), (*tty_out).old_tty as *mut emacs_tty);

  tty = *((*tty_out).old_tty as *mut emacs_tty);

  // C 1130-1266: `#if !defined (DOS_NT)` — live.
  {
    XSETINT! (Vtty_erase_char, tty.main.c_cc[VERASE] as EMACS_INT);

    tty.main.c_iflag |= (IGNBRK);	/* Ignore break condition */
    tty.main.c_iflag &= !ICRNL;	/* Disable map of CR to NL on input */
    /* C 1135-1138: `#ifdef INLCR` is live (the trailing comment in C's condition is
       tools/cinventory.py's probe defect — docs/ports/sysdep.md).  */
    {
      tty.main.c_iflag &= !INLCR;	/* Disable map of NL to CR on input */
    }
    /* C 1139-1141: `#ifdef ISTRIP` is live.  */
    {
      tty.main.c_iflag &= !ISTRIP;	/* don't strip 8th bit on input */
    }
    tty.main.c_lflag &= !ECHO;	/* Disable echo */
    tty.main.c_lflag &= !ICANON;	/* Disable erase/kill processing */
    /* C 1144-1146: `#ifdef IEXTEN` is live.  */
    {
      tty.main.c_lflag &= !IEXTEN;	/* Disable other editing characters.  */
    }
    tty.main.c_lflag |= ISIG;	/* Enable signals */
    if tty_bits! (flow_control, tty_out)
      {
	tty.main.c_iflag |= IXON;	/* Enable start/stop output control */
	/* C 1151-1153: `#ifdef IXANY` is live.  */
	{
	  tty.main.c_iflag &= !IXANY;
	}
      }
    else
      { tty.main.c_iflag &= !IXON; }	/* Disable start/stop output control */
    tty.main.c_oflag &= !ONLCR;	/* Disable map of NL to CR-NL
                                     on output */
    tty.main.c_oflag &= !TAB3;	/* Disable tab expansion */
    /* C 1160-1166: `#ifdef CS8` is live.  */
    {
      if (*tty_out).meta_key != 0
	{
	  tty.main.c_cflag |= CS8;	/* allow 8th bit on input */
	  tty.main.c_cflag &= !PARENB;/* Don't check parity */
	}
    }

    XSETTERMINAL! (terminal, (*tty_out).terminal);
    if !NILP (Fcontrolling_tty_p (terminal))
      {
	tty.main.c_cc[VINTR] = quit_char as cc_t;	/* C-g (usually) gives SIGINT */
	/* Set up C-g for both SIGQUIT and SIGINT.
	   We don't know which we will get, but we handle both alike
	   so which one it really gives us does not matter.  */
	tty.main.c_cc[VQUIT] = quit_char as cc_t;
      }
    else
      {
	/* We normally don't get interrupt or quit signals from tty
	   devices other than our controlling terminal; therefore,
	   we must handle C-g as normal input.  Unfortunately, this
	   means that the interrupt and quit feature must be
	   disabled on secondary ttys, or we would not even see the
	   keypress.

	   Note that even though emacsclient could have special code
	   to pass SIGINT to Emacs, we should _not_ enable
	   interrupt/quit keys for emacsclient frames.  This means
	   that we can't break out of loops in C code from a
	   secondary tty frame, but we can always decide what
	   display the C-g came from, which is more important from a
	   usability point of view.  (Consider the case when two
	   people work together using the same Emacs instance.)  */
	tty.main.c_cc[VINTR] = CDISABLE;
	tty.main.c_cc[VQUIT] = CDISABLE;
      }
    tty.main.c_cc[VMIN] = 1;	/* Input should wait for at least 1 char */
    tty.main.c_cc[VTIME] = 0;	/* no matter how long that takes.  */
    /* C 1199-1202: `#ifdef VSWTCH` is dead; carried.  */
    #[cfg (feature = "vswtch")]
    {
      tty.main.c_cc[VSWTCH] = CDISABLE;	/* Turn off shell layering use
					   of C-z */
    }

    /* C 1204-1206: `#ifdef VSUSP` is live.  */
    {
      tty.main.c_cc[VSUSP] = CDISABLE;	/* Turn off handling of C-z.  */
    }
    /* C 1207-1209: `#ifdef V_DSUSP` is dead; carried.  */
    #[cfg (feature = "v_dsusp")]
    {
      tty.main.c_cc[V_DSUSP] = CDISABLE; /* Turn off handling of C-y.  */
    }
    /* C 1210-1212: `#ifdef VDSUSP` is live.  */
    {
      tty.main.c_cc[VDSUSP] = CDISABLE;
    }
    /* C 1213-1215: `#ifdef VLNEXT` is live.  */
    {
      tty.main.c_cc[VLNEXT] = CDISABLE;
    }
    /* C 1216-1218: `#ifdef VREPRINT` is live.  */
    {
      tty.main.c_cc[VREPRINT] = CDISABLE;
    }
    /* C 1219-1221: `#ifdef VWERASE` is live.  */
    {
      tty.main.c_cc[VWERASE] = CDISABLE;
    }
    /* C 1222-1224: `#ifdef VDISCARD` is live.  */
    {
      tty.main.c_cc[VDISCARD] = CDISABLE;
    }

    if tty_bits! (flow_control, tty_out)
      {
	/* C 1228-1230: `#ifdef VSTART` is live.  */
	{
	  tty.main.c_cc[VSTART] = 0o021;
	}
	/* C 1231-1233: `#ifdef VSTOP` is live.  */
	{
	  tty.main.c_cc[VSTOP] = 0o023;
	}
      }
    else
      {
	/* C 1237-1239: `#ifdef VSTART` is live.  */
	{
	  tty.main.c_cc[VSTART] = CDISABLE;
	}
	/* C 1240-1242: `#ifdef VSTOP` is live.  */
	{
	  tty.main.c_cc[VSTOP] = CDISABLE;
	}
      }

    /* C 1245-1265: `#ifdef AIX` is dead; carried.  */
    #[cfg (feature = "aix")]
    {
      tty.main.c_cc[VSTRT] = CDISABLE;
      tty.main.c_cc[VSTOP] = CDISABLE;
      tty.main.c_cc[VSUSP] = CDISABLE;
      tty.main.c_cc[VDSUSP] = CDISABLE;
      if tty_bits! (flow_control, tty_out)
	{
	  tty.main.c_cc[VSTART] = 0o021;
	  tty.main.c_cc[VSTOP] = 0o023;
	}
      /* Also, PTY overloads NUL and BREAK.
	 don't ignore break, but don't signal either, so it looks like NUL.
	 This really serves a purpose only if running in an XTERM window
	 or via TELNET or the like, but does no harm elsewhere.  */
      tty.main.c_iflag &= !IGNBRK;
      tty.main.c_iflag &= !BRKINT;
    }
  }

  /* C 1268-1272: `#ifdef MSDOS` is dead; carried.  */
  #[cfg (feature = "msdos")]
  {
    if !tty_bits! (term_initted, tty_out)
      { internal_terminal_init (); }
    dos_ttraw (tty_out);
  }

  emacs_set_tty (fileno ((*tty_out).input), &mut tty, false);

  /* This code added to insure that, if flow-control is not to be used,
     we have an unlocked terminal at the start. */

  /* C 1279-1284: `#ifndef HAIKU` is live.  */
  {
    /* C 1281-1283: `#ifdef TCXONC` is dead on this target; carried.  */
    #[cfg (feature = "tcxonc")]
    {
      if !tty_bits! (flow_control, tty_out) { ioctl (fileno ((*tty_out).input), TCXONC, 1); }
    }
  }
  /* C 1285-1287: `#ifdef TIOCSTART` is live.  */
  {
    if !tty_bits! (flow_control, tty_out) { ioctl (fileno ((*tty_out).input), TIOCSTART, 0); }
  }

  /* C 1289-1293: `#if !defined (DOS_NT)` and its `#ifdef TCOON` are live.  */
  {
    {
      if !tty_bits! (flow_control, tty_out) { tcflow (fileno ((*tty_out).input), TCOON); }
    }
  }

  /* C 1295-1312: `#ifdef F_GETOWN` is live.  */
  {
    if interrupt_input
      {
	old_fcntl_owner[fileno ((*tty_out).input) as usize] =
	  fcntl (fileno ((*tty_out).input), F_GETOWN, 0);
	fcntl (fileno ((*tty_out).input), F_SETOWN, getpid ());
	init_sigio (fileno ((*tty_out).input));
	/* C 1302-1310: `#ifdef HAVE_GPM` is dead; carried.  */
	#[cfg (feature = "have_gpm")]
	{
	  if gpm_tty == tty_out
	    {
	      /* Arrange for mouse events to give us SIGIO signals.  */
	      fcntl (gpm_fd, F_SETOWN, getpid ());
	      fcntl (gpm_fd, F_SETFL, fcntl (gpm_fd, F_GETFL, 0) | O_NONBLOCK);
	      init_sigio (gpm_fd);
	    }
	}
      }
  }

  let buffer_size: ptrdiff_t = if (*tty_out).output_buffer_size != 0
				 { (*tty_out).output_buffer_size as ptrdiff_t }
				 else { BUFSIZ as ptrdiff_t };
  setvbuf ((*tty_out).output, std::ptr::null_mut (), _IOFBF, buffer_size as usize);

  if (*(*tty_out).terminal).set_terminal_modes_hook.is_some ()
    { (*(*tty_out).terminal).set_terminal_modes_hook.unwrap () ((*tty_out).terminal); }

  if !tty_bits! (term_initted, tty_out)
    {
      let mut tail: Lisp_Object = Qnil ();
      let mut frame: Lisp_Object = Qnil ();
      FOR_EACH_FRAME! (tail, frame, {
	  /* XXX This needs to be revised. */
	  if FRAME_TERMCAP_P (XFRAME (frame))
	      && FRAME_TTY! (XFRAME (frame)) == tty_out
	    { init_frame_faces (XFRAME (frame)); }
      });
    }

  if tty_bits! (term_initted, tty_out) && no_redraw_on_reenter
    {
      /* We used to call "direct_output_forward_char(0)" here,
	 but it's not clear why, since it may not do anything anyway.  */
    }
  else
    {
      let mut tail: Lisp_Object = Qnil ();
      let mut frame: Lisp_Object = Qnil ();
      frame_garbaged = true;
      FOR_EACH_FRAME! (tail, frame, {
	  if is_tty_frame (XFRAME (frame))
	      && FRAME_TTY! (XFRAME (frame)) == tty_out
	    { FRAME_GARBAGED_P! (XFRAME (frame), 1); }
      });
    }

  tty_bits! (term_initted, tty_out, 1);
}

/* Return true if safe to use tabs in output.
   At the time this is called, init_sys_modes has not been done yet.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tabs_safe_p (fd: c_int) -> bool {
  let mut etty: emacs_tty = std::mem::zeroed ();

  emacs_get_tty (fd, &mut etty);
  // C 1363-1371: `#ifndef DOS_NT` is live and its `#ifdef TABDLY` too.
  {
    // C 1364-1368: the `#else /* not TABDLY */` arm is carried.
    #[cfg (not (feature = "no_tabdly"))]
    {
      return (etty.main.c_oflag & TABDLY) != TAB3;
    }
    #[cfg (feature = "no_tabdly")]
    {
      return true;
    }
  }
  // C 1369-1371's `#else /* DOS_NT */` arm is carried inside the function.
  #[cfg (feature = "dos_nt")]
  {
    let _ = fd;
    return false;
  }
}

/* Discard echoing.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn suppress_echo_on_tty (fd: c_int) {
  let mut etty: emacs_tty = std::mem::zeroed ();

  emacs_get_tty (fd, &mut etty);
  // C 1382-1388: `#ifdef DOS_NT` is dead; carried.
  #[cfg (feature = "dos_nt")]
  {
    /* Set raw input mode.  */
    etty.main = std::mem::zeroed ();
  }
  #[cfg (not (feature = "dos_nt"))]
  {
    etty.main.c_lflag &= !ICANON;	/* Disable buffering */
    etty.main.c_lflag &= !ECHO;	/* Disable echoing */
  }
  emacs_set_tty (fd, &mut etty, false);
}

/* Get terminal size from system.
   Store number of lines into *HEIGHTP and width into *WIDTHP.
   We store 0 if there's no valid information.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_tty_size (fd: c_int, widthp: *mut c_int, heightp: *mut c_int) {
  /* C 1399: `#if defined TIOCGWINSZ` is live.  */
  {
    /* BSD-style.  */
    let mut size: winsize = std::mem::zeroed ();

    if ioctl (fd, TIOCGWINSZ, &mut size) == -1
      { *widthp = 0; *heightp = 0; }
    else
      {
	*widthp = size.ws_col as c_int;
	*heightp = size.ws_row as c_int;
      }
  }
  // C 1412-1446: the chain's first live arm is TIOCGWINSZ (measured), so these are dead and
  // carried.
  #[cfg (feature = "tiocgsize")]
  {
    /* SunOS - style.  */
    let mut size: ttysize = std::mem::zeroed ();

    if ioctl (fd, TIOCGSIZE, &mut size) == -1
      { *widthp = 0; *heightp = 0; }
    else
      {
	*widthp = size.ts_cols;
	*heightp = size.ts_lines;
      }
  }
  #[cfg (feature = "windowsnt")]
  {
    let mut info: console_screen_buffer_info = std::mem::zeroed ();
    if GetConsoleScreenBufferInfo (GetStdHandle (STD_OUTPUT_HANDLE), &mut info) != 0
      {
	*widthp = (info.srWindow.Right - info.srWindow.Left + 1) as c_int;
	*heightp = (info.srWindow.Bottom - info.srWindow.Top + 1) as c_int;
      }
    else
      { *widthp = 0; *heightp = 0; }
  }
  #[cfg (feature = "msdos")]
  {
    *widthp = ScreenCols ();
    *heightp = ScreenRows ();
  }
  #[cfg (feature = "no_tty_size")]
  {
    *widthp = 0;
    *heightp = 0;
  }
}

/* Set the logical window size associated with descriptor FD
   to HEIGHT and WIDTH.  This is used mainly with ptys.
   Return a negative value on failure.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_window_size (fd: c_int, height: c_int, width: c_int) -> c_int {
  /* C 1456: `#ifdef TIOCSWINSZ` is live.  */
  {
    /* BSD-style.  */
    let mut size: winsize = std::mem::zeroed ();
    memset (&mut size as *mut winsize as *mut c_void, 0, std::mem::size_of::<winsize> ());
    size.ws_row = height as c_ushort;
    size.ws_col = width as c_ushort;

    return ioctl (fd, TIOCSWINSZ, &mut size);
  }
  // C 1466-1479: the `TIOCSSIZE` arm and the `#else` arm are dead (clang's chain takes
  // TIOCSWINSZ); carried inside the function so the checker sees them.
  #[cfg (feature = "tiocssize")]
  {
    /* SunOS - style.  */
    let mut size: ttysize = std::mem::zeroed ();
    memset (&mut size as *mut ttysize as *mut c_void, 0, std::mem::size_of::<ttysize> ());
    size.ts_lines = height;
    size.ts_cols = width;

    return ioctl (fd, TIOCGSIZE, &mut size);
  }
  #[cfg (feature = "no_tiocswinsz")]
  {
    let _ = (fd, height, width);
    return -1;
  }
}

/* Prepare all terminal devices for exiting Emacs. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_all_sys_modes () {
  let mut tty: *mut tty_display_info;
  {
    let mut tty_: *mut tty_display_info = tty_list;
    loop
      {
	tty = tty_;
	if tty.is_null () { break; }
	for tty in std::iter::once (tty) {
	  reset_sys_modes (tty);
	}
	tty_ = (*tty).next;
      }
  }
}

/* Prepare the terminal for closing it; move the cursor to the
   bottom of the frame, turn off interrupt-driven I/O, etc.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_sys_modes (tty_out: *mut tty_display_info) {
  if noninteractive
    {
      fflush (stdout);
      return;
    }

  /* C 1506-1576 is `#ifndef HAVE_ANDROID`: for the Android configuration C has no body here, so
     the block is excluded rather than compiled (the port had it as live text, status 4.712).  */
  #[cfg (not (feature = "have_android"))]
  {
    if !tty_bits! (term_initted, tty_out)
      { return; }

    if (*tty_out).output.is_null ()
      { return; }                     /* The tty is suspended. */

    /* Go to and clear the last line of the terminal. */

    cmgoto (tty_out, FrameRows! (tty_out) - 1, 0);

    /* Code adapted from tty_clear_end_of_line. */
    if !(*tty_out).TS_clr_line.is_null ()
      {
	emacs_tputs! (tty_out, (*tty_out).TS_clr_line, 1, cmputc);
      }
    else
      {			/* have to do it the hard way */
	tty_turn_off_insert (tty_out);

	let mut i: c_int = cursorX! (tty_out);
	while i < FrameCols! (tty_out) - 1
	  {
	    putc (b' ' as c_int, (*tty_out).output);
	    i += 1;
	  }
      }

    cmgoto (tty_out, FrameRows! (tty_out) - 1, 0);
    fflush ((*tty_out).output);

    if (*(*tty_out).terminal).reset_terminal_modes_hook.is_some ()
      { (*(*tty_out).terminal).reset_terminal_modes_hook.unwrap () ((*tty_out).terminal); }

    /* Avoid possible loss of output when changing terminal modes.  */
    while tcdrain (fileno ((*tty_out).output)) != 0 && *errno () == EINTR
      { continue; }

    // C 1540-1551: `#ifndef DOS_NT` is live.
    {
      // C 1541-1548: `# ifdef F_SETOWN` is live.
      {
	if interrupt_input
	  {
	    reset_sigio (fileno ((*tty_out).input));
	    fcntl (fileno ((*tty_out).input), F_SETOWN,
		   old_fcntl_owner[fileno ((*tty_out).input) as usize]);
	  }
      }
      fcntl (fileno ((*tty_out).input), F_SETFL,
	     fcntl (fileno ((*tty_out).input), F_GETFL, 0) & !O_NONBLOCK);
    }

    if !(*tty_out).old_tty.is_null ()
      { while emacs_set_tty (fileno ((*tty_out).input),
			     (*tty_out).old_tty as *mut emacs_tty, false) < 0 && *errno () == EINTR
	{ } }

    /* C 1558-1560: `#ifdef MSDOS` is dead; carried.  */
    #[cfg (feature = "msdos")]
    { dos_ttcooked (); }

    widen_foreground_group (fileno ((*tty_out).input));
  }
}

// C 1566: `#ifdef HAVE_PTYS` — live (HAVE_PTYS is 1).

/* Set up the proper status flags for use of a pty.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_pty (fd: c_int) {
  /* I'm told that TOICREMOTE does not mean control chars
     "can't be sent" but rather that they don't have
     input-editing or signaling effects.
     That should be good, because we have other ways
     to do those things in Emacs.
     However, telnet mode seems not to work on 4.2.
     So TIOCREMOTE is turned off now. */

  /* Under hp-ux, if TIOCREMOTE is turned on, some calls
     will hang.  In particular, the "timeout" feature (which
     causes a read to return if there is no data available)
     does this.  Also it is known that telnet mode will hang
     in such a way that Emacs must be stopped (perhaps this
     is the same problem).

     If TIOCREMOTE is turned off, then there is a bug in
     hp-ux which sometimes loses data.  Apparently the
     code which blocks the master process when the internal
     buffer fills up does not work.  Other than this,
     though, everything else seems to work fine.

     Since the latter lossage is more benign, we may as well
     lose that way.  -- cph */
  /* C 1596-1603: `#ifdef FIONBIO` and its `#if defined (UNIX98_PTYS)` are live.  */
  {
    {
      let mut on: c_int = 1;
      ioctl (fd, FIONBIO, &mut on);
    }
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_system_name () {
  if !build_details
    {
      /* Set system-name to nil so that the build is deterministic.  */
      Vsystem_name = Qnil ();
      return;
    }
  let mut hostname_alloc: *mut c_char = std::ptr::null_mut ();
  let mut hostname: *mut c_char;
  /* C's two arms are *preprocessor* text, not blocks, so C's `struct utsname uts` (C 1618) and
     `char hostname_buf[256]` (C 1622) live at function scope and `hostname` stays valid to the
     end of the function.  The port's `#[cfg]` arms *are* blocks, whose locals die at the closing
     brace - ASan reported the later read of the buffer as `stack-use-after-scope`, and the same
     shape would have hit `uts.nodename` in the other arm (PORTING.md P-113's class,
     docs/status.md 4.426).  The storage is therefore declared here, where C declares it.  */
  #[cfg (feature = "no_have_gethostname")]
  let mut uts: utsname = std::mem::zeroed ();
  #[cfg (not (feature = "no_have_gethostname"))]
  let mut hostname_buf: [c_char; 256] = [0; 256];
  #[cfg (not (feature = "no_have_gethostname"))]
  let mut hostname_size: ptrdiff_t = std::mem::size_of::<[c_char; 256]> () as ptrdiff_t;

  // C 1618-1643: `#ifndef HAVE_GETHOSTNAME` is dead (`HAVE_GETHOSTNAME` is 1); carried.
  #[cfg (feature = "no_have_gethostname")]
  {
    uname (&mut uts);
    hostname = uts.nodename.as_mut_ptr ();
  }
  #[cfg (not (feature = "no_have_gethostname"))]
  {
    hostname = hostname_buf.as_mut_ptr ();

    /* Try to get the host name; if the buffer is too short, try
       again.  Apparently, the only indication gethostname gives of
       whether the buffer was large enough is the presence or absence
       of a '\0' in the string.  Eech.  */
    loop
      {
	gethostname (hostname, (hostname_size - 1) as usize);
	*hostname.add ((hostname_size - 1) as usize) = 0;

	/* Was the buffer large enough for the '\0'?  */
	if (strlen (hostname) as ptrdiff_t) < hostname_size - 1
	  { break; }

	hostname_alloc = xpalloc (hostname_alloc as *mut c_void, &mut hostname_size, 1,
				  (PTRDIFF_MAX as usize).min (SIZE_MAX) as ptrdiff_t, 1)
				 as *mut c_char;
	hostname = hostname_alloc;
      }
  }
  let mut p: *mut c_char;
  p = hostname;
  while *p != 0
    {
      if *p == b' ' as c_char || *p == b'\t' as c_char
	{ *p = b'-' as c_char; }
      p = p.add (1);
    }
  if ! (STRINGP (Vsystem_name) && SBYTES (Vsystem_name) == (p as isize - hostname as isize)
	 && strcmp (SSDATA (Vsystem_name), hostname) == 0)
    { Vsystem_name = build_string (hostname); }
  xfree (hostname_alloc as *mut c_void);
}

// C 1653-1656: `sigset_t empty_mask;` and `static struct sigaction process_fatal_action;`.
// The declaration is `syssignal.h`'s (the mirror's `syssignal_decls` module); this unit
// defines the object, as C does.
#[unsafe(no_mangle)]
pub static mut empty_mask: sigset_t = sigset_t { __val: 0 };

static mut process_fatal_action: sigaction = unsafe { std::mem::zeroed () };

unsafe fn emacs_sigaction_flags () -> c_int {
  /* C 1661-1673: `#ifdef SA_RESTART` is live (SA_RESTART is 0x0002).  */
  {
    /* SA_RESTART causes interruptible functions with timeouts (e.g.,
       'select') to reset their timeout on some platforms (e.g.,
       HP-UX 11), which is not what we want.  Also, when Emacs is
       interactive, we don't want SA_RESTART because we need to poll
       for pending input so we need long-running syscalls to be interrupted
       after a signal that sets pending_signals.

       Non-interactive keyboard input goes through stdio, where we
       always want restartable system calls.  */
    if noninteractive
      { return SA_RESTART; }
  }
  return 0;
}

/* Store into *ACTION a signal action suitable for Emacs, with handler
   HANDLER.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_sigaction_init (action: *mut sigaction,
                                               handler: signal_handler_t) {
  sigemptyset (&mut (*action).sa_mask);

  /* When handling a signal, block nonfatal system signals that are caught
     by Emacs.  This makes race conditions less likely.  */
  sigaddset (&mut (*action).sa_mask, SIGALRM);
  /* C 1687-1689: `#ifdef SIGCHLD` is live.  */
  {
    sigaddset (&mut (*action).sa_mask, SIGCHLD);
  }
  /* C 1690-1692: `#ifdef SIGDANGER` is dead (this target has no SIGDANGER); carried.  */
  #[cfg (feature = "sigdanger")]
  {
    sigaddset (&mut (*action).sa_mask, SIGDANGER);
  }
  /* C 1693-1695: `#ifdef PROFILER_CPU_SUPPORT` is live (syssignal.h 54 defines it).  */
  {
    sigaddset (&mut (*action).sa_mask, SIGPROF);
  }
  /* C 1696-1698: `#ifdef SIGWINCH` is live.  */
  {
    sigaddset (&mut (*action).sa_mask, SIGWINCH);
  }
  if ! noninteractive
    {
      sigaddset (&mut (*action).sa_mask, SIGINT);
      sigaddset (&mut (*action).sa_mask, SIGQUIT);
      /* C 1703-1707: `#ifdef USABLE_SIGIO` is live.  */
      {
	/* C 1703-1707: `#ifdef USABLE_SIGIO` is live; the `USABLE_SIGPOLL` arm is carried.  */
	#[cfg (not (feature = "usable_sigpoll"))]
	{
	  sigaddset (&mut (*action).sa_mask, SIGIO);
	}
	#[cfg (feature = "usable_sigpoll")]
	{
	  sigaddset (&mut (*action).sa_mask, SIGPOLL);
	}
      }
    }

  (*action).__sigaction_u.__sa_handler = handler;
  (*action).sa_flags = emacs_sigaction_flags ();
}

// C 1714-1716: `#ifdef FORWARD_SIGNAL_TO_MAIN_THREAD` is live (syssignal.h 39).
/* C 407/1196 declare `static pthread_t main_thread_id;`, i.e. zero-initialized.  The port's
   `pthread_t` is an opaque pointer on the frozen target and `long` on Android
   (`include/systhread.rs` 62-70), so the initializer is written as a cast rather than as
   `null_mut` - the mirror's Android arm made the pointer spelling an E0308 (measured).  */
static mut main_thread_id: pthread_t = 0 as pthread_t;

/* SIG has arrived at the current process.  Deliver it to the main
   thread, which should handle it with HANDLER.  (Delivering the
   signal to some other thread might not work if the other thread is
   about to exit.)

   If we are on the main thread, handle the signal SIG with HANDLER.
   Otherwise, redirect the signal to the main thread, blocking it from
   this thread.  POSIX says any thread can receive a signal that is
   associated with a process, process group, or asynchronous event.
   On GNU/Linux the main thread typically gets a process signal unless
   it's blocked, but other systems (FreeBSD at least) can deliver the
   signal to other threads.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn deliver_process_signal (sig: c_int, handler: signal_handler_t) {
  /* Preserve errno, to avoid race conditions with signal handlers that
     might change errno.  Races can occur even in single-threaded hosts.  */
  let old_errno: c_int = *errno ();

  let mut on_main_thread: bool = true;
  /* C 1738-1748: `#ifdef FORWARD_SIGNAL_TO_MAIN_THREAD` is live.  */
  {
    if pthread_equal (pthread_self (), main_thread_id) == 0
      {
	let mut blocked: sigset_t = std::mem::zeroed ();
	sigemptyset (&mut blocked);
	sigaddset (&mut blocked, sig);
	pthread_sigmask (SIG_BLOCK, &blocked, std::ptr::null_mut ());
	pthread_kill (main_thread_id, sig);
	on_main_thread = false;
      }
  }
  if on_main_thread
    { handler.unwrap () (sig); }

  *errno () = old_errno;
}

/* Static location to save a fatal backtrace in a thread.
   FIXME: If two subsidiary threads fail simultaneously, the resulting
   backtrace may be garbage.  */
// C 1758: `enum { BACKTRACE_LIMIT_MAX = 500 };`
pub const BACKTRACE_LIMIT_MAX: usize = 500;
static mut thread_backtrace_buffer: [*mut c_void; BACKTRACE_LIMIT_MAX + 1] =
  [std::ptr::null_mut (); BACKTRACE_LIMIT_MAX + 1];
static mut thread_backtrace_npointers: c_int = 0;

/* SIG has arrived at the current thread.
   If we are on the main thread, handle the signal SIG with HANDLER.
   Otherwise, this is a fatal error in the handling thread.  */
unsafe extern "C" fn deliver_thread_signal (sig: c_int, handler: signal_handler_t) {
  let old_errno: c_int = *errno ();

  /* C 1770-1782: `#ifdef FORWARD_SIGNAL_TO_MAIN_THREAD` - live for the frozen configuration,
     absent for Android (`HAVE_BACKTRACE` is absent there too, so its `backtrace` call would not
     link; C's macro is what C itself tests).  */
  #[cfg (feature = "forward_signal_to_main_thread")]
  {
    if pthread_equal (pthread_self (), main_thread_id) == 0
      {
	thread_backtrace_npointers =
	  backtrace (thread_backtrace_buffer.as_mut_ptr (), BACKTRACE_LIMIT_MAX as c_int);
	sigaction (sig, &process_fatal_action, std::ptr::null_mut ());
	pthread_kill (main_thread_id, sig);

	/* Avoid further damage while the main thread is exiting.  */
	while true
	  { sigsuspend (&empty_mask); }
      }
  }

  handler.unwrap () (sig);
  *errno () = old_errno;
}

/* Handle bus errors, invalid instruction, etc.  */
unsafe extern "C" fn handle_fatal_signal (sig: c_int) {
  terminate_due_to_signal (sig, 40);
}

unsafe extern "C" fn deliver_fatal_signal (sig: c_int) {
  deliver_process_signal (sig, Some (handle_fatal_signal));
}

unsafe extern "C" fn deliver_fatal_thread_signal (sig: c_int) {
  deliver_thread_signal (sig, Some (handle_fatal_signal));
}

unsafe extern "C" fn handle_arith_signal (sig: c_int) -> ! {
  let _ = sig;
  pthread_sigmask (SIG_SETMASK, &empty_mask, std::ptr::null_mut ());
  xsignal0 (Qarith_error ());
}

// C 1814-1846: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY && defined HAVE_MMAP`
// is dead; carried.
#[cfg (feature = "have_android_mmap")]
unsafe extern "C" fn handle_sigbus (sig: c_int, siginfo: *mut siginfo_t, arg: *mut c_void) {
  /* If this arrives during sfntfont_open, then Emacs may be
     screwed.  */

  let _ = arg;
  if sfntfont_detect_sigbus ((*siginfo).si_addr)
    { return; }

  handle_fatal_signal (sig);
}

/* Try to set up SIGBUS handling for the sfnt font driver.
   Value is 1 upon failure, 0 otherwise.  */

#[cfg (feature = "have_android_mmap")]
unsafe extern "C" fn init_sigbus () -> c_int {
  let mut sa: sigaction = std::mem::zeroed ();

  sigfillset (&mut sa.sa_mask);
  sa.__sigaction_u.__sa_sigaction =
    Some (std::mem::transmute::<unsafe extern "C" fn (c_int, *mut siginfo_t, *mut c_void),
				unsafe extern "C" fn (c_int, *mut c_void, *mut c_void)>
	    (handle_sigbus));
  sa.sa_flags = SA_SIGINFO;

  if sigaction (SIGBUS, &sa, std::ptr::null_mut ()) != 0
    { return 1; }

  return 0;
}

// C 1848-1977: `#if defined HAVE_STACK_OVERFLOW_HANDLING && !defined WINDOWSNT` is live
// (`HAVE_STACK_OVERFLOW_HANDLING` is 1).

/* Alternate stack used by SIGSEGV handler below.  */

/* Storage for the alternate signal stack.
   64 KiB is not too large for Emacs, and is large enough
   for all known platforms.  Smaller sizes may run into trouble.
   For example, libsigsegv 2.6 through 2.8 have a bug where some
   architectures use more than the Linux default of an 8 KiB alternate
   stack when deciding if a fault was caused by stack overflow.  */
static mut sigsegv_stack: [max_align_t; (64 * 1024
					 + std::mem::size_of::<max_align_t> () - 1)
					/ std::mem::size_of::<max_align_t> ()] =
  [0; (64 * 1024 + std::mem::size_of::<max_align_t> () - 1)
      / std::mem::size_of::<max_align_t> ()];

/* Return true if SIGINFO indicates a stack overflow.  */

unsafe extern "C" fn stack_overflow (siginfo: *mut siginfo_t) -> bool {
  if !attempt_stack_overflow_recovery
    { return false; }

  /* In theory, a more-accurate heuristic can be obtained by using
     GNU/Linux pthread_getattr_np along with POSIX pthread_attr_getstack
     and pthread_attr_getguardsize to find the location and size of the
     guard area.  In practice, though, these functions are so hard to
     use reliably that they're not worth bothering with.  E.g., see:
     https://sourceware.org/bugzilla/show_bug.cgi?id=16291
     Other operating systems also have problems, e.g., Solaris's
     stack_violation function is tailor-made for this problem, but it
     doesn't work on Solaris 11.2 x86-64 with a 32-bit executable.

     GNU libsigsegv is overkill for Emacs; otherwise it might be a
     candidate here.  */

  if siginfo.is_null ()
    { return false; }

  /* The faulting address.  */
  let addr: *mut c_char = (*siginfo).si_addr;
  if addr.is_null ()
    { return false; }

  /* The known top and bottom of the stack.  The actual stack may
     extend a bit beyond these boundaries.  */
  let bot: *const c_char = stack_bottom! ();
  let top: *const c_char = (*current_thread).stack_top as *const c_char;

  /* Log base 2 of the stack heuristic ratio.  This ratio is the size
     of the known stack divided by the size of the guard area past the
     end of the stack top.  The heuristic is that a bad address is
     considered to be a stack overflow if it occurs within
     stacksize>>LG_STACK_HEURISTIC bytes above the top of the known
     stack.  This heuristic is not exactly correct but it's good
     enough in practice.  */
  /* C 1906: `enum { LG_STACK_HEURISTIC = 8 };` */
  let lg_stack_heuristic: isize = 8;

  if bot < top
    { return 0 <= addr as isize - top as isize
	&& (addr as isize - top as isize) < ((top as isize - bot as isize) >> lg_stack_heuristic); }
  else
    { return 0 <= top as isize - addr as isize
	&& (top as isize - addr as isize) < ((bot as isize - top as isize) >> lg_stack_heuristic); }
}

/* Signal handler for SIGSEGV before our new handler was installed.  */
static mut old_sigsegv_handler: sigaction = unsafe { std::mem::zeroed () };

/* Attempt to recover from SIGSEGV caused by C stack overflow.  */

unsafe extern "C" fn handle_sigsegv (sig: c_int, siginfo: *mut siginfo_t, arg: *mut c_void) {
  /* Hard GC error may lead to stack overflow caused by
     too nested calls to mark_object.  No way to survive.  */
  let mut fatal: bool = gc_in_progress;

  /* C 1924-1927: `#ifdef FORWARD_SIGNAL_TO_MAIN_THREAD` is live.  */
  {
    if !fatal && pthread_equal (pthread_self (), main_thread_id) == 0
      { fatal = true; }
  }

  let _ = arg;
  if !fatal && stack_overflow (siginfo)
    { siglongjmp (&raw mut return_to_command_loop, 1); }

  /* C 1932-1939: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` is dead; carried.  */
  #[cfg (feature = "have_android")]
  {
    /* Tombstones (crash reports with stack traces) won't be generated on
       Android unless the original SIGSEGV handler is installed and the
       signal is resent, such as by returning from the first signal
       handler called.  */
    sigaction (SIGSEGV, &old_sigsegv_handler, std::ptr::null_mut ());
    return;
  }

  /* Otherwise we can't do anything with this.  */
  deliver_fatal_thread_signal (sig);
}

/* Return true if we have successfully set up SIGSEGV handler on alternate
   stack.  Otherwise we just treat SIGSEGV among the rest of fatal signals.  */

unsafe extern "C" fn init_sigsegv () -> bool {
  let mut sa: sigaction = std::mem::zeroed ();
  let mut ss: stack_t = std::mem::zeroed ();

  ss.ss_sp = sigsegv_stack.as_mut_ptr () as *mut c_void;
  ss.ss_size = std::mem::size_of_val (&sigsegv_stack);
  ss.ss_flags = 0;
  if sigaltstack (&ss, std::ptr::null_mut ()) < 0
    { return false; }

  sigfillset (&mut sa.sa_mask);
  /* signal.rs's union member spells the `siginfo_t *` parameter as `*mut c_void` (that
     mirror cannot name `siginfo_t`, P-72 rule 3), so C's assignment needs the ABI-equal
     transmute — the same conversion C's own prototype gets.  */
  sa.__sigaction_u.__sa_sigaction =
    Some (std::mem::transmute::<unsafe extern "C" fn (c_int, *mut siginfo_t, *mut c_void),
				unsafe extern "C" fn (c_int, *mut c_void, *mut c_void)>
	    (handle_sigsegv));
  sa.sa_flags = SA_SIGINFO | SA_ONSTACK | emacs_sigaction_flags ();
  if sigaction (SIGSEGV, &sa, &mut old_sigsegv_handler) < 0
    { return false; }

  return true;
}

// C 1969-1977's `#else /* not HAVE_STACK_OVERFLOW_HANDLING or WINDOWSNT */` arm is dead
// (clang takes the live arm above); carried.
#[cfg (feature = "no_stack_overflow_handling")]
unsafe extern "C" fn init_sigsegv () -> bool {
  return false;
}

unsafe extern "C" fn deliver_arith_signal (sig: c_int) {
  /* C's `handle_arith_signal` is `static AVOID` (never returns), while the handler slot's
     type is the plain `void (*) (int)`; C makes that conversion implicitly, so the port
     writes the fn-pointer transmute (the same ABI, the same never-returning body).  */
  deliver_thread_signal (sig, Some (std::mem::transmute::<unsafe extern "C" fn (c_int) -> !,
							   unsafe extern "C" fn (c_int)>
				     (handle_arith_signal)));
}

// C 1985-2002: `#ifdef SIGDANGER` is dead; carried.
#[cfg (feature = "sigdanger")]
unsafe extern "C" fn handle_danger_signal (sig: c_int) {
  let _ = sig;
  malloc_warning (c"Operating system warns that virtual memory is running low.\n".as_ptr ());

  /* It might be unsafe to call do_auto_save now.  */
  force_auto_save_soon ();
}

#[cfg (feature = "sigdanger")]
unsafe extern "C" fn deliver_danger_signal (sig: c_int) {
  deliver_process_signal (sig, Some (handle_danger_signal));
}

/* Treat SIG as a terminating signal, unless it is already ignored and
   we are in --batch mode.  Among other things, this makes nohup work.  */
unsafe extern "C" fn maybe_fatal_sig (sig: c_int) {
  let mut catch_sig: bool = !noninteractive;
  if !catch_sig
    {
      let mut old_action: sigaction = std::mem::zeroed ();
      sigaction (sig, std::ptr::null (), &mut old_action);
      catch_sig = std::mem::transmute::<signal_handler_t, usize>
		    (old_action.__sigaction_u.__sa_handler) != SIG_IGN;
    }
  if catch_sig
    { sigaction (sig, &process_fatal_action, std::ptr::null_mut ()); }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_signals () {
  let mut thread_fatal_action: sigaction = std::mem::zeroed ();
  let mut action: sigaction = std::mem::zeroed ();

  sigemptyset (&mut empty_mask);

  /* C 2028-2030: `#ifdef FORWARD_SIGNAL_TO_MAIN_THREAD` is live.  */
  {
    main_thread_id = pthread_self ();
  }

  sigfillset (&mut process_fatal_action.sa_mask);
  process_fatal_action.__sigaction_u.__sa_handler = Some (deliver_fatal_signal);
  process_fatal_action.sa_flags = emacs_sigaction_flags ();

  sigfillset (&mut thread_fatal_action.sa_mask);
  thread_fatal_action.__sigaction_u.__sa_handler = Some (deliver_fatal_thread_signal);
  thread_fatal_action.sa_flags = process_fatal_action.sa_flags;

  /* SIGINT may need special treatment on MS-Windows.  See
     https://lists.gnu.org/r/emacs-devel/2010-09/msg01062.html
     Please update the doc of kill-emacs, kill-emacs-hook, and
     NEWS if you change this.  */

  maybe_fatal_sig (SIGHUP);
  maybe_fatal_sig (SIGINT);
  maybe_fatal_sig (SIGTERM);

  /* Emacs checks for write errors, so it can safely ignore SIGPIPE.
     However, in batch mode leave SIGPIPE alone, as that causes Emacs
     to behave more like typical batch applications do.  */
  if ! noninteractive
    { signal (SIGPIPE, SIG_IGN); }

  sigaction (SIGQUIT, &process_fatal_action, std::ptr::null_mut ());
  /* C 2056-2058: `#ifndef __vax__` is live.  */
  {
    sigaction (SIGILL, &thread_fatal_action, std::ptr::null_mut ());
  }
  sigaction (SIGTRAP, &thread_fatal_action, std::ptr::null_mut ());

  /* Typically SIGFPE is thread-specific and is fatal, like SIGILL.
     But on a non-IEEE host SIGFPE can come from a trap in the Lisp
     interpreter's floating point operations, so treat SIGFPE as an
     arith-error if it arises in the main thread.  */
  if IEEE_FLOATING_POINT
    { sigaction (SIGFPE, &thread_fatal_action, std::ptr::null_mut ()); }
  else
    {
      emacs_sigaction_init (&mut action, Some (deliver_arith_signal));
      sigaction (SIGFPE, &action, std::ptr::null_mut ());
      /* C 2071-2075: `#ifdef __vax__` is dead; carried.  */
      #[cfg (feature = "vax")]
      {
	sigaction (SIGILL, &action, std::ptr::null_mut ());
      }
    }

  /* SIGUSR1 and SIGUSR2 are used internally by the android_select
     function.  */
  /* C 2080-2087: `#if !defined HAVE_ANDROID`.  On Android the arm is **dead**: making SIGUSR1 a
     user signal installs the Lisp-level handler over `android_handle_poll_signal`, after which
     the poll signal is swallowed instead of setting `cancel_signal_received`, and the select
     thread waits forever in `sigwait` (measured: status 4.958).  */
  #[cfg (not (feature = "have_android"))]
  {
    add_user_signal (SIGUSR1, c"sigusr1".as_ptr ());
    add_user_signal (SIGUSR2, c"sigusr2".as_ptr ());
  }

  sigaction (SIGABRT, &thread_fatal_action, std::ptr::null_mut ());
  /* C 2090-2163: the platform-specific signals.  SIGPRE/SIGORE/SIGUME/SIGDLK/SIGCPULIM/
     SIGGRANT/SIGMIGRATE/SIGMSG/SIGRETRACT/SIGSAK/SIGSOUND/SIGTALRM belong to other systems and
     are dead here (measured); SIGIOT, SIGEMT, SIGBUS, SIGSYS, SIGPROF, SIGVTALRM, SIGXCPU and
     SIGXFSZ are live.  `SIGDANGER` is dead.  */
  #[cfg (feature = "sigpre")]
  { sigaction (SIGPRE, &thread_fatal_action, std::ptr::null_mut ()); }
  #[cfg (feature = "sigore")]
  { sigaction (SIGORE, &thread_fatal_action, std::ptr::null_mut ()); }
  #[cfg (feature = "sigume")]
  { sigaction (SIGUME, &thread_fatal_action, std::ptr::null_mut ()); }
  #[cfg (feature = "sigdlk")]
  { sigaction (SIGDLK, &process_fatal_action, std::ptr::null_mut ()); }
  #[cfg (feature = "sigcpulim")]
  { sigaction (SIGCPULIM, &process_fatal_action, std::ptr::null_mut ()); }
  {
    sigaction (SIGIOT, &thread_fatal_action, std::ptr::null_mut ());
  }
  {
    sigaction (SIGEMT, &thread_fatal_action, std::ptr::null_mut ());
  }
  {
    /* C 2112-2116: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY && defined
       HAVE_MMAP` is dead; carried.  */
    #[cfg (feature = "have_android_mmap")]
    let mut sigbus_done: bool = false;
    #[cfg (feature = "have_android_mmap")]
    { sigbus_done = init_sigbus () != 0; }
    #[cfg (not (feature = "have_android_mmap"))]
    let sigbus_done: bool = true;
    if sigbus_done
      { sigaction (SIGBUS, &thread_fatal_action, std::ptr::null_mut ()); }
  }
  if !init_sigsegv ()
    { sigaction (SIGSEGV, &thread_fatal_action, std::ptr::null_mut ()); }
  {
    sigaction (SIGSYS, &thread_fatal_action, std::ptr::null_mut ());
  }
  sigaction (SIGTERM, &process_fatal_action, std::ptr::null_mut ());
  {
    signal (SIGPROF, SIG_IGN);
  }
  {
    sigaction (SIGVTALRM, &process_fatal_action, std::ptr::null_mut ());
  }
  {
    sigaction (SIGXCPU, &process_fatal_action, std::ptr::null_mut ());
  }
  {
    sigaction (SIGXFSZ, &process_fatal_action, std::ptr::null_mut ());
  }

  #[cfg (feature = "sigdanger")]
  {
    /* This just means available memory is getting low.  */
    emacs_sigaction_init (&mut action, Some (deliver_danger_signal));
    sigaction (SIGDANGER, &action, std::ptr::null_mut ());
  }

  /* AIX-specific signals.  */
  #[cfg (feature = "siggrant")]
  { sigaction (SIGGRANT, &process_fatal_action, std::ptr::null_mut ()); }
  #[cfg (feature = "sigmigrate")]
  { sigaction (SIGMIGRATE, &process_fatal_action, std::ptr::null_mut ()); }
  #[cfg (feature = "sigmsg")]
  { sigaction (SIGMSG, &process_fatal_action, std::ptr::null_mut ()); }
  #[cfg (feature = "sigretract")]
  { sigaction (SIGRETRACT, &process_fatal_action, std::ptr::null_mut ()); }
  #[cfg (feature = "sigsak")]
  { sigaction (SIGSAK, &process_fatal_action, std::ptr::null_mut ()); }
  #[cfg (feature = "sigsound")]
  { sigaction (SIGSOUND, &process_fatal_action, std::ptr::null_mut ()); }
  #[cfg (feature = "sig_talrm")]
  { sigaction (SIGTALRM, &thread_fatal_action, std::ptr::null_mut ()); }
}

// C 2166-2201: the `HAVE_RANDOM`/`RAND_BITS`/`random` selection.  The frozen configuration
// defines `HAVE_RANDOM` (macros.tsv 1927), so clang compiles the `srandom` arm of
// `set_random_seed` and `RAND_BITS` is 31; every other arm is dead and carried.
//
// C 2166-2170's `#ifndef HAVE_RANDOM` + `#ifdef random` are dead.
/// C 2168's `#define HAVE_RANDOM` (the dead `#ifdef random` arm's empty body).
#[cfg (feature = "no_have_random")]
pub const HAVE_RANDOM: () = ();
/* Figure out how many bits the system's random number generator uses.
   `random' and `lrand48' are assumed to return 31 usable bits.
   BSD `rand' returns a 31 bit value but the low order bits are unusable;
   so we'll shift it and treat it like the 15-bit USG `rand'.  */

// C 2177-2201's `#ifndef RAND_BITS` is live, and inside it the `#ifdef HAVE_RANDOM` arm too.
/// C 2179's `# define RAND_BITS 31` (the live arm).  The three later definitions
/// (`RAND_BITS 15` and the two `random` spellings) belong to the dead arms, which are
/// carried below; `RAND_BITS` has several definitions, so `tools/extract-abi.py` compares
/// its presence only.
pub const RAND_BITS: usize = 31;
/// C 2183's `# define random lrand48` (the dead `HAVE_LRAND48` arm) and C 2187/2193's
/// `# define random rand` (the dead 15-bit arms), plus C 2190/2195's
/// `# define random() (rand () >> 16)`.  C's `random ()` call site is C 2248/2264/2272's
/// `random ()`, which is `srandom`'s counterpart on this target — the port keeps the name as
/// a function of the live arm (the P-17 shape for a value-like macro).
#[cfg (feature = "no_rand_bits_lrand48")]
pub const random_LRAND48: c_int = 0;
#[cfg (feature = "rand_32767")]
pub const random_RAND: c_int = 0;
#[cfg (feature = "rand_2g")]
pub const RAND_BITS_15: c_int = 15;

// C 2203-2212's three `set_random_seed` definitions.
/// C 2205's `typedef unsigned int random_seed;` and its `set_random_seed` — the live arm
/// (`HAVE_RANDOM` is defined).
pub type random_seed = c_uint;
/// C 2205: `static void set_random_seed (random_seed arg) { srandom (arg); }` — the live arm
/// (`HAVE_RANDOM` is defined).  C's `static` is real even though the inventory's row lost the
/// marker (the scanner read the `typedef` line before the declarator); the reference object
/// exports no `_set_random_seed`, so the port keeps C's internal linkage (docs/status.md 3.67).
pub unsafe fn set_random_seed (arg: random_seed) { srandom (arg); }

/// C 2206-2208: the `HAVE_LRAND48` arm (`typedef long int random_seed; srand48 (arg)`).
#[cfg (feature = "no_have_random")]
pub type random_seed_lrand48 = c_long;
#[cfg (feature = "no_have_random")]
pub unsafe fn set_random_seed (arg: random_seed_lrand48) { srand48 (arg); }

/// C 2209-2212: the `#else` arm (`typedef unsigned int random_seed; srand (arg)`).
#[cfg (feature = "no_have_random_no_lrand48")]
pub type random_seed_rand = c_uint;
#[cfg (feature = "no_have_random_no_lrand48")]
pub unsafe fn set_random_seed (arg: random_seed_rand) { srand (arg); }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn seed_random (seed: *mut c_void, seed_size: ptrdiff_t) {
  let mut arg: random_seed = 0;
  let argp: *mut c_uchar = &mut arg as *mut random_seed as *mut c_uchar;
  let seedp: *mut c_uchar = seed as *mut c_uchar;
  let mut i: ptrdiff_t = 0;
  while i < seed_size
    {
      *argp.add ((i % std::mem::size_of::<random_seed> () as ptrdiff_t) as usize) ^=
	*seedp.add (i as usize);
      i += 1;
    }
  set_random_seed (arg);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_random () {
  let mut v: random_seed = 0;
  let mut success: bool = false;

  /* First, try seeding the PRNG from the operating system's entropy
     source.  This approach is both fast and secure.  */
  // C 2233-2239: `#ifdef WINDOWSNT` is dead; carried.
  #[cfg (feature = "windowsnt")]
  {
    /* FIXME: Perhaps getrandom can be used here too?  */
    success = w32_init_random (&mut v as *mut random_seed as *mut c_void,
			       std::mem::size_of::<random_seed> ()) == 0;
  }
  #[cfg (not (feature = "windowsnt"))]
  {
    static_assert! (std::mem::size_of::<random_seed> () <= 256);
    success = getrandom (&mut v as *mut random_seed as *mut c_void,
			 std::mem::size_of::<random_seed> (), 0)
	      == std::mem::size_of::<random_seed> () as ssize_t;
  }

  /* If that didn't work, just use the current time value and PID.
     It's at least better than XKCD 221.  */
  if !success
    {
      let t: timespec = current_timespec ();
      v = getpid () as c_uint ^ t.tv_sec as c_uint ^ t.tv_nsec as c_uint;
    }

  set_random_seed (v);
}

/*
 * Return a nonnegative random integer out of whatever we've got.
 * It contains enough bits to make a random (signed) Emacs fixnum.
 * This suffices even for a 64-bit architecture with a 15-bit rand.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_random () -> EMACS_INT {
  let mut val: EMACS_UINT = 0;
  let mut i: c_int;
  i = 0;
  while i < ((FIXNUM_BITS + RAND_BITS - 1) / RAND_BITS) as c_int
    {
      val = (random () as EMACS_UINT ^ (val << RAND_BITS)
	     ^ (val >> (EMACS_INT_WIDTH - RAND_BITS)));
      i += 1;
    }
  val ^= val >> (EMACS_INT_WIDTH - FIXNUM_BITS);
  return (val & INTMASK as EMACS_UINT) as EMACS_INT;
}

/* Return a random unsigned long.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_random_ulong () -> c_ulong {
  let mut r: c_ulong = 0;
  let mut i: c_int = 0;
  while i < (ULONG_WIDTH + RAND_BITS as c_int - 1) / RAND_BITS as c_int
    {
      r = random () as c_ulong ^ (r << RAND_BITS)
	  ^ (r >> (ULONG_WIDTH - RAND_BITS as c_int));
      i += 1;
    }
  return r;
}

// C 2279-2320: `#ifndef HAVE_SNPRINTF` — dead (`HAVE_SNPRINTF` is 1); carried.
/* Approximate snprintf as best we can on ancient hosts that lack it.  */
#[cfg (feature = "no_have_snprintf")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snprintf (buf: *mut c_char, bufsize: usize, format: *const c_char,
                                   mut args: ...) -> c_int {
  let size: ptrdiff_t = min (bufsize as ptrdiff_t, PTRDIFF_MAX);
  let mut nbytes: ptrdiff_t = size - 1;
  /* C's `va_list ap;` is the port's `VaList` (PORTING.md P-04: the `...` parameter *is* the
     `VaList`), and `va_start`/`va_end` have no counterpart for the same reason
     (`docs/ports/doprnt.md`; `doprnt.rs`'s own note).  */
  let mut ap: VaList<'_> = args;

  if size != 0
    {
      nbytes = doprnt (buf, size, format, 0, ap);
    }

  if nbytes == size - 1
    {
      /* Calculate the length of the string that would have been created
	 had the buffer been large enough.  */
      let mut stackbuf: [c_char; 4000] = [0; 4000];
      let mut b: *mut c_char = stackbuf.as_mut_ptr ();
      let mut bsize: ptrdiff_t = std::mem::size_of::<[c_char; 4000]> () as ptrdiff_t;
      nbytes = evxprintf (&mut b, &mut bsize, stackbuf.as_mut_ptr (), -1, format, ap.clone ());
      if b != stackbuf.as_mut_ptr ()
	{ xfree (b as *mut c_void); }
    }

  if INT_MAX < nbytes
    {
      /* C 2311-2315: `#ifdef EOVERFLOW` is live; the `#else` arm is carried.  */
      #[cfg (feature = "eoverflow")]
      {
	*errno () = EOVERFLOW;
      }
      #[cfg (feature = "no_eoverflow")]
      {
	*errno () = EDOM;
      }
      return -1;
    }
  return nbytes as c_int;
}

/* If a backtrace is available, output the top lines of it to stderr.
   Do not output more than BACKTRACE_LIMIT or BACKTRACE_LIMIT_MAX lines.
   This function may be called from a signal handler, so it should
   not invoke async-unsafe functions like malloc.

   If BACKTRACE_LIMIT is -1, initialize tables that 'backtrace' uses
   but do not output anything.  This avoids some problems that can
   otherwise occur if the malloc arena is corrupted before 'backtrace'
   is called, since 'backtrace' may call malloc if the tables are not
   initialized.

   If the static variable THREAD_BACKTRACE_NPOINTERS is nonzero, a
   fatal error has occurred in some other thread; generate a thread
   backtrace instead, ignoring BACKTRACE_LIMIT.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_backtrace (backtrace_limit: c_int) {
  let mut main_backtrace_buffer: [*mut c_void; BACKTRACE_LIMIT_MAX + 1] =
    [std::ptr::null_mut (); BACKTRACE_LIMIT_MAX + 1];
  let bounded_limit: c_int =
    (backtrace_limit as isize).min (BACKTRACE_LIMIT_MAX as isize) as c_int;
  let buffer: *mut *mut c_void;
  let npointers: c_int;

  if thread_backtrace_npointers != 0
    {
      buffer = thread_backtrace_buffer.as_mut_ptr ();
      npointers = thread_backtrace_npointers;
    }
  else
    {
      buffer = main_backtrace_buffer.as_mut_ptr ();

      /* Work around 'backtrace' bug; see Bug#19959 and glibc bug#18084.  */
      if bounded_limit < 0
	{
	  backtrace (buffer, 1);
	  return;
	}

      npointers = backtrace (buffer, bounded_limit + 1);
    }

  if npointers != 0
    {
      emacs_write (STDERR_FILENO, c"Backtrace:\n".as_ptr () as *const c_void, 11);
      backtrace_symbols_fd (buffer, npointers, STDERR_FILENO);
      if bounded_limit < npointers
	{ emacs_write (STDERR_FILENO, c"...\n".as_ptr () as *const c_void, 4); }
    }
}

/* Rename directory SRCFD's entry SRC to directory DSTFD's entry DST.
   This is like renameat except that it fails if DST already exists,
   or if this operation is not supported atomically.  Return 0 if
   successful, -1 (setting errno) otherwise.  */
// C 2376-2378: the `#ifndef HAVE_ANDROID` arm is live, so C's `static` keyword applies; the
// inventory's row lost the marker (the scanner read the `#endif` line before the declarator)
// and the port had given the definition an external name - the reference object exports no
// `_renameat_noreplace` (docs/status.md 3.67) ... because C 2377-2379 makes it `static` unless
// `HAVE_ANDROID`: `androidvfs.c` 894 calls it, so on that target C *does* export it and the port
// must too (status 4.724).  `cfg_attr` keeps one definition with C's two linkages.
#[cfg_attr (feature = "have_android", unsafe(no_mangle))]
pub unsafe fn renameat_noreplace (srcfd: c_int, src: *const c_char, dstfd: c_int,
                                  dst: *const c_char) -> c_int {
  /* C 2382-2396 is a four-arm chain and the arms are per *target*, so each one carries its own
     condition instead of the frozen target's arm standing for all of them: with
     `#elif defined RENAME_EXCL` (C 2407-2408) unconditional, the Android build imported
     `renameatx_np` - a name bionic's `<stdio.h>` does not declare (checked in the NDK's headers)
     and its libc does not export (the device reading lists it among the 53) - while C itself
     compiles C 2409's `#else`, `errno = ENOSYS; return -1;`.  `RENAME_EXCL` is a *header* macro:
     it is in the frozen target's macro table (0x00000004) and in neither the Android
     configuration nor bionic, which is why `tools/gen-android-config.sh` measures it with the
     target's own compiler and the arm is gated on C's macro: `feature = "not_rename_excl"` is
     exactly `#ifndef RENAME_EXCL`, and the frozen target sets no features at all.  */
  #[cfg (not (feature = "not_rename_excl"))]
  {
    return renameatx_np (srcfd, src, dstfd, dst, RENAME_EXCL);
  }
  #[cfg (feature = "have_renameat2")]
  {
    return renameat2 (srcfd, src, dstfd, dst, RENAME_NOREPLACE);
  }
  #[cfg (feature = "sys_renameat2")]
  {
    /* Linux kernel 3.15 (2014) or later, with glibc 2.27 (2018) or earlier.  */
    return syscall (SYS_renameat2, srcfd, src, dstfd, dst, RENAME_NOREPLACE);
  }
  // C 2409-2416's `#else` - the arm Android and any other target without one of the three
  // features above takes; `no_rename_excl` was a gate no configuration produces.
  #[cfg (all (feature = "not_rename_excl", not (feature = "have_renameat2"),
              not (feature = "sys_renameat2")))]
  {
    #[cfg (feature = "windowsnt")]
    {
      if srcfd == AT_FDCWD && dstfd == AT_FDCWD
	{ return sys_rename_replace (src, dst, 0); }
    }
    let _ = (srcfd, dstfd);
    *errno () = ENOSYS;
    return -1;
  }
}

/// The two names C's earlier `renameat_noreplace` arms use (Linux's entry and its request
/// number); the arms are never compiled, and `RENAME_NOREPLACE` is Linux's 1.
#[cfg (any (feature = "have_renameat2", feature = "sys_renameat2"))]
pub const RENAME_NOREPLACE: c_int = 1;
#[cfg (any (feature = "have_renameat2", feature = "sys_renameat2"))]
pub const SYS_renameat2: c_int = 316;
#[cfg (any (feature = "have_renameat2", feature = "sys_renameat2"))]
unsafe extern "C" {
  pub fn renameat2 (olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int,
                    newpath: *const c_char, flags: c_uint) -> c_int;
  pub fn syscall (number: c_long, ...) -> c_long;
  pub fn sys_rename_replace (from: *const c_char, to: *const c_char, force: c_int) -> c_int;
}

/// `<stdio.h>`: `int renameatx_np (int, const char *, int, const char *, unsigned int);`
/// — Darwin's entry C 2388 calls, gated with the arm that calls it: bionic declares neither the
/// entry nor `RENAME_EXCL` (measured with the target's own preprocessor), and the device does not
/// export it.
#[cfg (not (feature = "not_rename_excl"))]
unsafe extern "C" {
  pub fn renameatx_np (fromfd: c_int, from: *const c_char, tofd: c_int, to: *const c_char,
                       flags: c_uint) -> c_int;
}

/// `<stdio.h>`: `#define RENAME_EXCL 0x00000004` (the frozen macro table); gated with the arm
/// that reads it, so the Android unit carries neither name - C's own Android unit does not.
#[cfg (not (feature = "not_rename_excl"))]
pub const RENAME_EXCL: c_uint = 0x00000004;

/* C 2399-2405: `#if !defined HAVE_NTGUI && !(defined HAVE_ANDROID && !defined
   ANDROID_STUBIFY)` - the Android arm defines its own (`android.c` 5915 logs a stack trace),
   so this one is not compiled there.  Without the gate the shared library link read
   `duplicate symbol: emacs_abort` (measured).  */
#[cfg (not (all (feature = "have_android", feature = "not_android_stubify")))]
// C 2399-2406: `#if !defined HAVE_NTGUI && !(defined HAVE_ANDROID && !defined
// ANDROID_STUBIFY)` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_abort () {
  terminate_due_to_signal (SIGABRT, 40);
}

/* Assuming the directory DIRFD, store information about FILENAME into *ST,
   using FLAGS to control how the status is obtained.
   Do not fail merely because fetching info was interrupted by a signal.
   Allow the user to quit.

   The type of ST is void * instead of struct stat * because the
   latter type would be problematic in lisp.h.  Some platforms may
   play tricks like "#define stat stat64" in <sys/stat.h>, and lisp.h
   does not include <sys/stat.h>.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_fstatat (dirfd: c_int, filename: *const c_char, st: *mut c_void,
                                        flags: c_int) -> c_int {
  let mut r: c_int;
  // C 2422-2430: `#if !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)` is live.
  #[cfg (not (feature = "have_android"))]
  {
    loop
      {
	r = fstatat (dirfd, filename, st as *mut stat, flags);
	if r == 0 || *errno () != EINTR
	  { break; }
	maybe_quit ();
      }
  }
  #[cfg (feature = "have_android")]
  {
    loop
      {
	r = android_fstatat (dirfd, filename, st as *mut stat, flags);   // android.h 84's `struct stat *`
	if r == 0 || *errno () != EINTR
	  { break; }
	maybe_quit ();
      }
  }
  return r;
}

// C 2434-2450: `#if !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)` is live.
unsafe fn sys_openat (dirfd: c_int, file: *const c_char, oflags: c_int, mode: c_int) -> c_int {
  /* C 2439-2447: `#ifdef O_PATH` is dead (`O_PATH` does not exist on this target);
     carried.  */
  #[cfg (feature = "o_path")]
  {
    return openat (dirfd, file, oflags, mode);
  }
  #[cfg (not (feature = "o_path"))]
  {
    /* On platforms without O_PATH, emacs_openat's callers arrange for
       DIRFD to be AT_FDCWD, so it should be safe to just call 'open'.
       This ports to old platforms like OS X 10.9 that lack openat.  */
    eassert! (dirfd == AT_FDCWD);
    return open (file, oflags, mode);
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_fstat (fd: c_int, statb: *mut stat) -> c_int {
  // C 2455-2461: `#if !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)` is live.
  #[cfg (not (feature = "have_android"))]
  {
    return fstat (fd, statb);
  }
  #[cfg (feature = "have_android")]
  {
    return android_fstat (fd, statb);
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_faccessat (fd: c_int, pathname: *const c_char, mode: c_int,
                                        flags: c_int) -> c_int {
  // C 2465-2471: `#if !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)` is live.
  #[cfg (not (feature = "have_android"))]
  {
    return faccessat (fd, pathname, mode, flags);
  }
  #[cfg (feature = "have_android")]
  {
    return android_faccessat (fd, pathname, mode, flags);
  }
}

/* Assuming the directory DIRFD, open FILE for Emacs use,
   using open flags OFLAGS and mode MODE.
   Use binary I/O on systems that care about text vs binary I/O.
   Arrange for subprograms to not inherit the file descriptor.
   Prefer a method that is multithread-safe, if available.
   Do not fail merely because the open was interrupted by a signal.
   Allow the user to quit.  */

// C 2480-2494: `#if !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)` is live.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_openat (dirfd: c_int, file: *const c_char, mut oflags: c_int,
                                       mode: c_int) -> c_int {
  let mut fd: c_int;
  if (oflags & O_TEXT) == 0
    { oflags |= O_BINARY; }
  oflags |= O_CLOEXEC;
  loop
    {
      fd = sys_openat (dirfd, file, oflags, mode);
      if fd >= 0 || *errno () != EINTR
	{ break; }
      maybe_quit ();
    }
  return fd;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_open (file: *const c_char, oflags: c_int, mode: c_int) -> c_int {
  // C 2499-2510: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` is dead; carried.
  #[cfg (feature = "have_android")]
  let mut fd: c_int = 0;

  #[cfg (not (feature = "have_android"))]
  {
    return emacs_openat (AT_FDCWD, file, oflags, mode);
  }
  #[cfg (feature = "have_android")]
  {
    loop
      {
	fd = android_open (file, oflags, mode as mode_t);   // android.h 82's `mode_t`
	if fd >= 0 || *errno () != EINTR
	  { break; }
	maybe_quit ();
      }

    return fd;
  }
}

/* Same as above, but doesn't allow the user to quit.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_open_noquit (file: *const c_char, mut oflags: c_int, mode: c_int)
  -> c_int {
  let mut fd: c_int;
  if (oflags & O_TEXT) == 0
    { oflags |= O_BINARY; }
  oflags |= O_CLOEXEC;
  // C 2522-2530: `#if !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)` is live.
  #[cfg (not (feature = "have_android"))]
  {
    loop
      { fd = open (file, oflags, mode); if !(fd < 0 && *errno () == EINTR) { break; } }
  }
  #[cfg (feature = "have_android")]
  {
    loop
      { fd = android_open (file, oflags, mode as mode_t); if !(fd < 0 && *errno () == EINTR) { break; } }
  }
  return fd;
}

/* Open FILE as a stream for Emacs use, with mode MODE.
   Act like emacs_open with respect to threads, signals, and quits.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_fopen (file: *const c_char, mode: *const c_char) -> *mut FILE {
  let mut fd: c_int = 0;
  let mut omode: c_int = 0;
  let mut oflags: c_int = 0;
  let mut bflag: c_int = 0;
  let mut m: *const c_char = mode;

  m = m.add (1);
  match *m.offset (-1) as u8
    {
      b'r' => { omode = O_RDONLY; oflags = 0; }
      b'w' => { omode = O_WRONLY; oflags = O_CREAT | O_TRUNC; }
      b'a' => { omode = O_WRONLY; oflags = O_CREAT | O_APPEND; }
      _ => { emacs_abort (); }
    }

  while *m != 0
    {
      let c: c_char = *m;
      m = m.add (1);
      match c as u8
	{
	  b'+' => { omode = O_RDWR; }
	  b't' => { bflag = O_TEXT; }
	  _ => { /* Ignore.  */ }
	}
    }

  fd = emacs_open (file, omode | oflags | bflag, 0o666);
  return if fd < 0 { std::ptr::null_mut () } else { emacs_fdopen (fd, mode) };
}

/* Create a pipe for Emacs use.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_pipe (fd: *mut c_int) -> c_int {
  // C 2569-2573: `#ifdef MSDOS` is dead; carried.
  #[cfg (feature = "msdos")]
  {
    return pipe (fd);
  }
  #[cfg (not (feature = "msdos"))]
  {
    return pipe2! (fd, O_BINARY | O_CLOEXEC);   /* C: `pipe2` is unistd.in.h 1831's rename */
  }
}


/* Approximate posix_close and POSIX_CLOSE_RESTART well enough for Emacs.
   For the background behind this mess, please see Austin Group defect 529
   <https://austingroupbugs.net/view.php?id=529>.  */

// C 2580-2608: `#if !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)` is live.

/// C 2582-2583: `#ifndef POSIX_CLOSE_RESTART` is live (`POSIX_CLOSE_RESTART` is undefined on
/// this target), so C defines it as 1 and compiles `posix_close` below.
pub const POSIX_CLOSE_RESTART: c_int = 1;

unsafe fn posix_close (fd: c_int, flag: c_int) -> c_int {
  /* Only the POSIX_CLOSE_RESTART case is emulated.  */
  eassert! (flag == POSIX_CLOSE_RESTART);

  /* Things are tricky if close (fd) returns -1 with errno == EINTR
     on a system that does not define POSIX_CLOSE_RESTART.

     In this case, in some systems (e.g., GNU/Linux, AIX) FD is
     closed, and retrying the close could inadvertently close a file
     descriptor allocated by some other thread.  In other systems
     (e.g., HP/UX) FD is not closed.  And in still other systems
     (e.g., macOS, Solaris), maybe FD is closed, maybe not, and in a
     multithreaded program there can be no way to tell.

     So, in this case, pretend that the close succeeded.  This works
     well on systems like GNU/Linux that close FD.  Although it may
     leak a file descriptor on other systems, the leak is unlikely and
     it's better to leak than to close a random victim.  */
  return if close (fd) == 0 || *errno () == EINTR { 0 } else { -1 };
}

/* Close FD, retrying if interrupted.  If successful, return 0;
   otherwise, return -1 and set errno to a non-EINTR value.  Consider
   an EINPROGRESS error to be successful, as that's merely a signal
   arriving.  FD is always closed when this function returns, even
   when it returns -1.

   Do not call this function if FD is nonnegative and might already be closed,
   as that might close an innocent victim opened by some other thread.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_close (fd: c_int) -> c_int {
  let mut r: c_int;

  loop
    {
      // C 2626-2631: this is C's `#if !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)`.
      // Both arms were compiled on Android, so `posix_close` closed the descriptor first and
      // `android_close` then reported EBADF for every close (status 4.848: a pipe's own write end
      // read back as EBADF one statement after a successful write).
      #[cfg (not (all (feature = "have_android", feature = "not_android_stubify")))]
      {
	r = posix_close (fd, POSIX_CLOSE_RESTART);
      }
      #[cfg (feature = "have_android")]
      {
	r = if android_close (fd) == 0 || *errno () == EINTR { 0 } else { -1 };
      }

      if r == 0
	{ return r; }
      if POSIX_CLOSE_RESTART == 0 || *errno () != EINTR
	{
	  eassert! (*errno () != EBADF || fd < 0);
	  return if *errno () == EINPROGRESS { 0 } else { r };
	}
    }
}

/* Wrapper around fdopen.  On Android, this calls `android_fclose' to
   clear information associated with FD if necessary.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_fdopen (fd: c_int, mode: *const c_char) -> *mut FILE {
  // C 2649-2653: `#if !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)` is live.
  #[cfg (not (feature = "have_android"))]
  {
    return fdopen (fd, mode);
  }
  #[cfg (feature = "have_android")]
  {
    return android_fdopen (fd, mode);
  }
}

/* Wrapper around fclose.  On Android, this calls `android_fclose' to
   clear information associated with the FILE's file descriptor if
   necessary.  */

// C 2660-2666: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` is dead; carried.
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_fclose (stream: *mut FILE) -> c_int {
  return android_fclose (stream);
}

/* Wrappers around unlink, symlink, rename, renameat_noreplace, and
   rmdir.  These operations handle asset and content directories on
   Android, and may return EINTR.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_unlink (name: *const c_char) -> c_int {
  // C 2675-2679: the non-Android arm is live; the Android arm is carried.
  #[cfg (not (feature = "have_android"))]
  {
    return unlink (name);
  }
  #[cfg (feature = "have_android")]
  {
    return android_unlink (name);
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_symlink (target: *const c_char, linkname: *const c_char)
  -> c_int {
  // C 2685-2689: the non-Android arm is live; the Android arm is carried.
  #[cfg (not (feature = "have_android"))]
  {
    return symlink (target, linkname);
  }
  #[cfg (feature = "have_android")]
  {
    return android_symlink (target, linkname);
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_rmdir (dirname: *const c_char) -> c_int {
  // C 2695-2699: the non-Android arm is live; the Android arm is carried.
  #[cfg (not (feature = "have_android"))]
  {
    return rmdir (dirname);
  }
  #[cfg (feature = "have_android")]
  {
    return android_rmdir (dirname);
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_mkdir (dirname: *const c_char, mode: mode_t) -> c_int {
  // C 2705-2709: the non-Android arm is live; the Android arm is carried.
  #[cfg (not (feature = "have_android"))]
  {
    return mkdir (dirname, mode);
  }
  #[cfg (feature = "have_android")]
  {
    return android_mkdir (dirname, mode);
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_renameat_noreplace (srcfd: c_int, src: *const c_char,
                                                   dstfd: c_int, dst: *const c_char)
  -> c_int {
  // C 2716-2720: the non-Android arm is live; the Android arm is carried.
  #[cfg (not (feature = "have_android"))]
  {
    return renameat_noreplace (srcfd, src, dstfd, dst);
  }
  #[cfg (feature = "have_android")]
  {
    return android_renameat_noreplace (srcfd, src, dstfd, dst);
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_rename (src: *const c_char, dst: *const c_char) -> c_int {
  // C 2726-2730: the non-Android arm is live; the Android arm is carried.
  #[cfg (not (feature = "have_android"))]
  {
    return rename (src, dst);
  }
  #[cfg (feature = "have_android")]
  {
    return android_rename (src, dst);
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_fchmodat (fd: c_int, path: *const c_char, mode: mode_t,
                                         flags: c_int) -> c_int {
  // C 2736-2740: the non-Android arm is live; the Android arm is carried.
  #[cfg (not (feature = "have_android"))]
  {
    return fchmodat (fd, path, mode, flags);
  }
  #[cfg (feature = "have_android")]
  {
    return android_fchmodat (fd, path, mode, flags);
  }
}

/* Verify that SYS_BUFSIZE_MAX fits in the relevant standard types.  */
// C 2744-2746: `#ifndef SSIZE_MAX` is dead on this target (`SSIZE_MAX` comes from
// <stdint.h>); carried.
/// C 2745's `# define SSIZE_MAX TYPE_MAXIMUM (ssize_t)`.
#[cfg (feature = "no_ssize_max")]
pub const SSIZE_MAX: ptrdiff_t = TYPE_MAXIMUM! (ssize_t);
static_assert! (SYS_BUFSIZE_MAX <= PTRDIFF_MAX);
static_assert! (SYS_BUFSIZE_MAX as usize <= SIZE_MAX);
#[cfg (feature = "no_ssize_max")]
static_assert! (SYS_BUFSIZE_MAX <= SSIZE_MAX);

// C 2751-2757: `#ifdef WINDOWSNT` is dead; carried.
#[cfg (feature = "windowsnt")]
static_assert! (SYS_BUFSIZE_MAX <= INT_MAX as isize);
#[cfg (feature = "windowsnt")]
static_assert! (SYS_BUFSIZE_MAX <= UINT_MAX as isize);

/// `<intprops.h>`'s `TYPE_MAXIMUM (t)`: the largest value of the type `t` (the
/// `syntax.rs`/`filelock.rs` unit-local shape; `<intprops.h>` has no mirror, P-26).
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }

/// `lib/sys-limits.h`'s `SYS_BUFSIZE_MAX`, which C reaches through `<sys-limits.h>` (lisp.h
/// 31 includes it): `enum { SYS_BUFSIZE_MAX = INT_MAX >> 20 << 20 }` = 2,146,435,072.  The
/// port had `MAX_ALLOCA` (16,384) here, so every read above 16 KB tripped `emacs_intr_read`'s
/// `eassert` while `insert-file-contents` loaded a large Lisp file (device run, status 4.850).
pub const SYS_BUFSIZE_MAX: ptrdiff_t = ((INT_MAX >> 20) << 20) as ptrdiff_t;

/* Read from FD to a buffer BUF with size NBYTE.
   If interrupted, process any quits and pending signals immediately
   if INTERRUPTIBLE, and then retry the read unless quitting.
   Return the number of bytes read, which might be less than NBYTE.
   On error, set errno to a value other than EINTR, and return -1.  */
unsafe fn emacs_intr_read (fd: c_int, buf: *mut c_void, nbyte: ptrdiff_t,
                           interruptible: bool) -> ptrdiff_t {
  /* No caller should ever pass a too-large size to emacs_read.  */
  eassert! (nbyte <= SYS_BUFSIZE_MAX);

  let mut result: ssize_t;

  loop
    {
      if interruptible
	{ maybe_quit (); }
      result = read (fd, buf, nbyte as usize);
      if !(result < 0 && *errno () == EINTR)
	{ break; }
    }

  return result as ptrdiff_t;
}

/* Read from FD to a buffer BUF with size NBYTE.
   If interrupted, retry the read.  Return the number of bytes read,
   which might be less than NBYTE.  On error, set errno to a value
   other than EINTR, and return -1.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_read (fd: c_int, buf: *mut c_void, nbyte: ptrdiff_t)
  -> ptrdiff_t {
  return emacs_intr_read (fd, buf, nbyte, false);
}

/* Like emacs_read, but also process quits and pending signals.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_read_quit (fd: c_int, buf: *mut c_void, nbyte: ptrdiff_t)
  -> ptrdiff_t {
  return emacs_intr_read (fd, buf, nbyte, true);
}

/* Write to FILEDES from a buffer BUF with size NBYTE, retrying if
   interrupted or if a partial write occurs.  Process any quits
   immediately if INTERRUPTIBLE is positive, and process any pending
   signals immediately if INTERRUPTIBLE is nonzero.  Return the number
   of bytes written; if this is less than NBYTE, set errno to a value
   other than EINTR.  */
unsafe fn emacs_full_write (fd: c_int, buf: *const c_char, mut nbyte: ptrdiff_t,
			    interruptible: c_int) -> ptrdiff_t {
  let mut bytes_written: ptrdiff_t = 0;
  let mut buf: *const c_char = buf;

  while nbyte > 0
    {
      let n: ssize_t = write (fd, buf as *const c_void,
			      min (nbyte, SYS_BUFSIZE_MAX) as usize);

      if n < 0
	{
	  if *errno () != EINTR
	    { break; }

	  if interruptible != 0
	    {
	      if 0 < interruptible
		{ maybe_quit (); }
	      if pending_signals
		{ process_pending_signals (); }
	    }
	}
      else
	{
	  buf = buf.add (n as usize);
	  nbyte -= n as ptrdiff_t;
	  bytes_written += n as ptrdiff_t;
	}
    }

  return bytes_written;
}

/* Write to FD from a buffer BUF with size NBYTE, retrying if
   interrupted or if a partial write occurs.  Do not process quits or
   pending signals.  Return the number of bytes written, setting errno
   if this is less than NBYTE.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_write (fd: c_int, buf: *const c_void, nbyte: ptrdiff_t)
  -> ptrdiff_t {
  return emacs_full_write (fd, buf as *const c_char, nbyte, 0);
}

/* Like emacs_write, but also process pending signals.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_write_sig (fd: c_int, buf: *const c_void, nbyte: ptrdiff_t)
  -> ptrdiff_t {
  return emacs_full_write (fd, buf as *const c_char, nbyte, -1);
}

/* Like emacs_write, but also process quits and pending signals.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_write_quit (fd: c_int, buf: *const c_void, nbyte: ptrdiff_t)
  -> ptrdiff_t {
  return emacs_full_write (fd, buf as *const c_char, nbyte, 1);
}

/* Write a diagnostic to standard error that contains MESSAGE and a
   string derived from errno.  Preserve errno.  Do not buffer stderr.
   Do not process quits or pending signals if interrupted.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_perror (message: *const c_char) {
  let err: c_int = *errno ();
  let error_string: *const c_char = emacs_strerror (err);
  let command: *const c_char = if !initial_argv0.is_null () { initial_argv0 }
			       else { c"emacs".as_ptr () };
  /* Write it out all at once, if it's short; this is less likely to
     be interleaved with other output.  */
  /* C 2884's `char buf[min (PIPE_BUF, MAX_ALLOCA)]` in a const expression, so that the
     body keeps both of C's tokens.  */
  const BUF_SIZE: usize = if PIPE_BUF < MAX_ALLOCA as usize { PIPE_BUF }
			  else { MAX_ALLOCA as usize };
  let mut buf: [c_char; BUF_SIZE] = [0; BUF_SIZE];
  let nbytes: c_int = snprintf (buf.as_mut_ptr (), std::mem::size_of_val (&buf),
				c"%s: %s: %s\n".as_ptr (), command, message, error_string);
  if 0 <= nbytes && (nbytes as usize) < std::mem::size_of_val (&buf)
    { emacs_write (STDERR_FILENO, buf.as_ptr () as *const c_void, nbytes as ptrdiff_t); }
  else
    {
      emacs_write (STDERR_FILENO, command as *const c_void, strlen (command) as ptrdiff_t);
      emacs_write (STDERR_FILENO, c": ".as_ptr () as *const c_void, 2);
      emacs_write (STDERR_FILENO, message as *const c_void, strlen (message) as ptrdiff_t);
      emacs_write (STDERR_FILENO, c": ".as_ptr () as *const c_void, 2);
      emacs_write (STDERR_FILENO, error_string as *const c_void,
		   strlen (error_string) as ptrdiff_t);
      emacs_write (STDERR_FILENO, c"\n".as_ptr () as *const c_void, 1);
    }
  *errno () = err;
}

/* Like strsignal, except async-signal-safe, and this function
   returns a string in the C locale rather than the current locale.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_strsignal (code: c_int) -> *const c_char {
  let mut signame: *const c_char = sigdescr_np (code);

  if signame.is_null ()
    { signame = c"Unknown signal".as_ptr (); }

  return signame;
}

/* Output to stderr.  */

/* Return the error output stream.  */
unsafe fn errstream () -> *mut FILE {
  let err: *mut FILE = buferr;
  if err.is_null ()
    { return stderr; }
  fflush_unlocked! (stderr);
  return err;
}

/* These functions are like fputc, vfprintf, and fwrite,
   except that they output to stderr and buffer better on
   platforms that support line buffering.  This avoids interleaving
   output when Emacs and other processes write to stderr
   simultaneously, so long as the lines are short enough.  When a
   single diagnostic is emitted via a sequence of calls of one or more
   of these functions, the caller should arrange for the last called
   function to output a newline at the end.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn errputc (c: c_int) {
  fputc_unlocked! (c, errstream ());

  /* C 2932-2937: `#ifdef WINDOWSNT` is dead; carried.  */
  #[cfg (feature = "windowsnt")]
  {
    /* Flush stderr after outputting a newline since stderr is fully
       buffered when redirected to a pipe, contrary to POSIX.  */
    if c == b'\n' as c_int
      { fflush_unlocked! (stderr); }
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn errwrite (buf: *const c_void, nbuf: ptrdiff_t) {
  fwrite_unlocked! (buf, 1, nbuf as usize, errstream ());
}

/* Close standard output and standard error, reporting any write
   errors as best we can.  This is intended for use with atexit.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_output_streams () {
  /* Android comes with some kind of ``file descriptor sanitizer''
     that aborts when stdout or stderr is closed.  (bug#65340)

     Perform this unconditionally as long as __ANDROID__ is defined,
     since the file descriptor sanitizer also applies to regular TTY
     builds under Android.  */

  // C 2958-2975: `#ifdef __ANDROID__` is dead; carried.
  #[cfg (feature = "have_android")]
  {
    fflush (stderr);
    fflush (stdout);
  }
  #[cfg (not (feature = "have_android"))]
  {
    if close_stream (stdout) != 0
      {
	emacs_perror (c"Write error to standard output".as_ptr ());
	_exit (EXIT_FAILURE);
      }

    /* Do not close stderr if addresses are being sanitized, as the
       sanitizer might report to stderr after this function is invoked.  */
    let err: c_int =
      (!buferr.is_null () && (fflush (buferr) != 0 || ferror (buferr) != 0)) as c_int;
    let sanitizer_err: c_int =
      (if ADDRESS_SANITIZER
	 { fflush (stderr) != 0 || ferror (stderr) != 0 }
       else
	 { close_stream (stderr) != 0 }) as c_int;
    if (err | sanitizer_err) != 0
      { _exit (EXIT_FAILURE); }
  }
}

// C 2978: `#ifndef DOS_NT` — live.
/* For make-serial-process  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_open (port: Lisp_Object) -> c_int {
  let fd: c_int = emacs_open (SSDATA (port), O_RDWR | O_NOCTTY | O_NONBLOCK, 0);
  /* C 2984-2987: `#ifdef TIOCEXCL` is live.  */
  {
    if 0 <= fd
      { ioctl (fd, TIOCEXCL, std::ptr::null::<c_char> ()); }
  }

  return fd;
}

// C 2992-3004: `#if !defined (HAVE_CFMAKERAW)` is dead (`HAVE_CFMAKERAW` is 1); carried.
/* Workaround for targets which are missing cfmakeraw.  */
/* Pasted from man page.  */
#[cfg (feature = "no_have_cfmakeraw")]
unsafe fn cfmakeraw (termios_p: *mut termios) {
    (*termios_p).c_iflag &= !(IGNBRK|BRKINT|PARMRK|ISTRIP|INLCR|IGNCR|ICRNL|IXON);
    (*termios_p).c_oflag &= !OPOST;
    (*termios_p).c_lflag &= !(ECHO|ECHONL|ICANON|ISIG|IEXTEN);
    (*termios_p).c_cflag &= !(CSIZE|PARENB);
    (*termios_p).c_cflag |= CS8;
}

// C 3006-3014: `#if !defined (HAVE_CFSETSPEED)` is dead; carried.
/* Workaround for targets which are missing cfsetspeed.  */
#[cfg (feature = "no_have_cfsetspeed")]
unsafe fn cfsetspeed (termios_p: *mut termios, vitesse: speed_t) -> c_int {
  return (cfsetispeed (termios_p, vitesse)
	  + cfsetospeed (termios_p, vitesse));
}

/* The following is based on the glibc implementation of cfsetspeed.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct speed_struct {
  pub value: speed_t,
  pub internal: speed_t,
}

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<speed_struct> () == 16);
/* Two `speed_t`: 8 + 8 on the host, 4 + 4 where bionic's `speed_t` is `unsigned int`
   (status 4.795).  */
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<speed_struct> () == 8);
const _: () = core::assert! (std::mem::offset_of! (speed_struct, value) == 0);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (speed_struct, internal) == 8);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (speed_struct, internal) == 4);

static speeds: [speed_struct; 21] =
  [
    speed_struct { value: 0, internal: B0 },
    speed_struct { value: 50, internal: B50 },
    speed_struct { value: 75, internal: B75 },
    speed_struct { value: 110, internal: B110 },
    speed_struct { value: 134, internal: B134 },
    speed_struct { value: 150, internal: B150 },
    /* C 3044-3132: `#ifndef HAVE_TINY_SPEED_T` is live (`HAVE_TINY_SPEED_T` is undef), and
       inside it every `#ifdef Bnnn` arm this target defines.  C 3087-3131's `B153600`,
       `B307200`, `B460800`, `B500000`, `B576000`, `B921600`, `B1000000`, `B1152000`,
       `B1500000`, `B2000000`, `B2500000`, `B3000000`, `B3500000` and `B4000000` arms are
       dead on this target and are carried below the live table (a `static` array cannot
       hold a `#[cfg]`-gated element list without a second definition).  */
    speed_struct { value: 200, internal: B200 },
    speed_struct { value: 300, internal: B300 },
    speed_struct { value: 600, internal: B600 },
    speed_struct { value: 1200, internal: B1200 },
    /* C 3057's second `#ifdef B1200` arm repeats C 3054's entry verbatim in the reference
       (the preprocessor keeps both); the port keeps both, as C does.  */
    speed_struct { value: 1200, internal: B1200 },
    speed_struct { value: 1800, internal: B1800 },
    speed_struct { value: 2400, internal: B2400 },
    speed_struct { value: 4800, internal: B4800 },
    speed_struct { value: 9600, internal: B9600 },
    speed_struct { value: 19200, internal: B19200 },
    speed_struct { value: 38400, internal: B38400 },
    speed_struct { value: 57600, internal: B57600 },
    speed_struct { value: 76800, internal: B76800 },
    speed_struct { value: 115200, internal: B115200 },
    speed_struct { value: 230400, internal: B230400 },
  ];

/// C 3087-3131's fourteen dead `#ifdef Bnnn` arms, carried as their own table: the frozen
/// target has no such names (measured), so `speeds` above keeps the live entries and this
/// record keeps the text of the arms the frozen build does not compile (P-51).
#[cfg (any (feature = "b153600", feature = "b307200", feature = "b460800",
	     feature = "b500000", feature = "b576000", feature = "b921600",
	     feature = "b1000000", feature = "b1152000", feature = "b1500000",
	     feature = "b2000000", feature = "b2500000", feature = "b3000000",
	     feature = "b3500000", feature = "b4000000"))]
#[allow(dead_code)]
static speeds_dead_arms: [speed_struct; 14] =
  [
    speed_struct { value: 153600, internal: B153600 },
    speed_struct { value: 307200, internal: B307200 },
    speed_struct { value: 460800, internal: B460800 },
    speed_struct { value: 500000, internal: B500000 },
    speed_struct { value: 576000, internal: B576000 },
    speed_struct { value: 921600, internal: B921600 },
    speed_struct { value: 1000000, internal: B1000000 },
    speed_struct { value: 1152000, internal: B1152000 },
    speed_struct { value: 1500000, internal: B1500000 },
    speed_struct { value: 2000000, internal: B2000000 },
    speed_struct { value: 2500000, internal: B2500000 },
    speed_struct { value: 3000000, internal: B3000000 },
    speed_struct { value: 3500000, internal: B3500000 },
    speed_struct { value: 4000000, internal: B4000000 },
  ];

/* Convert a numerical speed (e.g., 9600) to a Bnnn constant (e.g.,
   B9600); see bug#49524.  */
unsafe fn convert_speed (speed: speed_t) -> speed_t {
  let mut i: ptrdiff_t = 0;
  while i < countof! (speeds) as ptrdiff_t
    {
      if speed == speeds[i as usize].internal
	{ return speed; }
      else if speed == speeds[i as usize].value
	{ return speeds[i as usize].internal; }
      i += 1;
    }
  return speed;
}

/* For serial-process-configure  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_configure (p: *mut Lisp_Process, contact: Lisp_Object) {
  let mut childp2: Lisp_Object = Qnil ();
  let mut tem: Lisp_Object = Qnil ();
  let mut attr: termios = std::mem::zeroed ();
  let mut err: c_int;
  let mut summary: [c_char; 4] =
    [b'?' as c_char, b'?' as c_char, b'?' as c_char, 0]; /* This usually becomes "8N1".  */

  childp2 = Fcopy_sequence ((*p).childp);

  /* Read port attributes and prepare default configuration.  */
  err = tcgetattr ((*p).outfd, &mut attr);
  if err != 0
    { report_file_error (c"Failed tcgetattr".as_ptr (), Qnil ()); }
  cfmakeraw (&mut attr);
  /* C 3168-3170: `#if defined (CLOCAL)` is live.  */
  {
    attr.c_cflag |= CLOCAL;
  }
  /* C 3171-3173: `#if defined (CREAD)` is live.  */
  {
    attr.c_cflag |= CREAD;
  }

  /* Configure speed.  */
  if !NILP (plist_member (contact, QCspeed ()))
    { tem = plist_get (contact, QCspeed ()); }
  else
    { tem = plist_get ((*p).childp, QCspeed ()); }
  CHECK_FIXNUM (tem);
  err = cfsetspeed (&mut attr, convert_speed (XFIXNUM (tem) as speed_t));
  if err != 0
    { report_file_error (c"Failed cfsetspeed".as_ptr (), tem); }
  childp2 = plist_put (childp2, QCspeed (), tem);

  /* Configure bytesize.  */
  if !NILP (plist_member (contact, QCbytesize ()))
    { tem = plist_get (contact, QCbytesize ()); }
  else
    { tem = plist_get ((*p).childp, QCbytesize ()); }
  if NILP (tem)
    { tem = make_fixnum (8); }
  CHECK_FIXNUM (tem);
  if XFIXNUM (tem) != 7 && XFIXNUM (tem) != 8
    { error (c":bytesize must be nil (8), 7, or 8".as_ptr ()); }
  summary[0] = (XFIXNUM (tem) + b'0' as EMACS_INT) as c_char;
  /* C 3197-3204: `#if defined (CSIZE) && defined (CS7) && defined (CS8)` is live.  */
  {
    attr.c_cflag &= !CSIZE;
    attr.c_cflag |= if XFIXNUM (tem) == 7 { CS7 } else { CS8 };
  }
  childp2 = plist_put (childp2, QCbytesize (), tem);

  /* Configure parity.  */
  if !NILP (plist_member (contact, QCparity ()))
    { tem = plist_get (contact, QCparity ()); }
  else
    { tem = plist_get ((*p).childp, QCparity ()); }
  if !NILP (tem) && !EQ (tem, Qeven ()) && !EQ (tem, Qodd ())
    { error (c":parity must be nil (no parity), `even', or `odd'".as_ptr ()); }
  /* C 3214-3237: `#if defined (PARENB) && defined (PARODD) && defined (IGNPAR) &&
     defined (INPCK)` is live.  */
  {
    attr.c_cflag &= !(PARENB | PARODD);
    attr.c_iflag &= !(IGNPAR | INPCK);
    if NILP (tem)
      {
	summary[1] = b'N' as c_char;
      }
    else if EQ (tem, Qeven ())
      {
	summary[1] = b'E' as c_char;
	attr.c_cflag |= PARENB;
	attr.c_iflag |= (IGNPAR | INPCK);
      }
    else if EQ (tem, Qodd ())
      {
	summary[1] = b'O' as c_char;
	attr.c_cflag |= (PARENB | PARODD);
	attr.c_iflag |= (IGNPAR | INPCK);
      }
  }
  childp2 = plist_put (childp2, QCparity (), tem);

  /* Configure stopbits.  */
  if !NILP (plist_member (contact, QCstopbits ()))
    { tem = plist_get (contact, QCstopbits ()); }
  else
    { tem = plist_get ((*p).childp, QCstopbits ()); }
  if NILP (tem)
    { tem = make_fixnum (1); }
  CHECK_FIXNUM (tem);
  if XFIXNUM (tem) != 1 && XFIXNUM (tem) != 2
    { error (c":stopbits must be nil (1 stopbit), 1, or 2".as_ptr ()); }
  summary[2] = (XFIXNUM (tem) + b'0' as EMACS_INT) as c_char;
  /* C 3251-3259: `#if defined (CSTOPB)` is live.  */
  {
    attr.c_cflag &= !CSTOPB;
    if XFIXNUM (tem) == 2
      { attr.c_cflag |= CSTOPB; }
  }
  childp2 = plist_put (childp2, QCstopbits (), tem);

  /* Configure flowcontrol.  */
  if !NILP (plist_member (contact, QCflowcontrol ()))
    { tem = plist_get (contact, QCflowcontrol ()); }
  else
    { tem = plist_get ((*p).childp, QCflowcontrol ()); }
  if !NILP (tem) && !EQ (tem, Qhw ()) && !EQ (tem, Qsw ())
    { error (c":flowcontrol must be nil (no flowcontrol), `hw', or `sw'".as_ptr ()); }
  /* C 3269-3271: `#if defined (CRTSCTS)` is live.  */
  {
    attr.c_cflag &= !CRTSCTS;
  }
  /* C 3272-3274: `#if defined (CNEW_RTSCTS)` is dead; carried.  */
  #[cfg (feature = "cnew_rtscts")]
  {
    attr.c_cflag &= !CNEW_RTSCTS;
  }
  /* C 3275-3277: `#if defined (IXON) && defined (IXOFF)` is live.  */
  {
    attr.c_iflag &= !(IXON | IXOFF);
  }
  if NILP (tem)
    {
      /* Already configured.  */
    }
  else if EQ (tem, Qhw ())
    {
      /* C 3284-3290: `#if defined (CRTSCTS)` is live.  */
      {
	attr.c_cflag |= CRTSCTS;
      }
    }
  else if EQ (tem, Qsw ())
    {
      /* C 3294-3296: `#if defined (IXON) && defined (IXOFF)` is live.  */
      {
	attr.c_iflag |= (IXON | IXOFF);
      }
    }
  childp2 = plist_put (childp2, QCflowcontrol (), tem);

  /* Activate configuration.  */
  err = tcsetattr ((*p).outfd, TCSANOW, &attr);
  if err != 0
    { report_file_error (c"Failed tcsetattr".as_ptr (), Qnil ()); }

  childp2 = plist_put (childp2, QCsummary (), build_string (summary.as_ptr ()));
  pset_childp (p, childp2);
}

/* System depended enumeration of and access to system processes a-la ps(1).  */

// C 3314: `#ifdef HAVE_PROCFS` — dead (`HAVE_PROCFS` is undefined); carried.
#[cfg (feature = "have_procfs")]
/* Process enumeration and access via /proc.  */

#[cfg (feature = "have_procfs")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn list_system_processes () -> Lisp_Object {
  let mut procdir: Lisp_Object = Qnil ();
  let mut match_: Lisp_Object = Qnil ();
  let mut proclist: Lisp_Object = Qnil ();
  let mut next: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object;

  /* For every process on the system, there's a directory in the
     "/proc" pseudo-directory whose name is the numeric ID of that
     process.  */
  procdir = build_string (c"/proc".as_ptr ());
  match_ = build_string (c"[0-9]+".as_ptr ());
  proclist = directory_files_internal (procdir, Qnil (), match_, Qt (),
                                       false, Qnil (), Qnil ());

  /* `proclist' gives process IDs as strings.  Destructively convert
     each string into a number.  */
  tail = proclist;
  while CONSP (tail)
    {
      next = XCDR (tail);
      XSETCAR (tail, Fstring_to_number (XCAR (tail), Qnil ()));   // lisp.rs 898 renders C's macro as a fn
      tail = next;
    }

  /* directory_files_internal returns the files in reverse order; undo
     that.  */
  proclist = Fnreverse (proclist);
  return proclist;
}

// C 3346-3404: `#elif defined DARWIN_OS || defined __FreeBSD__ || defined __OpenBSD__` — the
// frozen target's own arm (and the inner `#ifdef DARWIN_OS`/`#elif __OpenBSD__` arms below it),
// and C 3314's `#ifdef HAVE_PROCFS` arm above it is the one Android takes (`HAVE_PROCFS` is 1
// there).  `#elif` is exclusive and Rust's `cfg` is not, so the arm carries the negation of the
// condition before it - without it the Android build read `E0428 list_system_processes` (measured).
/* The `DARWIN_OS`/`__FreeBSD__`/`__OpenBSD__` half of C 3346 is a *platform* condition, not a
   configure macro - the darwin configuration carries neither name, so the port spells it with
   `target_os` (measured: with a feature the darwin build lost the symbol and `gate-load`
   reported `_list_system_processes` undefined).  */
#[cfg (all (not (feature = "have_procfs"),
            any (target_os = "macos", target_os = "freebsd", target_os = "openbsd")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn list_system_processes () -> Lisp_Object {
  /* C 3351-3358's three arrays, each with its own length: Darwin's is **three** entries
     (`{CTL_KERN, KERN_PROC, KERN_PROC_ALL}`), OpenBSD's six (`KERN_PROC_ALL, 0,
     sizeof (struct kinfo_proc), 4096`) and FreeBSD's three with `KERN_PROC_PROC`.  The port had
     one six-entry array for all three, so on Darwin `sysctl` was called with `namelen` 6 and
     refused it (`rc=-1`, measured with the same constants in C: 3 entries -> 701784 bytes,
     6 -> -1), which left `list-system-processes` empty.  */
  #[cfg (target_os = "macos")]
  let mut mib: [c_int; 3] = [CTL_KERN, KERN_PROC, KERN_PROC_ALL];
  #[cfg (target_os = "openbsd")]
  let mut mib: [c_int; 6] = [CTL_KERN, KERN_PROC, KERN_PROC_ALL, 0,
                             std::mem::size_of::<kinfo_proc> () as c_int, 4096];
  #[cfg (all (not (target_os = "macos"), not (target_os = "openbsd")))]
  let mut mib: [c_int; 3] = [CTL_KERN, KERN_PROC, KERN_PROC_PROC];
  /* PORTING.md 1.5: C declares `size_t len;` and `sysctl` fills it.  */
  let mut len: usize = 0;
  let mibsize: usize = countof! (mib);
  let procs: *mut kinfo_proc;
  let mut i: usize;

  let mut proclist: Lisp_Object = Qnil ();

  if sysctl (mib.as_mut_ptr (), mibsize as c_uint, std::ptr::null_mut (), &mut len,
	     std::ptr::null_mut (), 0) != 0 || len == 0
    { return proclist; }

  procs = xmalloc (len) as *mut kinfo_proc;
  if sysctl (mib.as_mut_ptr (), mibsize as c_uint, procs as *mut c_void, &mut len,
	     std::ptr::null_mut (), 0) != 0 || len == 0
    {
      xfree (procs as *mut c_void);
      return proclist;
    }

  len /= std::mem::size_of::<kinfo_proc> ();
  i = 0;
  while i < len
    {
      /* C 3379-3385: `#ifdef DARWIN_OS` is live.  */
      {
	proclist = Fcons (INT_TO_INTEGER ((*procs.add (i)).kp_proc.p_pid as EMACS_INT), proclist);
      }
      i += 1;
    }

  xfree (procs as *mut c_void);

  return  proclist;
}

/* The WINDOWSNT implementation is in w32.c.
   The MSDOS implementation is in dosfns.c.
   The Haiku implementation is in haiku.c.  */
// C 3396-3404: `#elif !defined (WINDOWSNT) && !defined (MSDOS) && !defined (HAIKU)` — dead
// (clang's chain takes the DARWIN_OS arm above); carried.
/* C 3396-3404: `#elif !defined (WINDOWSNT) && !defined (MSDOS) && !defined (HAIKU)` - the
   complement of the two arms above, spelled as such so the three stay exclusive.  */
#[cfg (all (not (feature = "have_procfs"),
            not (target_os = "macos"), not (target_os = "freebsd"), not (target_os = "openbsd"),
            not (feature = "windowsnt"), not (feature = "msdos"), not (feature = "haiku")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn list_system_processes () -> Lisp_Object {
  return Qnil ();
}

// C 3406-3419: `#if (HAVE_GETRUSAGE || defined __FreeBSD__ || defined DARWIN_OS || defined
// __OpenBSD__)` — live (`HAVE_GETRUSAGE` is 1).
unsafe fn make_lisp_s_us (s: time_t, us: c_long) -> Lisp_Object {
  let sec: Lisp_Object = make_int (s as intmax_t);
  let usec: Lisp_Object = make_fixnum (us as EMACS_INT);
  let hz: Lisp_Object = make_fixnum (1000000);
  let ticks: Lisp_Object = CALLN! (Fplus, CALLN! (Ftimes, sec, hz), usec);
  return Ftime_convert (Fcons (ticks, hz), Qnil ());
}

// C 3421-3429: `#if defined __FreeBSD__ || defined DARWIN_OS` — live (DARWIN_OS).
unsafe fn make_lisp_timeval (t: timeval) -> Lisp_Object {
  return make_lisp_s_us (t.tv_sec, t.tv_usec as c_long);
}

// C 3431-3810: `#if defined (GNU_LINUX) || defined (CYGWIN) || defined __ANDROID__` — dead;
// carried in full.
#[cfg (feature = "gnu_linux")]
unsafe fn time_from_jiffies (ticks: c_ulonglong, hz: Lisp_Object, form: Lisp_Object)
  -> Lisp_Object {
  return Ftime_convert (Fcons (make_uint (ticks as uintmax_t), hz), form);
}

#[cfg (feature = "gnu_linux")]
unsafe fn put_jiffies (attrs: Lisp_Object, propname: Lisp_Object, ticks: c_ulonglong,
		       hz: Lisp_Object) -> Lisp_Object {
  return Fcons (Fcons (propname, time_from_jiffies (ticks, hz, Qnil ())), attrs);
}

/* Return the host uptime with resolution HZ if successful, otherwise nil.
   Do not use get_boot_time, which returns a container's
   boot time instead of the underlying host's boot time.  */
#[cfg (feature = "gnu_linux")]
unsafe fn get_host_uptime (hz: Lisp_Object) -> Lisp_Object {
  /* clock_gettime is available in glibc 2.14+, Android, and musl libc.  */
  /* C 3453's `# if !defined __GLIBC__ || 2 < __GLIBC__ + (14 <= __GLIBC_MINOR__)` is
     live (`__GLIBC__` is undefined, so `!defined` is true).  */
  {
    let mut upt: timespec = std::mem::zeroed ();
    if 0 <= clock_gettime (CLOCK_BOOTTIME, &mut upt)
      { return Ftime_convert (timespec_to_lisp (upt), hz); }
  }
  return Qnil ();
}

// C 3461-3507: `# if defined GNU_LINUX || defined __ANDROID__` — dead; carried.

/// C 3462-3463's `MAJOR`/`MINOR` (the Linux device-number decoders).
#[cfg (feature = "gnu_linux")]
macro_rules! MAJOR { ($d:expr) => { (($d as c_uint) >> 8) & 0xfff }; }
#[cfg (feature = "gnu_linux")]
macro_rules! MINOR { ($d:expr) => {
  (($d as c_uint) & 0xff) | ((($d as c_uint) & 0xfff00000) >> 12)
}; }

#[cfg (feature = "gnu_linux")]
unsafe fn procfs_ttyname (rdev: c_int) -> Lisp_Object {
  let mut fdev: *mut FILE;
  let mut name: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];

  block_input ();
  fdev = emacs_fopen (c"/proc/tty/drivers".as_ptr (), c"r".as_ptr ());
  name[0] = 0;

  if !fdev.is_null ()
    {
      let mut major: c_uint = 0;
      let mut minor_beg: c_ulong = 0;
      let mut minor_end: c_ulong = 0;
      let mut minor: [c_char; 25] = [0; 25];	/* 2 32-bit numbers + dash */
      let mut endp: *mut c_char;

      while feof (fdev) == 0 && ferror (fdev) == 0
	{
	  name[0] = 0;
	  if fscanf (fdev, c"%*s %s %u %s %*s\n".as_ptr (), name.as_mut_ptr (), &mut major,
		     minor.as_mut_ptr ()) >= 3
	      && major == MAJOR! (rdev)
	    {
	      minor_beg = strtoul (minor.as_ptr (), &mut endp, 0);
	      if *endp == 0
		{ minor_end = minor_beg; }
	      else if *endp == b'-' as c_char
		{ minor_end = strtoul (endp.add (1), &mut endp, 0); }
	      else
		{ continue; }

	      if MINOR! (rdev) >= minor_beg && MINOR! (rdev) <= minor_end
		{
		  sprintf (name.as_mut_ptr ().add (strlen (name.as_ptr ())),
			   c"%u".as_ptr (), MINOR! (rdev));
		  break;
		}
	    }
	}
      emacs_fclose (fdev);
    }
  unblock_input ();
  return build_string (name.as_ptr ());
}

/* Total usable RAM in KiB.  */
#[cfg (feature = "gnu_linux")]
unsafe fn procfs_get_total_memory () -> uintmax_t {
  let mut fmem: *mut FILE;
  let mut retval: uintmax_t = 2 * 1024 * 1024; /* default: 2 GiB */
  let mut c: c_int;

  block_input ();
  fmem = emacs_fopen (c"/proc/meminfo".as_ptr (), c"r".as_ptr ());

  if !fmem.is_null ()
    {
      let mut entry_value: uintmax_t = 0;
      let mut done: bool = false;

      loop
	{
	  let scanned: c_int = fscanf (fmem, c"MemTotal: %ju".as_ptr (), &mut entry_value);
	  match scanned
	    {
	      1 =>
		{
		  retval = entry_value;
		  done = true;
		}
	      0 =>
		{
		  loop
		    {
		      c = getc (fmem);
		      if !(c != EOF && c != b'\n' as c_int)
			{ break; }
		    }
		  done = c == EOF;
		}
	      _ =>
		{
		  done = true;
		}
	    }
	  if done
	    { break; }
	}

      emacs_fclose (fmem);
    }
  unblock_input ();
  return retval;
}

#[cfg (feature = "gnu_linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_process_attributes (pid: Lisp_Object) -> Lisp_Object {
  let mut procfn: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
  let mut fn_: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
  let mut st: stat = std::mem::zeroed ();
  let mut pw: *mut passwd;
  let mut gr: *mut group;
  let mut clocks_per_sec: c_long;
  let mut procfn_end: *mut c_char;
  let mut procbuf: [c_char; 1025] = [0; 1025];
  let mut p: *mut c_char;
  let mut q: *mut c_char;
  let mut fd: c_int;
  let mut nread: ssize_t;
  static default_cmd: [c_char; 4] = *b"???\0";
  let mut cmd: *const c_char = default_cmd.as_ptr ();
  let mut cmdsize: c_int = std::mem::size_of::<[c_char; 4]> () as c_int - 1;
  let mut cmdline: *mut c_char = std::ptr::null_mut ();
  let mut cmdline_size: ptrdiff_t = 0;
  let mut c: c_char = 0;
  let mut proc_id: intmax_t;
  let mut ppid: c_int = 0;
  let mut pgrp: c_int = 0;
  let mut sess: c_int = 0;
  let mut tty: c_int = 0;
  let mut tpgid: c_int = 0;
  let mut thcount: c_int = 0;
  let mut uid: uid_t;
  let mut gid: gid_t;
  let mut u_time: c_ulonglong = 0;
  let mut s_time: c_ulonglong = 0;
  let mut cutime: c_ulonglong = 0;
  let mut cstime: c_ulonglong = 0;
  let mut start: c_ulonglong = 0;
  let mut priority: c_long = 0;
  let mut niceness: c_long = 0;
  let mut rss: c_long = 0;
  let mut minflt: c_ulong = 0;
  let mut majflt: c_ulong = 0;
  let mut cminflt: c_ulong = 0;
  let mut cmajflt: c_ulong = 0;
  let mut vsize: c_ulong = 0;
  let mut pcpu: f64;
  let mut pmem: f64;
  let mut attrs: Lisp_Object = Qnil ();
  let mut decoded_cmd: Lisp_Object;

  CHECK_NUMBER (pid);
  CONS_TO_INTEGER! (pid, proc_id);
  sprintf (procfn.as_mut_ptr (), c"/proc/%jd".as_ptr (), proc_id);
  if stat (procfn.as_ptr (), &mut st) < 0
    { return attrs; }

  /* euid egid */
  uid = st.st_uid;
  attrs = Fcons (Fcons (Qeuid (), INT_TO_INTEGER (uid as EMACS_INT)), attrs);
  block_input ();
  pw = getpwuid (uid);
  unblock_input ();
  if !pw.is_null ()
    { attrs = Fcons (Fcons (Quser (), build_string ((*pw).pw_name)), attrs); }

  gid = st.st_gid;
  attrs = Fcons (Fcons (Qegid (), INT_TO_INTEGER (gid as EMACS_INT)), attrs);
  block_input ();
  gr = getgrgid (gid);
  unblock_input ();
  if !gr.is_null ()
    { attrs = Fcons (Fcons (Qgroup (), build_string ((*gr).gr_name)), attrs); }

  let count: specpdl_ref = SPECPDL_INDEX ();
  strcpy (fn_.as_mut_ptr (), procfn.as_ptr ());
  procfn_end = fn_.as_mut_ptr ().add (strlen (fn_.as_ptr ()));
  strcpy (procfn_end, c"/stat".as_ptr ());
  fd = emacs_open (fn_.as_ptr (), O_RDONLY, 0);
  if fd < 0
    { nread = 0; }
  else
    {
      record_unwind_protect_int (close_file_unwind, fd);
      nread = emacs_read_quit (fd, procbuf.as_mut_ptr () as *mut c_void,
			       std::mem::size_of::<[c_char; 1025]> () as ptrdiff_t - 1);
    }
  if 0 < nread
    {
      procbuf[nread as usize] = 0;
      p = procbuf.as_mut_ptr ();

      p = strchr (p, b'(' as c_int);
      if !p.is_null ()
	{
	  q = strrchr (p.add (1), b')' as c_int);
	  /* comm */
	  if !q.is_null ()
	    {
	      cmd = p.add (1);
	      cmdsize = (q as isize - cmd as isize) as c_int;
	    }
	}
      else
	{ q = std::ptr::null_mut (); }
      /* Command name is encoded in locale-coding-system; decode it.  */
      AUTO_STRING_WITH_LEN! (cmd_str, cmd, cmdsize as ptrdiff_t);
      decoded_cmd = code_convert_string_norecord (cmd_str, Vlocale_coding_system, false);
      attrs = Fcons (Fcons (Qcomm (), decoded_cmd), attrs);

      /* state ppid pgrp sess tty tpgid . minflt cminflt majflt cmajflt
	 utime stime cutime cstime priority nice thcount . start vsize rss */
      if !q.is_null ()
	  && sscanf (q.add (2),
		     c"%c %d %d %d %d %d %*u %lu %lu %lu %lu %llu %llu %llu %llu %ld %ld %d %*d %llu %lu %ld".as_ptr (),
		     &mut c, &mut ppid, &mut pgrp, &mut sess, &mut tty, &mut tpgid,
		     &mut minflt, &mut cminflt, &mut majflt, &mut cmajflt,
		     &mut u_time, &mut s_time, &mut cutime, &mut cstime,
		     &mut priority, &mut niceness, &mut thcount, &mut start, &mut vsize, &mut rss)
	      == 20
	{
	  let mut state_str: [c_char; 2] = [0; 2];
	  state_str[0] = c;
	  state_str[1] = 0;
	  attrs = Fcons (Fcons (Qstate (), build_string (state_str.as_ptr ())), attrs);
	  attrs = Fcons (Fcons (Qppid (), INT_TO_INTEGER (ppid as EMACS_INT)), attrs);
	  attrs = Fcons (Fcons (Qpgrp (), INT_TO_INTEGER (pgrp as EMACS_INT)), attrs);
	  attrs = Fcons (Fcons (Qsess (), INT_TO_INTEGER (sess as EMACS_INT)), attrs);
	  attrs = Fcons (Fcons (Qttname (), procfs_ttyname (tty)), attrs);
	  attrs = Fcons (Fcons (Qtpgid (), INT_TO_INTEGER (tpgid as EMACS_INT)), attrs);
	  attrs = Fcons (Fcons (Qminflt (), INT_TO_INTEGER (minflt as EMACS_INT)), attrs);
	  attrs = Fcons (Fcons (Qmajflt (), INT_TO_INTEGER (majflt as EMACS_INT)), attrs);
	  attrs = Fcons (Fcons (Qcminflt (), INT_TO_INTEGER (cminflt as EMACS_INT)), attrs);
	  attrs = Fcons (Fcons (Qcmajflt (), INT_TO_INTEGER (cmajflt as EMACS_INT)), attrs);

	  clocks_per_sec = sysconf (_SC_CLK_TCK) as c_long;
	  if 0 < clocks_per_sec
	    {
	      let hz: Lisp_Object = make_int (clocks_per_sec as intmax_t);
	      attrs = put_jiffies (attrs, Qutime (), u_time, hz);
	      attrs = put_jiffies (attrs, Qstime (), s_time, hz);
	      attrs = put_jiffies (attrs, Qtime (), s_time + u_time, hz);
	      attrs = put_jiffies (attrs, Qcutime (), cutime, hz);
	      attrs = put_jiffies (attrs, Qcstime (), cstime, hz);
	      attrs = put_jiffies (attrs, Qctime (), cstime + cutime, hz);

	      let uptime: Lisp_Object = get_host_uptime (hz);
	      if !NILP (uptime)
		{
		  let now: Lisp_Object = Ftime_convert (Qnil (), hz);
		  let boot: Lisp_Object = Ftime_subtract (now, uptime);
		  let tstart: Lisp_Object = time_from_jiffies (start, hz, hz);
		  let lstart: Lisp_Object =
		    Ftime_convert (Ftime_add (boot, tstart), Qnil ());
		  attrs = Fcons (Fcons (Qstart (), lstart), attrs);
		  let etime: Lisp_Object =
		    Ftime_convert (Ftime_subtract (uptime, tstart), Qnil ());
		  attrs = Fcons (Fcons (Qetime (), etime), attrs);
		  pcpu = 100.0 * (s_time + u_time) as f64
			  / (clocks_per_sec as f64 * float_time (etime));
		  attrs = Fcons (Fcons (Qpcpu (), make_float (pcpu)), attrs);
		}
	    }

	  attrs = Fcons (Fcons (Qpri (), make_fixnum (priority as EMACS_INT)), attrs);
	  attrs = Fcons (Fcons (Qnice (), make_fixnum (niceness as EMACS_INT)), attrs);
	  attrs = Fcons (Fcons (Qthcount (), INT_TO_INTEGER (thcount as EMACS_INT)), attrs);
	  attrs = Fcons (Fcons (Qvsize (), INT_TO_INTEGER ((vsize / 1024) as EMACS_INT)), attrs);

	  /* RSS in KiB.  */
	  let mut rssk: uintmax_t = rss as uintmax_t;
	  rssk *= (getpagesize () >> 10) as uintmax_t;

	  attrs = Fcons (Fcons (Qrss (), INT_TO_INTEGER (rssk as EMACS_INT)), attrs);
	  pmem = 100.0 * rssk as f64 / procfs_get_total_memory () as f64;
	  if pmem > 100.0
	    { pmem = 100.0; }
	  attrs = Fcons (Fcons (Qpmem (), make_float (pmem)), attrs);
	}
    }
  unbind_to (count, Qnil ());

  // C 3713-3731: `# ifdef CYGWIN` is dead; carried.
  #[cfg (feature = "cygwin")]
  {
    /* ttname */
    strcpy (procfn_end, c"/ctty".as_ptr ());
    fd = emacs_open (fn_.as_ptr (), O_RDONLY, 0);
    if fd < 0
      { nread = 0; }
    else
      {
	record_unwind_protect_int (close_file_unwind, fd);
	nread = emacs_read_quit (fd, procbuf.as_mut_ptr () as *mut c_void,
				 std::mem::size_of::<[c_char; 1025]> () as ptrdiff_t);
      }
    /* /proc/<pid>/ctty should always end in newline. */
    if 0 < nread && procbuf[(nread - 1) as usize] == b'\n' as c_char
      { procbuf[(nread - 1) as usize] = 0; }
    else
      { procbuf[0] = 0; }
    attrs = Fcons (Fcons (Qttname (), build_string (procbuf.as_ptr ())), attrs);
    unbind_to (count, Qnil ());
  }

  /* args */
  strcpy (procfn_end, c"/cmdline".as_ptr ());
  fd = emacs_open (fn_.as_ptr (), O_RDONLY, 0);
  if fd >= 0
    {
      let mut readsize: ptrdiff_t;
      let mut nread_incr: ptrdiff_t;
      record_unwind_protect_int (close_file_unwind, fd);
      record_unwind_protect_nothing ();
      nread = 0;
      cmdline_size = 0;

      loop
	{
	  cmdline = xpalloc (cmdline as *mut c_void, &mut cmdline_size, 2,
			     STRING_BYTES_BOUND, 1) as *mut c_char;
	  set_unwind_protect_ptr (specpdl_ref_add (count, 1), Some (xfree),
				  cmdline as *mut c_void);

	  /* Leave room even if every byte needs escaping below.  */
	  readsize = (cmdline_size >> 1) - nread;

	  nread_incr = emacs_read_quit (fd, cmdline.add (nread as usize) as *mut c_void,
					readsize);
	  nread += std::cmp::max (0, nread_incr);
	  if nread_incr != readsize
	    { break; }
	}

      if nread != 0
	{
	  /* We don't want trailing null characters.  */
	  p = cmdline.add (nread as usize);
	  while cmdline < p && *p.offset (-1) == 0
	    { p = p.offset (-1); }

	  /* Escape-quote whitespace and backslashes.  */
	  q = cmdline.add (cmdline_size as usize);
	  while cmdline < p
	    {
	      p = p.offset (-1);
	      let c: c_char = *p;
	      q = q.offset (-1);
	      *q = if c != 0 { c } else { b' ' as c_char };
	      if c_isspace (c as c_int) || c == b'\\' as c_char
		{ q = q.offset (-1); *q = b'\\' as c_char; }
	    }

	  nread = (cmdline as isize + cmdline_size - q as isize) as ssize_t;
	}

      if nread == 0
	{
	  nread = (cmdsize + 2) as ssize_t;
	  cmdline_size = nread as ptrdiff_t + 1;
	  cmdline = xrealloc (cmdline as *mut c_void, cmdline_size as usize) as *mut c_char;
	  q = cmdline;
	  set_unwind_protect_ptr (specpdl_ref_add (count, 1), Some (xfree),
				  cmdline as *mut c_void);
	  sprintf (cmdline, c"[%.*s]".as_ptr (), cmdsize, cmd);
	}
      /* Command line is encoded in locale-coding-system; decode it.  */
      AUTO_STRING_WITH_LEN! (cmd_str, q, nread as ptrdiff_t);
      decoded_cmd = code_convert_string_norecord (cmd_str, Vlocale_coding_system, false);
      unbind_to (count, Qnil ());
      attrs = Fcons (Fcons (Qargs (), decoded_cmd), attrs);
    }

  return attrs;
}

// C 3794-3927: `#elif defined (SOLARIS2) && defined (HAVE_PROCFS)` — dead; carried.
#[cfg (feature = "solaris2")]
pub const PROCFS_FILE_OFFSET_BITS_HACK: c_int = 0;

#[cfg (feature = "solaris2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_process_attributes (pid: Lisp_Object) -> Lisp_Object {
  let mut procfn: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
  let mut fn_: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
  let mut st: stat = std::mem::zeroed ();
  let mut pw: *mut passwd;
  let mut gr: *mut group;
  let mut procfn_end: *mut c_char;
  let mut pinfo: psinfo = std::mem::zeroed ();
  let mut fd: c_int;
  let mut nread: ssize_t;
  let mut proc_id: intmax_t;
  let mut uid: uid_t;
  let mut gid: gid_t;
  let mut attrs: Lisp_Object = Qnil ();
  let mut decoded_cmd: Lisp_Object;

  CHECK_NUMBER (pid);
  /* The mirror's `ConsToInteger` covers the Lisp word types (i64/u64), so C's
     `CONS_TO_INTEGER (pid, pid_t, proc_id)` goes through the `i64` temporary
     (the callint.rs shape).  */
  let mut pid_i64: i64 = 0;
  CONS_TO_INTEGER! (pid, pid_i64);
  proc_id = pid_i64;
  sprintf (procfn.as_mut_ptr (), c"/proc/%jd".as_ptr (), proc_id);
  if stat (procfn.as_ptr (), &mut st) < 0
    { return attrs; }

  /* euid egid */
  uid = st.st_uid;
  attrs = Fcons (Fcons (Qeuid (), INT_TO_INTEGER (uid as EMACS_INT)), attrs);
  block_input ();
  pw = getpwuid (uid);
  unblock_input ();
  if !pw.is_null ()
    { attrs = Fcons (Fcons (Quser (), build_string ((*pw).pw_name)), attrs); }

  gid = st.st_gid;
  attrs = Fcons (Fcons (Qegid (), INT_TO_INTEGER (gid as EMACS_INT)), attrs);
  block_input ();
  gr = getgrgid (gid);
  unblock_input ();
  if !gr.is_null ()
    { attrs = Fcons (Fcons (Qgroup (), build_string ((*gr).gr_name)), attrs); }

  let count: specpdl_ref = SPECPDL_INDEX ();
  strcpy (fn_.as_mut_ptr (), procfn.as_ptr ());
  procfn_end = fn_.as_mut_ptr ().add (strlen (fn_.as_ptr ()));
  strcpy (procfn_end, c"/psinfo".as_ptr ());
  fd = emacs_open (fn_.as_ptr (), O_RDONLY, 0);
  if fd < 0
    { nread = 0; }
  else
    {
      record_unwind_protect_int (close_file_unwind, fd);
      nread = emacs_read_quit (fd, &mut pinfo as *mut psinfo as *mut c_void,
			       std::mem::size_of::<psinfo> () as ptrdiff_t);
    }

  if nread == std::mem::size_of::<psinfo> () as ssize_t
    {
      attrs = Fcons (Fcons (Qppid (), INT_TO_INTEGER (pinfo.pr_ppid as EMACS_INT)), attrs);
      attrs = Fcons (Fcons (Qpgrp (), INT_TO_INTEGER (pinfo.pr_pgid as EMACS_INT)), attrs);
      attrs = Fcons (Fcons (Qsess (), INT_TO_INTEGER (pinfo.pr_sid as EMACS_INT)), attrs);

      {
	let mut state_str: [c_char; 2] = [0; 2];
	state_str[0] = pinfo.pr_lwp.pr_sname;
	state_str[1] = 0;
	attrs = Fcons (Fcons (Qstate (), build_string (state_str.as_ptr ())), attrs);
      }

      /* FIXME: missing Qttyname. psinfo.pr_ttydev is a dev_t,
	 need to get a string from it.  */

      /* FIXME: missing: Qtpgid */

      /* FIXME: missing:
	    Qminflt
	    Qmajflt
	    Qcminflt
	    Qcmajflt

	    Qutime
	    Qcutime
	    Qstime
	    Qcstime
	    Are they available? */

      attrs = Fcons (Fcons (Qtime (), make_lisp_time (pinfo.pr_time)), attrs);
      attrs = Fcons (Fcons (Qctime (), make_lisp_time (pinfo.pr_ctime)), attrs);
      attrs = Fcons (Fcons (Qpri (), make_fixnum (pinfo.pr_lwp.pr_pri as EMACS_INT)), attrs);
      attrs = Fcons (Fcons (Qnice (), make_fixnum (pinfo.pr_lwp.pr_nice as EMACS_INT)), attrs);
      attrs = Fcons (Fcons (Qthcount (), INT_TO_INTEGER (pinfo.pr_nlwp as EMACS_INT)), attrs);

      attrs = Fcons (Fcons (Qstart (), make_lisp_time (pinfo.pr_start)), attrs);
      attrs = Fcons (Fcons (Qvsize (), INT_TO_INTEGER (pinfo.pr_size as EMACS_INT)), attrs);
      attrs = Fcons (Fcons (Qrss (), INT_TO_INTEGER (pinfo.pr_rssize as EMACS_INT)), attrs);

      /* pr_pctcpu and pr_pctmem are unsigned integers in the
	 range 0 .. 2**15, representing 0.0 .. 1.0.  */
      attrs = Fcons (Fcons (Qpcpu (),
			    make_float (100.0 / 0x8000 as f64 * pinfo.pr_pctcpu as f64)),
		     attrs);
      attrs = Fcons (Fcons (Qpmem (),
			    make_float (100.0 / 0x8000 as f64 * pinfo.pr_pctmem as f64)),
		     attrs);

      AUTO_STRING! (fname, pinfo.pr_fname.as_ptr ());
      decoded_cmd = code_convert_string_norecord (fname, Vlocale_coding_system, false);
      attrs = Fcons (Fcons (Qcomm (), decoded_cmd), attrs);
      AUTO_STRING! (psargs, pinfo.pr_psargs.as_ptr ());
      decoded_cmd = code_convert_string_norecord (psargs, Vlocale_coding_system, false);
      attrs = Fcons (Fcons (Qargs (), decoded_cmd), attrs);
    }
  return unbind_to (count, attrs);
}

/// `<procfs.h>`'s `struct psinfo` (SunOS): C 3805's `#include <procfs.h>`; the frozen target
/// has no such header, so the record carries the members the arm names and no measured
/// layout (the arm is never compiled — docs/ports/sysdep.md records the deviation).
#[cfg (feature = "solaris2")]
#[repr(C)]
pub struct psinfo {
  pub pr_ppid: pid_t,
  pub pr_pgid: pid_t,
  pub pr_sid: pid_t,
  pub pr_time: timespec,
  pub pr_ctime: timespec,
  pub pr_size: usize,
  pub pr_rssize: usize,
  pub pr_nlwp: c_int,
  pub pr_pctcpu: c_ushort,
  pub pr_pctmem: c_ushort,
  pub pr_fname: [c_char; 16],
  pub pr_psargs: [c_char; 80],
  pub pr_lwp: lwpsinfo,
}

/// `<procfs.h>`'s `lwpsinfo` (SunOS).
#[cfg (feature = "solaris2")]
#[repr(C)]
pub struct lwpsinfo {
  pub pr_sname: c_char,
  pub pr_pri: c_int,
  pub pr_nice: c_int,
}

// C 3927-4090: `#elif defined __FreeBSD__` — dead; carried.
#[cfg (feature = "freebsd")]
/// `<sys/user.h>`'s `struct kinfo_proc` (FreeBSD): the members C 3953-4075 names.  The
/// frozen target cannot compile this arm, so the layout is the vendor's field list and no
/// `const _` assertion is possible (docs/ports/sysdep.md records the deviation).
#[repr(C)]
pub struct kinfo_proc {
  pub ki_uid: uid_t,
  pub ki_svgid: gid_t,
  pub ki_comm: [c_char; 20],
  pub ki_stat: c_char,
  pub ki_ppid: pid_t,
  pub ki_pgid: pid_t,
  pub ki_sid: pid_t,
  pub ki_tdev: dev_t,
  pub ki_tpgid: pid_t,
  pub ki_rusage: rusage,
  pub ki_rusage_ch: rusage,
  pub ki_numthreads: c_int,
  pub ki_pri: pri,
  pub ki_nice: c_char,
  pub ki_start: timeval,
  pub ki_size: c_ulong,
  pub ki_rssize: c_long,
  pub ki_pctcpu: fixpt_t,
  pub ki_swtime: c_uint,
  pub ki_flag: c_int,
  pub p_pid: pid_t,
}

/// `<sys/user.h>`'s `struct pri` (FreeBSD).
#[cfg (feature = "freebsd")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pri {
  pub pri_native: c_int,
}

/// `<sys/types.h>`'s `fixpt_t` (FreeBSD).
#[cfg (feature = "freebsd")]
pub type fixpt_t = c_uint;

/// `<sys/user.h>`: `#define P_INMEM 0x00080000` (FreeBSD).
#[cfg (feature = "freebsd")]
pub const P_INMEM: c_int = 0x00080000;

/// `<sys/sysctl.h>`: `#define KERN_PROC_ARGS 7`, `KERN_FSCALE 32` (FreeBSD; the arm is
/// never compiled).
#[cfg (feature = "freebsd")]
pub const KERN_PROC_ARGS: c_int = 7;
#[cfg (feature = "freebsd")]
pub const KERN_FSCALE: c_int = 32;

/// `<sys/proc.h>`'s process states (BSD): SRUN 2, SSLEEP 3, SLOCK 4, SZOMB 5, SSTOP 6,
/// SIDL 1, SDEAD 7 (measured on this target for the five Darwin defines; the other two are
/// the vendor's).
#[cfg (any (feature = "freebsd", feature = "openbsd"))]
pub const SRUN: c_int = 2;
#[cfg (any (feature = "freebsd", feature = "openbsd"))]
pub const SSLEEP: c_int = 3;
#[cfg (feature = "freebsd")]
pub const SLOCK: c_int = 4;
#[cfg (any (feature = "freebsd", feature = "openbsd"))]
pub const SZOMB: c_int = 5;
#[cfg (any (feature = "freebsd", feature = "openbsd"))]
pub const SSTOP: c_int = 6;
#[cfg (feature = "freebsd")]
pub const SDEAD: c_int = 7;

/// `<sys/sysctl.h>`: `#define CTL_VM 2`, `VM_UVMEXP 4`, `KERN_PROC_ARGV 1` (OpenBSD), and
/// `<sys/param.h>`'s `ARG_MAX` (the arm is never compiled).
#[cfg (feature = "openbsd")]
pub const CTL_VM: c_int = 2;
#[cfg (feature = "openbsd")]
pub const VM_UVMEXP: c_int = 4;
#[cfg (feature = "openbsd")]
pub const KERN_PROC_ARGV: c_int = 1;
#[cfg (feature = "openbsd")]
pub const ARG_MAX: usize = 1024 * 1024;
/// `<sys/param.h>`: `#define MAXPATHLEN 1024` (the frozen target's, measured).
#[cfg (feature = "openbsd")]
pub const ARG_MAX_OPENBSD: usize = ARG_MAX;

#[cfg (feature = "freebsd")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_process_attributes (pid: Lisp_Object) -> Lisp_Object {
  let mut proc_id: c_int = 0;
  let pagesize: c_int = getpagesize ();
  let mut npages: c_ulong = 0;
  let mut fscale: c_int = 0;
  let mut pw: *mut passwd;
  let mut gr: *mut group;
  let mut ttyname: *mut c_char;
  let mut len: usize = 0;
  let mut args: [c_char; MAXPATHLEN] = [0; MAXPATHLEN];

  let mut mib: [c_int; 4] = [CTL_KERN, KERN_PROC, KERN_PROC_PID, 0];
  let mut proc: kinfo_proc = std::mem::zeroed ();
  let mut proclen: usize = std::mem::size_of::<kinfo_proc> ();

  let mut attrs: Lisp_Object = Qnil ();
  let mut decoded_comm: Lisp_Object;

  CHECK_NUMBER (pid);
  /* C's `CONS_TO_INTEGER (pid, int, proc_id)` (the mirror's trait covers i64/u64).  */
  let mut pid_i64: i64 = 0;
  CONS_TO_INTEGER! (pid, pid_i64);
  proc_id = pid_i64 as c_int;
  mib[3] = proc_id;

  if sysctl (mib.as_mut_ptr (), 4, &mut proc as *mut kinfo_proc as *mut c_void, &mut proclen,
	     std::ptr::null_mut (), 0) != 0 || proclen == 0
    { return attrs; }

  attrs = Fcons (Fcons (Qeuid (), INT_TO_INTEGER (proc.ki_uid as EMACS_INT)), attrs);

  block_input ();
  pw = getpwuid (proc.ki_uid);
  unblock_input ();
  if !pw.is_null ()
    { attrs = Fcons (Fcons (Quser (), build_string ((*pw).pw_name)), attrs); }

  attrs = Fcons (Fcons (Qegid (), INT_TO_INTEGER (proc.ki_svgid as EMACS_INT)), attrs);

  block_input ();
  gr = getgrgid (proc.ki_svgid);
  unblock_input ();
  if !gr.is_null ()
    { attrs = Fcons (Fcons (Qgroup (), build_string ((*gr).gr_name)), attrs); }

  AUTO_STRING! (comm, proc.ki_comm.as_ptr ());
  decoded_comm = code_convert_string_norecord (comm, Vlocale_coding_system, false);

  attrs = Fcons (Fcons (Qcomm (), decoded_comm), attrs);
  {
    let mut state: [c_char; 2] = [0, 0];
    match proc.ki_stat as c_int
      {
      SRUN => { state[0] = b'R' as c_char; }
      SSLEEP => { state[0] = b'S' as c_char; }
      SLOCK => { state[0] = b'D' as c_char; }
      SZOMB => { state[0] = b'Z' as c_char; }
      SSTOP => { state[0] = b'T' as c_char; }
      _ => { }
      }
    attrs = Fcons (Fcons (Qstate (), build_string (state.as_ptr ())), attrs);
  }

  attrs = Fcons (Fcons (Qppid (), INT_TO_INTEGER (proc.ki_ppid as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qpgrp (), INT_TO_INTEGER (proc.ki_pgid as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qsess (), INT_TO_INTEGER (proc.ki_sid as EMACS_INT)),  attrs);

  block_input ();
  ttyname = if proc.ki_tdev == NODEV { std::ptr::null_mut () }
	    else { devname (proc.ki_tdev, S_IFCHR) };
  unblock_input ();
  if !ttyname.is_null ()
    { attrs = Fcons (Fcons (Qttname (), build_string (ttyname)), attrs); }

  attrs = Fcons (Fcons (Qtpgid (),   INT_TO_INTEGER (proc.ki_tpgid as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qminflt (),  INT_TO_INTEGER (proc.ki_rusage.ru_minflt as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qmajflt (),  INT_TO_INTEGER (proc.ki_rusage.ru_majflt as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qcminflt (), make_fixnum (proc.ki_rusage_ch.ru_minflt as EMACS_INT)),
		 attrs);
  attrs = Fcons (Fcons (Qcmajflt (), make_fixnum (proc.ki_rusage_ch.ru_majflt as EMACS_INT)),
		 attrs);

  let utime: Lisp_Object = make_lisp_timeval (proc.ki_rusage.ru_utime);
  attrs = Fcons (Fcons (Qutime (), utime), attrs);
  let stime: Lisp_Object = make_lisp_timeval (proc.ki_rusage.ru_stime);
  attrs = Fcons (Fcons (Qstime (), stime), attrs);
  attrs = Fcons (Fcons (Qtime (), Ftime_add (utime, stime)), attrs);

  let cutime: Lisp_Object = make_lisp_timeval (proc.ki_rusage_ch.ru_utime);
  attrs = Fcons (Fcons (Qcutime (), cutime), attrs);
  let cstime: Lisp_Object = make_lisp_timeval (proc.ki_rusage_ch.ru_stime);
  attrs = Fcons (Fcons (Qcstime (), cstime), attrs);
  attrs = Fcons (Fcons (Qctime (), Ftime_add (cutime, cstime)), attrs);

  attrs = Fcons (Fcons (Qthcount (), INT_TO_INTEGER (proc.ki_numthreads as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qpri (),   make_fixnum (proc.ki_pri.pri_native as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qnice (),  make_fixnum (proc.ki_nice as EMACS_INT)), attrs);
  let start: Lisp_Object = make_lisp_timeval (proc.ki_start);
  attrs = Fcons (Fcons (Qstart (), start), attrs);
  attrs = Fcons (Fcons (Qvsize (), make_fixnum ((proc.ki_size >> 10) as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qrss (),   make_fixnum ((proc.ki_rssize * pagesize as c_long >> 10)
						 as EMACS_INT)), attrs);

  let now: Lisp_Object = Ftime_convert (Qnil (), make_fixnum (1000000));
  let etime: Lisp_Object = Ftime_convert (Ftime_subtract (now, start), Qnil ());
  attrs = Fcons (Fcons (Qetime (), etime), attrs);

  len = std::mem::size_of::<c_int> ();
  if sysctlbyname (c"kern.fscale".as_ptr (), &mut fscale as *mut c_int as *mut c_void, &mut len,
		   std::ptr::null_mut (), 0) == 0
    {
      let mut pcpu: f64;
      let mut ccpu: fixpt_t = 0;
      len = std::mem::size_of::<fixpt_t> ();
      if sysctlbyname (c"kern.ccpu".as_ptr (), &mut ccpu as *mut fixpt_t as *mut c_void, &mut len,
		       std::ptr::null_mut (), 0) == 0
	{
	  pcpu = 100.0 * proc.ki_pctcpu as f64 / fscale as f64
		 / (1.0 - exp (proc.ki_swtime as f64 * log (ccpu as f64 / fscale as f64)));
	  attrs = Fcons (Fcons (Qpcpu (), INT_TO_INTEGER (pcpu as EMACS_INT)), attrs);
	}
    }

  len = std::mem::size_of::<c_ulong> ();
  if sysctlbyname (c"hw.availpages".as_ptr (), &mut npages as *mut c_ulong as *mut c_void,
		   &mut len, std::ptr::null_mut (), 0) == 0
    {
      let pmem: f64 =
	if (proc.ki_flag & P_INMEM) != 0 { 100.0 * proc.ki_rssize as f64 / npages as f64 }
	else { 0.0 };
      attrs = Fcons (Fcons (Qpmem (), INT_TO_INTEGER (pmem as EMACS_INT)), attrs);
    }

  mib[2] = KERN_PROC_ARGS;
  len = MAXPATHLEN;
  if sysctl (mib.as_mut_ptr (), 4, args.as_mut_ptr () as *mut c_void, &mut len,
	     std::ptr::null_mut (), 0) == 0 && len != 0
    {
      let mut i: c_int = 0;
      while i < len as c_int
	{
	  if args[i as usize] == 0 && (i as usize) < len - 1
	    { args[i as usize] = b' ' as c_char; }
	  i += 1;
	}

      AUTO_STRING! (comm, args.as_ptr ());
      decoded_comm = code_convert_string_norecord (comm, Vlocale_coding_system, false);

      attrs = Fcons (Fcons (Qargs (), decoded_comm), attrs);
    }

  return attrs;
}

// C 4090-4266: `#elif defined __OpenBSD__` — dead; carried.
#[cfg (feature = "openbsd")]
/// `<sys/sysctl.h>`'s `struct kinfo_proc` (OpenBSD): the members C 4114-4245 names.  The
/// frozen target cannot compile this arm, so the layout is the vendor's field list and no
/// `const _` assertion is possible (docs/ports/sysdep.md records the deviation).
#[repr(C)]
pub struct kinfo_proc {
  pub p_uid: uid_t,
  pub p_svgid: gid_t,
  pub p_comm: [c_char; 24],
  pub p_stat: c_char,
  pub p_ppid: pid_t,
  pub p_gid: pid_t,
  pub p_sid: pid_t,
  pub p_tdev: dev_t,
  pub p_tpgid: pid_t,
  pub p_uru_minflt: c_ulong,
  pub p_uru_majflt: c_ulong,
  pub p_uutime_sec: c_long,
  pub p_uutime_usec: c_long,
  pub p_ustime_sec: c_long,
  pub p_ustime_usec: c_long,
  pub p_uctime_sec: c_long,
  pub p_uctime_usec: c_long,
  pub p_priority: c_int,
  pub p_nice: c_int,
  pub p_ustart_sec: c_long,
  pub p_ustart_usec: c_long,
  pub p_vm_tsize: c_int,
  pub p_vm_dsize: c_int,
  pub p_vm_ssize: c_int,
  pub p_vm_rssize: c_int,
  pub p_pctcpu: fixpt_t,
}

/// `<uvm/uvm_extern.h>`'s `struct uvmexp` (OpenBSD).
#[cfg (feature = "openbsd")]
#[repr(C)]
pub struct uvmexp {
  pub npages: c_int,
}

#[cfg (feature = "openbsd")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_process_attributes (pid: Lisp_Object) -> Lisp_Object {
  let mut proc_id: c_int = 0;
  let mut fscale: c_int = 0;
  let mut i: c_int = 0;
  let pagesize: c_int = getpagesize ();
  let mut mib: [c_int; 6] = [0; 6];
  let mut len: usize = 0;
  let mut pct: f64;
  let mut ttyname: *mut c_char;
  let mut args: [c_char; ARG_MAX] = [0; ARG_MAX];
  let mut proc: kinfo_proc = std::mem::zeroed ();
  let mut pw: *mut passwd;
  let mut gr: *mut group;
  let mut uvmexp: uvmexp = std::mem::zeroed ();

  let mut attrs: Lisp_Object = Qnil ();
  let mut decoded_comm: Lisp_Object;

  CHECK_NUMBER (pid);
  /* C's `CONS_TO_INTEGER (pid, int, proc_id)` (the mirror's trait covers i64/u64).  */
  let mut pid_i64: i64 = 0;
  CONS_TO_INTEGER! (pid, pid_i64);
  proc_id = pid_i64 as c_int;

  len = std::mem::size_of::<kinfo_proc> ();
  mib[0] = CTL_KERN;
  mib[1] = KERN_PROC;
  mib[2] = KERN_PROC_PID;
  mib[3] = proc_id;
  mib[4] = len as c_int;
  mib[5] = 1;
  if sysctl (mib.as_mut_ptr (), 6, &mut proc as *mut kinfo_proc as *mut c_void, &mut len,
	     std::ptr::null_mut (), 0) != 0
    { return attrs; }

  attrs = Fcons (Fcons (Qeuid (), INT_TO_INTEGER (proc.p_uid as EMACS_INT)), attrs);

  block_input ();
  pw = getpwuid (proc.p_uid);
  unblock_input ();
  if !pw.is_null ()
    { attrs = Fcons (Fcons (Quser (), build_string ((*pw).pw_name)), attrs); }

  attrs = Fcons (Fcons (Qegid (), INT_TO_INTEGER (proc.p_svgid as EMACS_INT)), attrs);

  block_input ();
  gr = getgrgid (proc.p_svgid);
  unblock_input ();
  if !gr.is_null ()
    { attrs = Fcons (Fcons (Qgroup (), build_string ((*gr).gr_name)), attrs); }

  AUTO_STRING! (comm, proc.p_comm.as_ptr ());
  decoded_comm = code_convert_string_norecord (comm, Vlocale_coding_system, false);
  attrs = Fcons (Fcons (Qcomm (), decoded_comm), attrs);

  {
    let mut state: [c_char; 2] = [0, 0];
    match proc.p_stat as c_int
      {
      SIDL => { state[0] = b'I' as c_char; }
      SRUN => { state[0] = b'R' as c_char; }
      SSLEEP => { state[0] = b'S' as c_char; }
      SSTOP => { state[0] = b'T' as c_char; }
      SZOMB => { state[0] = b'Z' as c_char; }
      SDEAD => { state[0] = b'D' as c_char; }
      _ => { }
      }
    attrs = Fcons (Fcons (Qstate (), build_string (state.as_ptr ())), attrs);
  }

  attrs = Fcons (Fcons (Qppid (), INT_TO_INTEGER (proc.p_ppid as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qpgrp (), INT_TO_INTEGER (proc.p_gid as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qsess (), INT_TO_INTEGER (proc.p_sid as EMACS_INT)),  attrs);

  block_input ();
  ttyname = if proc.p_tdev == NODEV { std::ptr::null_mut () }
	    else { devname (proc.p_tdev, S_IFCHR) };
  unblock_input ();
  if !ttyname.is_null ()
    { attrs = Fcons (Fcons (Qttname (), build_string (ttyname)), attrs); }

  attrs = Fcons (Fcons (Qtpgid (),   INT_TO_INTEGER (proc.p_tpgid as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qminflt (),  INT_TO_INTEGER (proc.p_uru_minflt as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qmajflt (),  INT_TO_INTEGER (proc.p_uru_majflt as EMACS_INT)), attrs);

  /* FIXME: missing cminflt, cmajflt. */

  let utime: Lisp_Object = make_lisp_s_us (proc.p_uutime_sec, proc.p_uutime_usec);
  attrs = Fcons (Fcons (Qutime (), utime), attrs);
  let stime: Lisp_Object = make_lisp_s_us (proc.p_ustime_sec, proc.p_ustime_usec);
  attrs = Fcons (Fcons (Qstime (), stime), attrs);
  attrs = Fcons (Fcons (Qtime (), Ftime_add (utime, stime)), attrs);

  attrs = Fcons (Fcons (Qcutime (), make_lisp_s_us (proc.p_uctime_sec,
						    proc.p_uctime_usec)),
		 attrs);

  /* FIXME: missing cstime and thus ctime. */

  attrs = Fcons (Fcons (Qpri (),   make_fixnum (proc.p_priority as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qnice (),  make_fixnum (proc.p_nice as EMACS_INT)), attrs);

  /* FIXME: missing thcount (thread count) */

  attrs = Fcons (Fcons (Qstart (), make_lisp_s_us (proc.p_ustart_sec,
						   proc.p_ustart_usec)),
		 attrs);

  len = ((proc.p_vm_tsize + proc.p_vm_dsize + proc.p_vm_ssize) as ptrdiff_t
	 * pagesize as ptrdiff_t >> 10) as usize;
  attrs = Fcons (Fcons (Qvsize (), make_fixnum (len as EMACS_INT)), attrs);

  attrs = Fcons (Fcons (Qrss (),   make_fixnum ((proc.p_vm_rssize as isize
						 * pagesize as isize >> 10) as EMACS_INT)),
		 attrs);

  let now: Lisp_Object = Ftime_convert (Qnil (), make_fixnum (1000000));
  let start: Lisp_Object = make_lisp_s_us (proc.p_ustart_sec, proc.p_ustart_usec);
  let etime: Lisp_Object = Ftime_convert (Ftime_subtract (now, start), Qnil ());
  attrs = Fcons (Fcons (Qetime (), etime), attrs);

  len = std::mem::size_of::<c_int> ();
  mib[0] = CTL_KERN;
  mib[1] = KERN_FSCALE;
  if sysctl (mib.as_mut_ptr (), 2, &mut fscale as *mut c_int as *mut c_void, &mut len,
	     std::ptr::null_mut (), 0) != -1
    {
      pct = proc.p_pctcpu as f64 / fscale as f64 * 100.0;
      attrs = Fcons (Fcons (Qpcpu (), make_float (pct)), attrs);
    }

  len = std::mem::size_of::<uvmexp> ();
  mib[0] = CTL_VM;
  mib[1] = VM_UVMEXP;
  if sysctl (mib.as_mut_ptr (), 2, &mut uvmexp as *mut uvmexp as *mut c_void, &mut len,
	     std::ptr::null_mut (), 0) != -1
    {
      pct = (100.0 * proc.p_vm_rssize as f64 / uvmexp.npages as f64);
      attrs = Fcons (Fcons (Qpmem (), make_float (pct)), attrs);
    }

  len = std::mem::size_of::<[c_char; ARG_MAX]> ();
  mib[0] = CTL_KERN;
  mib[1] = KERN_PROC_ARGS;
  mib[2] = proc_id;
  mib[3] = KERN_PROC_ARGV;
  if sysctl (mib.as_mut_ptr (), 4, args.as_mut_ptr () as *mut c_void, &mut len,
	     std::ptr::null_mut (), 0) == 0 && len != 0
    {
      let argv: *mut *mut c_char = args.as_mut_ptr () as *mut *mut c_char;

      /* concatenate argv reusing the existing storage storage.
	 sysctl(8) guarantees that "the buffer pointed to by oldp is
	 filled with an array of char pointers followed by the strings
	 themselves." */
      i = 0;
      while *argv.add (i as usize) != std::ptr::null_mut ()
	{
	  if *argv.add (i as usize + 1) != std::ptr::null_mut ()
	    {
	      len = strlen (*argv.add (i as usize)) as usize;
	      *(*argv.add (i as usize)).add (len) = b' ' as c_char;
	    }
	  i += 1;
	}

      AUTO_STRING! (comm, *argv);
      decoded_comm = code_convert_string_norecord (comm, Vlocale_coding_system, false);
      attrs = Fcons (Fcons (Qargs (), decoded_comm), attrs);
    }

  return attrs;
}

// C 4267-4517: `#elif defined DARWIN_OS`.
/// C 4269-4270's two macros, in C's own expression form (`__MAC_OS_X_VERSION_MIN_REQUIRED`
/// is 260000 on the frozen target, so both are true and both arms of C 4378/4447 are the
/// live ones).
pub const HAVE_RUSAGE_INFO_CURRENT: bool = __MAC_OS_X_VERSION_MIN_REQUIRED >= 101000;
pub const HAVE_PROC_PIDINFO: bool = __MAC_OS_X_VERSION_MIN_REQUIRED >= 1050;

/* The variant had no gate, so it was the *only* one the Android build compiled: C takes 4542's
   fallback there, and this body's `proc_pidinfo`/`proc_pid_rusage`/`mach_timebase_info`/`devname`
   are four of the names the delivery device does not export (status 4.749,
   docs/baseline/android-gaps.tsv).  `DARWIN_OS` is a platform condition, not a configure macro -
   the frozen configuration carries neither it nor `__FreeBSD__` - so it is spelled with
   `target_os`, the same way `list_system_processes` above does it, and the negations carry C's
   `#elif` exclusivity.  */
#[cfg (all (not (feature = "gnu_linux"), not (feature = "cygwin"),
            not (all (feature = "solaris2", feature = "have_procfs")),
            not (feature = "freebsd"), not (feature = "openbsd"), any (target_os = "macos")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_process_attributes (pid: Lisp_Object) -> Lisp_Object {
  let mut proc_id: c_int = 0;
  let mut i: c_int;
  let mut pw: *mut passwd;
  let mut gr: *mut group;
  let mut ttyname: *mut c_char;
  let mut starttime: timeval = std::mem::zeroed ();
  let mut tdev: dev_t;
  let mut uid: uid_t;
  let mut gid: gid_t;

  let mut mib: [c_int; 4] = [CTL_KERN, KERN_PROC, KERN_PROC_PID, 0];
  let mut proc: kinfo_proc = std::mem::zeroed ();
  let mut len: usize = std::mem::size_of::<kinfo_proc> ();

  let mut attrs: Lisp_Object = Qnil ();
  let mut decoded_comm: Lisp_Object;

  CHECK_NUMBER (pid);
  /* C's `CONS_TO_INTEGER (pid, int, proc_id)` (the mirror's trait covers i64/u64).  */
  let mut pid_i64: i64 = 0;
  CONS_TO_INTEGER! (pid, pid_i64);
  proc_id = pid_i64 as c_int;
  mib[3] = proc_id;

  if sysctl (mib.as_mut_ptr (), 4, &mut proc as *mut kinfo_proc as *mut c_void, &mut len,
	     std::ptr::null_mut (), 0) != 0 || len == 0
    { return attrs; }

  uid = proc.kp_eproc.e_ucred.cr_uid;
  attrs = Fcons (Fcons (Qeuid (), INT_TO_INTEGER (uid as EMACS_INT)), attrs);

  block_input ();
  pw = getpwuid (uid);
  unblock_input ();
  if !pw.is_null ()
    { attrs = Fcons (Fcons (Quser (), build_string ((*pw).pw_name)), attrs); }

  gid = proc.kp_eproc.e_pcred.p_svgid;
  attrs = Fcons (Fcons (Qegid (), INT_TO_INTEGER (gid as EMACS_INT)), attrs);

  block_input ();
  gr = getgrgid (gid);
  unblock_input ();
  if !gr.is_null ()
    { attrs = Fcons (Fcons (Qgroup (), build_string ((*gr).gr_name)), attrs); }

  let mut pathbuf: [c_char; PROC_PIDPATHINFO_MAXSIZE as usize] =
    [0; PROC_PIDPATHINFO_MAXSIZE as usize];
  let mut comm: *mut c_char;

  if proc_pidpath (proc_id, pathbuf.as_mut_ptr () as *mut c_void,
		   std::mem::size_of::<[c_char; PROC_PIDPATHINFO_MAXSIZE as usize]> () as u32) > 0
    {
      comm = strrchr (pathbuf.as_mut_ptr (), b'/' as c_int);
      if !comm.is_null ()
	{ comm = comm.add (1); }
      else
	{ comm = pathbuf.as_mut_ptr (); }
    }
  else
    { comm = proc.kp_proc.p_comm.as_mut_ptr (); }

  decoded_comm = code_convert_string_norecord (build_unibyte_string (comm),
					       Vlocale_coding_system, false);
  attrs = Fcons (Fcons (Qcomm (), decoded_comm), attrs);

  {
    let mut state: [c_char; 2] = [0, 0];
    match proc.kp_proc.p_stat
      {
      SRUN => { state[0] = b'R' as c_char; }
      SSLEEP => { state[0] = b'S' as c_char; }
      SZOMB => { state[0] = b'Z' as c_char; }
      SSTOP => { state[0] = b'T' as c_char; }
      SIDL => { state[0] = b'I' as c_char; }
      _ => { }
      }
    attrs = Fcons (Fcons (Qstate (), build_string (state.as_ptr ())), attrs);
  }

  attrs = Fcons (Fcons (Qppid (), INT_TO_INTEGER (proc.kp_eproc.e_ppid as EMACS_INT)), attrs);
  attrs = Fcons (Fcons (Qpgrp (), INT_TO_INTEGER (proc.kp_eproc.e_pgid as EMACS_INT)), attrs);

  tdev = proc.kp_eproc.e_tdev;
  block_input ();
  ttyname = if tdev == NODEV { std::ptr::null_mut () } else { devname (tdev, S_IFCHR) };
  unblock_input ();
  if !ttyname.is_null ()
    { attrs = Fcons (Fcons (Qttname (), build_string (ttyname)), attrs); }

  attrs = Fcons (Fcons (Qtpgid (), INT_TO_INTEGER (proc.kp_eproc.e_tpgid as EMACS_INT)), attrs);

  /* Captured from whichever rusage path is available below, then used
     after 'etime' is known to compute 'pcpu'.  Negative means unknown.  */
  let mut cpu_time_sec: f64 = -1.0;

  // C 4378-4423: `#if HAVE_RUSAGE_INFO_CURRENT` is live (measured true).
  {
    let mut ri: rusage_info_current = std::mem::zeroed ();
    if proc_pid_rusage (proc_id, RUSAGE_INFO_CURRENT,
			&mut ri as *mut rusage_info_current as *mut c_void) == 0
      {
	/* 'ri_user_time' and 'ri_system_time' are expressed in Mach
	   absolute-time units, not nanoseconds.  On Intel Macs the
	   timebase is 1:1 with nanoseconds, but on Apple Silicon it is
	   125/3, so convert through 'mach_timebase_info' to get actual
	   nanoseconds before exposing the values to Lisp.  */
	static mut timebase: mach_timebase_info_data_t = mach_timebase_info_data_t { numer: 0, denom: 0 };
	if timebase.denom == 0
	  { mach_timebase_info (&raw mut timebase); }
	let user_ns: u64 = ri.ri_user_time * timebase.numer as u64 / timebase.denom as u64;
	let sys_ns: u64  = ri.ri_system_time * timebase.numer as u64 / timebase.denom as u64;

	let utime: timespec = make_timespec ((user_ns / TIMESPEC_HZ as u64) as time_t,
					     (user_ns % TIMESPEC_HZ as u64) as c_long);
	let stime: timespec = make_timespec ((sys_ns / TIMESPEC_HZ as u64) as time_t,
					     (sys_ns % TIMESPEC_HZ as u64) as c_long);
	attrs = Fcons (Fcons (Qutime (), make_lisp_time (utime)), attrs);
	attrs = Fcons (Fcons (Qstime (), make_lisp_time (stime)), attrs);
	attrs = Fcons (Fcons (Qtime (), make_lisp_time (timespec_add (utime, stime))), attrs);

	attrs = Fcons (Fcons (Qmajflt (), INT_TO_INTEGER (ri.ri_pageins as EMACS_INT)), attrs);

	cpu_time_sec = (user_ns as f64 + sys_ns as f64) / TIMESPEC_HZ as f64;
    }
  }
  // C 4405-4423's `#else /* !HAVE_RUSAGE_INFO_CURRENT */` arm is dead; carried.
  #[cfg (feature = "no_have_rusage_info_current")]
  {
    let rusage: *mut rusage = proc.kp_proc.p_ru;
    if !rusage.is_null ()
      {
	attrs = Fcons (Fcons (Qminflt (), INT_TO_INTEGER ((*rusage).ru_minflt as EMACS_INT)), attrs);
	attrs = Fcons (Fcons (Qmajflt (), INT_TO_INTEGER ((*rusage).ru_majflt as EMACS_INT)), attrs);

	let utime: Lisp_Object = make_lisp_timeval ((*rusage).ru_utime);
	let stime: Lisp_Object = make_lisp_timeval ((*rusage).ru_stime);
	attrs = Fcons (Fcons (Qutime (), utime), attrs);
	attrs = Fcons (Fcons (Qstime (), stime), attrs);
	attrs = Fcons (Fcons (Qtime (), Ftime_add (utime, stime)), attrs);

	cpu_time_sec = ((*rusage).ru_utime.tv_sec + (*rusage).ru_stime.tv_sec) as f64
	  + ((*rusage).ru_utime.tv_usec + (*rusage).ru_stime.tv_usec) as f64 / 1.0e6;
      }
  }

  starttime = proc.kp_proc.p_starttime;
  attrs = Fcons (Fcons (Qnice (),  make_fixnum (proc.kp_proc.p_nice as EMACS_INT)), attrs);
  let start: Lisp_Object = make_lisp_timeval (starttime);
  attrs = Fcons (Fcons (Qstart (), start), attrs);

  let now: Lisp_Object = Ftime_convert (Qnil (), make_fixnum (1000000));
  let etime: Lisp_Object = Ftime_convert (Ftime_subtract (now, start), Qnil ());
  attrs = Fcons (Fcons (Qetime (), etime), attrs);

  /* Lifetime-average CPU usage: total CPU time divided by elapsed wall
     time, as on GNU/Linux.  Not an instantaneous rate, and it sums
     every thread, so it can exceed 100.  */
  if cpu_time_sec >= 0.0
    {
      let elapsed: f64 = float_time (etime);
      if elapsed > 0.0
	{
	  let pcpu: f64 = 100.0 * cpu_time_sec / elapsed;
	  attrs = Fcons (Fcons (Qpcpu (), make_float (pcpu)), attrs);
	}
    }

  // C 4447-4473: `#if HAVE_PROC_PIDINFO` is live (measured true).
  {
    let mut taskinfo: proc_taskinfo = std::mem::zeroed ();
    if proc_pidinfo (proc_id, PROC_PIDTASKINFO, 0,
		     &mut taskinfo as *mut proc_taskinfo as *mut c_void,
		     std::mem::size_of::<proc_taskinfo> () as c_int) > 0
      {
	let rss_kb: u64 = taskinfo.pti_resident_size / 1024;
	attrs = Fcons (Fcons (Qvsize (), make_fixnum ((taskinfo.pti_virtual_size / 1024) as EMACS_INT)), attrs);
	attrs = Fcons (Fcons (Qrss (), make_fixnum (rss_kb as EMACS_INT)), attrs);
	attrs = Fcons (Fcons (Qthcount (), make_fixnum (taskinfo.pti_threadnum as EMACS_INT)), attrs);

	/* Percentage of physical RAM used by the process's resident set.
	   Mirrors the GNU/Linux branch, which divides RSS by
	   procfs_get_total_memory ().  On Darwin the total comes from
	   sysctl hw.memsize (bytes).  */
	let mut total_mem: u64 = 0;
	let mut tlen: usize = std::mem::size_of::<u64> ();
	let mut mib2: [c_int; 2] = [ CTL_HW, HW_MEMSIZE ];
	if sysctl (mib2.as_mut_ptr (), 2, &mut total_mem as *mut u64 as *mut c_void, &mut tlen,
		   std::ptr::null_mut (), 0) == 0 && total_mem > 0
	  {
	    /* 'pti_resident_size' counts shared pages in full, so this can
	       exceed 100.  */
	    let mut pmem: f64 = 100.0 * rss_kb as f64 / (total_mem as f64 / 1024.0);
	    if pmem > 100.0
	      { pmem = 100.0; }
	    attrs = Fcons (Fcons (Qpmem (), make_float (pmem)), attrs);
	  }
      }
  }

  // C 4475-4509: `#ifdef KERN_PROCARGS2` is live (the frozen table has KERN_PROCARGS2).
  {
    let mut args: [c_char; ARG_MAX] = [0; ARG_MAX];
    mib[1] = KERN_PROCARGS2;
    mib[2] = proc_id;
    len = std::mem::size_of::<[c_char; ARG_MAX]> ();

    if sysctl (mib.as_mut_ptr (), 3, args.as_mut_ptr () as *mut c_void, &mut len,
	       std::ptr::null_mut (), 0) == 0 && len != 0
      {
	let mut start: *mut c_char;
	let mut end: *mut c_char;

	let argc: c_int = *(args.as_ptr () as *const c_int); /* argc is the first int */
	start = args.as_mut_ptr ().add (std::mem::size_of::<c_int> ());

	start = start.add (strlen (start) + 1); /* skip executable name and any '\0's */
	while (start as usize - args.as_ptr () as usize) < len && *start == 0
	  { start = start.add (1); }

	/* skip argv to find real end */
	i = 0;
	end = start;
	while i < argc && (end as usize - args.as_ptr () as usize) < len
	  {
	    end = end.add (strlen (end) + 1);
	    i += 1;
	  }

	len = end as usize - start as usize;
	let mut i: c_int = 0;
	while i < len as c_int
	  {
	    if *start.add (i as usize) == 0 && (i as usize) < len - 1
	      { *start.add (i as usize) = b' ' as c_char; }
	    i += 1;
	  }

	AUTO_STRING! (comm, start);
	decoded_comm = code_convert_string_norecord (comm, Vlocale_coding_system, false);
	attrs = Fcons (Fcons (Qargs (), decoded_comm), attrs);
      }
  }

  return attrs;
}

/// `<sys/param.h>`: `#define ARG_MAX (1024 * 1024)` (measured on this target).
pub const ARG_MAX: usize = 1024 * 1024;

/// `<sys/proc.h>`: SIDL 1, SRUN 2, SSLEEP 3, SSTOP 4, SZOMB 5 (measured for the Darwin arm,
/// which is the live one; the FreeBSD/OpenBSD arms declare their own).
pub const SIDL: c_int = 1;

/// `#define CTL_HW 6` — `sys_sysctl.rs` carries `CTL_HW`/`HW_NCPU`; C 4439's `CTL_HW` is the
/// mirror's.

/* The WINDOWSNT implementation is in w32.c.
   The MSDOS implementation is in dosfns.c.
   The Haiku implementation is in haiku.c.  */
// C 4539-4548: `#elif !defined (WINDOWSNT) && !defined (MSDOS) && !defined (HAIKU)` - dead on the
// frozen target (clang's chain takes the DARWIN_OS arm) and **live on Android**: the port had it
// behind `no_tty_process_attributes`, which no configuration emits, so the Android build compiled
// the Darwin body instead of this one (status 4.749).  The condition is C's, with the chain's
// earlier arms negated the way an `#elif` means them.
#[cfg (all (not (feature = "gnu_linux"), not (feature = "cygwin"),
            not (all (feature = "solaris2", feature = "have_procfs")),
            not (feature = "freebsd"), not (feature = "openbsd"),
            not (target_os = "macos"),
            not (feature = "windowsnt"), not (feature = "msdos"), not (feature = "have_haiku")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_process_attributes (pid: Lisp_Object) -> Lisp_Object {
  return Qnil ();
}

DEFUN! ("get-internal-run-time", Fget_internal_run_time, Sget_internal_run_time,
       0, 0, 0,
       doc: "Return the current run time used by Emacs.\nThe time is returned as in the style of `current-time'.\n\nOn systems that can't determine the run time, `get-internal-run-time'\ndoes the same thing as `current-time'.  ",
       (),
       {
  /* C 4536-4560: `#ifdef HAVE_GETRUSAGE` is live (HAVE_GETRUSAGE is 1).  */
  {
    let mut usage: rusage = std::mem::zeroed ();
    let mut secs: time_t;
    let mut usecs: c_int;

    if getrusage (RUSAGE_SELF, &mut usage) < 0
      /* This shouldn't happen.  What action is appropriate?  */
      { xsignal0 (Qerror ()); }

    /* Sum up user time and system time.  */
    secs = usage.ru_utime.tv_sec + usage.ru_stime.tv_sec;
    usecs = usage.ru_utime.tv_usec as c_int + usage.ru_stime.tv_usec as c_int;
    if usecs >= 1000000
      {
	usecs -= 1000000;
	secs += 1;
      }
    return make_lisp_s_us (secs, usecs as c_long);
  }
  /* C 4554-4560's `#else` arms (`w32_get_internal_run_time` and `Fcurrent_time`) are dead
     (clang takes the `HAVE_GETRUSAGE` arm); carried.  */
  #[cfg (feature = "no_have_getrusage")]
  {
    #[cfg (feature = "windowsnt")]
    { return w32_get_internal_run_time (); }
    #[cfg (not (feature = "windowsnt"))]
    { return Fcurrent_time (); }
  }
});

/* Wide character string collation.  */

// C 4565-4741: `#ifdef __STDC_ISO_10646__` — dead on this target (measured: clang does not
// define the macro with `-std=gnu23`); carried in full.
#[cfg (feature = "stdc_iso_10646")]
pub const LC_COLLATE_MASK: c_int = 0;
#[cfg (feature = "stdc_iso_10646")]
pub const LC_CTYPE_MASK: c_int = 0;

// C 4577-4665: `# ifndef HAVE_NEWLOCALE` is dead here (`HAVE_NEWLOCALE` is 1); carried.
/* C 4583-4587's five renames.  C's preprocessor makes each C name denote the renamed entry,
   and the port keeps both names the same way: the renamed entry is the type/function and
   the C name is the constant that denotes it (PORTING.md 10.13's rename row).  */
/// C 4588's `typedef char const *locale_t;` (after C 4584's rename: `emacs_locale_t`).
#[cfg (feature = "stdc_iso_10646_no_newlocale")]
pub type emacs_locale_t = *const c_char;
/// C 4584's `# define locale_t emacs_locale_t`.
#[cfg (feature = "stdc_iso_10646_no_newlocale")]
pub const locale_t: () = emacs_locale_t;
/// C 4588's `typedef char const *locale_t;` (the name the dead arm's code uses).
#[cfg (feature = "stdc_iso_10646_no_newlocale")]
pub type locale_t = *const c_char;
/// C 4585's `# define newlocale emacs_newlocale`.
#[cfg (feature = "stdc_iso_10646_no_newlocale")]
pub const newlocale: unsafe extern "C" fn (c_int, *const c_char, locale_t) -> locale_t =
  emacs_newlocale;
/// C 4583's `# define freelocale emacs_freelocale`.
#[cfg (feature = "stdc_iso_10646_no_newlocale")]
pub const freelocale: unsafe extern "C" fn (locale_t) = emacs_freelocale;
/// C 4586's `# define wcscoll_l emacs_wcscoll_l`.
#[cfg (feature = "stdc_iso_10646_no_newlocale")]
pub const wcscoll_l: unsafe extern "C" fn (*const wchar_t, *const wchar_t, locale_t) -> c_int =
  emacs_wcscoll_l;
/// C 4587's `# define towlower_l emacs_towlower_l`.
#[cfg (feature = "stdc_iso_10646_no_newlocale")]
pub const towlower_l: unsafe extern "C" fn (wint_t, locale_t) -> wint_t = emacs_towlower_l;

#[cfg (feature = "stdc_iso_10646_no_newlocale")]
unsafe extern "C" fn emacs_newlocale (category_mask: c_int, locale: *const c_char,
				     loc: locale_t) -> locale_t {
  let _ = category_mask;
  let _ = loc;
  return locale;
}

#[cfg (feature = "stdc_iso_10646_no_newlocale")]
unsafe extern "C" fn emacs_freelocale (loc: locale_t) {
  let _ = loc;
}

#[cfg (feature = "stdc_iso_10646_no_newlocale")]
unsafe extern "C" fn emacs_setlocale (category: c_int, locale: *const c_char)
  -> *mut c_char {
  *errno () = 0;
  let loc: *mut c_char = setlocale (category, locale);
  if !loc.is_null () || *errno () != 0
    { return loc; }
  *errno () = EINVAL;
  return std::ptr::null_mut ();
}

#[cfg (feature = "stdc_iso_10646_no_newlocale")]
unsafe extern "C" fn emacs_wcscoll_l (a: *const wchar_t, b: *const wchar_t, loc: locale_t)
  -> c_int {
  let mut result: c_int = 0;
  let oldloc: *mut c_char = emacs_setlocale (LC_COLLATE, std::ptr::null ());
  let mut err: c_int;

  if oldloc.is_null ()
    { err = *errno (); }
  else
    {
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
      let oldcopy: *mut c_char = SAFE_ALLOCA! (sa_avail, strlen (oldloc) + 1) as *mut c_char;
      strcpy (oldcopy, oldloc);
      if emacs_setlocale (LC_COLLATE, loc).is_null ()
	{ err = *errno (); }
      else
	{
	  *errno () = 0;
	  result = wcscoll (a, b);
	  err = *errno ();
	  if emacs_setlocale (LC_COLLATE, oldcopy).is_null ()
	    { err = *errno (); }
	}
      SAFE_FREE! (sa_count, sa_mark);
    }

  *errno () = err;
  return result;
}

#[cfg (feature = "stdc_iso_10646_no_newlocale")]
unsafe extern "C" fn emacs_towlower_l (wc: wint_t, loc: locale_t) -> wint_t {
  let mut result: wint_t = wc;
  let oldloc: *mut c_char = emacs_setlocale (LC_CTYPE, std::ptr::null ());

  if !oldloc.is_null ()
    {
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
      let oldcopy: *mut c_char = SAFE_ALLOCA! (sa_avail, strlen (oldloc) + 1) as *mut c_char;
      strcpy (oldcopy, oldloc);
      if !emacs_setlocale (LC_CTYPE, loc).is_null ()
	{
	  result = towlower (wc);
	  emacs_setlocale (LC_COLLATE, oldcopy);
	}
      SAFE_FREE! (sa_count, sa_mark);
    }

  return result;
}

#[cfg(feature = "stdc_iso_10646")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn str_collate (s1: Lisp_Object, s2: Lisp_Object, locale: Lisp_Object,
				      ignore_case: Lisp_Object) -> c_int {
  let mut res: c_int = 0;
  let mut err: c_int;
  let mut len: ptrdiff_t;
  let mut i: ptrdiff_t;
  let mut i_byte: ptrdiff_t;
  let mut p1: *mut wchar_t;
  let mut p2: *mut wchar_t;

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  /* Convert byte stream to code points.  */
  len = SCHARS (s1); i = 0; i_byte = 0;
  SAFE_NALLOCA! (p1, 1, len + 1);
  while i < len
    {
      let p: *mut wchar_t = p1.add (i as usize);
      *p = fetch_string_char_advance (s1, &mut i, &mut i_byte);
    }
  *p1.add (len as usize) = 0;

  len = SCHARS (s2); i = 0; i_byte = 0;
  SAFE_NALLOCA! (p2, 1, len + 1);
  while i < len
    {
      let p: *mut wchar_t = p2.add (i as usize);
      *p = fetch_string_char_advance (s2, &mut i, &mut i_byte);
    }
  *p2.add (len as usize) = 0;

  if STRINGP (locale)
    {
      let loc: locale_t = newlocale (LC_COLLATE_MASK | LC_CTYPE_MASK, SSDATA (locale),
				     std::ptr::null ());
      if loc.is_null ()
	{ error (c"Invalid locale %s: %s".as_ptr (), SSDATA (locale), emacs_strerror (*errno ())); }

      if ! NILP (ignore_case)
	{
	  let mut i: c_int = 1;
	  while i < 3
	    {
	      let p: *mut wchar_t = if i == 1 { p1 } else { p2 };
	      while *p != 0
		{
		  *p = towlower_l (*p, loc);
		  p = p.add (1);
		}
	      i += 1;
	    }
	}

      *errno () = 0;
      res = wcscoll_l (p1, p2, loc);
      err = *errno ();
      freelocale (loc);
    }
  else
    {
      if ! NILP (ignore_case)
	{
	  let mut i: c_int = 1;
	  while i < 3
	    {
	      let p: *mut wchar_t = if i == 1 { p1 } else { p2 };
	      while *p != 0
		{
		  *p = towlower (*p);
		  p = p.add (1);
		}
	      i += 1;
	    }
	}

      *errno () = 0;
      res = wcscoll (p1, p2);
      err = *errno ();
    }
  // C 4730-4736: `#  ifndef HAVE_NEWLOCALE` is dead (`HAVE_NEWLOCALE` is 1); carried.
  #[cfg (feature = "stdc_iso_10646_no_newlocale")]
  {
    if err != 0
      { error (c"Invalid locale or string for collation: %s".as_ptr (), emacs_strerror (err)); }
  }
  #[cfg (not (feature = "stdc_iso_10646_no_newlocale"))]
  {
    if err != 0
      { error (c"Invalid string for collation: %s".as_ptr (), emacs_strerror (err)); }
  }

  SAFE_FREE! (sa_count, sa_mark);
  return res;
}

// C 4743-4760: `#ifdef WINDOWSNT` — dead; carried.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn str_collate (s1: Lisp_Object, s2: Lisp_Object, locale: Lisp_Object,
				      ignore_case: Lisp_Object) -> c_int {

  let loc: *mut c_char = if STRINGP (locale) { SSDATA (locale) } else { std::ptr::null_mut () };
  let mut res: c_int = 0;
  let err: c_int = *errno ();

  *errno () = 0;
  res = w32_compare_strings (SSDATA (s1), SSDATA (s2), loc, !NILP (ignore_case));
  if *errno () != 0
    { error (c"Invalid string for collation: %s".as_ptr (), strerror (*errno ())); }

  *errno () = err;
  return res;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_sysdep () {
  /* PORTING.md P-126: sample the diagnostic switch here, where no handler can be running yet.  */
  FEMACS_CHILD_TRACE.store (! getenv (c"FEMACS_CHILD_TRACE".as_ptr ()).is_null (),
                            core::sync::atomic::Ordering::Relaxed);
  defsubr (&raw mut Sget_internal_run_time);
}
