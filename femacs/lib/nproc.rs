// femacs: port of lib/nproc.c @ fd7afced403.
//
// `lib/nproc.c` is in `docs/baseline/config/cond-eval-verified.txt`, so the gate evaluates its
// `#if`s against the frozen configuration and compares only the live text; the reference build
// does compile the unit (`build/emacs-ref/lib/nproc.o` exports `_num_processors` and imports
// `_getenv`, `_strtoul`, `_sysconf`, `_sysctl` — the port's five live calls exactly).
//
// The live arms are the `sysconf` pair (`_SC_NPROCESSORS_ONLN` 58 for `NPROC_CURRENT`,
// `_SC_NPROCESSORS_CONF` 57 for `NPROC_ALL`), the macOS/BSD `sysctl` fallback
// (`HAVE_SYSCTL` 1, not glibc/Linux, `HW_NCPU` 3 under `CTL_HW` 6) and the OpenMP environment
// parsing; every other arm is dead and is *named* at its site below (P-51):
// C 74-208's affinity-mask bodies (glibc's `pthread_getaffinity_np`,
// `HAVE_SCHED_GETAFFINITY_LIKE_GLIBC`'s static and `CPU_ALLOC` cpu_set_t forms,
// `HAVE_SCHED_GETAFFINITY_NP`'s NetBSD `cpuset_t` and the native-Windows
// `GetProcessAffinityMask`), C 293's glibc `/sys`/`/proc` fixup, C 300-321's HP-UX
// `pstat_getdynamic`, C 347-354's Windows `GetSystemInfo`, C 359-481's whole cgroup2 block
// (`cgroup2_mount`, `get_cgroup2_cpu_quota` and their `<mntent.h>`/`getline`/`snprintf`
// machinery) and C 493-508's `SCHED_DEADLINE` `sched_getscheduler` switch.  Their types
// (`cpu_set_t`, `cpuset_t`, `DWORD_PTR`, `SYSTEM_INFO`, `struct pst_dynamic`, `FILE`,
// `struct mntent`), their functions (`CPU_ALLOC`, `CPU_FREE`, `sched_getaffinity`,
// `cpuset_create`, `GetProcessAffinityMask`, `pstat_getdynamic`, `setmntent`, `getmntent`,
// `getline` …) and their headers (`<pthread.h>`, `<sched.h>`, `<windows.h>`,
// `<sys/pstat.h>`, `<mntent.h>`) are not on this target.
//
// The line drift this costs is declared in docs/ports/line-drift-tolerance.tsv.  What the
// port must *keep* is the shape: `num_processors_via_affinity_mask`'s live body really is
// just `return 0;` (C 230) on this target — every one of its arms is dead — and
// `cpu_quota`'s is `return quota;` with `quota` still `ULONG_MAX` (C 510), which is what
// makes `num_processors (NPROC_CURRENT)` fall through to the `sysconf`/`sysctl` chain.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* Written by Glen Lenker and Bruno Haible.  */

use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short,
               c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");     // <limits.h> (ULONG_MAX)
include!("include/stdint.in.rs");
include!("include/sys_types.in.rs");  // <sys/types.h> (size_t)
include!("include/unistd.in.rs");     // <unistd.h> (sysconf, _SC_NPROCESSORS_*)
include!("include/stdcountof.rs");    // <stdcountof.h> (countof)
include!("include/stdlib.rs");        // <stdlib.h> (getenv, strtoul)
include!("include/c-ctype.rs");        // "c-ctype.h" (c_isdigit, c_isspace)
include!("include/minmax.rs");         // "minmax.h" (MIN, MAX)
include!("include/sys_sysctl.rs");    // <sys/sysctl.h> (CTL_HW, HW_NCPU, sysctl)
include!("include/nproc.rs");          // "nproc.h" (enum nproc_query, num_processors)

/* C 34-40's `<pthread.h>`/`<sched.h>` includes are dead (the guarded blocks are all off), and
   C 42's `<sys/types.h>` and C 48-54's `<sys/param.h>`/`<sys/sysctl.h>` are the mirrors above.
   C 44-46's `<sys/pstat.h>`, C 56-59's `<windows.h>` and C 24-26's `<mntent.h>` are dead: only
   the arms named in the header note would need them.  C 30's `<stdio.h>` and C 31's
   `<string.h>` are reached only by the dead cgroup2 block (C 359-481), so this unit includes
   no stdio/string mirror.  */

/* C 65's `# define NPROC_MINIMUM 1`: a constant rather than a preprocessor line (the
   lib/malloc.rs shape).  */
const NPROC_MINIMUM: c_ulong = 1;

/* Return the number of processors available to the current process, based
   on a modern system call that returns the "affinity" between the current
   process and each CPU.  Return 0 if unknown or if such a system call does
   not exist.  */
unsafe fn num_processors_via_affinity_mask () -> c_ulong       /* C 71-72 */
{
  /* C 74-208's arms are all dead.  Their four bodies: the glibc >= 2.3.3/NPTL
     `pthread_getaffinity_np` block (C 74-97, behind `HAVE_PTHREAD_GETAFFINITY_NP &&
     defined __GLIBC__ && 0` — note C 110/225's own `&& 0` spellings), the NetBSD 5
     `cpuset_create`/`pthread_getaffinity_np` block (C 100-119, the `__NetBSD__` variant),
     C 121-200's `HAVE_SCHED_GETAFFINITY_LIKE_GLIBC` chain (`CPU_ALLOC`/`CPU_ALLOC_SIZE`/
     `CPU_COUNT_S`/`CPU_FREE` with the `alloc_count` retry loop, then the statically sized
     `cpu_set_t set` with `CPU_COUNT`/`CPU_ISSET`/`CPU_SETSIZE`), C 202-226's
     `HAVE_SCHED_GETAFFINITY_NP` NetBSD block (`cpuset_create`/`cpuset_size`/`cpuset_isset`/
     `cpuset_destroy`) and C 228-250's native-Windows `GetProcessAffinityMask` block
     (`DWORD_PTR`, `GetCurrentProcess`).  `HAVE_PTHREAD_GETAFFINITY_NP`,
     `HAVE_SCHED_GETAFFINITY_LIKE_GLIBC` and `HAVE_SCHED_GETAFFINITY_NP` are all undefined on
     this target and `_WIN32` is not defined, so the function's live text is the `return 0;`
     below — the same zero C returns here.  */
  return 0;                                                   /* C 230 */
}

/* Return the total number of processors.  Here QUERY must be one of
   NPROC_ALL, NPROC_CURRENT.  The result is guaranteed to be at least 1.  */
unsafe fn num_processors_available (query: nproc_query) -> c_ulong   /* C 236-237 */
{
  /* C 239-253's comment block is C's explanation of the three numbers (sysconf
     _SC_NPROCESSORS_CONF >= sysconf _SC_NPROCESSORS_ONLN >=
     num_processors_via_affinity_mask ()) and of the glibc /sys//proc caveat; the two arms it
     warns about return 1 or 2 in those situations, which is why C 290's `/sys` fixup
     (dead here: `__GLIBC__ >= 2 && defined __linux__`) exists at all.  */
  if query == nproc_query::NPROC_CURRENT                     /* C 254 */
    {
      /* Try the modern affinity mask system call.  */
      {
        let nprocs = num_processors_via_affinity_mask ();     /* C 258 */
        if nprocs > 0
          {
            return nprocs;
          }
      }

      /* C 264's `#if defined _SC_NPROCESSORS_ONLN` is live (the frozen table has 58).  */
      {
        let nprocs = sysconf (_SC_NPROCESSORS_ONLN);          /* C 267 */
        if nprocs > 0
          {
            return nprocs as c_ulong;
          }
      }
    }
  else /* query == NPROC_ALL */                               /* C 273 */
    {
      /* C 275's `#if defined _SC_NPROCESSORS_CONF` is live (the frozen table has 57); the
         `__GLIBC__ >= 2 && defined __linux__` fixup inside it (C 280-294) is dead.  */
      {
        let nprocs = sysconf (_SC_NPROCESSORS_CONF);          /* C 278 */
        if nprocs > 0
          {
            return nprocs as c_ulong;
          }
      }
    }

  /* C 300-321's `HAVE_PSTAT_GETDYNAMIC` arm (HP-UX's `struct pst_dynamic psd` and the
     `psd_proc_cnt`/`psd_max_proc_cnt` fields keyed on the query) is dead.  */

  /* Finally, as fallback, use the APIs that don't distinguish between
     NPROC_CURRENT and NPROC_ALL.  */
  /* C 326's `#if HAVE_SYSCTL && !(defined __GLIBC__ && defined __linux__) && defined HW_NCPU`
     is live; C 332-334's `#ifdef HW_NCPUONLINE` row is dead (this target does not define it),
     which is why the mib array below has the single `{ CTL_HW, HW_NCPU }` row of C 335.  */
  {
    let mut nprocs: c_int = 0;                                /* C 329 */
    let mut len: usize = std::mem::size_of_val (&nprocs);     /* C 330 */
    static mut mib: [[c_int; 2]; 1] = [[CTL_HW, HW_NCPU]];    /* C 331-336 */
    let mut i: c_int = 0;                                     /* C 337 */
    while i < countof! (mib) as c_int
      {
        if sysctl ((*std::ptr::addr_of_mut! (mib[i as usize])).as_mut_ptr (),
                   countof! (mib[i as usize]) as c_uint,
                   &mut nprocs as *mut c_int as *mut c_void, &mut len,
                   std::ptr::null_mut (), 0) == 0
            && len == std::mem::size_of_val (&nprocs)
            && 0 < nprocs
          {
            return nprocs as c_ulong;
          }
        i += 1;
      }
  }

  /* C 347-354's native-Windows `GetSystemInfo` arm is dead.  */

  return NPROC_MINIMUM;                                       /* C 356 */
}

/* C 359-481's `#if defined __linux__ || defined __ANDROID__` block - `cgroup2_mount`
   (C 373-410, with its `access`/`strdup`/`setmntent`/`getmntent`/`endmntent` and
   `<mntent.h>`), `get_cgroup2_cpu_quota` (C 413-481, with `fopen`/`getline`/`strncmp`/
   `strrchr`/`snprintf`/`sscanf`/`MAX` and its `PATH_MAX` `cpu_max_file` buffer) - is dead:
   neither macro is defined here, so neither static function exists on this target.  */

/* Return the cgroupv2 CPU quota if the current scheduler honors it.
   Otherwise return ULONG_MAX.
   Returned value will be >= 1.  */
unsafe fn cpu_quota () -> c_ulong                          /* C 487-488 */
{
  let mut quota: c_ulong = ULONG_MAX;                         /* C 490 */

  /* C 493-508's `HAVE_SCHED_GETAFFINITY_LIKE_GLIBC && defined SCHED_DEADLINE` block (the
     `sched_getscheduler (0)` switch that hands the cgroup2 quota to `get_cgroup2_cpu_quota`)
     is dead: both its guards belong to Linux.  */

  return quota;                                               /* C 510 */
}

/* Parse OMP environment variables without dependence on OMP.
   Return 0 for invalid values.  */
unsafe fn parse_omp_threads (mut threads: *const c_char) -> c_ulong   /* C 515-516 */
{
  if threads.is_null ()                                       /* C 518 */
    {
      return 0;
    }

  /* The OpenMP spec says that the value assigned to the environment variables
     "may have leading and trailing white space".  */
  while *threads != 0 && c_isspace (*threads as c_int)         /* C 523 */
    {
      threads = threads.add (1);
    }

  /* Convert it from positive decimal to 'unsigned long'.  */
  if c_isdigit (*threads as c_int)                             /* C 527 */
    {
      let mut endptr: *mut c_char = std::ptr::null_mut ();     /* C 529 */
      let mut value: c_ulong = strtoul (threads, &mut endptr, 10);   /* C 530 */
      while *endptr != 0 && c_isspace (*endptr as c_int)       /* C 531 */
        {
          endptr = endptr.add (1);
        }
      if *endptr == 0                                          /* C 533 */
        {
          return value;
        }
      /* Also accept the first value in a nesting level,
         since we can't determine the nesting level from env vars.  */
      else if *endptr == b',' as c_char                        /* C 537 */
        {
          return value;
        }
    }

  return 0;                                                    /* C 541 */
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn num_processors (mut query: nproc_query) -> c_ulong  /* C 544-545 */
{
  let mut nproc_limit: c_ulong = ULONG_MAX;                    /* C 547 */

  /* Honor the OpenMP environment variables, recognized also by all
     programs that are based on OpenMP.  */
  if query == nproc_query::NPROC_CURRENT_OVERRIDABLE           /* C 551 */
    {
      let omp_env_threads: c_ulong =
        parse_omp_threads (getenv (c"OMP_NUM_THREADS".as_ptr ()));        /* C 553-554 */

      let mut omp_env_limit: c_ulong =
        parse_omp_threads (getenv (c"OMP_THREAD_LIMIT".as_ptr ()));       /* C 556-557 */
      if omp_env_limit == 0                                    /* C 558 */
        {
          omp_env_limit = ULONG_MAX;
        }

      if omp_env_threads != 0                                  /* C 561 */
        {
          return MIN! (omp_env_threads, omp_env_limit);
        }

      nproc_limit = omp_env_limit;                             /* C 564 */
      query = nproc_query::NPROC_CURRENT;
    }

  /* Honor any CPU quotas.  */
  if query == nproc_query::NPROC_CURRENT && nproc_limit > NPROC_MINIMUM   /* C 569 */
    {
      let quota = cpu_quota ();                                /* C 571 */
      nproc_limit = MIN! (quota, nproc_limit);
    }

  if nproc_limit > NPROC_MINIMUM                               /* C 575 */
    {
      let nprocs = num_processors_available (query);            /* C 577 */
      nproc_limit = MIN! (nprocs, nproc_limit);
    }

  return nproc_limit;                                          /* C 581 */
}
