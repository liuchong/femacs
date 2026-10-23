// femacs: port of src/haiku.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2).
//
// **The file gate is `feature = "have_haiku"`** (P-86 clause 8): the reference builds `haiku.o`
// only in the Haiku branch of the build - `configure.ac` 3097-3107 (`HAIKU_OBJ="$HAIKU_OBJ
// haiku.o"` with 3107's `AC_DEFINE ([HAVE_HAIKU], [1], …)`) and `src/Makefile.in` 378/469 put
// `$(HAIKU_OBJ)` into the object list.  `docs/baseline/frozen-build-sources.txt` has no
// `src/haiku.c`, so the frozen configuration compiles nothing from this file at all, and **this
// machine has no Haiku SDK** (no `<kernel/OS.h>`, no `<pthread.h>` variant of it): the whole file
// is `text-only` (P-86 clause 3).
//
// The `<kernel/OS.h>` surface C 25 pulls in is declared in this unit (P-88 clause 1: no mirror in
// this tree carries it and this file is its only consumer), and each record carries **only the
// members the reference reads**, in C's order, with the offsets explicitly **UNVERIFIED** - there
// is no Haiku header here to measure them against (the `include/gtk.rs` `GtkWidgetClass` shape,
// status 3.481).  `docs/ports/haiku.md` names what a Haiku round owes each record.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, unused_mut,
         static_mut_refs, unsafe_op_in_unsafe_fn, clippy::all)]

include!("include/config.rs");                                  // C 19: `#include <config.h>`
#[cfg (feature = "have_haiku")]
include!("include/lisp.rs");                                    // C 21: `#include "lisp.h"`
#[cfg (feature = "have_haiku")]
include!("include/process.rs");                                 // C 22: `#include "process.h"`
#[cfg (feature = "have_haiku")]
include!("include/coding.rs");                                  // C 23: `#include "coding.h"`
/* C reaches `<sys/types.h>` and `<string.h>` through the three headers above (`process.h`'s and
   `lisp.h`'s own chains); the two mirrors that carry the names this file uses are named here
   (P-72 clause 4: a mirror may only name types it declares or reaches through its own
   `include!`s) - `uid_t`/`gid_t`/`ssize_t` and `strncmp`.  */
#[cfg (feature = "have_haiku")]
include!("../lib/include/sys_types.in.rs");                     // `uid_t`, `gid_t`
#[cfg (feature = "have_haiku")]
include!("../lib/include/unistd.in.rs");                        // `ssize_t` (the area cookie's type)
#[cfg (feature = "have_haiku")]
include!("../lib/include/string.rs");                           // lisp.h 30: `<string.h>` (`strncmp`)
#[cfg (feature = "have_haiku")]
include!("../lib/include/pwd.rs");                              // C 27: `#include <pwd.h>` (`getpwuid`)
/* C 28's `#include <stdlib.h>` comes through lisp.h's chain (the `lib/include/stdlib.rs` mirror),
   which is where `getloadavg`'s gnulib declaration also lives.  */

/* ------------------------------------------------------------------------------------------
   C 25: `#include <kernel/OS.h>`.  **Every name below is C's spelling, not a measurement** -
   this machine has no Haiku header (`ls /boot`, `find / -name OS.h` - there is no Haiku tree),
   so the widths and the offsets are `UNVERIFIED` (P-86 clause 3(2)).  The header's real widths
   (`int32`/`status_t` 32-bit, `bigtime_t` 64-bit, `team_id`/`thread_id`/`area_id` 32-bit) are the
   documented ones; the records carry the members this file reads and name the rest in the
   `## Owed` list of `docs/ports/haiku.md`.
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "have_haiku")]
pub type int32 = c_int;                     // C: `typedef int32 int32;`
#[cfg (feature = "have_haiku")]
pub type status_t = c_int;                  // C: `typedef int32 status_t;`
#[cfg (feature = "have_haiku")]
pub type int64 = i64;                       // C: `typedef int64 int64;` (OS.h's 64-bit signed)
#[cfg (feature = "have_haiku")]
pub type bigtime_t = int64;                 // C: `typedef int64 bigtime_t;` (microseconds)
#[cfg (feature = "have_haiku")]
pub type team_id = int32;
#[cfg (feature = "have_haiku")]
pub type thread_id = int32;
#[cfg (feature = "have_haiku")]
pub type area_id = int32;
#[cfg (feature = "have_haiku")]
pub const B_OK: status_t = 0;               // C: `#define B_OK 0`
#[cfg (feature = "have_haiku")]
pub const B_PAGE_SIZE: usize = 4096;        // C: `#define B_PAGE_SIZE 4096` (the x86 page)
#[cfg (feature = "have_haiku")]
pub const B_OS_NAME_LENGTH: usize = 32;     // C: `#define B_OS_NAME_LENGTH 32`

/* C: OS.h: `typedef struct { team_id team; int32 thread_count; int32 image_count; int32
   area_count; thread_id debugger_nub_thread; port_id debugger_nub_port; int32 argc;
   char args[64]; uid_t uid; gid_t gid; } team_info;` - this file reads `team`, `thread_count`,
   `args`, `uid` and `gid`; the offsets of `args`/`uid`/`gid` depend on the members carried above
   them and are **UNVERIFIED** here.  */
#[cfg (feature = "have_haiku")]
#[repr (C)]
pub struct team_info
{
  pub team: team_id,
  pub thread_count: int32,
  pub image_count: int32,                   // C's member, carried so `args` keeps C's order
  pub area_count: int32,                    // C's member, carried for the same reason
  pub debugger_nub_thread: thread_id,       // C's member
  pub debugger_nub_port: c_int,             // C's member (`port_id`)
  pub argc: int32,                          // C's member
  pub args: [c_char; 64],
  pub uid: uid_t,
  pub gid: gid_t,
}
#[cfg (feature = "have_haiku")]
#[repr (C)]
pub struct thread_info
{
  pub thread: thread_id,                    // C's member (unread here, carried for C's order)
  pub team: team_id,
  pub name: [c_char; B_OS_NAME_LENGTH],
  pub state: c_int,                         // C's member (`thread_state`)
  pub priority: int32,                      // C's member
  pub sem: int32,                           // C's member
  pub user_time: bigtime_t,
  pub kernel_time: bigtime_t,
}
#[cfg (feature = "have_haiku")]
#[repr (C)]
pub struct area_info
{
  pub area: area_id,                        // C's member (unread here, carried for C's order)
  pub name: [c_char; B_OS_NAME_LENGTH],     // C's member
  pub team: team_id,                        // C's member
  pub address: *mut c_void,                 // C's member
  pub size: u32,                            // C's member
  pub protection: u32,                      // C's member
  pub ram_size: usize,
}
#[cfg (feature = "have_haiku")]
#[repr (C)]
pub struct system_info
{
  pub boot_time: bigtime_t,                 // C's member (unread here, carried for C's order)
  pub cpu_count: u32,
  pub max_pages: u64,
  pub used_pages: u64,
  pub cached_pages: u64,
  pub block_cache_pages: u64,               // C's member
  pub ignored_pages: u64,                   // C's member
  pub needed_memory: u64,                   // C's member
  pub free_memory: u64,                     // C's member
  pub max_swap_pages: u64,                  // C's member
  pub free_swap_pages: u64,                 // C's member
  pub page_faults: u32,                     // C's member
  pub max_semaphores: u64,                  // C's member
  pub used_semaphores: u64,                 // C's member
  pub max_ports: u64,                       // C's member
  pub used_ports: u64,                      // C's member
  pub max_threads: u64,                     // C's member
  pub used_threads: u64,                    // C's member
  pub max_teams: u64,                       // C's member
  pub used_teams: u64,                      // C's member
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /* C: OS.h: `status_t get_next_team_info (int32 *cookie, team_info *info);`  */
  pub fn get_next_team_info (cookie: *mut int32, info: *mut team_info) -> status_t;
  /* C: OS.h: `status_t get_team_info (team_id team, team_info *info);`  */
  pub fn get_team_info (team: team_id, info: *mut team_info) -> status_t;
  /* C: OS.h: `status_t get_next_thread_info (team_id team, int32 *cookie, thread_info *info);`  */
  pub fn get_next_thread_info (team: team_id, cookie: *mut int32,
                               info: *mut thread_info) -> status_t;
  /* C: OS.h: `status_t get_next_area_info (team_id team, ssize_t *cookie, area_info *info);`  */
  pub fn get_next_area_info (team: team_id, cookie: *mut ssize_t,
                             info: *mut area_info) -> status_t;
  /* C: OS.h: `status_t get_system_info (system_info *info);`  */
  pub fn get_system_info (info: *mut system_info) -> status_t;
}

/* C 30-41: `list_system_processes`.  */
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn list_system_processes () -> Lisp_Object
{
  /* C declares `team_info info;` uninitialized; Rust refuses a `&raw mut` borrow of an
     uninitialized binding (E0381), so the port zeroes the storage first - `get_next_team_info`
     fills it before any read (the `src/systhread.rs` shape, status 3.498).  */
  let mut info: team_info = core::mem::zeroed ();
  let mut cookie: int32 = 0;
  let mut lval: Lisp_Object = Qnil ();

  while get_next_team_info (&raw mut cookie, &raw mut info) == B_OK
    { lval = Fcons (make_fixnum (info.team as EMACS_INT), lval); }

  return lval;
}

/* C 43-120: `system_process_attributes`.  */
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_process_attributes (pid: Lisp_Object) -> Lisp_Object
{
  CHECK_FIXNUM (pid);

  let mut info: team_info = core::mem::zeroed ();
  let mut lval: Lisp_Object = Qnil ();
  let mut inf: thread_info = core::mem::zeroed ();
  let mut area: area_info = core::mem::zeroed ();
  let id: team_id = XFIXNUM (pid) as team_id;
  let mut g: *mut passwd;
  let mut mem: usize = 0;

  if get_team_info (id, &raw mut info) != B_OK
    { return Qnil (); }

  let mut everything: bigtime_t = 0;
  let mut vsample: bigtime_t = 0;
  let mut cpu_eaten: bigtime_t = 0;
  let mut esample: bigtime_t = 0;

  lval = Fcons (Fcons (Qeuid (), make_fixnum (info.uid as EMACS_INT)), lval);
  lval = Fcons (Fcons (Qegid (), make_fixnum (info.gid as EMACS_INT)), lval);
  lval = Fcons (Fcons (Qthcount (), make_fixnum (info.thread_count as EMACS_INT)), lval);
  lval = Fcons (Fcons (Qcomm (), build_string_from_utf8 (info.args.as_ptr ())), lval);

  g = getpwuid (info.uid);

  if !g.is_null () && !(*g).pw_name.is_null ()
    { lval = Fcons (Fcons (Quser (), build_string ((*g).pw_name)), lval); }

  /* FIXME: Calculating this makes Emacs show up as using 100% CPU!  */

  {
    let mut team_cookie: int32 = 0;
    while get_next_team_info (&raw mut team_cookie, &raw mut info) == B_OK
      {
        let mut thread_cookie: int32 = 0;
        while get_next_thread_info (info.team, &raw mut thread_cookie, &raw mut inf) == B_OK
          {
            if inf.team == id && strncmp (inf.name.as_ptr (), c"idle thread ".as_ptr (), 12) != 0
              { cpu_eaten += inf.user_time + inf.kernel_time; }
            everything += inf.user_time + inf.kernel_time;
          }
      }
  }

  /* C 67's `sleep (0.05)`: the only `sleep` this translation unit has is `<unistd.h>`'s
     (`unsigned int sleep (unsigned int)`, through lisp.h's chain - `<kernel/OS.h>` is not on this
     machine to say otherwise), so C's fractional literal is converted to 0 by that prototype and
     the port spells the conversion C performs.  */
  sleep (0.05 as c_uint);

  {
    let mut team_cookie: int32 = 0;
    while get_next_team_info (&raw mut team_cookie, &raw mut info) == B_OK
      {
        let mut thread_cookie: int32 = 0;
        while get_next_thread_info (info.team, &raw mut thread_cookie, &raw mut inf) == B_OK
          {
            if inf.team == id && strncmp (inf.name.as_ptr (), c"idle thread ".as_ptr (), 12) != 0
              { esample += inf.user_time + inf.kernel_time; }
            vsample += inf.user_time + inf.kernel_time;
          }
      }
  }

  cpu_eaten = esample - cpu_eaten;
  everything = vsample - everything;

  if everything != 0
    { lval = Fcons (Fcons (Qpcpu (), make_float (((cpu_eaten as c_double)
                                                  / (everything as c_double)) * 100.0)),
                    lval); }
  else
    { lval = Fcons (Fcons (Qpcpu (), make_float (0.0)), lval); }

  {
    let mut area_cookie: ssize_t = 0;
    while get_next_area_info (id, &raw mut area_cookie, &raw mut area) == B_OK
      { mem += area.ram_size; }
  }

  let mut sinfo: system_info = core::mem::zeroed ();
  get_system_info (&raw mut sinfo);
  let max: int64 = (sinfo.max_pages as int64) * (B_PAGE_SIZE as int64);

  lval = Fcons (Fcons (Qpmem (), make_float (((mem as c_double) / (max as c_double)) * 100.0)),
                lval);
  lval = Fcons (Fcons (Qrss (), make_fixnum ((mem / 1024) as EMACS_INT)), lval);

  return lval;
}

/* C 123-131: the circular sample buffer C borrows from the w32 implementation.  */
/// C 125-131: `struct load_sample { time_t sample_time; bigtime_t idle; bigtime_t kernel;
/// bigtime_t user; };`
#[cfg (feature = "have_haiku")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct load_sample
{
  pub sample_time: time_t,
  pub idle: bigtime_t,
  pub kernel: bigtime_t,
  pub user: bigtime_t,
}

/// C 134: `static struct load_sample samples[16*60];` - C's tentative definition is zeroed
/// storage; Rust needs the initializer spelled out.
#[cfg (feature = "have_haiku")]
pub static mut samples: [load_sample; 16 * 60] =
  [load_sample { sample_time: 0, idle: 0, kernel: 0, user: 0 }; 16 * 60];
#[cfg (feature = "have_haiku")]
pub static mut first_idx: c_int = -1;
#[cfg (feature = "have_haiku")]
pub static mut last_idx: c_int = -1;
/// C 136: `static int max_idx = countof (samples);` - `countof` is C's `sizeof (a) / sizeof (a 
/// [0])`, i.e. 960.
#[cfg (feature = "have_haiku")]
pub static mut max_idx: c_int = 16 * 60;
#[cfg (feature = "have_haiku")]
pub static mut num_of_processors: c_uint = 0;

/* C 139-148: `buf_next` (static).  */
#[cfg (feature = "have_haiku")]
unsafe fn buf_next (from: c_int) -> c_int
{
  let mut next_idx: c_int = from + 1;

  if next_idx >= max_idx
    { next_idx = 0; }

  return next_idx;
}

/* C 150-159: `buf_prev` (static).  */
#[cfg (feature = "have_haiku")]
unsafe fn buf_prev (from: c_int) -> c_int
{
  let mut prev_idx: c_int = from - 1;

  if prev_idx < 0
    { prev_idx = max_idx - 1; }

  return prev_idx;
}

/* C 161-191: `getavg` (static).  */
#[cfg (feature = "have_haiku")]
unsafe fn getavg (which: c_int) -> c_double
{
  let mut retval: c_double = -1.0;
  let mut tdiff: c_double;
  let mut idx: c_int;
  let span: c_double = (if which == 0 { 1.0 } else if which == 1 { 5.0 } else { 15.0 }) * 60.0;
  let now: time_t = samples[last_idx as usize].sample_time;

  if first_idx != last_idx
    {
      idx = buf_prev (last_idx);
      loop
        {
          tdiff = difftime (now, samples[idx as usize].sample_time);
          if tdiff >= span - 2.0 * c_double::EPSILON * (now as c_double)
            {
              let sys: c_longdouble =
                ((samples[last_idx as usize].kernel + samples[last_idx as usize].user)
                 - (samples[idx as usize].kernel + samples[idx as usize].user)) as c_longdouble;
              let idl: c_longdouble =
                (samples[last_idx as usize].idle - samples[idx as usize].idle) as c_longdouble;

              retval = ((idl / (sys + idl)) * (num_of_processors as c_longdouble))
                as c_double;
              break;
            }
          if idx == first_idx
            { break; }
          idx = buf_prev (idx);
        }
    }

  return retval;
}

/* C 193-214: `sample_sys_load` (static).  */
#[cfg (feature = "have_haiku")]
unsafe fn sample_sys_load (idle: *mut bigtime_t, system: *mut bigtime_t, user: *mut bigtime_t)
{
  let mut i: bigtime_t = 0;
  let mut s: bigtime_t = 0;
  let mut u: bigtime_t = 0;
  let mut info: team_info = core::mem::zeroed ();
  let mut inf: thread_info = core::mem::zeroed ();

  let mut team_cookie: int32 = 0;
  while get_next_team_info (&raw mut team_cookie, &raw mut info) == B_OK
    {
      let mut thread_cookie: int32 = 0;
      while get_next_thread_info (info.team, &raw mut thread_cookie, &raw mut inf) == B_OK
        {
          if strncmp (inf.name.as_ptr (), c"idle thread ".as_ptr (), 12) == 0
            { i += inf.user_time + inf.kernel_time; }
          else
            { s += inf.kernel_time; u += inf.user_time; }
        }
    }

  *idle = i;
  *system = s;
  *user = u;
}

/* C 216-286: `getloadavg`.  C's `loadavg[]` parameter decays to a pointer and `nelem` is the
   caller's capacity; the port keeps both.  */
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getloadavg (loadavg: *mut c_double, nelem: c_int) -> c_int
{
  let mut elem: c_int;
  let mut idle: bigtime_t = 0;
  let mut kernel: bigtime_t = 0;
  let mut user: bigtime_t = 0;
  let now: time_t = time (core::ptr::null_mut ());

  if num_of_processors <= 0
    {
      let mut i: system_info = core::mem::zeroed ();
      if get_system_info (&raw mut i) == B_OK
        { num_of_processors = i.cpu_count; }
    }

  /* If system time jumped back for some reason, delete all samples whose time is later than the
     current wall-clock time.  This prevents load average figures from becoming frozen for
     prolonged periods of time, when system time is reset backwards.  */
  if last_idx >= 0
    {
      while difftime (now, samples[last_idx as usize].sample_time) < -1.0
        {
          if last_idx == first_idx
            {
              first_idx = -1;
              last_idx = -1;
              break;
            }
          last_idx = buf_prev (last_idx);
        }
    }

  /* Store another sample.  We ignore samples that are less than 1 sec apart.  */
  if last_idx < 0
      || (difftime (now, samples[last_idx as usize].sample_time)
          >= 1.0 - 2.0 * c_double::EPSILON * (now as c_double))
    {
      sample_sys_load (&raw mut idle, &raw mut kernel, &raw mut user);
      last_idx = buf_next (last_idx);
      samples[last_idx as usize].sample_time = now;
      samples[last_idx as usize].idle = idle;
      samples[last_idx as usize].kernel = kernel;
      samples[last_idx as usize].user = user;
      /* If the buffer has more that 15 min worth of samples, discard the old ones.  */
      if first_idx == -1
        { first_idx = last_idx; }
      while first_idx != last_idx
             && (difftime (now, samples[first_idx as usize].sample_time)
                 >= 15.0 * 60.0 + 2.0 * c_double::EPSILON * (now as c_double))
        { first_idx = buf_next (first_idx); }
    }

  elem = 0;
  while elem < nelem
    {
      let avg: c_double = getavg (elem);

      if avg < 0.0
        { break; }
      *loadavg.offset (elem as isize) = avg;
      elem += 1;
    }

  /* Always return at least one element, otherwise load-average returns nil, and Lisp programs
     might decide we cannot measure system load.  For example, jit-lock-stealth-load's defcustom
     might decide that feature is "unsupported".  */
  if elem == 0
    {
      *loadavg.offset (elem as isize) = 0.09;	/* < display-time-load-average-threshold */
      elem += 1;
    }

  return elem;
}

/// C's `long double` is the port's `c_longdouble`, aliased to `c_double` the way
/// `src/editfns.rs` 197 does (PORTING.md's `long double` row: never downgrade to `f64` silently;
/// the alias is the documented widening the other units use).
#[cfg (feature = "have_haiku")]
pub type c_longdouble = c_double;

/* C: time.h.  `<time.h>` reaches this file through `"lisp.h"`, whose chain carries `time` and
   `time_t` but not `difftime`, so that entry stands here (P-88 clause 1).  (A `///` with no item
   after it is a syntax error - the `include/pgtkterm.rs` 776 lesson, status 3.496.)  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /// C: time.h: `double difftime (time_t time1, time_t time0);`
  pub fn difftime (time1: time_t, time0: time_t) -> c_double;
}
